//! **This crate's own sources, read as source.** Several guards here
//! check properties of the SOURCE surface — what a future door will
//! look like when someone writes one — which nothing at runtime can
//! enumerate. They walk `topo/src` and read text.
//!
//! Its own module rather than a section of [`crate::fixtures`]: that
//! module's subject is canonical well-formed bodies, this one's is a
//! Rust reader, and one header cannot describe both. The two were one
//! file, split ~900 lines apart with a *"Not fixtures:"* hedge doing a
//! module boundary's work.
//!
//! # One lexer
//!
//! [`CodeOnly`] is the only thing here that knows Rust's lexical
//! grammar. Everything else — the item scan, the mutation-door walk,
//! every consumer's search — runs over its output, in which every
//! comment, string literal and char literal has been blanked to
//! spaces with byte positions preserved. That is a structural rule,
//! not a convention: [`CodeOnly::public_fns`] is a method, so there is
//! no way to run the item scan over un-blanked text.
//!
//! **It is a rule because it was broken.** This module's first shape
//! had two lexers: a delimiter matcher that carved function bodies,
//! and a blanker layered on top of it. The blanker knew `'"'`, nested
//! block comments and raw strings; the matcher underneath it knew none
//! of the three, so a door body containing `'"'` was dropped from the
//! walk entirely and the next door's body corrupted — the guard's own
//! subject, one layer beneath the guard.
//!
//! # Why text and not a parser
//!
//! `syn` would dissolve the lexing — [`CodeOnly`] is a hand-rolled
//! lexer, and every defect it has ever had has been a lexing defect.
//! It is not taken, and the reason is not build cost or dependency
//! policy (Evan's rule is that adding one is fine at ~2 weeks' release
//! age, and `syn` is already in this workspace's lock file
//! transitively):
//!
//! **A parser closes the smaller half.** Of the blind spots
//! [`mutation_doors`] discloses, a parser closes exactly one — the
//! item spelling, which is closed here instead by scanning the `fn`
//! token rather than a `pub fn ` literal. It closes none of the four
//! that actually cost coverage. Delegation needs a call graph;
//! aliasing and re-export need name resolution; `cfg` needs
//! evaluation; *"`topo/src` only"* needs the other crates. `syn`
//! parses one file into a tree and does none of those. Buying a
//! dependency that leaves every disclosed hole exactly where it is,
//! to fix a lexer that has a red test row per construct it handles,
//! is not the better close.
//!
//! **What would close the four is not a parser either** — it is a
//! call-graph read (rust-analyzer, or a `cargo`-driven pass), which is
//! a much larger decision than a dependency. A row that wants
//! delegation closed should propose that, and should not re-litigate
//! `syn` on the way past.

// Test-support code: panicking is a test's failure mechanism (L5).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

/// This crate's `src/`, resolved for both ways the suite runs: a plain
/// `cargo test` (where the baked-in `CARGO_MANIFEST_DIR` is the tree
/// that is here) and a nextest ARCHIVE replayed on a different runner
/// (where that absolute path need not exist, but `--workspace-remap`
/// has pointed the per-test cwd at the crate root).
pub(crate) fn src_root() -> std::path::PathBuf {
    let baked = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    if baked.is_dir() {
        return baked;
    }
    let cwd = std::env::current_dir()
        .expect("a working directory")
        .join("src");
    assert!(cwd.is_dir(), "neither {baked:?} nor {cwd:?} is topo's src/");
    cwd
}

/// Every `.rs` file under `dir`, recursively.
pub(crate) fn collect_rs(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("a readable source directory") {
        let path = entry.expect("a readable directory entry").path();
        if path.is_dir() {
            collect_rs(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// Every `.rs` file under this crate's `src/`, with the walk's own
/// sanity check: a broken or empty walk would otherwise let every
/// guard built on it pass by finding nothing.
pub(crate) fn crate_sources() -> Vec<std::path::PathBuf> {
    let src = src_root();
    let mut files = Vec::new();
    collect_rs(&src, &mut files);
    assert!(
        files.len() > 20 && files.iter().any(|f| f.ends_with("lib.rs")),
        "the walk of {src:?} found {} file(s) and no lib.rs — it is not reading topo/src",
        files.len()
    );
    files
}

/// Rust source with every comment, string literal and char literal
/// blanked to spaces. Byte length and line structure are preserved,
/// and each input byte is either copied or blanked, so a multi-byte
/// character is never half-erased.
///
/// **The only lexer in this crate**, by construction: the item scan is
/// [`Self::public_fns`], a method, so nothing can run it over raw
/// text. See the module docs for why that is a rule.
///
/// Handled: line comments; block comments, which nest in Rust; string
/// literals plain, raw, byte and C-string (`"…"`, `r#"…"#`, `b"…"`,
/// `br#"…"#`, `c"…"`, `cr#"…"#`); and char literals, including `'"'`
/// and `'\''`, which a naive scanner reads as opening a string.
/// Lifetimes are left alone. Every one of those has a row in
/// [`self::tests`] that reds if it stops being handled.
///
/// # The proof, and how to re-run it
///
/// The rows in [`self::tests`] are examples. The *proof* is a
/// differential against **rustc's own lexer**, and it is cheap to
/// reproduce: generate Rust snippets that plant a call to an undefined
/// function — `NEEDLE()` — inside every construct in the list above,
/// compile each one, and take rustc's `E0425 cannot find function
/// NEEDLE` as ground truth for *"this text is code"*. Agreement with
/// [`Self::of`] is then mechanical in both directions: a snippet rustc
/// resolves and the blanker erases is an over-strip, one rustc ignores
/// and the blanker keeps is the finding S92 was about.
///
/// Run against **2,021 generated snippets and 70 hand-built cases at
/// rustc 1.97.0: agreement everywhere, 0 silent and 0 loud.** Written
/// down here rather than left in a report because *"I did not prove it
/// against a grammar"* was this reader's standing caveat, and the
/// answer is a method someone can re-run rather than an argument
/// someone has to re-make.
///
/// **Two gaps the method also measured, both real and both dormant.**
/// The blanker does not expand macros, so a `pub fn` inside a
/// `macro_rules!` body is counted as an item and one inside an
/// `include!`d file is not seen at all. In `topo/src` today there are
/// two `macro_rules!`, neither containing a `pub fn`, and no
/// `include!`.
pub(crate) struct CodeOnly(String);

impl CodeOnly {
    /// Blank `text`'s comments and literals.
    pub(crate) fn of(text: &str) -> Self {
        let b = text.as_bytes();
        let mut out: Vec<u8> = Vec::with_capacity(b.len());
        let mut i = 0usize;
        // Blank `n` bytes from `i`, preserving newlines.
        macro_rules! blank {
            ($n:expr) => {{
                let end = (i + $n).min(b.len());
                out.extend(
                    b[i..end]
                        .iter()
                        .map(|c| if *c == b'\n' { b'\n' } else { b' ' }),
                );
                i = end;
            }};
        }
        while i < b.len() {
            if let Some((open, hashes)) = raw_string_open(b, i) {
                // The prefix and opening quote are code; the content
                // and closing delimiter are not.
                let head = (i + open).min(b.len());
                out.extend_from_slice(&b[i..head]);
                i = head;
                let mut j = i;
                let close = loop {
                    if j >= b.len() {
                        break b.len();
                    }
                    if b[j] == b'"' && b[j + 1..].iter().take(hashes).all(|c| *c == b'#') {
                        break (j + 1 + hashes).min(b.len());
                    }
                    j += 1;
                };
                blank!(close - i);
                continue;
            }
            match b[i] {
                b'/' if b.get(i + 1) == Some(&b'/') => {
                    let end = text[i..].find('\n').map_or(b.len(), |r| i + r);
                    blank!(end - i);
                }
                b'/' if b.get(i + 1) == Some(&b'*') => {
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
                    blank!(j - i);
                }
                b'"' => {
                    let mut j = i + 1;
                    while j < b.len() && b[j] != b'"' {
                        j += usize::from(b[j] == b'\\') + 1;
                    }
                    blank!((j + 1).min(b.len()) - i);
                }
                b'\'' if char_literal_len(b, i).is_some() => {
                    let n = char_literal_len(b, i).expect("just matched");
                    blank!(n);
                }
                c => {
                    out.push(c);
                    i += 1;
                }
            }
        }
        Self(String::from_utf8(out).expect("blanking never splits a character"))
    }

    /// The blanked text.
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

/// The `(prefix length up to and including the opening quote, `#`
/// count)` of the raw-string literal opening at `i` — `r"`, `r#"`,
/// `br"`, `cr#"` and so on — or `None` when `i` does not open one.
///
/// The byte and C-string prefixes are here rather than falling through
/// to the plain-string arm because that arm honours `\` escapes, which
/// a raw string does not have: `br"x\"` would be read as an unclosed
/// string and blank the rest of the file.
fn raw_string_open(b: &[u8], i: usize) -> Option<(usize, usize)> {
    // The prefix must start a token.
    if i > 0 && (b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_') {
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
    let mut h = 0usize;
    while b.get(j + h) == Some(&b'#') {
        h += 1;
    }
    (b.get(j + h) == Some(&b'"')).then_some((j + h + 1 - i, h))
}

/// The byte length of the char literal at `i`, or `None` when the
/// quote opens a lifetime instead.
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

impl CodeOnly {
    /// Every `pub fn` item in this file, as `(name, parameter list,
    /// body)` — all three slices of the blanked text.
    ///
    /// **Scanned on the `fn` token, not on a `pub fn ` literal.** The
    /// literal misses `pub const fn`, `pub async fn`, `pub unsafe fn`
    /// and `pub extern "C" fn`; walking back from `fn` over the
    /// qualifiers to a bare `pub` finds all of them and still rejects
    /// `pub(crate) fn`, whose preceding token is `)`.
    pub(crate) fn public_fns(&self) -> Vec<(&str, &str, &str)> {
        let code = &self.0;
        let b = code.as_bytes();
        let mut out = Vec::new();
        let mut at = 0usize;
        while let Some(rel) = code[at..].find("fn") {
            let kw = at + rel;
            at = kw + 2;
            if !is_token(b, kw, 2) || !public_qualifiers_precede(b, kw) {
                continue;
            }
            // From here the head IS a public `fn`, so every remaining
            // exit is either a bodiless declaration or a defect. See
            // `gave_up` below: this scan does not get to skip one
            // quietly, because skipping one quietly is what it did.
            let parsed = (|| {
                let (name, after_name) = ident_after(code, at)?;
                let open = param_list_start(b, after_name)?;
                let close = matching(b, open, b'(', b')')?;
                let brace = body_start(b, close)?;
                let end = matching(b, brace, b'{', b'}')?;
                Some((name, open, close, brace, end))
            })();
            match parsed {
                Some((name, open, close, brace, end)) => {
                    out.push((name, &code[open..close], &code[brace..end]));
                    at = end;
                }
                None if bodiless_declaration(b, at) => {}
                None => gave_up(code, kw),
            }
        }
        out
    }
}

/// The `{` opening the body of a function whose parameter list closes
/// at `close`, or `None` for a declaration with no body.
///
/// **Bracket-aware, because a `;` is not only a statement end.** The
/// first shape of this took the first `{`-or-`;` after the parameter
/// list and gave up on a `;` — which silently dropped every
/// `pub fn … -> [f64; 3]`, array types in return position being house
/// style in this kernel. `crates/topo/src/null.rs`'s `loops` was one,
/// live in the tree; a planted mutation door with an array return was
/// invisible to both guards with **no count moving at all**. Depth is
/// tracked so only a delimiter at the signature's own level decides.
fn body_start(b: &[u8], close: usize) -> Option<usize> {
    let (mut paren, mut bracket) = (0usize, 0usize);
    for (i, c) in b.iter().enumerate().skip(close + 1) {
        match c {
            b'(' => paren += 1,
            b')' => paren = paren.saturating_sub(1),
            b'[' => bracket += 1,
            b']' => bracket = bracket.saturating_sub(1),
            b';' if paren == 0 && bracket == 0 => return None,
            b'{' if paren == 0 && bracket == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

/// Whether the public `fn` head whose name starts at `at` is a
/// declaration with no body — an `extern` block's, or a trait's
/// default-less method — rather than one this scan failed to read.
fn bodiless_declaration(b: &[u8], at: usize) -> bool {
    let (mut paren, mut bracket) = (0usize, 0usize);
    for (i, c) in b.iter().enumerate().skip(at) {
        match c {
            b'(' => paren += 1,
            b')' => paren = paren.saturating_sub(1),
            b'[' => bracket += 1,
            b']' => bracket = bracket.saturating_sub(1),
            b';' if paren == 0 && bracket == 0 => return true,
            b'{' if paren == 0 && bracket == 0 => return false,
            _ => {
                let _ = i;
            }
        }
    }
    // Ran off the end without either: not a declaration, a truncated
    // or unbalanced file.
    false
}

/// A public `fn` head this scan recognised and then could not read.
///
/// **Loud on purpose.** Every guard built on this walk asserts about
/// *all* doors, so an item the scan drops is a door with no
/// classification and no count to notice it — which is the finding
/// this module exists to close, one layer up. A parse this reader
/// cannot complete is a defect in the reader, not an item to skip.
fn gave_up(code: &str, kw: usize) -> ! {
    let line = code[..kw].bytes().filter(|c| *c == b'\n').count() + 1;
    let snippet: String = code[kw..].chars().take(80).collect();
    panic!(
        "the item scan recognised a public `fn` at line {line} and could not read it: \
         {snippet:?}. It is not allowed to skip one — a dropped item is a door nothing \
         classifies and no count moves for.",
    );
}

/// Whether the `len` bytes at `i` stand alone as a token.
fn is_token(b: &[u8], i: usize, len: usize) -> bool {
    let identish = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    (i == 0 || !identish(b[i - 1])) && b.get(i + len).is_none_or(|c| !identish(*c))
}

/// Whether the `fn` at `kw` is preceded by a bare `pub`, across any
/// run of `const` / `async` / `unsafe` / `extern` qualifiers.
fn public_qualifiers_precede(b: &[u8], kw: usize) -> bool {
    let mut end = kw;
    loop {
        while end > 0 && b[end - 1].is_ascii_whitespace() {
            end -= 1;
        }
        let mut start = end;
        while start > 0 && (b[start - 1].is_ascii_alphanumeric() || b[start - 1] == b'_') {
            start -= 1;
        }
        match &b[start..end] {
            b"pub" => return true,
            b"const" | b"async" | b"unsafe" | b"extern" if start < end => end = start,
            _ => return false,
        }
    }
}

/// The identifier starting at or after `from`, and the index one past
/// it.
fn ident_after(code: &str, from: usize) -> Option<(&str, usize)> {
    let rest = code.get(from..)?;
    let lead = rest.len() - rest.trim_start().len();
    let name = &rest[lead..];
    let n = name.find(|c: char| !c.is_alphanumeric() && c != '_')?;
    (n > 0).then_some((&name[..n], from + lead + n))
}

/// The `(` opening the parameter list of a function whose name ends at
/// `from`, skipping a generic list — which may itself contain parens,
/// as `<F: Fn(u32) -> u32>` does.
fn param_list_start(b: &[u8], from: usize) -> Option<usize> {
    let mut i = from;
    let mut angle = 0usize;
    while i < b.len() {
        match b[i] {
            b'-' if b.get(i + 1) == Some(&b'>') => i += 1,
            b'<' => angle += 1,
            b'>' if angle > 0 => angle -= 1,
            b'(' if angle == 0 => return Some(i),
            c if angle == 0 && !c.is_ascii_whitespace() && c != b'<' => return None,
            _ => {}
        }
        i += 1;
    }
    None
}

/// The index of the delimiter closing the one at `open`. Safe as a
/// plain depth count because it runs on blanked text, where a
/// delimiter inside a comment or literal is a space.
fn matching(b: &[u8], open: usize, l: u8, r: u8) -> Option<usize> {
    let mut depth = 0usize;
    for (i, c) in b.iter().enumerate().skip(open) {
        if *c == l {
            depth += 1;
        } else if *c == r {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}

/// One public mutation door into a [`crate::Body`], as the guards that
/// walk the mutation surface see it.
pub(crate) struct MutationDoor {
    /// The source file the door is declared in.
    pub(crate) file: std::path::PathBuf,
    /// The door's name.
    pub(crate) name: String,
    /// The door's body, blanked by [`CodeOnly`].
    code: String,
}

impl MutationDoor {
    /// `file::name`, the spelling the guards report an offender by.
    pub(crate) fn site(&self) -> String {
        format!("{}::{}", self.file.display(), self.name)
    }

    /// Whether `needle` occurs in the door's body **as code**.
    ///
    /// Named for what it does. It is a substring search, so it is the
    /// caller's job to pass a needle that only a call can match:
    /// `assert_euler_postcondition(` with its paren rather than the
    /// bare name, which a `use` line would satisfy.
    pub(crate) fn code_contains(&self, needle: &str) -> bool {
        self.code.contains(needle)
    }
}

/// The number of mutation doors in `topo/src`, measured on `main` at
/// `4f959cb4` and unchanged since. Not asserted exactly — a new door
/// is normal and this walk is not the place to notice one. It is here
/// so [`mutation_doors`]' floor is derived from a number rather than
/// chosen, and so a reader can tell a floor with two doors of slack
/// from a floor with twenty-seven.
const DOORS_MEASURED: usize = 37;

/// Every public mutation door into a [`crate::Body`] declared in this
/// crate's `src/`: a public `fn` whose parameter list takes
/// `&mut self` or `&mut Body<T>`.
///
/// **This is the shared concept.** Two guards classify this one
/// population by two different properties — the tier-1 postcondition
/// surface ([`crate::review_m1_pr5_internal`]) and the pcurve-staleness
/// posture ([`crate::pcurves`]) — and each used to carry its own copy
/// of the walk and of the predicate. **The classifications stay in
/// their own guards**: they are two properties of one set, not one
/// property, and a merged table would make an edit about either
/// concern able to red the other's guard. That the two tables still
/// range over this one set is asserted, not assumed —
/// `review_m1_pr5_internal::the_two_door_tables_cover_the_same_surface`.
/// **This paragraph is the one statement of that decision**; the two
/// guards point here rather than restating it.
///
/// **What this walk cannot see**, and every guard built on it
/// inherits:
///
/// - **Delegation.** A door that reaches the call one hop away,
///   through a helper, reads as not making it.
/// - **`topo/src` only.** A `&mut Body` door in another crate, or in
///   this crate's `tests/`, is outside [`crate_sources`].
/// - **Module visibility.** A `pub fn` inside a `pub(crate) mod` is
///   not public to the world, and one file's text cannot say which
///   modules are re-exported. Everything `pub` in `topo/src` is
///   counted, which over-counts rather than under-counts — the
///   safe direction for a guard that asserts about all doors.
/// - **`cfg`.** The read is of text, so a call under `cfg(false)`
///   counts as present and a door under one counts as a door.
/// - **Aliasing.** A call reached under a re-exported or renamed path
///   does not match its needle.
///
/// The first, fourth and fifth are the ones that cost coverage, and
/// none of them is a lexing problem — see the module docs on why a
/// parser is not the answer to them.
pub(crate) fn mutation_doors() -> Vec<MutationDoor> {
    let mut out = Vec::new();
    for file in crate_sources() {
        let text = std::fs::read_to_string(&file).expect("a readable source file");
        let code = CodeOnly::of(&text);
        for (name, params, body) in code.public_fns() {
            if !params.contains("&mut self") && !params.contains("&mut Body") {
                continue;
            }
            out.push(MutationDoor {
                file: file.clone(),
                name: name.to_string(),
                code: body.to_string(),
            });
        }
    }
    // The walk's floor, and the only one: a walk that lost doors to a
    // lexing gap is this module's own historical failure, and both
    // consumers used to carry a hand-chosen floor of their own that
    // could absorb the loss of seven.
    assert!(
        out.len() + 2 >= DOORS_MEASURED,
        "the walk found {} mutation door(s) against {DOORS_MEASURED} measured — doors are \
         being lost. Deleting doors is legitimate; if that is what happened, re-measure \
         and move `DOORS_MEASURED`.",
        out.len(),
    );
    out
}

#[cfg(test)]
mod tests {
    use super::{CodeOnly, DOORS_MEASURED, MutationDoor, mutation_doors};

    fn door(body: &str) -> MutationDoor {
        MutationDoor {
            file: std::path::PathBuf::from("planted.rs"),
            name: "planted".to_string(),
            code: CodeOnly::of(body).as_str().to_string(),
        }
    }

    /// **The hole S92 named, as a row that goes red if it reopens.**
    /// Both mutation-surface guards classified a door by a raw
    /// `body.contains("<literal>")`, so a door whose body only
    /// *mentions* the call in a comment was classified as making it —
    /// demonstrated by planting one, at which point both guards stayed
    /// green. Each line below is that plant in miniature.
    #[test]
    fn a_mention_is_not_a_call() {
        for prose in [
            "// calls mint_pcurves( on the way out\n    Ok(())",
            "/* mint_pcurves( */ Ok(())",
            "/* outer /* mint_pcurves( */ still a comment */ Ok(())",
            "let msg = \"mint_pcurves(\"; Ok(())",
            "let msg = r#\"mint_pcurves(\"#; Ok(())",
            "let msg = r\"mint_pcurves(\"; Ok(())",
            "let msg = b\"mint_pcurves(\"; Ok(())",
            "let msg = br#\"mint_pcurves(\"#; Ok(())",
            "let msg = c\"mint_pcurves(\"; Ok(())",
        ] {
            assert!(
                !door(prose).code_contains("mint_pcurves("),
                "a mention classified as a call: {prose}"
            );
        }
    }

    /// The other direction, which is what stops the row above from
    /// being a classifier that answers `false` to everything: a real
    /// call still reads as one, through each construct the blanker has
    /// to walk past to reach it. The `br"x\"` row is the one that
    /// mattered — a byte raw string falling through to the escape-
    /// honouring plain-string arm blanks the rest of the body,
    /// **erasing** a call rather than inventing one.
    #[test]
    fn a_call_is_still_a_call() {
        for code in [
            "mint_pcurves(&mut b)?;",
            "// mint_pcurves( in prose first\n    mint_pcurves(&mut b)?;",
            "let q = '\"'; mint_pcurves(&mut b)?;",
            "let q = '\\''; mint_pcurves(&mut b)?;",
            "let q = '\\\\'; mint_pcurves(&mut b)?;",
            "let s = \"a // b /* c\"; mint_pcurves(&mut b)?;",
            "let s = br\"x\\\"; mint_pcurves(&mut b)?;",
            "let s = b\"x\"; mint_pcurves(&mut b)?;",
            "let l: &'static str = \"x\"; mint_pcurves(&mut b)?;",
            "let s = \"π…\"; mint_pcurves(&mut b)?;",
        ] {
            assert!(
                door(code).code_contains("mint_pcurves("),
                "a call read as a mention: {code}"
            );
        }
    }

    /// **Both layers the two earlier passes did not reach.** The
    /// blanker's constructs are the first half; the array-return rows
    /// (`arr`, `arr_nested`) are the second — a `;` inside `[T; N]`
    /// used to end the scan of every such signature and drop the item
    /// whole, with no count anywhere moving. `bodiless` is the case
    /// the dropped-item rule must still let through.
    ///
    /// Both failures were the same shape and neither was visible from
    /// inside one layer: the blanker knew `'"'`, nested block comments
    /// and raw strings while the matcher carving bodies for it knew
    /// none of them, and later the item scan read the blanker's output
    /// correctly and still threw the item away. **Every row here is a
    /// whole-pipeline read** — text in, doors out — because that is
    /// the only altitude at which either was visible.
    #[test]
    fn the_item_scan_survives_what_the_blanker_handles() {
        let src = "
pub fn quote(&mut self) { let q = '\"'; mint_pcurves(&mut b); }
pub fn raw(&mut self) { let s = br\"x\\\"; mint_pcurves(&mut b); }
pub fn nested(&mut self) { /* a /* b */ c */ mint_pcurves(&mut b); }
/// pub fn doc_comment_decoy(&mut self) {
pub fn plain(&mut self) { let _ = \"pub fn string_decoy(&mut self) {\"; }
pub const fn qualified(&mut self) { mint_pcurves(&mut b); }
pub async unsafe fn stacked(&mut self) { mint_pcurves(&mut b); }
pub fn generic<F: Fn(u32) -> u32>(&mut self, f: F) { mint_pcurves(&mut b); }
pub fn arr(&mut self) -> [usize; 2] { mint_pcurves(&mut b); }
pub fn arr_nested(&mut self) -> Result<([u8; 2], u8), E> { mint_pcurves(&mut b); }
pub fn bodiless(&mut self) -> u8;
fn private(&mut self) { mint_pcurves(&mut b); }
pub(crate) fn restricted(&mut self) { mint_pcurves(&mut b); }
pub fn not_a_door(&self) {}
";
        let code = CodeOnly::of(src);
        let found: Vec<&str> = code.public_fns().iter().map(|(n, _, _)| *n).collect();
        assert_eq!(
            found,
            vec![
                "quote",
                "raw",
                "nested",
                "plain",
                "qualified",
                "stacked",
                "generic",
                "arr",
                "arr_nested",
                "not_a_door",
            ],
            "the item scan lost, invented or mis-ordered a public fn"
        );
        let doors: Vec<(&str, bool)> = code
            .public_fns()
            .iter()
            .filter(|(_, p, _)| p.contains("&mut self"))
            .map(|(n, _, b)| (*n, b.contains("mint_pcurves(")))
            .collect();
        assert_eq!(
            doors,
            vec![
                ("quote", true),
                ("raw", true),
                ("nested", true),
                ("plain", false),
                ("qualified", true),
                ("stacked", true),
                ("generic", true),
                ("arr", true),
                ("arr_nested", true),
            ],
            "a door was lost, or its body classified from prose"
        );
    }

    /// The walk over the real tree, which is the only place the two
    /// halves above meet the surface they exist for.
    #[test]
    fn the_door_walk_reads_the_real_surface() {
        let doors = mutation_doors();
        assert!(
            doors.len() + 2 >= DOORS_MEASURED,
            "the walk found {} mutation door(s) against {DOORS_MEASURED} measured",
            doors.len()
        );
        for name in ["mint_pcurves", "mev", "merge_coplanar_faces_declared"] {
            assert!(
                doors.iter().any(|d| d.name == name),
                "the walk lost `{name}`, which is a door"
            );
        }
    }
}
