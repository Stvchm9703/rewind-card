pub mod convert_token;
pub mod parse;
pub mod resolve_token;
pub mod tailwind_token;

// use serde_json;
use crate::parse::parse_to_tw_token;
use rayon::prelude::*;
// use serde_json::{Result, Value};
use std::{
    // collections::HashMap,
    fs,
    path::Path,
};

fn main() {
    tailwind_token::init();

    let income_src_dir = Path::new("./input-src/");
    let outcome_src_dir = Path::new("./out-tw-token/");

    if outcome_src_dir.exists() == false || outcome_src_dir.is_dir() == false {
        fs::create_dir(outcome_src_dir).ok();
    }

    // for entry in income_src_dir.read_dir().expect("read_dir call failed") {
    let file_list: _ = income_src_dir
        .read_dir()
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect::<Vec<_>>();

    file_list.par_iter().for_each(|entry| {
        let file_name = entry.file_name().unwrap_or_default();
        let file_context = fs::read_to_string(&entry).unwrap();

        if entry.extension().unwrap() == "scss" || entry.extension().unwrap() == "scss" {
            
        }
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
        // }
    });
    // let mut income_src = Path::new("./input-src/cdt-grid-card.css");
}

// fn main() {
//     let ctx = fs::read_to_string("./preset/unocss.config.json").unwrap();

//     let mut lookup: HashMap<String, Value> = serde_json::from_str(&ctx).unwrap();

//     // println!("lookup , {}",lookup);
//     for (key , value) in lookup{
//         println!("key , {} , value : {}" , key, value.to_string());
//     }
//     // let y = json_to_hashmap(&ctx, []);
// }

// fn json_to_hashmap(json: &str, keys: Vec<&str>) -> Result<HashMap<String, Value>> {
//     let mut lookup: HashMap<String, Value> = serde_json::from_str(json).unwrap();
//     let mut map = HashMap::new();
//     for key in keys {
//         let (k, v) = lookup.remove_entry(key).unwrap();
//         map.insert(k, v);
//     }
//     Ok(map)
// }
