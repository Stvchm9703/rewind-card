mod convert_token;
mod tailwind_token;
use crate::convert_token::resolve_style;
use crate::tailwind_token::TailwindTokenSet;
use lightningcss::{
    properties::Property,
    rules::{style::StyleRule, CssRule},
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    traits::ToCss,
    values::{
        calc::Calc,
        length::{LengthPercentage, LengthPercentageOrAuto, LengthValue},
        percentage::{DimensionPercentage, Percentage},
    },
};
use serde_json;
use std::{fs, path::Path};
fn main() {
    let mut income_src = Path::new("./input-src/button.css");
    read_css_file(income_src);
}

fn read_css_file(file_path: &Path) {
    // let fs::read(file_path).unwrap();
    let mut tw_vec: Vec<tailwind_token::TailwindTokenSet> = Vec::new();
    let file_string = fs::read_to_string(file_path).unwrap();
    let parser_set = StyleSheet::parse(&file_string, ParserOptions::default()).unwrap();

    // println!("{}" , serde_json::to_string_pretty(&parser_set).unwrap());

    for rule in parser_set.rules.0 {
        let mut tw_set = tailwind_token::TailwindTokenSet::new();
        tw_set.set_layer_group(file_path.to_str().unwrap().to_string());
        tw_set.set_raw_property(rule.to_css_string(PrinterOptions::default()).unwrap());
        match rule {
            // CssRule::Media(p) => {}
            CssRule::Style(p) => {
                println!("selectors : {}", p.selectors);
                resolve_style(&p, &mut tw_set);
            }
            // CssRule::Import(_) => todo!(),
            // CssRule::Keyframes(_) => todo!(),
            // CssRule::FontFace(_) => todo!(),
            // CssRule::FontPaletteValues(_) => todo!(),
            // CssRule::Page(_) => todo!(),
            // CssRule::Supports(_) => todo!(),
            // CssRule::CounterStyle(_) => todo!(),
            // CssRule::Namespace(_) => todo!(),
            // CssRule::MozDocument(_) => todo!(),
            // CssRule::Nesting(_) => todo!(),
            // CssRule::Viewport(_) => todo!(),
            // CssRule::CustomMedia(_) => todo!(),
            // CssRule::LayerStatement(_) => todo!(),
            // CssRule::LayerBlock(_) => todo!(),
            // CssRule::Property(_) => todo!(),
            // CssRule::Container(_) => todo!(),
            // CssRule::Ignored => todo!(),
            // CssRule::Unknown(_) => todo!(),
            _ => {}
        }

        tw_vec.push(tw_set);
    }

    println!("{}", serde_json::to_string_pretty(&tw_vec).unwrap());
}
