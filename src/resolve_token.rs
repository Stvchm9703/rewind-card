use itertools::Itertools;
// use itertools::Itertools;
use lightningcss::{
    media_query::{
        MediaCondition,
        MediaFeature,
        MediaFeatureComparison,
        // MediaFeatureName,
        // Operator
        MediaFeatureValue,
        MediaQuery,
    },
    properties::{
        border::BorderSideWidth,
        font::{AbsoluteFontSize, AbsoluteFontWeight, FontSize, FontWeight, LineHeight},
        grid,
        size::{MaxSize, Size},
    },

    // rules::{style::StyleRule, CssRule},
    stylesheet::PrinterOptions,
    traits::ToCss,
    values::{
        color::CssColor,
        length::{LengthPercentage, LengthPercentageOrAuto, LengthValue},
        percentage::{DimensionPercentage, NumberOrPercentage},
        time::Time,
    },
    // targets::Targets,
};
// use parcel_selectors::SelectorList;

use regex::Regex;

use crate::tailwind_token::{
    search_color, search_font, search_media, search_media_v2, TailwindTokenSet,
};

pub fn resolve_track_size(
    income_value: &grid::TrackSize,
    tw_set: &mut TailwindTokenSet,
    token_prefix: &str,
) {
    match income_value {
        grid::TrackSize::TrackBreadth(k) => match k {
            grid::TrackBreadth::Flex(k) => tw_set.push_tailwind_token(token_prefix, k),
            grid::TrackBreadth::Length(k) => resolve_dimension(k, tw_set, token_prefix),
            grid::TrackBreadth::MinContent => {
                tw_set.push_tailwind_token(token_prefix, "min");
            }

            grid::TrackBreadth::MaxContent => {
                tw_set.push_tailwind_token(token_prefix, "max");
            }
            grid::TrackBreadth::Auto => {
                tw_set.push_tailwind_token(token_prefix, "auto");
            }
        },
        grid::TrackSize::MinMax { min: _, max: _ } => {
            resolve_raw_exp(income_value, tw_set, token_prefix)
        }
        grid::TrackSize::FitContent(_) => {
            tw_set.push_tailwind_token(token_prefix, "[fit-content]");
        }
    }
}

pub fn resolve_length_unit(
    income_value: &LengthPercentageOrAuto,
    tw_set: &mut TailwindTokenSet,
    token_prefix: &str,
) {
    match income_value {
        LengthPercentageOrAuto::Auto => {
            tw_set.push_tailwind_token_with_check(token_prefix, "auto");
        }
        LengthPercentageOrAuto::LengthPercentage(a) => {
            resolve_dimension(a, tw_set, token_prefix);
        }
    }
}

pub fn resolve_dimension(
    length_value: &DimensionPercentage<LengthValue>,
    tw_set: &mut TailwindTokenSet,
    token_prefix: &str,
) {
    match length_value {
        LengthPercentage::Dimension(value) => {
            let mut pxv = match value.to_px() {
                Some(v) => {
                    if v >= 24f32 {
                        format!("{}", (v / 4f32).round())
                    } else {
                        format!("{}", v.round() / 4f32)
                    }
                }
                None => String::from(""),
            };

            if pxv.eq("") {
                let (number_value, unit) = value.to_unit_value();
                if unit.to_lowercase().contains("em") {
                    pxv = format!("{}", (number_value * 4f32).round());
                } else {
                    pxv = format!("{}{}", number_value, unit)
                }
            }

            tw_set.push_tailwind_token_with_check(token_prefix, pxv);
        }
        DimensionPercentage::Percentage(percentage_val) => {
            tw_set.push_tailwind_token_with_check(
                token_prefix,
                percentage_val
                    .to_css_string(PrinterOptions::default())
                    .unwrap(),
            );
        }
        DimensionPercentage::Calc(s) => tw_set.push_tailwind_token_with_check(
            token_prefix,
            format!("[{}]", s.to_css_string(PrinterOptions::default()).unwrap()),
        ),
    };
}

pub fn resolve_length_size(income_value: &Size, tw_set: &mut TailwindTokenSet, token_prefix: &str) {
    match income_value {
        Size::Auto => {
            tw_set.push_tailwind_token(token_prefix, "auto");
        }
        Size::LengthPercentage(a) => {
            resolve_dimension(a, tw_set, token_prefix);
        }
        Size::MinContent(_) => {
            tw_set.push_tailwind_token(token_prefix, "min");
        }
        Size::MaxContent(_) => {
            tw_set.push_tailwind_token(token_prefix, "max");
        }
        Size::FitContent(_) => {
            tw_set.push_tailwind_token(token_prefix, "fit");
        }
        Size::FitContentFunction(_) => {}
        Size::Stretch(_) => {}
        Size::Contain => {
            tw_set.push_tailwind_token(token_prefix, "contain");
        }
    }
}
pub fn resolve_length_max_size(
    income_value: &MaxSize,
    tw_set: &mut TailwindTokenSet,
    token_prefix: &str,
) {
    match income_value {
        MaxSize::LengthPercentage(a) => {
            resolve_dimension(a, tw_set, token_prefix);
        }
        MaxSize::None => {
            tw_set.push_tailwind_token(token_prefix, "none");
        }
        MaxSize::MinContent(_) => {
            tw_set.push_tailwind_token(token_prefix, "min");
        }
        MaxSize::MaxContent(_) => {
            tw_set.push_tailwind_token(token_prefix, "max");
        }
        MaxSize::FitContent(_) => {
            tw_set.push_tailwind_token(token_prefix, "fit");
        }
        MaxSize::FitContentFunction(_) => {}
        MaxSize::Stretch(_) => {}
        MaxSize::Contain => {
            tw_set.push_tailwind_token(token_prefix, "contain");
        }
    }
}

pub fn resolve_keyword<F: ToCss>(
    income_value: &F,
    tw_set: &mut TailwindTokenSet,
    token_prefix: &str,
) {
    tw_set.push_tailwind_token(
        token_prefix,
        income_value
            .to_css_string(PrinterOptions {
                project_root: None,
                minify: false,
                source_map: None,
                targets: None.into(),
                analyze_dependencies: None,
                pseudo_classes: None,
            })
            .unwrap(),
    );
}

pub fn resolve_border_side_width(
    income_value: &BorderSideWidth,
    tw_set: &mut TailwindTokenSet,
    token_prefix: &str,
) {
    match income_value {
        BorderSideWidth::Length(p) => match p {
            lightningcss::values::length::Length::Value(value) => {
                let mut pxv = match value.to_px() {
                    Some(v) => {
                        if v >= 24f32 {
                            format!("{}", (v / 4f32).round())
                        } else {
                            format!("{}", v.round() / 4f32)
                        }
                    }
                    None => String::from(""),
                };

                if pxv.eq("") {
                    let (number_value, unit) = value.to_unit_value();
                    if unit.to_lowercase().contains("em") {
                        pxv = format!("{}", (number_value * 4f32).round());
                    } else {
                        pxv = format!("{}{}", number_value, unit)
                    }
                }

                tw_set.push_tailwind_token(token_prefix, pxv);
            }
            lightningcss::values::length::Length::Calc(s) => tw_set.push_tailwind_token(
                token_prefix,
                format!("[{}]", s.to_css_string(PrinterOptions::default()).unwrap()),
            ),
        },
        _ => resolve_keyword(income_value, tw_set, token_prefix),
    }
}

pub fn resolve_color(income_value: &CssColor, tw_set: &mut TailwindTokenSet, token_prefix: &str) {
    if let CssColor::CurrentColor = income_value {
        tw_set.push_tailwind_token(token_prefix, "current");
        return;
    };

    let mut red = 0u8;
    let mut green = 0u8;
    let mut blue = 0u8;
    let mut alpha = 255u8;
    let mut alpha_float = 1f32;
    // } ;

    if let Ok(CssColor::RGBA(pp)) = income_value.to_rgb() {
        red = pp.red;
        green = pp.green;
        blue = pp.blue;
        alpha = pp.alpha;
        alpha_float = pp.alpha_f32();
    };

    // println!(
    //     "[{}] :: r:{}, g:{}, b:{}, a:{}  ",
    //     token_prefix, red, green, blue, alpha
    // );
    if alpha != 255u8 {
        let op = token_prefix.to_owned() + "-op";
        tw_set.push_tailwind_token(op.as_str(), (alpha_float * 100f32).round());
    }
    let mut resolved_token = search_color(&red, &green, &blue);
    if resolved_token.len() == 0 {
        let resolved_raw = income_value
            .to_css_string(PrinterOptions::default())
            .unwrap();
        resolved_token.push(resolved_raw);
    } else if resolved_token.len() > 1 {
        let regex_set = Regex::new(r"^(\w+)-(\d+)").unwrap();
        resolved_token = resolved_token
            .into_iter()
            .filter(|token| regex_set.is_match(token) == false)
            .collect();
    }

    // println!("resolved-token {}", resolved_token.join(" "));
    tw_set.push_tailwind_token(token_prefix, &resolved_token[0]);

    // println!()
}

pub fn resolve_number_or_percentage(
    income_value: &NumberOrPercentage,
    tw_set: &mut TailwindTokenSet,
    token_prefix: &str,
) {
    match income_value {
        NumberOrPercentage::Percentage(p) => {
            let value = (p.0).round() / 100f32;
            tw_set.push_tailwind_token(token_prefix, &value);
        }
        NumberOrPercentage::Number(p) => {
            tw_set.push_tailwind_token(token_prefix, format!("{:.rounded$}", p, rounded = 2usize));
        }
    }
}

pub fn resolve_time(income_value: &Time, tw_set: &mut TailwindTokenSet, token_prefix: &str) {
    // #[warn(unused_assignments)]
    let mut time_set = 0f32;
    match *income_value {
        Time::Seconds(a) => {
            time_set = a * 1000f32;
        }
        Time::Milliseconds(a) => {
            time_set = a;
        }
    }

    tw_set.push_tailwind_token(token_prefix, time_set);
}

pub fn resolve_raw_exp<F: ToCss>(
    income_value: &F,
    tw_set: &mut TailwindTokenSet,
    token_prefix: &str,
) {
    let resolved_raw = income_value
        .to_css_string(PrinterOptions::default())
        .unwrap();
    tw_set.push_tailwind_token(
        token_prefix,
        format!("[{}]", resolved_raw.replace(r"[\s|,]", "_")),
    );
}

pub fn resolve_font_set(income_value: &FontSize, tw_set: &mut TailwindTokenSet) {
    match income_value {
        FontSize::Length(s) => {
            if let DimensionPercentage::Dimension(m) = s {
                let (value, unit) = m.to_unit_value();
                if unit.contains("em") {
                    let tkn = search_font(&value);
                    for tk in tkn {
                        tw_set.push_tailwind_token("text", tk);
                    }
                } else if m.to_px().is_some() {
                    let converted = m.to_px().unwrap();
                    let tkn = search_font(&(converted / 16f32));
                    for tk in tkn {
                        tw_set.push_tailwind_token("text", tk);
                    }
                } else {
                    let resolved_value = m.to_css_string(PrinterOptions::default()).unwrap();
                    tw_set.push_tailwind_token("text", format!("[{}]", resolved_value));
                }
            }
        }
        FontSize::Absolute(s) => match s {
            AbsoluteFontSize::XXSmall => tw_set.push_tailwind_token("text", "xs"),
            AbsoluteFontSize::XSmall => tw_set.push_tailwind_token("text", "xs"),
            AbsoluteFontSize::Small => tw_set.push_tailwind_token("text", "sm"),
            AbsoluteFontSize::Medium => tw_set.push_tailwind_token("text", "base"),
            AbsoluteFontSize::Large => tw_set.push_tailwind_token("text", "lg"),
            AbsoluteFontSize::XLarge => tw_set.push_tailwind_token("text", "xl"),
            AbsoluteFontSize::XXLarge => tw_set.push_tailwind_token("text", "2xl"),
            AbsoluteFontSize::XXXLarge => tw_set.push_tailwind_token("text", "3xl"),
        },
        FontSize::Relative(_) => {}
    }
}

pub fn resolve_font_weight(income_value: &FontWeight, tw_set: &mut TailwindTokenSet) {
    match income_value {
        FontWeight::Absolute(a) => match a {
            AbsoluteFontWeight::Weight(k) => {
                if k <= &200f32 {
                    tw_set.push_tailwind_token("font", "extralight");
                } else if k <= &300f32 {
                    tw_set.push_tailwind_token("font", "light");
                } else if k <= &400f32 {
                    tw_set.push_tailwind_token("font", "normal");
                } else if k <= &500f32 {
                    tw_set.push_tailwind_token("font", "medium");
                } else if k <= &600f32 {
                    tw_set.push_tailwind_token("font", "semibold");
                } else if k <= &700f32 {
                    tw_set.push_tailwind_token("font", "bold");
                } else {
                    tw_set.push_tailwind_token("font", "extrabold");
                }
            }
            AbsoluteFontWeight::Normal => tw_set.push_tailwind_token("font", "normal"),
            AbsoluteFontWeight::Bold => tw_set.push_tailwind_token("font", "bold"),
        },
        FontWeight::Bolder => tw_set.push_tailwind_token("font", "medium"),
        FontWeight::Lighter => {
            tw_set.push_tailwind_token("font", "light");
        }
    }
}

pub fn resolve_line_height_set(
    income_value: &LineHeight,
    tw_set: &mut TailwindTokenSet,
    token_prefix: &str,
) {
    fn token_set(n: &f32, tw_set: &mut TailwindTokenSet, token_prefix: &str) {
        if (n >= &1f32) && (n < &1.125f32) {
            tw_set.push_tailwind_token(token_prefix, "none");
        } else if (n >= &1.125f32) && (n < &1.3125f32) {
            tw_set.push_tailwind_token(token_prefix, "tight");
        } else if (n >= &1.3125f32) && (n < &1.4375f32) {
            tw_set.push_tailwind_token(token_prefix, "snug");
        } else if (n >= &1.4375f32) && (n < &1.5625f32) {
            tw_set.push_tailwind_token(token_prefix, "normal");
        } else if (n >= &1.5625f32) && (n < &1.8125f32) {
            tw_set.push_tailwind_token(token_prefix, "relaxed");
        } else if n <= &2f32 {
            tw_set.push_tailwind_token(token_prefix, "loose");
        }
    }
    match income_value {
        LineHeight::Normal => {}
        LineHeight::Number(n) => {
            token_set(n, tw_set, token_prefix);
        }
        LineHeight::Length(n) => {
            match n {
                DimensionPercentage::Dimension(p) => {
                    let mut rem_value = 1f32;
                    let (value, unit) = p.to_unit_value();
                    if unit.contains("em") {
                        rem_value = value;
                    } else if p.to_px().is_some() {
                        rem_value = p.to_px().unwrap() / 16f32;
                    }
                    tw_set.push_tailwind_token(token_prefix, (rem_value * 4f32).round());
                }
                DimensionPercentage::Percentage(p) => {
                    token_set(&((p.0).round() / 100f32), tw_set, token_prefix)
                }
                // DimensionPercentage::Calc(_) => todo!(),
                _ => {}
            }
        }
    }
}

pub fn resolve_media_query_prefix(q: MediaQuery) -> Vec<String> {
    let mut temp: Vec<String> = vec![];

    match q.condition.unwrap() {
        MediaCondition::Feature(ss) => resolve_media_query_feat(ss, &mut temp),
        MediaCondition::Operation {
            operator: _,
            conditions,
        } => {
            for cond in conditions {
                if let MediaCondition::Feature(fsa) = cond {
                    resolve_media_query_feat(fsa, &mut temp);
                }
            }
        }
        // MediaCondition::Not(_) => todo!(),
        _ => (),
    }
    // print!("{:#?} ", temp);
    // return temp;

    // if temp.len() == 0 {
    //     return temp;
    // }

    // let mut tem_2: Vec<String> = Vec::new();

    // if temp.iter().any(|t| t.starts_with("lt-"))
    //     && temp.iter().any(|t| t.starts_with("gt-"))
    //     && temp.iter().any(|t| t.starts_with("at-"))
    // {
    //     tem_2 = temp
    //         .iter()
    //         .filter(|&t| t.starts_with("at-"))
    //         .cloned()
    //         .collect();
    // } else if temp.iter().any(|t| t.starts_with("gt-")) && temp.iter().any(|t| t.starts_with("at-"))
    // {
    //     tem_2 = temp
    //         .iter()
    //         .filter(|&t| !t.starts_with("at-") && !t.starts_with("gt-"))
    //         .cloned()
    //         .collect();
    // }
    // tem_2.sort();
    // tem_2.dedup();
    return temp;
}

enum MinifiedMediaFeatureComparison {
    At,
    Gt,
    Lt,
}

fn resolve_media_query_feat(ss: MediaFeature, temp: &mut Vec<String>) {
    if let MediaFeature::Plain { name, value } = ss {
        let resolve_name = name.to_css_string(PrinterOptions::default()).unwrap();
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
            let yyy = search_media(&resolve_name, &rem_value);
            temp.extend_from_slice(&yyy);
        }
    } else if let MediaFeature::Range {
        name: _,
        operator,
        value,
    } = ss
    {
        let operator_token = match operator {
            MediaFeatureComparison::Equal => MinifiedMediaFeatureComparison::At,
            MediaFeatureComparison::GreaterThan => MinifiedMediaFeatureComparison::Gt,
            MediaFeatureComparison::GreaterThanEqual => MinifiedMediaFeatureComparison::Gt,
            MediaFeatureComparison::LessThan => MinifiedMediaFeatureComparison::Lt,
            MediaFeatureComparison::LessThanEqual => MinifiedMediaFeatureComparison::Lt,
        };
        let mut media_token: Vec<String> = vec![];
        if let MediaFeatureValue::Length(qwe) = value {
            let mut rem_value = -1f32;
            if let lightningcss::values::length::Length::Value(v) = qwe {
                let (vv, unit) = v.to_unit_value();
                if unit.to_lowercase().contains("em") {
                    rem_value = vv;
                } else {
                    let u = v.to_px().unwrap_or_default();
                    rem_value = (u / 16f32).round();
                }
            }
            media_token = search_media_v2(&operator, &rem_value);
        }
        match operator_token {
            MinifiedMediaFeatureComparison::At => {
                let t = media_token
                    .into_iter()
                    .find_or_first(|x| x.starts_with("at-"))
                    .unwrap();
                temp.push(t.to_owned());
            }
            MinifiedMediaFeatureComparison::Gt => {
                let t = media_token
                    .into_iter()
                    .find_or_first(|x| x.starts_with("gt-"))
                    .unwrap();
                temp.push(t.to_owned());
            }
            MinifiedMediaFeatureComparison::Lt => {
                let t = media_token
                    .into_iter()
                    .find_or_first(|x| x.starts_with("lt-"))
                    .unwrap();
                temp.push(t.to_owned());
            }
        }
    }
}
