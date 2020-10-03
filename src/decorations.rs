use std::ops::Range;

#[derive(Clone)]
enum TextRange<F> {
    Range(usize),
    Decoration(Decorations<F>),
}

impl<F> TextRange<F>
where
    F: Clone + PartialEq,
{
    fn is_range(&self) -> bool {
        use TextRange::*;
        match self {
            Range(_) => true,
            _ => false,
        }
    }

    fn is_decoration_face(&self, face: &F) -> bool {
        use TextRange::*;
        match self {
            Decoration(d) => d.face == *face,
            _ => false,
        }
    }

    fn len(&self) -> usize {
        use TextRange::*;
        match self {
            Range(len) => *len,
            Decoration(d) => d.len(),
        }
    }

    fn keep_start(&self, offset: usize) -> Option<TextRange<F>> {
        if offset == 0 {
            return None;
        }

        use TextRange::*;
        match &self {
            Decoration(d) => {
                let (idx, idx_offset) = d.fragment_index_of(offset)?;
                let mut new = d.sliced(0..idx);
                let relative_offset = offset - idx_offset;
                if let Some(tr) = d.fragments[idx].keep_start(relative_offset) {
                    new.fragments.push(tr);
                }
                Decoration(new)
            }
            Range(_len) => Range(offset),
        }
        .into()
    }

    fn keep_end(&self, offset: usize) -> Option<TextRange<F>> {
        if self.len() <= offset {
            return None;
        }

        use TextRange::*;
        match &self {
            Decoration(d) => {
                let (idx, idx_offset) = d.fragment_index_of(offset)?;
                let mut new = Decorations::new(d.face.clone());
                let relative_offset = offset - idx_offset;
                if let Some(tr) = d.fragments[idx].keep_end(relative_offset) {
                    new.fragments.push(tr);
                }
                let mut end = d.sliced(idx + 1..d.fragments.len());
                new.fragments.append(&mut end.fragments);
                Decoration(new)
            }
            Range(len) => Range(len - offset),
        }
        .into()
    }
}

#[derive(Clone)]
pub(crate) struct Decorations<F> {
    face: F,
    fragments: Vec<TextRange<F>>,
}

impl<F> Decorations<F>
where
    F: Clone + PartialEq,
{
    pub(crate) fn new(face: F) -> Decorations<F> {
        Decorations {
            face,
            fragments: Vec::new(),
        }
    }

    fn with_len(face: F, len: usize) -> Decorations<F> {
        let mut new = Decorations::new(face);
        new.fragments.push(TextRange::Range(len));
        new
    }

    fn sliced(&self, range: Range<usize>) -> Decorations<F> {
        Decorations {
            face: self.face.clone(),
            fragments: self.fragments[range].to_vec(),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.fragments.iter().fold(0, |acc, item| {
            use TextRange::*;
            match item {
                Range(len) => acc + len,
                Decoration(d) => acc + d.len(),
            }
        })
    }

    fn fragment_index_of(&self, offset: usize) -> Option<(usize, usize)> {
        let (mut idx, mut idx_offset) = (0, 0);
        let mut len = 0;
        loop {
            len += &self.fragments[idx].len();
            if len >= offset {
                break;
            }
            idx += 1;
            idx_offset = len;
        }
        Some((idx, idx_offset))
    }

    pub(crate) fn append(&mut self, face: F, len: usize) {
        if self.face == face {
            let last_is_range = self.fragments.last().map_or(false, TextRange::is_range);
            if last_is_range {
                let old_len = match self.fragments.pop() {
                    Some(TextRange::Range(len)) => len,
                    _ => unreachable!(),
                };
                self.fragments.push(TextRange::Range(old_len + len));
            } else {
                self.fragments.push(TextRange::Range(len));
            }
        } else {
            let last_is_face = self
                .fragments
                .last()
                .map_or(false, |tf| tf.is_decoration_face(&face));
            if last_is_face {
                match self.fragments.last_mut() {
                    Some(TextRange::Decoration(d)) => d.append(face.clone(), len),
                    _ => unreachable!(),
                }
            } else {
                self.fragments
                    .push(TextRange::Decoration(Decorations::with_len(face, len)));
            }
        }
    }

    pub(crate) fn set(&mut self, face: F, range: Range<usize>) {
        let (start, start_offset) = self.fragment_index_of(range.start).expect("invalid offset");
        let (end, end_offset) = self.fragment_index_of(range.end).expect("invalid offset");

        if start == end {
            if let TextRange::Decoration(d) = &mut self.fragments[start] {
                d.set(face, range);
                return;
            }
        }

        let mut new_fragments = Vec::new();
        if let Some(tf) = self.fragments[start].keep_start(range.start - start_offset) {
            new_fragments.push(tf);
        }
        new_fragments.push(TextRange::Decoration(Decorations::with_len(
            face,
            range.len(),
        )));
        if let Some(tf) = self.fragments[end].keep_end(range.end - end_offset) {
            new_fragments.push(tf);
        }
        self.fragments.splice(start..=end, new_fragments);
    }

    pub(crate) fn flatten(&self) -> Vec<(F, usize)> {
        let mut acc = Vec::new();
        for frag in &self.fragments {
            use TextRange::*;
            match frag {
                Range(len) => acc.push((self.face.clone(), *len)),
                Decoration(d) => acc.append(&mut d.flatten()),
            }
        }
        acc
    }
}
