use anyhow::{bail, Result};
use std::{
    borrow::Cow,
    env, fs,
    io::{self, Seek, Write},
    path::Path,
};

#[derive(Debug, Default)]
struct TaxpayerRecords {
    nppes_provider_first_name: Vec<String>,
    nppes_provider_last_org_name: Vec<String>,
    nppes_provider_state: Vec<String>,
}

/// We read the Taxpayer.csv dataset and translate it to a compressed format,
/// namely the three columns required for
/// the query:
/// SELECT "nppes_provider_first_name" FROM "Taxpayer_1"
///   WHERE (("nppes_provider_last_org_name" = 'HOLDER') AND ("nppes_provider_state" = 'WA'))
/// GROUP BY "nppes_provider_first_name";
/// We compress each using FSST independently. Then we serialize them in the following format:
/// - First a 32bit integer identifying the number of rows (N).
/// - Then, three 32bit integers identifying the *end* offsets of each of the three columns.
/// - Then the columns are serialized in the same format:
///   - First the FSST table.
///   - Then N 32bit integers identifying the *end* offset in the compressed column of each string.
///   - Then the FSST compressed strings.
///   - Padding to 8-byte boundary.
fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 3 || args.len() > 4 {
        bail!("usage: {} INPUT_DIR OUTPUT_DIR [FSST_ITERS]", args[0]);
    }

    let input_dir = &args[1];
    let output_dir = &args[2];
    let output_binary_path = format!("{output_dir}/taxpayer.bin");
    let output_csv_path = format!("{output_dir}/taxpayer.csv");
    let output_libfsst_binary_path = format!("{output_dir}/taxpayer-libfsst.bin");
    let fsst_iters = if args.len() == 4 {
        args[3].parse().expect("FSST_ITERS has invalid format")
    } else {
        5
    };

    let records = read_csv(input_dir)?;

    write_csv(&records, &output_csv_path)?;
    write_compressed(&records, fsst_iters, &output_binary_path)?;
    write_compressed_libfsst(&records, &output_libfsst_binary_path)?;

    Ok(())
}

fn read_csv<P: AsRef<Path>>(path: P) -> Result<TaxpayerRecords> {
    let mut records = TaxpayerRecords::default();
    let spinner = get_spinner("reading csv...");

    for i in 1..=1 {
        let mut path = path.as_ref().to_path_buf();
        let csv_file_name = format!("Taxpayer_{i}.csv");
        spinner.set_message(format!("reading csv {csv_file_name}"));
        path.push(csv_file_name);

        let mut csv = csv::ReaderBuilder::new()
            .delimiter(b'|')
            .has_headers(false)
            .from_path(path)?;

        for record in csv.records() {
            let record = record?;
            records.nppes_provider_first_name.push(map_string(record.get(15)));
            records.nppes_provider_last_org_name.push(map_string(record.get(17)));
            records.nppes_provider_state.push(map_string(record.get(19)));
            spinner.inc(1);
        }
    }
    spinner.finish();

    println!("Read {} records", records.nppes_provider_last_org_name.len());
    Ok(records)
}

fn write_csv(records: &TaxpayerRecords, output_path: &str) -> Result<()> {
    let mut csv = csv::WriterBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_path(output_path)?;

    for i in 0..records.nppes_provider_first_name.len() {
        csv.write_record([
            map_cvs_string(&records.nppes_provider_first_name[i]),
            map_cvs_string(&records.nppes_provider_last_org_name[i]),
            map_cvs_string(&records.nppes_provider_state[i]),
        ])?;
    }

    Ok(())
}

fn write_compressed(records: &TaxpayerRecords, fsst_iters: usize, output_path: &str) -> Result<()> {
    let mut writer = io::BufWriter::new(fs::File::create(output_path)?);

    let row_count = u32::try_from(records.nppes_provider_first_name.len()).expect("number of rows to fit in 32 bits");
    writer.write_all(&row_count.to_le_bytes())?;
    // Write the temporary column offsets. We will overwrite those at the end.
    for _ in 0..3 {
        writer.write_all(&0_u32.to_le_bytes())?;
    }

    let mut total_len = 16;
    let first_offset = u32::try_from(total_len).expect("offset to be 32bit");
    let first_len = compress::fsst::compress(
        &records.nppes_provider_first_name,
        &mut writer,
        fsst_iters,
        get_fsst_progress_bar_style(),
        None,
    )?;
    total_len += first_len;
    pad(&mut writer, &mut total_len)?;
    let second_offset = u32::try_from(total_len).expect("offset to be 32bit");
    let second_len = compress::fsst::compress(
        &records.nppes_provider_last_org_name,
        &mut writer,
        fsst_iters,
        get_fsst_progress_bar_style(),
        None,
    )?;
    total_len += second_len;
    pad(&mut writer, &mut total_len)?;
    let third_offset = u32::try_from(total_len).expect("offset to be 32bit");
    compress::fsst::compress(
        &records.nppes_provider_state,
        &mut writer,
        fsst_iters,
        get_fsst_progress_bar_style(),
        None,
    )?;

    drop(writer);

    // Now fixup the offsets.
    let mut writer = fs::File::options().write(true).open(output_path)?;
    writer.seek(io::SeekFrom::Start(4))?;
    writer.write_all(&first_offset.to_le_bytes())?;
    writer.write_all(&second_offset.to_le_bytes())?;
    writer.write_all(&third_offset.to_le_bytes())?;

    Ok(())
}

fn write_compressed_libfsst(records: &TaxpayerRecords, output_path: &str) -> Result<()> {
    let mut writer = io::BufWriter::new(fs::File::create(output_path)?);
    let row_count = u32::try_from(records.nppes_provider_first_name.len()).expect("number of rows to fit in 32 bits");
    writer.write_all(&row_count.to_le_bytes())?;
    // Write the temporary column offsets. We will overwrite those at the end.
    for _ in 0..3 {
        writer.write_all(&0_u32.to_le_bytes())?;
    }
    let mut total_len = 16;
    let mut offsets = vec![];

    for col in [
        &records.nppes_provider_first_name,
        &records.nppes_provider_last_org_name,
        &records.nppes_provider_state,
    ] {
        offsets.push(total_len);
        let data = col.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
        let data_len = col.iter().map(|s| s.len()).sum();
        let encoder = fsst_rs::FsstEncoder::new(&data);
        let header_vec = encoder.export();
        let mut output_buf = vec![0; data_len];
        let mut slices: Vec<&[u8]> = vec![&[]; col.len()];

        let output_count = encoder.compress_into(&data, &mut output_buf, &mut slices);
        assert_eq!(output_count, row_count as usize);

        let mut delta = 0;
        let lens = slices
            .into_iter()
            .flat_map(|s| {
                delta += s.len();
                u32::try_from(delta).unwrap().to_le_bytes()
            })
            .collect::<Vec<_>>();
        let output_len = delta;

        println!("HEADER OFFSET: {total_len}");
        writer.write_all(&header_vec)?;
        total_len += header_vec.len();
        pad(&mut writer, &mut total_len)?;
        println!("OFFSETS OFFSET: {total_len}");
        writer.write_all(&0_u32.to_le_bytes())?;
        writer.write_all(&lens)?;
        total_len += 4 + lens.len();
        pad(&mut writer, &mut total_len)?;
        println!("DATA OFFSET: {total_len}");
        writer.write_all(&output_buf[..output_len])?;
        total_len += output_len;
        pad(&mut writer, &mut total_len)?;
    }

    drop(writer);

    // Now fixup the offsets.
    let mut writer = fs::File::options().write(true).open(output_path)?;
    writer.seek(io::SeekFrom::Start(4))?;
    writer.write_all(&u32::try_from(offsets[0]).unwrap().to_le_bytes())?;
    writer.write_all(&u32::try_from(offsets[1]).unwrap().to_le_bytes())?;
    writer.write_all(&u32::try_from(offsets[2]).unwrap().to_le_bytes())?;

    Ok(())
}

fn map_string(src: Option<&str>) -> String {
    src.and_then(|x| if x == "null" { None } else { Some(x) })
        .map(|x| x.to_string())
        .unwrap_or_default()
}

fn map_cvs_string(src: &str) -> &str {
    if src.is_empty() {
        "NULL"
    } else {
        src
    }
}

fn pad<W: io::Write>(writer: &mut W, total_len: &mut usize) -> Result<()> {
    while *total_len % 64 != 0 {
        writer.write_all(&[0])?;
        *total_len += 1;
    }
    Ok(())
}

fn get_fsst_progress_bar_style() -> indicatif::ProgressStyle {
    use indicatif::ProgressStyle;
    ProgressStyle::with_template(
        " {spinner:.cyan} {prefix} [{elapsed_precise}] {wide_bar} {msg} [{decimal_bytes_per_sec}]",
    )
    .unwrap()
}

fn get_spinner<S: Into<Cow<'static, str>>>(msg: S) -> indicatif::ProgressBar {
    use indicatif::{ProgressBar, ProgressStyle};
    let style = ProgressStyle::with_template(" {spinner:.cyan} {prefix} [{elapsed_precise}] {msg} [{pos}] [{per_sec}]")
        .unwrap();

    let progress = ProgressBar::new_spinner().with_style(style);
    progress.set_prefix(msg);

    progress
}
