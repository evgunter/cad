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
//! held `Label` holds no line break, no control character and none of
//! the nine bidi embedding, override and isolate characters, and has a
//! character that is neither whitespace nor default-ignorable.

use core::fmt;

/// **A label's text**, validated ([`Label::new`]): no line break, no
/// control character, no bidi embedding, override or isolate, and a
/// character that is neither whitespace nor default-ignorable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label(String);

/// Why a text is not a [`Label`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelFault {
    /// Empty, or only whitespace and default-ignorable characters
    /// (zero-width spaces and joiners, variation selectors, Hangul
    /// fillers, tag characters): nothing a person could see. Any
    /// other character counts as one that shows, U+2800 BRAILLE
    /// PATTERN BLANK and a lone combining mark included.
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
    /// A bidi embedding, override or isolate (U+202A–U+202E,
    /// U+2066–U+2069) at this char index. Each opens a direction
    /// scope that runs until its terminator, so one in a label can
    /// reorder whatever a surface prints after it. Only these nine
    /// refuse: the bidi marks (U+200E, U+200F, U+061C, a trailing one
    /// included), U+206A–U+206F and the annotation characters
    /// U+FFF9–U+FFFB beside visible text, and a leading combining mark
    /// all pass.
    Direction {
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

/// The bidi embeddings and overrides (LRE, RLE, PDF, LRO, RLO) and
/// isolates (LRI, RLI, FSI, PDI).
fn sets_direction(ch: char) -> bool {
    matches!(ch, '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}

/// Unicode's `Default_Ignorable_Code_Point`: the characters a renderer
/// draws nothing for. The ranges are `DerivedCoreProperties.txt`'s,
/// Unicode 18.0.0, adjacent lines merged; nothing in the tree derives
/// them independently, because Unicode pre-assigns the property to the
/// reserved code points of the blocks it sets aside for format
/// characters, so a new version seldom moves the set (the 16.0.0 and
/// 18.0.0 files list the same one). On a Unicode upgrade, re-derive
/// the ranges and the test's count from that version's file.
fn is_default_ignorable(ch: char) -> bool {
    matches!(
        ch,
        '\u{00ad}'
            | '\u{034f}'
            | '\u{061c}'
            | '\u{115f}'..='\u{1160}'
            | '\u{17b4}'..='\u{17b5}'
            | '\u{180b}'..='\u{180f}'
            | '\u{200b}'..='\u{200f}'
            | '\u{202a}'..='\u{202e}'
            | '\u{2060}'..='\u{206f}'
            | '\u{3164}'
            | '\u{fe00}'..='\u{fe0f}'
            | '\u{feff}'
            | '\u{ffa0}'
            | '\u{fff0}'..='\u{fff8}'
            | '\u{1bca0}'..='\u{1bca3}'
            | '\u{1d173}'..='\u{1d17a}'
            | '\u{e0000}'..='\u{e0fff}'
    )
}

/// A character a person can see: not whitespace, and not one a
/// renderer draws nothing for.
fn shows(ch: char) -> bool {
    !ch.is_whitespace() && !is_default_ignorable(ch)
}

impl Label {
    /// The text as a label, or why it cannot be one. The text is
    /// stored as given: surrounding whitespace is the author's.
    ///
    /// # Errors
    ///
    /// [`LabelFault`]: the first line break, then the first other
    /// control character, then the first direction-setting character,
    /// then blankness.
    pub fn new(text: impl Into<String>) -> Result<Self, LabelFault> {
        let text = text.into();
        if let Some(at) = text.chars().position(breaks_a_line) {
            return Err(LabelFault::LineBreak { at });
        }
        if let Some((at, ch)) = text.chars().enumerate().find(|(_, ch)| ch.is_control()) {
            return Err(LabelFault::Control { at, ch });
        }
        if let Some((at, ch)) = text.chars().enumerate().find(|&(_, ch)| sets_direction(ch)) {
            return Err(LabelFault::Direction { at, ch });
        }
        if Self::is_blank(&text) {
            return Err(LabelFault::Blank);
        }
        Ok(Self(text))
    }

    /// Whether `text` has no character a person can see — every
    /// character whitespace or default-ignorable: the rule's
    /// [`LabelFault::Blank`], for a surface that reads such a text as
    /// "no label" rather than refusing it.
    #[must_use]
    pub fn is_blank(text: &str) -> bool {
        !text.chars().any(shows)
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
            Self::Blank => f.write_str("a label must have a character a person can see"),
            Self::LineBreak { at } => write!(
                f,
                "a label is one line, and this one breaks at character {at}"
            ),
            Self::Control { at, ch } => write!(
                f,
                "a label holds no control character, and this one has {} at character {at}",
                ch.escape_unicode()
            ),
            Self::Direction { at, ch } => write!(
                f,
                "a label holds no bidi embedding, override or isolate, and this one has {} at character {at}",
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
    use super::{Label, LabelFault, is_default_ignorable};

    /// Each rule, at a text that breaks only it, and the text kept
    /// verbatim when none breaks.
    #[test]
    fn a_label_is_visible_one_line_and_free_of_controls_and_direction_formatting() {
        let cases: [(&str, Result<&str, LabelFault>); 24] = [
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
            // Joiners, marks, selectors and tags that shape visible
            // text are the author's: an emoji family, a Persian word,
            // a subdivision flag, a mixed-direction label.
            ("👨\u{200d}👩\u{200d}👧", Ok("👨\u{200d}👩\u{200d}👧")),
            ("می\u{200c}خواهم", Ok("می\u{200c}خواهم")),
            (
                "🏴\u{e0067}\u{e0062}\u{e0073}\u{e0063}\u{e0074}\u{e007f}",
                Ok("🏴\u{e0067}\u{e0062}\u{e0073}\u{e0063}\u{e0074}\u{e007f}"),
            ),
            ("שלום\u{200f} M6", Ok("שלום\u{200f} M6")),
            ("x\u{00ad}y", Ok("x\u{00ad}y")),
            ("قطعة\u{061c} M6", Ok("قطعة\u{061c} M6")),
            // Nothing in these shows.
            ("\u{200b}\u{200b}", Err(LabelFault::Blank)),
            ("\u{3164}", Err(LabelFault::Blank)),
            ("\u{fe0f}", Err(LabelFault::Blank)),
            (" \u{2060}\u{feff} ", Err(LabelFault::Blank)),
            // A direction scope anywhere refuses, at its index; one
            // alone is a direction fault before it is a blank one.
            (
                "\u{202e}lid",
                Err(LabelFault::Direction {
                    at: 0,
                    ch: '\u{202e}',
                }),
            ),
            (
                "lid\u{202e}",
                Err(LabelFault::Direction {
                    at: 3,
                    ch: '\u{202e}',
                }),
            ),
            (
                "ab\u{2066}cd\u{2069}",
                Err(LabelFault::Direction {
                    at: 2,
                    ch: '\u{2066}',
                }),
            ),
            // A control is reported before a direction scope.
            ("\u{202e}\t", Err(LabelFault::Control { at: 1, ch: '\t' })),
            (
                "\u{202e}",
                Err(LabelFault::Direction {
                    at: 0,
                    ch: '\u{202e}',
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
            refused.to_string().contains("a person can see"),
            "the refusal names the fault: {refused}"
        );
    }

    /// `Default_Ignorable_Code_Point` from `DerivedCoreProperties.txt`
    /// (Unicode 18.0.0), adjacent lines merged: each range holds its
    /// ends and not the characters just outside them, and nothing
    /// outside the ranges is ignorable.
    #[test]
    fn default_ignorable_is_unicodes_ranges_and_nothing_else() {
        const RANGES: [(u32, u32); 17] = [
            (0x00ad, 0x00ad),
            (0x034f, 0x034f),
            (0x061c, 0x061c),
            (0x115f, 0x1160),
            (0x17b4, 0x17b5),
            (0x180b, 0x180f),
            (0x200b, 0x200f),
            (0x202a, 0x202e),
            (0x2060, 0x206f),
            (0x3164, 0x3164),
            (0xfe00, 0xfe0f),
            (0xfeff, 0xfeff),
            (0xffa0, 0xffa0),
            (0xfff0, 0xfff8),
            (0x1_bca0, 0x1_bca3),
            (0x1_d173, 0x1_d17a),
            (0xe_0000, 0xe_0fff),
        ];
        let at = |cp: u32| is_default_ignorable(char::from_u32(cp).expect("a scalar value"));
        for (lo, hi) in RANGES {
            assert!(at(lo) && at(hi), "U+{lo:04X}..U+{hi:04X} holds its ends");
            assert!(
                !at(lo - 1) && !at(hi + 1),
                "U+{lo:04X}..U+{hi:04X} stops at its ends"
            );
        }
        let total: u32 = RANGES.iter().map(|(lo, hi)| hi - lo + 1).sum();
        let found = (char::MIN..=char::MAX)
            .filter(|&ch| is_default_ignorable(ch))
            .count();
        assert_eq!(total, 4174, "the file's count of ignorable code points");
        assert_eq!(found, 4174, "no ignorable outside the ranges");
    }

    /// Of every `char`, exactly the nine embeddings, overrides and
    /// isolates refuse as a direction scope.
    #[test]
    fn exactly_the_nine_direction_scopes_refuse_as_direction() {
        let refused: Vec<char> = (char::MIN..=char::MAX)
            .filter(|&ch| {
                matches!(
                    Label::new(format!("a{ch}")),
                    Err(LabelFault::Direction { .. })
                )
            })
            .collect();
        assert_eq!(
            refused,
            [
                '\u{202a}', '\u{202b}', '\u{202c}', '\u{202d}', '\u{202e}', '\u{2066}', '\u{2067}',
                '\u{2068}', '\u{2069}'
            ],
            "the direction-setting characters"
        );
    }
}
