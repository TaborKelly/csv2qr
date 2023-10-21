#[cfg(not(test))]
use log::{debug, warn};
#[cfg(test)]
use std::{println as debug, println as warn};

use std::{fs, path};
use thiserror::Error;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum CsvToQrError {
    #[error("Parsing failure")]
    ParseError,
    #[error("Something weird happened")]
    RuntimeError,
}

#[derive(Debug)]
pub struct Record {
    title: String,
    value: String,
    intermediate_path: path::PathBuf,
    final_path: path::PathBuf,
}

const CALLING_CODE: &[u8] = include_bytes!("resources/CallingCode-Regular.ttf");

fn calc_output_path(title: &str, base_path: &path::Path, extension: &str) -> path::PathBuf {
    let mut path = path::PathBuf::from(base_path);
    let file_name = title.replace(' ', "_");
    let file_name_encoded = urlencoding::encode(&file_name);
    path.push(file_name_encoded.to_string());
    path.set_extension(extension);

    path
}

pub fn parse_records(csv_path: &path::Path, output_base_path: &path::Path) -> Result<Vec<Record>> {
    let mut records = Vec::new();

    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(csv_path)?;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        debug!("{:?}", record);

        if record.len() != 2 {
            return Err(Box::new(CsvToQrError::ParseError));
        }

        let title = record[0].trim().to_string();
        records.push(Record {
            title: title.clone(),
            value: record[1].trim().to_string(),
            intermediate_path: calc_output_path(&title, output_base_path, "png"),
            final_path: calc_output_path(&title, output_base_path, "pdf"),
        })
    }

    Ok(records)
}

pub fn generate_qrs(records: &Vec<Record>, ecc_level: qrcode_generator::QrCodeEcc) -> Result<()> {
    for r in records {
        debug!("processing {:?}", r);

        qrcode_generator::to_png_to_file(r.value.clone(), ecc_level, 1024, &r.intermediate_path)?;
    }

    Ok(())
}

pub fn generate_pdf(records: &Vec<Record>, save_intermediate: bool) -> Result<()> {
    let font_data = genpdf::fonts::FontData::new(CALLING_CODE.to_vec(), None)?;
    let font_family = genpdf::fonts::FontFamily {
        regular: font_data.clone(),
        bold: font_data.clone(),
        italic: font_data.clone(),
        bold_italic: font_data,
    };

    for r in records {
        // Create a document and set the default font family
        let mut doc = genpdf::Document::new(font_family.clone());
        // Change the default settings
        doc.set_title(r.title.clone());
        // Customize the pages
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(20);
        doc.set_page_decorator(decorator);
        // Add one or more elements

        let image = genpdf::elements::Image::from_path(&r.intermediate_path)
            .expect("Failed to load image")
            .with_alignment(genpdf::Alignment::Center); // Center the image on the page.
        doc.push(image);

        let mut label = genpdf::elements::Paragraph::new(r.title.clone());
        label.set_alignment(genpdf::Alignment::Center); // Center the image on the page.
        doc.push(label);

        // Render the document and write it to a file
        doc.render_to_file(&r.final_path)
            .expect("Failed to write PDF file");

        if !save_intermediate {
            match fs::remove_file(&r.intermediate_path) {
                Ok(_) => {}
                Err(e) => {
                    warn!("failed to delete {:?}, {}", r.intermediate_path, e);
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_records() -> Result<()> {
        let mut example_path = project_root::get_project_root()?;
        example_path.push("example/example.csv");

        let output_base_path = project_root::get_project_root()?;

        let records = parse_records(&example_path, &output_base_path)?;
        assert_eq!(4, records.len());
        assert_eq!(records[0].title, "Hack the planet");
        assert_eq!(records[0].value, "https://youtu.be/u3CKgkyc7Qo");
        assert_eq!(records[1].title, "Prodigy - Mind Fields");
        assert_eq!(records[1].value, "https://youtu.be/7mKieArPRkw");
        assert_eq!(records[2].title, "I am not a martyr I'm a problem");
        assert_eq!(
            records[2].value,
            "https://youtu.be/7Azv0G85lh8?si=awP06dwWDUcuOBaD&t=46"
        );
        assert_eq!(records[3].title, "This is what I do");
        assert_eq!(records[3].value, "https://youtu.be/YPL41OkVABk");

        Ok(())
    }

    #[test]
    fn test_generate_qrs() -> Result<()> {
        let mut example_path = project_root::get_project_root()?;
        example_path.push("example/example.csv");

        let tmp_dir = tempdir::TempDir::new("csv2qr")?;

        let records = parse_records(&example_path, &tmp_dir.path())?;
        let ecc_levels = [
            qrcode_generator::QrCodeEcc::Low,
            qrcode_generator::QrCodeEcc::Medium,
            qrcode_generator::QrCodeEcc::High,
            qrcode_generator::QrCodeEcc::Quartile,
        ];
        for ecc in ecc_levels {
            generate_qrs(&records, ecc)?;

            for r in &records {
                let img = image::open(&r.intermediate_path)?;

                // Use default decoder
                let decoder = bardecoder::default_decoder();

                let results = decoder.decode(&img);
                assert_eq!(1, results.len());
                assert_eq!(r.value, *results[0].as_ref().unwrap());
            }
        }

        Ok(())
    }
}
