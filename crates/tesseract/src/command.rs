use std::{fmt::Debug, process::Command};

pub struct Tesseract {
    pub data: TesseractCommand,
}

impl Default for Tesseract {
    fn default() -> Self {
        Tesseract {
            data: TesseractCommand::default(),
        }
    }
}

impl Debug for Tesseract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tesseract")
            .field("tesseract_command", &self.data)
            .finish()
    }
}
#[derive(Clone)]
pub enum OutputFormat {
    StdOut,
    Output,
    Pdf,
}
impl Debug for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OutputFormat::Output => write!(f, "output"),
            OutputFormat::Pdf => write!(f, "pdf"),
            OutputFormat::StdOut => write!(f, "stdout"),
        }
    }
}
impl From<OutputFormat> for String {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Output => "output".to_string(),
            OutputFormat::StdOut => "stdout".to_string(),
            OutputFormat::Pdf => format!("pdf"),
        }
    }
}
impl Tesseract {
    pub fn image_to_text(&self, image_path: &str, format: OutputFormat) -> Result<String, String> {
        if image_path.is_empty() {
            return Err("image path can't be empty".to_string());
        };
        let command_args = self.data.to_command_args();

        let command = Command::new("tesseract")
            .arg(image_path)
            // .arg("output")
            .arg(String::from(format))
            .arg(command_args)
            .output();

        match command {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&output.stderr).to_string())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
pub struct TesseractCommand {
    pub language: Option<String>,
    pub psm: Option<u8>,
    pub oem: Option<u8>,
    pub output_format: Option<String>,
    pub config_file: Option<String>,
    pub user_patterns: Option<String>,
    pub user_words: Option<String>,
    pub lang_path: Option<String>,
}
impl Debug for TesseractCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TesseractCommand")
            .field("language", &self.language)
            .field("psm", &self.psm)
            .field("oem", &self.oem)
            .field("output_format", &self.output_format)
            .field("config_file", &self.config_file)
            .field("user_patterns", &self.user_patterns)
            .field("user_words", &self.user_words)
            .field("lang_path", &self.lang_path)
            .finish()
    }
}
impl Default for TesseractCommand {
    fn default() -> Self {
        TesseractCommand {
            language: None,
            psm: None,
            oem: None,
            output_format: None,
            config_file: None,
            user_patterns: None,
            user_words: None,
            lang_path: None,
        }
    }
}

impl TesseractCommand {
    pub fn new(
        language: Option<String>,
        psm: Option<u8>,
        oem: Option<u8>,
        output_format: Option<String>,
        config_file: Option<String>,
        user_patterns: Option<String>,
        user_words: Option<String>,
        lang_path: Option<String>,
    ) -> Self {
        TesseractCommand {
            language,
            psm,
            oem,
            output_format,
            config_file,
            user_patterns,
            user_words,
            lang_path,
        }
    }

    pub fn to_command_args(&self) -> String {
        let mut command = String::new();

        if let Some(lang) = &self.language {
            command.push_str(&format!(" -l {}", lang));
        }
        if let Some(psm) = self.psm {
            command.push_str(&format!(" --psm {}", psm));
        }
        if let Some(oem) = self.oem {
            command.push_str(&format!(" --oem {}", oem));
        }
        if let Some(format) = &self.output_format {
            command.push_str(&format!(" -c tessedit_create_{}", format));
        }
        if let Some(config) = &self.config_file {
            command.push_str(&format!(" {}", config));
        }
        if let Some(patterns) = &self.user_patterns {
            command.push_str(&format!(" -c user_patterns_file={}", patterns));
        }
        if let Some(words) = &self.user_words {
            command.push_str(&format!(" -c user_words_file={}", words));
        }

        command.trim().to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_to_text() {
        let command = TesseractCommand::new(None, None, None, None, None, None, None, None);

        let tess = Tesseract { data: command };

        let result = tess
            .image_to_text("Capture.PNG", OutputFormat::Output)
            .unwrap();
        dbg!(result);
    }
}
