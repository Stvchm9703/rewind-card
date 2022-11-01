use lightningcss::{
    properties::{
        border::BorderSideWidth,
        font::{AbsoluteFontSize, FontSize, LineHeight},
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
};
use regex::Regex;

use crate::tailwind_token::{search_color, search_font, TailwindTokenSet};

pub fn resolve_track_size(
    income_value: &grid::TrackSize,
    tw_set: &mut TailwindTokenSet,
    token_prefix: &str,
) {
    match income_value {
        grid::TrackSize::TrackBreadth(k) => match k {
            lightningcss::properties::grid::TrackBreadth::Flex(k) => {
                tw_set.push_tailwind_token(token_prefix, k)
            }
            lightningcss::properties::grid::TrackBreadth::Length(k) => {
                resolve_dimension(k, tw_set, token_prefix)
            }
            lightningcss::properties::grid::TrackBreadth::MinContent => {
                tw_set.push_tailwind_token(token_prefix, "min");
            }

            lightningcss::properties::grid::TrackBreadth::MaxContent => {
                tw_set.push_tailwind_token(token_prefix, "max");
            }
            lightningcss::properties::grid::TrackBreadth::Auto => {
                tw_set.push_tailwind_token(token_prefix, "auto");
            }
        },
        grid::TrackSize::MinMax(_, _) => resolve_raw_exp(income_value, tw_set, token_prefix),
        grid::TrackSize::FitContent(_) => {
            tw_set.push_tailwind_token(token_prefix, "[fit-content]");
        }
    }
}

pub fn resolve_length_length_percentage_or_auto(
    income_value: &LengthPercentageOrAuto,
    tw_set: &mut TailwindTokenSet,
    token_prefix: &str,
) {
    match income_value {
        LengthPercentageOrAuto::Auto => {
            tw_set.push_tailwind_token(token_prefix, "auto");
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
                let (vall, unit) = value.to_unit_value();
                if unit.to_lowercase().contains("em") {
                    pxv = format!("{}", (vall * 4f32).round());
                } else {
                    pxv = format!("{}{}", vall, unit)
                }
            }

            tw_set.push_tailwind_token(token_prefix, pxv);
        }
        DimensionPercentage::Percentage(percentage_val) => {
            tw_set.push_tailwind_token(
                token_prefix,
                percentage_val
                    .to_css_string(PrinterOptions::default())
                    .unwrap(),
            );
        }
        DimensionPercentage::Calc(s) => tw_set.push_tailwind_token(
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
            .to_css_string(PrinterOptions::default())
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
                    let (vall, unit) = value.to_unit_value();
                    if unit.to_lowercase().contains("em") {
                        pxv = format!("{}", (vall * 4f32).round());
                    } else {
                        pxv = format!("{}{}", vall, unit)
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

    if let CssColor::RGBA(pp) = income_value.to_rgb() {
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
        let rege = Regex::new(r"^(\w+)-(\d+)").unwrap();
        resolved_token = resolved_token
            .into_iter()
            .filter(|token| rege.is_match(token) == false)
            .collect();
    }

    // println!("resolved-token {}", resolved_token.join(" "));
    tw_set.push_tailwind_token(token_prefix, &resolved_token[0]);

    println!()
}

pub fn resolve_percentage_or_number(
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
            tw_set.push_tailwind_token(token_prefix, format!("{:.prec$}", p, prec = 2usize));
        }
    }
}

pub fn resolve_time(income_value: &Time, tw_set: &mut TailwindTokenSet, token_prefix: &str) {
    let mut time_set = 0f32;
    match *income_value {
        Time::Seconds(a) => {
            time_set = a * 1000f32;
        }
        Time::Milliseconds(a) => {
            time_set = a;
        }
    }

    tw_set.push_tailwind_token(token_prefix, time_set.round());
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

pub fn resovle_font_set(
    income_value: &FontSize,
    tw_set: &mut TailwindTokenSet,
    token_prefix: &str,
) {
    match income_value {
        FontSize::Length(s) => {
            if let DimensionPercentage::Dimension(m) = s {
                let (value, unit) = m.to_unit_value();
                if unit.contains("em") {
                    let tkn = search_font(&value);
                    for tk in tkn {
                        tw_set.push_tailwind_token(token_prefix, tk);
                    }
                } else if m.to_px().is_some() {
                    let converted = m.to_px().unwrap();
                    let tkn = search_font(&(converted / 16f32));
                    for tk in tkn {
                        tw_set.push_tailwind_token(token_prefix, tk);
                    }
                } else {
                    let resolved_value = m.to_css_string(PrinterOptions::default()).unwrap();
                    tw_set.push_tailwind_token("text", format!("[{}]", resolved_value));
                }
            }
        }
        FontSize::Absolute(s) => match s {
            AbsoluteFontSize::XXSmall => tw_set.push_tailwind_token(token_prefix, "xs"),
            AbsoluteFontSize::XSmall => tw_set.push_tailwind_token(token_prefix, "xs"),
            AbsoluteFontSize::Small => tw_set.push_tailwind_token(token_prefix, "sm"),
            AbsoluteFontSize::Medium => tw_set.push_tailwind_token(token_prefix, "base"),
            AbsoluteFontSize::Large => tw_set.push_tailwind_token(token_prefix, "lg"),
            AbsoluteFontSize::XLarge => tw_set.push_tailwind_token(token_prefix, "xl"),
            AbsoluteFontSize::XXLarge => tw_set.push_tailwind_token(token_prefix, "2xl"),
        },
        FontSize::Relative(s) => {}
    }
}

pub fn resovle_line_height_set(
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
        } else if (n <= &2f32) {
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
