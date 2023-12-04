use nu_plugin::{self, EvaluatedCall, LabeledError};
use nu_plugin_image::{ansi_to_image, image_to_ansi, FontFamily, Palette};
use nu_protocol::{Category, PluginSignature, SyntaxShape, Type, Value};

pub struct Plugin;

impl nu_plugin::Plugin for Plugin {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("from png")
                // .switch(
                //     "verbose",
                //     "prints log of the work into the terminal",
                //     Some('v'),
                // )
                .switch(
                    "reverse-bg",
                    "reverse background and foreground colors",
                    Some('r'),
                )
                .switch("blink", "make blinking effect", Some('b'))
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
                    "char",
                    SyntaxShape::String,
                    "Character that will be used (like pixels)",
                    Some('c'),
                )
                .named(
                    "font-width",
                    SyntaxShape::Int,
                    "Font width, in pixels",
                    None,
                )
                .named(
                    "font-height",
                    SyntaxShape::Int,
                    "Font height, in pixels",
                    None,
                )
                .usage("create ansi text from an image")
                .input_output_type(Type::Binary, Type::String)
                .category(Category::Conversions),
            PluginSignature::build("to png")
                .named("width", SyntaxShape::Int, "output width", Some('w'))
                .named(
                    "output-path",
                    SyntaxShape::Filepath,
                    "output file path",
                    Some('o'),
                )
                .named("theme",SyntaxShape::String,format!("select theme of the output, one of: {:?}",Palette::list()),Some('t'))
                .named(
                    "font",
                    SyntaxShape::String,
                    format!(
                        "Select the font from one of {:?}, by default the first font in the list will be used",
                        FontFamily::list()
                    ),
                    None,
                )
                .named("font-regular", SyntaxShape::Filepath, "custom font Regular font path", None)
                .named("font-bold", SyntaxShape::Filepath, "custom font Bold font path", None)
                .named("font-italic", SyntaxShape::Filepath, "custom font Italic font path", None)
                .named("font-bold-italic", SyntaxShape::Filepath, "custom font Bold Italic font path", None)
                .usage("convert ansi output to image")
                .input_output_type(Type::String, Type::Nothing)
                .category(Category::Conversions),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
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
