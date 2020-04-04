extern crate app_dirs;
extern crate blake2b_simd;
extern crate calamine;
extern crate config;
extern crate csv;
extern crate docx;
extern crate exif;
extern crate image;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate msoffice_pptx;
extern crate msoffice_shared;
extern crate notify;
extern crate pdf_extract;
extern crate regex;
extern crate reverse_geocoder;
extern crate serde_derive;
extern crate serde_json;
extern crate simple_logger;
extern crate tantivy;
extern crate tract_tensorflow;

pub mod indexers;
pub mod query_executor;
pub mod tantivy_process;

mod file_watcher;

mod tantivy_api;
