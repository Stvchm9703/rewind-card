use crate::convert_token::resolve_style;
use crate::resolve_token::resolve_media_query_prefix;
use crate::tailwind_token::TailwindTokenSet;
use lightningcss::{
    rules::{style::StyleRule, CssRule},
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    traits::ToCss,
    // properties::Property,
};
// use serde_json;

use grass;

pub fn parse_scss_to_css(file_context: String) -> String {
    if file_context.contains(r"(\/deep\/|\:\:v-deep)") {}
    let y = grass::from_string(file_context, &grass::Options::default());
    return y.unwrap_or_default();
}
pub fn parse_to_tw_token(file_context: &str, layer: &str) -> Vec<TailwindTokenSet> {
    // let fs::read(file_path).unwrap();
    let mut tw_vec: Vec<TailwindTokenSet> = Vec::new();
    let parser_set = StyleSheet::parse(&file_context, ParserOptions::default()).unwrap();

    // println!("{}" , serde_json::to_string_pretty(&parser_set).unwrap());

    for rule in parser_set.rules.0 {
        let current_rule = rule.to_css_string(PrinterOptions::default()).unwrap();
        let current_layer = layer.to_owned();
        match rule {
            CssRule::Media(m) => {
                // p.query.
                let mut mq_token: Vec<String> = vec![];
                for q in m.query.media_queries {
                    // println!(", {:?}", q);
                    let ext = resolve_media_query_prefix(q);
                    // print!("{:#?} ", ext);
                    mq_token.extend_from_slice(&ext);
                }
                let sub_property_count = m.rules.0.len() as i32;
                for p in m.rules.0 {
                    if let CssRule::Style(s) = p {
                        let mut tw_set = create_new_tw_token(&s, &current_rule, &current_layer);
                        tw_set.set_raw_property_count(sub_property_count);
                        tw_set.push_media_queries(&mq_token);

                        // println!("{} part, ", tw_set.involved_classnames.join(" "),);
                        tw_vec.push(tw_set);
                    }
                }
            }
            CssRule::Style(p) => {
                let tw_set = create_new_tw_token(&p, &current_rule, &current_layer);

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

    // println!("{}", serde_json::to_string_pretty(&tw_vec).unwrap());
    tw_vec
}

fn create_new_tw_token(p: &StyleRule, current_rule: &str, current_layer: &str) -> TailwindTokenSet {
    let mut tw_set = TailwindTokenSet::new();
    tw_set.push_involved_classnames(
        p.selectors
            .to_string()
            .split(", ")
            .map(|f| f.to_owned())
            .collect(),
    );
    tw_set.set_layer_group(current_layer);
    tw_set.set_raw_property(current_rule);
    // let property_count = p.declarations.declarations.len() as i32;
    tw_set.set_raw_property_count(p.declarations.declarations.len() as i32);
    // let style_rule:Vec<Property> = p.declarations.declarations.iter().collect();
    resolve_style(p, &mut tw_set);
    tw_set
}
