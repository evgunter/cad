//! Reading Rust source as TEXT — the shared lexer, and the three views
//! of a file it supports, for the guards that pin a claim about the
//! code against the code itself.
//!
//! # Why this is here rather than in each guard
//!
//! A guard that greps its own crate's sources needs an answer to *is
//! this text code, prose, or a literal*, and the answer each guard
//! reached for was its own — no two lexing the same language, most
//! knowing only a line-leading `//`. The direction that costs is the
//! silent one: a real site commented out leaves its text in the file,
//! so a count does not move and the guard stays green over exactly the
//! change it exists to catch.
//!
//! **This is the shared answer, not the only one in the tree.**
//! `crates/test-utils/tests/reader_census.rs` enumerates every site
//! that reads Rust source as text and says which reader each uses; the
//! readers still outside this module are named there with the track
//! that owns each, and the shell gates under `scripts/gates/` have a
//! second home of their own in awk. **Read the ledger for the count —
//! this paragraph deliberately carries none**, because a number here
//! is a copy that goes stale in the direction that matters.
//!
//! This crate is where the shared answer lives because it is a
//! zero-dependency leaf that can sit below everything. Most crates
//! already dev-depend on it; `pncad` and `pncad-py` do not, and that
//! is not a detail — `pncad/tests/all.rs` holds the class's largest
//! unconverted reader, and adding the dev-dependency is the first
//! step of converting it.
//!
//! # One lexer, three views, and why the count is three
//!
//! [`keeping`] is the only thing in this module that knows Rust's
//! lexical grammar. Everything else is a SELECTION over its output, or
//! an operation that reads its output as balanced text, which is the
//! structural rule: a guard whose needle is a shape the three named
//! views do not cover asks [`keeping`] for the region set it wants,
//! and does not write a second lexer to get it.
//!
//! The three named views are the three a needle can want:
//!
//! - [`code_only`] — the needle is a **code fragment** (a call, an
//!   operator, an item head). Comments and literals both blanked:
//!   nothing an identifier read can hide inside survives, so blanking
//!   removes false positives and can lose no real site.
//! - [`code_and_literals`] — the needle **contains a string literal**
//!   (`#[path = "x.rs"]`, `decide("split_arc_window"`). Blanking the
//!   literal would make such a guard vacuous; blanking the comment is
//!   still the whole point.
//! - [`comments_only`] — the needle is **prose** (a doc-comment
//!   heading that is itself the ledger being pinned). The inverse
//!   view: code and literals blanked, so a heading spelled in a
//!   `format!` cannot satisfy a guard that means to read the docs.
//!   **A `#[doc = "…"]` ATTRIBUTE is not in this view**: it is a
//!   literal to the lexer, as it is to `rustc`, so a ledger written
//!   in attributes rather than in `///` reads as absent. That is the
//!   fail-red direction, and it is stated because it is the one place
//!   *"the docs"* and *"the comments"* are not the same set.
//!
//! # The operations over a blanked view
//!
//! A blanked view has a property raw source does not: **every bracket
//! in it is a real bracket**, because one inside a literal or a comment
//! is a space. Every operation below depends on exactly that
//! precondition, which is why each lives beside the lexer rather than
//! at each call site — [`balanced_end`], [`angle_end`],
//! [`top_level_split`] and [`item_body`], plus the traversals
//! [`rust_sources`] and [`suite_files`].
//!
//! A second group reads a DECLARATION out of a blanked view rather
//! than walking brackets in it: [`initializers`], [`sole_initializer`]
//! and [`plain_string_literal`], with [`blanked`] for the
//! same-bytes-blanked-in-place precondition every pin that locates in
//! one view and reads in another depends on. They are here for the
//! reason the lexer is: `tools/tess-meter` and `tools/k-lint` both pin
//! a constant across a cargo-root boundary and each arrived at the
//! same three functions. **No count is written here**,
//! for the reason the paragraph above gives about the ledger's: a
//! number in prose beside a list that grows is a copy that goes stale
//! in the silent direction.
//!
//! # The one thing here that is not a reader
//!
//! [`crate::every_suite_file_is_aggregated!`] is exported from this file and is
//! a `macro_rules!` that GENERATES A TEST FUNCTION — not a view, not an
//! operation over one, and not a traversal. It is here because its
//! expansion's whole body is [`aggregation_violations`] and the two are
//! one mechanism split by where the compiler has to expand it; homing it
//! anywhere else would put a call site and its reader in different
//! files. It is called out because every other paragraph in this header
//! describes something that reads text, and a fourth category that goes
//! unmentioned is how a header stops being an enumeration.
//!
//! # What it does not model
//!
//! It is a lexer, not a parser. An identifier assembled by a macro
//! (`concat_idents!`, `paste!`) is invisible to any textual walk, a
//! `pub fn` inside a `macro_rules!` body is text like any other — which
//! is now a fact about THIS file, since the macro above holds one, and
//! `crates/test-utils/tests/reader_census.rs` reads that body as text
//! for exactly that reason — and an `include!`d file is not seen at
//! all. Nested block comments,
//! every string prefix (`b`, `c`, `r`, `br`, `cr`) and the
//! lifetime-versus-char-literal distinction ARE modelled, each with a
//! row in this module's tests that reds if it stops being.

// This module PANICS and `expect`s, deliberately, for `fuzz`'s reason
// one file over: a guard whose source walk cannot read the tree, or
// finds nothing in it, must go RED rather than quietly assert over an
// empty set — that vacuity is the failure this crate exists to
// prevent. The workspace's no-panic rule is about production code and
// nothing on a shipped build path can reach here: no production
// manifest names this crate at all (see the crate docs).
#![allow(clippy::panic, clippy::expect_used)]

/// The three regions every byte of a Rust file lies in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Region {
    /// Everything that is neither a comment nor a literal.
    Code,
    /// `//` line comments and nested `/* … */` blocks, doc comments
    /// (`///`, `//!`, `/** */`) included — they are comments to the
    /// lexer and prose to a reader, which is what [`comments_only`]
    /// exists to search.
    Comment,
    /// String, byte-string, C-string, raw-string and char literals,
    /// **prefix and delimiters included**. A lifetime (`&'a str`) is
    /// code: a `'` opens a literal only when it closes within one
    /// escape or one scalar.
    Literal,
}

/// `text` with every byte OUTSIDE `keep` blanked to a space.
///
/// Newlines survive and every other removed byte becomes one space, so
/// byte offsets, line numbers and line structure are all preserved and
/// a caller may report a position into the ORIGINAL text from a match
/// in this one. A multi-byte character is blanked byte for byte, so it
/// is never half-erased.
///
/// **This is the only Rust lexer in this module, and the region set is
/// the knob.** The three named views below are selections over it; a
/// needle wanting a fourth combination passes the combination, and
/// does not fork the lexer to get it. Other Rust lexers do exist in
/// the tree — `crates/test-utils/tests/reader_census.rs` names each
/// with the track that owes its conversion.
#[must_use]
pub fn keeping(text: &str, keep: &[Region]) -> String {
    let b = text.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    let mut i = 0usize;
    while i < b.len() {
        let (end, region) = span(b, i);
        emit(&mut out, &b[i..end], keep.contains(&region));
        i = end;
    }
    String::from_utf8(out).expect("blanking never splits a character")
}

/// Rust source with every comment and every literal blanked — the view
/// for a needle that is a **code fragment**.
#[must_use]
pub fn code_only(text: &str) -> String {
    keeping(text, &[Region::Code])
}

/// Rust source with every comment blanked and literals KEPT — the view
/// for a needle that **contains a string literal**, which
/// [`code_only`] would erase and leave the guard vacuous.
///
/// The tree's largest caller is [`aggregation_violations`], the body of
/// the `every_suite_file_is_aggregated` row every aggregating crate's
/// `tests/all.rs` carries, whose needle is the mount
/// `#[path = "<suite>.rs"]`. **The argument is stated here so it is
/// stated once**: a mount that has been commented out must not answer
/// for the file it names, because `autotests = false` then drops a
/// whole suite from the build with the guard still green — the silent
/// direction, at its largest.
#[must_use]
pub fn code_and_literals(text: &str) -> String {
    keeping(text, &[Region::Code, Region::Literal])
}

/// A file's PROSE alone, with code and literals blanked — the inverse
/// view, for a guard whose subject is a doc comment.
///
/// The comment markers themselves survive, so a caller that means
/// `///` rather than `//` can still say so with a line test.
///
/// **`#[doc = "…"]` is NOT here.** A doc attribute is a string
/// literal, and this view blanks literals; a guard reading a ledger
/// that is written in attributes finds nothing and goes red, which is
/// the safe direction but is a surprise worth having in writing.
#[must_use]
pub fn comments_only(text: &str) -> String {
    keeping(text, &[Region::Comment])
}

/// Copy `bytes` through, or blank them keeping newlines.
fn emit(out: &mut Vec<u8>, bytes: &[u8], kept: bool) {
    out.extend(
        bytes
            .iter()
            .map(|&c| if kept || c == b'\n' { c } else { b' ' }),
    );
}

/// The end offset and region of the span starting at `i`.
///
/// Code is returned one byte at a time; every comment and literal is
/// returned whole, which is what makes a needle unable to straddle the
/// boundary.
fn span(b: &[u8], i: usize) -> (usize, Region) {
    if let Some(n) = raw_string_len(b, i) {
        return (i + n, Region::Literal);
    }
    match b[i] {
        b'/' if b.get(i + 1) == Some(&b'/') => {
            let end = b[i..]
                .iter()
                .position(|&c| c == b'\n')
                .map_or(b.len(), |r| i + r);
            (end, Region::Comment)
        }
        b'/' if b.get(i + 1) == Some(&b'*') => {
            // Rust's block comments NEST: the first `*/` does not
            // necessarily close the one that opened here.
            let (mut depth, mut j) = (1usize, i + 2);
            while j < b.len() && depth > 0 {
                if b[j] == b'/' && b.get(j + 1) == Some(&b'*') {
                    depth += 1;
                    j += 2;
                } else if b[j] == b'*' && b.get(j + 1) == Some(&b'/') {
                    depth -= 1;
                    j += 2;
                } else {
                    j += 1;
                }
            }
            (j.min(b.len()), Region::Comment)
        }
        b'"' => (i + quoted_len(b, i), Region::Literal),
        b'b' | b'c' if b.get(i + 1) == Some(&b'"') && token_start(b, i) => {
            (i + 1 + quoted_len(b, i + 1), Region::Literal)
        }
        // `b'x'` — the byte-CHAR literal. Only `b` takes this prefix;
        // there is no `c'…'` in Rust. Written out because the string
        // arm above does not cover it, and a prefix left behind as
        // code is [`Region::Literal`]'s contract broken rather than
        // its partition: the stray `b` is a token no guard means.
        b'b' if b.get(i + 1) == Some(&b'\'') && token_start(b, i) => {
            match char_literal_len(b, i + 1) {
                Some(n) => (i + 1 + n, Region::Literal),
                None => (i + 1, Region::Code),
            }
        }
        b'\'' => match char_literal_len(b, i) {
            Some(n) => (i + n, Region::Literal),
            // A lifetime. One byte of code, so the scan resumes on the
            // name rather than swallowing the rest of the file.
            None => (i + 1, Region::Code),
        },
        _ => (i + 1, Region::Code),
    }
}

/// Whether the token starting at `i` starts a token — nothing
/// identifier-shaped immediately before it.
fn token_start(b: &[u8], i: usize) -> bool {
    !i.checked_sub(1)
        .is_some_and(|p| b[p].is_ascii_alphanumeric() || b[p] == b'_')
}

/// The byte length of the `"…"` literal opening at `i`, escapes
/// honoured. An unterminated literal runs to end of input.
fn quoted_len(b: &[u8], i: usize) -> usize {
    let mut j = i + 1;
    while j < b.len() && b[j] != b'"' {
        j += usize::from(b[j] == b'\\') + 1;
    }
    (j + 1).min(b.len()) - i
}

/// The byte length of the RAW string literal opening at `i` — `r"…"`,
/// `r#"…"#`, `br"…"`, `cr#"…"#` and so on — or `None`.
///
/// Raw strings are lexed apart from the plain arm rather than falling
/// through to it because that arm honours `\` escapes, which a raw
/// string does not have: `br"x\"` read as an escaped quote is an
/// unclosed string that blanks the rest of the file. That exact
/// spelling is the one this tree has got wrong three times, which is
/// why every prefix has a row in this module's tests.
fn raw_string_len(b: &[u8], i: usize) -> Option<usize> {
    if !token_start(b, i) {
        return None;
    }
    let mut j = i;
    if matches!(b.get(j), Some(b'b' | b'c')) {
        j += 1;
    }
    if b.get(j) != Some(&b'r') {
        return None;
    }
    j += 1;
    let mut hashes = 0usize;
    while b.get(j + hashes) == Some(&b'#') {
        hashes += 1;
    }
    if b.get(j + hashes) != Some(&b'"') {
        return None;
    }
    let mut k = j + hashes + 1;
    loop {
        if k >= b.len() {
            return Some(b.len() - i);
        }
        if b[k] == b'"'
            && b[k + 1..]
                .iter()
                .take(hashes)
                .filter(|c| **c == b'#')
                .count()
                == hashes
        {
            return Some((k + 1 + hashes).min(b.len()) - i);
        }
        k += 1;
    }
}

/// The byte length of the char literal at `i`, or `None` when the
/// quote opens a LIFETIME instead. Mis-reading `'a` as an opening
/// quote would swallow the rest of the file.
fn char_literal_len(b: &[u8], i: usize) -> Option<usize> {
    if b.get(i + 1) == Some(&b'\\') {
        // `'\n'`, `'\''`, `'\\'`, `'\u{1F600}'`. The scan starts AT the
        // backslash, so the escape skip applies to it — starting one
        // byte later reads `'\''` as ending early and `'\\'` as
        // unterminated.
        let mut j = i + 1;
        while j < b.len() && b[j] != b'\'' {
            j += usize::from(b[j] == b'\\') + 1;
        }
        return (j < b.len()).then_some(j + 1 - i);
    }
    // One character, then a closing quote. A multi-byte char is one
    // char and several bytes, so step by the character's own width.
    let rest = std::str::from_utf8(b.get(i + 1..)?).ok()?;
    let c = rest.chars().next()?;
    let w = c.len_utf8();
    (b.get(i + 1 + w) == Some(&b'\'')).then_some(w + 2)
}

/// Every `.rs` file under `dir`, **recursively**, sorted.
///
/// Here rather than at each caller because a flat `read_dir` is the
/// other half of the copied walk: sharing the *predicate*
/// ([`keeping`]) and re-forking the *traversal* leaves each guard free
/// to miss a subdirectory, silently and in the green direction.
///
/// **Panics** if the walk finds nothing: a guard built on an empty
/// traversal passes by finding no sites, which is the vacuity this
/// crate exists to forbid. A caller wanting a stronger floor (a
/// **The text a pair of sentinel comments guards**, as a byte range
/// into `text`: from the end of `begin` to the start of `end`.
///
/// A guard whose subject is one REGION of a file — a match, a module,
/// a door — brackets it with two comments and reads between them, so
/// that a row added inside is measured the moment it is typed and one
/// added outside is not silently counted. Three sites had written this
/// walk themselves before it was hoisted here, and the third was
/// nearly line-for-line the second.
///
/// **Located in the RAW text, deliberately.** The sentinels are
/// comments, so every [`keeping`] view blanks them — and because a
/// view blanks in place, the range this answers is valid in the raw
/// text and in every view of it alike. A caller locates the region
/// here and reads it out of whichever view its needle wants.
///
/// `what` names the text in the refusals: a guard that reported a
/// missing sentinel without saying which file sends its reader
/// nowhere.
///
/// **Panics** when either sentinel is absent or they are inverted. A
/// region that cannot be located is not an empty region — a guard that
/// answered `0..0` would report green over the text it exists to read,
/// which is the silent direction.
#[must_use]
pub fn sentinel_region(text: &str, what: &str, begin: &str, end: &str) -> std::ops::Range<usize> {
    let b = text
        .find(begin)
        .unwrap_or_else(|| panic!("{what}: the opening sentinel `{begin}` is gone"));
    let e = text
        .find(end)
        .unwrap_or_else(|| panic!("{what}: the closing sentinel `{end}` is gone"));
    assert!(
        b < e,
        "{what}: `{begin}` must precede `{end}`, and does not"
    );
    b + begin.len()..e
}

/// **Every offset `needle` matches at in `text`, refused when there is
/// none** — [`str::match_indices`] for a scan whose file is REQUIRED to
/// carry its needle.
///
/// A needle that stopped matching — a marker renamed, a file moved, a
/// path re-spelled — makes a census read an empty set, and an empty
/// set satisfies every equality and containment it feeds. That is not
/// an honest zero: the scan's premise is broken, so it refuses at the
/// read, naming the text and the needle, rather than hand the caller a
/// set whose emptiness some later assertion has to remember to check.
/// A scan that may legitimately match nothing wants
/// [`str::match_indices`] itself.
///
/// `what` names the text in the refusal, as for [`sentinel_region`].
#[must_use]
pub fn required_matches(text: &str, what: &str, needle: &str) -> Vec<usize> {
    let found: Vec<usize> = text.match_indices(needle).map(|(at, _)| at).collect();
    assert!(
        !found.is_empty(),
        "{what}: `{needle}` matches nothing, and this scan's text is required to carry it — \
         the needle or the file has drifted from what the scan was built to read"
    );
    found
}

/// The offset at which the line holding `at` begins.
///
/// The offset half of [`line()`], which answers the line NUMBER. Here
/// because three readers had spelled the same `rfind('\n')` fold, and
/// a caller wanting the text before a match ON ITS OWN LINE — a
/// declaration's modifiers, an attribute's indentation — is asking one
/// question the tree had three answers to.
#[must_use]
pub fn line_start(text: &str, at: usize) -> usize {
    text[..at].rfind('\n').map_or(0, |nl| nl + 1)
}

/// **The 1-based line `at` falls on**, for a guard whose refusal a
/// reader has to be able to open. Offsets come out of a blanked view
/// and a view blanks in place, so this is correct against the raw text
/// and against every view of it alike.
///
/// Shared because two censuses wrote it byte-identically before this
/// existed, and a guard that spells its own version of a shared
/// operation is the defect the module docs above argue against.
#[must_use]
pub fn line(text: &str, at: usize) -> usize {
    text[..at].lines().count()
}

/// **Is the byte before `at` outside an identifier?** — what makes a
/// needle match a WHOLE name. Without it `operand` matches inside
/// `wrong_operand` and `kind(` inside `carrier_kind(`; `true` at the
/// start of the text, where there is no preceding byte to disqualify
/// the match.
///
/// A needle that ends in punctuation (`kind(`, `Variant {`) carries
/// its own trailing boundary and needs only this side;
/// [`boundary_after`] is the other half, for a needle that is a bare
/// identifier and carries neither.
#[must_use]
pub fn boundary_before(code: &str, at: usize) -> bool {
    code[..at]
        .chars()
        .next_back()
        .is_none_or(|c| !c.is_alphanumeric() && c != '_')
}

/// **Is the byte at `at` outside an identifier?** — the trailing half
/// of a whole-word match, where `at` is one past the needle's last
/// byte. `true` at the end of the text, where there is no following
/// byte to disqualify the match.
///
/// **A bare-identifier needle needs both halves and neither is
/// optional.** `deny_unknown_fields` is a whole attribute word and
/// also the prefix of `deny_unknown_fields_census`, so a leading-only
/// check reports a match inside the longer name and sends the reader
/// looking for a declaration that is not there. This is the shared
/// spelling of that check because the predicate — not alphanumeric and
/// not `_` — is [`boundary_before`]'s byte for byte, and a caller that
/// writes its own copy has minted the duplication this module exists
/// to remove.
#[must_use]
pub fn boundary_after(code: &str, at: usize) -> bool {
    code[at..]
        .chars()
        .next()
        .is_none_or(|c| !c.is_alphanumeric() && c != '_')
}

/// minimum count, a required file) should assert it on the result.
#[must_use]
pub fn rust_sources(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    fn collect(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        let entries = std::fs::read_dir(dir)
            .unwrap_or_else(|e| panic!("a readable source directory {}: {e}", dir.display()));
        for entry in entries {
            let path = entry.expect("a readable directory entry").path();
            if path.is_dir() {
                collect(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    collect(dir, &mut out);
    assert!(
        !out.is_empty(),
        "the walk of {} found no .rs file — every guard built on it would pass by \
         finding nothing",
        dir.display()
    );
    out.sort();
    out
}

/// The byte offset of the delimiter closing the bracket that opens at
/// `open`, or `None` if it never closes.
///
/// **`blanked` must be a view from [`keeping`] that drops literals and
/// comments** — that is the precondition the whole operation rests on:
/// in such a view every bracket is a real bracket, so depth counting
/// IS the parse and no literal tracker is needed. Run over raw source
/// it is wrong, silently.
///
/// `(`, `[` and `{` all count, because a call's argument list can
/// contain either of the others and a guard that carves one wants the
/// region, not the kind.
#[must_use]
pub fn balanced_end(blanked: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (off, c) in blanked[open..].char_indices() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => {
                // `checked_sub`, so an `open` that is not an opener
                // answers `None` rather than underflowing — a panic in
                // debug and a wrong answer in release is the one
                // outcome a shared helper must not have. Where
                // [`top_level_split`] clamps instead, the difference is
                // deliberate and stated there.
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(open + off);
                }
            }
            _ => {}
        }
    }
    None
}

/// What follows an item's head: a body, a `;`, or neither.
///
/// Three variants because a guard that walks items needs all three
/// apart, and the third is the one a two-way answer loses. **A head
/// that runs to end of input without either terminator is not a
/// declaration** — it is a truncated or unbalanced text — and a walk
/// that reads it as one skips an item silently, which for a guard
/// asserting about ALL items is the failure it exists to prevent.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ItemBody {
    /// The `{ … }` region, **both braces included**. Slice
    /// `start + 1..end - 1` for the inside.
    Body(std::ops::Range<usize>),
    /// A head with no body — a trait's default-less method, an
    /// `extern` block's declaration, a unit struct — at the offset of
    /// the `;` that ends it. The offset is carried so a walk can
    /// resume after the declaration rather than re-scanning for it.
    Declaration(usize),
    /// Neither terminator before end of input.
    Unterminated,
}

/// What follows the item head starting at `from` — [`ItemBody`].
///
/// Same precondition as [`balanced_end`], and the same reason it is
/// here: over a [`keeping`] view with comments and literals dropped
/// every bracket is a real bracket, so this is a parse and not a
/// guess. Over raw text a `}` inside a string ends the body early and
/// the caller silently loses everything after the cut.
///
/// **The terminator is the first `{` or `;` OUTSIDE every round and
/// square bracket**, which is the whole content of the operation and
/// the part a call-site copy gets wrong. `fn f() -> [f64; 3] {` and
/// `fn g(p: impl Into<[u8; 2]>) {` each carry a `;` that ends no item;
/// read as a terminator it answers [`ItemBody::Declaration`] and the
/// item — its body, and every needle in it — is dropped with no count
/// anywhere moving. Array types in return position are house style in
/// this kernel, so that is not a hypothetical.
///
/// `from` may be anywhere in the head, including its first byte: a
/// `pub(crate)` before the `fn` opens and closes its own paren and is
/// stepped over. What this cannot check is that `from` IS a head — it
/// answers about the first terminator after the offset it was given,
/// so a caller that located the head by a substring search owes its
/// own check that the match is a head and not a mention (the blanked
/// view has already removed the mentions that live in prose and
/// literals).
#[must_use]
pub fn item_body(blanked: &str, from: usize) -> ItemBody {
    let b = blanked.as_bytes();
    let (mut paren, mut bracket) = (0usize, 0usize);
    let mut open = None;
    for (i, c) in b.iter().enumerate().skip(from) {
        match c {
            b'(' => paren += 1,
            b')' => paren = paren.saturating_sub(1),
            b'[' => bracket += 1,
            b']' => bracket = bracket.saturating_sub(1),
            b';' if paren == 0 && bracket == 0 => return ItemBody::Declaration(i),
            b'{' if paren == 0 && bracket == 0 => {
                open = Some(i);
                break;
            }
            _ => {}
        }
    }
    match open.and_then(|o| balanced_end(blanked, o).map(|end| o..end + 1)) {
        Some(body) => ItemBody::Body(body),
        // A `{` that never closes is the same broken text as no
        // terminator at all, and says so rather than answering a range
        // that runs to the end of the file.
        None => ItemBody::Unterminated,
    }
}

/// The byte offset of the `>` closing the generic list that opens at
/// `open`, or `None` if the item's body arrives first.
///
/// Same precondition as [`balanced_end`]: a [`keeping`] view with
/// literals and comments dropped, so every bracket is a real bracket.
///
/// **The item's body opens at the first `{` or `;` OUTSIDE every
/// square and round bracket.** A fixed-size array in a generic
/// argument — `<Item = [Expr; 2]>` — carries a `;` that ends no item,
/// and reading it as a terminator closes the list early. That is the
/// same nesting [`top_level_split`] respects for its separators, and
/// it is here rather than at a call site for the reason stated there.
///
/// **`->` does not close a list.** An arrow's `>` is not an angle
/// bracket, so a bound like `<F: Fn(u32) -> u32>` would otherwise
/// drive the depth to zero early and answer a list that is too SHORT
/// — an undercount, and silent, because a short list still parses.
///
/// **The residue, stated:** a genuine `>` comparison inside a const
/// generic argument still closes the list early, and this answers
/// `Some` at the wrong place rather than `None`. Angle brackets are
/// not a bracket language; depth counting is a heuristic here in a way
/// it is not in [`balanced_end`]. A caller that must not undercount
/// owes its own check that the answer it got is where it expected one.
#[must_use]
pub fn angle_end(blanked: &str, open: usize) -> Option<usize> {
    let (mut depth, mut brackets) = (0i32, 0i32);
    let bytes = blanked.as_bytes();
    for (off, c) in blanked[open..].char_indices() {
        let at = open + off;
        match c {
            '<' => depth += 1,
            // `-` immediately before is an arrow, not a closer.
            '>' if at > 0 && bytes[at - 1] == b'-' => {}
            '>' => {
                depth -= 1;
                if depth == 0 {
                    return Some(at);
                }
            }
            '[' | '(' => brackets += 1,
            ']' | ')' => brackets -= 1,
            '{' | ';' if brackets == 0 => return None,
            _ => {}
        }
    }
    None
}

/// What an `impl` head names: the trait it implements, where it names
/// one, and the self type it implements it for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplHead {
    /// The trait as the head WRITES it, path qualification and generic
    /// arguments kept: `core::fmt::Debug`, `PartialEq<Other>`. `None`
    /// on an inherent `impl`. A spelling, never a resolution — a
    /// caller wanting the bare word asks [`type_base`] for it, and one
    /// that must not conflate `PartialEq<Other>` with `PartialEq` does
    /// not.
    pub trait_path: Option<String>,
    /// The self type as the head writes it, whitespace collapsed:
    /// `Coset`, `SignCertificate<'_, T>`.
    pub self_type: String,
}

/// The trait and self type the `impl` at `impl_at` names, or `None`
/// where the head is one this reader cannot parse.
///
/// **The generic list is stepped over first.** `impl<P: PartialEq>
/// Doc<P>` names a trait in a BOUND and implements none; reading the
/// whole head for the word answers a trait the impl does not have.
///
/// `blanked` is a [`code_only`] view and `body_start` the offset of
/// the body's `{`, which is what keeps a `for` or a `<` inside a
/// comment or a literal out of the head.
///
/// **Shared because two censuses read impl heads.**
/// `crates/test-utils/tests/hand_written_impl_census.rs` keys its
/// suppression list on `(path, trait, self type)` and
/// `crates/pncad-py/src/tests.rs` keys its minting roster on
/// `(trait, self type, item name)`; both had written this walk, by two
/// algorithms, and a reader hosted inside one of its consumers is how
/// the tree got its drift.
#[must_use]
pub fn impl_head(blanked: &str, impl_at: usize, body_start: usize) -> Option<ImplHead> {
    let mut at = skip_ws(blanked, impl_at + "impl".len());
    if blanked[at..].starts_with('<') {
        at = angle_end(blanked, at)? + 1;
    }
    let head = blanked.get(at..body_start)?;
    let Some(for_at) = top_level_for(head) else {
        return Some(ImplHead {
            trait_path: None,
            self_type: collapsed(head),
        });
    };
    Some(ImplHead {
        trait_path: Some(collapsed(&head[..for_at])),
        self_type: collapsed(&head[for_at + "for".len()..]),
    })
}

/// A type or trait spelling with its `where` clause, its body brace and
/// its surrounding whitespace dropped, and the whitespace inside it
/// collapsed to one space.
///
/// A `where` clause is a bound on the impl and not part of the type,
/// and an UNTERMINATED head runs past the body's own brace — so both
/// end the spelling.
fn collapsed(spelling: &str) -> String {
    let mut end = where_at(spelling).unwrap_or(spelling.len());
    if let Some(brace) = spelling[..end].find('{') {
        end = brace;
    }
    spelling[..end]
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// The offset of the first whole-word `where` in `spelling`, or `None`.
///
/// **Whole-word, and the first one that is.** A `where` clause is a
/// bound and not part of the spelling before it, so a reader cutting a
/// type or a return type ends it here; an identifier that merely
/// contains the letters (`somewhere`, `where_clause`) ends nothing, and
/// a scan that stops at the first substring match instead reads past
/// the real keyword or cuts at the wrong place. Same precondition as
/// [`balanced_end`]: a [`code_only`] view, so a `where` in a comment or
/// a literal is not in the text.
#[must_use]
pub fn where_at(spelling: &str) -> Option<usize> {
    spelling
        .match_indices("where")
        .map(|(at, _)| at)
        .find(|&at| word_at(spelling, at, "where"))
}

/// The offset of the first whole-word `for` in `head` at bracket depth
/// zero, or `None`.
///
/// **Depth matters and is not decoration.** A bound like
/// `impl<T: Fn(&str) -> bool> …` and a higher-ranked
/// `for<'a> Trait<'a>` both put a `for`-shaped token where it ends no
/// trait; reading either as the separator answers a trait that is not
/// one. Round and square brackets are counted, angle brackets are not —
/// the generic list is stepped over by [`angle_end`] before this runs,
/// so a `for<'a>` inside a WHERE clause is past the body already.
fn top_level_for(head: &str) -> Option<usize> {
    let (mut paren, mut bracket) = (0usize, 0usize);
    for (at, c) in head.char_indices() {
        match c {
            '(' => paren += 1,
            ')' => paren = paren.saturating_sub(1),
            '[' => bracket += 1,
            ']' => bracket = bracket.saturating_sub(1),
            'f' if paren == 0 && bracket == 0 && word_at(head, at, "for") => {
                // `for<'a>` is a binder, not the separator.
                let after = skip_ws(head, at + 3);
                if !head[after..].starts_with('<') {
                    return Some(at);
                }
            }
            _ => {}
        }
    }
    None
}

/// The bare name at the head of a type or trait spelling: its path
/// qualification and its generic arguments dropped.
///
/// `SignCertificate<'_, T>` is destructured as `SignCertificate { … }`
/// and `crate::mate::Coset` as `Coset { … }`, so this is the word a
/// pattern would carry — and the word a roster keyed on a type carries
/// for `Foo<'a>` and `Foo` alike.
#[must_use]
pub fn type_base(spelling: &str) -> &str {
    spelling
        .split('<')
        .next()
        .unwrap_or_default()
        .rsplit("::")
        .next()
        .unwrap_or_default()
        .trim()
}

/// The identifier beginning at `at`, empty when none does.
///
/// **A raw identifier is ONE identifier, `r#` included.** Reading
/// `r#type` as `r` leaves the reader looking at a `#`, and a struct
/// variant whose name is a keyword then reads as a unit one — a false
/// green, since the enum has a named field the attribute denies. A
/// second reader had written the plain-alphanumeric half of this
/// without the `r#` arm, which is the same defect one keyword away.
#[must_use]
pub fn ident(code: &str, at: usize) -> &str {
    let from = if code[at..].starts_with("r#") {
        at + 2
    } else {
        at
    };
    let end = code[from..]
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .map_or(code.len(), |off| from + off);
    &code[at..end]
}

/// The byte ranges of the `sep`-separated items of `blanked` at bracket
/// depth zero.
///
/// Same precondition as [`balanced_end`], and the same reason it is
/// here rather than at its call site: over a blanked view a comma
/// inside a string literal is a space, so splitting an argument list
/// needs no literal tracking — and a copy of this loop at the call
/// site is a copy of a lexer's postcondition, which is how the tree
/// got its readers in the first place.
///
/// `<` and `>` count as brackets: an argument list is the caller, and
/// a turbofish or a generic argument must not split one.
///
/// **Depth CLAMPS at zero here, where [`balanced_end`] refuses.** The
/// input is a fragment carved out of a larger expression, so a closer
/// with no opener inside it is ordinary — `>` in `a -> b` is one —
/// and the operation is "split at depth zero", which a clamp answers
/// and an underflow does not.
#[must_use]
pub fn top_level_split(blanked: &str, sep: char) -> Vec<std::ops::Range<usize>> {
    let mut depth = 0usize;
    let mut out = Vec::new();
    let mut start = 0usize;
    for (off, c) in blanked.char_indices() {
        match c {
            '(' | '[' | '{' | '<' => depth += 1,
            ')' | ']' | '}' | '>' => depth = depth.saturating_sub(1),
            c if c == sep && depth == 0 => {
                out.push(start..off);
                start = off + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(start..blanked.len());
    out
}

/// The value of a plain string literal, or `None` where `text` is not
/// one.
///
/// **Plain, and nothing is decoded.** A `concat!`, a raw string or an
/// escape is not a literal this answers for — the callers refuse what
/// it returns `None` on rather than guessing, because a mis-decoded
/// constant is a green pin over a value nothing in the tree uses.
///
/// `text` is a slice of a [`code_and_literals`] view, which is what
/// makes the quotes real quotes.
#[must_use]
pub fn plain_string_literal(text: &str) -> Option<&str> {
    text.trim()
        .strip_prefix('"')
        .and_then(|q| q.strip_suffix('"'))
}

/// Every initializer following `decl` in `view`, as byte ranges: from
/// the end of the declaration head to the `;` that closes it.
///
/// **`view` is a blanked view, and that is what makes an answer a
/// declaration.** Over raw text the first occurrence wins, so a doc
/// comment quoting the declaration — directly above it, where such a
/// comment is written — outranks the declaration itself and the
/// caller reads prose; over [`code_only`] every occurrence is real
/// code. The closing `;` is sought in the same view, so one inside the
/// initializer's own string cannot end the statement early.
///
/// **A `;` inside the declaration's own TYPE ends it early**, which an
/// array type (`[&str; 3]`) has and a scalar one does not; a caller
/// whose `decl` stops before a type of that shape wants its own walk.
///
/// **An empty answer is legitimate here, and not refused**, unlike
/// [`required_matches`]: the callers ask one file at a time for a
/// declaration that lives in only one of them, so most reads are empty
/// by construction. A caller that needs exactly one declaration refuses
/// on the count it gathers, as [`sole_initializer`] does over one view
/// and `refusal_concision_chains`' constant read does across a tree.
#[must_use]
pub fn initializers(view: &str, decl: &str) -> Vec<std::ops::Range<usize>> {
    view.match_indices(decl)
        .map(|(at, _)| {
            let start = at + decl.len();
            let end = start + view[start..].find(';').expect("the declaration ends");
            start..end
        })
        .collect()
}

/// The ONE initializer `decl` has in `view`, which `searched` names
/// for the refusal below.
///
/// Exactly one: a second declaration of the same name is an ambiguity
/// a textual pin cannot resolve, and answering with either of them
/// silently is the failure this helper exists to refuse. `searched` is
/// a parameter because `view` is any text — a fixture as readily as a
/// crate's source — and a message naming the wrong one sends its
/// reader to a file that is not the one that failed.
#[must_use]
pub fn sole_initializer(view: &str, searched: &str, decl: &str) -> std::ops::Range<usize> {
    let mut found = initializers(view, decl);
    assert!(
        found.len() == 1,
        "`{decl}` is declared {} times in {searched}, not once",
        found.len()
    );
    found.remove(0)
}

/// `view(text)`, refused unless it is `text`'s bytes blanked IN PLACE.
///
/// Every pin that LOCATES a declaration in one view and READS its
/// value out of another depends on the two views agreeing byte for
/// byte, and each such pin asserted that for itself. `searched` names
/// the text in the refusal.
#[must_use]
pub fn blanked(view: fn(&str) -> String, searched: &str, text: &str) -> String {
    let out = view(text);
    assert_eq!(
        out.len(),
        text.len(),
        "a blanked view of {searched} is the original's bytes, blanked in place"
    );
    out
}

/// The directory of the crate whose `CARGO_MANIFEST_DIR` is `baked`.
///
/// **Both ways a suite runs.** A plain `cargo test` resolves the path
/// baked in at compile time; a nextest ARCHIVE replayed on another
/// runner has no such directory, and `--workspace-remap` has instead
/// pointed the per-test cwd at the crate root. Every guard that opens
/// a file relative to its own crate needs both, and each one that
/// worked it out again wrote the same paragraph next to the same six
/// lines.
///
/// **Panics** when neither candidate holds a `Cargo.toml`: a guard
/// that silently resolved to the wrong root would read the wrong
/// tree.
#[must_use]
pub fn crate_dir(baked: &str) -> std::path::PathBuf {
    let baked = std::path::PathBuf::from(baked);
    if baked.join("Cargo.toml").is_file() {
        return baked;
    }
    let cwd = std::env::current_dir().expect("a working directory");
    assert!(
        cwd.join("Cargo.toml").is_file(),
        "neither {} nor {} is a crate root",
        baked.display(),
        cwd.display()
    );
    cwd
}

/// The REPOSITORY root, for a guard whose subject is the whole tree:
/// [`crate_dir`]'s answer two levels up, canonicalized.
///
/// **Canonical, so `..` is not a path COMPONENT.** Every caller of
/// this walks `rust_sources` from the root and skips directories by
/// component name; a root still spelled `…/crates/test-utils/../..`
/// carries a `..` component of its own, which a skip list matches
/// against every file in the tree at once — and that looks exactly
/// like a clean walk.
///
/// **Panics** when the result holds no `Cargo.toml`: a guard that
/// resolved to the wrong root would read the wrong tree and report
/// agreement over it. Five guards had written these eight lines
/// themselves, in four crates.
#[must_use]
pub fn repo_root(baked: &str) -> std::path::PathBuf {
    let root = crate_dir(baked)
        .join("../..")
        .canonicalize()
        .expect("the repository root resolves");
    assert!(
        root.join("Cargo.toml").is_file(),
        "{} is not the repository root",
        root.display()
    );
    root
}

/// `at` advanced past whitespace.
///
/// Here rather than at each reader for [`boundary_after`]'s reason: a
/// walk that steps a cursor over a blanked view needs it at every
/// token, and three of them had written the same five lines. A
/// comment or a literal is whitespace in a blanked view, so a step
/// over this crosses them too.
#[must_use]
pub fn skip_ws(code: &str, mut at: usize) -> usize {
    while let Some(c) = code[at..].chars().next() {
        if !c.is_whitespace() {
            break;
        }
        at += c.len_utf8();
    }
    at
}

/// Whether `word` sits at `at` as a WHOLE word — both boundaries, and
/// neither is optional.
///
/// The positional half of the whole-word test: a caller that has
/// already found a candidate offset asks this, where one searching for
/// the first occurrence filters `match_indices` through it.
#[must_use]
pub fn word_at(code: &str, at: usize, word: &str) -> bool {
    code[at..].starts_with(word)
        && boundary_before(code, at)
        && boundary_after(code, at + word.len())
}

/// Every SUITE file under a crate's `tests/` directory, relative to it,
/// `/`-separated and sorted, with `all.rs` itself excluded.
///
/// Recursive, and shared for the reason [`rust_sources`] is: every
/// aggregating crate carries a row asserting every suite is mounted in
/// `tests/all.rs`, and while each of them walked `tests/` itself all
/// but one used a FLAT `read_dir` — so all but one could not see a
/// suite in a group directory at all, and nothing recorded that they
/// were the weaker variant. The row's whole body is
/// [`aggregation_violations`] now, and this is the walk it makes.
///
/// **A suite and a shared HELPER are told apart by Rust's own module
/// rule, not by a list.** A subdirectory holding a `mod.rs` is a module
/// directory: its files are reached through `mod <name>;` inside
/// whatever suite declares it, so they are not test targets and owe no
/// `#[path]` line. A subdirectory without one is a group of suites,
/// each of which does. That is why `tests/common/mod.rs` is not
/// reported and `tests/curves/boxes.rs` is.
#[must_use]
pub fn suite_files(tests_dir: &std::path::Path) -> Vec<String> {
    rust_sources(tests_dir)
        .iter()
        .filter_map(|path| {
            let rel = path
                .strip_prefix(tests_dir)
                .expect("a walked file lies under tests/")
                .to_string_lossy()
                .replace('\\', "/");
            if rel == "all.rs" {
                return None;
            }
            // A module directory anywhere above it: its files are
            // reached through `mod <name>;`, never through `#[path]`.
            let mut dir = path.parent();
            while let Some(d) = dir {
                if d == tests_dir {
                    break;
                }
                if d.join("mod.rs").is_file() {
                    return None;
                }
                dir = d.parent();
            }
            Some(rel)
        })
        .collect()
}

/// Every `mod <name>;` declaration in a Rust source text — the FILE
/// form of a module declaration, never an inline `mod <name> { … }`.
///
/// **ONE HOME, and why the shape is the thing being counted.** In an
/// aggregated test binary (`autotests = false`, every suite mounted in
/// `tests/all.rs` with `#[path]`) a `mod <name>;` inside a SUITE loads
/// that helper file as a module of that suite — so the helper is
/// parsed, resolved, type-checked and codegen'd once per suite that
/// declares it, inside the one binary. Declared once at the root of
/// `all.rs` and reached with `use crate::<name>;`, it is compiled once
/// for the whole binary. An inline `mod <name> { … }` block loads no
/// file and duplicates nothing, so it is deliberately NOT reported:
/// suites use it freely to group rows (`mod interval { … }`).
///
/// The view is [`code_only`], not [`code_and_literals`]: this needle is
/// a code fragment carrying no literal of its own, so blanking literals
/// as well as comments can lose no real declaration and it removes the
/// one false positive available — a `"mod x;"` written inside a string.
///
/// Names are returned in source order, with repeats kept: the caller is
/// reporting sites, not a set.
#[must_use]
pub fn file_module_decls(text: &str) -> Vec<String> {
    let blanked = code_only(text);
    let b = blanked.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    while let Some(off) = blanked[i..].find("mod") {
        let at = i + off;
        i = at + "mod".len();
        // `mod` has to be a whole token, not the tail of `xmod`.
        if at > 0 && (b[at - 1].is_ascii_alphanumeric() || b[at - 1] == b'_') {
            continue;
        }
        let mut j = i;
        // ... and the head of one: `model` is not a declaration.
        if j >= b.len() || !b[j].is_ascii_whitespace() {
            continue;
        }
        while j < b.len() && b[j].is_ascii_whitespace() {
            j += 1;
        }
        let start = j;
        while j < b.len() && (b[j].is_ascii_alphanumeric() || b[j] == b'_') {
            j += 1;
        }
        if j == start {
            continue;
        }
        let name = &blanked[start..j];
        while j < b.len() && b[j].is_ascii_whitespace() {
            j += 1;
        }
        // `;` is the file form; `{` is an inline block and costs nothing.
        if j < b.len() && b[j] == b';' {
            out.push(name.to_string());
        }
    }
    out
}

/// **The aggregation guard's whole verdict**, as the list of violations
/// it found — empty when a crate's `tests/` directory and its
/// `tests/all.rs` agree. Each aggregating crate's
/// `every_suite_file_is_aggregated` row asserts this list is empty, and
/// that call is the whole of it.
///
/// # Why the checks live here and not at each aggregator
///
/// Every aggregating crate carries the same row, and the row used to be
/// the same twenty-five lines copied into each of them: the walk, the
/// mount scan, the module-declaration read, and three messages. A
/// message, an exemption or a fourth check was then one edit per copy,
/// all of which had to agree — the shape that drifts, and a guard that
/// has drifted is weaker in whichever copy was missed without saying
/// so. One home makes it one edit, and makes the sentence every crate
/// prints on a red the same sentence.
///
/// # What is checked, and why each direction is needed
///
/// `autotests = false` plus one `[[test]]` target means a `tests/*.rs`
/// file that no `#[path]` line mounts is not compiled and does not run
/// — indistinguishable, from outside, from a suite that passes.
///
/// 1. **Every suite file on disk is mounted.** That silent direction.
/// 2. **Every mount answers to a suite file**: one `#[path]` line per
///    file and no orphan declaration, computed rather than restated, so
///    no number about the set is written in prose.
/// 3. **No suite file declares a module of its own.** A `mod <name>;`
///    inside a SUITE loads that helper as a module of THAT suite, so
///    inside the one binary the helper is parsed, resolved,
///    type-checked and codegen'd once per declaring suite; declared
///    once at the root of `all.rs` and reached with
///    `use crate::<name>;` it is compiled once for the binary. Inline
///    `mod <name> { … }` blocks load no file and stay legal
///    ([`file_module_decls`]), and helper TREES are directories
///    carrying a `mod.rs`, which [`suite_files`] already excludes.
///
/// # The two inputs, and the one caller that supplies them
///
/// **There is exactly one caller**, and it is
/// [`crate::every_suite_file_is_aggregated!`]'s expansion, a hundred lines
/// below. Both arguments are shaped by where that expansion lands, so
/// what follows is a constraint on the macro rather than advice to an
/// author of an `all.rs`; there are no longer fifteen of those.
///
/// `tests_dir` is the crate's `tests/` directory: the walk and each
/// suite's text are read from it at RUN time, so the macro passes
/// [`crate_dir`]'s answer rather than a baked path — a nextest archive
/// replayed on another runner has no compile-time directory.
///
/// `all_rs` is the aggregator's OWN source, passed as
/// `include_str!("all.rs")` so the text judged is the one that was
/// compiled. It is read through [`code_and_literals`]: the needle is
/// the mount `#[path = "<suite>.rs"]`, whose payload is a string
/// literal, and a mount that has been commented out must not answer for
/// the file it names.
///
/// **Panics** when the walk finds nothing or a walked suite cannot be
/// read — a guard that cannot see the tree goes red rather than green
/// over an empty set ([`rust_sources`]).
///
/// All three checks run before anything is reported, so a red names
/// every violation found rather than the first. **The one exception is
/// an unreadable suite file**: check 3 reads each walked file, so a
/// file the walk saw and the reader cannot open panics HERE, with the
/// `expect` below, rather than through check 1's sentence. Loud either
/// way, and it is a broken checkout rather than a test outcome.
///
/// # How the call site is seen by the source-reader census
///
/// `crates/test-utils/tests/reader_census.rs` has to tell an `all.rs`
/// that READS Rust source from one that merely mounts modules, and the
/// tell moved when this function's call site did. While each
/// aggregator spelled the row out by hand it was arithmetic: every
/// `#[path = "<suite>.rs"]` contributes exactly one `.rs"` literal, and
/// the `include_str!("all.rs")` handed to this function was the single
/// extra one, so `named > mounted` held by a margin of exactly one —
/// which one `#[path = "` written inside a string literal in an
/// `all.rs` would have closed, silently, leaving that file's `Shared`
/// ledger line to red as `stale`. The row is now
/// [`crate::every_suite_file_is_aggregated!`], which carries those
/// tokens in its expansion rather than in the file's text: every
/// aggregator's margin is zero, and the census recognises the
/// INVOCATION instead.
///
/// Both are textual proxies for the same fact and both can be closed
/// without touching what they proxy; they differ in how easily. The
/// margin went to a stray `#[path = "` inside any string literal in the
/// file. The invocation needle goes to deleting the row, or to renaming
/// the macro — and the census holds BOTH of those: a needle that names
/// no macro reds on its own row rather than as fifteen stale ledger
/// lines. What it does not go to is re-delimiting:
/// `every_suite_file_is_aggregated! { }` compiles, runs and passes, and
/// the needle carries no delimiter for that reason.
#[must_use]
pub fn aggregation_violations(tests_dir: &std::path::Path, all_rs: &str) -> Vec<String> {
    let src = code_and_literals(all_rs);
    let found = suite_files(tests_dir);
    let mut violations = Vec::new();

    let missing: Vec<&String> = found
        .iter()
        .filter(|rel| !src.contains(&format!("#[path = \"{rel}\"]")))
        .collect();
    if !missing.is_empty() {
        violations.push(format!(
            "suites under tests/ are not declared in tests/all.rs, so `autotests = false` \
             is silently dropping them: {missing:?}. Add a `#[path]` line for each."
        ));
    }

    // The converse. The `format!` above spells its quote ESCAPED, so it
    // is not one of these matches.
    let declared = src.matches("#[path = \"").count();
    if declared != found.len() {
        violations.push(format!(
            "tests/all.rs declares {declared} suites but {} suite files exist under tests/",
            found.len()
        ));
    }

    let redeclared: Vec<String> = found
        .iter()
        .flat_map(|rel| {
            let text = std::fs::read_to_string(tests_dir.join(rel))
                .expect("a walked suite file reads back");
            file_module_decls(&text)
                .into_iter()
                .map(move |name| format!("{rel}: mod {name};"))
        })
        .collect();
    if !redeclared.is_empty() {
        violations.push(format!(
            "a suite declares a module of its own, which compiles that file once per \
             declaring suite inside this one binary: {redeclared:?}. Declare it once in \
             tests/all.rs (`mod <name>;`, no `#[path]`) and say `use crate::<name>;` here."
        ));
    }

    violations
}

/// **The aggregation row itself, in one spelling.** Emits the
/// `every_suite_file_is_aggregated` row that every aggregating crate's
/// `tests/all.rs` carries, whose body is [`aggregation_violations`]:
///
/// ```ignore
/// test_utils::every_suite_file_is_aggregated!();
/// ```
///
/// It takes no arguments. Both of the row's crate-specific inputs are
/// supplied by the compiler at the site where the tokens land, so there
/// is nothing for a caller to name — and a list of crate names or paths
/// passed in here would be the hand-kept copy this macro exists to end,
/// wearing one invocation instead of fifteen.
///
/// # Why a macro, and not a function in this module
///
/// [`crate_dir`] already takes `CARGO_MANIFEST_DIR` as an argument for a
/// related reason, and the same force applies one level up.
/// `env!("CARGO_MANIFEST_DIR")` reads the environment of the crate being
/// compiled, and `include_str!("all.rs")` resolves relative to the FILE
/// holding the token. Written as a function here, both would answer for
/// `test-utils`: every crate's row would walk `test-utils`' own `tests/`
/// and judge `test-utils`' own aggregator, and fifteen rows would pass
/// by checking one crate. That is a guard green because it cannot see
/// its subject, which is the exact failure this row exists to make
/// impossible. A macro puts the two tokens in the INVOKING file, where
/// they expand against the invoking crate — measured on a two-crate
/// probe before this macro was written, for the bare and the
/// `::core::`-qualified spelling both.
///
/// **That measurement has no mechanical guard and cannot have one, and
/// here is the reason.** If invocation-site resolution ever flipped,
/// `crate_dir(env!(…))` and `include_str!("all.rs")` would BOTH answer
/// for `test-utils`, so every row would compare `test-utils`' own
/// aggregator against `test-utils`' own `tests/` — self-consistent, and
/// green in every crate. Nothing in this tree would red, which is the
/// same failure named three paragraphs above. No guard can be written
/// for it either: a guard sited in some crate would have to compare what
/// the macro sees against that crate's directory, and that comparison is
/// what the macro already is, so it would flip with it. The reason it is
/// safe to leave unguarded is not that the failure would be loud: it is
/// that `include_str!` and `env!` resolving at the invocation site is
/// part of Rust's stability promise, so the flip is a breaking language
/// change and not a regression this repository can land. A scheduled
/// re-measure would buy nothing that `rustc`'s own release process does
/// not, which is why there is not one.
///
/// The generated `fn` keeps the name `every_suite_file_is_aggregated`,
/// so the row's identity in a PASS list, in `--filter` arguments and in
/// the prose that names it across the tree is unchanged.
///
/// # What it does NOT enforce
///
/// **Nothing here checks that a crate HAS this row.** A crate that
/// aggregates suites and never invokes the macro is silent — exactly as
/// a crate that never pasted the five lines was silent before. This
/// spelling neither closes that hole nor widens it, and a sixteenth
/// crate added tomorrow with no invocation goes unremarked.
///
/// What covers the neighbouring ground, and does not cover this:
/// `scripts/gates/test-aggregation.sh` holds every member to at most one
/// `[[test]]` target, which is the opt-in itself rather than the row;
/// and `crates/bvh/tests/aggregator_headers.rs`'s
/// `a_non_aggregating_tests_directory_holds_one_suite_file` holds a
/// crate that mounts NOTHING to a single suite file. Neither says that a
/// crate which mounts suites carries the row.
///
/// `crates/pncad/` is the deliberate non-carrier and must stay one: its
/// `tests/` holds a single file, its header says why it has no row, and
/// the guard named above is what keeps that sentence true.
///
/// # What the collapse cost, which is redundancy
///
/// Fifteen copies were fifteen INDEPENDENTLY CHECKED facts.
/// `crates/test-utils/tests/reader_census.rs` reads each `all.rs` and
/// asserts a `Shared` ledger line against that file's own text, so a
/// reversion in one of them red'd one file and named it. One macro is
/// one fact: this body answers for fifteen call sites, and the file it
/// lives in is dispositioned `Home`, which that row filters out before
/// it looks at anything. Rewriting this expansion into a hand-rolled
/// `read_dir` walk therefore leaves all fifteen `Shared` lines green.
///
/// `the_aggregation_row_macro_reaches_the_shared_lexer` is the row that
/// closes it, by reading THIS body as text and asserting it still calls
/// [`crate_dir`] and [`aggregation_violations`] — measured: that
/// reversion reds it and nothing else. So the claim is checked, but it
/// is checked ONCE. A single check with a single subject is a weaker
/// thing than fifteen checks with fifteen subjects, and that difference
/// is what collapsing fifteen copies to one is bought with. It is worth
/// it here because the fifteen were byte-identical and their drift was
/// the defect; it would not be worth it where the copies differed.
#[macro_export]
macro_rules! every_suite_file_is_aggregated {
    () => {
        /// The aggregation and ONE HOME checks, whose one home — the walk, the
        /// three checks and the argument for each — is
        /// `test_utils::source::aggregation_violations`.
        #[test]
        fn every_suite_file_is_aggregated() {
            let tests = $crate::source::crate_dir(::core::env!("CARGO_MANIFEST_DIR")).join("tests");
            let violations =
                $crate::source::aggregation_violations(&tests, ::core::include_str!("all.rs"));
            assert!(violations.is_empty(), "{}", violations.join("\n"));
        }
    };
}

// ------------------------------------------------------------------
// The predicate-name census: every `decide*` call site a crate's `src`
// spells, and every one of them this reader could not read.
// ------------------------------------------------------------------

/// How a predicate name that is NOT written at its `decide*` call
/// reaches the funnel.
///
/// A census declares one of these per carrier it knows about; anything
/// it has not declared comes back as an indirect or unreadable site
/// rather than being dropped.
#[derive(Clone, Copy, Debug)]
pub enum NameCarrier {
    /// A call or struct literal: the names are the plain string
    /// literals among the DIRECT arguments (or field values) of the
    /// bracket group the token opens. A literal nested deeper — inside
    /// a closure body, inside another call — is not an argument of this
    /// one and is not read as a name.
    Call(&'static str),
    /// A `const NAME: &str = "…";` whose value is the name.
    Const(&'static str),
}

/// What [`predicate_census`] found.
///
/// The last three fields are the point: a reader that silently skipped
/// what it could not parse would report a complete-looking `names` over
/// a crate it had not read, and a census green on a gate it never saw
/// is worse than no census.
#[derive(Debug, Default)]
pub struct PredicateCensus {
    /// The names, from a plain literal at the call or out of a declared
    /// carrier.
    pub names: std::collections::BTreeSet<String>,
    /// `<file>: <expression>` per call whose name-bearing argument is
    /// not a plain literal, and per declared carrier call that holds no
    /// literal argument. Each is a site whose carrier the caller must
    /// declare — declaring it makes that carrier's own call sites part
    /// of the census, so the chain terminates at literals or stays red.
    pub indirect: std::collections::BTreeSet<String>,
    /// `<file>: <text>` per `decide*` occurrence this reader cannot
    /// even find an argument list for — a spelling it does not know.
    pub unreadable: std::collections::BTreeSet<String>,
    /// `<file>:<line>` per `include!` or `#[path]` in the tree walked.
    /// **This reader walks `<src>/**/*.rs` and nothing else**, so a
    /// gate in a file pulled in by either is outside what it read; the
    /// blind spot is stated here so a caller can red on it rather than
    /// inherit it.
    pub unwalked: std::collections::BTreeSet<String>,
}

/// Every predicate name decided under `src`, and every site the reader
/// could not read.
///
/// The funnel is `geom_core::k_stats::decide` and the per-crate
/// wrappers that forward to it, so the scan is for a `decide` token —
/// with or without a suffix (`_flagged`, `_invariant`) and with or
/// without a turbofish — followed by an argument list whose first
/// argument is the name.
///
/// Three hand-rolled copies of this walk stood in three suites before
/// it existed, and two of them were defeated by mutation with ordinary
/// spellings: a turbofish, and a one-line wrapper around a declared
/// carrier. Both are read here; anything still unread is reported, not
/// dropped.
#[must_use]
pub fn predicate_census(src: &std::path::Path, carriers: &[NameCarrier]) -> PredicateCensus {
    let mut out = PredicateCensus::default();
    for path in rust_sources(src) {
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("a readable source file {}: {e}", path.display()));
        // Comments are blanked in place, so prose that spells `decide (`
        // is not a call site and the offsets still index the file byte
        // for byte.
        let code = code_and_literals(&text);
        // Tokens are LOCATED in the code-only view, where a literal's
        // bytes are blanked, so prose inside a string that spells
        // `decide(` is not a call site either; they are READ from the
        // view that keeps literals. Both are blanked in place, so one
        // set of offsets indexes both.
        let bare = code_only(&text);
        let rel = path
            .strip_prefix(src)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        for (kind, payload) in funnel_sites(&bare, &code) {
            match kind {
                SiteKind::Name => {
                    out.names.insert(payload);
                }
                SiteKind::Indirect => {
                    out.indirect.insert(format!("{rel}: {payload}"));
                }
                SiteKind::Unreadable => {
                    out.unreadable.insert(format!("{rel}: {payload}"));
                }
            }
        }
        for carrier in carriers {
            match *carrier {
                NameCarrier::Call(token) => {
                    for open in carrier_groups(&bare, token) {
                        let Some(end) = balanced_end(&bare, open) else {
                            out.unreadable.insert(format!("{rel}: {token}(… unclosed"));
                            continue;
                        };
                        let literals = direct_literals(&code[open + 1..end]);
                        if literals.is_empty() {
                            out.indirect
                                .insert(format!("{rel}: {}", one_line(&code[open..=end])));
                        } else {
                            out.names.extend(literals);
                        }
                    }
                }
                NameCarrier::Const(token) => {
                    if let Some(decl) = find_word(&bare, &format!("const {token}")) {
                        match const_literal(&code[decl..]) {
                            Some(name) => {
                                out.names.insert(name);
                            }
                            None => {
                                out.unreadable.insert(format!("{rel}: const {token}"));
                            }
                        }
                    }
                }
            }
        }
        for (needle, raw) in [("include!", &bare), ("#[path", &text)] {
            let mut at = 0usize;
            while let Some(hit) = raw[at..].find(needle) {
                let start = at + hit;
                at = start + needle.len();
                out.unwalked
                    .insert(format!("{rel}:{}", line(raw, start) + 1));
            }
        }
    }
    out
}

/// What a `decide*` occurrence turned out to be.
enum SiteKind {
    /// A plain string literal at the call: the payload is the name.
    Name,
    /// A first argument that is not a plain literal: the payload is the
    /// expression.
    Indirect,
    /// A spelling with no argument list this reader can find: the
    /// payload is the text it stopped on.
    Unreadable,
}

/// Every `decide*` occurrence in a blanked view, classified.
fn funnel_sites(bare: &str, code: &str) -> Vec<(SiteKind, String)> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(hit) = bare[at..].find("decide") {
        let start = at + hit;
        at = start + "decide".len();
        if !boundary_before(bare, start) {
            continue;
        }
        // `fn decide<T: Decide>(…)` declares the door; it is not a
        // call of it, and the crate wrapper's own body holds the call.
        let before = bare[..start].trim_end();
        if before.ends_with("fn") && boundary_before(before, before.len() - 2) {
            continue;
        }
        // The suffixed spellings of the same door (`_flagged`,
        // `_invariant`) are part of the token; a BARE alphanumeric
        // continuation is a different identifier (`decided`), not a
        // suffix, and this is not its call site.
        let mut cursor = at;
        if bare[cursor..].starts_with('_') {
            while bare[cursor..]
                .chars()
                .next()
                .is_some_and(|c| c == '_' || c.is_ascii_alphanumeric())
            {
                cursor += 1;
            }
        } else if bare[cursor..]
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphanumeric())
        {
            continue;
        }
        let rest = bare[cursor..].trim_start();
        cursor = bare.len() - rest.len();
        // A turbofish is part of the call, not a different spelling of
        // the name: read through it.
        if rest.starts_with("::<") {
            match angle_end(bare, cursor + 2) {
                Some(close) => {
                    let tail = bare[close + 1..].trim_start();
                    cursor = bare.len() - tail.len();
                }
                None => {
                    out.push((SiteKind::Unreadable, one_line(&code[start..cursor + 3])));
                    continue;
                }
            }
        }
        match bare[cursor..].chars().next() {
            // A path or a value, not a call: `use …::decide;`,
            // `decide::inner`, `decide,` in an import list.
            Some(';' | ',' | ')' | '}' | '>' | '.' | ':') | None => continue,
            Some('(') => {}
            // Something between the token and its arguments that this
            // reader does not know. Loud, because a spelling it cannot
            // parse is a gate it cannot see.
            Some(c) => {
                out.push((
                    SiteKind::Unreadable,
                    one_line(&code[start..cursor + c.len_utf8()]),
                ));
                continue;
            }
        }
        let open = cursor;
        at = open + 1;
        let arg = code[open + 1..].trim_start();
        match first_literal(arg) {
            Some(name) => out.push((SiteKind::Name, name)),
            None => {
                let expr: String = arg
                    .chars()
                    .take_while(|c| *c != ',' && *c != ')')
                    .collect::<String>()
                    .trim()
                    .to_string();
                out.push((SiteKind::Indirect, expr));
            }
        }
    }
    out
}

/// The contents of a plain string literal at the head of `text`.
fn first_literal(text: &str) -> Option<String> {
    let rest = text.strip_prefix('"')?;
    let end = rest.find('"')?;
    plain_string_literal(&text[..=end + 1]).map(str::to_string)
}

/// The byte offsets of the `(` or `{` each real occurrence of `token`
/// opens — real meaning a call or a struct literal, not the item that
/// declares the name.
fn carrier_groups(code: &str, token: &str) -> Vec<usize> {
    const DECLARERS: [&str; 7] = ["fn", "struct", "enum", "impl", "use", "trait", "type"];
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(hit) = code[at..].find(token) {
        let start = at + hit;
        at = start + token.len();
        if !boundary_before(code, start) || !boundary_after(code, at) {
            continue;
        }
        let before = code[..start].trim_end();
        if DECLARERS
            .iter()
            .any(|kw| before.ends_with(kw) && boundary_before(before, before.len() - kw.len()))
        {
            continue;
        }
        let rest = code[at..].trim_start();
        let skipped = code[at..].len() - rest.len();
        if rest.starts_with('(') || rest.starts_with('{') {
            out.push(at + skipped);
        }
    }
    out
}

/// The plain string literals among a group's DIRECT arguments.
fn direct_literals(inner: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let bytes = inner.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        match bytes[i] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth = depth.saturating_sub(1),
            b'"' if depth == 0 => {
                if let Some(name) = first_literal(&inner[i..]) {
                    i += name.len() + 2;
                    out.push(name);
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }
    out
}

/// The first plain literal of a `const NAME: &str = "…";` declaration
/// whose head starts at the front of `text`.
fn const_literal(text: &str) -> Option<String> {
    let end = text.find(';')?;
    let open = text[..end].find('"')?;
    first_literal(&text[open..])
}

/// The offset of `needle` as a whole word, if the view holds one.
fn find_word(code: &str, needle: &str) -> Option<usize> {
    let mut at = 0usize;
    while let Some(hit) = code[at..].find(needle) {
        let start = at + hit;
        at = start + needle.len();
        if boundary_before(code, start) {
            return Some(start);
        }
    }
    None
}

/// `text` with its whitespace collapsed, for an assertion message.
fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    // These rows touch no process-global state and are safe to run in
    // parallel with anything. `vacuity`'s do not — its `caught` helper
    // swaps the process-wide panic hook from parallel test threads and
    // flakes about one run in thirteen (15/200; **issue #882**). Do not
    // copy that shape here.
    use super::{
        ItemBody, Region, aggregation_violations, angle_end, balanced_end, boundary_after,
        boundary_before, code_and_literals, code_only, comments_only, file_module_decls, item_body,
        keeping, top_level_split, where_at,
    };

    /// The keyword, not the letters: an identifier that contains
    /// `where` at either end is stepped over, and the answer is the
    /// first `where` that is a word.
    #[test]
    fn a_where_clause_starts_at_the_keyword() {
        let head = "Lane<Somewhere, where_ok>\nwhere\n    T: X";
        assert_eq!(where_at(head), Some(head.find("\nwhere").unwrap() + 1));
        assert_eq!(where_at("Lane<Somewhere, where_ok>"), None);
        assert_eq!(where_at("Self where T: X"), Some("Self ".len()));
    }

    /// A whole-word match needs BOTH boundaries, and each half refuses
    /// a different way of being inside a longer name.
    #[test]
    fn a_whole_word_match_needs_both_boundaries() {
        let code = "wrong_operand operand operand_kind";
        let first = code.find("operand").unwrap();
        // Inside `wrong_operand`: the leading half refuses it, the
        // trailing half cannot see anything wrong.
        assert!(!boundary_before(code, first));
        assert!(boundary_after(code, first + "operand".len()));

        let bare = code[first + 1..].find("operand").unwrap() + first + 1;
        assert!(boundary_before(code, bare));
        assert!(boundary_after(code, bare + "operand".len()));

        // A PREFIX of a longer name: now the halves swap roles, which
        // is why a guard that asks only the leading one matches here.
        let prefix = code.rfind("operand").unwrap();
        assert!(boundary_before(code, prefix));
        assert!(!boundary_after(code, prefix + "operand".len()));

        // Either end of the text is a boundary: there is no byte there
        // to disqualify the match.
        assert!(boundary_before(code, 0));
        assert!(boundary_after(code, code.len()));
    }

    /// A generic list closes at its own `>`, and the constructs that
    /// carry a `>` or a `;` without ending it do not close it early.
    ///
    /// **Each row is planted against a specific way to get this
    /// wrong**, so a repair reverted anywhere here reds on its own
    /// evidence rather than on whichever door happens to be spelled
    /// with the construct that day.
    #[test]
    fn a_generic_list_closes_at_its_own_angle_bracket() {
        // The plain case: the answer is the closing `>`.
        let plain = "fn f<T>(x: T) {}";
        let open = plain.find('<').unwrap();
        assert_eq!(angle_end(plain, open), Some(plain.find('>').unwrap()));

        // A `;` INSIDE a square bracket is an array type, not the
        // item's body. Counting it as a terminator is the defect this
        // row exists for.
        let array = "fn f(p: impl IntoIterator<Item = [Expr; 2]>) -> Self {}";
        let open = array.find('<').unwrap();
        let end = angle_end(array, open).expect("an array type does not end the list");
        assert_eq!(&array[end..=end], ">");
        assert_eq!(end, array.find("]>").unwrap() + 1);

        // An arrow's `>` is not a closer: reading it as one answers a
        // list that is too SHORT, which is the silent direction.
        let arrow = "fn f<F: Fn(u32) -> u32>(g: F) {}";
        let open = arrow.find('<').unwrap();
        let end = angle_end(arrow, open).expect("an arrow does not end the list");
        assert_eq!(end, arrow.find(">(").unwrap());

        // Nesting still closes at the OUTER list.
        let nested = "fn f<T: Into<Vec<u8>>>(x: T) {}";
        let open = nested.find('<').unwrap();
        let end = angle_end(nested, open).expect("a nested list closes");
        assert_eq!(end, nested.find(">>>").unwrap() + 2);

        // A `;` at bracket depth zero IS the item's body: a list that
        // never closes answers `None` rather than running on.
        assert_eq!(angle_end("type A = B<C;", "type A = B".len()), None);
        // As does one that simply runs out of input.
        assert_eq!(angle_end("fn f<T", 4), None);
    }

    /// Every construct a needle can hide in, and the code read each one
    /// must not cost. Written once and asserted from three directions
    /// below, so a construct added here is exercised in all of them.
    const HIDING_PLACES: [&str; 12] = [
        "let x = 1; // eps",
        "/// eps",
        "//! eps",
        "/* eps */",
        "/* outer /* eps */ inner */",
        "let s = \"eps\";",
        "let s = \"he said \\\"eps\\\"\";",
        "#[doc = \"eps\"]",
        "panic!(\"over eps {eps_x}\");",
        "let s = r\"eps\";",
        "let s = br#\"eps\"#;",
        "let s = cr\"eps\";",
    ];

    /// Genuine code reads of `eps`, which no view of the code may lose.
    ///
    /// Rows 7-12 are the shapes that erase the REST OF THE FILE when
    /// the reader gets them wrong, which is why each is a code read
    /// after the construct rather than the construct alone: code
    /// following a blanked region on the same line (a line-prefix
    /// comment test cannot see it at all), and the four quote-shaped
    /// literals a naive scanner mis-closes — a double-quote char, an
    /// escaped-quote char, an escaped-backslash char, and a string
    /// whose body spells a comment delimiter — plus a multi-byte
    /// literal body, where a blanker stepping bytes rather than
    /// characters splits one.
    const CODE_READS: [&str; 12] = [
        "gap * lever < eps",
        "let eps = tol.eps();",
        "f(a, b, eps)",
        "struct T<'a> { eps: &'a f64 }",
        "let c = 'e'; let d = eps;",
        "let c = b'e'; let d = eps;",
        "/* a block */ let d = eps;",
        "if line.contains('\"') { let d = eps; }",
        "let q = '\\''; let d = eps;",
        "let q = '\\\\'; let d = eps;",
        "let s = \"a // b /* c\"; let d = eps;",
        "let s = \"π…\"; let d = eps;",
    ];

    /// S13's ratified shape for a text-matching guard: **a clean
    /// fixture must pass and every planted violation must fire.** Here
    /// the "violation" is a needle that survives blanking when it
    /// should not, or is lost when it should not be.
    #[test]
    fn every_hiding_place_is_blanked_and_no_code_read_is_lost() {
        for row in HIDING_PLACES {
            assert!(
                row.contains("eps"),
                "a row cannot hide what it lacks: {row}"
            );
            assert_eq!(
                code_only(row).matches("eps").count(),
                0,
                "survived blanking: {row}"
            );
        }
        for row in CODE_READS {
            assert_eq!(
                code_only(row).matches("eps").count(),
                row.matches("eps").count(),
                "lost a code read: {row}"
            );
        }
    }

    /// **The needle that IS a string literal.** `code_and_literals`
    /// blanks the comment and keeps the literal, so a guard reading for
    /// `#[path = "…"]` sees the live mount and not the commented-out
    /// one. Under [`code_only`] the same guard would be vacuous, which
    /// is why the two views both exist.
    #[test]
    fn the_literal_view_keeps_the_needle_and_still_drops_the_comment() {
        let live = "#[path = \"e4_dual_door.rs\"]\nmod e4;";
        let dead = "// #[path = \"e4_dual_door.rs\"]\n";
        assert!(code_and_literals(live).contains("#[path = \"e4_dual_door.rs\"]"));
        assert!(!code_and_literals(dead).contains("#[path = \"e4_dual_door.rs\"]"));
        assert!(
            !code_only(live).contains("e4_dual_door"),
            "code view blanks it"
        );
        // And a literal is a literal wherever it sits: a needle inside
        // a doc comment is prose, not a site.
        assert!(!code_and_literals("/// #[path = \"x.rs\"]").contains("x.rs"));
    }

    /// **The inverse view.** A guard whose subject is a doc-comment
    /// ledger must not be satisfiable by code, and must still see the
    /// prose that `code_only` would erase.
    #[test]
    fn the_prose_view_reads_docs_and_refuses_to_be_satisfied_by_code() {
        assert!(comments_only("/// Version 7 is a break.").contains("Version 7 is"));
        assert!(!comments_only("f(format!(\"Version 7 is\"));").contains("Version 7 is"));
        assert!(!comments_only("let version_7_is = 1;").contains("version_7_is"));
    }

    /// **The three views partition the file**, byte for byte: every
    /// byte is code, comment or literal and never two of them, and
    /// keeping all three returns the input unchanged. This is what
    /// makes a fourth view a SELECTION rather than a second lexer —
    /// there is nothing outside the three to build one from.
    #[test]
    fn the_three_views_partition_every_byte_of_a_file() {
        let text = include_str!("source.rs");
        assert_eq!(
            keeping(text, &[Region::Code, Region::Comment, Region::Literal]),
            text,
            "keeping every region is the identity"
        );
        let views = [
            code_only(text),
            comments_only(text),
            keeping(text, &[Region::Literal]),
        ];
        for v in &views {
            assert_eq!(v.len(), text.len(), "byte offsets must survive");
        }
        for (i, &c) in text.as_bytes().iter().enumerate() {
            let kept = views.iter().filter(|v| v.as_bytes()[i] == c).count();
            if c == b' ' || c == b'\n' {
                continue; // Indistinguishable from its own blanking.
            }
            assert_eq!(kept, 1, "byte {i} ({:?}) is in {kept} regions", c as char);
        }
    }

    #[test]
    fn line_structure_survives_so_a_caller_can_report_a_line_number() {
        let multi = "a\n// eps\nb\n";
        for view in [
            code_only(multi),
            comments_only(multi),
            code_and_literals(multi),
        ] {
            assert_eq!(view.lines().count(), multi.lines().count());
        }
        assert_eq!(code_only(multi).lines().nth(1).unwrap().trim(), "");
        assert_eq!(comments_only(multi).lines().next().unwrap().trim(), "");
    }

    #[test]
    fn a_lifetime_is_not_an_opening_quote() {
        // If `'a` opened a literal, everything after it would blank and
        // the trailing read would be lost.
        let row = "fn f<'a>(x: &'a f64, eps: f64) -> bool { x < &eps }";
        assert_eq!(code_only(row).matches("eps").count(), 2, "{row}");
    }

    /// **All four raw-string prefixes, and the byte one is the point.**
    /// `br"x\"` is a CLOSED raw string whose body ends in a backslash;
    /// read through the escape-honouring plain-string arm it is an
    /// unclosed literal that blanks the rest of the file, losing every
    /// code read after it. That exact spelling is the one this tree got
    /// wrong three times before the lexer was shared.
    #[test]
    fn a_raw_string_closes_at_its_own_delimiter_and_loses_no_following_code() {
        let row = "let s = br\"a\\\"; let y = eps;";
        assert_eq!(
            code_only(row).matches("eps").count(),
            1,
            "the read after a `br\"x\\\"` must survive: {row}"
        );
        // Hashes, and a quote inside the body that does not close it.
        let hashed = "let s = r#\"a \"quoted\" b\"#; let y = eps;";
        assert_eq!(code_only(hashed).matches("eps").count(), 1);
        assert!(!code_only(hashed).contains("quoted"), "body is a literal");
        // An identifier ENDING in b/c/r before a quote is not a prefix.
        assert!(
            code_only("let ab = ar\"x\";").contains("ar"),
            "not a raw string"
        );
    }

    #[test]
    fn the_walk_is_recursive_and_refuses_to_be_empty() {
        let dir = std::env::temp_dir().join(format!("tu-src-{}", std::process::id()));
        let sub = dir.join("deep").join("deeper");
        std::fs::create_dir_all(&sub).expect("temp dirs");
        std::fs::write(dir.join("top.rs"), "// eps\n").expect("write");
        std::fs::write(sub.join("hidden.rs"), "let eps = 1;\n").expect("write");
        std::fs::write(dir.join("notrust.txt"), "eps").expect("write");
        let found = super::rust_sources(&dir);
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(found.iter().any(|p| p.ends_with("hidden.rs")), "{found:?}");
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }

    /// A required needle answers its offsets, and one that matches
    /// nothing refuses naming the text and the needle — the empty set
    /// is never handed back.
    #[test]
    fn a_required_needle_that_matches_nothing_refuses_at_the_scan() {
        assert_eq!(super::required_matches("a b a", "t", "a"), vec![0, 4]);
        let said = crate::panic_capture::caught(|| {
            let _ = super::required_matches("a b", "the.rs", "zz");
        })
        .expect("an absent required needle refuses");
        assert!(said.contains("the.rs") && said.contains("`zz`"), "{said}");
    }

    /// **The precondition is the operation.** Over a blanked view every
    /// bracket is a real bracket, so depth counting is the whole parse
    /// — including when a literal holds an unbalanced one, which is the
    /// case a call-site copy gets wrong.
    #[test]
    fn the_balanced_region_is_carved_over_the_blanked_view() {
        let raw = "f(a, g(b), \"(((\", [c]) tail";
        let code = code_only(raw);
        let open = code.find('(').expect("an opener");
        let end = balanced_end(&code, open).expect("it closes");
        assert_eq!(&raw[open..=end], "(a, g(b), \"(((\", [c])", "{code}");
        // Raw text is the direction this exists to stop: the literal's
        // three unbalanced parens run the scan off the end.
        assert_eq!(balanced_end(raw, open), None, "raw source must not be used");
        assert_eq!(balanced_end(&code_only("f(a"), 1), None, "never closes");
    }

    /// **The item carve, and the three answers it has to keep apart.**
    /// Each row is a way the hand-rolled spellings of this get it
    /// wrong: a `;` inside a bracket read as a terminator (which drops
    /// the whole item), a `}` inside a literal read as the close
    /// (which drops the tail of the body), and a truncated head read
    /// as a declaration (which drops the item silently, where saying
    /// so lets the caller be loud).
    #[test]
    fn an_item_body_is_carved_from_its_head_and_a_declaration_is_not_one() {
        let plain = code_only("pub(crate) fn f(&mut self) { g(); } fn after() {}");
        let ItemBody::Body(body) = item_body(&plain, 0) else {
            panic!("a body: {:?}", item_body(&plain, 0))
        };
        assert_eq!(&plain[body], "{ g(); }", "braces included, and it stops");

        // The `;` that ends no item. Read as a terminator, every
        // `-> [T; N]` signature in the tree is dropped whole.
        let array = code_only("pub fn f() -> [f64; 3] { g(); }");
        let ItemBody::Body(body) = item_body(&array, 0) else {
            panic!("an array return type is not a declaration")
        };
        assert_eq!(&array[body], "{ g(); }");
        let nested =
            code_only("pub fn f(p: impl Into<[u8; 2]>) -> Result<([u8; 2], u8), E> { g(); }");
        assert!(
            matches!(item_body(&nested, 0), ItemBody::Body(_)),
            "{nested}"
        );

        // The `}` that closes nothing. This is the precondition
        // [`balanced_end`] states, one level up: over raw text the
        // literal's brace ends the body and `after()` is lost.
        let raw = "fn f() { let s = \"}\"; after(); }";
        let code = code_only(raw);
        let ItemBody::Body(body) = item_body(&code, 0) else {
            panic!("a body")
        };
        assert!(&raw[body].contains("after()"), "the carve stopped early");

        // A head with no body, and the offset is the `;` so a walk can
        // resume past it rather than re-scanning.
        let declared = code_only("pub fn f(&self) -> u8; pub fn g() {}");
        let ItemBody::Declaration(semi) = item_body(&declared, 0) else {
            panic!("a declaration")
        };
        assert_eq!(&declared[semi..=semi], ";");
        assert!(matches!(item_body(&declared, semi + 1), ItemBody::Body(_)));

        // Neither terminator, and neither answer. A truncated text is
        // not a declaration, and a caller that must not skip an item
        // needs to be able to tell.
        assert_eq!(
            item_body(&code_only("pub fn f(&self)"), 0),
            ItemBody::Unterminated
        );
        assert_eq!(
            item_body(&code_only("fn f() { g();"), 0),
            ItemBody::Unterminated
        );
    }

    /// Splitting an argument list: a comma inside a nested bracket, a
    /// generic argument or a blanked literal is not a separator.
    #[test]
    fn a_top_level_split_ignores_nested_and_generic_commas() {
        let raw = "name, Probe<A, B>, f(x, y), \"a,b\"";
        let code = code_only(raw);
        let parts: Vec<&str> = top_level_split(&code, ',')
            .into_iter()
            .map(|r| raw[r].trim())
            .collect();
        assert_eq!(
            parts,
            ["name", "Probe<A, B>", "f(x, y)", "\"a,b\""],
            "{code}"
        );
    }

    /// A suite in a group directory is a suite; a file under a `mod.rs`
    /// directory is a helper, reached through `mod <name>;`. Twelve of
    /// the thirteen mount guards could see neither.
    #[test]
    fn the_suite_walk_recurses_and_skips_module_directories() {
        let root = std::env::temp_dir().join(format!("tu-suites-{}", std::process::id()));
        let group = root.join("curves");
        let module = root.join("common");
        std::fs::create_dir_all(&group).expect("dirs");
        std::fs::create_dir_all(module.join("deep")).expect("dirs");
        for (path, text) in [
            (root.join("all.rs"), "// aggregator"),
            (root.join("flat.rs"), "// suite"),
            (group.join("boxes.rs"), "// suite in a group"),
            (module.join("mod.rs"), "// shared helper"),
            (module.join("deep").join("more.rs"), "// under the helper"),
        ] {
            std::fs::write(path, text).expect("write");
        }
        let mut found = super::suite_files(&root);
        found.sort();
        assert_eq!(found, ["curves/boxes.rs", "flat.rs"], "{found:?}");
        std::fs::remove_dir_all(&root).expect("cleanup");
    }

    #[test]
    fn a_crate_dir_is_the_one_holding_a_manifest() {
        let here = super::crate_dir(env!("CARGO_MANIFEST_DIR"));
        assert!(here.join("Cargo.toml").is_file(), "{here:?}");
        assert!(here.ends_with("test-utils"), "{here:?}");
    }

    /// **A literal is blanked PREFIX AND ALL**, which is
    /// [`Region::Literal`]'s stated contract and not merely its
    /// partition. `b'x'` is the construct that had no row: the
    /// partition holds either way — the `b` is *some* region — so
    /// neither the partition test nor 2,125 externally generated
    /// snippets could tell that the prefix was surviving as code.
    #[test]
    fn every_literal_prefix_is_blanked_with_its_literal() {
        for literal in [
            "b'x'",
            "b\"x\"",
            "c\"x\"",
            "br\"x\"",
            "cr#\"x\"#",
            "'x'",
            "b'\\''",
        ] {
            let row = format!("let z = {literal};");
            let want = format!("let z = {};", " ".repeat(literal.len()));
            assert_eq!(code_only(&row), want, "{row}");
        }
        // A lifetime after `b` is not a byte-char literal: it never
        // closes, so the `b` stays code and so does the read after it.
        let row = "fn f<'a>(b: &'a f64) -> f64 { *b }";
        assert_eq!(code_only(row).matches('b').count(), 2, "{row}");
    }

    /// A doc ATTRIBUTE is a literal, not a comment — `rustc` agrees,
    /// and a guard reading a ledger through [`comments_only`] must
    /// find nothing rather than half of it.
    #[test]
    fn a_doc_attribute_is_a_literal_and_not_prose() {
        let row = "#[doc = \"Version 7 is a break.\"]\nstruct S;";
        assert!(!comments_only(row).contains("Version 7 is"), "not prose");
        assert!(code_and_literals(row).contains("Version 7 is"), "a literal");
        assert!(!code_only(row).contains("Version 7 is"), "blanked as one");
        // The `///` spelling, which IS prose, for contrast.
        assert!(comments_only("/// Version 7 is a break.").contains("Version 7 is"));
    }

    /// The selftest for the ONE HOME guard the aggregated crates carry:
    /// the FILE form of a module declaration is reported, and every
    /// shape that is not one is not — an inline block, a hiding place,
    /// and an identifier that merely starts or ends with `mod`.
    #[test]
    fn file_module_decls_reports_the_file_form_and_nothing_else() {
        let src = "\
mod common;
pub mod helper ;
pub(crate) mod nested;
mod interval { fn f() {} }
// mod commented_out;
/// mod in_a_doc_comment;
let s = \"mod in_a_literal;\";
let m = model;
let x = xmod;
";
        assert_eq!(
            file_module_decls(src),
            vec![
                "common".to_string(),
                "helper".to_string(),
                "nested".to_string()
            ]
        );
        // Repeats are sites, not a set — the caller reports each one.
        assert_eq!(file_module_decls("mod a;\nmod a;").len(), 2);
    }

    /// A throwaway `tests/`-shaped tree under the system temp dir.
    /// nextest runs one process per test, so the pid names it uniquely.
    fn plant(files: &[(&str, &str)]) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("cad-aggregation-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for (rel, body) in files {
            let path = dir.join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, body).unwrap();
        }
        dir
    }

    /// S13's ratified shape, for the row fourteen aggregating crates
    /// now share: **a clean tree passes and every planted violation
    /// fires**, each with the message that names its own fix.
    ///
    /// The plantings are the four ways the pairing between a `tests/`
    /// directory and its `all.rs` can break, and the fourth is the one
    /// a naive scan gets wrong: a mount COMMENTED OUT aggregates
    /// nothing, so it must not answer for the file it names.
    #[test]
    fn the_aggregation_guard_passes_a_clean_tree_and_fires_on_each_planting() {
        let suite = "#[test]\nfn a() {}\n";
        let dir = plant(&[("one.rs", suite), ("group/two.rs", suite)]);
        let clean = "#[path = \"one.rs\"]\nmod one;\n#[path = \"group/two.rs\"]\nmod two;\n";
        assert_eq!(aggregation_violations(&dir, clean), Vec::<String>::new());

        let fires = |all: &str, needles: &[&str]| {
            let found = aggregation_violations(&dir, all);
            for needle in needles {
                assert!(
                    found.iter().any(|m| m.contains(needle)),
                    "planted violation did not fire on {needle:?}: {found:#?}"
                );
            }
        };

        // 1. A suite on disk that no mount names — the silent direction,
        //    reported together with the count it moves.
        fires(
            "#[path = \"one.rs\"]\nmod one;\n",
            &[
                "silently dropping them: [\"group/two.rs\"]",
                "declares 1 suites but 2 suite files",
            ],
        );

        // 2. A mount answering to no file: nothing is missing, and the
        //    count is what says so.
        fires(
            &format!("{clean}#[path = \"three.rs\"]\nmod three;\n"),
            &["declares 3 suites but 2 suite files"],
        );

        // 3. A mount that has been commented out aggregates nothing.
        fires(
            "// #[path = \"one.rs\"]\nmod one;\n#[path = \"group/two.rs\"]\nmod two;\n",
            &["silently dropping them: [\"one.rs\"]"],
        );

        // 4. A suite declaring a module of its own — the ONE HOME half.
        //    The inline block beside it is legal and must not fire.
        std::fs::write(
            dir.join("one.rs"),
            "mod helper;\nmod inline { fn f() {} }\n",
        )
        .unwrap();
        fires(clean, &["one.rs: mod helper;"]);
        assert!(
            !aggregation_violations(&dir, clean)
                .iter()
                .any(|m| m.contains("mod inline;")),
            "an inline block loads no file and duplicates nothing"
        );

        std::fs::remove_dir_all(&dir).unwrap();
    }
}
