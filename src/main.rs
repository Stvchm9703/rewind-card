pub mod convert_token;
pub mod parse;
pub mod resolve_token;
pub mod tailwind_token;

// use serde_json;
use std::{fs, path::Path};

use crate::parse::parse_to_tw_token;

fn main() {
    tailwind_token::init();

    let income_src_dir = Path::new("./input-src/");
    let outcome_src_dir = Path::new("./out-tw-token/");

    if outcome_src_dir.exists() == false || outcome_src_dir.is_dir() == false {
        fs::create_dir(outcome_src_dir).ok();
    }

    for entry in income_src_dir.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            // println!("{:?}", entry.path());
            // if let Ok(file_type) = entry.file_type() {
            //     if file_type.is_file() && entry.path().extension().is_some_and(|&f| f.to_str().contains("scss")) {

            //     }
            // }
            // let mut income_src = Path::new("./input-src/cdt-grid-card.css");
            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap_or_default();
            let file_context = fs::read_to_string(&entry_path).unwrap();
            let resolved_token = parse_to_tw_token(&file_context, &file_name.to_str().unwrap());
            let outpath = outcome_src_dir
                .join("s")
                .with_file_name(file_name)
                .with_extension("json");
            println!("outpath: {}", outpath.as_path().display().to_string());
            fs::write(
                outpath.as_path(),
                serde_json::to_string_pretty(&resolved_token).unwrap_or_default(),
            )
            .ok();
        }
    }
    // let mut income_src = Path::new("./input-src/cdt-grid-card.css");
}
