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
//! # The question, and what decides it
//!
//! A classifier that had to resolve each impl's self type to its
//! DECLARATION would need the type graph, across crates, from text.
//! **It does not need to**, because the defect is visible in the impl
//! itself: one that reads `<self>.<name>` is reading a field by name,
//! and one that destructures is not. What it reads besides the body is
//! the head's self type as WRITTEN — `Coset`, `S<'_>` — which is a
//! spelling and never a resolution: it is what lets a destructure
//! spelled with the concrete name count as one, and it is the key
//! [`KNOWN_HAND_LISTED`] is written on.
//!
//! Three shapes answer themselves rather than needing a roster:
//!
//! - a newtype reads `self.0` — a tuple index, which cannot fall
//!   behind a NAMED field of its own declaration. **It answers the
//!   index and nothing further**: `self.0.name` reads a named field of
//!   the INNER type, and that hand-list is on the blind-spot list
//!   below, not here;
//! - an enum walk reads `match self` or `match (self, other)`, whose
//!   arms the compiler holds exhaustive for VARIANTS (E0004) and, in a
//!   struct-variant pattern, for that variant's fields (E0027) — but
//!   only where the pattern binds them all. A `Self::V { a, .. }` arm
//!   defeats E0027 exactly as `Self { a, .. }` does and is read here
//!   exactly the same way, as [`Verdict::RestPattern`];
//! - a destructured walk reads its bindings, never `<self>.<name>`.
//!   `Self { … }`, `Self::V { … }` and the self type's own name all
//!   destructure.
//!
//! # What this cannot see
//!
//! - **A field read through a METHOD.** `self.origin()` and
//!   `self.bit_eq(other)` are calls, and no reader can tell a getter
//!   from any other call without resolving it. A hand-list moved one
//!   level down into a helper is invisible here.
//!   `crates/profile/src/lib.rs`'s `SketchPlane` is the standing
//!   example: `PartialEq` delegates to an inherent `bit_eq` that reads
//!   twelve stored coordinates, this reader sees a call and answers
//!   [`Verdict::NoFieldRead`], and it answers that whether those
//!   twelve are bound by name or not. What holds `bit_eq` is the
//!   patterns written INSIDE it and nothing here.
//! - **A field of an INNER type, read through a tuple index.**
//!   `self.0.name` is a hand-list one level down: `self.0` ties this
//!   impl to nothing but the newtype's own single field, and the named
//!   field belongs to a declaration this reader never sees. The
//!   standing example is
//!   `crates/editor-core/src/names/role.rs`'s `NameRef`, which has
//!   five walks over the shared `Held` — of which **this reader looks
//!   at two**, `Debug` and `PartialEq`, because [`TRAITS`] is those
//!   two; it reports both as [`Verdict::NoFieldRead`], `self.0` being
//!   a tuple index it stops at, and never looks at `Display`, `Hash`
//!   or `Ord` at all. Each of the five destructures `Held`, so `Held`'s
//!   fields are held by E0027 and not by anything on this page. That
//!   is the same shape as the method case one line up, reached through
//!   a field rather than a call.
//! - **A read through a LOCAL that is not `self`.** `let me = self;`
//!   then `me.a` is a hand-list this reader follows, because the
//!   binding is one `let` away in the same body ([`self_aliases`]);
//!   one that arrives through a match binding, a closure argument or a
//!   helper's parameter is not, and following those would make this a
//!   dataflow analysis over text.
//! - **An enum arm with a catch-all.** `_ => false` is exhaustive to
//!   the compiler and silent about a new variant, which is the same
//!   defect one level up from this one. `topo`'s `CensusSubject` is the
//!   live instance —
//!   `work/topo/censussubject-eq-answers-false-for-a-new-variant-against-itself.md`.
//! - **One exhaustive pattern anywhere in the body answers for the
//!   whole body** — `Self { … }`, `Self::V { … }` or the type's own
//!   name alike. A struct LITERAL reads as a destructure, and so would
//!   a body that destructured in one branch and hand-read a field in
//!   another. Neither shape exists in a `fmt` or an `eq` in this
//!   tree; the literal is pinned in
//!   [`the_classifier_answers_each_impl_shape`] so a future one is a
//!   known false green rather than an unknown.
//! - **A heterogeneous `impl PartialEq<Other> for T`.** The trait's
//!   generic arguments are part of the name this reader compares, so
//!   only the homogeneous spelling matches. None in this tree.
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
//! **This reads the WORKING TREE, not the build.** An untracked,
//! uncompiled scratch `.rs` under the root is walked like any other
//! file and can red both sight rows on its own — the fail-loud
//! direction, and the same reading `reader_census.rs` and
//! `deny_unknown_fields_census.rs` take of the same tree.
//!
//! **What is NOT on that list, because the classifier answers it:** a
//! rest arm in any destructure, `Self { a, .. }` and `Self::V { a, .. }`
//! alike, which is a destructure that is not exhaustive and reds here;
//! a destructure spelled with the self type's own name
//! (`let Coset { subgroup, representative } = self`), which is the
//! same pattern with a different word for the type;
//! `(*self).a`, which is `self.a` with a
//! deref written out; `self.conv::<u16>()`, a turbofished CALL and not
//! a field; and `impl<P: PartialEq> Doc<P>`, an inherent impl whose
//! BOUND names the trait. Each has its row in
//! [`the_classifier_answers_each_impl_shape`] or
//! [`the_head_reader_tells_a_trait_impl_from_a_bound`].

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::Path;

use test_utils::source::{
    ItemBody, balanced_end, boundary_after, boundary_before, code_only, impl_head, item_body,
    repo_root, rust_sources, skip_ws, type_base, word_at,
};

/// The repository's own directories, skipped by NAME: a build
/// directory, and — by the rule in [`sites`] rather than by a name
/// here — anything hidden. Same rule, and the same reason, as
/// `reader_census.rs` and `deny_unknown_fields_census.rs`: one walk
/// from the root reaches every `.rs` file in the working tree, so a
/// new top-level Rust tree is covered the day it lands rather than the
/// day someone remembers a roster.
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
    /// `match self`, a delegation. Nothing THIS READER can attribute
    /// to a declaration falls behind one.
    ///
    /// **The qualifier is load-bearing**, and the header's
    /// *What this cannot see* list is where it is argued: a delegation
    /// and a `self.0.<name>` both land here, and both can be reading
    /// named fields one level down. The verdict says what this walk
    /// saw, not what the impl does. Whether either shape should get a
    /// verdict of its own is
    /// `work/tint/census-answers-no-field-read-for-a-walk-that-reads-a-field.md`.
    ///
    /// **"No census here", not "the census is the empty set".** An
    /// `eq` that reads nothing and answers `true`
    /// (`crates/editor-core/src/names/table.rs`'s `Sealed`, where a
    /// cache flag is deliberately not part of the value) lands here
    /// too, and it is the one walk that cannot fall behind a
    /// declaration because it never consults one. This verdict says
    /// only that the defect this census names is absent; whether the
    /// answer is right is a question no text reader asks.
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
    /// The self type as the head WRITES it, whitespace collapsed:
    /// `Coset`, `SignCertificate<'_, T>`. A spelling, never a
    /// resolution — see [`KNOWN_HAND_LISTED`], whose key it is.
    self_type: String,
    verdict: Verdict,
}

/// **The sites this census knows are hand-listed and does not red on**,
/// each with the row that owns the repair.
///
/// Every entry is a SECOND FINDING stacked on the first, never an
/// exemption — the shape `reader_census.rs` calls `Unconverted` and for
/// the same reason. An entry sits here because the repair belongs to
/// another program's row rather than to whoever is passing, and this
/// constant is what keeps the finding from going quiet in the
/// meantime. It shrinks as those rows land: one repair, one entry
/// deleted, in the diff that makes the walk stop reporting the impl
/// hand-listed.
///
/// **It is also this walk's per-file sight anchor.** An entry that
/// matches no live site reds — so a reader that goes blind to
/// `clearance.rs` cannot report agreement over the files it can still
/// read.
///
/// **Keyed by `(path, trait, self type)`, and the third element is the
/// whole of the argument.** A suppression that names less than the
/// impl is a suppression that grows: `(path, trait)` alone covers
/// every impl of that trait the file will ever hold, so a hand-listed
/// `impl PartialEq for ProbeQ` appended to `clearance.rs` — an arrival
/// of exactly the defect class this census exists to detect — lands
/// green, suppressed by an entry written about `GeometryWitness`. The
/// self type closes it because a homogeneous trait has AT MOST ONE
/// impl per type, so the key names one impl and cannot cover a second.
///
/// It also stops the suppression reaching a COMPLIANT sibling under
/// the same `(path, trait)` as the offender — the case the entries
/// deleted from this list left behind, where `coset.rs`'s
/// `PartialEq for Subgroup`, an exhaustive enum walk with nothing
/// wrong with it, shared a file and a trait with the one that was
/// suppressed.
///
/// **What it costs is a spelling in the key.** The self type is read
/// as WRITTEN — `SignCertificate<'_, T>`, lifetimes and parameters and
/// all — because resolving it is what this census refuses to do; a
/// rename or a re-spelled parameter list makes the entry stale and
/// [`every_known_hand_listed_impl_is_still_found`] reds until it is
/// re-typed. That is the loud direction and it is the price of the
/// key. Not keyed by LINE, which rots on every edit above it.
const KNOWN_HAND_LISTED: [(&str, &str, &str, &str); 1] = [(
    "crates/editor-core/src/clearance.rs",
    "PartialEq",
    "GeometryWitness",
    "work/shell/geometrywitness-eq-ignores-the-two-chart-axes-its-uv-fields-are-stated-in.md",
)];

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
/// line, not a re-count.
///
/// **The residue, stated as it actually stands, and it has grown.** A
/// blindness that hides some but not all of one file's impls moves
/// nothing in THIS row: every file still contributes its first impl,
/// so the set is unchanged. [`every_known_hand_listed_impl_is_still_found`]
/// used to catch the worst case for the impls [`KNOWN_HAND_LISTED`]
/// named, by self type, so it red wherever in its file the impl sat.
/// **It no longer catches it at all.** Measured, on this tree and on
/// the tree before the four CENSUS rows landed: a classifier that
/// stops after the first Debug-or-PartialEq impl of each file red that
/// row before and is entirely green now, because the single remaining
/// entry names its file's ONLY such impl and so survives the
/// blindness. The anchor was never per-impl sight; it was the accident
/// that ONE of five entries — `mate/coset.rs`, which holds
/// `PartialEq for Subgroup` above `PartialEq for Coset` — had a
/// compliant sibling above it, and that entry is one of the four this
/// unit repaired. The other four entries were each their file's first
/// and only such impl, so they never carried per-impl sight at all.
/// (An earlier draft of this paragraph said FOUR of five, and a style
/// review executed the mutation and read the failure, which names one
/// entry.)
///
/// **So no row here sees a partial blindness inside a file**, for this
/// file's 22 or any of them: `hull.rs`, `knots.rs` and
/// `surfaces/nurbs.rs` each hold a second impl that would vanish
/// unremarked. What closes it is a per-impl population, which is the
/// tally this row declines above. The trade is recorded rather than
/// hidden, and that a SUPPRESSION LIST was standing in for it — so
/// that the instrument weakens exactly as the rows it names are
/// repaired — is
/// `work/tint/the-per-impl-sight-anchor-is-a-suppression-list-that-shrinks.md`.
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

/// The trait this census's subject list names, and the self type the
/// impl is for.
struct Head {
    /// One of [`TRAITS`].
    trait_name: &'static str,
    /// The self type, whitespace collapsed: `Coset`,
    /// `SignCertificate<'_, T>`.
    self_type: String,
}

/// The trait and self type this impl head names, or `None` when the
/// head is an inherent impl or names some other trait.
///
/// **The head itself is [`impl_head`]'s answer**, shared with the
/// minting census in `crates/pncad-py/src/tests.rs`, which reads the
/// same construct for a different key. What is this census's own is
/// the filter below: only a trait in [`TRAITS`] is its subject.
///
/// **The trait's generic arguments are kept, and that is deliberate.**
/// `PartialEq<Other>` does not match `PartialEq`, so a heterogeneous
/// impl answers `None` rather than reading as a homogeneous one — the
/// residue this file's header names, not an accident of the walk.
fn head_of(code: &str, impl_at: usize, body_start: usize) -> Option<Head> {
    let head = impl_head(code, impl_at, body_start)?;
    let named = head.trait_path?;
    let named = named.rsplit("::").next()?.trim();
    let trait_name = TRAITS.into_iter().find(|t| named == *t)?;
    Some(Head {
        trait_name,
        self_type: head.self_type,
    })
}

/// Every local in `body` that is another name for `self`, `self`
/// first.
///
/// **One `let` from the binding, and no further.** `let me = self;`
/// (and its `&`/`*` spellings) rewrites a hand-list into a name this
/// reader would otherwise not follow, and following it costs one scan.
/// A name bound by a match arm, a closure parameter or a helper's
/// argument is NOT followed: that is dataflow over text, and it is on
/// this file's blind-spot list rather than pretended away here.
fn self_aliases(body: &str) -> Vec<&str> {
    let mut out = vec!["self"];
    let mut from = 0usize;
    while let Some(off) = body[from..].find("let") {
        let at = from + off;
        from = at + "let".len();
        if !word_at(body, at, "let") {
            continue;
        }
        let mut name_at = skip_ws(body, from);
        if word_at(body, name_at, "mut") {
            name_at = skip_ws(body, name_at + "mut".len());
        }
        let name_end = body[name_at..]
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .map_or(body.len(), |off| name_at + off);
        let name = &body[name_at..name_end];
        if name.is_empty() {
            continue;
        }
        let eq = skip_ws(body, name_end);
        if !body[eq..].starts_with('=') || body[eq..].starts_with("==") {
            continue;
        }
        let mut rhs = skip_ws(body, eq + 1);
        while body[rhs..].starts_with('&') || body[rhs..].starts_with('*') {
            rhs = skip_ws(body, rhs + 1);
        }
        if !word_at(body, rhs, "self") {
            continue;
        }
        if !body[skip_ws(body, rhs + "self".len())..].starts_with(';') {
            continue;
        }
        out.push(name);
    }
    out
}

/// The first `<alias>.<name>` in `body` where `<name>` is a field and
/// not a method or a tuple index, over every name for `self`.
///
/// A trailing `(` makes it a call and so does a turbofish's `::` — a
/// field access carries neither — and a leading digit makes it a tuple
/// index. None of the three is a census that can fall behind a NAMED
/// field of THIS declaration.
///
/// **A raw identifier is ONE identifier, `r#` included.** Reading
/// `self.r#type` as `self.r` names a field that does not exist, and the
/// message this feeds is the only thing an author has to go on.
fn first_field_read(body: &str) -> Option<String> {
    self_aliases(body)
        .into_iter()
        .filter_map(|alias| first_read_of(body, alias))
        .min()
        .map(|(_, read)| read)
}

/// [`first_field_read`] for one name, with the offset it was found at
/// so the earliest read over all the names is the one reported.
fn first_read_of(body: &str, alias: &str) -> Option<(usize, String)> {
    let mut from = 0usize;
    while let Some(off) = body[from..].find(alias) {
        let at = from + off;
        from = at + alias.len();
        if !boundary_before(body, at) || !boundary_after(body, at + alias.len()) {
            continue;
        }
        // `(*self).a` is `self.a` with the deref written out. The
        // parens are stepped over only where a `*` opened them, so
        // `Wrapper(self).a` — a field of something else — is not read
        // as a field of this one.
        let mut dot = at + alias.len();
        if body[..at].trim_end().ends_with('*') {
            while body[dot..].starts_with(')') {
                dot += 1;
            }
        }
        if !body[dot..].starts_with('.') {
            continue;
        }
        let rest = &body[dot + 1..];
        let skip = usize::from(rest.starts_with("r#")) * 2;
        let end = rest[skip..]
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .map_or(rest.len(), |off| skip + off);
        let name = &rest[..end];
        if name.is_empty() || name.starts_with(|c: char| c.is_ascii_digit()) || name == "r#" {
            continue;
        }
        let tail = rest[end..].trim_start();
        if tail.starts_with('(') || tail.starts_with("::") {
            continue;
        }
        return Some((at, format!("{alias}.{name}")));
    }
    None
}

/// Whether a `{ … }` pattern's OWN level carries a rest arm.
///
/// **Its own level, and the rest-arm position.** A `..` inside a
/// nested pattern — `Self { a: Foo { x, .. } }` — says nothing about
/// whether this pattern binds all of `Self`'s fields, and a
/// `contains("..")` over the whole span reds a walk that is held. The
/// rest arm follows the brace or a comma, which is also what tells it
/// from a range pattern (`Self { a: 1..=2 }`).
fn has_rest_arm(span: &str) -> bool {
    let (mut curly, mut paren, mut bracket) = (0usize, 0usize, 0usize);
    let mut prev = '{';
    for (at, c) in span.char_indices() {
        match c {
            '{' => curly += 1,
            '}' => curly = curly.saturating_sub(1),
            '(' => paren += 1,
            ')' => paren = paren.saturating_sub(1),
            '[' => bracket += 1,
            ']' => bracket = bracket.saturating_sub(1),
            '.' if curly == 0
                && paren == 0
                && bracket == 0
                && span[at..].starts_with("..")
                && (prev == '{' || prev == ',') =>
            {
                return true;
            }
            _ => {}
        }
        if !c.is_whitespace() {
            prev = c;
        }
    }
    false
}

/// Whether `body` destructures its value, and whether the pattern is
/// exhaustive.
///
/// `Some(true)` is an exhaustive pattern; `Some(false)` is one with a
/// rest arm; `None` is no destructure at all. A pattern whose brace
/// never closes answers `None` — broken text is not a destructure.
///
/// **Three spellings, one question.** `Self { … }` is the canonical
/// one; `Self::V { … }` is a struct-variant arm, where E0027 holds
/// that variant's fields exactly as it holds a struct's and a `..`
/// defeats it exactly as it does there; and `Coset { … }` is the same
/// pattern written with the type's own name, which a reader that knew
/// only the word `Self` would call no destructure at all and red a
/// walk that is held.
fn destructures(body: &str, base: &str) -> Option<bool> {
    let mut sawrest = false;
    let mut names = vec!["Self"];
    if !base.is_empty() && base != "Self" {
        names.push(base);
    }
    for name in names {
        if pattern_of(body, name, &mut sawrest) {
            return Some(true);
        }
    }
    sawrest.then_some(false)
}

/// Whether `body` carries an exhaustive `name { … }` or
/// `name::V { … }` pattern, setting `sawrest` when it finds one whose
/// arm is a rest.
fn pattern_of(body: &str, name: &str, sawrest: &mut bool) -> bool {
    let mut from = 0usize;
    while let Some(off) = body[from..].find(name) {
        let at = from + off;
        from = at + name.len();
        if !boundary_before(body, at) || !boundary_after(body, at + name.len()) {
            continue;
        }
        let mut after = skip_ws(body, from);
        if body[after..].starts_with("::") {
            let variant = skip_ws(body, after + 2);
            let end = body[variant..]
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .map_or(body.len(), |off| variant + off);
            if end == variant {
                continue;
            }
            after = skip_ws(body, end);
        }
        if !body[after..].starts_with('{') {
            continue;
        }
        let Some(end) = balanced_end(body, after) else {
            continue;
        };
        // A rest arm binds nothing and stops nothing: `Self { a, .. }`
        // compiles with a field added, which is the property being
        // bought and the reason this is not simply "a `Self {` is
        // present".
        if has_rest_arm(&body[after + 1..end]) {
            *sawrest = true;
            continue;
        }
        return true;
    }
    false
}

/// The verdict for one impl body, given its self type's bare name.
fn verdict(body: &str, base: &str) -> Verdict {
    match destructures(body, base) {
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
            // A head that reaches end of input without either
            // terminator is broken text, not a declaration. It is a
            // red — a walk that cannot parse a site and shrugs is the
            // failure a census exists to prevent — but only where the
            // head names one of THIS census's traits: an unterminated
            // inherent impl is someone else's broken file, and reding
            // on it would put a path this census has no subject in
            // into its own sight anchor. The trait is read over the
            // rest of the file, since there is no body to stop at.
            ItemBody::Unterminated => {
                if let Some(head) = head_of(&code, at, code.len()) {
                    out.push(Site {
                        path: path.to_string(),
                        line: test_utils::source::line(&code, at),
                        trait_name: head.trait_name,
                        self_type: head.self_type,
                        verdict: Verdict::Unreadable(
                            "the impl head reaches end of file unterminated",
                        ),
                    });
                }
                continue;
            }
        };
        let Some(head) = head_of(&code, at, body_start) else {
            continue;
        };
        out.push(Site {
            path: path.to_string(),
            line: test_utils::source::line(&code, at),
            trait_name: head.trait_name,
            verdict: verdict(&body, type_base(&head.self_type)),
            self_type: head.self_type,
        });
    }
    out
}

/// Every hand-written impl in the working tree, sorted by path.
///
/// **A build directory and every hidden one are skipped**, the second
/// by the leading dot rather than by a name in [`SKIPPED_DIRS`] — the
/// rule `reader_census.rs` and `deny_unknown_fields_census.rs` state
/// and this one had only claimed. A `.`-directory holds a checkout's
/// own machinery, never a crate of this workspace, and walking into
/// one puts a vendored or cached `.rs` file into a census of THIS
/// tree's impls.
fn sites(root: &Path) -> Vec<Site> {
    let mut out = Vec::new();
    for file in rust_sources(root) {
        // The RELATIVE path, because the skip reads components and the
        // root's own ancestors are not this repository's directories:
        // a checkout under a hidden one would otherwise skip the whole
        // tree and leave every row below passing over nothing.
        let relative = file
            .strip_prefix(root)
            .expect("the walk stays under the root");
        if relative.components().any(|c| {
            let c = c.as_os_str().to_string_lossy();
            SKIPPED_DIRS.contains(&c.as_ref()) || c.starts_with('.')
        }) {
            continue;
        }
        let rel = relative.to_string_lossy().replace('\\', "/");
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
    let found = sites(&repo_root(env!("CARGO_MANIFEST_DIR")));
    let offenders: Vec<String> = found
        .iter()
        .filter(|s| {
            !KNOWN_HAND_LISTED
                .iter()
                .any(|(path, trait_name, self_type, _)| {
                    s.path == *path && s.trait_name == *trait_name && s.self_type == *self_type
                })
        })
        .filter_map(|s| match &s.verdict {
            Verdict::HandListed(read) => Some(format!(
                "{}:{} impl {} for {} — reads {read} with no exhaustive pattern",
                s.path, s.line, s.trait_name, s.self_type
            )),
            Verdict::RestPattern => Some(format!(
                "{}:{} impl {} for {} — destructures with a rest arm, which binds nothing \
                 and stops nothing",
                s.path, s.line, s.trait_name, s.self_type
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
    let found = sites(&repo_root(env!("CARGO_MANIFEST_DIR")));
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

/// **Every entry of [`KNOWN_HAND_LISTED`] still names a live impl.**
///
/// A stale entry is the blindness this census most needs to see: the
/// walk keeps reporting agreement, and the one impl it was told to
/// expect a defect in has gone quiet. Keyed on the self type, so it is
/// that IMPL the walk must still find and not merely something in its
/// file — which is what makes the row independent of where in the file
/// the impl sits.
#[test]
fn every_known_hand_listed_impl_is_still_found() {
    let found = sites(&repo_root(env!("CARGO_MANIFEST_DIR")));
    let stale: Vec<String> = KNOWN_HAND_LISTED
        .iter()
        .filter(|(path, trait_name, self_type, _)| {
            !found.iter().any(|s| {
                s.path == *path
                    && s.trait_name == *trait_name
                    && s.self_type == *self_type
                    && matches!(s.verdict, Verdict::HandListed(_))
            })
        })
        .map(|(path, trait_name, self_type, row)| {
            format!("{path} impl {trait_name} for {self_type} — {row}")
        })
        .collect();
    assert!(
        stale.is_empty(),
        "KNOWN_HAND_LISTED names an impl this walk no longer finds hand-listed. If the \
         repair landed, delete the entry and close the row it names. If the type was \
         renamed, re-type the entry's third element. If neither, this reader has gone \
         blind to that impl:\n{stale:#?}"
    );
}

/// **The classifier, pinned shape by shape**, so that a reader which
/// stops telling the shapes apart reds on text it cannot blame on the
/// tree.
#[test]
fn the_classifier_answers_each_impl_shape() {
    let cases: [(&str, Verdict, &str); 21] = [
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
            "a newtype reading a NAMED field of its inner type — the blind spot, pinned \
             here so it is a known false green and not an unknown one",
            Verdict::NoFieldRead,
            "impl PartialEq for N {\n    fn eq(&self, other: &Self) -> bool {\n        \
             self.0.name == other.0.name\n    }\n}",
        ),
        (
            "an enum walk whose struct-variant arm binds every field",
            Verdict::Destructured,
            "impl PartialEq for E {\n    fn eq(&self, other: &Self) -> bool {\n        \
             match (self, other) {\n            (Self::A { w: a }, Self::A { w: b }) => a == \
             b,\n            _ => false,\n        }\n    }\n}",
        ),
        (
            "an enum walk whose struct-variant arm carries a rest, which defeats E0027 \
             exactly as it does in a struct pattern",
            Verdict::RestPattern,
            "impl core::fmt::Debug for E {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             match self {\n            Self::A { w, .. } => write!(f, \"A({w})\"),\n        \
             }\n    }\n}",
        ),
        (
            "a nested rest, which says nothing about the OUTER pattern's exhaustiveness",
            Verdict::Destructured,
            "impl core::fmt::Debug for S {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             let Self { a: Foo { x, .. }, b } = self;\n        \
             f.debug_struct(\"S\").field(\"x\", x).field(\"b\", b).finish()\n    }\n}",
        ),
        (
            "a destructure spelled with the type's OWN name",
            Verdict::Destructured,
            "impl core::fmt::Debug for Named {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             let Named { a, b } = self;\n        \
             f.debug_struct(\"Named\").field(\"a\", a).field(\"b\", b).finish()\n    }\n}",
        ),
        (
            "the same spelling with a rest arm, which buys nothing either",
            Verdict::RestPattern,
            "impl core::fmt::Debug for Named {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             let Named { a, .. } = self;\n        \
             f.debug_struct(\"Named\").field(\"a\", a).finish()\n    }\n}",
        ),
        (
            "a field read through a written-out deref",
            Verdict::HandListed("self.a".into()),
            "impl core::fmt::Debug for S {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             f.debug_struct(\"S\").field(\"a\", &(*self).a).finish()\n    }\n}",
        ),
        (
            "a field read through a local rebound from `self`",
            Verdict::HandListed("me.a".into()),
            "impl core::fmt::Debug for S {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             let me = self;\n        \
             f.debug_struct(\"S\").field(\"a\", &me.a).finish()\n    }\n}",
        ),
        (
            "a TURBOFISHED call, which is a call and not a field",
            Verdict::NoFieldRead,
            "impl core::fmt::Debug for S {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             write!(f, \"{}\", self.conv::<u16>())\n    }\n}",
        ),
        (
            "a field of something else, reached through a call on `self`",
            Verdict::NoFieldRead,
            "impl core::fmt::Debug for S {\n    fn fmt(&self, f: &mut F) -> R {\n        \
             write!(f, \"{}\", Wrapper(self).a)\n    }\n}",
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
            "an unterminated head naming one of this census's traits",
            Verdict::Unreadable("the impl head reaches end of file unterminated"),
            "impl core::fmt::Debug for S {\n    fn fmt(&self, f: &mut F) -> R {\n",
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
    let cases: [(&str, usize, &str); 7] = [
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
        (
            "an unterminated head that is an INHERENT impl, which is not this subject",
            0,
            "impl<T> Doc<T> {\n    fn f(&self) -> u8 { self.a }\n",
        ),
    ];
    for (what, want, text) in cases {
        assert_eq!(sites_in("fixture.rs", text).len(), want, "{what}:\n{text}");
    }
}

/// **The self type, pinned as the key it is.** A spelling that moved
/// would re-point every [`KNOWN_HAND_LISTED`] entry at once, and the
/// row that would notice reds with a message about blindness rather
/// than about a reader that changed its mind.
#[test]
fn the_head_reader_spells_the_self_type_as_written() {
    let cases: [(&str, &str, &str); 5] = [
        (
            "a bare name",
            "Coset",
            "impl PartialEq for Coset {\n    fn eq(&self, other: &Self) -> bool { true }\n}",
        ),
        (
            "generic arguments, kept",
            "SignCertificate<'_, T>",
            "impl<T: Decide> fmt::Debug for SignCertificate<'_, T> {\n    fn fmt(&self) {}\n}",
        ),
        (
            "a WHERE clause, which bounds the impl and is not the type",
            "S<F>",
            "impl<F> core::fmt::Debug for S<F> where F: for<'a> Fn(&'a u8) {\n    fn fmt(&self) \
             {}\n}",
        ),
        (
            "whitespace collapsed, so a re-wrapped head keys the same",
            "S<'_, E>",
            "impl<E: Bound> core::fmt::Debug\n    for S<'_, E>\n{\n    fn fmt(&self) {}\n}",
        ),
        (
            "an unterminated head, which has no body brace to stop at",
            "S",
            "impl core::fmt::Debug for S {\n    fn fmt(&self, f: &mut F) -> R {\n",
        ),
    ];
    for (what, want, text) in cases {
        let found = sites_in("fixture.rs", text);
        assert_eq!(found.len(), 1, "{what}: expected one impl in\n{text}");
        assert_eq!(found[0].self_type, want, "{what}:\n{text}");
    }
}
