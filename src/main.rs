mod convert_token;
mod tailwind_token;
use crate::convert_token::resolve_style;
use crate::tailwind_token::{init, TailwindTokenSet};
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
    tailwind_token::init();
    let mut income_src = Path::new("./input-src/cdt-navbar.css");
    read_css_file(income_src);
}

fn read_css_file(file_path: &Path) {
    // let fs::read(file_path).unwrap();
    let mut tw_vec: Vec<tailwind_token::TailwindTokenSet> = Vec::new();
    let file_string = fs::read_to_string(file_path).unwrap();
    let parser_set = StyleSheet::parse(&file_string, ParserOptions::default()).unwrap();

    // println!("{}" , serde_json::to_string_pretty(&parser_set).unwrap());

    for rule in parser_set.rules.0 {
        let current_rule= rule.to_css_string(PrinterOptions::default()).unwrap();
        let current_layer = file_path.to_str().unwrap().to_string();
        match rule {
            CssRule::Media(_) => {
                // p.query.
                // for q in p.query.media_queries {
                //     match q.condition.unwrap() {
                //         lightningcss::media_query::MediaCondition::Feature(ss) => {
                //             println!(
                //                 "Feature: {}",
                //                 ss.to_css_string(PrinterOptions::default()).unwrap()
                //             );

                //             match ss {
                //                 lightningcss::media_query::MediaFeature::Plain { name, value } => {
                //                     println!(
                //                         "Plain: {} --- {}",
                //                         name,
                //                         value.to_css_string(PrinterOptions::default()).unwrap()
                //                     );

                //                     match value{
                //                         lightningcss::media_query::MediaFeatureValue::Length(qwe) => {},
                //                         lightningcss::media_query::MediaFeatureValue::Number(_) => {},
                //                         lightningcss::media_query::MediaFeatureValue::Resolution(_) => {},
                //                         lightningcss::media_query::MediaFeatureValue::Ratio(_) => {},
                //                         lightningcss::media_query::MediaFeatureValue::Ident(_) => {},
                //                     }
                //                 }
                //                 lightningcss::media_query::MediaFeature::Boolean(_) => {}
                //                 lightningcss::media_query::MediaFeature::Range {
                //                     name,
                //                     operator,
                //                     value,
                //                 } => {
                //                     println!(
                //                         "Range: {} -- {} --- {}",
                //                         name,
                //                         operator.to_css_string(PrinterOptions::default()).unwrap(),
                //                         value.to_css_string(PrinterOptions::default()).unwrap()
                //                     );
                //                 }
                //                 lightningcss::media_query::MediaFeature::Interval {
                //                     name,
                //                     start,
                //                     start_operator,
                //                     end,
                //                     end_operator,
                //                 } => {}
                //             }
                //         }
                //         lightningcss::media_query::MediaCondition::Not(_) => {}
                //         lightningcss::media_query::MediaCondition::Operation(_, _) => {}
                //         lightningcss::media_query::MediaCondition::InParens(_) => {}
                //     }
                //     if let Some(qualifier) = q.qualifier {
                //         println!(
                //             "qualifier: {}",
                //             qualifier.to_css_string(PrinterOptions::default()).unwrap()
                //         );
                //     }

                //     // match q.media_type {
                //     //     lightningcss::media_query::MediaType::All => {
                //     //     }
                //     //     lightningcss::media_query::MediaType::Print => {},
                //     //     lightningcss::media_query::MediaType::Screen => {},
                //     //     lightningcss::media_query::MediaType::Custom(_) => {},
                //     // }
                //     // println!("media_type : {}" , q.media_type);
                // }
            }
            CssRule::Style(p) => {
                let mut tw_set = tailwind_token::TailwindTokenSet::new();
                tw_set.set_layer_group(current_layer);
                tw_set.set_raw_property(current_rule);
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
