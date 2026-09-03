//! Corpus document **die_composed_tour** — the demo tour's composed
//! die, registered rather than re-transcribed.
//!
//! `die_composed` beside it is the surgery at its smallest honest
//! size: one pip, one rim, fourteen selected edges. The tour builds
//! the same surgery at the size a person actually sees — twenty-one
//! pips cut in one grouped tool, then two `Node::Fillet` sites over
//! it: twelve box edges, and forty-two rim arcs whose names are twenty
//! pairwise unions deep. Every property the small document pins holds
//! here at a scale where a naming or selection defect has room to show
//! itself.
//!
//! # Which of the tour's dice this is
//!
//! The scene's `build` authors THREE fillets, because it narrates
//! three stops and a stop renders one body. The third is the blank —
//! the same cube filleted with no pips cut — and it is a DAG sink, so
//! in a document it is a second product root sitting exactly on the
//! first: coincident faces, doubled volume, the #1162 separation
//! defect. The tour already ruled on that for its viewer document and
//! deletes the blank (`diefillet::gallery_document`), and the ruling
//! does not weaken when the consumer is a corpus — the registry
//! evaluates, gathers and round-trips what it holds. So this is the
//! tour's DOCUMENT, blank deleted, and the exported log carries that
//! deletion rather than a shorter build.
//!
//! Registering the three-root form instead was measured, not assumed:
//! its product refuses `assemble` at `f64` with vertex-vertex
//! `UndeclaredContact` findings between the two dice, which puts it in
//! the divergence set `r2_m10_di_probes` pins — a row about coincident
//! roots inside a pin about the dual census door.
//!
//! # Why the document arrives as BYTES
//!
//! The authoring site is `demos/tour/src/diefillet.rs::build`, and it
//! stays the only one. That crate is a detached cargo workspace which
//! the kernel must never depend on, so there is no call to make: the
//! tour writes the document through its own `die-corpus` mode
//! (`diefillet::corpus_text`), the bytes are committed here, and this
//! module replays them. The alternative — a hand-transcribed twin —
//! is exactly what this document exists to retire, because a
//! transcription drifts silently and a byte file does not.
//!
//! Regenerate it with the tour's own door:
//!
//! ```text
//! cd demos/tour && cargo run --release -- die-corpus \
//!     ../../crates/editor-core/tests/corpus/tour/die_composed_tour.pncad
//! ```
//!
//! `ci.yml` runs exactly that line and diffs the result against the
//! committed file, so a scene change that moves the document is a red
//! run rather than a stale asset. That gate rides the tour job's
//! sampled row, not every code-tier run — see the PR that landed it.
//!
//! # Why the EDIT LOG and not the snapshot
//!
//! The file's snapshot is the EMPTY document; the model is its edit
//! log. That is deliberate and it is what makes the bytes usable here
//! at all. A snapshot records the ε it was written under and
//! `persist::load` refuses it against a different process ε (one
//! process, one ε — `PersistError::ToleranceConflict`), while the
//! registry is evaluated at every CI ε row. An edit log records no ε,
//! so replaying it into a document minted at the ambient tolerance
//! gives every row the same recipe.
//!
//! The price is that this module does NOT go through `persist::load`:
//! it reads the body's edit log directly under today's types, so a
//! committed body this build cannot read refuses here, loudly, with
//! the regeneration line above as the recourse (the same recourse the
//! persistence door gives — the format carries no schema version).
//!
//! # The selections are FROZEN at the tour's tolerance
//!
//! Both geometric selections were materialized by the tour, under the
//! ε the file was generated at, and travel in the log as stored names.
//! Every ε row therefore blends the SAME fifty-four edges rather than
//! re-selecting per row — which is what a stored selection means
//! everywhere else in the registry (`Node::Fillet`'s payload docs), and
//! is the property the name-digest gate measures.
//!
//! # No mass pin
//!
//! π-valued closed forms are not dyadic — the `die_composed` /
//! `die_fillet` disposition. The tour's own stop meters the composed
//! volume against its closed form.

use std::path::Path;

use editor_core::{
    Axis3, DocEdit, Node, ProfileDoc, ProfileProgram, RecipeNodeId, SlotId, header_document_id,
};
use geom_core::Tol;

use super::super::fixture::len;
use super::CorpusDoc;

/// The committed document, relative to the crate manifest. Read at
/// run time rather than `include_str!`d: the corpus module is compiled
/// once per suite in the aggregated test binary, and this file is
/// large enough that thirty embedded copies would be a measurable
/// share of it.
const FILE: &str = "tests/corpus/tour/die_composed_tour.pncad";

/// The label the tour authors under, and so the identity the committed
/// file must carry — checked, because a file replayed under the wrong
/// label is a different document with the same nodes.
const DOC_LABEL: &str = "die";

/// The bumped Y coordinate of the top face's single pip, meters
/// (dyadic; a 1/32 slide, which keeps the whole cavity on its face and
/// clear of the box-edge blend bands).
pub const PIP_Y_BUMPED: f64 = 0.53125;

/// The tour's edit log, parsed out of the committed save file.
///
/// # Panics
///
/// Loudly, on every arm: a missing file, a schema version this build
/// does not write, an identity that is not [`DOC_LABEL`]'s, or a body
/// today's types cannot read. Each names the regeneration line.
fn edits() -> Vec<DocEdit<ProfileProgram>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(FILE);
    let recourse = "regenerate it: cd demos/tour && cargo run --release -- \
                    die-corpus ../../crates/editor-core/tests/corpus/tour/\
                    die_composed_tour.pncad";
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "the tour die document is missing ({}): {e} — {recourse}",
            path.display()
        )
    });
    // The header goes through the persistence layer's own parser, so
    // the id line is validated by the code that writes it rather than
    // by a second reader here.
    let id = header_document_id(&text)
        .unwrap_or_else(|e| panic!("the tour die document's header refuses: {e:?} — {recourse}"));
    assert_eq!(
        id,
        ProfileDoc::empty_derived(DOC_LABEL, Tol::witness()).id(),
        "the committed document is not the tour's `{DOC_LABEL}` document — {recourse}"
    );
    let (_, body) = text.split_once('\n').expect("an id line");
    let mut value: serde_json::Value = serde_json::from_str(body)
        .unwrap_or_else(|e| panic!("the tour die document's body refuses: {e} — {recourse}"));
    serde_json::from_value(value["edits"].take())
        .unwrap_or_else(|e| panic!("the tour die document's edit log refuses: {e} — {recourse}"))
}

/// The first `Transform` in the log — the +Z face's single pip, which
/// is the first placement the tour emits. Derived from the document
/// rather than transcribed as an id, so the bump follows the scene
/// instead of pinning a number that would go quietly wrong.
fn first_pip(doc: &ProfileDoc) -> RecipeNodeId {
    doc.order()
        .iter()
        .copied()
        .find(|id| {
            matches!(
                doc.node(*id).expect("an ordered node exists"),
                Node::Transform { .. }
            )
        })
        .expect("the die places its pips with Transform nodes")
}

/// The tour's composed-die corpus document.
pub fn document() -> CorpusDoc {
    let edits = edits();
    let mut doc = ProfileDoc::empty_derived(DOC_LABEL, Tol::witness());
    for edit in &edits {
        doc = editor_core::apply(&doc, edit, Tol::witness())
            .expect("the tour's edit log replays")
            .doc;
    }
    let composed = *doc.order().last().expect("the die has nodes");
    assert!(
        matches!(
            doc.node(composed).expect("the last node"),
            Node::Fillet { .. }
        ),
        "the tour's die ends in the rim blend; if it no longer does, \
         this document's `result` names the wrong node"
    );
    let pip = first_pip(&doc);

    CorpusDoc {
        name: "die_composed_tour",
        about: "the demo tour's die: 21 pips cut in one grouped tool, then 12 box edges \
                and 42 rim arcs blended behind names 20 unions deep",
        edits,
        doc,
        result: Some(composed),
        // π-valued closed forms are not dyadic — module docs.
        pin: None,
        // D2's incremental probe, `die_composed`'s bump at this
        // document's scale: slide the top face's pip. The cube chain,
        // the master ball and the twenty other placements are reused;
        // the union tool below this pip, the cut and both surgeries
        // recompute. The slide mints and retires no edge, so both
        // frozen selections still resolve.
        bump: DocEdit::SetParam {
            node: pip,
            slot: SlotId::Translation(Axis3::Y),
            expr: len(PIP_Y_BUMPED),
        },
        bump_root: pip,
    }
}
