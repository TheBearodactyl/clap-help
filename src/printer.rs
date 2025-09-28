#![allow(clippy::needless_doctest_main)]

use {
    clap::{ArgAction, Command},
    std::collections::HashMap,
    termimad::{
        minimad::{OwningTemplateExpander, TextTemplate},
        FmtText, MadSkin,
    },
};

/// Default template for the "title" section
pub static TEMPLATE_TITLE: &str = "# **${name}** ${version}";

/// Default template for the "author" section
pub static TEMPLATE_AUTHOR: &str = "
*by* ${author}
";

/// Default template for the "usage" section
pub static TEMPLATE_USAGE: &str = "
**Usage: ** `${name} [options]${positional-args}`
";

/// Default template for the "positionals" section
pub static TEMPLATE_POSITIONALS: &str = "
${positional-lines
* `${key}` : ${help}
}
";

/// Default template for the "description" section
pub static TEMPLATE_DESCRIPTION: &str = "
${about}
";

/// Default template for the "subcommands" section
pub static TEMPLATE_SUBCOMMANDS: &str = "
**Subcommands:**
${subcommand-lines
* `${sub-name}`: ${sub-about}
}
";

/// Default template for an "examples" section
pub static TEMPLATE_EXAMPLES: &str = "
**Examples:**
${after_help}
";

/// Default template for the "options" section
pub static TEMPLATE_OPTIONS: &str = "
**Options:**
|:-:|:-:|:-:|:-|
| short | long | value | description |
|:-:|:-|:-:|:-|
${option-lines
| ${short} | ${long} | ${value} | ${help}${possible_values}${default} |
}
|-
";

/// a template for the "options" section with the value merged to short and long
pub static TEMPLATE_OPTIONS_MERGED_VALUE: &str = "
**Options:**
|:-:|:-:|:-|
|short|long|description|
|:-:|:-|:-|
${option-lines
|${short} *${value-short-braced}*|${long} *${value-long-braced}*|${help}${possible_values}${default}|
}
|-
";

pub static TEMPLATE_OPTIONS_LIST: &str = "
**Options:**
${option-lines
* `${flags-compact}` `${value-braced}`
    *${help}*${details-default}${details-possible-values}${details-env}
}
";

pub static TEMPLATE_OPTIONS_COMPACT_TABLE: &str = "
**Options:**
|:-|:-|
|flags|description|
|---|---|
${option-lines
|`${flags-compact}` `${value-braced}`|${help}${possible_values}${default}|
}
|-
";

pub static TEMPLATE_OPTIONS_VERBOSE: &str = "
**Options:**
${option-lines
---
**`${flags-compact}`** `${value-braced}`
> ${help}
${details-default}${details-possible-values}${details-env}
}
";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StylePreset {
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    RosePineMain,
    RosePineMoon,
    RosePineDawn,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
}

impl StylePreset {
    /// Get all available preset names
    pub fn all_names() -> Vec<&'static str> {
        vec![
            "catppuccin-latte",
            "catppuccin-frappe",
            "catppuccin-macchiato",
            "catppuccin-mocha",
            "rose-pine-main",
            "rose-pine-moon",
            "rose-pine-dawn",
            "kanagawa-wave",
            "kanagawa-dragon",
            "kanagawa-lotus",
        ]
    }

    /// Parse a preset from a string name
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "catppuccin-latte" => Some(Self::CatppuccinLatte),
            "catppuccin-frappe" => Some(Self::CatppuccinFrappe),
            "catppuccin-macchiato" => Some(Self::CatppuccinMacchiato),
            "catppuccin-mocha" => Some(Self::CatppuccinMocha),
            "rose-pine-main" => Some(Self::RosePineMain),
            "rose-pine-moon" => Some(Self::RosePineMoon),
            "rose-pine-dawn" => Some(Self::RosePineDawn),
            "kanagawa-wave" => Some(Self::KanagawaWave),
            "kanagawa-dragon" => Some(Self::KanagawaDragon),
            "kanagawa-lotus" => Some(Self::KanagawaLotus),
            _ => None,
        }
    }

    /// Get the preset name as a string
    pub fn name(&self) -> &'static str {
        match self {
            Self::CatppuccinLatte => "catppuccin-latte",
            Self::CatppuccinFrappe => "catppuccin-frappe",
            Self::CatppuccinMacchiato => "catppuccin-macchiato",
            Self::CatppuccinMocha => "catppuccin-mocha",
            Self::RosePineMain => "rose-pine-main",
            Self::RosePineMoon => "rose-pine-moon",
            Self::RosePineDawn => "rose-pine-dawn",
            Self::KanagawaWave => "kanagawa-wave",
            Self::KanagawaDragon => "kanagawa-dragon",
            Self::KanagawaLotus => "kanagawa-lotus",
        }
    }

    /// Create a MadSkin with the preset's styling
    pub fn create_skin(&self) -> MadSkin {
        match self {
            Self::CatppuccinLatte => {
                let mut skin = MadSkin::default();
                skin.set_headers_fg(termimad::rgb(136, 57, 239));
                skin.bold.set_fg(termimad::rgb(210, 15, 57));
                skin.italic.set_fg(termimad::rgb(234, 118, 203));
                skin.code_block.set_fg(termimad::rgb(64, 160, 43));
                skin.inline_code.set_fg(termimad::rgb(23, 146, 153));
                skin.strikeout.set_fg(termimad::rgb(108, 111, 133));
                skin.paragraph.set_fg(termimad::rgb(76, 79, 105));
                skin
            }
            Self::CatppuccinFrappe => {
                let mut skin = MadSkin::default();
                skin.set_headers_fg(termimad::rgb(202, 158, 230));
                skin.bold.set_fg(termimad::rgb(231, 130, 132));
                skin.italic.set_fg(termimad::rgb(244, 184, 228));
                skin.code_block.set_fg(termimad::rgb(166, 209, 137));
                skin.inline_code.set_fg(termimad::rgb(129, 200, 190));
                skin.strikeout.set_fg(termimad::rgb(165, 173, 206));
                skin.paragraph.set_fg(termimad::rgb(198, 208, 245));
                skin
            }
            Self::CatppuccinMacchiato => {
                let mut skin = MadSkin::default();
                skin.set_headers_fg(termimad::rgb(198, 160, 246));
                skin.bold.set_fg(termimad::rgb(237, 135, 150));
                skin.italic.set_fg(termimad::rgb(245, 189, 230));
                skin.code_block.set_fg(termimad::rgb(166, 218, 149));
                skin.inline_code.set_fg(termimad::rgb(139, 213, 202));
                skin.strikeout.set_fg(termimad::rgb(165, 173, 203));
                skin.paragraph.set_fg(termimad::rgb(202, 211, 245));
                skin
            }
            Self::CatppuccinMocha => {
                let mut skin = MadSkin::default();
                skin.set_headers_fg(termimad::rgb(203, 166, 247));
                skin.bold.set_fg(termimad::rgb(243, 139, 168));
                skin.italic.set_fg(termimad::rgb(245, 194, 231));
                skin.code_block.set_fg(termimad::rgb(166, 227, 161));
                skin.inline_code.set_fg(termimad::rgb(148, 226, 213));
                skin.strikeout.set_fg(termimad::rgb(166, 173, 200));
                skin.paragraph.set_fg(termimad::rgb(205, 214, 244));
                skin
            }
            Self::RosePineMain => {
                let mut skin = MadSkin::default();
                skin.set_headers_fg(termimad::rgb(196, 167, 231));
                skin.bold.set_fg(termimad::rgb(235, 111, 146));
                skin.italic.set_fg(termimad::rgb(246, 193, 119));
                skin.code_block.set_fg(termimad::rgb(49, 116, 143));
                skin.inline_code.set_fg(termimad::rgb(156, 207, 216));
                skin.strikeout.set_fg(termimad::rgb(110, 106, 134));
                skin.paragraph.set_fg(termimad::rgb(224, 222, 244));
                skin
            }
            Self::RosePineMoon => {
                let mut skin = MadSkin::default();
                skin.set_headers_fg(termimad::rgb(196, 167, 231));
                skin.bold.set_fg(termimad::rgb(235, 111, 146));
                skin.italic.set_fg(termimad::rgb(246, 193, 119));
                skin.code_block.set_fg(termimad::rgb(62, 143, 176));
                skin.inline_code.set_fg(termimad::rgb(156, 207, 216));
                skin.strikeout.set_fg(termimad::rgb(110, 106, 134));
                skin.paragraph.set_fg(termimad::rgb(224, 222, 244));
                skin
            }
            Self::RosePineDawn => {
                let mut skin = MadSkin::default();
                skin.set_headers_fg(termimad::rgb(144, 122, 169));
                skin.bold.set_fg(termimad::rgb(180, 99, 122));
                skin.italic.set_fg(termimad::rgb(234, 157, 52));
                skin.code_block.set_fg(termimad::rgb(40, 105, 131));
                skin.inline_code.set_fg(termimad::rgb(86, 148, 159));
                skin.strikeout.set_fg(termimad::rgb(152, 147, 165));
                skin.paragraph.set_fg(termimad::rgb(87, 82, 121));
                skin
            }
            Self::KanagawaWave => {
                let mut skin = MadSkin::default();
                skin.set_headers_fg(termimad::rgb(149, 127, 184));
                skin.bold.set_fg(termimad::rgb(192, 163, 110));
                skin.italic.set_fg(termimad::rgb(255, 160, 102));
                skin.code_block.set_fg(termimad::rgb(118, 148, 106));
                skin.inline_code.set_fg(termimad::rgb(122, 168, 159));
                skin.strikeout.set_fg(termimad::rgb(84, 84, 109));
                skin.paragraph.set_fg(termimad::rgb(220, 215, 186));
                skin
            }
            Self::KanagawaDragon => {
                let mut skin = MadSkin::default();
                skin.set_headers_fg(termimad::rgb(139, 164, 176));
                skin.bold.set_fg(termimad::rgb(196, 116, 110));
                skin.italic.set_fg(termimad::rgb(196, 178, 138));
                skin.code_block.set_fg(termimad::rgb(135, 169, 135));
                skin.inline_code.set_fg(termimad::rgb(142, 164, 162));
                skin.strikeout.set_fg(termimad::rgb(98, 94, 90));
                skin.paragraph.set_fg(termimad::rgb(197, 201, 197));
                skin
            }
            Self::KanagawaLotus => {
                let mut skin = MadSkin::default();
                skin.set_headers_fg(termimad::rgb(111, 92, 124));
                skin.bold.set_fg(termimad::rgb(200, 64, 83));
                skin.italic.set_fg(termimad::rgb(204, 109, 0));
                skin.code_block.set_fg(termimad::rgb(111, 137, 78));
                skin.inline_code.set_fg(termimad::rgb(89, 123, 117));
                skin.strikeout.set_fg(termimad::rgb(113, 110, 97));
                skin.paragraph.set_fg(termimad::rgb(84, 84, 100));
                skin
            }
        }
    }

    /// Check if this is a light theme
    pub fn is_light(&self) -> bool {
        matches!(
            self,
            Self::CatppuccinLatte | Self::RosePineDawn | Self::KanagawaLotus
        )
    }

    /// Get the theme family name
    pub fn family(&self) -> &'static str {
        match self {
            Self::CatppuccinLatte
            | Self::CatppuccinFrappe
            | Self::CatppuccinMacchiato
            | Self::CatppuccinMocha => "Catppuccin",
            Self::RosePineMain | Self::RosePineMoon | Self::RosePineDawn => "Rose Pine",
            Self::KanagawaWave | Self::KanagawaDragon | Self::KanagawaLotus => "Kanagawa",
        }
    }

    /// Get the variant name within the theme family
    pub fn variant(&self) -> &'static str {
        match self {
            Self::CatppuccinLatte => "Latte",
            Self::CatppuccinFrappe => "Frappé",
            Self::CatppuccinMacchiato => "Macchiato",
            Self::CatppuccinMocha => "Mocha",
            Self::RosePineMain => "Main",
            Self::RosePineMoon => "Moon",
            Self::RosePineDawn => "Dawn",
            Self::KanagawaWave => "Wave",
            Self::KanagawaDragon => "Dragon",
            Self::KanagawaLotus => "Lotus",
        }
    }
}

/// Keys used to enable/disable/change templates
pub static TEMPLATES: &[&str] = &[
    "title",
    "author",
    "description",
    "introduction",
    "usage",
    "positionals",
    "options",
    "subcommands",
    "examples",
];

/// An object which you can configure to print the help of a command
///
/// For example, changing the color of bold text and using an alternate
///   template for the options section:
///
/// ```rust
/// use clap::{CommandFactory, Parser, ValueEnum};
/// use clap_help::Printer;
///
/// #[derive(Parser, Debug)]
/// #[command(author, version, about, disable_help_flag = true)]
/// struct Args {
///
///     /// Print help
///     #[arg(long)]
///     help: bool,
///
///     /// Comma separated list of features
///     #[clap(long, value_name = "features")]
///     pub features: Option<String>,
/// }
///
/// fn main() {
///     let args = Args::parse();
///     if args.help {
///         let mut printer = clap_help::Printer::new(Args::command())
///             .with("options", clap_help::TEMPLATE_OPTIONS_MERGED_VALUE);
///         printer.skin_mut().bold.set_fg(termimad::ansi(204));
///         printer.print_help();
///         return;
///     }
///     // rest of the program
/// }
///
/// ```
pub struct Printer<'t> {
    skin: MadSkin,
    expander: OwningTemplateExpander<'static>,
    template_keys: Vec<&'static str>,
    templates: HashMap<&'static str, &'t str>,
    pub full_width: bool,
    pub max_width: Option<usize>,
}

impl<'t> Printer<'t> {
    pub fn new(mut cmd: Command) -> Self {
        cmd.build();
        let expander = Self::make_expander(&cmd);
        let mut templates = HashMap::new();
        templates.insert("title", TEMPLATE_TITLE);
        templates.insert("author", TEMPLATE_AUTHOR);
        templates.insert("usage", TEMPLATE_USAGE);
        templates.insert("positionals", TEMPLATE_POSITIONALS);
        templates.insert("options", TEMPLATE_OPTIONS);
        templates.insert("description", TEMPLATE_DESCRIPTION);
        templates.insert("subcommands", TEMPLATE_SUBCOMMANDS);
        templates.insert("examples", TEMPLATE_EXAMPLES);
        Self {
            skin: Self::make_skin(),
            expander,
            templates,
            template_keys: TEMPLATES.to_vec(),
            full_width: false,
            max_width: None,
        }
    }

    /// Build a skin for the detected theme of the terminal
    /// (i.e. dark, light, or other)
    pub fn make_skin() -> MadSkin {
        match terminal_light::luma() {
            Ok(luma) if luma > 0.85 => MadSkin::default_light(),
            Ok(luma) if luma < 0.2 => MadSkin::default_dark(),
            _ => MadSkin::default(),
        }
    }

    /// Use the provided skin
    pub fn with_skin(mut self, skin: MadSkin) -> Self {
        self.skin = skin;
        self
    }

    /// Set a maximal width, so that the whole terminal width isn't used.
    ///
    /// This may make some long sentences easier to read on super wide
    /// terminals, especially when the whole text is short.
    /// Depending on your texts and parameters, you may set up a width
    /// of 100 or 150.
    pub fn with_max_width(mut self, w: usize) -> Self {
        self.max_width = Some(w);
        self
    }

    /// Give a mutable reference to the current skin
    /// (by default the automatically selected one)
    /// so that it can be modified
    pub fn skin_mut(&mut self) -> &mut MadSkin {
        &mut self.skin
    }

    /// Change a template
    pub fn set_template(&mut self, key: &'static str, template: &'t str) {
        self.templates.insert(key, template);
    }

    /// Change or add a template
    pub fn with(mut self, key: &'static str, template: &'t str) -> Self {
        self.set_template(key, template);
        self
    }

    /// Unset a template
    pub fn without(mut self, key: &'static str) -> Self {
        self.templates.remove(key);
        self
    }

    /// A mutable reference to the list of template keys, so that you can
    /// insert new keys, or change their order.
    /// Any key without matching template will just be ignored
    pub fn template_keys_mut(&mut self) -> &mut Vec<&'static str> {
        &mut self.template_keys
    }

    /// A mutable reference to the list of template keys, so that you can
    /// insert new keys, or change their order.
    /// Any key without matching template will just be ignored
    #[deprecated(since = "0.6.2", note = "use template_keys_mut instead")]
    pub fn template_order_mut(&mut self) -> &mut Vec<&'static str> {
        &mut self.template_keys
    }

    fn make_expander(cmd: &Command) -> OwningTemplateExpander<'static> {
        let mut expander = OwningTemplateExpander::new();
        expander.set_default("");
        let name = cmd.get_bin_name().unwrap_or_else(|| cmd.get_name());
        expander.set("name", name);

        if let Some(author) = cmd.get_author() {
            expander.set("author", author);
        }
        if let Some(version) = cmd.get_version() {
            expander.set("version", version);
        }

        if let Some(about) = cmd.get_about() {
            expander.set_md("about", about.to_string());
        }

        if let Some(after_help) = cmd.get_after_help() {
            expander.set_md("after_help", after_help.to_string());
        }

        let options = cmd
            .get_arguments()
            .filter(|a| !a.is_hide_set())
            .filter(|a| a.get_short().is_some() || a.get_long().is_some());

        for arg in options {
            let sub = expander.sub("option-lines");

            let short_flag = arg.get_short();
            let long_flag = arg.get_long();

            let flags_compact = match (short_flag, long_flag) {
                (Some(s), Some(l)) => format!("-{s}, --{l}"),
                (None, Some(l)) => format!("    --{l}"),
                (Some(s), None) => format!("-{s}"),
                (None, None) => String::new(),
            };
            sub.set("flags-compact", flags_compact);

            if let Some(short) = short_flag {
                sub.set("short", format!("-{short}"));
            }
            if let Some(long) = long_flag {
                sub.set("long", format!("--{long}"));
            }

            if let Some(help) = arg.get_help() {
                sub.set_md("help", help.to_string());
            }

            if arg.get_action().takes_values() {
                if let Some(name) = arg.get_value_names().and_then(|arr| arr.first()) {
                    sub.set("value", name);
                    let braced = format!("<{}>", name);
                    sub.set("value-braced", &braced);
                    if arg.get_short().is_some() {
                        sub.set("value-short-braced", &braced);
                        sub.set("value-short", name);
                    }
                    if arg.get_long().is_some() {
                        sub.set("value-long-braced", &braced);
                        sub.set("value-long", name);
                    }
                };
            }

            if let Some(env_var) = arg.get_env() {
                sub.set_md(
                    "details-env",
                    format!("\n    * *Environment*: `{}`", env_var.to_string_lossy()),
                );
            }

            let mut possible_values = arg.get_possible_values();
            if !possible_values.is_empty() {
                let values: Vec<String> = possible_values
                    .drain(..)
                    .map(|v| format!("`{}`", v.get_name()))
                    .collect();
                sub.set_md(
                    "possible_values",
                    format!(" Possible values: [{}]", values.join(", ")),
                );
                sub.set_md(
                    "details-possible-values",
                    format!("\n    * *Possible values*: {}", values.join(", ")),
                );
            }

            if let Some(default) = arg.get_default_values().first() {
                if matches!(arg.get_action(), ArgAction::Set | ArgAction::Append) {
                    let default_str = default.to_string_lossy();
                    sub.set_md("default", format!(" Default: `{}`", &default_str));
                    sub.set_md(
                        "details-default",
                        format!("\n    * *Default*: `{}`", &default_str),
                    );
                }
            }
        }

        let mut args = String::new();
        for arg in cmd.get_positionals() {
            let Some(key) = arg.get_value_names().and_then(|arr| arr.first()) else {
                continue;
            };
            args.push(' ');
            if !arg.is_required_set() {
                args.push('[');
            }
            if arg.is_last_set() {
                args.push_str("-- ");
            }
            args.push_str(key);
            if !arg.is_required_set() {
                args.push(']');
            }
            let sub = expander.sub("positional-lines");
            sub.set("key", key);
            if let Some(help) = arg.get_help() {
                sub.set("help", help);
            }
        }
        expander.set("positional-args", args);

        for subcmd in cmd.get_subcommands() {
            let sub = expander.sub("subcommand-lines");
            sub.set("sub-name", subcmd.get_name());
            if let Some(about) = subcmd.get_about() {
                sub.set_md("sub-about", about.to_string());
            }
        }

        expander
    }

    /// Give you a mut reference to the expander, so that you can overload
    /// the variable of the expander used to fill the templates of the help,
    /// or add new variables for your own templates
    pub fn expander_mut(&mut self) -> &mut OwningTemplateExpander<'static> {
        &mut self.expander
    }

    /// Print the provided template with the printer's expander
    ///
    /// It's normally more convenient to change template_keys or some
    /// templates, unless you want none of the standard templates
    pub fn print_template(&self, template: &str) {
        self.skin.print_owning_expander_md(&self.expander, template);
    }

    /// Print all the templates, in order
    pub fn print_help(&self) {
        if self.full_width {
            self.print_help_full_width()
        } else {
            self.print_help_content_width()
        }
    }

    fn print_help_full_width(&self) {
        for key in &self.template_keys {
            if let Some(template) = self.templates.get(key) {
                self.print_template(template);
            }
        }
    }

    fn print_help_content_width(&self) {
        let (width, _) = termimad::terminal_size();
        let mut width = width as usize;
        if let Some(max_width) = self.max_width {
            width = width.min(max_width);
        }
        let mut texts: Vec<FmtText> = self
            .template_keys
            .iter()
            .filter_map(|key| self.templates.get(key))
            .map(|&template| {
                let template = TextTemplate::from(template);
                let text = self.expander.expand(&template);
                FmtText::from_text(&self.skin, text, Some(width))
            })
            .collect();
        let content_width = texts
            .iter()
            .fold(0, |cw, text| cw.max(text.content_width()));
        for text in &mut texts {
            text.set_rendering_width(content_width);
            println!("{}", text);
        }
    }
}
