use std::fmt::Debug;

use tesseract::command::{OutputFormat, Tesseract, TesseractCommand};

use crate::format::ImageFormat;

pub struct IdVerif<'a> {
    pub image_format: ImageFormat,
    pub path: &'a str,
    pub output_format: OutputFormat,
    pub information_tesseract: InformationTessract,
}
impl Default for IdVerif<'_> {
    fn default() -> Self {
        IdVerif {
            image_format: ImageFormat::PNG,
            path: "",
            output_format: OutputFormat::StdOut,
            information_tesseract: InformationTessract::default(),
        }
    }
}
pub enum InformationTessract {
    Data {
        command: Option<TesseractCommand>,
        engine: Tesseract,
    },
}
impl Default for InformationTessract {
    fn default() -> Self {
        InformationTessract::Data {
            command: None,
            engine: Tesseract::default(),
        }
    }
}
impl Debug for InformationTessract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InformationTessract::Data { command, engine } => f
                .debug_struct("Data")
                .field("command", &command)
                .field("engine", &engine)
                .finish(),
        }
    }
}
impl InformationTessract {
    pub fn extract_data(&self) -> (&Option<TesseractCommand>, &Tesseract) {
        match &self {
            InformationTessract::Data { command, engine } => (command, engine),
        }
    }
}
impl<'a> IdVerif<'a> {
    pub fn new(data: IdVerif<'a>) -> Self {
        Self {
            image_format: data.image_format,
            path: &data.path,
            output_format: data.output_format,
            information_tesseract: data.information_tesseract,
        }
    }

    pub fn get_data_from_img(&self) -> Result<Result<String, String>, String> {
        if self.path.is_empty() {
            return Err("this must be initialized with valid picture path image".to_string());
        }
        // todo! better program
        let (_command, tess) = self.information_tesseract.extract_data();

        let image_to_str = tess.image_to_text(&self.path, self.output_format.clone());

        Ok(image_to_str)
    }
}

pub fn image_to_string(verif: IdVerif) -> Result<Result<String, String>, String> {
    verif.get_data_from_img()
}
