//! The Part 21 substrate (Leg A): tokenizer and exchange-structure
//! parser for the subset the writer emits — HEADER and DATA sections,
//! simple entity instances, and complex (multi-record) instances (the
//! writer's rational B-spline complexes, `SI_UNIT` records, and the
//! `GEOMETRIC_REPRESENTATION_CONTEXT`).
//!
//! Values stay close to the wire: numbers keep their raw token (reals
//! parse downstream with `str::parse::<f64>`, which round-trips the
//! writer's printer to identical bits), strings are unescaped, and
//! typed parameters (`LENGTH_MEASURE(1e-9)`) are their own variant.
//! Errors are typed and carry the 1-based line number.

use std::collections::BTreeMap;

use crate::error::StepImportError;

/// One parsed Part 21 parameter value.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Value {
    /// `$` — the unset parameter.
    Null,
    /// `*` — the derived/redeclared parameter.
    Star,
    /// `#id` — an entity instance reference.
    Ref(u64),
    /// A numeric token, kept raw (parsed on demand — integer slots
    /// and real slots read the same token differently).
    Number(String),
    /// A string literal, unescaped (`''` → `'`, `\\` → `\`).
    Str(String),
    /// An enumeration value: `.METRE.` → `METRE`.
    Enum(String),
    /// A parenthesized list.
    List(Vec<Value>),
    /// A typed parameter: `LENGTH_MEASURE(1e-9)`.
    Typed(String, Vec<Value>),
}

/// One record of an instance: keyword + argument list. A simple
/// instance has one; a complex instance has one per component, in
/// file order.
pub(crate) type Record = (String, Vec<Value>);

/// One `#id = …;` entity instance.
#[derive(Clone, Debug)]
pub(crate) struct Instance {
    /// The records: `len() == 1` for a simple instance, one per
    /// component for a complex `( A(..) B(..) … )` instance.
    pub(crate) records: Vec<Record>,
}

/// A parsed exchange file: header records (in file order) and the
/// data section keyed by instance id (deterministic B-tree order).
#[derive(Debug)]
pub(crate) struct StepFile {
    /// HEADER section records, in file order.
    pub(crate) header: Vec<Record>,
    /// DATA section instances by id.
    pub(crate) data: BTreeMap<u64, Instance>,
}

/// Character cursor with 1-based line tracking.
struct Cursor<'a> {
    text: &'a [u8],
    pos: usize,
    line: usize,
}

impl<'a> Cursor<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            text: text.as_bytes(),
            pos: 0,
            line: 1,
        }
    }

    fn syntax(&self, expected: &'static str) -> StepImportError {
        StepImportError::Syntax {
            line: self.line,
            expected,
        }
    }

    /// Skips whitespace and `/* … */` comments.
    fn skip_ws(&mut self) {
        loop {
            match self.text.get(self.pos) {
                Some(b'\n') => {
                    self.line += 1;
                    self.pos += 1;
                }
                Some(c) if c.is_ascii_whitespace() => self.pos += 1,
                Some(b'/') if self.text.get(self.pos + 1) == Some(&b'*') => {
                    self.pos += 2;
                    while self.pos < self.text.len() {
                        if self.text[self.pos] == b'\n' {
                            self.line += 1;
                        }
                        if self.text[self.pos] == b'*' && self.text.get(self.pos + 1) == Some(&b'/')
                        {
                            self.pos += 2;
                            break;
                        }
                        self.pos += 1;
                    }
                }
                _ => break,
            }
        }
    }

    fn peek(&self) -> Option<u8> {
        self.text.get(self.pos).copied()
    }

    /// Consumes `token` (exact bytes) after whitespace, or errors.
    fn expect(&mut self, token: &str, expected: &'static str) -> Result<(), StepImportError> {
        self.skip_ws();
        if self.text[self.pos..].starts_with(token.as_bytes()) {
            self.pos += token.len();
            Ok(())
        } else {
            Err(self.syntax(expected))
        }
    }

    /// A standard keyword: `[A-Z_][A-Z0-9_-]*` (the writer emits only
    /// standard keywords; user-defined `!` keywords are out of subset
    /// and fail as syntax).
    fn keyword(&mut self) -> Result<String, StepImportError> {
        self.skip_ws();
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_ascii_uppercase()
                || c == b'_'
                || c == b'-'
                || (c.is_ascii_digit() && self.pos > start)
            {
                self.pos += 1;
            } else {
                break;
            }
        }
        if self.pos == start {
            return Err(self.syntax("an entity keyword"));
        }
        // The byte range is ASCII by construction.
        Ok(String::from_utf8_lossy(&self.text[start..self.pos]).into_owned())
    }

    /// An unsigned decimal integer (entity ids).
    fn integer(&mut self, expected: &'static str) -> Result<u64, StepImportError> {
        self.skip_ws();
        let start = self.pos;
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.pos += 1;
        }
        if self.pos == start {
            return Err(self.syntax(expected));
        }
        String::from_utf8_lossy(&self.text[start..self.pos])
            .parse()
            .map_err(|_| self.syntax(expected))
    }

    /// A string literal (after the opening `'`): `''` unescapes to
    /// `'`, `\\` to `\`. Content is the Part 21 basic alphabet only —
    /// a raw byte outside 0x20..=0x7E or an undecoded control
    /// directive (`\S\`, `\X2\…`) refuses typed rather than mangling
    /// (the writer enforces the mirror-image boundary on export).
    ///
    /// **Wrapped literals (M7-4 Leg A).** ST-Developer folds its output
    /// at column ~72 wherever it happens to be, including the middle of
    /// a string, with no continuation character: the record is one
    /// logical line the writer physically broke. A raw `\n` (or
    /// `\r\n`) inside a literal is therefore *spliced out* — the two
    /// halves are one word (`…batt` ⁄ `ery…` is `battery`) — while the
    /// line counter still advances so later errors keep their true
    /// line. This is a lexical un-fold, not a decode: no character is
    /// invented, and every other non-basic byte (a lone `\r`, a raw
    /// tab, a UTF-8 lead byte) still refuses typed.
    fn string_body(&mut self) -> Result<String, StepImportError> {
        let mut out = String::new();
        loop {
            match self.peek() {
                Some(b'\'') => {
                    self.pos += 1;
                    if self.peek() == Some(b'\'') {
                        out.push('\'');
                        self.pos += 1;
                    } else {
                        return Ok(out);
                    }
                }
                Some(b'\\') if self.text.get(self.pos + 1) == Some(&b'\\') => {
                    out.push('\\');
                    self.pos += 2;
                }
                Some(b'\\') => {
                    return Err(self.syntax(
                        "a doubled backslash (a Part 21 string control directive — \
                         \\X2\\ / \\X\\ / \\S\\ — is outside the imported subset: \
                         these encode annotation text the import never consumes, and \
                         decoding one would put invented characters in a name)",
                    ));
                }
                // The writer's physical fold: spliced out, line counted.
                Some(b'\r') if self.text.get(self.pos + 1) == Some(&b'\n') => {
                    self.line += 1;
                    self.pos += 2;
                }
                Some(b'\n') => {
                    self.line += 1;
                    self.pos += 1;
                }
                Some(c) if (0x20..=0x7E).contains(&c) => {
                    out.push(c as char);
                    self.pos += 1;
                }
                Some(_) => {
                    return Err(
                        self.syntax("a Part 21 basic-alphabet string character (0x20..=0x7E)")
                    );
                }
                None => return Err(self.syntax("a terminated string literal")),
            }
        }
    }

    /// One parameter value.
    fn value(&mut self) -> Result<Value, StepImportError> {
        self.skip_ws();
        match self.peek() {
            Some(b'$') => {
                self.pos += 1;
                Ok(Value::Null)
            }
            Some(b'*') => {
                self.pos += 1;
                Ok(Value::Star)
            }
            Some(b'#') => {
                self.pos += 1;
                Ok(Value::Ref(self.integer("an entity id after '#'")?))
            }
            Some(b'\'') => {
                self.pos += 1;
                Ok(Value::Str(self.string_body()?))
            }
            Some(b'.') => {
                // Enumeration: .NAME. (LOGICALs .T./.F./.U. included).
                self.pos += 1;
                let start = self.pos;
                while self
                    .peek()
                    .is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_')
                {
                    self.pos += 1;
                }
                let name = String::from_utf8_lossy(&self.text[start..self.pos]).into_owned();
                if name.is_empty() || self.peek() != Some(b'.') {
                    return Err(self.syntax("an enumeration value '.NAME.'"));
                }
                self.pos += 1;
                Ok(Value::Enum(name))
            }
            Some(b'(') => {
                self.pos += 1;
                Ok(Value::List(self.value_list()?))
            }
            Some(c) if c == b'+' || c == b'-' || c.is_ascii_digit() => {
                let start = self.pos;
                self.pos += 1;
                while self.peek().is_some_and(|c| {
                    c.is_ascii_digit()
                        || c == b'.'
                        || c == b'E'
                        || c == b'e'
                        || c == b'+'
                        || c == b'-'
                }) {
                    self.pos += 1;
                }
                Ok(Value::Number(
                    String::from_utf8_lossy(&self.text[start..self.pos]).into_owned(),
                ))
            }
            Some(c) if c.is_ascii_uppercase() => {
                // Typed parameter: KEYWORD(args).
                let kw = self.keyword()?;
                self.expect("(", "'(' after a typed parameter keyword")?;
                Ok(Value::Typed(kw, self.value_list()?))
            }
            _ => Err(self.syntax("a parameter value")),
        }
    }

    /// The remainder of a `( … )` list after the opening parenthesis.
    fn value_list(&mut self) -> Result<Vec<Value>, StepImportError> {
        let mut out = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b')') {
            self.pos += 1;
            return Ok(out);
        }
        loop {
            out.push(self.value()?);
            self.skip_ws();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b')') => {
                    self.pos += 1;
                    return Ok(out);
                }
                _ => return Err(self.syntax("',' or ')' in a parameter list")),
            }
        }
    }

    /// One record: `KEYWORD(args)`.
    fn record(&mut self) -> Result<Record, StepImportError> {
        let kw = self.keyword()?;
        self.expect("(", "'(' after an entity keyword")?;
        Ok((kw, self.value_list()?))
    }
}

/// Parses a whole exchange file (module docs).
pub(crate) fn parse_file(text: &str) -> Result<StepFile, StepImportError> {
    let mut c = Cursor::new(text);
    c.expect("ISO-10303-21;", "the 'ISO-10303-21;' exchange-file magic")?;
    c.expect("HEADER;", "the HEADER section")?;
    let mut header = Vec::new();
    loop {
        c.skip_ws();
        if c.text[c.pos..].starts_with(b"ENDSEC;") {
            c.pos += "ENDSEC;".len();
            break;
        }
        let record = c.record()?;
        c.expect(";", "';' after a header record")?;
        header.push(record);
    }
    c.expect("DATA;", "the DATA section")?;
    let mut data = BTreeMap::new();
    loop {
        c.skip_ws();
        if c.text[c.pos..].starts_with(b"ENDSEC;") {
            c.pos += "ENDSEC;".len();
            break;
        }
        let line = c.line;
        c.expect("#", "'#id = …;' or 'ENDSEC;' in the DATA section")?;
        let id = c.integer("an entity id after '#'")?;
        c.expect("=", "'=' after the entity id")?;
        c.skip_ws();
        let records = if c.peek() == Some(b'(') {
            // Complex (multi-record) instance.
            c.pos += 1;
            let mut records = Vec::new();
            loop {
                c.skip_ws();
                if c.peek() == Some(b')') {
                    c.pos += 1;
                    break;
                }
                records.push(c.record()?);
            }
            records
        } else {
            vec![c.record()?]
        };
        c.expect(";", "';' after an entity instance")?;
        if data.insert(id, Instance { records }).is_some() {
            return Err(StepImportError::Syntax {
                line,
                expected: "a unique entity id (this id is defined twice)",
            });
        }
    }
    c.expect("END-ISO-10303-21;", "the 'END-ISO-10303-21;' trailer")?;
    Ok(StepFile { header, data })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{Value, parse_file};

    /// A file whose HEADER carries `body` verbatim and whose DATA
    /// section is empty — the lexical rows need no entities.
    fn header_only(body: &str) -> String {
        format!("ISO-10303-21;\nHEADER;\n{body}\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;")
    }

    /// Leg A: ST-Developer folds a long literal at column ~72 with no
    /// continuation character. The two halves are one word, and the
    /// line counter still advances (the `#` after the fold reports
    /// its true line).
    #[test]
    fn a_string_folded_across_a_raw_newline_splices_back_into_one_word() {
        let file = parse_file(&header_only("FILE_NAME('328 2500mAh batt\nery.step');")).unwrap();
        assert_eq!(file.header[0].0, "FILE_NAME");
        assert_eq!(
            file.header[0].1,
            [Value::Str("328 2500mAh battery.step".to_owned())]
        );
    }

    /// The same fold with NIST's CRLF line endings: `\r\n` is one
    /// break, not a carriage return inside the word.
    #[test]
    fn a_crlf_fold_splices_the_same_way_and_a_lone_cr_still_refuses() {
        let file = parse_file(&header_only("FILE_NAME('asserted c\r\nonnectivities');")).unwrap();
        assert_eq!(
            file.header[0].1,
            [Value::Str("asserted connectivities".to_owned())]
        );
        let lone = parse_file(&header_only("FILE_NAME('asserted c\ronnectivities');"));
        assert!(
            lone.unwrap_err()
                .to_string()
                .contains("basic-alphabet string character"),
            "a lone CR is not a fold — it stays outside the basic alphabet"
        );
    }

    /// The fold does not swallow the line count: an error after two
    /// folded literals still names the line it is on.
    #[test]
    fn folding_advances_the_line_counter() {
        let err = parse_file(&header_only("FILE_NAME('a\nb');\nFILE_SCHEMA(@);"))
            .unwrap_err()
            .to_string();
        // Line 1 is the magic, 2 the HEADER keyword, 3 the literal's
        // first half, 4 its folded tail; the bad record is on 5.
        assert!(err.contains("line 5"), "got {err}");
    }

    /// Fusion/ST-Developer annotate each argument slot with a comment
    /// inside the record; comments are whitespace at every position.
    #[test]
    fn comments_inside_an_entity_record_are_whitespace() {
        let file =
            parse_file(&header_only("FILE_NAME(\n/* name */ 'x',\n/* stamp */ 'y');")).unwrap();
        assert_eq!(
            file.header[0].1,
            [Value::Str("x".to_owned()), Value::Str("y".to_owned())]
        );
    }

    /// `\X2\` stays a typed refusal that names the directive family —
    /// it encodes annotation text the import never consumes, and
    /// decoding it would put invented characters in a name.
    #[test]
    fn an_x2_control_directive_refuses_typed_naming_the_directive() {
        let err = parse_file(&header_only(r"FILE_NAME('\X2\30D530A9\X0\');"))
            .unwrap_err()
            .to_string();
        assert!(err.contains(r"\X2\"), "got {err}");
    }
}
