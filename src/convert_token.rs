use lightningcss::{
    properties::{
        effects::Filter,
        Property,
        grid,
        font,
        transform,
        effects,
    },
    values::{
        easing,
    },
    rules::{style::StyleRule },
    stylesheet::{ PrinterOptions},
    traits::ToCss,
   
};
use regex::Regex;

use crate::tailwind_token::{ TailwindTokenSet};

use crate::resolve_token::{
    resolve_track_size, 
    resolve_length_length_percentage_or_auto,
    resolve_dimension,
    resolve_length_size,
    resolve_length_max_size,
    resolve_keyword,
    resolve_border_side_width,
    resolve_color,
    resolve_percentage_or_number,
    resolve_time,
    resolve_raw_exp,
    resolve_font_set,
    resolve_line_height_set,
};

pub fn resolve_style(rule: &StyleRule, tw_set: &mut TailwindTokenSet) {
    for prop in &rule.declarations.declarations {
        // prop.value_to_css_string(PrinterOptions::default());
        match prop {
            Property::BackgroundColor(p) => resolve_color(p, tw_set, "bg"),
            // Property::BackgroundImage(_) => todo!(),
            Property::BackgroundPositionX(p) => {
                for s in p {
                    resolve_keyword(s, tw_set, "bg");
                }
            }
            Property::BackgroundPositionY(p) => {
                for s in p {
                    resolve_keyword(s, tw_set, "bg");
                }
            }
            Property::BackgroundPosition(p) => {
                for s in p {
                    resolve_keyword(s, tw_set, "bg");
                }
            }
            Property::BackgroundSize(p) => {
                for s in p {
                    resolve_keyword(s, tw_set, "bg");
                }
            }
            Property::BackgroundRepeat(p) => {
                for s in p {
                    resolve_keyword(s, tw_set, "bg")
                }
            }
            Property::BackgroundAttachment(p) => {
                for s in p {
                    resolve_keyword(s, tw_set, "bg")
                }
            }
            Property::BackgroundOrigin(p) => resolve_keyword(p, tw_set, "bg-origin"),
            Property::Background(p) => {
                for s in p {
                    resolve_color(&s.color, tw_set, "bg");
                }
            }
            // Property::BoxShadow(_, _) => todo!(),
            Property::Opacity(p) => resolve_keyword(p, tw_set, "opacity"),
            Property::Color(p) => resolve_color(p, tw_set, "text"),
            Property::Display(p) => match *p {
                lightningcss::properties::display::Display::Keyword(a) => match a {
                    lightningcss::properties::display::DisplayKeyword::None => {
                        tw_set.push_tailwind_token("", "hidden");
                    }
                    _ => resolve_keyword(p, tw_set, ""),
                },
                _ => resolve_keyword(p, tw_set, ""),
            },

            Property::Visibility(p) => match *p {
                lightningcss::properties::display::Visibility::Hidden => {
                    tw_set.push_tailwind_token("", "invisible");
                }
                _ => resolve_keyword(p, tw_set, ""),
            },
            Property::Width(p) => {
                resolve_length_size(p, tw_set, "w");
            }
            Property::Height(p) => {
                resolve_length_size(p, tw_set, "h");
            }
            Property::MinWidth(p) => {
                resolve_length_size(p, tw_set, "min-w");
            }
            Property::MinHeight(p) => {
                resolve_length_size(p, tw_set, "min-h");
            }
            Property::MaxWidth(p) => {
                resolve_length_max_size(p, tw_set, "max-w");
            }
            Property::MaxHeight(p) => {
                resolve_length_max_size(p, tw_set, "max-h");
            }
            Property::BlockSize(p) => {
                resolve_length_size(p, tw_set, "block");
            }
            Property::InlineSize(p) => {
                resolve_length_size(p, tw_set, "inline");
            }
            Property::MinBlockSize(p) => {
                resolve_length_size(p, tw_set, "min-block");
            }
            Property::MinInlineSize(p) => {
                resolve_length_size(p, tw_set, "min-inline");
            }
            Property::MaxBlockSize(p) => {
                resolve_length_max_size(p, tw_set, "max-block");
            }
            Property::MaxInlineSize(p) => {
                resolve_length_max_size(p, tw_set, "max-inline");
            }
            Property::BoxSizing(p, _) => resolve_keyword(p, tw_set, "box"),
            Property::Overflow(p) => {
                resolve_keyword(&p.x, tw_set, "overflow-x");
                resolve_keyword(&p.y, tw_set, "overflow-y");
            }
            Property::OverflowX(p) => {
                resolve_keyword(p, tw_set, "overflow-x");
            }
            Property::OverflowY(p) => {
                resolve_keyword(p, tw_set, "overflow-y");
            }
            Property::TextOverflow(p, _) => {
                resolve_keyword(p, tw_set, "text");
            }
            Property::Position(p) => {
                resolve_keyword(p, tw_set, "");
            }
            Property::Top(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "top");
            }
            Property::Bottom(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "bottom");
            }
            Property::Left(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "left");
            }
            Property::Right(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "right");
            }
            Property::InsetBlockStart(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "top");
            }
            Property::InsetBlockEnd(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "bottom");
            }
            Property::InsetInlineStart(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "left");
            }
            Property::InsetInlineEnd(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "right");
            }
            Property::InsetBlock(p) => {
                resolve_length_length_percentage_or_auto(&p.block_start, tw_set, "top");
                resolve_length_length_percentage_or_auto(&p.block_end, tw_set, "bottom");
            }
            Property::InsetInline(p) => {
                resolve_length_length_percentage_or_auto(&p.inline_start, tw_set, "left");
                resolve_length_length_percentage_or_auto(&p.inline_end, tw_set, "right");
            }
            Property::Inset(p) => {
                resolve_length_length_percentage_or_auto(&p.top, tw_set, "top");
                resolve_length_length_percentage_or_auto(&p.left, tw_set, "left");
                resolve_length_length_percentage_or_auto(&p.bottom, tw_set, "bottom");
                resolve_length_length_percentage_or_auto(&p.right, tw_set, "right");
            }

            // Property::BorderSpacing(_) => todo!(),
            Property::BorderTopColor(p) => resolve_color(p, tw_set, "b-t"),
            Property::BorderBottomColor(p) => resolve_color(p, tw_set, "b-b"),
            Property::BorderLeftColor(p) => resolve_color(p, tw_set, "b-l"),
            Property::BorderRightColor(p) => resolve_color(p, tw_set, "b-r"),
            Property::BorderBlockStartColor(p) => resolve_color(p, tw_set, "b-t"),
            Property::BorderBlockEndColor(p) => resolve_color(p, tw_set, "b-b"),
            Property::BorderInlineStartColor(p) => resolve_color(p, tw_set, "b-l"),
            Property::BorderInlineEndColor(p) => resolve_color(p, tw_set, "b-r"),
            Property::BorderTopStyle(p) => resolve_keyword(p, tw_set, "b-t"),
            Property::BorderBottomStyle(p) => resolve_keyword(p, tw_set, "b-b"),
            Property::BorderLeftStyle(p) => resolve_keyword(p, tw_set, "b-l"),
            Property::BorderRightStyle(p) => resolve_keyword(p, tw_set, "b-r"),
            Property::BorderBlockStartStyle(p) => resolve_keyword(p, tw_set, "b-t"),
            Property::BorderBlockEndStyle(p) => resolve_keyword(p, tw_set, "b-b"),
            Property::BorderInlineStartStyle(p) => resolve_keyword(p, tw_set, "b-l"),
            Property::BorderInlineEndStyle(p) => resolve_keyword(p, tw_set, "b-r"),
            Property::BorderTopWidth(p) => resolve_border_side_width(p, tw_set, "b-t"),
            Property::BorderBottomWidth(p) => resolve_border_side_width(p, tw_set, "b-b"),
            Property::BorderLeftWidth(p) => resolve_border_side_width(p, tw_set, "b-l"),
            Property::BorderRightWidth(p) => resolve_border_side_width(p, tw_set, "b-r"),
            Property::BorderBlockStartWidth(p) => resolve_border_side_width(p, tw_set, "b-t"),
            Property::BorderBlockEndWidth(p) => resolve_border_side_width(p, tw_set, "b-b"),
            Property::BorderInlineStartWidth(p) => resolve_border_side_width(p, tw_set, "b-l"),
            Property::BorderInlineEndWidth(p) => resolve_border_side_width(p, tw_set, "b-r"),
            Property::BorderTopLeftRadius(p, _) => resolve_dimension(&p.0, tw_set, "rounded-tl"),
            Property::BorderTopRightRadius(p, _) => resolve_dimension(&p.0, tw_set, "rounded-tr"),
            Property::BorderBottomLeftRadius(p, _) => resolve_dimension(&p.0, tw_set, "rounded-bl"),
            Property::BorderBottomRightRadius(p, _) => {
                resolve_dimension(&p.0, tw_set, "rounded-br")
            }
            Property::BorderStartStartRadius(p) => resolve_dimension(&p.0, tw_set, "rounded-tl"),
            Property::BorderStartEndRadius(p) => resolve_dimension(&p.0, tw_set, "rounded-tr"),
            Property::BorderEndStartRadius(p) => resolve_dimension(&p.0, tw_set, "rounded-bl"),
            Property::BorderEndEndRadius(p) => resolve_dimension(&p.0, tw_set, "rounded-br"),
            Property::BorderRadius(p, _) => {
                resolve_dimension(&p.top_left.0, tw_set, "rounded-tl");
                resolve_dimension(&p.top_right.0, tw_set, "rounded-tr");
                resolve_dimension(&p.bottom_left.0, tw_set, "rounded-bl");
                resolve_dimension(&p.bottom_right.0, tw_set, "rounded-br");
            }
            // Property::BorderImageSource(_) => todo!(),
            // Property::BorderImageOutset(_) => todo!(),
            // Property::BorderImageRepeat(_) => todo!(),
            // Property::BorderImageWidth(_) => todo!(),
            // Property::BorderImageSlice(_) => todo!(),
            // Property::BorderImage(_, _) => todo!(),
            Property::BorderColor(p) => {
                resolve_color(&p.top, tw_set, "b-t");
                resolve_color(&p.bottom, tw_set, "b-b");
                resolve_color(&p.left, tw_set, "b-l");
                resolve_color(&p.right, tw_set, "b-r");
            }
            Property::BorderStyle(p) => {
                resolve_keyword(&p.top, tw_set, "b-t");
                resolve_keyword(&p.bottom, tw_set, "b-b");
                resolve_keyword(&p.left, tw_set, "b-l");
                resolve_keyword(&p.right, tw_set, "b-r");
            }
            Property::BorderWidth(p) => {
                resolve_border_side_width(&p.top, tw_set, "b-t");
                resolve_border_side_width(&p.bottom, tw_set, "b-b");
                resolve_border_side_width(&p.left, tw_set, "b-l");
                resolve_border_side_width(&p.right, tw_set, "b-r");
            }
            Property::BorderBlockColor(p) => {
                resolve_color(&p.start, tw_set, "b-t");
                resolve_color(&p.end, tw_set, "b-b");
            }
            Property::BorderBlockStyle(p) => {
                resolve_keyword(&p.start, tw_set, "b-t");
                resolve_keyword(&p.end, tw_set, "b-b");
            }
            Property::BorderBlockWidth(p) => {
                resolve_border_side_width(&p.start, tw_set, "b-t");
                resolve_border_side_width(&p.end, tw_set, "b-b");
            }
            Property::BorderInlineColor(p) => {
                resolve_color(&p.start, tw_set, "b-l");
                resolve_color(&p.end, tw_set, "b-r");
            }
            Property::BorderInlineStyle(p) => {
                resolve_keyword(&p.start, tw_set, "b-l");
                resolve_keyword(&p.end, tw_set, "b-r");
            }
            Property::BorderInlineWidth(p) => {
                resolve_border_side_width(&p.start, tw_set, "b-l");
                resolve_border_side_width(&p.end, tw_set, "b-r");
            }
            Property::Border(p) => {
                resolve_keyword(&p.style, tw_set, "b");
                resolve_border_side_width(&p.width, tw_set, "b");
                resolve_color(&p.color, tw_set, "b");
            }
            Property::BorderTop(p) => {
                resolve_keyword(&p.style, tw_set, "b-t");
                resolve_border_side_width(&p.width, tw_set, "b-t");
                resolve_color(&p.color, tw_set, "b-t");
            }
            Property::BorderBottom(p) => {
                resolve_keyword(&p.style, tw_set, "b-b");
                resolve_border_side_width(&p.width, tw_set, "b-b");
                resolve_color(&p.color, tw_set, "b-b");
            }
            Property::BorderLeft(p) => {
                resolve_keyword(&p.style, tw_set, "b-l");
                resolve_border_side_width(&p.width, tw_set, "b-l");
                resolve_color(&p.color, tw_set, "b-l");
            }
            Property::BorderRight(p) => {
                resolve_keyword(&p.style, tw_set, "b-r");
                resolve_border_side_width(&p.width, tw_set, "b-r");
                resolve_color(&p.color, tw_set, "b-r");
            }
            Property::BorderBlock(p) => {
                resolve_color(&p.color, tw_set, "b-t");
                resolve_keyword(&p.style, tw_set, "b-t");
                resolve_border_side_width(&p.width, tw_set, "b-t");
                resolve_color(&p.color, tw_set, "b-b");
                resolve_keyword(&p.style, tw_set, "b-b");
                resolve_border_side_width(&p.width, tw_set, "b-b");
            }
            Property::BorderBlockStart(p) => {
                resolve_color(&p.color, tw_set, "b-t");
                resolve_keyword(&p.style, tw_set, "b-t");
                resolve_border_side_width(&p.width, tw_set, "b-t");
            }
            Property::BorderBlockEnd(p) => {
                resolve_color(&p.color, tw_set, "b-b");
                resolve_keyword(&p.style, tw_set, "b-b");
                resolve_border_side_width(&p.width, tw_set, "b-b");
            }
            Property::BorderInline(p) => {
                resolve_keyword(&p.style, tw_set, "b-l");
                resolve_border_side_width(&p.width, tw_set, "b-l");
                resolve_color(&p.color, tw_set, "b-l");

                resolve_keyword(&p.style, tw_set, "b-r");
                resolve_border_side_width(&p.width, tw_set, "b-r");
                resolve_color(&p.color, tw_set, "b-r");
            }
            Property::BorderInlineStart(p) => {
                resolve_color(&p.color, tw_set, "b-l");
                resolve_keyword(&p.style, tw_set, "b-l");
                resolve_border_side_width(&p.width, tw_set, "b-l");
            }
            Property::BorderInlineEnd(p) => {
                resolve_color(&p.color, tw_set, "b-r");
                resolve_keyword(&p.style, tw_set, "b-r");
                resolve_border_side_width(&p.width, tw_set, "b-r");
            }
            Property::Outline(p) => {
                resolve_color(&p.color, tw_set, "outline");
                resolve_border_side_width(&p.width, tw_set, "outline");
                resolve_keyword(&p.style, tw_set, "outline");
            }
            Property::OutlineColor(p) => resolve_color(p, tw_set, "outline"),
            Property::OutlineStyle(p) => resolve_keyword(p, tw_set, "outline"),
            Property::OutlineWidth(p) => resolve_border_side_width(p, tw_set, "outline"),
            Property::FlexDirection(p, _) => resolve_keyword(p, tw_set, "flex"),
            Property::FlexWrap(p, _) => resolve_keyword(p, tw_set, "flex"),
            Property::FlexFlow(p, _) => resolve_keyword(p, tw_set, "flex"),
            Property::FlexGrow(p, _) => tw_set.push_tailwind_token("grow", p),
            Property::FlexShrink(p, _) => tw_set.push_tailwind_token("shrink", p),
            Property::FlexBasis(p, _) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "basis")
            }
            Property::Flex(p, _) => {
                resolve_length_length_percentage_or_auto(&p.basis, tw_set, "basis");
                tw_set.push_tailwind_token("shrink", &p.shrink);
                tw_set.push_tailwind_token("grow", &p.grow);
            }
            Property::Order(p, _) => tw_set.push_tailwind_token("order", p),
            Property::AlignContent(p, _) => resolve_keyword(p, tw_set, "content"),
            Property::JustifyContent(p, _) => resolve_keyword(p, tw_set, "justify"),
            Property::PlaceContent(p) => resolve_keyword(p, tw_set, "place-content"),
            Property::AlignSelf(p, _) => resolve_keyword(p, tw_set, "self"),
            Property::JustifySelf(p) => resolve_keyword(p, tw_set, "justify-self"),
            Property::PlaceSelf(p) => resolve_keyword(p, tw_set, "place-self"),
            Property::AlignItems(p, _) => resolve_keyword(p, tw_set, "items"),
            Property::JustifyItems(p) => resolve_keyword(p, tw_set, "justify-items"),
            Property::PlaceItems(p) => resolve_keyword(p, tw_set, "place-items"),
            Property::RowGap(p) => resolve_keyword(p, tw_set, "gap-x"),
            Property::ColumnGap(p) => resolve_keyword(p, tw_set, "gap-y"),
            Property::Gap(p) => resolve_keyword(p, tw_set, "gap"),
            // Property::BoxOrient(_, _) => todo!(),
            // Property::BoxDirection(_, _) => todo!(),
            // Property::BoxOrdinalGroup(_, _) => todo!(),
            // Property::BoxAlign(_, _) => todo!(),
            // Property::BoxFlex(_, _) => todo!(),
            // Property::BoxFlexGroup(_, _) => todo!(),
            // Property::BoxPack(_, _) => todo!(),
            // Property::BoxLines(_, _) => todo!(),
            // Property::FlexPack(_, _) => todo!(),
            Property::FlexOrder(p, _) => tw_set.push_tailwind_token("order", p),
            // Property::FlexAlign(p, _) => {},
            Property::FlexItemAlign(p, _) => {
                resolve_keyword(p, tw_set, "items");
            }
            // Property::FlexLinePack(_, _) => todo!(),
            // Property::FlexPositive(_, _) => todo!(),
            // Property::FlexNegative(_, _) => todo!(),
            // Property::FlexPreferredSize(_, _) => todo!(),
            // ====
            Property::GridTemplateColumns(p) => match p {
                grid::TrackSizing::None => {
                    resolve_keyword(p, tw_set, "grid-cols")
                }
                grid::TrackSizing::TrackList(a) => {
                    resolve_raw_exp(a, tw_set, "grid-cols");
                }
            },
            Property::GridTemplateRows(p) => match p {
                grid::TrackSizing::None => {
                    resolve_keyword(p, tw_set, "grid-rows")
                }
                grid::TrackSizing::TrackList(a) => {
                    resolve_raw_exp(a, tw_set, "grid-rows");
                }
            },
            Property::GridAutoColumns(p) => {
                for a in &p.0 {
                    resolve_track_size(a, tw_set, "auto-cols");
                }
            }
            Property::GridAutoRows(p) => {
                for a in &p.0 {
                    resolve_track_size(a, tw_set, "auto-rows");
                }
            }
            Property::GridAutoFlow(p) => resolve_keyword(p, tw_set, "grid-flow"),
            Property::GridTemplateAreas(p) => {
                resolve_raw_exp(p, tw_set, "grid-areas");
            }
            // Property::GridTemplate(_) => todo!(),
            Property::Grid(p) => {
                resolve_raw_exp(&p.areas, tw_set, "grid-areas");
                resolve_keyword(&p.auto_flow, tw_set, "grid-flow");
                for a in &p.auto_columns.0 {
                    resolve_track_size(a, tw_set, "auto-cols");
                }

                for a in &p.auto_rows.0 {
                    resolve_track_size(a, tw_set, "auto-rows");
                }

                match &p.columns {
                    grid::TrackSizing::None => {
                        tw_set.push_tailwind_token("grid-cols", "none");
                    }
                    grid::TrackSizing::TrackList(a) => {
                        resolve_raw_exp(a, tw_set, "grid-cols");
                    }
                }
                match &p.rows {
                    grid::TrackSizing::None => {
                        tw_set.push_tailwind_token("grid-rows", "none");
                    }
                    grid::TrackSizing::TrackList(a) => {
                        resolve_raw_exp(a, tw_set, "grid-rows");
                    }
                }
            }
            Property::GridRowStart(p) => {
                match p {
                    grid::GridLine::Auto => {
                        tw_set.push_tailwind_token("row-start", "auto")
                    }
                    s => resolve_raw_exp(s, tw_set, "grid-rows"),
                }
                //  resolve_keyword(p, tw_set, "col-start"),
            }
            Property::GridRowEnd(p) => {
                match p {
                    grid::GridLine::Auto => {
                        tw_set.push_tailwind_token("row-end", "auto")
                    }
                    s => resolve_raw_exp(s, tw_set, "row-end"),
                }
                //  resolve_keyword(p, tw_set, "row-start"),
            }
            Property::GridColumnStart(p) => {
                match p {
                    grid::GridLine::Auto => {
                        tw_set.push_tailwind_token("col-start", "auto")
                    }
                    s => resolve_raw_exp(s, tw_set, "col-start"),
                }
                //  resolve_keyword(p, tw_set, "col-start"),
            }
            Property::GridColumnEnd(p) => {
                match p {
                    grid::GridLine::Auto => {
                        tw_set.push_tailwind_token("col-end", "auto")
                    }
                    s => resolve_raw_exp(s, tw_set, "col-end"),
                }
                //  resolve_keyword(p, tw_set, "col-start"),
            }
            Property::GridRow(p) => {
                match &p.start {
                    grid::GridLine::Auto => {
                        tw_set.push_tailwind_token("row-start", "auto")
                    }
                    s => resolve_raw_exp(s, tw_set, "row-start"),
                }
                match &p.end {
                    grid::GridLine::Auto => {
                        tw_set.push_tailwind_token("row-end", "auto")
                    }
                    s => resolve_raw_exp(s, tw_set, "row-end"),
                }
            }
            Property::GridColumn(p) => {
                match &p.start {
                    grid::GridLine::Auto => {
                        tw_set.push_tailwind_token("col-start", "auto")
                    }
                    s => resolve_raw_exp(s, tw_set, "col-start"),
                }
                match &p.end {
                    grid::GridLine::Auto => {
                        tw_set.push_tailwind_token("col-end", "auto")
                    }
                    s => resolve_raw_exp(s, tw_set, "col-end"),
                }
            }
            Property::GridArea(p) => {
                let resolve_name = p.to_css_string(PrinterOptions::default()).unwrap();
                if resolve_name.contains(r"[\d|\/]") == false {
                    tw_set.push_tailwind_token(
                        "area-",
                        format!("[{}]", resolve_name.replace(r"[\s|,]", "_")),
                    );

                    return;
                }

                match &p.row_start {
                    grid::GridLine::Auto => {
                        tw_set.push_tailwind_token("row-start", "auto")
                    }
                    s => resolve_raw_exp(s, tw_set, "row-start"),
                }
                match &p.row_end {
                    grid::GridLine::Auto => {
                        tw_set.push_tailwind_token("row-end", "auto")
                    }
                    s => resolve_raw_exp(s, tw_set, "row-end"),
                }

                match &p.column_start {
                    grid::GridLine::Auto => {
                        tw_set.push_tailwind_token("col-start", "auto")
                    }
                    s => resolve_raw_exp(s, tw_set, "col-start"),
                }
                match &p.column_end {
                    grid::GridLine::Auto => {
                        tw_set.push_tailwind_token("col-end", "auto")
                    }
                    s => resolve_raw_exp(s, tw_set, "col-end"),
                }
            }
            Property::MarginTop(p) => resolve_length_length_percentage_or_auto(&p, tw_set, "mt"),
            Property::MarginBottom(p) => resolve_length_length_percentage_or_auto(&p, tw_set, "mb"),
            Property::MarginLeft(p) => resolve_length_length_percentage_or_auto(&p, tw_set, "ml"),
            Property::MarginRight(p) => resolve_length_length_percentage_or_auto(&p, tw_set, "mr"),
            Property::MarginBlockStart(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "mt");
            }
            Property::MarginBlockEnd(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "mb");
            }
            Property::MarginInlineStart(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "ml");
            }
            Property::MarginInlineEnd(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "mr");
            }
            Property::MarginBlock(p) => {
                resolve_length_length_percentage_or_auto(&p.block_start, tw_set, "mt");
                resolve_length_length_percentage_or_auto(&p.block_end, tw_set, "mb");
            }
            Property::MarginInline(p) => {
                resolve_length_length_percentage_or_auto(&p.inline_start, tw_set, "ml");
                resolve_length_length_percentage_or_auto(&p.inline_end, tw_set, "mr");
            }
            Property::Margin(p) => {
                resolve_length_length_percentage_or_auto(&p.top, tw_set, "mt");
                resolve_length_length_percentage_or_auto(&p.left, tw_set, "ml");
                resolve_length_length_percentage_or_auto(&p.bottom, tw_set, "mb");
                resolve_length_length_percentage_or_auto(&p.right, tw_set, "mr");
            }
            Property::PaddingTop(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "pt");
            }
            Property::PaddingBottom(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "pb");
            }
            Property::PaddingLeft(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "pl");
            }
            Property::PaddingRight(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "pr");
            }
            Property::PaddingBlockStart(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "pt");
            }
            Property::PaddingBlockEnd(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "pb");
            }
            Property::PaddingInlineStart(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "pl");
            }
            Property::PaddingInlineEnd(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "pr");
            }
            Property::PaddingBlock(p) => {
                resolve_length_length_percentage_or_auto(&p.block_start, tw_set, "pt");
                resolve_length_length_percentage_or_auto(&p.block_end, tw_set, "pb");
            }
            Property::PaddingInline(p) => {
                resolve_length_length_percentage_or_auto(&p.inline_start, tw_set, "pl");
                resolve_length_length_percentage_or_auto(&p.inline_end, tw_set, "pr");
            }
            Property::Padding(p) => {
                resolve_length_length_percentage_or_auto(&p.top, tw_set, "pt");
                resolve_length_length_percentage_or_auto(&p.left, tw_set, "pl");
                resolve_length_length_percentage_or_auto(&p.bottom, tw_set, "pb");
                resolve_length_length_percentage_or_auto(&p.right, tw_set, "pr");
            }

            Property::ScrollMarginTop(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-mt");
            }
            Property::ScrollMarginBottom(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-mb");
            }
            Property::ScrollMarginLeft(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-ml");
            }
            Property::ScrollMarginRight(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-mr");
            }
            Property::ScrollMarginBlockStart(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-mt");
            }
            Property::ScrollMarginBlockEnd(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-mb");
            }
            Property::ScrollMarginInlineStart(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-ml");
            }
            Property::ScrollMarginInlineEnd(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-mr");
            }
            Property::ScrollMarginBlock(p) => {
                resolve_length_length_percentage_or_auto(&p.block_start, tw_set, "scroll-mt");
                resolve_length_length_percentage_or_auto(&p.block_end, tw_set, "scroll-mb");
            }
            Property::ScrollMarginInline(p) => {
                resolve_length_length_percentage_or_auto(&p.inline_start, tw_set, "scroll-ml");
                resolve_length_length_percentage_or_auto(&p.inline_end, tw_set, "scroll-mr");
            }
            Property::ScrollMargin(p) => {
                resolve_length_length_percentage_or_auto(&p.top, tw_set, "scroll-mt");
                resolve_length_length_percentage_or_auto(&p.left, tw_set, "scroll-ml");
                resolve_length_length_percentage_or_auto(&p.bottom, tw_set, "scroll-mb");
                resolve_length_length_percentage_or_auto(&p.right, tw_set, "scroll-mr");
            }
            Property::ScrollPaddingTop(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-pt");
            }
            Property::ScrollPaddingBottom(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-pb");
            }
            Property::ScrollPaddingLeft(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-pl");
            }
            Property::ScrollPaddingRight(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-pr");
            }
            Property::ScrollPaddingBlockStart(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-pt");
            }
            Property::ScrollPaddingBlockEnd(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-pb");
            }
            Property::ScrollPaddingInlineStart(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-pl");
            }
            Property::ScrollPaddingInlineEnd(p) => {
                resolve_length_length_percentage_or_auto(&p, tw_set, "scroll-pr");
            }
            Property::ScrollPaddingBlock(p) => {
                resolve_length_length_percentage_or_auto(&p.block_start, tw_set, "scroll-pt");
                resolve_length_length_percentage_or_auto(&p.block_end, tw_set, "scroll-pb");
            }
            Property::ScrollPaddingInline(p) => {
                resolve_length_length_percentage_or_auto(&p.inline_start, tw_set, "scroll-pl");
                resolve_length_length_percentage_or_auto(&p.inline_end, tw_set, "scroll-pr");
            }
            Property::ScrollPadding(p) => {
                resolve_length_length_percentage_or_auto(&p.top, tw_set, "scroll-pt");
                resolve_length_length_percentage_or_auto(&p.left, tw_set, "scroll-pl");
                resolve_length_length_percentage_or_auto(&p.bottom, tw_set, "scroll-pb");
                resolve_length_length_percentage_or_auto(&p.right, tw_set, "scroll-pr");
            }

            Property::FontWeight(p) => {
                match p {
                    font::FontWeight::Absolute(a) => match a {
                        font::AbsoluteFontWeight::Weight(k) => {
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
                        font::AbsoluteFontWeight::Normal => {
                            tw_set.push_tailwind_token("font", "normal")
                        }
                        font::AbsoluteFontWeight::Bold => {
                            tw_set.push_tailwind_token("font", "bold")
                        }
                    },
                    font::FontWeight::Bolder => {
                        tw_set.push_tailwind_token("font", "medium")
                    }
                    font::FontWeight::Lighter => {
                        tw_set.push_tailwind_token("font", "light");
                    }
                }
                // resolve_keyword(p, tw_set, "font")
            }
            Property::FontSize(p) => resolve_font_set(p , tw_set, ""),
            // Property::FontStretch(_) => todo!(),
            // Property::FontFamily(_) => {},
            Property::FontStyle(p) => match p {
                font::FontStyle::Normal => {}
                _ => resolve_keyword(p, tw_set, "font"),
            },
            // Property::FontVariantCaps(_) => todo!(),
            Property::LineHeight(p) => resolve_line_height_set(p , tw_set,"leading"),
            // Property::Font(_) => todo!(),
            // Property::VerticalAlign(_) => todo!(),
            // Property::FontPalette(_) => todo!(),

            Property::TransitionProperty(p, _) => {
                for q in p {
                    if q.name() == "all" {
                        tw_set.push_tailwind_token("transition", "all");
                    } 
                    else if q.name().contains("color") {
                        tw_set.push_tailwind_token("transition", "colors");
                    }
                     else if q.name().contains("opacity") {
                        tw_set.push_tailwind_token("transition", "opacity");
                    }
                    else if q.name().contains("shadow") {
                        tw_set.push_tailwind_token("transition", "shadow");
                    }
                    else if q.name().contains("transform") {
                        tw_set.push_tailwind_token("transition", "transform");
                    }
                }
            },
            Property::TransitionDuration(p, _) => {
                for q in p {
                    resolve_time(q, tw_set, "duration");
                }
            }
            Property::TransitionDelay(p, _) => {
                for q in p {
                    resolve_time(q, tw_set, "delay");
                }
            }
            Property::TransitionTimingFunction(p, _) => {
                for q in p {
                    // resolve_time(q, tw_set, "delay");
                    match *q {
                        easing::EasingFunction::CubicBezier(a, b, c, d) => {
                            // /**
                            // standard: {
                            //     productive: 'cubic-bezier(0.2, 0, 0.38, 0.9)',
                            //     expressive: 'cubic-bezier(0.4, 0.14, 0.3, 1)',
                            // },
                            // entrance: {
                            //     productive: 'cubic-bezier(0, 0, 0.38, 0.9)',
                            //     expressive: 'cubic-bezier(0, 0, 0.3, 1)',
                            // },
                            // exit: {
                            //     productive: 'cubic-bezier(0.2, 0, 1, 0.9)',
                            //     expressive: 'cubic-bezier(0.4, 0.14, 1, 1)',
                            // },
                            //  */

                            if (a == 0.2f32) && (b == 0f32) && (c == 0.38f32) && (d == 0.9f32) {
                                tw_set.push_tailwind_token("ease", "standard-productive");
                            }
                            else if (a == 0.4f32) && (b == 0.14f32) && (c == 0.3f32) && (d == 1f32) {
                                tw_set.push_tailwind_token("ease", "standard-expressive");
                            }
                            else if (a == 0f32) && (b == 0f32) && (c == 0.38f32) && (d == 0.9f32) {
                                tw_set.push_tailwind_token("ease", "entrance-productive");
                            }
                            else if (a == 0f32) && (b == 0f32) && (c == 0.3f32 )&& (d == 1f32) {
                                tw_set.push_tailwind_token("ease", "entrance-expressive");
                            }  
                            else if (a == 0.2f32) && (b == 0f32) && (c == 1f32) && (d == 0.9f32) {
                                tw_set.push_tailwind_token("ease", "exit-productive");
                            }
                            else if (a == 0.4f32) && (b == 0.14f32) && (c == 1f32) && (d == 1f32) {
                                tw_set.push_tailwind_token("ease", "exit-expressive");
                            } 
                            else {
                                tw_set.push_tailwind_token("ease", "exit-expressive");
                            } 
                            
                        },
                        easing::EasingFunction::Steps(_, _) => {},
                        easing::EasingFunction::Linear => {
                            tw_set.push_tailwind_token("ease", "linear");
                        },
                        _ => {
                            resolve_keyword(q, tw_set, "");
                        }
                        // easing::EasingFunction::Ease => todo!(),
                        // easing::EasingFunction::EaseIn => todo!(),
                        // easing::EasingFunction::EaseOut => todo!(),
                        // easing::EasingFunction::EaseInOut => todo!(),
                    }
                }
            }
            Property::Transition(p, _) => {
                for q in p {
                    resolve_time(&q.delay, tw_set, "delay");
                    resolve_time(&q.duration, tw_set, "duration");
                    match q.timing_function {
                        easing::EasingFunction::CubicBezier(a, b, c, d) => {
                            // /**
                            // standard: {
                            //     productive: 'cubic-bezier(0.2, 0, 0.38, 0.9)',
                            //     expressive: 'cubic-bezier(0.4, 0.14, 0.3, 1)',
                            // },
                            // entrance: {
                            //     productive: 'cubic-bezier(0, 0, 0.38, 0.9)',
                            //     expressive: 'cubic-bezier(0, 0, 0.3, 1)',
                            // },
                            // exit: {
                            //     productive: 'cubic-bezier(0.2, 0, 1, 0.9)',
                            //     expressive: 'cubic-bezier(0.4, 0.14, 1, 1)',
                            // },
                            //  */

                            if (a == 0.2f32) && (b == 0f32) && (c == 0.38f32) && (d == 0.9f32) {
                                tw_set.push_tailwind_token("ease", "standard-productive");
                            }
                            else if (a == 0.4f32) && (b == 0.14f32) && (c == 0.3f32) && (d == 1f32) {
                                tw_set.push_tailwind_token("ease", "standard-expressive");
                            }
                            else if (a == 0f32) && (b == 0f32) && (c == 0.38f32) && (d == 0.9f32) {
                                tw_set.push_tailwind_token("ease", "entrance-productive");
                            }
                            else if (a == 0f32) && (b == 0f32) && (c == 0.3f32 )&& (d == 1f32) {
                                tw_set.push_tailwind_token("ease", "entrance-expressive");
                            }  
                            else if (a == 0.2f32) && (b == 0f32) && (c == 1f32) && (d == 0.9f32) {
                                tw_set.push_tailwind_token("ease", "exit-productive");
                            }
                            else if (a == 0.4f32) && (b == 0.14f32) && (c == 1f32) && (d == 1f32) {
                                tw_set.push_tailwind_token("ease", "exit-expressive");
                            } 
                            else {
                                tw_set.push_tailwind_token("ease", format!("[cubic-bezier:{}_{}_{}_{}]" , a,b,c,d));
                            } 
                            
                        },
                        easing::EasingFunction::Steps(_, _) => {},
                        easing::EasingFunction::Linear => {
                            tw_set.push_tailwind_token("ease", "linear");
                        },
                        _ => {
                            resolve_keyword(q, tw_set, "");
                        }
                    }
                }
            }

            // Property::AnimationName(_, _) => todo!(),
            // Property::AnimationDuration(_, _) => todo!(),
            // Property::AnimationTimingFunction(_, _) => todo!(),
            // Property::AnimationIterationCount(_, _) => todo!(),
            // Property::AnimationDirection(_, _) => todo!(),
            // Property::AnimationPlayState(_, _) => todo!(),
            // Property::AnimationDelay(_, _) => todo!(),
            // Property::AnimationFillMode(_, _) => todo!(),
            // Property::Animation(_, _) => todo!(),

            Property::Transform(p, _) => {
                for q in &p.0{
                    match q {
                        transform::Transform::Translate(x, y) => {
                            resolve_dimension(&x, tw_set, "translate-x");
                            resolve_dimension(&y, tw_set, "translate-y");
                        },
                        transform::Transform::TranslateX(x) => {
                            resolve_dimension(&x, tw_set, "translate-x");
                        },
                        transform::Transform::TranslateY(y) => {
                            resolve_dimension(&y, tw_set, "translate-y");
                        },
                        transform::Transform::TranslateZ(z) => {
                            resolve_keyword(&z, tw_set, "translate-z");

                        },
                        transform::Transform::Translate3d(x, y, z) => {
                            resolve_dimension(&x, tw_set, "translate-x");
                            resolve_dimension(&y, tw_set, "translate-y");
                            resolve_keyword(&z, tw_set, "translate-z");
                        },
                        transform::Transform::Scale(x, y) => {
                            resolve_percentage_or_number(&x, tw_set, "scale-x");
                            resolve_percentage_or_number(&y, tw_set, "scale-y");
                        },
                        transform::Transform::ScaleX(x) => {
                            resolve_percentage_or_number(&x, tw_set, "scale-x");
                        },
                        transform::Transform::ScaleY(y) => {
                            resolve_percentage_or_number(&y, tw_set, "scale-y");
                        },
                        transform::Transform::ScaleZ(z) => {
                            resolve_percentage_or_number(&z, tw_set, "scale-z");

                        },
                        transform::Transform::Scale3d(x, y, z) => {
                            resolve_percentage_or_number(&x, tw_set, "scale-x");
                            resolve_percentage_or_number(&y, tw_set, "scale-y");
                            resolve_percentage_or_number(&z, tw_set, "scale-z");
                        },
                        transform::Transform::Rotate(x) => {
                            tw_set.push_tailwind_token("rotate", x.to_degrees());
                        },
                        transform::Transform::RotateX(x) => {
                            tw_set.push_tailwind_token("rotate-x", x.to_degrees());
                        },
                        transform::Transform::RotateY(x) => {
                            tw_set.push_tailwind_token("rotate-y", x.to_degrees());
                        },
                        transform::Transform::RotateZ(x) => {
                            tw_set.push_tailwind_token("rotate-z", x.to_degrees());
                        },
                        transform::Transform::Rotate3d(x, y, z, _) => {
                            tw_set.push_tailwind_token("rotate-x", x.to_degrees());
                            tw_set.push_tailwind_token("rotate-y", y.to_degrees());
                            tw_set.push_tailwind_token("rotate-z", z.to_degrees());
                        },
                        transform::Transform::Skew(x, y) => {
                            tw_set.push_tailwind_token("skew-x", x.to_degrees());
                            tw_set.push_tailwind_token("skew-y", y.to_degrees());
                        },
                        transform::Transform::SkewX(x) => {
                            tw_set.push_tailwind_token("skew-x", x.to_degrees());
                        },
                        transform::Transform::SkewY(y) => {
                            tw_set.push_tailwind_token("skew-y", y.to_degrees());

                        },
                        // transform::Transform::Perspective(_) => todo!(),
                        // transform::Transform::Matrix(_) => todo!(),
                        // transform::Transform::Matrix3d(_) => todo!(),
                        _ => (),
                    }
                }
            },
            Property::TransformOrigin(p, _) => {
                let resolved_raw = p
                    .to_css_string(PrinterOptions::default())
                    .unwrap();
                tw_set.push_tailwind_token(
                    "origin",
                   resolved_raw.replace(" ", "-") ,
                );
            },
            Property::TransformStyle(p, _) => resolve_keyword(p, tw_set, "preserve"),
            // Property::TransformBox(_) => todo!(),

            // Property::BackfaceVisibility(_, _) => todo!(),
            Property::Perspective(p, _) => resolve_keyword(p, tw_set, "perspective"),
            // Property::PerspectiveOrigin(_, _) => todo!(),
            Property::Translate(p) => resolve_keyword(p, tw_set, "translate"),
            Property::Rotate(p) => {
                resolve_keyword(&p.x, tw_set, "rotate-x");
                resolve_keyword(&p.y, tw_set, "rotate-y");
                resolve_keyword(&p.z, tw_set, "rotate-z");
            }
            Property::Scale(p) => {
                resolve_keyword(&p.x, tw_set, "scale-x");
                resolve_keyword(&p.y, tw_set, "scale-y");
                resolve_keyword(&p.z, tw_set, "scale-z");
            }
            Property::TextTransform(p) => resolve_keyword(p, tw_set, ""),
            Property::WhiteSpace(p) => resolve_keyword(p, tw_set, "whitespace"),
            Property::TabSize(p, _) => resolve_keyword(p, tw_set, "tab"),
            Property::WordBreak(p) => resolve_keyword(p, tw_set, ""),
            // Property::LineBreak(_) => todo!(),
            // Property::Hyphens(_, _) => todo!(),
            Property::OverflowWrap(p) => resolve_keyword(p, tw_set, ""),
            // Property::WordWrap(_) => todo!(),
            Property::TextAlign(p) => resolve_keyword(p, tw_set, "text"),
            Property::TextAlignLast(p, _) => resolve_keyword(p, tw_set, "last-text"),
            // Property::TextJustify(_) => todo!(),
            // Property::WordSpacing(_) => todo!(),
            // Property::LetterSpacing(_) => todo!(),
            // Property::TextIndent(_) => todo!(),
            // Property::TextDecorationLine(_, _) => todo!(),
            Property::TextDecorationStyle(p, _) => resolve_keyword(p, tw_set, "decoration"),
            // Property::TextDecorationColor(_, _) => todo!(),
            // Property::TextDecorationThickness(_) => todo!(),
            Property::TextDecoration(p, _) => resolve_keyword(&p.style, tw_set, "decoration"),
            // Property::TextDecorationSkipInk(_, _) => todo!(),
            // Property::TextEmphasisStyle(_, _) => todo!(),
            // Property::TextEmphasisColor(_, _) => todo!(),
            // Property::TextEmphasis(_, _) => todo!(),
            // Property::TextEmphasisPosition(_, _) => todo!(),
            // Property::TextShadow(_) => todo!(),
            Property::BoxDecorationBreak(p, _) => resolve_keyword(&p, tw_set, "decoration"),
            Property::Resize(p) => resolve_keyword(&p, tw_set, "resize"),
            Property::Cursor(p) => resolve_keyword(p, tw_set, "cursor"),
            // Property::CaretColor(p) => match p {
            //     lightningcss::properties::ui::ColorOrAuto::Auto => todo!(),
            //     lightningcss::properties::ui::ColorOrAuto::Color(a) => {
            //         // let sa = a.to_css_string(PrinterOptions::default()).unwrap();
            //     }
            // },
            // Property::CaretShape(_) => todo!(),
            // Property::Caret(_) => todo!(),
            Property::UserSelect(p, _) => resolve_keyword(p, tw_set, "select"),
            // Property::AccentColor(_) => todo!(),
            // Property::Appearance(_, _) => todo!(),
            Property::ListStyleType(p) => resolve_keyword(p, tw_set, "list"),
            // Property::ListStyleImage(_) => todo!(),
            Property::ListStylePosition(p) => resolve_keyword(p, tw_set, "list"),
            Property::ListStyle(p) => resolve_keyword(p, tw_set, "list"),
            // Property::MarkerSide(_) => todo!(),
            // Property::Composes(_) => todo!(),
            Property::Fill(p) => match p {
                lightningcss::properties::svg::SVGPaint::None => resolve_keyword(p, tw_set, "fill"),
                lightningcss::properties::svg::SVGPaint::Color(a) => {
                    resolve_color(&a, tw_set, "fill")
                }
                _ => {}
            },
            // Property::FillRule(_) => todo!(),
            // Property::FillOpacity(_) => todo!(),
            Property::Stroke(p) => match p {
                lightningcss::properties::svg::SVGPaint::None => {
                    tw_set.push_tailwind_token("stroke", "none")
                }
                lightningcss::properties::svg::SVGPaint::Color(a) => {
                    resolve_color(a, tw_set, "fill")
                }
                _ => {}
            },
            // Property::StrokeOpacity(_) => todo!(),
            Property::StrokeWidth(p) => resolve_dimension(p, tw_set, "stroke"),
            Property::StrokeLinecap(p) => resolve_keyword(p, tw_set, "stroke-cap"),
            Property::StrokeLinejoin(p) => resolve_keyword(p, tw_set, "stroke-join"),
            // Property::StrokeMiterlimit(_) => todo!(),
            // Property::StrokeDasharray(_) => todo!(),
            // Property::StrokeDashoffset(_) => todo!(),
            // Property::MarkerStart(_) => todo!(),
            // Property::MarkerMid(_) => todo!(),
            // Property::MarkerEnd(_) => todo!(),
            // Property::Marker(_) => todo!(),
            // Property::ColorInterpolation(_) => todo!(),
            // Property::ColorInterpolationFilters(_) => todo!(),
            // Property::ColorRendering(_) => todo!(),
            // Property::ShapeRendering(_) => todo!(),
            // Property::TextRendering(_) => todo!(),
            Property::ImageRendering(p) => resolve_keyword(p, tw_set, "image-render"),
            // Property::ClipPath(_, _) => todo!(),
            // Property::ClipRule(_) => todo!(),
            // Property::MaskImage(_, _) => todo!(),
            // Property::MaskMode(_) => todo!(),
            // Property::MaskRepeat(_, _) => todo!(),
            // Property::MaskPositionX(_) => todo!(),
            // Property::MaskPositionY(_) => todo!(),
            // Property::MaskPosition(_, _) => todo!(),
            // Property::MaskClip(_, _) => todo!(),
            // Property::MaskOrigin(_, _) => todo!(),
            // Property::MaskSize(_, _) => todo!(),
            // Property::MaskComposite(_) => todo!(),
            // Property::MaskType(_) => todo!(),
            // Property::Mask(p, _) => todo!(),
            // Property::MaskBorderSource(_) => todo!(),
            // Property::MaskBorderMode(_) => todo!(),
            // Property::MaskBorderSlice(_) => todo!(),
            // Property::MaskBorderWidth(_) => todo!(),
            // Property::MaskBorderOutset(_) => todo!(),
            // Property::MaskBorderRepeat(_) => todo!(),
            // Property::MaskBorder(_) => todo!(),
            // Property::WebKitMaskComposite(_) => todo!(),
            // Property::WebKitMaskSourceType(_, _) => todo!(),
            // Property::WebKitMaskBoxImage(_, _) => todo!(),
            // Property::WebKitMaskBoxImageSource(_, _) => todo!(),
            // Property::WebKitMaskBoxImageSlice(_, _) => todo!(),
            // Property::WebKitMaskBoxImageWidth(_, _) => todo!(),
            // Property::WebKitMaskBoxImageOutset(_, _) => todo!(),
            // Property::WebKitMaskBoxImageRepeat(_, _) => todo!(),
            Property::Filter(p, _) => match p {
                effects::FilterList::None => {
                    tw_set.push_tailwind_token("filter", "none")
                }
                effects::FilterList::Filters(dd) => {
                    for aaa in dd {
                        match aaa {
                            // effects::Filter::Url(_) => {},
                            Filter::Blur(p) => resolve_keyword(p, tw_set, "filter-blur"),
                            Filter::Brightness(p) => {
                                resolve_percentage_or_number(p, tw_set, "filter-brightness")
                            }
                            Filter::Contrast(p) => {
                                resolve_percentage_or_number(p, tw_set, "filter-contrast")
                            }
                            Filter::Grayscale(p) => {
                                resolve_percentage_or_number(p, tw_set, "filter-grayscale")
                            }
                            Filter::HueRotate(p) => resolve_keyword(p, tw_set, "filter-hue-rotate"),
                            Filter::Invert(p) => {
                                resolve_percentage_or_number(p, tw_set, "filter-invert")
                            }
                            // Filter::Opacity(_) => resolve_keyword(p, tw_set, "filter-blur"),
                            Filter::Saturate(p) => {
                                resolve_percentage_or_number(p, tw_set, "filter-saturate")
                            }
                            Filter::Sepia(p) => {
                                resolve_percentage_or_number(p, tw_set, "filter-sepia")
                            }
                            Filter::DropShadow(p) => {
                                resolve_keyword(p, tw_set, "filter-drop-shadow")
                            }
                            _ => {}
                        }
                        // resolve_keyword(aaa, tw_set, "filter");
                    }
                }
            },
            Property::BackdropFilter(p, _) => match p {
                effects::FilterList::None => {
                    tw_set.push_tailwind_token("backdrop-filter", "none")
                }
                effects::FilterList::Filters(dd) => {
                    for aaa in dd {
                        match aaa {
                            // effects::Filter::Url(_) => {},
                            Filter::Blur(p) => resolve_keyword(p, tw_set, "backdrop-filter-blur"),
                            Filter::Brightness(p) => resolve_percentage_or_number(
                                p,
                                tw_set,
                                "backdrop-filter-brightness",
                            ),
                            Filter::Contrast(p) => {
                                resolve_percentage_or_number(p, tw_set, "backdrop-filter-contrast")
                            }
                            Filter::Grayscale(p) => {
                                resolve_percentage_or_number(p, tw_set, "backdrop-filter-grayscale")
                            }
                            Filter::HueRotate(p) => {
                                resolve_keyword(p, tw_set, "backdrop-filter-hue-rotate")
                            }
                            Filter::Invert(p) => {
                                resolve_percentage_or_number(p, tw_set, "backdrop-filter-invert")
                            }
                            // Filter::Opacity(_) => resolve_keyword(p, tw_set, "backdrop-filter-blur"),
                            Filter::Saturate(p) => {
                                resolve_percentage_or_number(p, tw_set, "backdrop-filter-saturate")
                            }
                            Filter::Sepia(p) => {
                                resolve_percentage_or_number(p, tw_set, "backdrop-filter-sepia")
                            }
                            Filter::DropShadow(p) => {
                                resolve_keyword(p, tw_set, "backdrop-filter-drop-shadow")
                            }
                            _ => (),
                        }
                        // resolve_keyword(aaa, tw_set, "filter");
                    }
                }
            },
            Property::ZIndex(p) => resolve_keyword(p, tw_set, "z"),
            // Property::ContainerType(_) => todo!(),
            // Property::ContainerName(_) => todo!(),
            // Property::Container(_) => todo!(),
            // Property::Unparsed(_) => todo!(),
            // Property::Custom(_) => todo!(),
            _ => {}
        }
    }
}

