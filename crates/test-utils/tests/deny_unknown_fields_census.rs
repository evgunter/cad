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
//! - **Macro-generated declarations.** A `struct` or `enum` assembled
//!   inside a `macro_rules!` body, or by `paste!`/`concat_idents!`, is
//!   text to a lexer and a declaration only after expansion. An
//!   attribute reaching one through `$attr` is invisible here.
//! - **`#[cfg]`-gated declarations** are read exactly like any other,
//!   because this walks text: a site behind a feature nothing in CI
//!   enables is still counted, which is the fail-loud direction.
//! - **An enum discriminant whose value is a block** (`A = const { … }`)
//!   would read as a struct variant and answer [`Governed::NamedField`]
//!   for an enum that has no named field. That is the one false-green
//!   shape in the classifier and this tree holds none; a discriminant
//!   here is a bare path or integer.
//! - **Anything that is not a `.rs` file** — the two design pages that
//!   discuss the attribute in prose (`crates/profile/README.md`,
//!   `crates/verbs/README.md`) are outside this walk, as is every
//!   comment, since the classifier reads the code view alone.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::{Path, PathBuf};

use test_utils::source::{
    ItemBody, balanced_end, boundary_before, code_only, item_body, rust_sources,
};

/// The repository's own directories, skipped by NAME. Same rule, and
/// the same reason, as `reader_census.rs`: one walk from the root sees
/// every tracked `.rs` file, so a new top-level Rust tree is covered
/// the day it lands rather than the day someone remembers a roster.
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
    /// The reader could not resolve the site to a declaration. **This
    /// is a red, never a skip**: a walk that cannot parse a site and
    /// shrugs is the failure a census exists to prevent.
    Unreadable(&'static str),
}

/// The first offset in `code` at which `word` appears as a whole word.
fn word(code: &str, word: &str) -> Option<usize> {
    code.match_indices(word)
        .find(|(at, _)| {
            boundary_before(code, *at)
                && code[at + word.len()..]
                    .chars()
                    .next()
                    .is_none_or(|c| !c.is_alphanumeric() && c != '_')
        })
        .map(|(at, _)| at)
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
        while code[i..].chars().next().is_some_and(char::is_whitespace) {
            i += code[i..].chars().next().unwrap().len_utf8();
        }
        if !code[i..].starts_with("#[") {
            break;
        }
        match balanced_end(code, i + 1) {
            Some(end) => i = end + 1,
            None => return Governed::Unreadable("an attribute before the head never closes"),
        }
    }
    let rest = &code[i..];
    let limit = rest.find(['{', ';']).unwrap_or(rest.len());
    let head = &rest[..limit];
    let (kind, at_kind) = match (word(head, "struct"), word(head, "enum")) {
        (Some(s), None) => ("struct", s),
        (None, Some(e)) => ("enum", e),
        (Some(s), Some(e)) if s < e => ("struct", s),
        (Some(_), Some(e)) => ("enum", e),
        (None, None) => {
            return Governed::Unreadable("the attribute heads no `struct` or `enum` declaration");
        }
    };
    match item_body(code, i + at_kind) {
        // `struct X;` and `struct X(A, B);` both end at a `;` and both
        // have nothing named. An `enum` cannot, so one that does is
        // broken text and says so.
        ItemBody::Declaration(_) if kind == "struct" => Governed::NothingToDeny,
        ItemBody::Declaration(_) => Governed::Unreadable("an `enum` with no body"),
        ItemBody::Unterminated => Governed::Unreadable("the declaration's body never closes"),
        ItemBody::Body(body) => {
            let inner = &code[body.start + 1..body.end - 1];
            let named = if kind == "struct" {
                // A braced struct's members are all named, so the only
                // question is whether it has any.
                !inner.trim().is_empty()
            } else {
                // A variant with a brace is a struct variant, and one
                // is enough.
                inner.contains('{')
            };
            if named {
                Governed::NamedField
            } else {
                Governed::NothingToDeny
            }
        }
    }
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

/// The number of attribute sites in the tree. **A tripwire, hand-synced,
/// and it is not the guard.**
///
/// The guard is the row below, which reds when a site has no field to
/// deny. What a count adds is the direction that row cannot fail in: a
/// walk that stops SEEING — a changed spelling, a traversal that no
/// longer reaches `crates/`, a classifier that returns early — reports
/// agreement over the sites it can still read, and every site it can
/// still read agrees. PR 2501 measured exactly that shape.
///
/// It is an equality and not a floor on purpose. A floor catches a walk
/// that narrows to zero and misses one that narrows to most of the
/// tree, which is the failure that actually happened.
const ATTRIBUTE_SITES_TODAY: usize = 37;

/// **The invariant.** An attribute with nothing to deny is a guarantee
/// the code does not make, and the prose that reasons from one is the
/// cost.
#[test]
fn every_deny_unknown_fields_attribute_has_a_named_field_to_deny() {
    let root = repo_root();
    let offenders: Vec<String> = sites(&root)
        .into_iter()
        .filter_map(|(site, verdict)| match verdict {
            Governed::NamedField => None,
            Governed::NothingToDeny => Some(format!(
                "{site} — the declaration has no named field anywhere, so the attribute \
                 denies nothing"
            )),
            Governed::Unreadable(why) => Some(format!("{site} — unreadable: {why}")),
        })
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
/// is the silent direction and the one a census is built to close.
#[test]
fn the_walk_still_sees_every_attribute_site() {
    let found = sites(&repo_root());
    assert_eq!(
        found.len(),
        ATTRIBUTE_SITES_TODAY,
        "the walk found {} `{NEEDLE}` attribute sites and ATTRIBUTE_SITES_TODAY says \
         {ATTRIBUTE_SITES_TODAY}. If you added or removed one, re-sync the constant. If \
         you did not, this reader has gone blind and the row above is passing over the \
         sites it can no longer read. Found:\n{:#?}",
        found.len(),
        found.iter().map(|(s, _)| s).collect::<Vec<_>>()
    );
}

/// **The classifier, pinned shape by shape**, so that a reader which
/// stops telling the shapes apart reds on text it cannot blame on the
/// tree.
#[test]
fn the_classifier_answers_each_declaration_shape() {
    let cases: [(&str, Governed, &str); 8] = [
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
    ];
    for (what, want, decl) in cases {
        let src = format!("#[derive(Deserialize)]\n#[serde({NEEDLE})]\n{decl}\n");
        let code = code_only(&src);
        let at = code.find(NEEDLE).expect("the fixture carries the needle");
        assert_eq!(verdict(&code, at), want, "{what}: {decl}");
    }
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

/// The repository root, resolved from this crate's manifest.
fn repo_root() -> PathBuf {
    let root = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        // Canonical, so `..` is not a path COMPONENT: the skip in
        // `sites` reads components, and a relative one matches every
        // file in the tree at once — which looks exactly like a clean
        // walk.
        .canonicalize()
        .expect("the repository root resolves");
    assert!(
        root.join("Cargo.toml").is_file(),
        "{} is not the repository root",
        root.display()
    );
    root
}
