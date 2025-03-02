use anyhow::{bail, Result};
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use std::{
    borrow::Cow,
    env, fs,
    io::{self, Seek, Write},
};

#[derive(serde::Deserialize, Debug)]
struct LineItemRow {
    orderkey: i32,
    partkey: i32,
    suppkey: i32,
    linenumber: u8,
    quantity: Decimal,
    extendedprice: Decimal,
    discount: Decimal,
    tax: Decimal,
    returnflag: char,
    linestatus: char,
    shipdate: NaiveDate,
    commitdate: NaiveDate,
    receiptdate: NaiveDate,
    shipinstruct: String,
    shipmode: String,
    comment: String,
}

struct LineItemTable {
    orderkey: Vec<i32>,
    partkey: Vec<i32>,
    suppkey: Vec<i32>,
    linenumber: Vec<u8>,
    quantity: Vec<Decimal>,
    extendedprice: Vec<Decimal>,
    discount: Vec<Decimal>,
    tax: Vec<Decimal>,
    returnflag: Vec<u8>,
    linestatus: Vec<u8>,
    shipdate: Vec<NaiveDate>,
    commitdate: Vec<NaiveDate>,
    receiptdate: Vec<NaiveDate>,
    shipinstruct: Vec<String>,
    shipmode: Vec<String>,
    comment: Vec<String>,
}

/// We read the TPC-H lineitem table and apply various compression to each column.
///     l_orderkey      integer        - RLE
///     l_partkey       integer        - Trunc18 (sc=1), Trunc21 (sc=10)
///     l_suppkey       integer        - Trunc14 (sc=1), Trunc17 (sc=10)
///     l_linenumber    integer        - FSSTBlocked, treated as one long string
///     l_quantity      decimal(12, 2) - IntFor
///     l_extendedprice decimal(12, 2) - IntFor
///     l_discount      decimal(12, 2) - IntFor
///     l_tax           decimal(12, 2) - IntFor
///     l_returnflag    char(1)        - FSSTBlocked, treated as one long string
///     l_linestatus    char(1)        - FSSTBlocked, treated as one long string
///     l_shipdate      date           - IntFor
///     l_commitdate    date           - IntFor
///     l_receiptdate   date           - IntFor
///     l_shipinstruct  char(25)       - FSST
///     l_shipmode      char(10)       - FSST
///     l_comment       varchar(44)    - FSST
/// All columns are non-nullable.
fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 3 || args.len() > 4 {
        bail!("usage: {} INPUT_CSV OUTPUT_PATH", args[0]);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let mut csv = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_path(input_path)?;

    let mut table = LineItemTable::with_capacity(6_000_000); // for sc = 1
    let spinner = get_spinner("reading csv...");

    for result in csv.deserialize() {
        let record: LineItemRow = result?;
        table.push(record);
        spinner.inc(1);
    }

    let mut writer = io::BufWriter::new(fs::File::create(output_path)?);
    let row_count = u32::try_from(table.len()).expect("number of rows to fit in 32 bits");
    let mut total_len = 0;
    writer.write_all(&row_count.to_le_bytes())?;
    total_len += 4;
    // Write the temporary column offsets. We will overwrite those at the end.
    for _ in 0..table.num_cols() {
        writer.write_all(&0_u32.to_le_bytes())?;
        total_len += 4;
    }
    pad(&mut writer, &mut total_len)?;
    let mut offsets = [0; 16];

    offsets[0] = total_len;
    total_len += compress_rle(&table.orderkey, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[1] = total_len;
    total_len += compress_trunc18(&table.partkey, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[2] = total_len;
    total_len += compress_int_for_32(&table.suppkey, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[3] = total_len;
    total_len += compress_fsst_blocked(&table.linenumber, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[4] = total_len;
    total_len += compress_int_for_decimal(&table.quantity, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[5] = total_len;
    total_len += compress_int_for_decimal(&table.extendedprice, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[6] = total_len;
    total_len += compress_int_for_decimal(&table.discount, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[7] = total_len;
    total_len += compress_int_for_decimal(&table.tax, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[8] = total_len;
    total_len += compress_fsst_blocked(&table.returnflag, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[9] = total_len;
    total_len += compress_fsst_blocked(&table.linestatus, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[10] = total_len;
    total_len += compress_int_for_date(&table.shipdate, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[11] = total_len;
    total_len += compress_int_for_date(&table.commitdate, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[12] = total_len;
    total_len += compress_int_for_date(&table.receiptdate, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[13] = total_len;
    total_len += compress_fsst(&table.shipinstruct, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[14] = total_len;
    total_len += compress_fsst(&table.shipmode, &mut writer)?;
    pad(&mut writer, &mut total_len)?;
    offsets[15] = total_len;
    total_len += compress_fsst(&table.comment, &mut writer)?;
    pad(&mut writer, &mut total_len)?;

    // Now fixup the offsets.
    let mut writer = fs::File::options().write(true).open(output_path)?;
    writer.seek(io::SeekFrom::Start(4))?;
    for offset in offsets {
        writer.write_all(&(u32::try_from(offset).expect("offsets to fit in 32 bits")).to_le_bytes())?;
    }

    Ok(())
}

fn compress_trunc18<W: Write>(column: &[i32], output: W) -> Result<usize> {
    Ok(compress::trunc18::compress_32(
        column,
        output,
        get_progress_bar_style(),
    )?)
}

fn compress_int_for_32<W: Write>(column: &[i32], output: W) -> Result<usize> {
    const BLOCK_SIZE: usize = 256;
    Ok(compress::int_for::compress_32(
        column,
        output,
        BLOCK_SIZE,
        get_progress_bar_style(),
    )?)
}

fn compress_int_for_64<W: Write>(column: &[i64], output: W) -> Result<usize> {
    const BLOCK_SIZE: usize = 256;
    Ok(compress::int_for::compress_64(
        column,
        output,
        BLOCK_SIZE,
        get_progress_bar_style(),
    )?)
}

fn compress_int_for_decimal<W: Write>(column: &[Decimal], output: W) -> Result<usize> {
    let ints = column
        .iter()
        .map(|d| i64::try_from(d.mantissa()).expect("decimals to fit in 64 bits"))
        .collect::<Vec<_>>();
    compress_int_for_64(&ints, output)
}

fn compress_int_for_date<W: Write>(column: &[NaiveDate], output: W) -> Result<usize> {
    let ints = column
        .iter()
        .map(|d| d.signed_duration_since(NaiveDateTime::UNIX_EPOCH.date()).num_days())
        .map(|d| i32::try_from(d).expect("dates to fit in 32 bits"))
        .collect::<Vec<_>>();
    compress_int_for_32(&ints, output)
}

fn compress_rle<W: Write>(column: &[i32], output: W) -> Result<usize> {
    Ok(compress::rle::compress(column, output, get_progress_bar_style())?)
}

fn compress_fsst<S: AsRef<[u8]>, W: Write>(column: &[S], output: W) -> Result<usize> {
    const FSST_ITERS: usize = 10;
    Ok(compress::fsst::compress(
        column,
        output,
        FSST_ITERS,
        get_progress_bar_style(),
        None,
    )?)
}

fn compress_fsst_blocked<W: Write>(column: &[u8], output: W) -> Result<usize> {
    const FSST_ITERS: usize = 10;
    const BLOCK_SIZE: usize = 64 * 1024;
    Ok(compress::fsst::compress_blocked(
        column,
        output,
        FSST_ITERS,
        BLOCK_SIZE,
        get_progress_bar_style(),
        None,
    )?)
}

fn pad<W: io::Write>(writer: &mut W, total_len: &mut usize) -> Result<()> {
    while *total_len % 8 != 0 {
        writer.write_all(&[0])?;
        *total_len += 1;
    }
    Ok(())
}

impl LineItemTable {
    pub fn with_capacity(size: usize) -> Self {
        Self {
            orderkey: Vec::with_capacity(size),
            partkey: Vec::with_capacity(size),
            suppkey: Vec::with_capacity(size),
            linenumber: Vec::with_capacity(size),
            quantity: Vec::with_capacity(size),
            extendedprice: Vec::with_capacity(size),
            discount: Vec::with_capacity(size),
            tax: Vec::with_capacity(size),
            returnflag: Vec::with_capacity(size),
            linestatus: Vec::with_capacity(size),
            shipdate: Vec::with_capacity(size),
            commitdate: Vec::with_capacity(size),
            receiptdate: Vec::with_capacity(size),
            shipinstruct: Vec::with_capacity(size),
            shipmode: Vec::with_capacity(size),
            comment: Vec::with_capacity(size),
        }
    }

    pub fn push(&mut self, record: LineItemRow) {
        self.orderkey.push(record.orderkey);
        self.partkey.push(record.partkey);
        self.suppkey.push(record.suppkey);
        self.linenumber.push(record.linenumber);
        self.quantity.push(record.quantity);
        self.extendedprice.push(record.extendedprice);
        self.discount.push(record.discount);
        self.tax.push(record.tax);
        self.returnflag
            .push(u8::try_from(record.returnflag).expect("chars to be ascii"));
        self.linestatus
            .push(u8::try_from(record.linestatus).expect("chars to be ascii"));
        self.shipdate.push(record.shipdate);
        self.commitdate.push(record.commitdate);
        self.receiptdate.push(record.receiptdate);
        self.shipinstruct.push(record.shipinstruct);
        self.shipmode.push(record.shipmode);
        self.comment.push(record.comment);
    }

    pub fn num_cols(&self) -> usize {
        16
    }

    pub fn len(&self) -> usize {
        self.orderkey.len()
    }
}

fn get_progress_bar_style() -> indicatif::ProgressStyle {
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
