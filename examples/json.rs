#[macro_use]
extern crate serde;

use ornament::{Decorator, Text};
use serde_json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Face {
    Default,
    Strong,
}

impl Default for Face {
    fn default() -> Self {
        Face::Default
    }
}

fn main() {
    let text = Decorator::with_text("This part is important.")
        .set(Face::Strong, 5..9)
        .build();

    let jsonified = serde_json::to_string(&text).unwrap();
    println!("{}", jsonified);

    let parsed_text: Text<Face> = serde_json::from_str(&jsonified).unwrap();
    println!("{:?}", parsed_text);
}
