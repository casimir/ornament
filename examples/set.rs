use ornament::{Decorator, TextFragment};

#[derive(Clone, Debug, PartialEq)]
enum Face {
    Default,
    Strong,
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
        Strong => format!("**{}**", tf.text),
    }
}

fn main() {
    let text = Decorator::with_text("This part is important.")
        .set(Face::Strong, 5..9)
        .build();
    println!("{}", text.render(decorator));
    // output: This **part** is important!
}
