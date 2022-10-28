use lightningcss::{
    properties::{
        border::BorderSideWidth,
        size::{BoxSizing, MaxSize, Size},
        Property,
    },
    rules::{style::StyleRule, CssRule},
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    traits::ToCss,
    values::{
        calc::Calc,
        length::{LengthPercentage, LengthPercentageOrAuto, LengthValue},
        percentage::{DimensionPercentage, Percentage},
    },
};

use crate::tailwind_token::TailwindTokenSet;

pub fn resolve_style(rule: &StyleRule, tw_set: &mut TailwindTokenSet) {
    for prop in &rule.declarations.declarations {
        // prop.value_to_css_string(PrinterOptions::default());
        match prop {
            // Property::BackgroundColor(_) => todo!(),
            // Property::BackgroundImage(_) => todo!(),
            // Property::BackgroundPositionX(_) => todo!(),
            // Property::BackgroundPositionY(_) => todo!(),
            // Property::BackgroundPosition(_) => todo!(),
            // Property::BackgroundSize(_) => todo!(),
            // Property::BackgroundRepeat(_) => todo!(),
            // Property::BackgroundAttachment(_) => todo!(),
            // Property::BackgroundClip(_, _) => todo!(),
            Property::BackgroundOrigin(p) => resolve_keyword(p, tw_set, "bg-origin"),
            // Property::Background(_) => todo!(),
            // Property::BoxShadow(_, _) => todo!(),
            Property::Opacity(p) => resolve_keyword(p, tw_set, "opacity"),
            // Property::Color(_) => todo!(),
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
            // Property::BoxSizing(_, _) => todo!(),
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
            // Property::BorderTopColor(_) => todo!(),
            // Property::BorderBottomColor(_) => todo!(),
            // Property::BorderLeftColor(_) => todo!(),
            // Property::BorderRightColor(_) => todo!(),
            // Property::BorderBlockStartColor(_) => todo!(),
            // Property::BorderBlockEndColor(_) => todo!(),
            // Property::BorderInlineStartColor(_) => todo!(),
            // Property::BorderInlineEndColor(_) => todo!(),
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
            // Property::BorderColor(_) => todo!(),
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
            // Property::BorderBlockColor(_) => todo!(),
            Property::BorderBlockStyle(p) => {
                resolve_keyword(&p.start, tw_set, "b-t");
                resolve_keyword(&p.end, tw_set, "b-b");
            }
            Property::BorderBlockWidth(p) => {
                resolve_border_side_width(&p.start, tw_set, "b-t");
                resolve_border_side_width(&p.end, tw_set, "b-b");
            }
            // Property::BorderInlineColor(_) => todo!(),
            Property::BorderInlineStyle(p) => {
                resolve_keyword(&p.start, tw_set, "b-l");
                resolve_keyword(&p.end, tw_set, "b-r");
            }
            Property::BorderInlineWidth(p) => {
                resolve_border_side_width(&p.start, tw_set, "b-l");
                resolve_border_side_width(&p.end, tw_set, "b-r");
            }
            Property::Border(p) => {
                resolve_keyword(&p.style, tw_set, "b-a");
                resolve_border_side_width(&p.width, tw_set, "b-a");
            }
            Property::BorderTop(p) => {
                resolve_keyword(&p.style, tw_set, "b-t");
                resolve_border_side_width(&p.width, tw_set, "b-t");
            }
            Property::BorderBottom(p) => {
                resolve_keyword(&p.style, tw_set, "b-b");
                resolve_border_side_width(&p.width, tw_set, "b-b");
            }
            Property::BorderLeft(p) => {
                resolve_keyword(&p.style, tw_set, "b-l");
                resolve_border_side_width(&p.width, tw_set, "b-l");
            }
            Property::BorderRight(p) => {
                resolve_keyword(&p.style, tw_set, "b-r");
                resolve_border_side_width(&p.width, tw_set, "b-r");
            }
            Property::BorderBlock(p) => {
                // todo: color

                resolve_keyword(&p.style, tw_set, "b-t");
                resolve_border_side_width(&p.width, tw_set, "b-t");

                resolve_keyword(&p.style, tw_set, "b-b");
                resolve_border_side_width(&p.width, tw_set, "b-b");
            }
            Property::BorderBlockStart(p) => {
                // todo: color
                resolve_keyword(&p.style, tw_set, "b-t");
                resolve_border_side_width(&p.width, tw_set, "b-t");
            }
            Property::BorderBlockEnd(p) => {
                // todo: color
                resolve_keyword(&p.style, tw_set, "b-b");
                resolve_border_side_width(&p.width, tw_set, "b-b");
            }
            Property::BorderInline(p) => {
                // todo: color
                resolve_keyword(&p.style, tw_set, "b-l");
                resolve_border_side_width(&p.width, tw_set, "b-l");
                resolve_keyword(&p.style, tw_set, "b-r");
                resolve_border_side_width(&p.width, tw_set, "b-r");
            }
            Property::BorderInlineStart(p) => {
                // todo: color
                resolve_keyword(&p.style, tw_set, "b-l");
                resolve_border_side_width(&p.width, tw_set, "b-l");
            }
            Property::BorderInlineEnd(p) => {
                // #todo : color
                resolve_keyword(&p.style, tw_set, "b-r");
                resolve_border_side_width(&p.width, tw_set, "b-r");
            }
            Property::Outline(p) => {
                // #todo : color
                resolve_border_side_width(&p.width, tw_set, "outline");
                resolve_keyword(&p.style, tw_set, "outline");
            }
            // Property::OutlineColor(_) => todo!(),
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
            // Property::Flex(_, _) => todo!(),
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
            Property::RowGap(p) => resolve_keyword(p, tw_set, "gap"),
            Property::ColumnGap(p) => resolve_keyword(p, tw_set, "gap"),
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
            // Property::GridTemplateColumns(_) => todo!(),
            // Property::GridTemplateRows(_) => todo!(),
            // Property::GridAutoColumns(_) => todo!(),
            // Property::GridAutoRows(_) => todo!(),
            // Property::GridAutoFlow(_) => todo!(),
            // Property::GridTemplateAreas(_) => todo!(),
            // Property::GridTemplate(_) => todo!(),
            // Property::Grid(_) => todo!(),
            // Property::GridRowStart(_) => todo!(),
            // Property::GridRowEnd(_) => todo!(),
            // Property::GridColumnStart(_) => todo!(),
            // Property::GridColumnEnd(_) => todo!(),
            // Property::GridRow(_) => todo!(),
            // Property::GridColumn(_) => todo!(),
            // Property::GridArea(_) => todo!(),
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

            Property::FontWeight(p) => resolve_keyword(p, tw_set, "font"),
            // Property::FontSize(_) => todo!(),
            // Property::FontStretch(_) => todo!(),
            // Property::FontFamily(_) => {},
            Property::FontStyle(p) => match p {
                lightningcss::properties::font::FontStyle::Normal => {}
                _ => resolve_keyword(p, tw_set, "font"),
            },
            // Property::FontVariantCaps(_) => todo!(),
            // Property::LineHeight(_) => todo!(),
            // Property::Font(_) => todo!(),
            // Property::VerticalAlign(_) => todo!(),
            // Property::FontPalette(_) => todo!(),
            // Property::TransitionProperty(_, _) => todo!(),
            // Property::TransitionDuration(_, _) => todo!(),
            // Property::TransitionDelay(_, _) => todo!(),
            // Property::TransitionTimingFunction(_, _) => todo!(),
            // Property::Transition(_, _) => todo!(),
            // Property::AnimationName(_, _) => todo!(),
            // Property::AnimationDuration(_, _) => todo!(),
            // Property::AnimationTimingFunction(_, _) => todo!(),
            // Property::AnimationIterationCount(_, _) => todo!(),
            // Property::AnimationDirection(_, _) => todo!(),
            // Property::AnimationPlayState(_, _) => todo!(),
            // Property::AnimationDelay(_, _) => todo!(),
            // Property::AnimationFillMode(_, _) => todo!(),
            // Property::Animation(_, _) => todo!(),
            // Property::Transform(_, _) => todo!(),
            // Property::TransformOrigin(_, _) => todo!(),
            // Property::TransformStyle(_, _) => todo!(),
            // Property::TransformBox(_) => todo!(),
            // Property::BackfaceVisibility(_, _) => todo!(),
            Property::Perspective(p, _) => resolve_keyword(p, tw_set, "perspective"),
            // Property::PerspectiveOrigin(_, _) => todo!(),
            // Property::Translate(_) => todo!(),
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
            // Property::TextTransform(_) => todo!(),
            // Property::WhiteSpace(_) => todo!(),
            // Property::TabSize(_, _) => todo!(),
            // Property::WordBreak(_) => todo!(),
            // Property::LineBreak(_) => todo!(),
            // Property::Hyphens(_, _) => todo!(),
            // Property::OverflowWrap(_) => todo!(),
            // Property::WordWrap(_) => todo!(),
            Property::TextAlign(p) => resolve_keyword(p, tw_set, "text"),
            Property::TextAlignLast(p, _) => resolve_keyword(p, tw_set, "last-text"),
            // Property::TextJustify(_) => todo!(),
            // Property::WordSpacing(_) => todo!(),
            // Property::LetterSpacing(_) => todo!(),
            // Property::TextIndent(_) => todo!(),
            // Property::TextDecorationLine(_, _) => todo!(),
            // Property::TextDecorationStyle(_, _) => todo!(),
            // Property::TextDecorationColor(_, _) => todo!(),
            // Property::TextDecorationThickness(_) => todo!(),
            // Property::TextDecoration(_, _) => todo!(),
            // Property::TextDecorationSkipInk(_, _) => todo!(),
            // Property::TextEmphasisStyle(_, _) => todo!(),
            // Property::TextEmphasisColor(_, _) => todo!(),
            // Property::TextEmphasis(_, _) => todo!(),
            // Property::TextEmphasisPosition(_, _) => todo!(),
            // Property::TextShadow(_) => todo!(),
            // Property::BoxDecorationBreak(_, _) => todo!(),
            // Property::Resize(_) => todo!(),
            Property::Cursor(p) => resolve_keyword(p, tw_set, "cursor"),
            // Property::CaretColor(_) => todo!(),
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
            Property::Fill(p) => resolve_keyword(p, tw_set, "fill"),
            // Property::FillRule(_) => todo!(),
            // Property::FillOpacity(_) => todo!(),
            // Property::Stroke(_) => todo!(),
            // Property::StrokeOpacity(_) => todo!(),
            Property::StrokeWidth(p) => resolve_dimension(p, tw_set, "stroke"),
            // Property::StrokeLinecap(_) => todo!(),
            // Property::StrokeLinejoin(_) => todo!(),
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
            // Property::Mask(_, _) => todo!(),
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
                lightningcss::properties::effects::FilterList::None => {
                    tw_set.push_tailwind_token("filter", "none")
                }
                lightningcss::properties::effects::FilterList::Filters(dd) => {
                    for aaa in dd {
                        resolve_keyword(aaa, tw_set, "filter");
                    }
                }
            },
            Property::BackdropFilter(p, _) => match p {
                lightningcss::properties::effects::FilterList::None => {
                    tw_set.push_tailwind_token("filter", "none")
                }
                lightningcss::properties::effects::FilterList::Filters(dd) => {
                    for aaa in dd {
                        resolve_keyword(aaa, tw_set, "filter");
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

fn resolve_length_length_percentage_or_auto(
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

fn resolve_dimension(
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

fn resolve_length_size(income_value: &Size, tw_set: &mut TailwindTokenSet, token_prefix: &str) {
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
fn resolve_length_max_size(
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

fn resolve_keyword<F: ToCss>(income_value: &F, tw_set: &mut TailwindTokenSet, token_prefix: &str) {
    tw_set.push_tailwind_token(
        token_prefix,
        income_value
            .to_css_string(PrinterOptions::default())
            .unwrap(),
    );
}

fn resolve_border_side_width(
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
