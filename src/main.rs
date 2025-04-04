use nu_plugin::{self, EvaluatedCall, Plugin, PluginCommand, SimplePluginCommand};
use nu_plugin_image::{ansi_to_image, image_to_ansi, logging::logger, FontFamily, Palette};
use nu_protocol::{Category, Signature, SyntaxShape, Type, Value};

pub struct ImageConversionPlugin;

impl Plugin for ImageConversionPlugin {
    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(FromPngCommand::new()),
            Box::new(ToPngCommand::new()),
        ]
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }
}
struct FromPngCommand;

impl FromPngCommand {
    pub fn new() -> FromPngCommand {
        FromPngCommand {}
    }
}
impl SimplePluginCommand for FromPngCommand {
    type Plugin = ImageConversionPlugin;

    fn name(&self) -> &str {
        "from png"
    }

    fn signature(&self) -> Signature {
        Signature::build("from png")
            .named(
                "width",
                SyntaxShape::Int,
                "Output width, in characters.",
                Some('x'),
            )
            .named(
                "height",
                SyntaxShape::Int,
                "Output height, in characters.",
                Some('y'),
            )
            .named(
                "log-level",
                SyntaxShape::String,
                "sets log level (CRITICAL (c) ERROR (e) WARN (w) INFO (i) DEBUG (d) TRACE (t)) defaults to INFO",
                None,
            )
            .input_output_type(Type::Binary, Type::String)
            .category(Category::Conversions)
    }

    fn description(&self) -> &str {
        "create ansi text from an image"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        if let Some(Value::String { val, .. }) = call.get_flag_value("log-level") {
            logger::set_verbose(val);
        }
        image_to_ansi(call, input)
    }
}
struct ToPngCommand;
impl ToPngCommand {
    pub fn new() -> ToPngCommand {
        ToPngCommand {}
    }
}
impl SimplePluginCommand for ToPngCommand {
    type Plugin = ImageConversionPlugin;

    fn name(&self) -> &str {
        "to png"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("to png")
        .optional(
            "output-path",
            SyntaxShape::Filepath,
            "output file path (by default uses current timestamp)",
        )
                .named("width", SyntaxShape::Int, "output width", Some('w'))
                .named("theme",SyntaxShape::String,format!("select theme of the output, one of: {:?}\n\t\tby default uses `vscode` theme and you can mix this flag with custom theme colors every other colors will be from the selected theme",Palette::list()),Some('t'))
                .named(
                    "font",
                    SyntaxShape::String,
                    format!(
                        "Select the font from one of {:?}, by default the first font in the list will be used",
                        FontFamily::list()
                    ),
                    None,
                )
                .named("custom-font-regular", SyntaxShape::Filepath, "custom font Regular font path", None)
                .named("custom-font-bold", SyntaxShape::Filepath, "custom font Bold font path", None)
                .named("custom-font-italic", SyntaxShape::Filepath, "custom font Italic font path", None)
                .named("custom-font-bold_italic", SyntaxShape::Filepath, "custom font Bold Italic font path", None)
                .named("custom-theme-fg", SyntaxShape::String, "custom foreground color in hex format (0x040404)", None)
                .named("custom-theme-bg", SyntaxShape::String, "custom background color in hex format (0x040404)", None)
                .named("custom-theme-black", SyntaxShape::String, "custom black color in hex format (0x040404)", None)
                .named("custom-theme-red", SyntaxShape::String, "custom red color in hex format (0x040404)", None)
                .named("custom-theme-green", SyntaxShape::String, "custom green color in hex format (0x040404)", None)
                .named("custom-theme-yellow", SyntaxShape::String, "custom yellow color in hex format (0x040404)", None)
                .named("custom-theme-blue", SyntaxShape::String, "custom blue color in hex format (0x040404)", None)
                .named("custom-theme-magenta", SyntaxShape::String, "custom magenta color in hex format (0x040404)", None)
                .named("custom-theme-cyan", SyntaxShape::String, "custom cyan color in hex format (0x040404)", None)
                .named("custom-theme-white", SyntaxShape::String, "custom white color in hex format (0x040404)", None)
                .named("custom-theme-bright_black", SyntaxShape::String, "custom bright black color in hex format (0x040404)", None)
                .named("custom-theme-bright_red", SyntaxShape::String, "custom bright red color in hex format (0x040404)", None)
                .named("custom-theme-bright_green", SyntaxShape::String, "custom bright green color in hex format (0x040404)", None)
                .named("custom-theme-bright_yellow", SyntaxShape::String, "custom bright yellow color in hex format (0x040404)", None)
                .named("custom-theme-bright_blue", SyntaxShape::String, "custom bright blue color in hex format (0x040404)", None)
                .named("custom-theme-bright_magenta", SyntaxShape::String, "custom bright magenta color in hex format (0x040404)", None)
                .named("custom-theme-bright_cyan", SyntaxShape::String, "custom bright cyan color in hex format (0x040404)", None)
                .named("custom-theme-bright_white", SyntaxShape::String, "custom bright white color in hex format (0x040404)", None)
                .named(
                    "log-level",
                    SyntaxShape::String,
                    "sets log level (CRITICAL (c) ERROR (e) WARN (w) INFO (i) DEBUG (d) TRACE (t)) defaults to INFO",
                    None,
                )
                .input_output_type(Type::String, Type::String)
                // .plugin_examples(
                //     vec![
                //         PluginExample{
                //             description: "creates image of `ls` command's output and save it in the `ls.png` file".to_string(),
                //             example: "ls | table -c | to png --theme ubuntu --font Ubuntu --output-path ls.png".to_string(),
                //             result: None,
                //         },
                //         PluginExample{
                //             description: "creates image of `ls` command's output and save it in the `ls.png` file with custom greenish background color".to_string(),
                //             example: "ls | table -c | to png --theme ubuntu --font Ubuntu --custom-theme-bg 0x112411 --output-path ls.png".to_string(),
                //             result: None, 
                //         },
                //     ]
                // )
                .category(Category::Conversions)
    }

    fn description(&self) -> &str {
        "converts ansi string into png image"
    }
    fn extra_description(&self) -> &str {
        "if you change font and theme they will be used as base theme of the output and every custom flag you provide will override the selected theme or font"
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        engine: &nu_plugin::EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        if let Some(Value::String { val, .. }) = call.get_flag_value("log-level") {
            logger::set_verbose(val);
        }
        ansi_to_image(engine, call, input)
    }
}

fn main() {
    nu_plugin::serve_plugin(
        &mut ImageConversionPlugin {},
        nu_plugin::MsgPackSerializer {},
    )
}
