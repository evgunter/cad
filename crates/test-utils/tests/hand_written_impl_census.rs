//! **A hand-written `Debug` or `PartialEq` that reads a field by name
//! destructures `Self`.**
//!
//! A walk that names its value's fields by hand has no compile-time tie
//! to the declaration: a field added to the struct is silently absent
//! from every dump and silently outside equality. Destructuring `Self`
//! exhaustively buys the tie — a new field is an E0027 unbound-pattern
//! error — and a field the walk will not carry binds to `_`, which
//! makes the omission a decision a reader can see and the compiler
//! still forces.
//!
//! # Why this is a reader when the per-field question is not
//!
//! **The per-field question is compiler-known and is not asked here.**
//! Once an impl destructures, E0027 is a stronger guard than any
//! census: it needs no traversal, it cannot go blind, and it names the
//! field. A census re-asking it would be a hand-maintained reader
//! standing in for a check the language performs.
//!
//! **The ARRIVAL question is not compiler-known**, and that is the
//! whole of this file. Nothing in the language, and nothing in this
//! tree before this row, reds when a NEW hand-listed walk lands without
//! the destructure. Four such impls landed in nine days and no count
//! anywhere moved
//! (`work/census/hand-listed-debug-censuses-in-geom-core-geom-and-topo.md`).
//!
//! # The question, and why it is decidable from the body alone
//!
//! A classifier that had to resolve each impl's self type to its
//! declaration would need the type graph, across crates, from text.
//! **It does not need to**, because the defect is visible in the body:
//! an impl that reads `self.<name>` is reading a field by name, and one
//! that destructures `Self` is not. Three shapes therefore answer
//! themselves rather than needing a roster:
//!
//! - a newtype reads `self.0` — a tuple index, no census to fall
//!   behind, and the declaration cannot grow a *named* field without
//!   the impl being rewritten anyway;
//! - an enum walk reads `match self` or `match (self, other)`, whose
//!   arms the compiler already holds exhaustive (E0004 for a new
//!   variant, E0027 inside a struct-variant pattern);
//! - a destructured walk reads its bindings, never `self.<name>`.
//!
//! # What this cannot see
//!
//! - **A field read through a METHOD.** `self.origin()` and
//!   `self.bit_eq(other)` are calls, and no reader can tell a getter
//!   from any other call without resolving it. A hand-list moved one
//!   level down into a helper is invisible here, and
//!   `crates/profile/src/lib.rs`'s `SketchPlane` is the live instance —
//!   `work/census/hand-listed-partialeq-siblings-outside-the-census-debug-fence.md`.
//! - **An enum arm with a catch-all.** `_ => false` is exhaustive to
//!   the compiler and silent about a new variant, which is the same
//!   defect one level up from this one. `topo`'s `CensusSubject` is the
//!   live instance —
//!   `work/topo/censussubject-eq-answers-false-for-a-new-variant-against-itself.md`.
//! - **A `Self { … }` struct LITERAL** in such a body would read as a
//!   destructure. No `fmt` or `eq` in this tree builds one, and the
//!   shape is pinned in [`the_classifier_answers_each_impl_shape`] so a
//!   future one is a known false green rather than an unknown.
//! - **An impl whose `Debug`, `PartialEq` or `for` arrives from a macro
//!   METAVARIABLE.** A `macro_rules!` body is text and is read like any
//!   other — `crates/geom/src/curves/nurbs.rs`'s `nurbs_curve!` is
//!   classified once here and expands twice — but an impl assembled
//!   from `$trait` fragments is invisible until expansion.
//! - **`#[derive]`d impls**, entirely and deliberately: a derive IS the
//!   tie to the declaration.
//! - **Anything that is not a `.rs` file**, and every comment, since
//!   the classifier reads the code view alone. `#[cfg]`-gated code is
//!   read exactly like any other, which is the fail-loud direction.
//!
//! **What is NOT on that list, because the classifier answers it:** a
//! `Self { .. }` pattern with a rest arm, which is a destructure that
//! is not exhaustive and reds here; `impl<P: PartialEq> Doc<P>`, an
//! inherent impl whose BOUND names the trait; and `self.0.name`, a
//! field read through a tuple index. Each has its row in
//! [`the_classifier_answers_each_impl_shape`].

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::{Path, PathBuf};

use test_utils::source::{
    ItemBody, angle_end, balanced_end, boundary_after, boundary_before, code_only, item_body,
    rust_sources,
};

/// The repository's own directories, skipped by NAME. Same rule, and
/// the same reason, as `reader_census.rs` and
/// `deny_unknown_fields_census.rs`: one walk from the root reaches
/// every `.rs` file in the working tree, so a new top-level Rust tree
/// is covered the day it lands rather than the day someone remembers a
/// roster.
const SKIPPED_DIRS: [&str; 1] = ["target"];

/// The two traits whose hand-written impls carry a census of the
/// declaration. **Both, not just `Debug`**: the repair is the same
/// destructure, and an `Eq` that misses a field answers *wrong* where a
/// `Debug` that misses one only misleads.
const TRAITS: [&str; 2] = ["Debug", "PartialEq"];

/// What an impl body does about its declaration.
#[derive(Debug, PartialEq, Eq)]
enum Verdict {
    /// Destructures `Self` exhaustively. The compiler holds it from
    /// here on; this census has nothing further to say about it.
    Destructured,
    /// Reads no field by name — a newtype's `self.0`, an enum's
    /// `match self`, a delegation. Nothing to fall behind.
    NoFieldRead,
    /// Reads `self.<name>` with no exhaustive `Self` pattern: the
    /// defect. The payload is the first field read, so the message
    /// names something an author can find.
    HandListed(String),
    /// Destructures with a REST pattern (`Self { a, .. }`), which is a
    /// destructure that is not exhaustive and buys nothing. Named apart
    /// from [`Verdict::HandListed`] because the repair is different:
    /// drop the `..` and bind what it hid.
    RestPattern,
    /// The reader could not resolve the impl to a body. **A red, never
    /// a skip**: a walk that cannot parse a site and shrugs is the
    /// failure a census exists to prevent.
    Unreadable(&'static str),
}

/// One hand-written impl, as the walk found it.
#[derive(Debug)]
struct Site {
    /// Path relative to the repository root, `/`-separated.
    path: String,
    /// The line the `impl` keyword is on.
    line: usize,
    /// The trait, as [`TRAITS`] spells it.
    trait_name: &'static str,
    verdict: Verdict,
}

/// **The sites this census knows are hand-listed and does not red on**,
/// each with the row that owns the repair.
///
/// Every entry is a SECOND FINDING stacked on the first, never an
/// exemption — the shape `reader_census.rs` calls `Unconverted` and for
/// the same reason. They sit outside the fence of the unit that built
/// this census, which swept the types whose `Debug` it touched, so they
/// are filed rather than swept and this constant is what keeps them
/// from going quiet in the meantime.
///
/// **It is also this walk's per-file sight anchor.** An entry that
/// matches no live site reds — so a reader that goes blind to
/// `clearance.rs` cannot report agreement over the files it can still
/// read.
///
/// **Keyed by `(path, trait)` and not by path alone.** A file may hold
/// a hand-listed `PartialEq` beside a `Debug` that is already held to
/// its declaration, and a suppression covering the whole file would
/// take the second with the first — a suppression that grows silently
/// is the shape this census exists to refuse. Not keyed by LINE, which
/// rots on every edit above it.
const KNOWN_HAND_LISTED: [(&str, &str, &str); 5] = [
    (
        "crates/editor-core/src/clearance.rs",
        "PartialEq",
        "work/shell/geometrywitness-eq-ignores-the-two-chart-axes-its-uv-fields-are-stated-in.md",
    ),
    (
        "crates/editor-core/src/expr.rs",
        "PartialEq",
        "work/census/hand-listed-partialeq-siblings-outside-the-census-debug-fence.md",
    ),
    (
        "crates/editor-core/src/mate/coset.rs",
        "PartialEq",
        "work/census/hand-listed-partialeq-siblings-outside-the-census-debug-fence.md",
    ),
    (
        "crates/editor-core/src/program.rs",
        "PartialEq",
        "work/census/hand-listed-partialeq-siblings-outside-the-census-debug-fence.md",
    ),
    // Ruled out of CENSUS-DEBUG's hit list on the terminator — it uses
    // `write!`, not `debug_struct(…).finish()` — and in the class by
    // THIS census's question, which is about the tie to the
    // declaration rather than about what the renderer claims. It
    // renders in braced struct shape and reads `self.runs` by name.
    (
        "crates/topo/src/props.rs",
        "Debug",
        "work/census/hand-listed-partialeq-siblings-outside-the-census-debug-fence.md",
    ),
];

/// **Every file holding a hand-written `Debug` or `PartialEq`** —
/// sorted, hand-synced, and **it is not the guard**.
///
/// The guard is [`every_hand_written_walk_is_held_to_its_declaration`],
/// which reds when a walk reads a field by name. What this adds is the
/// direction that row cannot fail in: a walk that stops SEEING — a
/// changed spelling, a traversal that no longer reaches a crate, a
/// classifier that returns early — reports agreement over the sites it
/// can still read, and every site it can still read agrees.
///
/// **A set of paths rather than a tally of impls, and that is the whole
/// of the choice.** Per-file sight is what catches a reader gone blind
/// to one crate while another grew; a per-impl count would catch a
/// PARTIAL blindness inside a file as well, and is refused because this
/// census's subject is arrival and a compliant arrival should cost one
/// line, not a re-count. **The residue, stated:** a blindness that
/// hides some but not all of one file's impls moves nothing here, and
/// nothing else would name it.
///
/// **A new line is the arrival this row exists to detect.** Adding one
/// is a deliberate act and the message below says which acts are
/// honest.
const IMPL_FILES_TODAY: [&str; 22] = [
    "crates/editor-core/src/clearance.rs",
    "crates/editor-core/src/eval/mod.rs",
    "crates/editor-core/src/expr.rs",
    "crates/editor-core/src/mate/coset.rs",
    "crates/editor-core/src/meta/mod.rs",
    "crates/editor-core/src/names/role.rs",
    "crates/editor-core/src/names/table.rs",
    "crates/editor-core/src/program.rs",
    "crates/editor-core/src/resolve/pick.rs",
    "crates/geom-core/src/spline/hull.rs",
    "crates/geom-core/src/spline/knots.rs",
    "crates/geom-core/src/sym/memo.rs",
    "crates/geom/src/curves/nurbs.rs",
    "crates/geom/src/surfaces/nurbs.rs",
    "crates/mesh/src/memo.rs",
    "crates/profile/src/lib.rs",
    "crates/topo/src/param_source.rs",
    "crates/topo/src/props.rs",
    "crates/topo/src/validate.rs",
    "crates/viewer/src/camera.rs",
    "crates/viewer/src/pickcache.rs",
    "crates/viewer/src/session.rs",
];

/// The repository root: this crate's directory, two levels up.
fn repo_root() -> PathBuf {
    let root = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        // Canonical, so `..` is not a path COMPONENT: the skip below
        // reads components.
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
fn skip_ws(code: &str, mut at: usize) -> usize {
    while let Some(c) = code[at..].chars().next() {
        if !c.is_whitespace() {
            break;
        }
        at += c.len_utf8();
    }
    at
}

/// Whether `word` sits at `at` as a whole word.
fn is_word_at(code: &str, at: usize, word: &str) -> bool {
    code[at..].starts_with(word)
        && boundary_before(code, at)
        && boundary_after(code, at + word.len())
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
            'f' if paren == 0 && bracket == 0 && is_word_at(head, at, "for") => {
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

/// The trait this impl head names, as one of [`TRAITS`], or `None` when
/// the head is an inherent impl or names some other trait.
///
/// **The generic list is stepped over first.** `impl<P: PartialEq>
/// Doc<P>` names the trait in a BOUND and implements nothing; reading
/// the whole head for the word would call it a `PartialEq` impl and
/// then red on a body that is not one.
fn trait_of(code: &str, impl_at: usize, body_start: usize) -> Option<&'static str> {
    let mut at = skip_ws(code, impl_at + "impl".len());
    if code[at..].starts_with('<') {
        at = angle_end(code, at)? + 1;
    }
    let head = code.get(at..body_start)?;
    let for_at = top_level_for(head)?;
    // The trait, with any path qualification: `core::fmt::Debug`.
    let named = head[..for_at].trim().rsplit("::").next()?.trim();
    TRAITS.into_iter().find(|t| named == *t)
}

/// The first `self.<name>` in `body` where `<name>` is a field and not
/// a method or a tuple index.
///
/// A trailing `(` makes it a call, and a leading digit makes it a tuple
/// index — neither is a census that can fall behind a NAMED field.
///
/// **A raw identifier is ONE identifier, `r#` included.** Reading
/// `self.r#type` as `self.r` names a field that does not exist, and the
/// message this feeds is the only thing an author has to go on.
fn first_field_read(body: &str) -> Option<String> {
    let mut from = 0usize;
    while let Some(off) = body[from..].find("self.") {
        let at = from + off;
        from = at + "self.".len();
        if !boundary_before(body, at) {
            continue;
        }
        let rest = &body[from..];
        let skip = usize::from(rest.starts_with("r#")) * 2;
        let end = rest[skip..]
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .map_or(rest.len(), |off| skip + off);
        let name = &rest[..end];
        if name.is_empty() || name.starts_with(|c: char| c.is_ascii_digit()) || name == "r#" {
            continue;
        }
        if rest[end..].trim_start().starts_with('(') {
            continue;
        }
        return Some(format!("self.{name}"));
    }
    None
}

/// Whether `body` destructures `Self`, and whether the pattern is
/// exhaustive.
///
/// `Some(true)` is an exhaustive `Self { … }` pattern; `Some(false)` is
/// one with a rest arm; `None` is no `Self { … }` at all. A pattern
/// whose brace never closes answers `None` — broken text is not a
/// destructure.
fn destructures(body: &str) -> Option<bool> {
    let mut from = 0usize;
    let mut sawrest = false;
    while let Some(off) = body[from..].find("Self") {
        let at = from + off;
        from = at + "Self".len();
        if !boundary_before(body, at) || !boundary_after(body, at + "Self".len()) {
            continue;
        }
        let brace = skip_ws(body, from);
        if !body[brace..].starts_with('{') {
            continue;
        }
        let Some(end) = balanced_end(body, brace) else {
            continue;
        };
        // A rest arm binds nothing and stops nothing: `Self { a, .. }`
        // compiles with a field added, which is the property being
        // bought and the reason this is not simply "a `Self {` is
        // present".
        if body[brace + 1..end].contains("..") {
            sawrest = true;
            continue;
        }
        return Some(true);
    }
    sawrest.then_some(false)
}

/// The verdict for one impl body.
fn verdict(body: &str) -> Verdict {
    match destructures(body) {
        Some(true) => Verdict::Destructured,
        Some(false) => Verdict::RestPattern,
        None => match first_field_read(body) {
            Some(read) => Verdict::HandListed(read),
            None => Verdict::NoFieldRead,
        },
    }
}

/// Every hand-written `Debug`/`PartialEq` impl in one file's code view.
fn sites_in(path: &str, text: &str) -> Vec<Site> {
    let code = code_only(text);
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(off) = code[from..].find("impl") {
        let at = from + off;
        from = at + "impl".len();
        if !boundary_before(&code, at) || !boundary_after(&code, at + "impl".len()) {
            continue;
        }
        let (body_start, body) = match item_body(&code, at) {
            ItemBody::Body(r) => (r.start, code[r.start + 1..r.end - 1].to_string()),
            // A `;` terminator after an `impl` keyword is not an impl
            // at all (a `impl Trait` return type in a signature the
            // search landed inside); nothing to classify.
            ItemBody::Declaration(_) => continue,
            ItemBody::Unterminated => {
                out.push(Site {
                    path: path.to_string(),
                    line: test_utils::source::line(&code, at),
                    trait_name: "Debug",
                    verdict: Verdict::Unreadable("the impl head reaches end of file unterminated"),
                });
                continue;
            }
        };
        let Some(trait_name) = trait_of(&code, at, body_start) else {
            continue;
        };
        out.push(Site {
            path: path.to_string(),
            line: test_utils::source::line(&code, at),
            trait_name,
            verdict: verdict(&body),
        });
    }
    out
}

/// Every hand-written impl in the working tree, sorted by path.
fn sites(root: &Path) -> Vec<Site> {
    let mut out = Vec::new();
    for file in rust_sources(root) {
        if file
            .components()
            .any(|c| SKIPPED_DIRS.contains(&c.as_os_str().to_string_lossy().as_ref()))
        {
            continue;
        }
        let rel = file
            .strip_prefix(root)
            .expect("the walk stays under the root")
            .to_string_lossy()
            .replace('\\', "/");
        let text = std::fs::read_to_string(&file).expect("a readable source file");
        out.extend(sites_in(&rel, &text));
    }
    out.sort_by(|a, b| (&a.path, a.line).cmp(&(&b.path, b.line)));
    out
}

/// **The invariant.** A walk that reads a field by name has no tie to
/// the declaration, and the field added tomorrow is the cost.
#[test]
fn every_hand_written_walk_is_held_to_its_declaration() {
    let found = sites(&repo_root());
    let offenders: Vec<String> = found
        .iter()
        .filter(|s| {
            !KNOWN_HAND_LISTED
                .iter()
                .any(|(path, trait_name, _)| s.path == *path && s.trait_name == *trait_name)
        })
        .filter_map(|s| match &s.verdict {
            Verdict::HandListed(read) => Some(format!(
                "{}:{} impl {} — reads {read} with no exhaustive `Self` pattern",
                s.path, s.line, s.trait_name
            )),
            Verdict::RestPattern => Some(format!(
                "{}:{} impl {} — destructures `Self` with a rest arm, which binds nothing \
                 and stops nothing",
                s.path, s.line, s.trait_name
            )),
            Verdict::Unreadable(why) => Some(format!("{}:{} — {why}", s.path, s.line)),
            Verdict::Destructured | Verdict::NoFieldRead => None,
        })
        .collect();
    assert!(
        offenders.is_empty(),
        "a hand-written `Debug`/`PartialEq` that reads a field by name has no compile-time \
         tie to the declaration: a field added to the struct is silently outside the dump \
         and outside equality. Destructure `Self` exhaustively — `let Self {{ a, b }} = \
         self;` — so a new field is E0027, and bind a field the walk will not carry to `_` \
         (a walk that does so ends in `finish_non_exhaustive`, since `finish` claims every \
         field is shown):\n{offenders:#?}"
    );
}

/// **The reader's own guard: a walk that stops seeing reds here.**
///
/// Without this row the census above passes by finding nothing, which
/// is the silent direction and the one a census is built to close.
#[test]
fn the_walk_still_sees_every_file_holding_one() {
    let found = sites(&repo_root());
    let mut got: Vec<&str> = found.iter().map(|s| s.path.as_str()).collect();
    got.sort_unstable();
    got.dedup();
    let want: Vec<&str> = IMPL_FILES_TODAY.to_vec();
    let rendered: String = got.iter().map(|p| format!("    \"{p}\",\n")).collect();
    assert_eq!(
        got, want,
        "the set of files holding a hand-written `Debug`/`PartialEq` has moved away from \
         IMPL_FILES_TODAY. A file ADDED is an arrival, which is what this census exists to \
         detect — read the row above, then add the line. A file GONE with no impl removed \
         means this reader has gone blind to it and the row above is passing over sites it \
         can no longer read:\n{rendered}"
    );
}

/// **Every entry of [`KNOWN_HAND_LISTED`] still names a live site.**
///
/// A stale entry is the per-file blindness this census most needs to
/// see: the walk keeps reporting agreement, and the one file it was
/// told to expect a defect in has gone quiet.
#[test]
fn every_known_hand_listed_file_still_holds_one() {
    let found = sites(&repo_root());
    let stale: Vec<String> = KNOWN_HAND_LISTED
        .iter()
        .filter(|(path, trait_name, _)| {
            !found.iter().any(|s| {
                s.path == *path
                    && s.trait_name == *trait_name
                    && matches!(s.verdict, Verdict::HandListed(_))
            })
        })
        .map(|(path, trait_name, row)| format!("{path} impl {trait_name} — {row}"))
        .collect();
    assert!(
        stale.is_empty(),
        "KNOWN_HAND_LISTED names a file this walk no longer finds a hand-listed impl in. \
         If the repair landed, delete the entry and close the row it names. If it did not, \
         this reader has gone blind to that file:\n{stale:#?}"
    );
}

/// **The classifier, pinned shape by shape**, so that a reader which
/// stops telling the shapes apart reds on text it cannot blame on the
/// tree.
#[test]
fn the_classifier_answers_each_impl_shape() {
    let cases: [(&str, Verdict, &str); 12] = [
        (
            "a hand-listed struct walk",
            Verdict::HandListed("self.knots".into()),
            "impl core::fmt::Debug for S<'_> {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             f.debug_struct(\"S\").field(\"knots\", &self.knots).finish()\n    }\n}",
        ),
        (
            "a destructured struct walk",
            Verdict::Destructured,
            "impl core::fmt::Debug for S<'_> {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             let Self { knots, coeffs } = self;\n        \
             f.debug_struct(\"S\").field(\"knots\", knots).field(\"c\", coeffs).finish()\n    \
             }\n}",
        ),
        (
            "a destructure with a rest arm, which binds nothing",
            Verdict::RestPattern,
            "impl core::fmt::Debug for S {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             let Self { knots, .. } = self;\n        f.debug_struct(\"S\").finish()\n    }\n}",
        ),
        (
            "a newtype reading its tuple index",
            Verdict::NoFieldRead,
            "impl core::fmt::Debug for P {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             write!(f, \"P(<{} bytes>)\", self.0.len())\n    }\n}",
        ),
        (
            "a newtype reading a field THROUGH its tuple index",
            Verdict::NoFieldRead,
            "impl PartialEq for N {\n    fn eq(&self, other: &Self) -> bool {\n        \
             self.0.name == other.0.name\n    }\n}",
        ),
        (
            "an enum walk the compiler already holds exhaustive",
            Verdict::NoFieldRead,
            "impl PartialEq for E {\n    fn eq(&self, other: &Self) -> bool {\n        \
             match (self, other) {\n            (Self::A { w: a }, Self::A { w: b }) => a == \
             b,\n            _ => false,\n        }\n    }\n}",
        ),
        (
            "a field read through a METHOD, which is the stated blind spot",
            Verdict::NoFieldRead,
            "impl PartialEq for S {\n    fn eq(&self, other: &Self) -> bool {\n        \
             self.bit_eq(other)\n    }\n}",
        ),
        (
            "a RAW-IDENTIFIER field, which is one identifier and `r#` included",
            Verdict::HandListed("self.r#type".into()),
            "impl core::fmt::Debug for S {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             f.debug_struct(\"S\").field(\"t\", &self.r#type).finish()\n    }\n}",
        ),
        (
            "a walk whose unshown field binds to `_`",
            Verdict::Destructured,
            "impl core::fmt::Debug for T {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             let Self { forward, sealed: _ } = self;\n        \
             f.debug_struct(\"T\").field(\"f\", forward).finish_non_exhaustive()\n    }\n}",
        ),
        (
            "a `Self { … }` LITERAL, which reads as a destructure — the known false green",
            Verdict::Destructured,
            "impl core::fmt::Debug for S {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             let probe = Self { a: 1, b: 2 };\n        \
             f.debug_struct(\"S\").field(\"a\", &self.a).finish()\n    }\n}",
        ),
        (
            "a destructure of `other` beside one of `self`",
            Verdict::Destructured,
            "impl PartialEq for S {\n    fn eq(&self, other: &Self) -> bool {\n        \
             let Self { a } = self;\n        let Self { a: other_a } = other;\n        \
             a == other_a\n    }\n}",
        ),
        (
            "a `selfish` local, which is not `self`",
            Verdict::NoFieldRead,
            "impl core::fmt::Debug for S {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             let myself = 1;\n        write!(f, \"{myself}\")\n    }\n}",
        ),
    ];
    for (what, want, text) in cases {
        let found = sites_in("fixture.rs", text);
        assert_eq!(found.len(), 1, "{what}: expected one impl in\n{text}");
        assert_eq!(found[0].verdict, want, "{what}:\n{text}");
    }
}

/// **The head reader, pinned apart from the body reader**, because the
/// two fail in opposite directions: a head misread as a trait impl reds
/// on an inherent impl's body, and a trait impl misread as inherent
/// leaves a real defect unseen.
#[test]
fn the_head_reader_tells_a_trait_impl_from_a_bound() {
    let cases: [(&str, usize, &str); 6] = [
        (
            "an inherent impl whose BOUND names the trait",
            0,
            "impl<P: PartialEq + Payload> Doc<P> {\n    fn f(&self) -> u8 { self.a }\n}",
        ),
        (
            "a qualified trait path",
            1,
            "impl<E: Bound> core::fmt::Debug for S<'_, E> {\n    fn fmt(&self) {}\n}",
        ),
        (
            "an unqualified trait, imported",
            1,
            "impl<T: Real> PartialEq for W<'_, T> {\n    fn eq(&self) {}\n}",
        ),
        (
            "some OTHER trait, which is not this census's subject",
            0,
            "impl core::fmt::Display for S {\n    fn fmt(&self) { self.a }\n}",
        ),
        (
            "a bound carrying a `for<'a>` binder before the separator",
            1,
            "impl<F> core::fmt::Debug for S<F> where F: for<'a> Fn(&'a u8) {\n    fn fmt(&self) \
             {}\n}",
        ),
        (
            "a bound whose parentheses carry a `for`-shaped name",
            1,
            "impl<F: Fn(Formatter) -> bool> PartialEq for S<F> {\n    fn eq(&self) {}\n}",
        ),
    ];
    for (what, want, text) in cases {
        assert_eq!(sites_in("fixture.rs", text).len(), want, "{what}:\n{text}");
    }
}
