//! **A label** (DESIGN.md Band 1, "Node labels"): the human text a
//! person gives a recipe node, or a face or body through
//! [`crate::Attr::Label`]. Document data, never identity: labels repeat,
//! nothing resolves one, and a node's label is held beside the node
//! ([`crate::Doc::label`]) rather than inside it, so it is in neither
//! the id's mint nor any content key.
//!
//! [`Label`] is the one validated spelling. Every door that takes a
//! label's text — the edit, the appearance attribute, the load door
//! reading either out of a file — goes through [`Label::new`], so a
//! held `Label` is non-blank, one line, and free of control
//! characters.

use core::fmt;

/// **A label's text**, validated ([`Label::new`]): it has a character
/// that is not whitespace, it is one line, and it holds no control
/// character.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label(String);

/// Why a text is not a [`Label`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelFault {
    /// Empty, or only whitespace: nothing a person could read.
    Blank,
    /// A line break (`\n`, `\r`, a vertical tab or form feed, U+0085,
    /// U+2028, U+2029) at this char index: a label is one line,
    /// because every surface that shows one shows it on a row.
    LineBreak {
        /// The char index of the first break.
        at: usize,
    },
    /// A control character that is not a line break (a tab, `NUL`,
    /// an escape) at this char index.
    Control {
        /// The char index of the first one.
        at: usize,
        /// The character.
        ch: char,
    },
}

/// The characters that end a line, in Unicode's reading and in a
/// terminal's.
fn breaks_a_line(ch: char) -> bool {
    matches!(
        ch,
        '\n' | '\r' | '\u{0b}' | '\u{0c}' | '\u{85}' | '\u{2028}' | '\u{2029}'
    )
}

impl Label {
    /// The text as a label, or why it cannot be one. The text is
    /// stored as given: surrounding whitespace is the author's.
    ///
    /// # Errors
    ///
    /// [`LabelFault`]: the first line break, then the first other
    /// control character, then blankness.
    pub fn new(text: impl Into<String>) -> Result<Self, LabelFault> {
        let text = text.into();
        if let Some(at) = text.chars().position(breaks_a_line) {
            return Err(LabelFault::LineBreak { at });
        }
        if let Some((at, ch)) = text.chars().enumerate().find(|(_, ch)| ch.is_control()) {
            return Err(LabelFault::Control { at, ch });
        }
        if text.chars().all(char::is_whitespace) {
            return Err(LabelFault::Blank);
        }
        Ok(Self(text))
    }

    /// The text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Display for LabelFault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blank => f.write_str("a label must have a character that is not whitespace"),
            Self::LineBreak { at } => write!(
                f,
                "a label is one line, and this one breaks at character {at}"
            ),
            Self::Control { at, ch } => write!(
                f,
                "a label holds no control character, and this one has {} at character {at}",
                ch.escape_unicode()
            ),
        }
    }
}

impl core::error::Error for LabelFault {}

impl TryFrom<String> for Label {
    type Error = LabelFault;
    fn try_from(text: String) -> Result<Self, LabelFault> {
        Self::new(text)
    }
}

impl From<Label> for String {
    fn from(label: Label) -> Self {
        label.0
    }
}

impl serde::Serialize for Label {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(&self.0)
    }
}

/// The load door's half of the rule: a file's label text passes
/// [`Label::new`] or the parse refuses, naming the fault.
impl<'de> serde::Deserialize<'de> for Label {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let text = String::deserialize(de)?;
        Self::new(text).map_err(|fault| serde::de::Error::custom(format!("label refused: {fault}")))
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::{Label, LabelFault};

    /// Each rule, at a text that breaks only it, and the text kept
    /// verbatim when none breaks.
    #[test]
    fn a_label_is_non_blank_one_line_and_free_of_control_characters() {
        let cases: [(&str, Result<&str, LabelFault>); 9] = [
            ("base plate", Ok("base plate")),
            ("  padded  ", Ok("  padded  ")),
            ("Bolt \"M6\" ⌀6", Ok("Bolt \"M6\" ⌀6")),
            ("", Err(LabelFault::Blank)),
            (" \u{3000} ", Err(LabelFault::Blank)),
            ("two\nlines", Err(LabelFault::LineBreak { at: 3 })),
            ("ab\u{2028}", Err(LabelFault::LineBreak { at: 2 })),
            ("tab\there", Err(LabelFault::Control { at: 3, ch: '\t' })),
            (
                "\u{1b}[31m",
                Err(LabelFault::Control {
                    at: 0,
                    ch: '\u{1b}',
                }),
            ),
        ];
        for (text, want) in cases {
            let got = Label::new(text);
            assert_eq!(
                got.as_ref().map(Label::as_str).map_err(|f| *f),
                want,
                "Label::new({text:?})"
            );
        }
    }

    /// A file's label is read through the same rule: the valid text
    /// round-trips, and a blank one refuses at the parse.
    #[test]
    fn the_load_door_reads_a_label_through_the_same_rule() {
        let label = Label::new("lid").expect("a valid label");
        let json = serde_json::to_string(&label).expect("serializes");
        assert_eq!(json, "\"lid\"");
        let back: Label = serde_json::from_str(&json).expect("reads back");
        assert_eq!(back, label);
        let refused = serde_json::from_str::<Label>("\"  \"").expect_err("a blank label refuses");
        assert!(
            refused.to_string().contains("not whitespace"),
            "the refusal names the fault: {refused}"
        );
    }
}
