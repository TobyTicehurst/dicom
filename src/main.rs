use anyhow::Result;
use dicom_dictionary_std::tags;
use dicom_object::open_file;
use walkdir::WalkDir;
use serde::Serialize;
use std::fs;
use std::io;
use clap::Parser;
use std::path::Path;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long, default_value_t = String::from("output.json"))]
    pub output: String,
}

impl Cli {
    pub fn from_args() -> Self {
        Cli::parse()
    }
}

#[derive(Serialize)]
struct DicomMetadata {
    filepath: String,
    patient_name: String,
    // would normally implement this as a u64 but the DICOM standard implements this as a "long string", therefore String is more appropriate
    patient_id: String,
}

fn parse_dicom_file(filepath: &Path) -> Result<DicomMetadata> {
    match open_file(filepath) {
        Ok(obj) => {
            // TODO - proper error handling
            let patient_name = obj.element(tags::PATIENT_NAME)?.to_str()?.to_string();
            let patient_id = obj.element(tags::PATIENT_ID)?.to_str()?.to_string();
            let filepath = filepath.to_str().unwrap().to_string();
            Ok(DicomMetadata { filepath, patient_name, patient_id })
        }
        // could print a message here, or even further match the dicom_object::ReadError
        Err(e) => Err(e.into()),
    }
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    let mut metadata = vec![];

    // silently skip directories with permission errors (could use partition() if we would like to keep these errors)
    for entry in WalkDir::new(args.input).into_iter().filter_map(|e| e.ok()) {
        // again silently skipping permission errors
        if let Ok(file_metadata) = entry.metadata() && file_metadata.is_file() {
            // silently skip errors, assuming they aren't valid dicom files
            parse_dicom_file(entry.path()).ok().map(|e| metadata.push(e));
        }
    }

    // write to file in json format
    let file = fs::File::create(args.output)?;
    let mut writer = io::BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &metadata)?;

    Ok(())
}
