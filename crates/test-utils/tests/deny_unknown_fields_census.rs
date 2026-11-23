//! **Every `#[serde(deny_unknown_fields)]` in the tree sits on a
//! declaration with a NAMED FIELD to deny.**
//!
//! The attribute governs unknown *named fields*. On a unit enum, a
//! tuple-variant-only enum, a tuple struct or a unit struct there is no
//! named field anywhere, so it compiles, reads as a guarantee, and does
//! nothing — and the cost is not the attribute but the prose that
//! reasons from it: a doc that credits it with a refusal actually
//! performed by the version door, or by an externally-tagged enum's
//! unconditional rejection of a variant name it has no arm for.
//!
//! An externally-tagged enum refuses an unknown VARIANT with or without
//! it; serde's derived reader refuses a DUPLICATE field with or without
//! it. What it adds, and the only thing it adds, is the refusal of a
//! named field the declaration does not have.
//!
//! # No exemptions, deliberately
//!
//! An inert attribute kept as a habit-guard against a future
//! named-field variant is a defensible thing to want, and it is refused
//! here anyway: an exemption is a hand-written list of declarations,
//! which is a fresh instance of the class this census belongs to. The
//! habit-guard is the named field itself — add the attribute with the
//! variant that needs it.
//!
//! # What this cannot see
//!
//! - **The OTHER direction, entirely.** This reds on an attribute with
//!   no field to deny; nothing reds on a named field with no attribute,
//!   which is the same silence pointed the other way and the defect
//!   `MatePrimitive::PlanarRest` actually has. The complement wants an
//!   instrument of its own and a verdict key this one does not need —
//!   `work/bind/census-sees-an-inert-attribute-but-not-a-missing-one.md`.
//! - **Macro-generated declarations.** A `struct` or `enum` assembled
//!   inside a `macro_rules!` body, or by `paste!`/`concat_idents!`, is
//!   text to a lexer and a declaration only after expansion. An
//!   attribute reaching one through `$attr` is invisible here.
//! - **`#[cfg]`-gated declarations** are read exactly like any other,
//!   because this walks text: a site behind a feature nothing in CI
//!   enables is still counted, which is the fail-loud direction.
//! - **A `Serialize`-only declaration.** `deny_unknown_fields` governs
//!   DESERIALIZATION, so on a type that derives only `Serialize` it is
//!   as inert as one with no named field — and this census reads the
//!   declaration's shape, not its derives, so such a site passes.
//!   Every site in this tree derives `Deserialize`, which is why the
//!   hole is future and not present.
//! - **A const-generic default that is a block**
//!   (`struct S<const N: usize = { 2 }>(…)`) is REFUSED rather than
//!   classified: the generic list's `>` cannot be found by a reader
//!   that takes the first unbracketed `{` as the item's body, so the
//!   site answers [`Governed::Unreadable`] and reds. That is the
//!   fail-loud direction and the deliberate answer to a shape whose
//!   classification would otherwise be a guess.
//! - **A `>` used as a COMPARISON inside a const expression** in an
//!   enum body can unbalance the depth count that splits the body into
//!   variants, and a variant split in the wrong place can be read
//!   either way. Angle brackets are not a bracket language; this is
//!   [`test_utils::source::top_level_split`]'s own stated residue
//!   reaching this reader.
//! - **Anything that is not a `.rs` file** — the two design pages that
//!   discuss the attribute in prose (`crates/profile/README.md`,
//!   `crates/verbs/README.md`) are outside this walk, as is every
//!   comment, since the classifier reads the code view alone.
//!
//! **What is NOT on this list, because the classifier answers it:** a
//! brace that reaches an enum body without a named field behind it — a
//! discriminant (`A = { 1 }`), a tuple variant's array length
//! (`A([u8; { 2 }])`), a variant-level attribute. Each is a shape a
//! reader that asks "does the body contain a `{`" calls a struct
//! variant, and each has its row in
//! [`the_classifier_answers_each_declaration_shape`].

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::path::Path;

use test_utils::source::{
    ItemBody, angle_end, balanced_end, code_only, ident, item_body, repo_root, rust_sources,
    skip_ws, top_level_split, word_at,
};

/// The repository's own directories, skipped by NAME. Same rule, and
/// the same reason, as `reader_census.rs`: one walk from the root
/// reaches every `.rs` file in the WORKING TREE — tracked or not, so a
/// scratch file counts toward the census and a new top-level Rust tree
/// is covered the day it lands rather than the day someone remembers a
/// roster.
const SKIPPED_DIRS: [&str; 1] = ["target"];

const NEEDLE: &str = "deny_unknown_fields";

/// What the declaration under an attribute site offers it.
#[derive(Debug, PartialEq, Eq)]
enum Governed {
    /// A named field somewhere — a braced struct, or an enum with at
    /// least one struct variant. The attribute does work.
    NamedField,
    /// A unit struct, a tuple struct, or an enum all of whose variants
    /// are unit or tuple variants. The attribute is inert.
    NothingToDeny,
    /// A declaration serde deserializes through another type
    /// (`#[serde(try_from = …)]` or `#[serde(from = …)]`): the derived
    /// reader is that type's, so this one's container attributes,
    /// `deny_unknown_fields` among them, are never read. The attribute
    /// is inert; the other type's is the one that refuses.
    Delegated,
    /// The reader could not resolve the site to a declaration. **This
    /// is a red, never a skip**: a walk that cannot parse a site and
    /// shrugs is the failure a census exists to prevent.
    Unreadable(&'static str),
}

/// The first offset in `code` at which `word` appears as a whole word.
fn word(code: &str, word: &str) -> Option<usize> {
    code.match_indices(word)
        .find(|(at, _)| word_at(code, *at, word))
        .map(|(at, _)| at)
}

/// Does any variant in an enum body carry a `{ … }` of its own?
///
/// **A brace anywhere in the body is not the question.** A tuple
/// variant's array length (`A([u8; { 2 }])`), an enum discriminant
/// (`A = { 1 }`) and a variant-level attribute each put a brace inside
/// the body with no named field in sight, and a reader that answers
/// "there is a `{`" calls all three struct variants. The question is
/// per variant, and it is whether the token immediately after the
/// variant's own name is a `{`.
///
/// `None` when a variant's attribute never closes — broken text, which
/// is a refusal and not an answer.
fn has_struct_variant(inner: &str) -> Option<bool> {
    for range in top_level_split(inner, ',') {
        let variant = &inner[range];
        let mut i = 0;
        loop {
            i = skip_ws(variant, i);
            if !variant[i..].starts_with("#[") {
                break;
            }
            i = balanced_end(variant, i + 1)? + 1;
        }
        i = skip_ws(variant, i);
        let name = ident(variant, i);
        if name.is_empty() {
            // A trailing separator, or an empty body.
            continue;
        }
        i = skip_ws(variant, i + name.len());
        if variant[i..].starts_with('{') {
            return Some(true);
        }
    }
    Some(false)
}

/// What the declaration carrying the attribute at `at` offers it.
///
/// `code` is a [`code_only`] view, which is the precondition every
/// bracket operation below rests on.
fn verdict(code: &str, at: usize) -> Governed {
    let Some(hash) = code[..at].rfind("#[") else {
        return Governed::Unreadable("no `#[` opens an attribute before the needle");
    };
    let Some(close) = balanced_end(code, hash + 1) else {
        return Governed::Unreadable("the attribute's `[` never closes");
    };
    if close < at {
        return Governed::Unreadable("the needle lies outside the attribute before it");
    }
    // Everything between the attribute and the declaration head is
    // whitespace or further attributes — `doc.rs`'s `Doc<P>` carries a
    // multi-line `#[serde(bound(…))]` there, and a walk that took the
    // next line as the head would read the wrong declaration.
    let mut i = close + 1;
    loop {
        i = skip_ws(code, i);
        if !code[i..].starts_with("#[") {
            break;
        }
        match balanced_end(code, i + 1) {
            Some(end) => i = end + 1,
            None => return Governed::Unreadable("an attribute before the head never closes"),
        }
    }
    // The declaration's whole attribute block lies between the previous
    // item's end and the head, so a delegating attribute on either side
    // of the needle's is in that span.
    let block = code[..hash].rfind([';', '}']).map_or(0, |end| end + 1);
    if delegates(&code[block..i]) {
        return Governed::Delegated;
    }
    // The head is read TOKEN BY TOKEN from here — a visibility, the
    // keyword, the name, then the generic parameter list — rather than
    // by searching a slice cut at the first `{`. The cut is what reads
    // a const-generic default's block (`struct S<const N: usize = { 2 }>`)
    // as the item's body, and every bracket operation below inherits
    // that mistake.
    if ident(code, i) == "pub" {
        i = skip_ws(code, i + "pub".len());
        if code[i..].starts_with('(') {
            match balanced_end(code, i) {
                Some(end) => i = skip_ws(code, end + 1),
                None => return Governed::Unreadable("a visibility whose `(` never closes"),
            }
        }
    }
    let kind = ident(code, i);
    if kind != "struct" && kind != "enum" {
        return Governed::Unreadable("the attribute heads no `struct` or `enum` declaration");
    }
    i = skip_ws(code, i + kind.len());
    i += ident(code, i).len();
    i = skip_ws(code, i);
    if code[i..].starts_with('<') {
        match angle_end(code, i) {
            Some(gt) => i = gt + 1,
            // `angle_end` reads the first `{` outside every round and
            // square bracket as the item's body, so a const-generic
            // default that is a block leaves it with no answer. Fail
            // LOUD: a site this reader cannot place is a red, and a
            // guess here is a false green on an inert attribute.
            None => {
                return Governed::Unreadable("a generic parameter list whose `>` is not found");
            }
        }
    }
    match item_body(code, i) {
        // `struct X;` and `struct X(A, B);` both end at a `;` and both
        // have nothing named. An `enum` cannot, so one that does is
        // broken text and says so.
        ItemBody::Declaration(_) if kind == "struct" => Governed::NothingToDeny,
        ItemBody::Declaration(_) => Governed::Unreadable("an `enum` with no body"),
        ItemBody::Unterminated => Governed::Unreadable("the declaration's body never closes"),
        ItemBody::Body(body) => {
            let inner = &code[body.start + 1..body.end - 1];
            if kind == "struct" {
                // A braced struct's members are all named, so the only
                // question is whether it has any.
                if inner.trim().is_empty() {
                    Governed::NothingToDeny
                } else {
                    Governed::NamedField
                }
            } else {
                match has_struct_variant(inner) {
                    Some(true) => Governed::NamedField,
                    Some(false) => Governed::NothingToDeny,
                    None => Governed::Unreadable("a variant attribute that never closes"),
                }
            }
        }
    }
}

/// Whether an attribute block names a serde conversion door
/// (`try_from = …` or `from = …`), which hands deserialization to
/// another type.
fn delegates(block: &str) -> bool {
    ["try_from", "from"].iter().any(|key| {
        let mut rest = block;
        while let Some(at) = word(rest, key) {
            let after = skip_ws(rest, at + key.len());
            if rest[after..].starts_with('=') {
                return true;
            }
            rest = &rest[at + key.len()..];
        }
        false
    })
}

/// Every attribute site in the tree: `path:line`, and its verdict.
fn sites(root: &Path) -> Vec<(String, Governed)> {
    let mut out = Vec::new();
    for path in rust_sources(root) {
        let relative = path
            .strip_prefix(root)
            .expect("a walked file lies under the root");
        if relative.components().any(|c| {
            let c = c.as_os_str().to_string_lossy();
            SKIPPED_DIRS.contains(&c.as_ref()) || c.starts_with('.')
        }) {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("a readable source file");
        if !text.contains(NEEDLE) {
            continue;
        }
        // The CODE view: the needle in a doc-comment is prose about the
        // attribute, not an attribute.
        let code = code_only(&text);
        let mut from = 0;
        // A WHOLE word, both ends. `mod deny_unknown_fields_census;`
        // in the aggregator one directory up has the needle as a
        // PREFIX of an identifier, and a prefix match sends the reader
        // looking for a declaration that is not there.
        while let Some(off) = word(&code[from..], NEEDLE) {
            let at = from + off;
            from = at + NEEDLE.len();
            out.push((
                format!(
                    "{}:{}",
                    relative.to_string_lossy().replace('\\', "/"),
                    test_utils::source::line(&code, at)
                ),
                verdict(&code, at),
            ));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// One site's reading by the census: `Some` when it is an offence, and
/// the sentence a reader acts on.
///
/// Separate from the row below because the filter is half the guard —
/// a verdict nothing reports is a verdict nothing enforces, and
/// [`the_reader_refuses_rather_than_guessing_and_says_which_way`] pins
/// this half where the tree offers no site to pin it with.
fn offence(site: &str, verdict: &Governed) -> Option<String> {
    match verdict {
        Governed::NamedField => None,
        Governed::NothingToDeny => Some(format!(
            "{site} — the declaration has no named field anywhere, so the attribute \
             denies nothing"
        )),
        Governed::Delegated => Some(format!(
            "{site} — the declaration deserializes through another type (`try_from` or \
             `from`), whose reader runs instead, so the attribute denies nothing"
        )),
        Governed::Unreadable(why) => Some(format!("{site} — unreadable: {why}")),
    }
}

/// The files whose one inert site is pinned red: each may hold at most
/// one site read as having nothing to deny, and a repair that leaves it
/// none is no red.
const PINNED_INERT: [&str; 2] = [
    "crates/editor-core/src/names/nest_reference.rs",
    "crates/editor-core/src/names/role.rs",
];

/// **The invariant.** An attribute with nothing to deny is a guarantee
/// the code does not make, and the prose that reasons from one is the
/// cost.
#[test]
fn every_deny_unknown_fields_attribute_has_a_named_field_to_deny() {
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
    // Pinned red: the two `Qualifier` enums lost their one named variant
    // and still carry the attribute
    // (`work/bind/main-red-inert-deny-on-the-qualifier-enums`). Each
    // named file may hold one site read as having nothing to deny; an
    // unreadable site is never pinned.
    let found = sites(&root);
    for file in PINNED_INERT {
        let inert = found
            .iter()
            .filter(|(site, verdict)| {
                site.starts_with(&format!("{file}:")) && *verdict == Governed::NothingToDeny
            })
            .count();
        assert!(
            inert <= 1,
            "{file}: a second inert site beside the pinned one"
        );
    }
    let offenders: Vec<String> = found
        .into_iter()
        .filter(|(site, verdict)| {
            !(*verdict == Governed::NothingToDeny
                && PINNED_INERT
                    .iter()
                    .any(|file| site.starts_with(&format!("{file}:"))))
        })
        .filter_map(|(site, verdict)| offence(&site, &verdict))
        .collect();
    assert!(
        offenders.is_empty(),
        "`#[serde(deny_unknown_fields)]` needs a NAMED field to deny. These sites have \
         none, so the attribute is inert — remove it, and say what actually refuses \
         wherever a doc reasons from it:\n{offenders:#?}"
    );
}

/// **The reader's own guard: a walk that stops seeing reds here.**
///
/// Without this row the census above passes by finding nothing, which
/// is the silent direction and the one a census is built to close. The
/// walk's per-file tally is compared with a second, cruder count that
/// shares none of its reading: its own directory walk with its own skip
/// list (`target`, `.git`), over the raw text, counting every line that
/// opens an attribute (`#[`) and holds `deny_unknown_fields)` or
/// `deny_unknown_fields,` as a plain substring — no [`SKIPPED_DIRS`],
/// [`NEEDLE`], `word` or lexer. A walk that stops reaching a file, a
/// skip that swallows the tree, or a reader that stops finding the word differs
/// from it, wherever the sites are and however many there are.
#[test]
fn the_walk_still_sees_every_attribute_site() {
    let root = repo_root(env!("CARGO_MANIFEST_DIR"));
    let mut walk: BTreeMap<String, usize> = BTreeMap::new();
    for (site, _) in sites(&root) {
        let file = site.rsplit_once(':').expect("a `path:line` site").0;
        *walk.entry(file.to_string()).or_default() += 1;
    }
    let crude = crude_attribute_lines(&root);
    assert!(
        !crude.is_empty(),
        "the crude count found no `{NEEDLE}` attribute anywhere: it is the one that has \
         gone blind"
    );
    assert_eq!(
        walk, crude,
        "the census walk's per-file tally of `{NEEDLE}` sites (left) differs from a raw \
         count of attribute lines naming it (right). A file on the right only is one this \
         reader has gone blind to; an attribute split across lines reads on the left only."
    );
}

/// Every `.rs` file under `root` holding an attribute line that names
/// `deny_unknown_fields`, with how many.
fn crude_attribute_lines(root: &Path) -> BTreeMap<String, usize> {
    fn walk(root: &Path, dir: &Path, out: &mut BTreeMap<String, usize>) {
        for entry in std::fs::read_dir(dir)
            .expect("a readable directory")
            .flatten()
        {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            if path.is_dir() {
                // Its own skip list, spelled here and nowhere else: a
                // narrowing of the census walk's skips cannot narrow this.
                if name != "target" && name != ".git" {
                    walk(root, &path, out);
                }
            } else if name.ends_with(".rs") {
                let text = std::fs::read_to_string(&path).expect("a readable source file");
                let n = text
                    .lines()
                    .filter(|l| {
                        l.trim_start().starts_with("#[")
                            && (l.contains("deny_unknown_fields)")
                                || l.contains("deny_unknown_fields,"))
                    })
                    .count();
                if n > 0 {
                    let rel = path.strip_prefix(root).expect("under the root");
                    out.insert(rel.to_string_lossy().replace('\\', "/"), n);
                }
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(root, root, &mut out);
    out
}

/// **The classifier, pinned shape by shape**, so that a reader which
/// stops telling the shapes apart reds on text it cannot blame on the
/// tree.
#[test]
fn the_classifier_answers_each_declaration_shape() {
    let cases: [(&str, Governed, &str); 19] = [
        ("unit struct", Governed::NothingToDeny, "pub struct U;"),
        (
            "tuple struct",
            Governed::NothingToDeny,
            "pub struct T(pub String);",
        ),
        (
            "named struct",
            Governed::NamedField,
            "pub struct N { a: u8 }",
        ),
        ("unit enum", Governed::NothingToDeny, "pub enum E { A, B }"),
        (
            "tuple-variant enum",
            Governed::NothingToDeny,
            "enum E { A(u8), B(Vec<(u8, u8)>) }",
        ),
        (
            "struct-variant enum",
            Governed::NamedField,
            "enum E { A(u8), B { w: f64 } }",
        ),
        (
            "a head behind a second attribute",
            Governed::NamedField,
            "#[serde(bound(\n    de = \"P: X\"\n))]\npub struct D<P> { p: P }",
        ),
        (
            "a tuple struct whose field type carries a `;`",
            Governed::NothingToDeny,
            "pub struct A(pub [f64; 3]);",
        ),
        // The four ways a brace reaches an enum body or an item head
        // without a named field behind it. Each of the first three
        // read as a struct variant to a classifier that asks whether
        // the body contains a `{`; the fourth puts the brace in front
        // of the body altogether.
        (
            "a tuple variant whose array length is a block",
            Governed::NothingToDeny,
            "enum E { A([u8; { 2 }]), B }",
        ),
        (
            "an enum discriminant that is a block",
            Governed::NothingToDeny,
            "enum E { A = { 1 }, B = 2 }",
        ),
        (
            "a variant-level attribute carrying a brace",
            Governed::NothingToDeny,
            "enum E { #[cfg_attr(test, foo({}))] A(u8), B }",
        ),
        (
            "a const-generic default that is a block",
            Governed::Unreadable("a generic parameter list whose `>` is not found"),
            "pub struct S<const N: usize = { 2 }>(pub [u8; N]);",
        ),
        (
            "a visibility with a restriction path",
            Governed::NamedField,
            "pub(crate) struct R { a: u8 }",
        ),
        (
            "a named struct behind a where clause",
            Governed::NamedField,
            "pub struct W<T> where T: Copy { a: T }",
        ),
        (
            "a struct variant under a raw identifier",
            Governed::NamedField,
            "enum E { A(u8), r#type { w: f64 } }",
        ),
        (
            "a tuple variant under a raw identifier",
            Governed::NothingToDeny,
            "enum E { r#fn(u8), B }",
        ),
        (
            "a named struct read through `try_from`",
            Governed::Delegated,
            "#[serde(try_from = \"W\")]\npub struct N { a: u8 }",
        ),
        (
            "a struct-variant enum read through `from`",
            Governed::Delegated,
            "#[serde(from = \"W\", into = \"W\")]\nenum E { A { w: f64 } }",
        ),
        (
            "a serialize-side `into` alone",
            Governed::NamedField,
            "#[serde(into = \"W\")]\npub struct N { a: u8 }",
        ),
    ];
    for (what, want, decl) in cases {
        let src = format!("#[derive(Deserialize)]\n#[serde({NEEDLE})]\n{decl}\n");
        let code = code_only(&src);
        let at = word(&code, NEEDLE).expect("the fixture carries the needle");
        assert_eq!(verdict(&code, at), want, "{what}: {decl}");
    }
    // A conversion door on EITHER side of the needle's attribute
    // delegates: the block is read back to the previous item's end.
    let before = code_only(&format!(
        "fn f() {{}}\n#[serde(try_from = \"W\")]\n#[serde({NEEDLE})]\npub struct N {{ a: u8 }}\n"
    ));
    let at = word(&before, NEEDLE).expect("the fixture carries the needle");
    assert_eq!(
        verdict(&before, at),
        Governed::Delegated,
        "a `try_from` attribute above the needle's"
    );
    // The needle in prose is not an attribute site at all, which is the
    // half of the population the code view removes.
    let prose = code_only(&format!(
        "/// `{NEEDLE}` is what refuses.\npub struct N {{ a: u8 }}\n"
    ));
    assert!(
        !prose.contains(NEEDLE),
        "a doc-comment mention survives the code view"
    );
    // The needle as a PREFIX of an identifier is not a site either —
    // `tests/all.rs` mounts this suite by its module name, which begins
    // with it.
    assert!(
        word(&code_only(&format!("mod {NEEDLE}_census;\n")), NEEDLE).is_none(),
        "an identifier the needle merely prefixes reads as a site"
    );
}

/// **Every way the reader refuses, with text that provokes it.**
///
/// [`Governed::Unreadable`] is the arm that carries the census's
/// fail-loud posture, and it is the arm the tree produces none of — so
/// without these rows a regression that turned any refusal into a
/// `continue`, or into [`Governed::NamedField`], leaves every other row
/// in this file green. One fixture per reason, so the row that goes red
/// names the arm that moved.
#[test]
fn the_reader_refuses_rather_than_guessing_and_says_which_way() {
    // Whole sources, not declarations: every row here is a shape the
    // wrapper the row above uses could not express.
    let cases: [(&str, &str); 10] = [
        (
            "no `#[` opens an attribute before the needle",
            "pub fn f() { let deny_unknown_fields = 1; }",
        ),
        (
            "the attribute's `[` never closes",
            "#[serde(deny_unknown_fields)\npub struct N { a: u8 }",
        ),
        (
            "the needle lies outside the attribute before it",
            "#[derive(Deserialize)]\npub struct N { deny_unknown_fields: u8 }",
        ),
        (
            "an attribute before the head never closes",
            "#[serde(deny_unknown_fields)]\n#[derive(\npub struct N { a: u8 }",
        ),
        (
            "a visibility whose `(` never closes",
            "#[serde(deny_unknown_fields)]\npub(crate struct N { a: u8 }",
        ),
        (
            "the attribute heads no `struct` or `enum` declaration",
            "#[serde(deny_unknown_fields)]\npub fn f() {}",
        ),
        (
            "a generic parameter list whose `>` is not found",
            "#[serde(deny_unknown_fields)]\npub struct S<const N: usize = { 2 }>(pub [u8; N]);",
        ),
        (
            "an `enum` with no body",
            "#[serde(deny_unknown_fields)]\npub enum E;",
        ),
        (
            "the declaration's body never closes",
            "#[serde(deny_unknown_fields)]\npub struct N { a: u8",
        ),
        // Two arrows drive the variant split's depth count to zero
        // inside the attribute, so the split lands mid-attribute and
        // the fragment's `#[` has no `]`. This is the `>`-is-not-a-
        // bracket residue the header names, reaching the reader as a
        // refusal rather than as a wrong answer.
        (
            "a variant attribute that never closes",
            "#[serde(deny_unknown_fields)]\npub enum E { #[x(a -> b -> c, d)] A(u8), B }",
        ),
    ];
    for (why, src) in cases {
        let code = code_only(src);
        let at = word(&code, NEEDLE).expect("the fixture carries the needle");
        assert_eq!(verdict(&code, at), Governed::Unreadable(why), "{src}");
    }
    // And a refusal is what the census REPORTS. The arm above decides
    // the verdict; this decides whether the verdict reaches anyone,
    // and a `None` here would be the silent skip in its other half.
    assert!(
        offence("x.rs:1", &Governed::Unreadable("why")).is_some(),
        "a site the reader cannot place must not pass as governed"
    );
    assert!(offence("x.rs:1", &Governed::NothingToDeny).is_some());
    assert!(offence("x.rs:1", &Governed::NamedField).is_none());
}
