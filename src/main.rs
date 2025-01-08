use std::{fs::{create_dir_all, OpenOptions}, io::{BufWriter, Write}, path::Path};

use base64::Engine;
use nom_teltonika::AVLFrame;
use chrono::Timelike;

/// [VP-Kuljetus Vehicle Data Receiver](https://www.github.com/metatavu/vp-kuljetus-vehicle-data-receiver) can be configured to write all incoming [`AVLFrame`]s to a file.
///
/// This program reads those files and writes them as JSON files in a directory structure based on the timestamp of each record.
///
/// # Example
/// `vp-kuljetus-vehicle-data-receiver-log-reader input.txt`
///
/// # Output
/// ```
/// - input
///  ├── 0
///     ├── {hour}:{minute}:{second}.{millisecond}.json
///  ├── 1
///     ├── {hour}:{minute}:{second}.{millisecond}.json
///  |--- input.json
/// ```
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Usage: cargo run <filename>");
    }
    let input_filename = &args[1];
    let input_file_contents = std::fs::read_to_string(input_filename)?;
    let output_dirname = input_filename.replace(".txt", "");
    create_dir_all(output_dirname.clone())?;
    let output_filename = Path::new(&output_dirname).join(input_filename.replace(".txt", ".json"));
    let output_file = std::fs::File::create(&output_filename)?;

    let mut output_writer = BufWriter::new(output_file);

    let encoded_frames: Vec<&str> = input_file_contents.trim_start().split("\\n").filter(|x| !x.is_empty()).collect();

    let mut frames = Vec::new();
    let mut records = Vec::new();

    for encoded_frame in encoded_frames.iter() {
        let decoded_frame = match base64::prelude::BASE64_STANDARD.decode(encoded_frame) {
            Ok(decoded) => decoded,
            Err(e) => {
                eprintln!("Error decoding frame: {:?}", e);
                continue;
            }
        };
        match serde_json::from_slice::<AVLFrame>(&decoded_frame) {
            Ok(frame) => {
                records.append(&mut frame.records.clone());
                frames.push(frame);
            }
            Err(e) => {
                eprintln!("Error parsing frame: {:?}", e);
                continue;
            }
        }
    }

    records.iter().for_each(|record| {
        let dirname = output_dirname.clone();
        let hour = record.timestamp.hour();
        let filename = record.timestamp.format("%H:%M:%S.%3f").to_string();
        let record_dir_path = format!("{}/{}", dirname, hour);
        let record_file_path = format!("{}/{}.json", record_dir_path, filename);
        create_dir_all(record_dir_path).unwrap();
        let mut file = OpenOptions::new().create(true).write(true).open(record_file_path).unwrap();
        file.write(serde_json::to_string_pretty(record).unwrap().as_bytes()).unwrap();
    });

    serde_json::to_writer_pretty(&mut output_writer, &frames)?;
    output_writer.flush()?;

    Ok(())
}