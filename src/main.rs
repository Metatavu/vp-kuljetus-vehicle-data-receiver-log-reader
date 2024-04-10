use std::io::{BufWriter, Write};

use base64::Engine;
use nom_teltonika::{parser::tcp_frame, AVLFrame};

/// [VP-Kuljetus Vehicle Data Receiver](https://www.github.com/metatavu/vp-kuljetus-vehicle-data-receiver) can be configured to write all incoming [`AVLFrame`]s to a file.
///
/// This program reads those files and writes them into a JSON file.
///
/// # Example
/// `vp-kuljetus-vehicle-data-receiver-log-reader input.txt` outputs `input.json`
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Usage: cargo run <filename>");
    }
    let input_filename = &args[1];
    let input_file_contents = std::fs::read_to_string(input_filename)?;
    let output_filename = input_filename.replace(".txt", ".json");
    let output_file = std::fs::File::create(&output_filename)?;

    let mut output_writer = BufWriter::new(output_file);

    let encoded_frames: Vec<&str> = input_file_contents.trim_start().split("\\n").filter(|x| !x.is_empty()).collect();

    println!("Log contains {} frames", encoded_frames.len());
    let mut frames: Vec<AVLFrame> = Vec::new();

    for (index, encoded_frame) in encoded_frames.iter().enumerate() {
        let decoded_frame = match base64::prelude::BASE64_STANDARD.decode(encoded_frame) {
            Ok(decoded) => decoded,
            Err(e) => {
                eprintln!("Error decoding frame: {:?}", e);
                continue;
            }
        };
        let avl_frame = tcp_frame(&decoded_frame);
        match avl_frame {
            Ok((_, frame)) => {
                println!("{}. Frame has {} records", index + 1, frame.records.len());
                frames.push(frame);
            }
            Err(e) => {
                eprintln!("Error parsing frame: {:?}", e);
                continue;
            }
        }
    }
    serde_json::to_writer_pretty(&mut output_writer, &frames)?;
    output_writer.flush()?;


    Ok(())
}