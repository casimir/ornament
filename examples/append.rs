use ornament::{Decorator, TextFragment};

#[derive(Clone, Debug, PartialEq)]
enum Face {
    Default,
    Error,
}

impl Default for Face {
    fn default() -> Self {
        Face::Default
    }
}

fn decorator(tf: &TextFragment<Face>) -> String {
    use Face::*;
    match tf.face {
        Default => tf.text.to_owned(),
        Error => format!("*{}*", tf.text),
    }
}

fn main() {
    let text = Decorator::with_text("This ")
        .set_face(Face::Error)
        .append("error")
        .reset_face()
        .append(" is important!")
        .build();
    println!("{}", text.render(decorator));
    // output: This *error* is important!
}
