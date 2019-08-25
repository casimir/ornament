//! A helper to create decorated text.
//!
//! # Examples
//!
//! This example creates a `Text` and renders it following a simple Markdown-like.
//!
//! ```
//! use ornament::Decorator;
//!
//! #[derive(Clone, Debug, PartialEq)]
//! enum Face {
//!     Default,
//!     Emphasis,
//!     Strong,
//! }
//!
//! impl Default for Face {
//!     fn default() -> Self {
//!         Face::Default
//!     }
//! }
//!
//! let text = Decorator::with_text("Text can be with emphasis or even strong.")
//!     .set(Face::Emphasis, 17..25)
//!     .set(Face::Strong, 34..40)
//!     .build();
//!
//! let rendered = text.render(|tf| {
//!     match tf.face {
//!         Face::Default => tf.text.to_owned(),
//!         Face::Emphasis => format!("_{}_", tf.text),
//!         Face::Strong => format!("**{}**", tf.text),
//!     }
//! });
//! assert_eq!(rendered, "Text can be with _emphasis_ or even **strong**.");
//!
//! // In some cases it can be easier to build it part by part.
//! let other_text = Decorator::new()
//!     .append("Text can be with ")
//!     .set_face(Face::Emphasis)
//!     .append("emphasis")
//!     .reset_face()
//!     .append(" or even ")
//!     .set_face(Face::Strong)
//!     .append("strong")
//!     .reset_face()
//!     .append(".")
//!     .build();
//! assert_eq!(other_text, text);
//!
//! // Both methods can be combined together.
//! let another_other_text = Decorator::new()
//!     .append("Text can be with ")
//!     .set_face(Face::Emphasis)
//!     .append("emphasis")
//!     .reset_face()
//!     .append(" or even strong.")
//!     .set(Face::Strong, 34..40)
//!     .build();
//! assert_eq!(another_other_text, text);
//! ```

mod decorations;
mod text;

use std::cmp::{max, min};
use std::ops::Range;

#[cfg(feature = "serde_support")]
#[macro_use]
extern crate serde;

use decorations::Decorations;
pub use text::{Text, TextFragment};

/// A helper type to build a [`Text`] instance.
///
/// It can be used in several manners.
/// - Incremental, using a "current face" (`*_face` and [`append`] methods).
/// - Immediate by setting face ranges directly ([`set`] method).
/// - A combination of both.
///
/// [`Text`]: struct.Text.html
/// [`append`]: struct.Decorator.html#method.append
/// [`set`]: struct.Decorator.html#method.set
pub struct Decorator<F: Default> {
    text: String,
    current_face: F,
    decorations: Decorations<F>,
}

impl<F> Decorator<F>
where
    F: Clone + Default + PartialEq,
{
    /// Creates a new empty `Decorator`.
    pub fn new() -> Decorator<F> {
        Decorator {
            text: String::new(),
            current_face: F::default(),
            decorations: Decorations::new(F::default()),
        }
    }

    /// Creates a new `Decorator` initialized with `text`.
    pub fn with_text(text: &str) -> Decorator<F> {
        let mut decorator = Decorator::new();
        decorator.append(text);
        decorator
    }

    /// Returns the current face. On init this value will be equivalent to `F::default()`.
    ///
    /// This face is mostly used by [`append`] which will assign the current face to the appended
    /// text.
    ///
    /// [`append`]: struct.Decorator.html#method.append
    pub fn current_face(&self) -> &F {
        &self.current_face
    }

    /// Sets the current face.
    ///
    /// This method is chainable.
    pub fn set_face(&mut self, face: F) -> &mut Self {
        self.current_face = face;
        self
    }

    /// Resets the current face. This is equivalent to calling [`set_face`] with `F::default()`.
    ///
    /// This method is chainable.
    ///
    /// [`set_face`]: struct.Decorator.html#method.set_face
    pub fn reset_face(&mut self) -> &mut Self {
        self.set_face(F::default());
        self
    }

    /// Appends `text` to the buffer and assigns it the current face.
    ///
    /// This method is chainable.
    pub fn append(&mut self, text: &str) -> &mut Self {
        self.text += text;
        self.decorations
            .append(self.current_face.clone(), text.len());
        self
    }

    /// Assigns `face` to the given range. It overrides all faces previously assigned to this range.
    ///
    /// This method is chainable.
    pub fn set(&mut self, face: F, range: Range<usize>) -> &mut Self {
        let safe_range = max(range.start, 0)..min(range.end, self.decorations.len());
        self.decorations.set(face, safe_range);
        self
    }

    /// Processes all face assignations and returns the corresponding `Text`.
    pub fn build(&mut self) -> Text<F> {
        let mut fragments = Vec::new();
        let mut acc = 0;
        for (face, len) in &self.decorations.flatten() {
            fragments.push(TextFragment {
                text: self.text[acc..acc + len].to_owned(),
                face: face.clone(),
            });
            acc += len;
        }
        fragments.into()
    }
}

impl<F> Default for Decorator<F>
where
    F: Clone + Default + PartialEq,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    enum Face {
        Default,
        Star,
        Pipe,
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
            Star => format!("*{}*", tf.text),
            Pipe => format!("|{}|", tf.text),
        }
    }

    #[test]
    fn build_text() {
        let text = Decorator::new()
            .append("This ")
            .set_face(Face::Star)
            .append("error")
            .reset_face()
            .append(" is important!")
            .build();
        assert_eq!(text.render(decorator), "This *error* is important!");
    }

    #[test]
    fn set_decorations() {
        let text = Decorator::with_text("This error is important!")
            .set(Face::Star, 5..10)
            .append(" ")
            .set_face(Face::Star)
            .append("Really")
            .reset_face()
            .append(".")
            .build();
        assert_eq!(
            text.render(decorator),
            "This *error* is important! *Really*."
        );
    }

    #[test]
    fn splitted_decorations() {
        let text = Decorator::new()
            .append("This ")
            .set_face(Face::Star)
            .append("weird ")
            .set_face(Face::Pipe)
            .append("tiny")
            .set_face(Face::Star)
            .append(" error")
            .reset_face()
            .append(" is important!")
            .build();
        assert_eq!(
            text.render(decorator),
            "This *weird *|tiny|* error* is important!"
        );
    }
}
