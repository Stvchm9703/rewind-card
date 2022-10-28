// use std::fmt::Format;
use lightningcss::{
    stylesheet::{ParserOptions, PrinterOptions},
    traits::ToCss,
    values::color::{self, CssColor},
};
use serde::{Deserialize, Serialize};
use std::fs;
pub static mut TAILWIND_GERENAL_TOKEN: Vec<GerenalToken> = Vec::new();

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TailwindTokenSet {
    is_based: bool,
    // involved class
    pub involved_classnames: Vec<String>,
    pub tailwind_token: Vec<String>,
    pub layer_group: String,
    pub media_query: Vec<String>,
    pub raw_property: String,
}
impl TailwindTokenSet {
    pub fn new() -> TailwindTokenSet {
        TailwindTokenSet {
            is_based: false,
            involved_classnames: vec![],
            tailwind_token: vec![],
            layer_group: "".to_owned(),
            media_query: Vec::new(),
            raw_property: "".to_owned(),
        }
    }

    pub fn set_layer_group(&mut self, income_str: String) {
        self.layer_group = income_str;
    }
    pub fn push_involved_classname(&mut self, income_str: String) {
        self.involved_classnames.push(income_str);
    }
    pub fn push_involved_classnames(&mut self, income_arr: Vec<&str>) {
        for t in income_arr {
            self.involved_classnames.push(t.to_owned());
        }
    }

    pub fn push_media_query(&mut self, income_str: String) {
        self.media_query.push(income_str);
    }
    pub fn push_media_queries(&mut self, income_arr: Vec<&str>) {
        for t in income_arr {
            self.media_query.push(t.to_owned());
        }
    }
    pub fn push_tailwind_token<F: ToString>(&mut self, property_name: &str, property_value: F) {
        if property_name != "" {
            self.tailwind_token
                .push(format!("{}-{}", property_name, property_value.to_string()));
        } else {
            self.tailwind_token.push(property_value.to_string());
        }
    }
    pub fn set_raw_property(&mut self, income_str: String) {
        self.raw_property = income_str;
    }

    pub fn export_token() -> Vec<String> {
        vec![]
    }
}

pub enum TwToken {
    GerenalToken,
    ColorToken,
    TypographyToken,
}
// impl TwToken {
//     fn get_token(&self) -> String;
//     fn match_token<F: ToString>(&self, input_property_name: &str, input_property_value: F) -> bool;
//     // fn match_token(&self, input_property_name: &str, input_property_value: &str) -> bool;
// }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GerenalToken {
    pub property_name: String,
    pub property_value: String,
    pub tailwind_token: String,
}

impl GerenalToken {
    fn get_token(&self) -> String {
        return self.tailwind_token.to_owned();
    }
    fn match_token(&self, input_property_name: &str, input_property_value: &str) -> bool {
        self.property_name.eq(input_property_name) && self.property_value.eq(input_property_value)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorToken {
    pub token_name: String,
    pub token_value: String,
}
impl ColorToken {
    fn get_token(&self) -> String {
        return self.token_name.to_owned();
    }
    fn match_token(&self, input_st: &str) -> bool {
        self.token_value.eq(input_st)
    }
    fn similar_token(&self, input_color: &color::CssColor) -> bool {
        let income_rgb = input_color
            .to_css_string(PrinterOptions::default())
            .unwrap();
        // income_rgb.
        return false;
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypographyToken {
    pub token_name: String,
    pub font_size: String,
    pub line_height: String,
}

impl TypographyToken {
    fn get_token(&self) -> String {
        return self.token_name.to_owned();
    }
    fn match_token(&self, input_font_size: &str) -> bool {
        self.font_size.eq(input_font_size)
    }
}

fn init() {

    // TAILWIND_GERENAL_TOKEN.push();
}
