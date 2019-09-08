/// A piece of a decorated text.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct TextFragment<F> {
    /// The raw text.
    pub text: String,
    /// The associated face.
    pub face: F,
}

impl<F: Default> From<&str> for TextFragment<F> {
    fn from(s: &str) -> TextFragment<F> {
        TextFragment {
            text: s.to_owned(),
            face: F::default(),
        }
    }
}

impl<F: Default> From<String> for TextFragment<F> {
    fn from(s: String) -> TextFragment<F> {
        TextFragment {
            text: s.to_owned(),
            face: F::default(),
        }
    }
}

/// A decorated text. This is a collection of [`TextFragment`].
///
/// [`TextFragment`]: struct.TextFragment.html
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Text<F>(Vec<TextFragment<F>>);

impl<F> Text<F> {
    /// Returns the length of the underlying text, without decorations, in bytes.
    pub fn text_len(&self) -> usize {
        self.iter().fold(0, |acc, x| acc + x.text.len())
    }

    /// Converts the decorated text into rich text, using `decorator` to handle the different faces.
    pub fn render<G>(&self, decorator: G) -> String
    where
        G: Fn(&TextFragment<F>) -> String,
    {
        self.iter().map(decorator).collect::<Vec<String>>().join("")
    }

    /// Converts the decorated text into plain text, stripping all decorations.
    pub fn plain(&self) -> String {
        self.iter().fold(String::new(), |acc, x| acc + &x.text)
    }
}

pub type TextIterator<'a, F> = std::slice::Iter<'a, TextFragment<F>>;

impl<F> Text<F> {
    /// Returns an iterator over the text.
    pub fn iter(&self) -> TextIterator<'_, F> {
        self.0.iter()
    }
}

impl<'a, F> IntoIterator for &'a Text<F> {
    type Item = &'a TextFragment<F>;
    type IntoIter = TextIterator<'a, F>;

    fn into_iter(self) -> TextIterator<'a, F> {
        self.iter()
    }
}

impl<F> From<Vec<TextFragment<F>>> for Text<F> {
    fn from(tfs: Vec<TextFragment<F>>) -> Text<F> {
        Text(tfs)
    }
}

impl<F> From<TextFragment<F>> for Text<F> {
    fn from(tf: TextFragment<F>) -> Text<F> {
        Text(vec![tf])
    }
}

impl<F: Default> From<&str> for Text<F> {
    fn from(s: &str) -> Text<F> {
        Text(vec![TextFragment {
            text: s.to_owned(),
            face: F::default(),
        }])
    }
}

impl<F: Default> From<String> for Text<F> {
    fn from(s: String) -> Text<F> {
        Text(vec![TextFragment {
            text: s.to_owned(),
            face: F::default(),
        }])
    }
}
