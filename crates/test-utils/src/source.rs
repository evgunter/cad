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
//! **Scope is not answered here.** *Which binding does this name reach*
//! (a `let`, a pattern, a parameter, a `use`) is a question about the
//! syntax tree, and `crates/source-scope` answers it from a `syn` parse.
//! The ledger's second half sends scope questions there and refuses a
//! hand-rolled scope reader. This module keeps lexing and the item- and
//! brace-level helpers, and stays zero-dependency.
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
//! [`top_level_split`], [`item_body`] and [`attributed_extent`], plus
//! the traversals [`rust_sources`] and [`suite_files`].
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
    text[..at].matches('\n').count() + 1
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

/// Every `.rs` file under `dir`, sorted; `dir` itself when it is one
/// `.rs` file, so a guard can read a module's `foo.rs` beside its
/// `foo/` directory. Panics on an unreadable directory or an empty
/// result; a guard with a stronger premise (a minimum count, a required
/// file) should assert it on the result.
#[must_use]
pub fn rust_sources(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    if dir.is_file() && dir.extension().is_some_and(|e| e == "rs") {
        return vec![dir.to_path_buf()];
    }
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

/// **Every `fn` item with a body in a blanked view**: its name and its
/// body's extent ([`item_body`]'s `Body`, both braces included), in
/// order of appearance. A head is a `fn` keyword at a word boundary
/// followed by a name, so a function-pointer type (`fn(u8)`) is no
/// head. A declaration (a trait's default-less method) has no body and
/// is skipped.
///
/// # Errors
///
/// `Err` naming the item where a head has no terminator or its `{`
/// never closes ([`ItemBody::Unterminated`]): a walk that skipped one
/// would lose every needle in what follows, with no count moving.
pub fn fn_items(blanked: &str) -> Result<Vec<(String, std::ops::Range<usize>)>, String> {
    let mut out = Vec::new();
    for (at, _) in blanked.match_indices("fn ") {
        if !boundary_before(blanked, at) {
            continue;
        }
        let name = ident(blanked, skip_ws(blanked, at + 3));
        if name.is_empty() {
            continue;
        }
        match item_body(blanked, at) {
            ItemBody::Body(body) => out.push((name.to_string(), body)),
            ItemBody::Declaration(_) => {}
            ItemBody::Unterminated => {
                return Err(format!("`fn {name}` at byte {at} has no terminated body"));
            }
        }
    }
    Ok(out)
}

/// The span an outer attribute starting at `attr_at` (its `#`) governs:
/// from the attribute, through any attributes stacked after it, to the
/// end of the thing they are attached to. `None` on unbalanced text.
///
/// Same precondition as [`balanced_end`]. **The end depends on what
/// the attribute sits on**, and reading every attribute as an item's is
/// the defect this exists to prevent: a field, a variant or a match arm
/// has no `;` or `{` of its own, so an item-shaped scan runs past the
/// enclosing `}` and takes the NEXT item with it.
///
/// - **An item or a `let`** (its head, past any `pub(..)`, is an item
///   keyword) ends where [`item_body`] says: its body's `}` or its `;`.
///   A `,` ends nothing here, since a generic list carries one.
/// - **Anything else** — a field, a variant, an arm, an argument, an
///   expression statement — ends at the first `,` or `;` at bracket
///   depth zero (included), before a closer at depth zero (the
///   enclosing `}`, `)` or `]`), or at the close of a `{` opened at
///   depth zero (a struct variant's fields, an arm's block).
///
/// **The residue, stated:** in the second case `<` is not a bracket,
/// so a field typed `Map<K, V>` ends at its first `,`; what is left
/// over is the rest of that field, never the next item. An arm whose
/// body continues past a depth-zero block (`X => S { .. }.f()`) ends
/// at the block.
#[must_use]
pub fn attributed_extent(blanked: &str, attr_at: usize) -> Option<std::ops::Range<usize>> {
    const ITEM_KEYWORDS: [&str; 16] = [
        "fn",
        "impl",
        "mod",
        "struct",
        "enum",
        "union",
        "trait",
        "type",
        "const",
        "static",
        "use",
        "extern",
        "unsafe",
        "async",
        "let",
        "macro_rules",
    ];
    let mut head = attr_at;
    while blanked[head..].starts_with("#[") {
        head = skip_ws(blanked, balanced_end(blanked, head + 1)? + 1);
    }
    let mut word = head;
    if word_at(blanked, word, "pub") {
        word = skip_ws(blanked, word + "pub".len());
        if blanked[word..].starts_with('(') {
            word = skip_ws(blanked, balanced_end(blanked, word)? + 1);
        }
    }
    if ITEM_KEYWORDS.contains(&ident(blanked, word)) {
        return match item_body(blanked, head) {
            ItemBody::Body(body) => Some(attr_at..body.end),
            ItemBody::Declaration(semi) => Some(attr_at..semi + 1),
            ItemBody::Unterminated => None,
        };
    }
    let mut depth = 0usize;
    for (off, c) in blanked[head..].char_indices() {
        let at = head + off;
        match c {
            '{' if depth == 0 => return balanced_end(blanked, at).map(|end| attr_at..end + 1),
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' if depth == 0 => return Some(attr_at..at),
            ')' | ']' | '}' => depth -= 1,
            ',' | ';' if depth == 0 => return Some(attr_at..at + 1),
            _ => {}
        }
    }
    None
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
    /// `<file>: <fn>: <expression>` per call whose name-bearing argument
    /// is not a plain literal, and per declared carrier call that holds
    /// no literal argument — named by its first argument — where `<fn>`
    /// is the innermost `fn` item it stands in ([`fn_items`]; `-` outside
    /// any). One entry stands for every site of ONE function that spells
    /// the same expression, so a second forwarder in a file is its own
    /// entry and reds until a roster declares it. A `read_identity` call
    /// is keyed `<file>: read_identity#<n>(<head>)` by its place in its
    /// file instead. Each is a site whose carrier the
    /// caller must declare — declaring it makes that carrier's own call
    /// sites part of the census, so the chain terminates at literals or
    /// stays red.
    pub indirect: std::collections::BTreeSet<String>,
    /// `<file>: <path>` per site whose name is a `const`, in place of an
    /// `indirect` entry: the path's last segment is a `const` this walk
    /// found declared under `src`. A roster holds each to a declared
    /// [`NameCarrier::Const`], which reads the name out of it.
    pub consts: std::collections::BTreeSet<String>,
    /// Each declared carrier that matched nothing in the walk — for a
    /// [`NameCarrier::Const`], no declaration that holds a name; for a
    /// [`NameCarrier::Call`], no call or literal. A stale carrier reads
    /// as `Const(NAME)` or `Call(token)`.
    pub unfound: std::collections::BTreeSet<String>,
    /// `(name, <file>:<line>)` per predicate name a site REFERS to
    /// rather than decides: a plain literal in `predicate: Some("…")`,
    /// `predicate == Some("…")` or `predicate, Some("…")` — a fixture
    /// building a refusal, a test asserting the name one carried. A
    /// roster holds each to a name some census decides
    /// ([`unresolved_references`]), so a renamed predicate reds every
    /// fixture still spelling the old one. A deliberate reference to a
    /// name nothing decides says so on the line above it
    /// (`// UNDECIDED-NAME: <why>`) and is collected in
    /// [`Self::marked_references`] instead.
    pub references: std::collections::BTreeSet<(String, String)>,
    /// `(name, <file>:<line>)` per reference under an `UNDECIDED-NAME`
    /// marker: a roster holds each to a name NO census decides, so a
    /// marker outlives the name's absence only as a red.
    pub marked_references: std::collections::BTreeSet<(String, String)>,
    /// `<file>:<line>` per `UNDECIDED-NAME` marker outside test code — a
    /// `#[cfg(test)]` item, a file a `#[cfg(test)] mod` mounts, or a
    /// suite under `tests/`. A production reference to a name nothing
    /// decides is a name with no home, not a deliberate negative.
    pub misplaced_markers: std::collections::BTreeSet<String>,
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
///
/// **`geom_core::ladder::read_identity` is a funnel too**, deciding each
/// datum under the name it is handed beside it, and its data are an
/// iterator this reader cannot follow. Every call of it is therefore an
/// `indirect` site, `<file>: read_identity(<its first argument's head>)`,
/// so a crate's roster lists each and declares where its data's names
/// are spelled.
#[must_use]
pub fn predicate_census(src: &std::path::Path, carriers: &[NameCarrier]) -> PredicateCensus {
    let mut out = PredicateCensus::default();
    let files: Vec<(String, String)> = rust_sources(src)
        .into_iter()
        .map(|path| {
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("a readable source file {}: {e}", path.display()));
            // A walk over one file names its sites by that file's name.
            let rel = match path.strip_prefix(src) {
                Ok(rel) if !rel.as_os_str().is_empty() => rel,
                _ => path
                    .file_name()
                    .map_or(path.as_path(), std::path::Path::new),
            }
            .to_string_lossy()
            .replace('\\', "/");
            (rel, text)
        })
        .collect();
    // Every `const` the walk declares, so a site whose name is a path to
    // one is told apart from a forwarded expression by what the tree
    // holds, not by how the path is spelled.
    let declared: std::collections::BTreeSet<String> = files
        .iter()
        .flat_map(|(_, text)| const_names(&code_only(text)))
        .collect();
    let mut found = std::collections::BTreeSet::new();
    for (rel, text) in &files {
        // Comments are blanked in place, so prose that spells `decide (`
        // is not a call site and the offsets still index the file byte
        // for byte.
        let code = code_and_literals(text);
        // Tokens are LOCATED in the code-only view, where a literal's
        // bytes are blanked, so prose inside a string that spells
        // `decide(` is not a call site either; they are READ from the
        // view that keeps literals. Both are blanked in place, so one
        // set of offsets indexes both.
        let bare = code_only(text);
        // Every indirect site is keyed by the `fn` it stands in
        // (`<file>: <fn>: <head>`), so each forwarder is its own entry:
        // a new one reusing a parameter name another already spells
        // (`predicate`, `name`) reds rather than folding into it.
        let items = match fn_items(&bare) {
            Ok(items) => items,
            Err(why) => {
                out.unreadable.insert(format!("{rel}: {why}"));
                Vec::new()
            }
        };
        let enclosing = |at: usize| -> &str {
            items
                .iter()
                .filter(|(_, body)| body.contains(&at))
                .min_by_key(|(_, body)| body.len())
                .map_or("-", |(name, _)| name.as_str())
        };
        for (kind, payload, at) in funnel_sites_at(&bare, &code) {
            match kind {
                SiteKind::Name => {
                    out.names.insert(payload);
                }
                SiteKind::Indirect if names_a_const(&payload, &declared) => {
                    out.consts.insert(format!("{rel}: {payload}"));
                }
                SiteKind::Indirect => {
                    out.indirect
                        .insert(format!("{rel}: {}: {payload}", enclosing(at)));
                }
                SiteKind::Unreadable => {
                    out.unreadable.insert(format!("{rel}: {payload}"));
                }
            }
        }
        for (ordinal, (head, literals)) in identity_sites(&bare, &code).into_iter().enumerate() {
            out.indirect
                .insert(format!("{rel}: read_identity#{}({head})", ordinal + 1));
            out.names.extend(literals);
        }
        for carrier in carriers {
            match *carrier {
                NameCarrier::Call(token) => {
                    for open in carrier_groups(&bare, token) {
                        found.insert(format!("Call({token})"));
                        let Some(end) = balanced_end(&bare, open) else {
                            out.unreadable.insert(format!("{rel}: {token}(… unclosed"));
                            continue;
                        };
                        let literals = direct_literals(&code[open + 1..end]);
                        let consts: Vec<String> = arguments(&bare[open + 1..end])
                            .into_iter()
                            .filter(|arg| names_a_const(arg, &declared))
                            .collect();
                        if !literals.is_empty() {
                            out.names.extend(literals);
                        } else if !consts.is_empty() {
                            // A name handed on in a `const`: a const site, as
                            // a `decide*` call's is, so a roster holds it to
                            // a declared carrier.
                            for name in consts {
                                out.consts.insert(format!("{rel}: {name}"));
                            }
                        } else {
                            // A name handed on from elsewhere, named by the
                            // group's first argument as a `decide*` site is.
                            let head = first_argument(&bare[open + 1..end], &code[open + 1..end]);
                            out.indirect
                                .insert(format!("{rel}: {}: {head}", enclosing(open)));
                        }
                    }
                }
                NameCarrier::Const(token) => {
                    for decl in const_decls(&bare, token) {
                        match const_literal(&bare[decl..], &code[decl..]) {
                            Some(names) => {
                                if !names.is_empty() {
                                    found.insert(format!("Const({token})"));
                                }
                                out.names.extend(names);
                            }
                            None => {
                                out.unreadable.insert(format!("{rel}: const {token}"));
                            }
                        }
                    }
                }
            }
        }
        for (needle, raw) in [("include!", &bare), ("#[path", &code)] {
            let mut at = 0usize;
            while let Some(hit) = raw[at..].find(needle) {
                let start = at + hit;
                at = start + needle.len();
                out.unwalked.insert(format!("{rel}:{}", line(raw, start)));
            }
        }
    }
    let refs = read_references(&files, false);
    out.references = refs.found;
    out.marked_references = refs.marked;
    out.misplaced_markers = refs.misplaced;
    out.unfound = carriers
        .iter()
        .map(|carrier| match carrier {
            NameCarrier::Call(token) => format!("Call({token})"),
            NameCarrier::Const(token) => format!("Const({token})"),
        })
        .filter(|key| !found.contains(key))
        .collect();
    out
}

/// **The checks every predicate-name roster makes of its census**, in
/// one place: each crate's `recourse_roster` asks this for its
/// violations and asserts there are none, and holds its own doors'
/// names on top.
///
/// - Nothing is invisible: no spelling the reader cannot read, and no
///   `include!` or `#[path]` pulling source from outside `src` — a
///   `#[path = "x.rs"]` naming a file under `src`, which the walk reads
///   whole, is not a gap.
/// - No declared carrier is stale: each [`NameCarrier`] matched
///   something ([`PredicateCensus::unfound`]).
/// - Every site whose name is a `const` names a declared
///   [`NameCarrier::Const`], which reads the name out of it.
/// - Where the roster lists its indirect sites (`listed`, as
///   `(site, what carries the name there)`), the census's indirect sites
///   are exactly those. A roster that admits them by a rule of its own
///   passes `None` and holds them itself.
#[must_use]
pub fn roster_violations(
    src: &std::path::Path,
    census: &PredicateCensus,
    carriers: &[NameCarrier],
    listed: Option<&[(&str, &str)]>,
) -> Vec<String> {
    let mut out: Vec<String> = census
        .unreadable
        .iter()
        .map(|site| format!("a spelling the reader cannot read: {site}"))
        .collect();
    for site in &census.unwalked {
        if let Err(why) = mounted_under(src, site) {
            out.push(format!("source the reader does not walk: {site}: {why}"));
        }
    }
    out.extend(
        census
            .unfound
            .iter()
            .map(|carrier| format!("a declared carrier matched nothing in src: {carrier}")),
    );
    let consts: std::collections::BTreeSet<&str> = carriers
        .iter()
        .filter_map(|carrier| match carrier {
            NameCarrier::Const(name) => Some(*name),
            NameCarrier::Call(_) => None,
        })
        .collect();
    for site in &census.consts {
        let (_, path) = site.split_once(": ").expect("`<file>: <path>`");
        let last = path.rsplit("::").next().unwrap_or(path);
        if !consts.contains(last) {
            out.push(format!(
                "a site names the const `{last}`, not declared a carrier: {site}"
            ));
        }
    }
    if let Some(listed) = listed {
        let named: std::collections::BTreeSet<&str> =
            listed.iter().map(|(site, _)| *site).collect();
        if named.len() != listed.len() {
            out.push("an indirect site is listed twice".to_string());
        }
        out.extend(
            census
                .indirect
                .iter()
                .filter(|site| !named.contains(site.as_str()))
                .map(|site| format!("an indirect site not listed: {site}")),
        );
        out.extend(
            named
                .iter()
                .filter(|site| !census.indirect.contains(**site))
                .map(|site| format!("a listed indirect site the census no longer finds: {site}")),
        );
    }
    out
}

/// Whether the unwalked site `<file>:<line>` under `src` is one the
/// walk covers after all: a `#[path = "…"]` mounting a file under
/// `src`, which the walk reads; a `#[path]` mounting a file outside it
/// that spells no `decide`; or an `include!("…")` of a data file under
/// `src` — not a `.rs` file, and spelling no `decide`. Neither of the
/// last two holds a site to read.
fn mounted_under(src: &std::path::Path, site: &str) -> Result<(), String> {
    let (file, line) = site.rsplit_once(':').ok_or("not `<file>:<line>`")?;
    let line: usize = line.parse().map_err(|_| "not a line number")?;
    let file = src.join(file);
    let text = std::fs::read_to_string(&file).map_err(|e| e.to_string())?;
    let code = code_and_literals(&text);
    let attr = code
        .lines()
        .nth(line.saturating_sub(1))
        .ok_or("no such line")?
        .trim();
    let mount = attr
        .strip_prefix("#[path =")
        .and_then(|rest| rest.trim().strip_suffix(']'))
        .and_then(plain_string_literal);
    let data = attr.split_once("include!(").and_then(|(_, rest)| {
        let close = rest.find(')')?;
        plain_string_literal(rest[..close].trim())
    });
    let (target, is_data) = match (mount, data) {
        (Some(target), _) => (target, false),
        (None, Some(target)) => (target, true),
        (None, None) => {
            return Err(format!(
                "neither a `#[path = …]` mount nor an `include!` of a data file: {attr}"
            ));
        }
    };
    let mounted = file
        .parent()
        .ok_or("a file with no directory")?
        .join(target);
    if !mounted.is_file() {
        return Err(format!("mounts `{target}`, which is not a file"));
    }
    let under_src = !target.contains("..") && mounted.starts_with(src);
    if !under_src && !is_data {
        // A `#[path]` mount outside `src` (a test oracle shared with the
        // integration suites) is walked by no census: it is no blind spot
        // only while it spells no `decide`.
        let body = std::fs::read_to_string(&mounted).map_err(|e| e.to_string())?;
        if body.contains("decide") {
            return Err(format!(
                "mounts `{target}`, which is not under src and spells `decide`"
            ));
        }
        return Ok(());
    }
    if !under_src {
        return Err(format!(
            "includes `{target}`, which is not a file under src"
        ));
    }
    if is_data {
        let body = std::fs::read_to_string(&mounted).map_err(|e| e.to_string())?;
        if target.ends_with(".rs") || body.contains("decide") {
            return Err(format!(
                "includes `{target}`, source the walk does not read rather than data"
            ));
        }
    }
    Ok(())
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
    funnel_sites_at(bare, code)
        .into_iter()
        .map(|(kind, payload, _)| (kind, payload))
        .collect()
}

/// [`funnel_sites`], each with the offset of the occurrence that made
/// it, so a caller can place an indirect site in its enclosing `fn`.
fn funnel_sites_at(bare: &str, code: &str) -> Vec<(SiteKind, String, usize)> {
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
        // `let decide_m = |name, m| decide(name, m, band);` binds a local
        // funnel: the binding is not a call, and the closure's body and
        // its call sites are read where they stand.
        if before.ends_with("let") && boundary_before(before, before.len() - 3) {
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
                    out.push((
                        SiteKind::Unreadable,
                        one_line(&code[start..cursor + 3]),
                        start,
                    ));
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
                    start,
                ));
                continue;
            }
        }
        let open = cursor;
        at = open + 1;
        let arg = code[open + 1..].trim_start();
        let arg_bare = &bare[bare.len() - arg.len()..];
        if let Some(names) = literal_tuple(arg_bare, arg) {
            out.extend(names.into_iter().map(|name| (SiteKind::Name, name, start)));
            continue;
        }
        // A group of names some of which are not literals
        // (`[OFFSET, "row_floor"]`): its literals are names, and the group
        // is an indirect site for the rest.
        if (arg_bare.starts_with('(') || arg_bare.starts_with('['))
            && let Some(close) = balanced_end(arg_bare, 0)
        {
            out.extend(
                direct_literals(&arg[1..close])
                    .into_iter()
                    .map(|name| (SiteKind::Name, name, start)),
            );
        }
        match first_literal(arg) {
            Some(name) => out.push((SiteKind::Name, name, start)),
            None => {
                let close = balanced_end(bare, open).unwrap_or(bare.len());
                let expr = first_argument(&bare[open + 1..close], &code[open + 1..close]);
                out.push((SiteKind::Indirect, expr, start));
            }
        }
    }
    out
}

/// Every `read_identity` call in a blanked view, in order: the head of
/// its first argument (up to its first top-level comma, on one line),
/// and every plain literal inside the call — the data's names where
/// they are spelled at the call.
fn identity_sites(bare: &str, code: &str) -> Vec<(String, Vec<String>)> {
    const TOKEN: &str = "read_identity";
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(hit) = bare[at..].find(TOKEN) {
        let start = at + hit;
        at = start + TOKEN.len();
        if !boundary_before(bare, start) || !boundary_after(bare, at) {
            continue;
        }
        let before = bare[..start].trim_end();
        if before.ends_with("fn") && boundary_before(before, before.len() - 2) {
            continue;
        }
        let rest = bare[at..].trim_start();
        if !rest.starts_with('(') {
            // An import or a doc path, not a call.
            continue;
        }
        let open = bare.len() - rest.len();
        let Some(close) = balanced_end(bare, open) else {
            out.push(("… unclosed".to_string(), Vec::new()));
            continue;
        };
        out.push((
            first_argument(&bare[open + 1..close], &code[open + 1..close]),
            literals_within(&code[open + 1..close]),
        ));
    }
    out
}

/// Every plain string literal in `code` (a [`code_and_literals`] view),
/// at any depth.
fn literals_within(code: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(hit) = code[at..].find('"') {
        let start = at + hit;
        match first_literal(&code[start..]) {
            Some(name) => {
                at = start + name.len() + 2;
                out.push(name);
            }
            None => at = start + 1,
        }
    }
    out
}

/// Every plain literal a `predicate` field or comparison names
/// (`predicate: Some("…")`, `predicate == Some("…")`, `predicate,
/// Some("…")`), with its offset, in a blanked view and the view keeping
/// literals.
fn referenced_names(bare: &str, code: &str) -> Vec<(String, usize)> {
    const TOKEN: &str = "predicate";
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(hit) = bare[at..].find(TOKEN) {
        let start = at + hit;
        at = start + TOKEN.len();
        if !boundary_before(bare, start) || !boundary_after(bare, at) {
            continue;
        }
        let rest = bare[at..].trim_start();
        let Some(rest) = [":", "==", ","]
            .iter()
            .find_map(|sep| rest.strip_prefix(sep))
            .map(str::trim_start)
        else {
            continue;
        };
        let Some(rest) = rest.strip_prefix("Some(") else {
            continue;
        };
        // Every alternative of an or-pattern (`Some("a" | "b")`) is a
        // reference, each at its own offset.
        let mut after = bare.len() - rest.len();
        loop {
            let held = code[after..].trim_start();
            let lit = code.len() - held.len();
            let Some(name) = first_literal(held) else {
                break;
            };
            after = lit + name.len() + 2;
            out.push((name, lit));
            match bare[after..].trim_start().strip_prefix('|') {
                Some(next) => after = bare.len() - next.len(),
                None => break,
            }
        }
    }
    out
}

/// The why of a `// <WORD>: <why>` marker standing on the line directly
/// above the 1-based `line_no` of `text`, where the why is not empty.
#[must_use]
pub fn marker_above<'t>(text: &'t str, line_no: usize, word: &str) -> Option<&'t str> {
    let above = text.lines().nth(line_no.checked_sub(2)?)?;
    let why = above
        .trim_start()
        .strip_prefix("// ")?
        .strip_prefix(word)?
        .strip_prefix(':')?
        .trim();
    (!why.is_empty()).then_some(why)
}

/// What [`read_references`] reads off a set of files.
#[derive(Default)]
struct References {
    found: std::collections::BTreeSet<(String, String)>,
    marked: std::collections::BTreeSet<(String, String)>,
    misplaced: std::collections::BTreeSet<String>,
}

/// The references `files` (`(<rel path>, text)`) hold, each under its
/// marker or not, and the markers that stand outside test code. Every
/// file is test code where `all_test` (a crate's `tests/`).
fn read_references(files: &[(String, String)], all_test: bool) -> References {
    let mounted = test_mounted(files);
    let mut out = References::default();
    for (rel, text) in files {
        let bare = code_only(text);
        let code = code_and_literals(text);
        let in_test_file = all_test || mounted.iter().any(|dir| under_module(rel, dir));
        let extents = cfg_test_extents(&bare);
        for (name, at) in referenced_names(&bare, &code) {
            let line_no = line(text, at);
            let site = format!("{rel}:{line_no}");
            if marker_above(text, line_no, "UNDECIDED-NAME").is_some() {
                if !in_test_file && !extents.iter().any(|e| e.contains(&at)) {
                    out.misplaced.insert(site.clone());
                }
                out.marked.insert((name, site));
            } else {
                out.found.insert((name, site));
            }
        }
    }
    out
}

/// The byte ranges of the test-only items in a blanked view: those
/// under `#[cfg(test)]` or `#[cfg(any(test, …))]` (a test-support
/// feature beside `test`).
fn cfg_test_extents(bare: &str) -> Vec<std::ops::Range<usize>> {
    const ATTR: &str = "#[cfg(";
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(hit) = bare[at..].find(ATTR) {
        let start = at + hit;
        at = start + ATTR.len();
        let rest = bare[at..].trim_start();
        let test_only = rest.starts_with("test)") || {
            rest.strip_prefix("any(")
                .is_some_and(|inner| inner.trim_start().starts_with("test"))
        };
        if test_only && let Some(extent) = attributed_extent(bare, start) {
            out.push(extent);
        }
    }
    out
}

/// The module paths (`<rel>` without `.rs`) that a test-only `mod
/// <name>;` among `files` mounts — at `#[path = "…"]` where it carries
/// one: each is test code, with everything under it.
fn test_mounted(files: &[(String, String)]) -> Vec<String> {
    let mut out = Vec::new();
    for (rel, text) in files {
        let bare = code_only(text);
        let code = code_and_literals(text);
        // A `mod` declared in `a/b.rs` mounts `a/b/<name>`; one in a
        // `lib.rs`, `main.rs` or `mod.rs` mounts its sibling. A `#[path]`
        // is relative to the declaring file's directory.
        let (dir, stem) = match rel.rsplit_once('/') {
            Some((dir, file)) => (dir.to_string(), file.trim_end_matches(".rs").to_string()),
            None => (String::new(), rel.trim_end_matches(".rs").to_string()),
        };
        let join = |base: &str, tail: &str| {
            if base.is_empty() {
                tail.to_string()
            } else {
                format!("{base}/{tail}")
            }
        };
        let base = if ["lib", "main", "mod"].contains(&stem.as_str()) {
            dir.clone()
        } else {
            join(&dir, &stem)
        };
        for extent in cfg_test_extents(&bare) {
            // Past the item's attributes, reading a `#[path]` among them.
            let mut item = extent.start;
            let mut path = None;
            while bare[item..extent.end].starts_with("#[") {
                let Some(close) = balanced_end(&bare, item + 1) else {
                    break;
                };
                let attr = code[item..=close].trim();
                if let Some(target) = attr
                    .strip_prefix("#[path =")
                    .and_then(|r| r.trim().strip_suffix(']'))
                    .and_then(plain_string_literal)
                {
                    path = Some(target.trim_end_matches(".rs").to_string());
                }
                item = skip_ws(&bare, close + 1);
            }
            let decl = bare[item..extent.end].trim();
            let decl = decl.strip_prefix("pub(crate) ").unwrap_or(decl);
            let decl = decl.strip_prefix("pub ").unwrap_or(decl);
            if let Some(name) = decl
                .strip_prefix("mod ")
                .and_then(|rest| rest.trim().strip_suffix(';'))
            {
                out.push(match &path {
                    Some(target) => join(&dir, target),
                    None => join(&base, name.trim()),
                });
            }
        }
    }
    out
}

/// Whether the file at `rel` is the module `module` or under it.
fn under_module(rel: &str, module: &str) -> bool {
    let path = rel.trim_end_matches(".rs");
    path == module || path.starts_with(&format!("{module}/"))
}

/// Whether `dir` is a directory holding a `.rs` file at any depth —
/// what a walk that refuses an empty tree ([`rust_sources`]) needs to
/// know of a directory that may hold none (a crate's `tests/` of data).
#[must_use]
pub fn holds_rust_sources(dir: &std::path::Path) -> bool {
    std::fs::read_dir(dir).is_ok_and(|entries| {
        entries.flatten().any(|entry| {
            let path = entry.path();
            if path.is_dir() {
                holds_rust_sources(&path)
            } else {
                path.extension().is_some_and(|e| e == "rs")
            }
        })
    })
}

/// `(name, <file>:<line>)` per site, the shape a reference reads as.
pub type SiteNames = std::collections::BTreeSet<(String, String)>;

/// **The references a crate's `tests/` hold**: the same reading the
/// census makes of `src` ([`PredicateCensus::references`] and its
/// marked twin), over every `.rs` file under `tests`, all of it test
/// code. `(found, marked)`, each `(name, <file>:<line>)` relative to
/// `tests`.
#[must_use]
pub fn test_references(tests: &std::path::Path) -> (SiteNames, SiteNames) {
    if !holds_rust_sources(tests) {
        return Default::default();
    }
    let files: Vec<(String, String)> = rust_sources(tests)
        .into_iter()
        .filter_map(|path| {
            let text = std::fs::read_to_string(&path).ok()?;
            let rel = path
                .strip_prefix(tests)
                .ok()?
                .to_string_lossy()
                .replace('\\', "/");
            Some((rel, text))
        })
        .collect();
    let refs = read_references(&files, true);
    (refs.found, refs.marked)
}

/// The references in `census` that name no name in `decided`, as
/// `<file>:<line>: <name>`.
#[must_use]
pub fn unresolved_references(
    census: &PredicateCensus,
    decided: &std::collections::BTreeSet<String>,
) -> Vec<String> {
    census
        .references
        .iter()
        .filter(|(name, _)| !decided.contains(name))
        .map(|(name, site)| format!("{site}: {name}"))
        .collect()
}

/// The identifier of every `const NAME` declaration in a blanked view.
fn const_names(bare: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(hit) = bare[at..].find("const ") {
        let start = at + hit;
        at = start + "const ".len();
        if !boundary_before(bare, start) {
            continue;
        }
        let name: String = bare[at..]
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if !name.is_empty() && bare[at + name.len()..].trim_start().starts_with(':') {
            out.push(name);
        }
    }
    out
}

/// Whether `expr` is a path to one of `declared` — `NAME`,
/// `super::NAME`, `Self::NAME`.
fn names_a_const(expr: &str, declared: &std::collections::BTreeSet<String>) -> bool {
    let segments: Vec<&str> = expr.split("::").collect();
    segments
        .iter()
        .all(|seg| !seg.is_empty() && seg.chars().all(|c| c.is_alphanumeric() || c == '_'))
        && segments.last().is_some_and(|last| declared.contains(*last))
}

/// The head of a call's first argument, as a site is named by it: the
/// argument up to its first top-level comma, its first line, with the
/// whitespace collapsed. `bare` is the call's inner text blanked and
/// `code` the same bytes with literals kept.
fn first_argument(bare: &str, code: &str) -> String {
    let first = top_level_split(bare, ',')
        .into_iter()
        .next()
        .map_or(0..bare.len(), |r| r);
    one_line(code[first].trim().lines().next().unwrap_or(""))
}

/// The direct arguments of a bracket group, trimmed, in a blanked view.
fn arguments(inner: &str) -> Vec<String> {
    top_level_split(inner, ',')
        .into_iter()
        .map(|r| inner[r].trim().to_string())
        .collect()
}

/// The names of a tuple or array of plain string literals at the head
/// of `code` — a funnel deciding under a row and its straddle row
/// (`decide_levered(("row", "row_straddle"), …)`,
/// `decide_across_reported(["row", "row_floor"], …)`) — or `None` where
/// the head is not one. `bare` is the same text blanked, so its brackets
/// are real.
fn literal_tuple(bare: &str, code: &str) -> Option<Vec<String>> {
    if !bare.starts_with('(') && !bare.starts_with('[') {
        return None;
    }
    let close = balanced_end(bare, 0)?;
    let names: Vec<String> = top_level_split(&bare[1..close], ',')
        .into_iter()
        .map(|r| code[1 + r.start..1 + r.end].trim())
        .filter(|item| !item.is_empty())
        .map(|item| plain_string_literal(item).map(str::to_string))
        .collect::<Option<_>>()?;
    (!names.is_empty()).then_some(names)
}

/// **The carriers a roster file declares**, read off its text: each
/// `NameCarrier::Const("…")` and `NameCarrier::Call("…")` in code, in
/// order. The tree-wide coverage row builds each roster crate's census
/// with its own roster's carriers this way, so the union it resolves
/// references against is what the rosters read, with no second copy of
/// their lists.
#[must_use]
pub fn declared_carriers(roster: &str) -> Vec<NameCarrier> {
    let code = code_and_literals(roster);
    let mut found: Vec<(usize, NameCarrier)> = Vec::new();
    for (head, call) in [("NameCarrier::Const(", false), ("NameCarrier::Call(", true)] {
        let mut at = 0usize;
        while let Some(hit) = code[at..].find(head) {
            let open = at + hit + head.len();
            at = open;
            let Some(close) = code[open..].find(')') else {
                break;
            };
            if let Some(token) = plain_string_literal(code[open..open + close].trim()) {
                let token: &'static str = Box::leak(token.to_string().into_boxed_str());
                let carrier = if call {
                    NameCarrier::Call(token)
                } else {
                    NameCarrier::Const(token)
                };
                found.push((open, carrier));
            }
        }
    }
    found.sort_by_key(|(at, _)| *at);
    found.into_iter().map(|(_, carrier)| carrier).collect()
}

/// **The roster a crate runs, if it runs one**: the carriers
/// `<crate>/tests/recourse_roster.rs` declares, where that file calls
/// [`roster_violations`] in code; `None` where it does not, or there is
/// no such file. The coverage row's one test of "runs a roster".
#[must_use]
pub fn crate_roster(crate_dir: &std::path::Path) -> Option<Vec<NameCarrier>> {
    let text = std::fs::read_to_string(crate_dir.join("tests/recourse_roster.rs")).ok()?;
    code_only(&text)
        .contains("roster_violations(")
        .then(|| declared_carriers(&text))
}

/// **The names a crate that runs no roster yet spells where a name is
/// decided**: each `const NAME: &str` (or tuple or array of them) it
/// declares and, for a crate whose names live in a closed type's table
/// (`closed_table`), each `Variant => "lit"` arm. Never a reference site
/// and never an arbitrary literal, so a hold is only ever a decide-shaped
/// spelling of the name (the coverage row's `KNOWN_MISSING`).
#[must_use]
pub fn held_spellings(
    src: &std::path::Path,
    closed_table: bool,
) -> std::collections::BTreeSet<String> {
    let mut out = std::collections::BTreeSet::new();
    for file in rust_sources(src) {
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        let bare = code_only(&text);
        let code = code_and_literals(&text);
        for name in const_names(&bare) {
            for decl in const_decls(&bare, &name) {
                if let Some(names) = const_literal(&bare[decl..], &code[decl..]) {
                    out.extend(names);
                }
            }
        }
        if closed_table {
            for body in closed_table_bodies(&bare) {
                out.extend(variant_arms(&bare[body.clone()], &code[body]));
            }
        }
    }
    out
}

/// The fn names whose body is a closed type's name table: a variant's
/// row, its straddle and its floor.
const TABLE_FNS: [&str; 5] = ["name", "predicate", "straddle", "floor", "row"];

/// **The bodies a closed type spells its names in**, in a blanked view:
/// each `fn name/predicate/straddle/floor/row` body, and each braced
/// macro invocation (`gates! { … }`, the gate table's) — the only places
/// a `Variant => "lit"` arm is a decided name rather than a label.
fn closed_table_bodies(bare: &str) -> Vec<std::ops::Range<usize>> {
    let mut out = Vec::new();
    for name in TABLE_FNS {
        let needle = format!("fn {name}(");
        let mut at = 0usize;
        while let Some(hit) = bare[at..].find(&needle) {
            let start = at + hit;
            at = start + needle.len();
            if !boundary_before(bare, start) {
                continue;
            }
            let Some(open) = bare[at..].find('{').map(|i| at + i) else {
                continue;
            };
            if bare[at..open].contains(';') {
                continue; // a trait's declaration, no body
            }
            if let Some(close) = balanced_end(bare, open) {
                out.push(open..close);
            }
        }
    }
    let mut at = 0usize;
    while let Some(hit) = bare[at..].find('!') {
        let bang = at + hit;
        at = bang + 1;
        let head = bare[..bang].trim_end();
        let ident_start = head
            .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .map_or(0, |i| i + 1);
        let ident = &head[ident_start..];
        if ident.is_empty() || ident == "macro_rules" {
            continue;
        }
        let open = skip_ws(bare, at);
        if bare[open..].starts_with('{')
            && let Some(close) = balanced_end(bare, open)
        {
            out.push(open..close);
        }
    }
    out
}

/// The literal of every `Variant => "lit"` arm: a `=>` whose left side
/// ends in a capitalised identifier and whose right side opens with a
/// plain literal.
fn variant_arms(bare: &str, code: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(hit) = bare[at..].find("=>") {
        let arrow = at + hit;
        at = arrow + 2;
        let left = bare[..arrow].trim_end();
        let ident_start = left
            .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .map_or(0, |i| i + 1);
        let capitalised = left[ident_start..]
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_uppercase());
        if !capitalised {
            continue;
        }
        let right = code[at..].trim_start();
        if let Some(name) = first_literal(right) {
            out.push(name);
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
        // A return type (`-> Rows {`) opens the body, not a literal.
        if before.ends_with("->")
            || DECLARERS
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

/// The names a `const NAME: &str = "…";` declaration whose head starts
/// at the front of `text` holds — one, or each of a tuple of them
/// (`const SIDE: (&str, &str) = ("row", "row_straddle");`).
///
/// `Some` and empty where the declaration is not a name: a `const` of
/// the same identifier and another type (a number, a struct), or a
/// trait's declaration with no value, whose implementations carry the
/// names. `None` where it is a `&str` or a tuple of them this reader
/// cannot read.
fn const_literal(bare: &str, code: &str) -> Option<Vec<String>> {
    const STR: [&str; 2] = ["&str", "&'staticstr"];
    // The declaration's own `;`, at depth zero: `[&str; 2]` holds one.
    let mut depth = 0usize;
    let end = bare.char_indices().find_map(|(at, c)| {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth = depth.saturating_sub(1),
            ';' if depth == 0 => return Some(at),
            _ => {}
        }
        None
    })?;
    let Some(eq) = code[..end].find('=') else {
        return Some(Vec::new());
    };
    let ty: String = code[..eq]
        .split_once(':')
        .map_or("", |(_, ty)| ty)
        .split_whitespace()
        .collect();
    let value = code[eq + 1..end].trim_start();
    let at = end - value.len();
    if STR.contains(&ty.as_str()) {
        return plain_string_literal(value).map(|name| vec![name.to_string()]);
    }
    let parts: Option<Vec<&str>> = ty
        .strip_prefix('(')
        .and_then(|t| t.strip_suffix(')'))
        .map(|t| t.split(',').filter(|p| !p.is_empty()).collect());
    // An array of them: `[&str; 2]`.
    let array: Option<usize> = ty
        .strip_prefix('[')
        .and_then(|t| t.strip_suffix(']'))
        .and_then(|t| t.split_once(';'))
        .filter(|(elem, _)| STR.contains(elem))
        .and_then(|(_, n)| n.parse().ok());
    match (parts, array) {
        (Some(parts), _) if !parts.is_empty() && parts.iter().all(|p| STR.contains(p)) => {
            literal_tuple(&bare[at..end], &code[at..end]).filter(|names| names.len() == parts.len())
        }
        (_, Some(n)) => {
            literal_tuple(&bare[at..end], &code[at..end]).filter(|names| names.len() == n)
        }
        _ => Some(Vec::new()),
    }
}

/// The offset of every `const <token>` declaration, as a whole word, in
/// a blanked view.
fn const_decls(code: &str, token: &str) -> Vec<usize> {
    let needle = format!("const {token}");
    let mut out = Vec::new();
    let mut at = 0usize;
    while let Some(hit) = code[at..].find(&needle) {
        let start = at + hit;
        at = start + needle.len();
        if boundary_before(code, start) && boundary_after(code, at) {
            out.push(start);
        }
    }
    out
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
        ItemBody, Region, aggregation_violations, angle_end, attributed_extent, balanced_end,
        boundary_after, boundary_before, code_and_literals, code_only, comments_only,
        file_module_decls, item_body, keeping, top_level_split, where_at,
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

    /// **Every `read_identity` call is an indirect site, keyed by its
    /// ordinal in its file**, so two calls rustfmt lays out alike are two
    /// entries; its declaration, an import and a doc mention are not
    /// calls, and a literal inside a call is read as a name.
    #[test]
    fn a_read_identity_call_is_an_indirect_site() {
        let raw = "use ladder::{read_identity};\n\
                   /// [`read_identity`] reads data.\n\
                   pub fn read_identity<T>(d: T) {}\n\
                   fn f() { geom_core::ladder::read_identity(\n    [(\"lit\", m)], band); \
                   read_identity(\n    [(\"other\", m)], band); \
                   read_identity(xs.map(|x| (NAME, x)), band); }\n";
        let dir = std::env::temp_dir().join(format!("tu-identity-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        std::fs::write(dir.join("a.rs"), raw).expect("write");
        let census = super::predicate_census(&dir, &[]);
        std::fs::remove_dir_all(&dir).expect("cleanup");
        assert_eq!(
            census.indirect.iter().collect::<Vec<_>>(),
            [
                "a.rs: read_identity#1([(\"lit\", m)])",
                "a.rs: read_identity#2([(\"other\", m)])",
                "a.rs: read_identity#3(xs.map(|x| (NAME, x)))",
            ],
            "{raw}"
        );
        assert_eq!(
            census.names.iter().collect::<Vec<_>>(),
            ["lit", "other"],
            "{raw}"
        );
    }

    /// **A `Const` carrier reads every `&str` declaration of its name,
    /// or tuple or array of them, and only those**: a longer identifier is not
    /// it, a same-named const of another type or a trait's valueless
    /// declaration is not a name, and a `&str` whose value is not a
    /// plain literal is unreadable.
    #[test]
    fn a_const_carrier_reads_each_str_declaration_of_its_name() {
        let raw = "const APEX_DOWN: &str = \"no\";\n\
                   mod a { pub(crate) const APEX: &str = \"one\"; }\n\
                   mod b { const APEX: &'static str = \"two\"; }\n\
                   mod t { const APEX: (f64, f64) = (0.0, 1.0); }\n\
                   trait R { const APEX: &'static str; }\n\
                   const APEX: &str = concat!(\"a\", \"b\");\n\
                   const APEX: (&str, &'static str) = (\"row\", \"row_straddle\");\n\
                   const APEX: (&str, &str) = (\"row\", other);\n\
                   const APEX: [&str; 2] = [\"gap\", \"gap_floor\"];\n";
        let (bare, code) = (code_only(raw), code_and_literals(raw));
        let read: Vec<Option<Vec<String>>> = super::const_decls(&bare, "APEX")
            .into_iter()
            .map(|at| super::const_literal(&bare[at..], &code[at..]))
            .collect();
        let names = |n: &[&str]| Some(n.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        assert_eq!(
            read,
            [
                names(&["one"]),
                names(&["two"]),
                names(&[]),
                names(&[]),
                None,
                names(&["row", "row_straddle"]),
                None,
                names(&["gap", "gap_floor"]),
            ],
            "{raw}"
        );
    }

    /// **A tuple or array of plain literals at a funnel's head is its
    /// names**: a row and its straddle row are both read. A group holding
    /// anything but literals is an indirect site, and its literals are
    /// still read.
    #[test]
    fn a_funnels_literal_tuple_is_read_as_its_names() {
        let raw = "decide_levered((\"row\", \"row_straddle\"), x, band); \
                   decide_across([\"gap\", \"gap_floor\"], x, band); \
                   decide_levered((\"lone\", s), x, band);";
        let sites = super::funnel_sites(&code_only(raw), &code_and_literals(raw));
        let named: Vec<&str> = sites
            .iter()
            .filter(|(kind, _)| matches!(kind, super::SiteKind::Name))
            .map(|(_, name)| name.as_str())
            .collect();
        assert_eq!(
            named,
            ["row", "row_straddle", "gap", "gap_floor", "lone"],
            "{raw}"
        );
        assert!(
            sites
                .iter()
                .any(|(kind, expr)| matches!(kind, super::SiteKind::Indirect)
                    && expr.starts_with("(\"lone\"")),
            "a part-literal tuple is indirect: {raw}"
        );
    }

    /// **A `let` binding of a local funnel is not a call**: its body and
    /// its call sites are, each read where it stands.
    #[test]
    fn a_let_bound_funnel_is_read_at_its_body_and_its_calls() {
        let raw = "let decide_m = |name, m| decide(name, m, band); decide_m(\"lit\", m)?;";
        let sites = super::funnel_sites(&code_only(raw), &code_and_literals(raw));
        let read: Vec<(bool, &str)> = sites
            .iter()
            .map(|(kind, s)| (matches!(kind, super::SiteKind::Name), s.as_str()))
            .collect();
        assert_eq!(read, [(false, "name"), (true, "lit")], "{raw}");
    }

    /// **A declared carrier's call is read for its literals, else is a
    /// const site where an argument names a `const` the tree declares,
    /// else is named by its first argument**, so a forwarding hop is
    /// named as a `decide*` site's is and the same hop twice in one file
    /// is one entry.
    #[test]
    fn a_carrier_call_is_named_by_its_consts_or_its_first_argument() {
        let dir = std::env::temp_dir().join(format!("tu-carrier-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        std::fs::write(
            dir.join("a.rs"),
            "const ROW: &str = \"row\";\n\
             fn f() { sign_as(\"lit\", d); sign_as(super::ROW, d); \
             sign_as(name, d); sign_as(name, e); }\n",
        )
        .expect("write");
        let census = super::predicate_census(&dir, &[super::NameCarrier::Call("sign_as")]);
        std::fs::remove_dir_all(&dir).expect("cleanup");
        assert_eq!(
            census.names.iter().collect::<Vec<_>>(),
            ["lit"],
            "{census:?}"
        );
        assert_eq!(
            census.indirect.iter().collect::<Vec<_>>(),
            ["a.rs: f: name"],
            "{census:?}"
        );
        assert_eq!(
            census.consts.iter().collect::<Vec<_>>(),
            ["a.rs: super::ROW"],
            "{census:?}"
        );
    }

    /// **The roster harness reports each kind of gap once and passes a
    /// clean tree**: a `#[path]` to a file under `src` is walked, one
    /// outside it is not; a stale carrier, an undeclared const site, and
    /// an indirect site missing from or stale in the list each red.
    /// **Each forwarder is its own indirect site**: two functions in one
    /// file handing on a parameter of the same name are two entries,
    /// keyed by the `fn` each stands in, so a new forwarder beside a
    /// listed one reds rather than folding into its entry; a site outside
    /// any `fn` keys as `-`.
    #[test]
    fn a_forwarder_is_keyed_by_its_enclosing_fn() {
        let dir = std::env::temp_dir().join(format!("tu-fwd-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        std::fs::write(
            dir.join("a.rs"),
            "fn listed(predicate: &'static str) { decide(predicate, m); }\n\
             impl S { fn fresh_forward(&self, predicate: &'static str) {\n    \
             decide(predicate, m);\n    let inner = |x| { fn nested(predicate: &str) { decide(predicate, m); } };\n} }\n\
             static S: X = X { a: decide(predicate, m) };\n",
        )
        .expect("write");
        let census = super::predicate_census(&dir, &[]);
        std::fs::remove_dir_all(&dir).expect("cleanup");
        assert_eq!(
            census.indirect.iter().collect::<Vec<_>>(),
            [
                "a.rs: -: predicate",
                "a.rs: fresh_forward: predicate",
                "a.rs: listed: predicate",
                "a.rs: nested: predicate"
            ]
        );
    }

    #[test]
    fn the_roster_harness_names_each_gap() {
        let dir = std::env::temp_dir().join(format!("tu-roster-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        std::fs::write(
            dir.join("a.rs"),
            "#[path = \"b.rs\"]\nmod b;\n#[path = \"../c.rs\"]\nmod c;\n\
             const ROW: &str = \"row\";\nconst OTHER: &str = \"other\";\n\
             fn f() { decide(ROW, m); decide(OTHER, m); decide(name, m); }\n\
             /// prose naming a `#[path]` mount is not one\n",
        )
        .expect("write");
        std::fs::write(dir.join("b.rs"), "fn g() {}\n").expect("write");
        let carriers = [
            super::NameCarrier::Const("ROW"),
            super::NameCarrier::Call("gone"),
        ];
        let census = super::predicate_census(&dir, &carriers);
        let said = super::roster_violations(&dir, &census, &carriers, Some(&[("a.rs: stale", "")]));
        std::fs::remove_dir_all(&dir).expect("cleanup");
        assert_eq!(
            census.consts.iter().collect::<Vec<_>>(),
            ["a.rs: OTHER", "a.rs: ROW"]
        );
        assert_eq!(
            census.unwalked.iter().collect::<Vec<_>>(),
            ["a.rs:1", "a.rs:3"],
            "a doc comment's `#[path]` is prose"
        );
        let expect = [
            "source the reader does not walk: a.rs:3",
            "a declared carrier matched nothing in src: Call(gone)",
            "a site names the const `OTHER`, not declared a carrier: a.rs: OTHER",
            "an indirect site not listed: a.rs: f: name",
            "a listed indirect site the census no longer finds: a.rs: stale",
        ];
        assert_eq!(said.len(), expect.len(), "{said:#?}");
        for (said, expect) in said.iter().zip(expect) {
            assert!(said.starts_with(expect), "{said} is not `{expect}…`");
        }
    }

    /// **A census over one file names its sites by that file's name**,
    /// as a walk over a directory names them by their path under it.
    #[test]
    fn a_one_file_census_names_its_sites_by_the_file() {
        let dir = std::env::temp_dir().join(format!("tu-onefile-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let file = dir.join("table.rs");
        std::fs::write(
            &file,
            "const ROW: &str = \"row\";\nfn f() { decide(ROW, m); decide(name, m); }\n",
        )
        .expect("write");
        let census = super::predicate_census(&file, &[]);
        std::fs::remove_dir_all(&dir).expect("cleanup");
        assert_eq!(census.consts.iter().collect::<Vec<_>>(), ["table.rs: ROW"]);
        assert_eq!(
            census.indirect.iter().collect::<Vec<_>>(),
            ["table.rs: f: name"]
        );
    }

    /// **A literal a `predicate` field or comparison names is a
    /// reference**, read beside the decided names; one under an
    /// `UNDECIDED-NAME` marker with its why is not, and a name only
    /// mentioned in prose or in an unrelated `Some("…")` is not.
    #[test]
    fn a_predicate_literal_is_a_reference_unless_marked() {
        let dir = std::env::temp_dir().join(format!("tu-refs-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        std::fs::write(
            dir.join("a.rs"),
            "fn f() {\n\
                 let e = Indeterminate { predicate: Some(\"made\"), ..x };\n\
                 assert_eq!(d.predicate, Some(\"asserted\"));\n\
                 assert!(d.predicate == Some(\"compared\"));\n\
                 // UNDECIDED-NAME: a name no door decides, by design\n\
                 assert_eq!(d.predicate, Some(\"negative\"));\n\
                 let other = Some(\"not_a_name\");\n\
                 // predicate: Some(\"prose\")\n\
             }\n",
        )
        .expect("write");
        let census = super::predicate_census(&dir, &[]);
        std::fs::remove_dir_all(&dir).expect("cleanup");
        let names: Vec<&str> = census.references.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(names, ["asserted", "compared", "made"], "{census:?}");
        let decided: std::collections::BTreeSet<String> = ["made".to_string()].into();
        assert_eq!(
            super::unresolved_references(&census, &decided),
            ["a.rs:3: asserted", "a.rs:4: compared"]
        );
    }

    /// **An `include!` of a data file is walked; one of source is not**:
    /// the site is keyed at the line the `include!` stands on, even mid-
    /// line, and a data file under `src` spelling no `decide` is no
    /// blind spot, while an included `.rs` file is.
    #[test]
    fn an_included_data_file_is_no_blind_spot() {
        let dir = std::env::temp_dir().join(format!("tu-include-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        std::fs::write(
            dir.join("a.rs"),
            "fn f() {\n    let t: [f64; 1] = include!(\"t.in\");\n}\n",
        )
        .expect("write");
        std::fs::write(dir.join("t.in"), "[1.0]\n").expect("write");
        std::fs::write(dir.join("b.rs"), "fn g() {\n    include!(\"c.rs\");\n}\n").expect("write");
        std::fs::write(dir.join("c.rs"), "fn h() {}\n").expect("write");
        let census = super::predicate_census(&dir, &[]);
        let violations = super::roster_violations(&dir, &census, &[], None);
        std::fs::remove_dir_all(&dir).expect("cleanup");
        let unwalked: Vec<&str> = census.unwalked.iter().map(String::as_str).collect();
        assert_eq!(unwalked, ["a.rs:2", "b.rs:2"], "{census:?}");
        assert_eq!(violations.len(), 1, "{violations:?}");
        assert!(
            violations[0].contains("b.rs:2") && violations[0].contains("c.rs"),
            "{violations:?}"
        );
    }

    /// **A mount from outside `src` is no blind spot while it spells no
    /// `decide`**: a shared test oracle passes, a mounted file that
    /// decides is red.
    #[test]
    fn a_mount_from_outside_src_is_walked_only_while_it_decides_nothing() {
        let base = std::env::temp_dir().join(format!("tu-outside-{}", std::process::id()));
        let (dir, outside) = (base.join("src"), base.join("tests"));
        std::fs::create_dir_all(&dir).expect("temp dir");
        std::fs::create_dir_all(&outside).expect("temp dir");
        std::fs::write(outside.join("oracle.rs"), "pub fn q() {}\n").expect("write");
        std::fs::write(outside.join("decides.rs"), "fn d() { decide(x) }\n").expect("write");
        std::fs::write(
            dir.join("m.rs"),
            "#[path = \"../tests/oracle.rs\"]\nmod oracle;\n\
             #[path = \"../tests/decides.rs\"]\nmod decides;\n",
        )
        .expect("write");
        let census = super::predicate_census(&dir, &[]);
        let violations = super::roster_violations(&dir, &census, &[], None);
        std::fs::remove_dir_all(&base).expect("cleanup");
        assert_eq!(violations.len(), 1, "{violations:?}");
        assert!(
            violations[0].contains("m.rs:3") && violations[0].contains("decides.rs"),
            "{violations:?}"
        );
    }

    /// **A roster's carriers are read off its text, in order, and only a
    /// file that calls the harness in code runs a roster**: a commented
    /// call, or no file, is none.
    #[test]
    fn a_roster_is_read_off_its_text_and_runs_only_when_it_calls_the_harness() {
        let text = "const C: &[NameCarrier] = &[\n\
                    NameCarrier::Call(\"row_as\"),\n\
                    // NameCarrier::Const(\"COMMENTED\"),\n\
                    NameCarrier::Const(\"GAP\"),\n];\n";
        let got: Vec<String> = super::declared_carriers(text)
            .iter()
            .map(|c| format!("{c:?}"))
            .collect();
        assert_eq!(got, [r#"Call("row_as")"#, r#"Const("GAP")"#], "{got:?}");
        let dir = std::env::temp_dir().join(format!("tu-roster-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("tests")).expect("temp dir");
        assert!(super::crate_roster(&dir).is_none(), "no file is no roster");
        let roster = dir.join("tests/recourse_roster.rs");
        std::fs::write(&roster, format!("{text}// roster_violations(&src)\n")).expect("write");
        assert!(
            super::crate_roster(&dir).is_none(),
            "a commented call runs nothing"
        );
        std::fs::write(
            &roster,
            format!("{text}fn t() {{ roster_violations(&src); }}\n"),
        )
        .expect("write");
        let carriers = super::crate_roster(&dir);
        std::fs::remove_dir_all(&dir).expect("cleanup");
        assert_eq!(carriers.map(|c| c.len()), Some(2));
    }

    /// **A hold is a decide-shaped spelling alone**: a `const` of a
    /// name, and for a closed table a `Variant => "lit"` arm of its name
    /// table (a `fn name`-shaped body, or a braced macro invocation) —
    /// never a label arm elsewhere (`Low => "low"`), a reference, an
    /// argument or prose.
    #[test]
    fn a_listed_crate_holds_a_name_only_where_it_is_spelled_as_decided() {
        let dir = std::env::temp_dir().join(format!("tu-held-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        std::fs::write(
            dir.join("a.rs"),
            "// \"in_prose\"\n\
             const ROW: &str = \"const_row\";\n\
             const PAIR: [&str; 2] = [\"pair_a\", \"pair_b\"];\n\
             fn f() {\n\
                 shape(\"circle\");\n\
                 assert_eq!(d.predicate, Some(\"self_held\"));\n\
                 match g { Level::Low => \"low\", _ => \"fallback\" }\n\
             }\n\
             impl Gate { fn name(self) -> &'static str { match self { Gate::Overlap => \"arm_row\" } } }\n\
             gates! { Gated => \"macro_row\", Sized(1), \"its prose\"; }\n",
        )
        .expect("write");
        let roster = super::held_spellings(&dir, false);
        let table = super::held_spellings(&dir, true);
        std::fs::remove_dir_all(&dir).expect("cleanup");
        assert_eq!(
            roster.into_iter().collect::<Vec<_>>(),
            ["const_row", "pair_a", "pair_b"]
        );
        assert_eq!(
            table.into_iter().collect::<Vec<_>>(),
            ["arm_row", "const_row", "macro_row", "pair_a", "pair_b"]
        );
    }

    /// **Every alternative of an or-pattern is a reference**, and a
    /// marker outside test code is misplaced while one in a
    /// `#[cfg(test)]` item, or a file a `#[cfg(test)] mod` mounts, is
    /// not.
    #[test]
    fn or_patterns_are_read_whole_and_a_marker_belongs_to_test_code() {
        let dir = std::env::temp_dir().join(format!("tu-marks-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("lib")).expect("temp dir");
        std::fs::write(
            dir.join("lib.rs"),
            "fn prod() -> E {\n\
                 E {\n\
                     // UNDECIDED-NAME: a raise with no home\n\
                     predicate: Some(\"prod_raise\"),\n\
                 }\n\
             }\n\
             #[cfg(test)]\n\
             mod tests {\n\
                 fn t() {\n\
                     assert!(matches!(d.predicate, Some(\"alt_a\" | \"alt_b\")));\n\
                     // UNDECIDED-NAME: the cause this test hands the door\n\
                     assert_eq!(d.predicate, Some(\"inline_fixture\"));\n\
                 }\n\
             }\n\
             #[cfg(test)]\n\
             mod fixtures;\n\
             #[cfg(test)]\n\
             #[path = \"elsewhere.rs\"]\n\
             mod renamed;\n\
             #[cfg(any(test, feature = \"test-support\"))]\n\
             mod support;\n",
        )
        .expect("write");
        for (file, name) in [
            ("elsewhere.rs", "path_fixture"),
            ("support.rs", "support_fixture"),
        ] {
            std::fs::write(
                dir.join(file),
                format!(
                    "fn f() {{\n    // UNDECIDED-NAME: {name}'s own cause\n    \
                     let e = E {{ predicate: Some(\"{name}\") }};\n}}\n"
                ),
            )
            .expect("write");
        }
        std::fs::write(
            dir.join("fixtures.rs"),
            "fn f() {\n\
                 // UNDECIDED-NAME: a mounted fixture's own cause\n\
                 let e = E { predicate: Some(\"mounted_fixture\") };\n\
             }\n",
        )
        .expect("write");
        let census = super::predicate_census(&dir, &[]);
        std::fs::remove_dir_all(&dir).expect("cleanup");
        let found: Vec<&str> = census.references.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(found, ["alt_a", "alt_b"], "{census:?}");
        let marked: Vec<&str> = census
            .marked_references
            .iter()
            .map(|(n, _)| n.as_str())
            .collect();
        assert_eq!(
            marked,
            [
                "inline_fixture",
                "mounted_fixture",
                "path_fixture",
                "prod_raise",
                "support_fixture"
            ]
        );
        assert_eq!(
            census.misplaced_markers.iter().collect::<Vec<_>>(),
            ["lib.rs:4"]
        );
    }

    /// **A crate's `tests/` are read for references**, every file of
    /// them test code.
    #[test]
    fn a_crates_tests_are_read_for_references() {
        let dir = std::env::temp_dir().join(format!("tu-testrefs-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("sub")).expect("temp dir");
        std::fs::write(
            dir.join("sub/a.rs"),
            "fn t() {\n\
                 assert_eq!(e.predicate, Some(\"suite_ref\"));\n\
                 // UNDECIDED-NAME: the suite's own probe\n\
                 assert_eq!(e.predicate, Some(\"suite_probe\"));\n\
             }\n",
        )
        .expect("write");
        let (found, marked) = super::test_references(&dir);
        std::fs::remove_dir_all(&dir).expect("cleanup");
        assert_eq!(
            found.into_iter().collect::<Vec<_>>(),
            [("suite_ref".to_string(), "sub/a.rs:2".to_string())]
        );
        assert_eq!(
            marked.into_iter().collect::<Vec<_>>(),
            [("suite_probe".to_string(), "sub/a.rs:4".to_string())]
        );
    }

    /// **`fn_items` walks every item with a body through `item_body`**: a
    /// bracketed `;` in a signature ends nothing, a declaration and a
    /// function-pointer type are no items, and an unclosed body is an
    /// error.
    #[test]
    fn fn_items_names_each_body_and_refuses_an_unclosed_one() {
        let text = "fn a() -> [u8; 2] { [0; 2] }\n\
                    trait T { fn b(&self); fn c(&self) {} }\n\
                    type F = fn(u8);\n";
        let names: Vec<String> = super::fn_items(text)
            .expect("well-formed")
            .into_iter()
            .map(|(n, _)| n)
            .collect();
        assert_eq!(names, ["a", "c"]);
        assert!(super::fn_items("fn d() { {").is_err());
    }

    /// **A return type is not a struct literal**: `-> Rows {` opens the
    /// function's body, and only the literal inside it is a carrier
    /// group.
    #[test]
    fn a_return_type_opens_no_carrier_group() {
        let raw = "fn rows() -> Rows { Rows { a: \"x\" } }";
        let bare = code_only(raw);
        let groups = super::carrier_groups(&bare, "Rows");
        assert_eq!(groups.len(), 1, "{groups:?}");
        assert!(raw[groups[0]..].starts_with("{ a:"), "{raw}");
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

    /// **An attribute governs its own thing and no further.** Each row
    /// is a shape whose end is not an item's: read as one, the span
    /// runs past the enclosing `}` and takes the next item with it.
    #[test]
    fn an_attribute_spans_its_field_variant_arm_or_item_and_stops() {
        let span = |text: &str| {
            let code = code_only(text);
            let at = code.find("#[").expect("an attribute");
            let range = attributed_extent(&code, at).expect("balanced");
            text[range].to_owned()
        };
        assert_eq!(
            span("struct S { a: u8, #[cfg(test)] b: [u8; 2] } impl X for S {}"),
            "#[cfg(test)] b: [u8; 2] ",
            "a last field stops before the struct's closing brace"
        );
        assert_eq!(
            span("struct S { #[cfg(test)] b: (u8, u8), c: u8 }"),
            "#[cfg(test)] b: (u8, u8),"
        );
        assert_eq!(
            span("enum E { #[cfg(test)]\n#[doc(hidden)] V { x: u8 }, W }"),
            "#[cfg(test)]\n#[doc(hidden)] V { x: u8 }",
            "stacked attributes, and a struct variant's fields"
        );
        assert_eq!(
            span("match x { #[cfg(test)] A => f(a, b), B => g() }"),
            "#[cfg(test)] A => f(a, b),"
        );
        assert_eq!(
            span("#[cfg(test)] pub(crate) fn f<A, B>() -> [u8; 2] { g(); } fn h() {}"),
            "#[cfg(test)] pub(crate) fn f<A, B>() -> [u8; 2] { g(); }",
            "an item's generic `,` ends nothing"
        );
        assert_eq!(span("#[cfg(test)] mod m; fn h() {}"), "#[cfg(test)] mod m;");
        assert_eq!(
            span("fn f() { #[cfg(test)] tear(a, b); after(); }"),
            "#[cfg(test)] tear(a, b);"
        );
        assert_eq!(
            attributed_extent(&code_only("#[cfg(test)] fn f() {"), 0),
            None
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
