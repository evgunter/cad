//! The two things an STL file carries that are not triangles, and the
//! types that carry them.
//!
//! **The pair, in one place.** An STL export has exactly two knobs
//! besides the mesh: the ASCII format's `solid <name>` name and the
//! binary format's 80-byte header. They live in this module together
//! so the pair is discoverable, and in **separate option structs**
//! because each writer reads exactly one of them —
//! [`write_ascii`](crate::write_ascii) takes [`AsciiOptions`],
//! [`write_binary`](crate::write_binary) takes [`BinaryOptions`], and
//! neither can be handed a field its format does not read.
//!
//! They name **different kinds of thing**, which is why they are two
//! types rather than two spellings of one: the solid name is the
//! *part* the file describes; the 80 bytes are free text the format
//! leaves to the writer, conventionally identifying the *producer*.
//! That is also why neither converts into the other.
//!
//! **Validation happens here, at construction.** [`SolidName`] and
//! [`BinaryHeader`] are validated newtypes: a value that exists is a
//! value the writer can emit, so a caller who builds options once and
//! exports many meshes learns about a bad name once, immediately,
//! rather than once per export after tessellation. The writers'
//! [`StlError`](crate::StlError) is therefore about the *mesh and the
//! sink* only.
//!
//! Nothing here derives `Eq`. `Eq` is not a feature — it is a marker
//! asserting that the derived `PartialEq` is reflexive — and no
//! consumer in the workspace uses one of these types as a set element,
//! a map key, or anywhere total equality is required, so asserting it
//! would be unearned.

use crate::ascii::DEFAULT_SOLID_NAME;
use crate::binary::DEFAULT_HEADER;

/// Why a [`SolidName`] was refused.
///
/// Its own type rather than an arm of [`StlError`](crate::StlError):
/// the writers cannot produce this failure, and an error variant no
/// door can return is a claim nothing can falsify.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SolidNameError {
    /// The name carries a character outside the printable-ASCII range
    /// the `solid <name>` grammar admits — a newline would make
    /// `endsolid <name>` unmatchable and the file unparseable, so the
    /// name is refused rather than sanitized.
    Unrepresentable {
        /// The offending character.
        character: char,
    },
}

/// Why a [`BinaryHeader`] was refused.
///
/// Its own type, for the same reason as [`SolidNameError`], and a
/// *different* type because the two fields fail in different ways: a
/// name cannot be too long and a header cannot be unrepresentable.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryHeaderError {
    /// The header does not fit binary STL's 80-byte header field.
    /// Refused rather than truncated.
    TooLong {
        /// The requested header's length in bytes.
        len: usize,
    },
    /// The header reads as the ASCII-STL `solid` keyword, which makes
    /// the file sniff as ASCII STL in the parsers that decide by it.
    /// The recognised class is wider than a byte-exact prefix —
    /// leading whitespace is skipped and case is folded, because
    /// sniffers in the wild do both — so `" Solid v2"` is refused as
    /// well as `"solid v2"`.
    SniffsAscii,
}

impl core::fmt::Display for SolidNameError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unrepresentable { character } => write!(
                f,
                "stl export: solid name contains {character:?}, outside the printable \
                 ASCII the single-line `solid <name>` grammar admits"
            ),
        }
    }
}

impl core::fmt::Display for BinaryHeaderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooLong { len } => write!(
                f,
                "stl export: {len}-byte header exceeds binary STL's 80-byte header field"
            ),
            Self::SniffsAscii => write!(
                f,
                "stl export: a binary header reading as the `solid` keyword (leading \
                 whitespace skipped, case folded) sniffs as ASCII STL"
            ),
        }
    }
}

impl std::error::Error for SolidNameError {}
impl std::error::Error for BinaryHeaderError {}

/// The ASCII writer's `solid <name>` name, closed by the matching
/// `endsolid <name>`.
///
/// **This names the solid the file describes**, not the software that
/// wrote it — the producer belongs in [`BinaryHeader`], which is the
/// field the format leaves free.
///
/// Printable ASCII (`0x20..=0x7E`) only, checked by [`Self::new`]: the
/// grammar is one line, so anything else is a refusal rather than a
/// silently mangled file. The empty string is accepted and writes
/// `solid` followed by a trailing space; the format admits it and
/// nothing downstream objects.
///
/// [`Default`] is a generic part name, which carries no project or
/// producer identity — so the exported bytes do not depend on Q9.
#[derive(Clone, Debug, PartialEq)]
pub struct SolidName(String);

/// The binary writer's header text, zero-padded to the format's
/// 80-byte header field.
///
/// Free text by the format; conventionally the producer, which is why
/// [`Default`] names this writer rather than the part.
///
/// At most 80 bytes (measured in **bytes**, not characters) and never
/// reading as the ASCII-STL `solid` keyword, both checked by
/// [`Self::new`] — see [`BinaryHeaderError::SniffsAscii`] for how wide
/// that check is and why. The empty string is accepted and writes 80
/// zero bytes.
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryHeader(String);

impl SolidName {
    /// Validates `name` and takes it, or refuses.
    ///
    /// # Errors
    ///
    /// [`SolidNameError::Unrepresentable`] for any character outside
    /// `0x20..=0x7E`.
    pub fn new(name: impl Into<String>) -> Result<Self, SolidNameError> {
        let name = name.into();
        // The bound is BOTH-SIDED on purpose: below 0x20 are the
        // control characters that break the one-line grammar, and at or
        // above 0x7F are DEL and everything non-ASCII, which no STL
        // reader agrees on the encoding of.
        match name.chars().find(|c| !(' '..='~').contains(c)) {
            Some(character) => Err(SolidNameError::Unrepresentable { character }),
            None => Ok(Self(name)),
        }
    }

    /// The validated name, as the writer emits it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl BinaryHeader {
    /// Validates `header` and takes it, or refuses.
    ///
    /// # Errors
    ///
    /// [`BinaryHeaderError::TooLong`] over 80 bytes;
    /// [`BinaryHeaderError::SniffsAscii`] for anything reading as the
    /// `solid` keyword.
    pub fn new(header: impl Into<String>) -> Result<Self, BinaryHeaderError> {
        let header = header.into();
        if header.len() > 80 {
            return Err(BinaryHeaderError::TooLong { len: header.len() });
        }
        if sniffs_as_ascii(&header) {
            return Err(BinaryHeaderError::SniffsAscii);
        }
        Ok(Self(header))
    }

    /// The validated header text, before zero-padding.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SolidName {
    fn default() -> Self {
        Self(DEFAULT_SOLID_NAME.to_owned())
    }
}

impl Default for BinaryHeader {
    fn default() -> Self {
        Self(DEFAULT_HEADER.to_owned())
    }
}

/// Options for [`write_ascii`](crate::write_ascii): the solid name, and
/// nothing else, because the ASCII format carries nothing else that is
/// not a triangle.
///
/// A plain struct with public fields, like `step_export::StepOptions` —
/// the validation lives in [`SolidName`], so the struct itself has no
/// invariant to protect and no constructor to hide behind. Its sibling
/// is [`BinaryOptions`].
///
/// Determinism (D9): the writer is a pure function of
/// `(mesh, options)` — equal inputs give byte-identical files. It
/// reads no clock, no environment variable and no global state, and
/// the default carries none.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AsciiOptions {
    /// The `solid <name>` name this export writes.
    pub solid_name: SolidName,
}

/// Options for [`write_binary`](crate::write_binary): the 80-byte
/// header, and nothing else.
///
/// A plain struct with public fields, like `step_export::StepOptions`.
/// Its sibling is [`AsciiOptions`]; the two are separate because the
/// binary writer never reads a solid name and the ASCII writer never
/// reads a header, so **neither can be given a field it ignores**.
///
/// Determinism (D9): as [`AsciiOptions`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BinaryOptions {
    /// The 80-byte header field's text, zero-padded at write time.
    pub header: BinaryHeader,
}

/// Whether `header` would make a binary file read as ASCII STL in a
/// reader that decides by the `solid` keyword.
///
/// **Wider than a byte-exact prefix, deliberately.** Sniffers in the
/// wild skip leading whitespace and fold case before comparing, so a
/// header of `" Solid v2"` is misread by some readers and not others —
/// which is worse than either. The check covers the whole class the
/// keyword can be recognised through, and the cost is that a header
/// which merely *starts* with those five letters (`SolidWorks export`)
/// is refused too. A refusal the caller can see beats a file another
/// tool silently reads as text.
fn sniffs_as_ascii(header: &str) -> bool {
    let head = header.trim_start().as_bytes();
    head.len() >= 5 && head[..5].eq_ignore_ascii_case(b"solid")
}
