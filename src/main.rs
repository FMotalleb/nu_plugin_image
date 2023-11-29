use nu_plugin::{self, EvaluatedCall, LabeledError};
use nu_plugin_image::{ansi_to_image, image_to_ansi};
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
                .named("path", SyntaxShape::Filepath, "output file path", Some('p'))
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
