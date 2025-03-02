use rayon::prelude::*;
use std::{
    borrow::Cow,
    fmt::Display,
    io,
    path::Path,
    time::{Duration, Instant},
};

pub struct Bench {
    name: String,
    setup_ms: Measurements,
    run_ms: Measurements,
    peak_mem: usize,
}

pub struct BenchInContext {
    bench: Bench,
    in_size: usize,
    out_size: usize,
}

pub struct Measurements {
    data: Vec<u128>,
    sum: u128,
    rem_samples: usize,
    precise: bool,
}

impl Bench {
    fn new(name: String, precise: bool) -> Self {
        Self {
            name,
            setup_ms: Measurements::new(precise),
            run_ms: Measurements::new(precise),
            peak_mem: 0,
        }
    }

    pub fn with_context(self, in_size: usize, out_size: usize) -> BenchInContext {
        BenchInContext {
            bench: self,
            in_size,
            out_size,
        }
    }

    pub fn par_run<T, FS, F>(name: String, setup_f: FS, f: F, threads: usize, precise: bool) -> Self
    where
        FS: (Fn(usize) -> T) + Send + Sync,
        F: (Fn(T) -> usize) + Send + Sync,
    {
        let progress = get_progress_bar(format!("Measuring '{name}'..."));
        progress.set_message("warming up...");

        for _ in 0..10 {
            // Warmup.
            (0..threads).into_par_iter().for_each(|i| {
                let data = setup_f(i);
                f(data);
            });
            progress.tick();
        }

        let mut bench = Bench::new(name, precise);

        while bench.run_ms.needs_more() {
            let ds = (0..threads)
                .into_par_iter()
                .map(|i| {
                    let (setup_d, data) = Measurements::measure_one_setup(&mut || setup_f(i));
                    let (run_d, mem) = Measurements::measure_one_run(data, &mut |x| f(x));
                    (setup_d, run_d, mem)
                })
                .collect::<Vec<_>>();
            let longest = ds.iter().max_by_key(|(s, r, _)| *s + *r).unwrap();
            let largest_mem = ds.iter().map(|(_, _, m)| m).max().unwrap();
            bench.setup_ms.add(longest.0);
            bench.run_ms.add(longest.1);
            bench.record_mem(*largest_mem);
            progress.tick();
            progress.set_message(format!(
                "measuring (current mean: {} ({} + {}), stddev: {}, peak mem: {})",
                DisplayNs(bench.avg()),
                DisplayNs(bench.setup_ms.avg()),
                DisplayNs(bench.run_ms.avg()),
                DisplayNs(bench.run_ms.stddev()),
                DisplayB(bench.peak_mem)
            ));
        }

        progress.finish_and_clear();
        bench
    }

    pub fn run<T, FS, F>(name: String, mut setup_f: FS, mut f: F, precise: bool) -> Self
    where
        FS: FnMut() -> T,
        F: FnMut(T),
    {
        let progress = get_progress_bar(format!("Measuring '{name}'..."));
        progress.set_message("warming up...");

        for _ in 0..10 {
            // Warmup.
            let data = setup_f();
            f(data);
            progress.tick();
        }

        let mut bench = Bench::new(name, precise);

        while bench.setup_ms.needs_more() || bench.run_ms.needs_more() {
            let (setup_d, data) = Measurements::measure_one_setup(&mut setup_f);
            let (run_d, _) = Measurements::measure_one_run(data, &mut f);
            bench.setup_ms.add(setup_d);
            bench.run_ms.add(run_d);
            progress.tick();
            progress.set_message(format!(
                "measuring (current mean: {} ({} + {}), stddev: {})",
                DisplayNs(bench.avg()),
                DisplayNs(bench.setup_ms.avg()),
                DisplayNs(bench.run_ms.avg()),
                DisplayNs(bench.run_ms.stddev()),
            ));
        }

        progress.finish_and_clear();
        bench
    }

    fn avg(&self) -> f64 {
        self.setup_ms.avg() + self.run_ms.avg()
    }

    fn variance(&self) -> f64 {
        self.iter_pairs()
            .map(|(s, r)| (self.avg() - (s as f64 + r as f64)).powi(2))
            .sum()
    }

    fn stddev(&self) -> f64 {
        self.variance().sqrt()
    }

    fn max(&self) -> u128 {
        self.iter_pairs().map(|(s, r)| s + r).max().unwrap_or(0)
    }

    fn min(&self) -> u128 {
        self.iter_pairs().map(|(s, r)| s + r).min().unwrap_or(0)
    }

    fn median(&self) -> u128 {
        let mut sorted: Vec<u128> = self.iter_pairs().map(|(s, r)| s + r).collect::<Vec<_>>();
        sorted.sort();
        let len = sorted.len();

        if len % 2 == 0 {
            (sorted[len / 2] + sorted[len / 2 + 1]) / 2
        } else {
            sorted[len / 2]
        }
    }

    fn iter_pairs(&self) -> impl Iterator<Item = (u128, u128)> + '_ {
        self.setup_ms
            .data
            .iter()
            .zip(self.run_ms.data.iter())
            .map(|(s, r)| (*s, *r))
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        use std::io::Write;
        if let Some(dir_path) = path.as_ref().parent() {
            std::fs::create_dir_all(dir_path)?
        }
        let mut file = std::fs::File::create(&path)?;

        writeln!(file, "setup_ns,run_ns,peak_mem")?;
        for (s, r) in self.iter_pairs() {
            writeln!(file, "{s},{r},{}", self.peak_mem)?;
        }

        println!("Saved {} bench results to {}", self.name, path.as_ref().display());
        Ok(())
    }

    fn record_mem(&mut self, mem: usize) {
        self.peak_mem = std::cmp::max(self.peak_mem, mem);
    }
}

impl Measurements {
    fn new(precise: bool) -> Self {
        Self {
            data: vec![],
            sum: 0,
            rem_samples: 10,
            precise,
        }
    }

    fn add(&mut self, duration: Duration) {
        let old_var = self.variance();

        let x = duration.as_nanos();
        self.data.push(x);
        self.sum += x;

        let new_var = self.variance();
        let ratio = new_var / old_var;
        let delta = (ratio - 1.0).abs();

        let threshold = if self.precise { 0.001 } else { 0.05 };

        if delta < threshold {
            self.rem_samples = self.rem_samples.saturating_sub(1);
        } else if self.rem_samples < 10 {
            self.rem_samples = 10;
        }
    }

    fn needs_more(&self) -> bool {
        (self.rem_samples > 0 && self.sum < 60_000_000_000) || self.sum < 1_000_000
    }

    pub fn measure<T, FS, F>(name: String, mut setup_f: FS, mut f: F) -> Self
    where
        FS: FnMut() -> T,
        F: FnMut(T),
    {
        let progress = get_progress_bar(format!("Measuring '{name}'..."));
        progress.set_message("warming up...");

        for _ in 0..10 {
            // Warmup.
            let data = setup_f();
            f(data);
            progress.tick();
        }

        let mut measurements = Measurements::new(false);

        while measurements.needs_more() {
            let data = setup_f();
            let (duration, _) = Self::measure_one_run(data, &mut f);
            measurements.add(duration);
            progress.tick();
            progress.set_message(format!(
                "measuring (current mean: {:.4}ms, var: {:.4})",
                measurements.avg() / 1_000_000.0,
                measurements.variance() / 1_000_000.0,
            ));
        }

        progress.finish_and_clear();
        measurements
    }

    fn measure_one_setup<T, F>(f: &mut F) -> (Duration, T)
    where
        F: FnMut() -> T,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (duration, result)
    }

    fn measure_one_run<I, R, F>(input: I, f: &mut F) -> (Duration, R)
    where
        F: FnMut(I) -> R,
    {
        let start = Instant::now();
        let result = f(input);
        let duration = start.elapsed();
        (duration, result)
    }

    fn avg(&self) -> f64 {
        self.sum as f64 / self.data.len() as f64
    }

    fn variance(&self) -> f64 {
        self.data
            .iter()
            .map(|x| (self.avg() - (*x as f64)).powi(2))
            .sum::<f64>()
            / (self.data.len() as f64 - 1.0)
    }

    fn stddev(&self) -> f64 {
        self.variance().sqrt()
    }

    fn max(&self) -> u128 {
        self.data.iter().copied().max().unwrap_or(0)
    }

    fn min(&self) -> u128 {
        self.data.iter().copied().min().unwrap_or(0)
    }

    fn median(&self) -> u128 {
        let mut sorted: Vec<u128> = self.data.to_vec();
        sorted.sort();
        let len = sorted.len();

        if len % 2 == 0 {
            (sorted[len / 2] + sorted[len / 2 + 1]) / 2
        } else {
            sorted[len / 2]
        }
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) {
        use std::io::Write;
        if let Some(dir_path) = path.as_ref().parent() {
            std::fs::create_dir_all(dir_path).expect("dir create");
        }
        let mut file = std::fs::File::create(path).expect("file create");

        for d in &self.data {
            writeln!(file, "{d}").expect("file write");
        }
    }
}

fn get_progress_bar<S: Into<Cow<'static, str>>>(msg: S) -> indicatif::ProgressBar {
    use indicatif::{ProgressBar, ProgressStyle};
    let style = ProgressStyle::with_template(" {spinner:.cyan} {prefix} [{elapsed_precise}] {msg}").unwrap();

    let progress = ProgressBar::new_spinner().with_style(style);
    progress.set_prefix(msg);

    progress
}

struct DisplayNs(f64);
struct BytesPerNanosecond(usize, f64);
struct DisplayB(usize);

impl Display for DisplayNs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.4}ms", self.0 / 1_000_000.0)
    }
}

impl Display for BytesPerNanosecond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gbs = self.0 as f64 / self.1;
        write!(f, "{gbs: >8.4} GB/s")
    }
}

impl Display for DisplayB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mb = self.0 as f64 / 1_000_000.0;
        write!(f, "{:.4}MB", mb)
    }
}

impl Display for Bench {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:", self.name)?;
        writeln!(
            f,
            "mean     : {}
              {} setup
              {} run",
            DisplayNs(self.avg()),
            DisplayNs(self.setup_ms.avg()),
            DisplayNs(self.run_ms.avg())
        )?;
        writeln!(
            f,
            "stddev   : {}
              {} setup
              {} run",
            DisplayNs(self.stddev()),
            DisplayNs(self.setup_ms.stddev()),
            DisplayNs(self.run_ms.stddev())
        )?;
        writeln!(f, "peak mem : {}", DisplayB(self.peak_mem))?;
        writeln!(f, "[{:.4}ns..{:.4}ns..{:.4}ns]", self.min(), self.median(), self.max())?;
        writeln!(
            f,
            "setup:    [{:.4}ns..{:.4}ns..{:.4}ns]",
            self.setup_ms.min(),
            self.setup_ms.median(),
            self.setup_ms.max()
        )?;
        writeln!(
            f,
            "run:      [{:.4}ns..{:.4}ns..{:.4}ns]",
            self.run_ms.min(),
            self.run_ms.median(),
            self.run_ms.max()
        )
    }
}

impl Display for BenchInContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let in_bpns = BytesPerNanosecond(self.in_size, self.bench.avg());
        let out_bpns = BytesPerNanosecond(self.out_size, self.bench.avg());

        writeln!(f, "{}:", self.bench.name)?;
        writeln!(
            f,
            "mean     : {}
              {} setup
              {} run",
            DisplayNs(self.bench.avg()),
            DisplayNs(self.bench.setup_ms.avg()),
            DisplayNs(self.bench.run_ms.avg())
        )?;
        writeln!(
            f,
            "stddev   : {}
              {} setup
              {} run",
            DisplayNs(self.bench.stddev()),
            DisplayNs(self.bench.setup_ms.stddev()),
            DisplayNs(self.bench.run_ms.stddev())
        )?;
        writeln!(f, "peak mem : {}", DisplayB(self.bench.peak_mem))?;
        writeln!(f, "mean I/O thpt : {in_bpns} / {out_bpns}")?;
        writeln!(
            f,
            "compression factor : {:.2}",
            self.out_size as f64 / self.in_size as f64
        )?;
        writeln!(
            f,
            "[{:.4}ns..{:.4}ns..{:.4}ns]",
            self.bench.min(),
            self.bench.median(),
            self.bench.max()
        )?;
        writeln!(
            f,
            "setup: [{:.4}ns..{:.4}ns..{:.4}ns]",
            self.bench.setup_ms.min(),
            self.bench.setup_ms.median(),
            self.bench.setup_ms.max()
        )?;
        writeln!(
            f,
            "run:   [{:.4}ns..{:.4}ns..{:.4}ns]",
            self.bench.run_ms.min(),
            self.bench.run_ms.median(),
            self.bench.run_ms.max()
        )
    }
}

impl Display for Measurements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "mean     : {}", DisplayNs(self.avg()))?;
        writeln!(f, "stddev   : {}", DisplayNs(self.stddev()))?;
        writeln!(f, "[{:.4}ns..{:.4}ns..{:.4}ns]", self.min(), self.median(), self.max())?;

        Ok(())
    }
}
