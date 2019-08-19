use std::fmt;

#[derive(Debug)]
pub(crate) enum Label<'a> {
    End,
    Data(&'a [u8]),
    Reference(u16),
}

#[derive(Debug)]
pub(crate) struct LabelSet<'a>(pub(crate) Vec<Label<'a>>);

impl<'a> fmt::Display for Label<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Label::End => Ok(()),
            Label::Data(bytes) => match std::str::from_utf8(bytes) {
                Ok(label) => write!(f, "{}", label),
                Err(e) => write!(f, "{}", e),
            },
            Label::Reference(offset) => write!(f, "@{}", *offset),
        }
    }
}

impl<'a> fmt::Display for LabelSet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, label) in self.0.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", label)?;
            } else {
                write!(f, ".{}", label)?;
            }
        }
        Ok(())
    }
}

impl<'a> From<&'a str> for LabelSet<'a> {
    fn from(s: &'a str) -> Self {
        let labels: Vec<_> = s
            .split('.')
            .map(|label| Label::Data(label.as_bytes()))
            .chain(std::iter::once(Label::End))
            .collect();
        LabelSet(labels)
    }
}
