//! **A name is scoped by the variable that holds its body** (NAMES N1,
//! the scope; FORK-DM4's second unit). One table per output variable: a
//! split's two halves are two tables, its roles spell no half, and a
//! reference sited by a half's read resolves in that half's table alone.
//!
//! The rows here are the ones the scope itself owes:
//!
//! - a piece whose split side flips when the plane moves leaves its
//!   read's table rather than renaming, and N5 diagnoses `PredicateFlip`
//!   from the split's recorded classification, not from the name
//!   (NAMES N2, the Split op's pieces) — through the sited door, since
//!   the bare door finds the name in the other half and calls it
//!   resolved;
//! - a name a split mints in both halves (the section face) is one name,
//!   each half's read its own copy, and a paint keyed by one half's read
//!   leaves the other half's copy unpainted;
//! - a name saved with a half refuses typed at load, the regenerate
//!   recourse (`persist`: no migration chain).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert, len, on_frame, scl};
use editor_core::{
    Axis3, Datum, Diagnosis, DocEdit, EntityKind, Entry, ExtrudeSide, Node, ProfileDoc,
    RecipeNodeId, Resolution, ResolveError, RoleSeg, RunCtx, SlotId, SplitHalf, StableName,
    resolve, resolve_at, resolve_at_with_prior,
};
use geom_core::Tol;

/// A step in plan, extruded `z ∈ [0, 1]`: low `y ∈ [0, 1]` over
/// `x ∈ [0, 2]`, high `y ∈ [0, 2]` over `x ∈ [2, 4]`, so the riser wall
/// stands at `x = 2` (profile segment 3). Split by the plane `x = at`,
/// normal `+x`: the riser lies Above for `at < 2` and Below for `at > 2`,
/// and either plane cuts the step.
fn step_split(at: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("dm4_names_by_variable", Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.0, 0.0),
            (4.0, 0.0),
            (4.0, 2.0),
            (2.0, 2.0),
            (2.0, 1.0),
            (0.0, 1.0),
        ]],
    );
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, tool) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(at), len(0.0), len(0.0)],
            normal: [scl(1.0), scl(0.0), scl(0.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: ext.into(),
            tool: tool.into(),
        },
    );
    (doc, ext, tool, split)
}

/// **A piece whose side flips leaves its read's table, and N5 says why.**
/// The riser passes through the split whole, under the extrude's own
/// name, in the half its side puts it in. Moving the plane across it
/// moves it to the other half's table: the bare door still finds the
/// name there and resolves it, which is exactly why a reference is sited
/// by its read; through the read of the half it was in, it vanishes,
/// diagnosed `PredicateFlip` off the split's recorded classification.
#[test]
fn a_piece_whose_side_flips_leaves_its_reads_table_diagnosed_as_a_predicate_flip() {
    let (doc, ext, tool, split) = step_split(1.0);
    let riser = fixture::fname(ext, fixture::wall(&doc, ext, 3));
    let above = doc
        .output(split, SplitHalf::Above.port())
        .expect("the upper half");
    let below = doc
        .output(split, SplitHalf::Below.port())
        .expect("the lower half");
    let before = fixture::run(&doc, &Default::default());
    let held = |ev: &editor_core::Evaluation<f64>, read| {
        matches!(
            ev.table(read).expect("each half evaluates").lookup(&riser),
            Some(Entry::Unique(_))
        )
    };
    assert!(
        held(&before, above) && !held(&before, below),
        "the premise: the riser is the upper half's"
    );

    let (moved, _) = fixture::step(
        doc.clone(),
        DocEdit::SetParam {
            node: tool,
            slot: SlotId::Origin(Axis3::X),
            value: len(3.0).into(),
            fresh: Vec::new(),
        },
    );
    let after = fixture::run(&moved, &Default::default());
    assert!(
        !held(&after, above) && held(&after, below),
        "the plane moved across the riser: it is the lower half's now"
    );
    let new = RunCtx {
        doc: &moved,
        eval: &after,
    };
    let prior = RunCtx {
        doc: &doc,
        eval: &before,
    };

    assert!(
        matches!(resolve(new, &riser), Resolution::Resolved(_)),
        "the bare door finds the name in the other half"
    );
    assert!(
        matches!(resolve_at(new, below, &riser), Resolution::Resolved(_)),
        "and the lower half's read holds it"
    );
    match resolve_at_with_prior(new, prior, above, &riser) {
        Resolution::Failed(f) => match f.error {
            ResolveError::Vanished {
                diagnosis: Diagnosis::PredicateFlip { predicate, .. },
                ..
            } => assert_eq!(
                predicate, "split_vertex_side",
                "the split's own classification of the riser's vertices"
            ),
            other => panic!("the upper half's read: a predicate flip, not {other:?}"),
        },
        other => panic!("the upper half's read no longer holds the riser: {other:?}"),
    }
}

/// **A name both halves carry is one name, each read its own copy.**
/// The section face is minted in both halves under one spelling: the
/// read that holds it says which half's it is, so each half's read
/// resolves it to that half's own face.
#[test]
fn the_section_face_is_one_name_and_each_halfs_read_resolves_its_own() {
    let (doc, _, _, split) = step_split(1.0);
    let ev = fixture::run(&doc, &Default::default());
    let section = StableName {
        kind: EntityKind::Face,
        node: split,
        path: vec![RoleSeg::SectionFace { section: 0 }],
    };
    let ctx = RunCtx {
        doc: &doc,
        eval: &ev,
    };
    let mut held = Vec::new();
    for half in [SplitHalf::Above, SplitHalf::Below] {
        let read = doc.output(split, half.port()).expect("a half's read");
        let Resolution::Resolved(r) = resolve_at(ctx, read, &section) else {
            panic!("{half:?}: the section face resolves through its read");
        };
        assert_eq!(r.node, split, "{half:?}");
        assert_eq!(
            r.entity.body,
            half.output_body(),
            "{half:?}: the half's own face, in the split's value"
        );
        held.push(r.entity);
    }
    assert_ne!(held[0], held[1], "two faces, one name");
}

/// **A paint on one half's section face leaves the other half's
/// unpainted.** The two halves carry the section face under one name
/// (above); an appearance entry is keyed by the body read and the name
/// (ruling A on FORK-DM4 unit 2; D10, a repair is addressed by body and
/// name), so it paints the face the read it names holds, and only that
/// one. Keyed by the bare name it would paint both — one half's colour
/// on the other.
#[test]
fn a_paint_on_one_halfs_section_face_leaves_the_other_halfs_unpainted() {
    let (doc, _, _, split) = step_split(1.0);
    let section = StableName {
        kind: EntityKind::Face,
        node: split,
        path: vec![RoleSeg::SectionFace { section: 0 }],
    };
    let read = |half: SplitHalf| doc.output(split, half.port()).expect("a half's read");
    let (painted, _) = fixture::step(
        doc.clone(),
        DocEdit::SetAppearance {
            body: read(SplitHalf::Above),
            name: section.clone(),
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(200, 30, 30)),
        },
    );
    let ev = fixture::run(&painted, &Default::default());
    assert!(ev.appearance.is_lossless(), "{:?}", ev.appearance.losses);
    let ctx = RunCtx {
        doc: &painted,
        eval: &ev,
    };
    let face = |half| match resolve_at(ctx, read(half), &section) {
        Resolution::Resolved(r) => r.entity,
        other => panic!("{half:?}: the section face resolves: {other:?}"),
    };
    let at_split = ev
        .appearance
        .for_node(split)
        .expect("the split's value carries the paint");
    assert!(
        at_split.contains_key(&face(SplitHalf::Above)),
        "the upper half's section face is painted"
    );
    assert!(
        !at_split.contains_key(&face(SplitHalf::Below)),
        "the lower half's section face is not"
    );
    assert_eq!(at_split.len(), 1, "one face painted: {at_split:?}");
}

/// **A name saved with a half refuses typed.** The half is not a field
/// of a split's roles any more; the format carries no migration chain,
/// so a document spelling one refuses at the deserializer, which names
/// what it could not place.
#[test]
fn a_split_role_spelled_with_a_half_refuses_typed() {
    for old in [
        r#"{"SectionFace":{"side":"Above","section":0}}"#,
        r#"{"SplitBody":"Below"}"#,
    ] {
        let read: Result<RoleSeg, _> = serde_json::from_str(old);
        assert!(read.is_err(), "{old} reads as {read:?}");
    }
    let now: RoleSeg = serde_json::from_str(r#"{"SectionFace":{"section":0}}"#)
        .expect("the half-free spelling reads");
    assert_eq!(now, RoleSeg::SectionFace { section: 0 });
}

/// The stored documents this unit regenerated: each one whose
/// appearance store holds an entry (`M4_PR6_BLESS_GOLDEN=1` for the
/// golden).
const REGENERATED: [&str; 1] = ["crates/editor-core/tests/golden/golden.cad"];

/// **The regenerated documents are the base's under one total map**: an
/// appearance key, which was a name, is that name read in the output its
/// minting node defines (`{ "at": read, "name": name }`, the
/// [`editor_core::AppearanceKey`] shape), and nothing else moves. No
/// stored document spells a split role, so the half the roles drop
/// strips nothing here (`a_split_role_spelled_with_a_half_refuses_typed`
/// is what a file that did would meet).
///
/// Run against a checkout of the base: `DM4_BASE_TREE=<that checkout>`
/// and `--run-ignored only`. Ignored because the base is not in this
/// tree; the PR states the run.
#[test]
#[ignore = "one shot against a checkout of the base, named by DM4_BASE_TREE"]
fn every_regenerated_document_is_the_base_one_with_its_appearance_keys_sited() {
    let base = std::path::PathBuf::from(
        std::env::var("DM4_BASE_TREE").expect("DM4_BASE_TREE names a checkout of the base"),
    );
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for file in REGENERATED {
        let read =
            |root: &std::path::Path| std::fs::read_to_string(root.join(file)).expect("reads");
        let old = crate::wire::wire_body(&read(&base));
        let mut new = crate::wire::wire_body(&read(&here));
        let defined_by = |var: &str| -> Option<String> {
            let def = &new["snapshot"]["vars"][var]["def"]["Output"];
            Some(def.get("node")?.as_str()?.to_owned())
        };
        let mut stripped = Vec::new();
        for pair in new["snapshot"]["appearance"]
            .as_array()
            .expect("an appearance pair list")
        {
            let (key, record) = (&pair[0], &pair[1]);
            let at = key["at"].as_str().expect("a key's read");
            let name = &key["name"];
            assert_eq!(
                defined_by(at).as_deref(),
                name["node"].as_str(),
                "{file}: {at} is the output its name's minting node defines"
            );
            stripped.push(serde_json::json!([name, record]));
        }
        new["snapshot"]["appearance"] = serde_json::Value::Array(stripped);
        assert_eq!(
            old, new,
            "{file}: the base's save, its appearance keys unsited"
        );
        println!("{file}: the base's save under the total map");
    }
}
