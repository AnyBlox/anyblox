use std::{
    error::Error,
    fs::File,
    io::{BufReader, ErrorKind, Read},
    sync::Arc,
};

use arrow::{
    array::{Int32Array, RecordBatch},
    datatypes::{DataType, Field, Schema},
    ipc,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<_>>();

    let input_path = &args[1];
    let output_path = &args[2];

    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);
    let mut ints = vec![];
    let mut buf = [0; 4];

    loop {
        match reader.read_exact(&mut buf) {
            Ok(_) => {
                let int = i32::from_le_bytes(buf);
                ints.push(int);
            }
            Err(err) if err.kind() == ErrorKind::UnexpectedEof => {
                break;
            }
            Err(err) => return Err(err.into()),
        }
    }

    let schema = Schema::new(vec![Field::new("col", DataType::Int32, false)]);
    let array = Int32Array::from_iter_values(ints[..].iter().copied());

    let output_file = File::create(output_path)?;
    let mut writer = ipc::writer::StreamWriter::try_new(output_file, &schema)?;
    let batch = RecordBatch::try_new(Arc::new(schema), vec![Arc::new(array)]).unwrap();
    writer.write(&batch)?;
    writer.finish()?;

    Ok(())
}
