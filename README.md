# How to run

`cargo run -- --input <input-directory> --output <output-file> --verbosity <verbosity>`

examples:

`cargo run --release -- --input "./data"`

`cargo run -- --input "./data" --output "output.json" --verbosity debug`

`cargo run -- --input "./data" --verbosity debug > "output.json"`

help:

`cargo run -- --help`

# Explanation

A valid directory must be specified via the `--input` command line argument, using `clap` for argument parsing. This directory is then recursively searched for files using the `walkdir` crate. An attempt to parse each of these files as dicom is then made using the `dicom_*` crates. On failure an error is logged (to stderr) using the `log` and `stderrlog` crates at debug verbosity (specified via `--verbosity debug`). Error handling is made simpler using the `anyhow` crate. On successful parsing of a dicom file, the patient name and id are extracted. These values, along with the filepath are used to create a struct `DicomMetadata`. This struct uses the `serde` and `serde_json` crates to serialize this data into json format. This json is then written to the file specified via `--output`, or to stdout if no file is specified.

# Optimisations

 - `DicomMetadata::from_file_optimised` attempts to read the required data from the file reading as little as possible. In my opinion it is somewhat hacky to use the tag `ISSUER_OF_PATIENT_ID` as a mark of when to stop reading, it's just the only way I could see given limited research time.
 - The file read could maybe be further optimised by only checking the preamble, reading the metadata, then reading only the required tags. I didn't have time to look into the dicom-rs crates further to figure out how to do this.
 - `async_parser` uses tokio tasks and a multithreaded runtime to parallelise the operations. Since there may be many many files I use tasks over threads. Given more time I would implement a solution with Rayon, which I imagine would be faster at this type of parallelised file IO work.
 - Compiling with `--release` of course results in faster code

# Tests

I am not familiar with the ins and outs of the dicom file format so I downloaded and tested with the sample files found here: https://www.rubomedical.com/dicom_files/

# AI disclaimer

No AI was used to assist in this project in any way. All code is my own
