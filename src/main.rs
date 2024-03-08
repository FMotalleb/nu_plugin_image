
use nu_plugin::{self, EvaluatedCall, LabeledError};
use nu_plugin_image::{ansi_to_image, image_to_ansi, FontFamily, Palette};
use nu_protocol::{Category, PluginExample, PluginSignature, SyntaxShape, Type, Value};

pub struct Plugin;

impl nu_plugin::Plugin for Plugin {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("from png")
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
                .switch(
                    "verbose",
                    "prints log of the work into the terminal",
                    Some('v'),
                )
                .usage("create ansi text from an image")
                .input_output_type(Type::Binary, Type::String)
                .category(Category::Conversions),
            PluginSignature::build("to png")
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
                .named("custom-theme-fg", SyntaxShape::Int, "custom foreground color in hex format (0x040404)", None)
                .named("custom-theme-bg", SyntaxShape::Int, "custom background color in hex format (0x040404)", None)
                .named("custom-theme-black", SyntaxShape::Int, "custom black color in hex format (0x040404)", None)
                .named("custom-theme-red", SyntaxShape::Int, "custom red color in hex format (0x040404)", None)
                .named("custom-theme-green", SyntaxShape::Int, "custom green color in hex format (0x040404)", None)
                .named("custom-theme-yellow", SyntaxShape::Int, "custom yellow color in hex format (0x040404)", None)
                .named("custom-theme-blue", SyntaxShape::Int, "custom blue color in hex format (0x040404)", None)
                .named("custom-theme-magenta", SyntaxShape::Int, "custom magenta color in hex format (0x040404)", None)
                .named("custom-theme-cyan", SyntaxShape::Int, "custom cyan color in hex format (0x040404)", None)
                .named("custom-theme-white", SyntaxShape::Int, "custom white color in hex format (0x040404)", None)
                .named("custom-theme-bright_black", SyntaxShape::Int, "custom bright black color in hex format (0x040404)", None)
                .named("custom-theme-bright_red", SyntaxShape::Int, "custom bright red color in hex format (0x040404)", None)
                .named("custom-theme-bright_green", SyntaxShape::Int, "custom bright green color in hex format (0x040404)", None)
                .named("custom-theme-bright_yellow", SyntaxShape::Int, "custom bright yellow color in hex format (0x040404)", None)
                .named("custom-theme-bright_blue", SyntaxShape::Int, "custom bright blue color in hex format (0x040404)", None)
                .named("custom-theme-bright_magenta", SyntaxShape::Int, "custom bright magenta color in hex format (0x040404)", None)
                .named("custom-theme-bright_cyan", SyntaxShape::Int, "custom bright cyan color in hex format (0x040404)", None)
                .named("custom-theme-bright_white", SyntaxShape::Int, "custom bright white color in hex format (0x040404)", None)
                .switch(
                    "verbose",
                    "prints log of the work into the terminal",
                    Some('v'),
                )
                .usage("converts ansi string into png image")
                .extra_usage("if you change font and theme they will be used as base theme of the output and every custom flag you provide will override the selected theme or font")
                .input_output_type(Type::String, Type::Nothing)
                .input_output_type(Type::Binary, Type::Nothing)
                .plugin_examples(
                    vec![
                        PluginExample{
                            description: "creates image of `ls` command's output and save it in the `ls.png` file".to_string(),
                            example: "ls | table -c | to png ls.png --theme ubuntu --font Ubuntu".to_string(),
                            result: None,
                        },
                        PluginExample{
                            description: "creates image of `ls` command's output and save it in the `ls.png` file with custom greenish background color".to_string(),
                            example: "ls | table -c | to png ls.png --theme ubuntu --font Ubuntu --custom-theme-bg 0x112411".to_string(),
                            result: None, 
                        },
                    ]
                )
                .category(Category::Conversions),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        _config: &Option<Value>,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        if let Ok(is_verbose) = call.has_flag("verbose"){
            nu_plugin_image::logging::logger::set_verbose(is_verbose);
        }
        match name {
            "from png" => image_to_ansi(call, input),
            "to png" => ansi_to_image(call, input),
            _ => Ok(Value::string("test", call.head)),
        }
    }
}

fn main() {
    nu_plugin::serve_plugin(&mut Plugin {}, nu_plugin::MsgPackSerializer {})
}
