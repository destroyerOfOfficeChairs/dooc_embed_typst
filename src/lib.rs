use chrono::Local;
use std::collections::HashMap;
use typst::Library;
use typst::LibraryExt;
use typst::World;
use typst::diag::FileError;
use typst::foundations::{Bytes, Datetime, Dict, Value};
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;

pub fn compile(
    code: String,
    inputs: HashMap<String, Value>,
    files: HashMap<String, Vec<u8>>,
    fonts: Vec<Vec<u8>>,
) -> Result<Vec<u8>, String> {
    let world = DoocWorld::new(code, fonts, inputs, files);

    let document = match typst::compile(&world).output {
        Ok(doc) => doc,
        Err(errors) => {
            let mut error_msg = String::new();
            for error in errors {
                error_msg.push_str(&format!("Error: {}\n", error.message));
            }
            return Err(error_msg);
        }
    };

    match typst_pdf::pdf(&document, &typst_pdf::PdfOptions::default()) {
        Ok(bytes) => Ok(bytes),
        Err(_) => Err("Failed to export PDF internally".to_string()),
    }
}

pub struct DoocWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    source: Source,
    time: time::OffsetDateTime,
    files: HashMap<String, Bytes>,
}

impl DoocWorld {
    pub fn new(
        source_code: String,
        font_data: Vec<Vec<u8>>,
        inputs: HashMap<String, Value>,
        files: HashMap<String, Vec<u8>>,
    ) -> Self {
        let mut input_dict = Dict::new();
        for (key, value) in inputs {
            input_dict.insert(key.into(), value);
        }

        let library = Library::builder().with_inputs(input_dict).build();

        let mut files_map = HashMap::new();
        for (filename, data) in files {
            files_map.insert(filename, Bytes::new(data));
        }

        let mut fonts = Vec::new();
        for data in font_data {
            let buffer = Bytes::new(data);
            for i in 0.. {
                match Font::new(buffer.clone(), i) {
                    Some(font) => fonts.push(font),
                    None => break,
                }
            }
        }

        let book = FontBook::from_fonts(&fonts);
        let source = Source::detached(source_code);
        let now = Local::now();
        let time = time::OffsetDateTime::from_unix_timestamp(now.timestamp()).unwrap();

        Self {
            library: LazyHash::new(library),
            book: LazyHash::new(book),
            fonts,
            source,
            time,
            files: files_map,
        }
    }
}

impl World for DoocWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }
    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }
    fn main(&self) -> FileId {
        self.source.id()
    }
    fn source(&self, id: FileId) -> Result<Source, FileError> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            let path = id.vpath().as_rootless_path();
            let path_str = path.to_string_lossy().to_string();

            if let Some(bytes) = self.files.get(&path_str) {
                let text = std::str::from_utf8(bytes)
                    .map_err(|_| FileError::InvalidUtf8)?
                    .to_string();

                Ok(Source::new(id, text))
            } else {
                Err(FileError::NotFound(path.to_path_buf()))
            }
        }
    }
    fn file(&self, id: FileId) -> Result<Bytes, FileError> {
        let path = id.vpath().as_rootless_path();
        let path_str = path.to_string_lossy().to_string();
        if let Some(bytes) = self.files.get(&path_str) {
            Ok(bytes.clone())
        } else {
            Err(FileError::NotFound(path.to_path_buf()))
        }
    }
    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }
    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        Some(Datetime::Date(self.time.date()))
    }
}
