use std::{fs::{create_dir_all, read_to_string, File}, io::{stdin, BufWriter, IsTerminal, Read, Write}, path::Path};
use base64::{prelude::BASE64_STANDARD, Engine};
use clap::{arg, command, Parser};
use nom_teltonika::AVLFrame;
use chrono::{Local, Timelike};

const LOGS_FILE_NAME: &str = "frames.json";

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

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    output: Option<String>,
    #[arg(short, long)]
    input: Option<String>,
}
fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let now = Local::now();
    let default_output_dir = format!("vp-kuljetus-logs/{}", now.format("%Y-%m-%d-%H-%M-%S"));

    let output_dir = args.output.unwrap_or_else(|| default_output_dir);
    let output_dir = Path::new(&output_dir);

    create_dir_all(output_dir)?;

    let input_contents = match args.input {
        Some(file_name) => read_to_string(file_name),
        None => {
            if stdin().is_terminal() {
                panic!("No input file provided and no input detected from stdin");
            }
            let mut buffer = Vec::new();
            stdin().lock().read_to_end(&mut buffer)?;
            Ok(String::from_utf8(buffer).unwrap())
        }
    };
    let input_contents = match input_contents  {
        Ok(input_contents) => input_contents,
        Err(error) => panic!("Error reading input: {:?}", error)
    };

    let mut frames = Vec::new();
    let mut records = Vec::new();

    let encoded_frames = input_contents.trim_start().split("\\n").filter(|x| !x.is_empty());

    for encoded_frame in encoded_frames {
        let decoded_frame = match BASE64_STANDARD.decode(encoded_frame) {
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

    for record in &records {
        let hour = record.timestamp.hour();
        let filename = Path::new(&record.timestamp.format("%H:%M:%S.%3f").to_string()).with_extension("json");
        let record_dir_path = output_dir.join(&hour.to_string());
        let record_file_path = record_dir_path.join(&filename);
        create_dir_all(record_dir_path).unwrap();
        let mut file = File::create(record_file_path).unwrap();
        file.write(serde_json::to_string_pretty(&record).unwrap().as_bytes()).unwrap();
    };

    let output_file = File::create(output_dir.join(LOGS_FILE_NAME))?;
    let mut output_writer = BufWriter::new(output_file);

    serde_json::to_writer_pretty(&mut output_writer, &frames)?;
    output_writer.flush()?;

    println!("Wrote {} records to {}", records.len(), "vp-kuljetus-logs");
    Ok(())
}