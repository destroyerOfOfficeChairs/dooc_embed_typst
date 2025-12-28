use dooc_embed_typst::compile;
use std::collections::HashMap;
use typst::foundations::Value;

// BAKING ASSETS
// These run at compile time. The files MUST exist or 'cargo build' will fail.
// This makes it so that you do not have to distribute an assets folder, instead
// everything is included in your binary after compiling.
// NOTE: Paths are relative to the SOURCE file.
const FONT_ROBOTO: &[u8] = include_bytes!("assets/fonts/Roboto-Regular.ttf");
const FONT_MATH: &[u8] = include_bytes!("assets/fonts/NewCMMath-Regular.otf");
const CUTE_IMAGE_BY_MY_DAUGHTER: &[u8] = include_bytes!("assets/images/cute.png"); // Example
const TYPST_TEMPLATE: &str = include_str!("assets/templates/demo.typ");

fn main() {
    // Currently, dooc_embed_typst must have a single string which contains
    // everything you want to compile into a PDF.
    //
    // This means you do not have the ability to #include or #import other
    // typst files. You must instead concatenate them all manually.
    //
    // I recommend using askama.
    let my_code = TYPST_TEMPLATE.to_string();

    // This is where your sys.inputs go.
    // Normally when you compile a typst document, you do something like this:
    // typst compile foo.typ --input bar=42
    //
    // and in your typst file you'd do:
    // #let baz = sys.inputs.bar
    //
    // typst::foundations::Value has several types you can use.
    // Here, we are just using Value::Int. There are 30 variants.
    //
    // Within our typst file, we can use it as an actual int rather than a
    // string (as it normally would be), so we could do mathematical operations
    // on it directly if we wanted.
    //
    // This can safely be left empty. You will just get a compiler warning.
    let mut inputs = HashMap::new();
    inputs.insert("year_drawn".to_string(), Value::Int(2025));

    // You can put images or other files here.
    //
    // Currently, dooc_embed_typst has no means to support packages or plugins.
    //
    // This can safely be left empty. You will just get a compiler warning.
    let mut files = HashMap::new();
    files.insert("cute.png".to_string(), CUTE_IMAGE_BY_MY_DAUGHTER.to_vec());

    // Fonts are baked into the binary at compile time.
    // It doesn't matter what system your users are on, they will have access
    // to the fonts you include.
    let fonts = vec![FONT_ROBOTO.to_vec(), FONT_MATH.to_vec()];

    // To compile your PDF, you need these four arguments:
    // code: String
    // inputs: HashMap<String, Value>
    // files: HashMap<String, Vec<u8>>
    // fonts: Vec<Vec<u8>>
    //
    // I could've made dooc_embed_typst save the file itself,
    // but instead I decided to let the caller decide what to do with the
    // PDF bytes. You could write it to a file, or send it directly to a
    // printer, or beam it to the moon! We're just writing it to a file here.
    // This allows the caller to decide what the path/name of the output file
    // will be. I went with "output.pdf" because I wrote the .gitignore to
    // exclude that file.
    match compile(my_code, inputs, files, fonts) {
        Ok(pdf_bytes) => {
            // You decide the filename here!
            std::fs::write("output.pdf", pdf_bytes).expect("Failed to write file");
            println!("Success! Created output.pdf");
        }
        Err(e) => {
            eprintln!("Compilation failed:\n{}", e);
            std::process::exit(1);
        }
    }
}
