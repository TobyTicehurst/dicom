# Initial Thoughts #

Single threaded approach to get the prototype working as quickly as possible

- dicom-rs for dicom parsing
- clap for cli utilities (cargo run -- --filepath <string-filepath>)
- serde for file serialization and deserialization

Couldn't easily find test data via the links provided and instead used: https://www.rubomedical.com/dicom_files/

Useful dicom file format resource: https://dicom.innolitics.com/ciods/rt-dose/patient/00100010

