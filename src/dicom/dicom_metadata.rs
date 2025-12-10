use anyhow::{Context, Result};
use dicom_dictionary_std::tags;
use dicom_object::OpenFileOptions;
use dicom_object::open_file;
use serde::Serialize;
use std::path::Path;

#[derive(Serialize, Clone)]
pub struct DicomMetadata {
    filepath: String,
    patient_name: String,
    // would normally implement this as a u64 but the DICOM standard implements this as a "long string", therefore String is more appropriate
    patient_id: String,
}

impl DicomMetadata {
    #[allow(dead_code)]
    pub fn from_file(filepath: &Path) -> Result<Self> {
        match open_file(filepath) {
            Ok(obj) => {
                let patient_name = obj.element(tags::PATIENT_NAME)?.to_str()?.to_string();
                let patient_id = obj.element(tags::PATIENT_ID)?.to_str()?.to_string();
                // will only see this error if filepaths are allowed to be invalid utf-8
                let filepath = filepath
                    .to_str()
                    .context("Failed to parse filepath to str")?
                    .to_string();
                Ok(Self {
                    filepath,
                    patient_name,
                    patient_id,
                })
            }
            Err(e) => Err(e.into()),
        }
    }

    #[allow(dead_code)]
    pub fn from_file_optimised(filepath: &Path) -> Result<Self> {
        let file = OpenFileOptions::new()
            .read_until(tags::ISSUER_OF_PATIENT_ID)
            .open_file(filepath)?;

        let patient_name = file.element(tags::PATIENT_NAME)?.to_str()?.to_string();
        let patient_id = file.element(tags::PATIENT_ID)?.to_str()?.to_string();
        // will only see this error if filepaths are allowed to be invalid utf-8
        let filepath = filepath
            .to_str()
            .context("Failed to parse filepath to str")?
            .to_string();
        Ok(Self {
            filepath,
            patient_name,
            patient_id,
        })
    }
}
