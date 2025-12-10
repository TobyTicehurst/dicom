use anyhow::Result;
use log::debug;
use std::fs;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tokio::{main, task};
use walkdir::WalkDir;

mod dicom;
mod util;

use dicom::DicomMetadata;
use util::Cli;

#[allow(dead_code)]
fn sync_parser(input_directory: String) -> Vec<DicomMetadata> {
    let mut metadata_list = vec![];

    // silently skip directories with permission errors (could use partition() if we would like to keep these errors)
    for entry in WalkDir::new(input_directory).into_iter().filter_map(|e| {
        e.map_err(|err| debug!("Failed to parse filepath: {err}"))
            .ok()
    }) {
        // again silently skipping permission errors
        if let Ok(file_metadata) = entry.metadata()
            && file_metadata.is_file()
        {
            // debug logging any errors as a demo of how I would log the previously ignored errors if needed
            // since the requirements are to gracefully handle errors, we should continue not return on error
            match DicomMetadata::from_file_optimised(entry.path()) {
                Ok(metadata) => metadata_list.push(metadata),
                Err(err) => debug!("Failed to parse file: {err}"),
            }
        }
    }

    metadata_list
}

#[allow(dead_code)]
async fn async_parser(input_directory: String) -> Vec<DicomMetadata> {
    let mut join_handles = vec![];
    let metadata_list: Arc<Mutex<Vec<DicomMetadata>>> = Arc::new(Mutex::new(vec![]));

    // silently skip directories with permission errors (could use partition() if we would like to keep these errors)
    for entry in WalkDir::new(input_directory).into_iter().filter_map(|e| {
        e.map_err(|err| debug!("Failed to parse filepath: {err}"))
            .ok()
    }) {
        // again silently skipping permission errors
        if let Ok(file_metadata) = entry.metadata()
            && file_metadata.is_file()
        {
            let metadata_list_clone = metadata_list.clone();
            join_handles.push(task::spawn(async move {
                // debug logging any errors as a demo of how I would log the previously ignored errors if needed
                // since the requirements are to gracefully handle errors, we should continue not return on error
                match DicomMetadata::from_file_optimised(entry.path()) {
                    Ok(metadata) => metadata_list_clone.lock().unwrap().push(metadata),
                    Err(err) => debug!("Failed to parse file: {err}"),
                }
            }));
        }
    }

    for handle in join_handles {
        handle.await.unwrap();
    }

    metadata_list.lock().unwrap().clone()
}

#[main]
async fn main() -> Result<()> {
    let args = Cli::from_args();

    // only debug logs are implemented
    // error logging is being handled by returning anyhow::Result from main - only due to needing to code this quickly
    stderrlog::new().verbosity(args.verbosity).init().unwrap();

    // start timer
    let now = SystemTime::now();

    // async parser ends up being around 4 times faster in my own testing
    //let metadata_list = sync_parser(args.input);
    let metadata_list = async_parser(args.input).await;

    // end timer
    let time = now.elapsed()?;
    debug!("Time to parse: {} microseconds", time.as_micros());

    // write to file or stdout in json format
    let writer: Box<dyn io::Write> = match args.output {
        Some(s) => Box::new(fs::File::create(s)?),
        None => Box::new(io::stdout()),
    };

    let mut writer = io::BufWriter::new(writer);
    serde_json::to_writer_pretty(&mut writer, &metadata_list)?;

    Ok(())
}
