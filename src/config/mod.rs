use std::{collections::HashMap, hash::Hash};

use lazy_static::lazy_static;

// pub struct Record = Map<String, String>;
#[derive(Default)]
pub type RecordToken = HashMap<&str, &str>;
pub type ArrayRecordToken = HashMap<&str, Vec<&str>>;
enum MapRecordValue {
    MapRecordToken(MapRecordToken), 
    RecordToken(RecordToken), 
    String(String)
}
pub type MapRecordToken = HashMap<&str, MapRecordValue>;

impl RecordToken {
    pub fn default() -> RecordToken {
        HashMap::from::<&str, &str>([ ])
    }

    pub fn default_from_layout()-> RecordToken {
        HashMap::from::<&str, &str>([
            ("xs", ""),
            ("sm", ""),
            ("md", ""),
            ("lg", ""),
            ("xl", ""),
            ("2xl", ""),
            ("3xl", ""),
            ("4xl", ""),
            ("5xl", ""),
            ("6xl", ""),
            ("7xl", ""),
        ])
    }
}


lazy_static! {
pub static TailwindConfigThemeField : Vec<&str> = vec![ 
    "width", "height", "maxWidth", "maxHeight", "minWidth", "minHeight", 
    "inlineSize", "blockSize", "maxInlineSize", "maxBlockSize", "minInlineSize", "minBlockSize", 
    "colors", "fontFamily", "fontSize", "breakpoints", "verticalBreakpoints", "borderRadius", 
    "lineHeight", "letterSpacing", "wordSpacing", "boxShadow", "textIndent", "textShadow", 
    "textStrokeWidth", "blur", "dropShadow", "easing", "lineWidth", "spacing", "duration", 
    "ringWidth", "preflightBase", "containers",  
]; 
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TailwindConfigSet {
    pub width:  Option<RecordToken>,
    pub height: Option<RecordToken>,
    pub max_width: Option<RecordToken>,
    pub max_height: Option<RecordToken>,
    pub min_width: Option<RecordToken>,
    pub min_height: Option<RecordToken>,
    pub inline_size: Option<RecordToken>,
    pub block_size: Option<RecordToken>,
    pub max_inline_size: Option<RecordToken>,
    pub max_block_size: Option<RecordToken>,
    pub min_inline_size: Option<RecordToken>,
    pub min_block_size: Option<RecordToken>,
    pub border_radius: Option<RecordToken>,
    pub breakpoints: Option<RecordToken>,
    pub vertical_breakpoints: Option<RecordToken>,
    pub colors: Option<RecordToken , MapRecordToken>,
    pub font_family: Option<RecordToken, ArrayRecordToken>,
    pub font_size: Option<RecordToken, ArrayRecordToken>,
    pub line_height: Option<RecordToken>,
    pub letter_spacing: Option<RecordToken>,
    pub word_spacing: Option<RecordToken>,
    pub box_shadow:Option<RecordToken, ArrayRecordToken>,
    pub text_indent: Option<RecordToken>,
    pub text_shadow:Option<RecordToken, ArrayRecordToken>,
    pub text_stroke_width: Option<RecordToken>,
    pub ring_width: Option<RecordToken>,
    pub line_width: Option<RecordToken>,
    pub spacing: Option<RecordToken>,
    pub duration: Option<RecordToken>,
    pub aria: Option<RecordToken>,
    pub data: Option<RecordToken>,
    // filters
    pub blur: Option<RecordToken>,
    pub drop_shadow:Option<RecordToken, ArrayRecordToken>,
    // transitions
    pub easing: Option<RecordToken>,
    // media queries
    pub media: Option<RecordToken>,
    // supports queries
    pub supports: Option<RecordToken>,
    // container queries
    pub containers: Option<RecordToken>,
    // animation
    // pub  animation: Option<ThemeAnimation>,
    // grids
    pub grid_auto_column: Option<RecordToken>,
    pub grid_auto_row: Option<RecordToken>,
    pub grid_column: Option<RecordToken>,
    pub grid_row: Option<RecordToken>,
    pub grid_template_column: Option<RecordToken>,
    pub grid_template_row: Option<RecordToken>,
    // container

  // vars
  /** Used to generate CSS variables placeholder in preflight */
//   preflightRoot?: Arrayable<string>
//   preflightBase?: Record<string, string | number>
}


impl TailwindConfigSet {
    pub fn default() -> TailwindConfigSet{
        TailwindConfigSet{
            width: HashMap<String, String>::new(),
            height: HashMap<String, String>::new(),
            max_width: HashMap<String, String>::new(),
            max_height: HashMap<String, String>::new(),
            min_width: HashMap<String, String>::new(),
            min_height: HashMap<String, String>::new(),
            inline_size: HashMap<String, String>::new(),
            block_size: HashMap<String, String>::new(),
            max_inline_size: HashMap<String, String>::new(),
            max_block_size: HashMap<String, String>::new(),
            min_inline_size: HashMap<String, String>::new(),
            min_block_size: HashMap<String, String>::new(),
            border_radius: HashMap<String, String>::new(),
            breakpoints: HashMap<String, String>::new(),
            vertical_breakpoints: HashMap<String, String>::new(),
            colors: HashMap<String, String>::new(),
            font_family: HashMap<String, String>::new(),
            font_size: HashMap<String, String>::new(),
            line_height: HashMap<String, String>::new(),
            letter_spacing: HashMap<String, String>::new(),
            word_spacing: HashMap<String, String>::new(),
            box_shadow: HashMap<String, String>::new(),
            text_indent: HashMap<String, String>::new(),
            text_shadow: HashMap<String, String>::new(),
            text_stroke_width: HashMap<String, String>::new(),
            ring_width: HashMap<String, String>::new(),
            line_width: HashMap<String, String>::new(),
            spacing: HashMap<String, String>::new(),
            duration: HashMap<String, String>::new(),
            aria: HashMap<String, String>::new(),
            data: HashMap<String, String>::new(),
            blur: HashMap<String, String>::new(),
            drop_shadow: HashMap<String, String>::new(),
            easing: HashMap<String, String>::new(),
            media: HashMap<String, String>::new(),
            supports: HashMap<String, String>::new(),
            containers: HashMap<String, String>::new(),
            // animation: HashMap<String, String>::new(),
            grid_auto_column: HashMap<String, String>::new(),
            grid_auto_row: HashMap<String, String>::new(),
            grid_column: HashMap<String, String>::new(),
            grid_row: HashMap<String, String>::new(),
            grid_template_column: HashMap<String, String>::new(),
            grid_template_row: HashMap<String, String>::new(),
        }
    }
    pub fn from_json_string() -> TailwindConfigSet {
        return TailwindConfigSet::default();
        // TailwindConfigSet{}
    }
}



pub struct TailwindRawConfigSet {
    pub theme: TailwindConfigSet,
    // pub rules: Vec<>
}