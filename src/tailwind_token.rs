// use std::fmt::Format;
use csv;
use lightningcss::{
    media_query::{MediaCondition, MediaFeature, MediaQuery},
    properties::font::{FontSize, LineHeight},
    stylesheet::StyleAttribute,
    stylesheet::{ParserOptions, PrinterOptions},
    traits::ToCss,
    values::color::CssColor,
    values::length::{Length, LengthValue},
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

// use std::fs;
pub static mut TAILWIND_TYPOGRAPHY_TOKEN: Vec<TypographyToken> = Vec::new();
pub static mut TAILWIND_COLOR_TOKEN: Vec<ColorToken> = Vec::new();
pub static mut TAILWIND_MEDIA_LAYOUT_TOKEN: Vec<MediaToken> = Vec::new();

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
    ColorToken,
    TypographyToken,
}
// impl TwToken {
//     fn get_token(&self) -> String;
//     fn match_token<F: ToString>(&self, input_property_name: &str, input_property_value: F) -> bool;
//     // fn match_token(&self, input_property_name: &str, input_property_value: &str) -> bool;
// }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorToken {
    pub token_name: String,
    pub token_value: String,
    pub color_set: CssColor,
    pub color_set_red: u8,
    pub color_set_green: u8,
    pub color_set_blue: u8,
}
impl ColorToken {
    pub fn get_token(&self) -> String {
        return self.token_name.to_owned();
    }

    // pub fn similar_token(&self, input_color: &CssColor) -> bool {
    //     // let income_rgb = struct {
    //     let mut red = 0f32;
    //     let mut green = 0f32;
    //     let mut blue = 0f32;
    //     let mut alpha = 1f32;
    //     // } ;

    //     if let CssColor::RGBA(pp) = input_color.to_rgb() {
    //         red = pp.red_f32();
    //         green = pp.green_f32();
    //         blue = pp.blue_f32();
    //         alpha = pp.alpha_f32();
    //     };

    //     println!("r:{}, g:{}, b:{}, a:{}  ", red, green, blue, alpha);

    //     // income_rgb.
    //     return false;
    // }
}

// pub fn similar_token

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypographyToken {
    pub token_name: String,
    pub font_size: String,
    pub line_height: String,
    pub font_size_set: FontSize,
    pub line_height_set: LineHeight,
}

impl TypographyToken {
    fn get_token(&self) -> String {
        return self.token_name.to_owned();
    }
    fn match_token(&self, input_font_size: &str) -> bool {
        self.font_size.eq(input_font_size)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaToken {
    pub token_name: String,
    pub min_width_string: String,
    pub max_width_string: String,
    pub min_width: Option<Length>,
    pub max_width: Option<Length>,
}

pub fn init() {
    // fs::read_to_string("./preset/color-token.csv");

    let mut csv_color_token = csv::Reader::from_path("./preset/color-token.csv").unwrap();
    for record in csv_color_token.records() {
        let weee = record.unwrap();
        let mut color_token_set: ColorToken = ColorToken {
            token_name: weee.get(0).unwrap().to_owned(),
            token_value: weee.get(1).unwrap().to_owned(),
            color_set: CssColor::CurrentColor,
            color_set_red: 0u8,
            color_set_green: 0u8,
            color_set_blue: 0u8,
        };
        let dummy_color_set = format!("color: {};", color_token_set.token_value);
        let mut css_attr =
            StyleAttribute::parse(&dummy_color_set, ParserOptions::default()).unwrap();
        for t in css_attr.declarations.iter_mut() {
            // println!("t {}" , t.to_css_string(false, PrinterOptions::default()).unwrap());

            if let lightningcss::properties::Property::Color(p) = t {
                // println!("ccc {}" , p.to_rgb().to_css_string(PrinterOptions::default()).unwrap());

                color_token_set.color_set = p.to_owned();
                // // println!()
                // let rgb_set = color_token_set.color_set.to_rgb();
                if let CssColor::RGBA(_) = p {
                    let rgb_set = p.to_rgb();
                    if let CssColor::RGBA(pp) = rgb_set {
                        color_token_set.color_set_red = pp.red;
                        color_token_set.color_set_green = pp.green;
                        color_token_set.color_set_blue = pp.blue;
                    }
                }
            }
        }
        unsafe {
            TAILWIND_COLOR_TOKEN.push(color_token_set);
        }
    }

    let mut csv_typography_token = csv::Reader::from_path("./preset/typography-token.csv").unwrap();
    for record in csv_typography_token.records() {
        let weee = record.unwrap();
        let mut type_token_set: TypographyToken = TypographyToken {
            token_name: weee.get(0).unwrap().to_owned(),
            font_size: weee.get(1).unwrap().to_owned(),
            line_height: weee.get(2).unwrap().to_owned(),
            font_size_set: FontSize::Relative(
                lightningcss::properties::font::RelativeFontSize::Larger,
            ),
            line_height_set: LineHeight::Length(
                lightningcss::values::percentage::DimensionPercentage::Percentage(
                    lightningcss::values::percentage::Percentage(100f32),
                ),
            ),
        };
        let dummy_set = format!(
            "font-size: {}; line-height: {};",
            type_token_set.font_size, type_token_set.line_height
        );
        let mut css_attr = StyleAttribute::parse(&dummy_set, ParserOptions::default()).unwrap();
        for t in css_attr.declarations.iter_mut() {
            if let lightningcss::properties::Property::FontSize(p) = t {
                type_token_set.font_size_set = p.to_owned();
            }
            if let lightningcss::properties::Property::LineHeight(p) = t {
                type_token_set.line_height_set = p.to_owned();
            }
        }
        unsafe {
            TAILWIND_TYPOGRAPHY_TOKEN.push(type_token_set);
        }
    }

    let mut csv_media_query_token = csv::Reader::from_path("./preset/media-query.csv").unwrap();
    for record in csv_media_query_token.records() {
        let weee = record.unwrap().clone();
        let mut token_set: MediaToken = MediaToken {
            token_name: weee.get(0).unwrap().to_owned(),
            min_width_string: weee.get(1).unwrap().to_string(),
            max_width_string: weee.get(2).unwrap().to_string(),
            min_width: None,
            max_width: None,
        };
        let re = Regex::new(r"(?P<number_value>[\d|.]+)(?P<unit>\w+)$").unwrap();

        let mut min_width_value = Length::Value(LengthValue::Rem(0f32));
        if &token_set.min_width_string != "" {
            let ssss = re.captures(&token_set.min_width_string).unwrap();
            let num = ssss
                .name("number_value")
                .unwrap()
                .as_str()
                .to_owned()
                .parse::<f32>()
                .unwrap();
            let unit = ssss.name("unit").unwrap().as_str().to_lowercase();

            if unit == "rem" || unit == "em" {
                min_width_value = Length::Value(LengthValue::Rem(num));
            } else if unit == "px" {
                min_width_value = Length::Value(LengthValue::Px(num));
            }
            token_set.min_width = Some(min_width_value);
        }

        let mut max_width_value = Length::Value(LengthValue::Rem(0f32));

        if &token_set.max_width_string != "" {
            let ssss = re.captures(&token_set.max_width_string).unwrap();
            let num = ssss
                .name("number_value")
                .unwrap()
                .as_str()
                .to_owned()
                .parse::<f32>()
                .unwrap();
            let unit = ssss.name("unit").unwrap().as_str().to_lowercase();

            if unit == "rem" || unit == "em" {
                max_width_value = Length::Value(LengthValue::Rem(num));
            } else if unit == "px" {
                max_width_value = Length::Value(LengthValue::Px(num));
            }
            token_set.min_width = Some(max_width_value);
        }

        unsafe {
            TAILWIND_MEDIA_LAYOUT_TOKEN.push(token_set);
        }
    }
    // let mut leng_st = Length::Value(lightningcss::values::length::LengthValue::Rem(1f32));
    // unsafe {
    //     println!(
    //         "color-token : {}",
    //         serde_json::to_string(&TAILWIND_COLOR_TOKEN).unwrap()
    //     );
    //     println!(
    //         "typography-token : {}",
    //         serde_json::to_string(&TAILWIND_TYPOGRAPHY_TOKEN).unwrap()
    //     );

    //       println!(
    //         "media-token : {}",
    //         serde_json::to_string(&TAILWIND_MEDIA_LAYOUT_TOKEN).unwrap()
    //     );
    // }
}

fn in_range(arange: &u8, income_val: &u8) -> bool {
    let mut lower_val = arange.to_owned();
    let mut upper_val: u8 = arange.to_owned();
    if lower_val == 1u8 {
        lower_val = 0u8;
    } else if lower_val != 0u8 {
        lower_val -= 2u8;
    }
    if upper_val == 254u8 {
        upper_val = 255u8;
    } else if upper_val != 255u8 {
        upper_val += 2u8;
    }
    return (income_val >= &lower_val) && (income_val <= &upper_val);
}

pub fn search_color(r: &u8, g: &u8, b: &u8) -> Vec<String> {
    let mut return_set: Vec<String> = vec![];
    // if income_color

    unsafe {
        for color_set in &TAILWIND_COLOR_TOKEN {
            if (&color_set.color_set_red == r)
                && (&color_set.color_set_blue == b)
                && (&color_set.color_set_green == g)
            {
                return_set.push(color_set.token_name.clone());
            } else if in_range(&color_set.color_set_red, r)
                && in_range(&color_set.color_set_blue, b)
                && in_range(&color_set.color_set_green, g)
            {
                return_set.push(color_set.token_name.clone());
            }
        }
    }

    return return_set;
}
