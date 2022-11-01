mod convert_token;
mod resolve_token;
mod tailwind_token;
use crate::convert_token::resolve_style;
use crate::tailwind_token::{search_media, TailwindTokenSet};
use lightningcss::media_query::{MediaFeatureValue, MediaQuery};
use lightningcss::{
    media_query::{MediaCondition, MediaFeature},
    rules::CssRule,
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    traits::ToCss,
};

use serde_json;
use std::{fs, path::Path};
fn main() {
    tailwind_token::init();
    let mut income_src = Path::new("./input-src/cdt-grid-card.css");
    read_css_file(income_src);
}

fn read_css_file(file_path: &Path) {
    // let fs::read(file_path).unwrap();
    let mut tw_vec: Vec<TailwindTokenSet> = Vec::new();
    let file_string = fs::read_to_string(file_path).unwrap();
    let parser_set = StyleSheet::parse(&file_string, ParserOptions::default()).unwrap();

    // println!("{}" , serde_json::to_string_pretty(&parser_set).unwrap());

    for rule in parser_set.rules.0 {
        let current_rule = rule.to_css_string(PrinterOptions::default()).unwrap();
        let current_layer = file_path.to_str().unwrap().to_string();
        match rule {
            CssRule::Media(m) => {
              
                // p.query.
                let mut mq_token: Vec<String> = vec![];
                for q in m.query.media_queries {
                    let ext = resolve_media_query_prefix(q);
                    mq_token.extend_from_slice(&ext);
                }
                
                for p in m.rules.0 {
                    if let CssRule::Style(s) = p {
                        let mut tw_set = TailwindTokenSet::new();
                        tw_set.set_layer_group(&current_layer);
                        tw_set.set_raw_property(&current_rule);
                        resolve_style(&s, &mut tw_set);
                        tw_vec.push(tw_set);
                    }
                }
            }
            CssRule::Style(p) => {
                let mut tw_set = TailwindTokenSet::new();
                tw_set.set_layer_group(&current_layer);
                tw_set.set_raw_property(&current_rule);
                resolve_style(&p, &mut tw_set);
                tw_vec.push(tw_set);
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
    }

    println!("{}", serde_json::to_string_pretty(&tw_vec).unwrap());
}

fn resolve_media_query_prefix(q: MediaQuery) -> Vec<String> {
    let mut temp: Vec<String> = vec![];
    let mut resolve_name = String::new();
    if let MediaCondition::Feature(ss) = q.condition.unwrap() {
        if let MediaFeature::Plain { name, value } = ss {
            resolve_name = name.to_string();
            let mut rem_value = -1f32;
            if let MediaFeatureValue::Length(qwe) = value {
                if let lightningcss::values::length::Length::Value(v) = qwe {
                    // let u = v.to_px();
                    let (vv, unit) = v.to_unit_value();
                    // println!("vv {} , unit {}", vv, unit);
                    if unit.to_lowercase().contains("em") {
                        rem_value = vv;
                    } else {
                        let u = v.to_px().unwrap_or_default();
                        rem_value = (u / 16f32).round();
                    }
                }
            }
            let yyy = search_media(&resolve_name, &rem_value);
            temp.extend_from_slice(&yyy);
        }
    }

    return temp;
}
