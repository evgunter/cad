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
//!   (NAMES N2, the Split op's pieces) — resolution names the read, and
//!   the other half's read holds the name now;
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
    resolve, resolve_with_prior,
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
/// moves it to the other half's table, where the other half's read
/// resolves it; through the read of the half it was in, it vanishes,
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
        matches!(
            resolve(new, below, &riser).unwrap(),
            Resolution::Resolved(_)
        ),
        "the lower half's read holds it"
    );
    match resolve_with_prior(new, prior, above, &riser).unwrap() {
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
        let Resolution::Resolved(r) = resolve(ctx, read, &section).unwrap() else {
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
    let face = |half| match resolve(ctx, read(half), &section).unwrap() {
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

/// A fragment of an extrude face that both of `split`'s halves hold,
/// with the face it fragments.
fn shared_fragment(
    ev: &editor_core::Evaluation<f64>,
    doc: &ProfileDoc,
    split: RecipeNodeId,
) -> (StableName, StableName) {
    let (above, below) = (
        doc.output(split, 0).expect("above"),
        doc.output(split, 1).expect("below"),
    );
    let (ta, tb) = (
        ev.table(above).expect("above"),
        ev.table(below).expect("below"),
    );
    ta.iter()
        .find_map(|(n, _)| match n.path.last() {
            Some(RoleSeg::SplitFragment { parent })
                if n.kind == EntityKind::Face && tb.lookup(n).is_some() =>
            {
                Some((n.clone(), (**parent).clone()))
            }
            _ => None,
        })
        .expect("a wall fragment both halves hold")
}

/// **A vanished name's last-good entry is the one its read held**
/// (PR #4674 review): a fragment both halves hold, referenced through
/// the lower half's read, leaves when the plane leaves the body; its
/// tombstone is the lower half's fragment, said in the split's value
/// (body 1), never the upper half's.
#[test]
fn a_vanished_shared_fragments_tombstone_is_its_own_reads() {
    let (doc, _ext, tool, split) = step_split(1.0);
    let below = doc.output(split, 1).expect("below");
    let before = fixture::run(&doc, &Default::default());
    let (frag, _) = shared_fragment(&before, &doc, split);
    let prior = RunCtx {
        doc: &doc,
        eval: &before,
    };
    let Resolution::Resolved(was) = resolve(prior, below, &frag).unwrap() else {
        panic!("the lower read resolves the fragment");
    };
    assert_eq!(was.entity.body, 1, "the lower half is the value's body 1");
    let (moved, _) = fixture::step(
        doc.clone(),
        DocEdit::SetParam {
            node: tool,
            slot: SlotId::Origin(Axis3::X),
            value: len(5.0).into(),
            fresh: Vec::new(),
        },
    );
    let after = fixture::run(&moved, &Default::default());
    let new = RunCtx {
        doc: &moved,
        eval: &after,
    };
    let Resolution::Failed(failed) = resolve_with_prior(new, prior, below, &frag).unwrap() else {
        panic!("the lower read lost the fragment");
    };
    let ResolveError::Vanished {
        last_good: Some(ghost),
        ..
    } = failed.error
    else {
        panic!("vanished, with a tombstone: {:?}", failed.error);
    };
    assert_eq!(
        (ghost.patch.node, ghost.patch.entity),
        (was.node, was.entity),
        "the ghost is the lower half's fragment"
    );
}

/// **A re-attachment offer names every read holding the name**: a key
/// on a face the split cuts is offered the face's fragment in each half
/// that holds it, so either half can be repainted from the offers.
#[test]
fn a_rebind_offer_names_every_read_holding_a_shared_name() {
    let (doc, ext, _, split) = step_split(1.0);
    let ev = fixture::run(&doc, &Default::default());
    let (frag, parent) = shared_fragment(&ev, &doc, split);
    let (above, below) = (doc.output(split, 0).unwrap(), doc.output(split, 1).unwrap());
    let (painted, _) = fixture::step(
        doc.clone(),
        DocEdit::SetAppearance {
            body: fixture::out(&doc, ext),
            name: parent.clone(),
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
        },
    );
    let ev = fixture::run(&painted, &Default::default());
    let offers = editor_core::appearance_rebind_suggestions(painted.appearance(), &ev);
    let mine = offers
        .iter()
        .find(|(key, _)| key.name == parent)
        .map(|(_, offers)| offers.clone())
        .expect("the key has an offer row");
    let reads: Vec<_> = mine
        .iter()
        .filter(|o| o.name == frag)
        .map(|o| o.at)
        .collect();
    assert!(
        reads.contains(&above) && reads.contains(&below),
        "both halves hold the fragment, so both reads are offered: {reads:?}"
    );
}

/// **An appearance entry is read in a body**: a datum plane's output is
/// a live read holding no body, and a deleted operation's read is no
/// live read at all, so `SetAppearance` refuses both; deleting the
/// operation a key's read is an output of strands the key `Took::Body`.
#[test]
fn an_appearance_read_must_hold_a_body_and_its_delete_strands_the_key() {
    let (doc, ext, tool, split) = step_split(1.0);
    let riser = fixture::fname(ext, fixture::wall(&doc, ext, 3));
    let paint = |doc: &ProfileDoc, body| {
        editor_core::apply(
            doc,
            &DocEdit::SetAppearance {
                body,
                name: riser.clone(),
                attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
    };
    let plane = doc.output(tool, 0).expect("the plane's output");
    assert!(
        matches!(
            paint(&doc, plane),
            Err(editor_core::EditError::AppearanceBodyNotARead { .. })
        ),
        "a datum plane's read holds no body"
    );
    let above = doc.output(split, 0).unwrap();
    let painted = paint(&doc, above).expect("a half's read holds a body").doc;
    let applied = editor_core::apply(
        &painted,
        &DocEdit::DeleteNode { id: split },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a delete strands, it does not refuse");
    assert!(
        applied.maintenance.iter().any(|m| matches!(
            m,
            editor_core::Maintenance::StrandedAppearance {
                took: editor_core::Took::Body,
                ..
            }
        )),
        "the delete strands the key by its read: {:?}",
        applied.maintenance
    );
    assert!(
        matches!(
            paint(&applied.doc, above),
            Err(editor_core::EditError::AppearanceBodyNotARead { .. })
        ),
        "a deleted operation's read is no body read"
    );
}

/// **A node named alone is one read in every door**: a revolve's body
/// beside its axis is the node's one body, to the document and to the
/// evaluation alike; a split's two halves are no one read to either.
#[test]
fn a_node_named_alone_is_the_same_read_to_the_document_and_the_evaluation() {
    let doc = ProfileDoc::empty_derived("dm4_named_alone", Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        vec![fixture::square(2.0, 0.0, 0.5)],
    );
    let (doc, revolve) = insert(
        doc,
        Node::Revolve {
            profile: profile.into(),
            axis_origin: [len(0.0), len(0.0)],
            axis_direction: [scl(0.0), scl(1.0)],
            angle: fixture::ang(std::f64::consts::TAU),
        },
    );
    let ev = fixture::run(&doc, &Default::default());
    assert!(doc.read_of_node(revolve).is_some(), "the revolve's body");
    assert_eq!(ev.at_var(revolve), doc.read_of_node(revolve));
    let (doc, _, _, split) = step_split(1.0);
    let ev = fixture::run(&doc, &Default::default());
    assert_eq!((ev.at_var(split), doc.read_of_node(split)), (None, None));
}

/// **A re-point that takes a key's name out of its read's reach strands
/// the key** (PR #4674 review), as it strands a selection there: a key
/// read in a transform's output, its name the input block's cap; the
/// transform re-pointed at another block reports the key `Took::Reach`.
#[test]
fn a_repoint_that_takes_a_keys_name_out_of_reach_strands_it() {
    let doc = ProfileDoc::empty_derived("dm4_repoint", Tol::witness());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let sq = |x: f64| vec![vec![(x, 0.0), (x + 1.0, 0.0), (x + 1.0, 1.0), (x, 1.0)]];
    let (doc, pa) = insert(doc, Node::Profile(fixture::desc(plane, sq(0.0))));
    let (doc, pb) = insert(doc, Node::Profile(fixture::desc(plane, sq(3.0))));
    let ext = |p: RecipeNodeId| Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
        side: ExtrudeSide::Along,
    };
    let (doc, a) = insert(doc, ext(pa));
    let (doc, b) = insert(doc, ext(pb));
    let (doc, t) = insert(
        doc,
        fixture::xform(a, [0.0, 0.0, 2.0], [0.0, 0.0, 1.0], 0.0),
    );
    let cap = fixture::fname(a, RoleSeg::Cap(editor_core::CapEnd::End));
    let (doc, _) = fixture::step(
        doc.clone(),
        DocEdit::SetAppearance {
            body: fixture::out(&doc, t),
            name: cap,
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
        },
    );
    assert!(
        fixture::run(&doc, &Default::default())
            .appearance
            .is_lossless()
    );
    let applied = editor_core::apply(
        &doc,
        &DocEdit::SetParam {
            node: t,
            slot: SlotId::Operand(editor_core::OperandSlot::Input),
            value: editor_core::Operand::from(fixture::out(&doc, b)).into(),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a re-point applies");
    assert!(
        applied.maintenance.iter().any(|m| matches!(
            m,
            editor_core::Maintenance::StrandedAppearance {
                took: editor_core::Took::Reach,
                ..
            }
        )),
        "the key's name left its read's reach: {:?}",
        applied.maintenance
    );
}

/// A cube and a transform of it: the cube's cap, carried through the
/// transform's read under the cube's own name.
fn cube_and_transform() -> (ProfileDoc, RecipeNodeId, RecipeNodeId, StableName) {
    let doc = ProfileDoc::empty_derived("dm4_read_gone", Tol::witness());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(fixture::desc(
            plane,
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let (doc, cube) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, moved) = insert(
        doc,
        fixture::xform(cube, [0.0, 0.0, 2.0], [0.0, 0.0, 1.0], 0.0),
    );
    let cap = fixture::fname(cube, RoleSeg::Cap(editor_core::CapEnd::End));
    (doc, cube, moved, cap)
}

/// **A reference whose read is gone is stranded with its read, first**
/// (N5's `ReadGone`, rung 0): the transform deleted, its read went with
/// it, and the cap read there answers `ReadGone { Deleted }` — never a
/// `Vanished` that blames the cube, which did not change. A read the
/// document never minted is `Foreign`. With the cube deleted too, the
/// name's own node is gone as well, and the read still answers first.
#[test]
fn a_reference_whose_read_is_gone_is_read_gone_before_node_gone() {
    let (doc, cube, moved, cap) = cube_and_transform();
    let read = fixture::out(&doc, moved);
    let gone_read = |doc: &ProfileDoc, read| {
        let ev = fixture::run(doc, &Default::default());
        match resolve(RunCtx { doc, eval: &ev }, read, &cap).expect("a held read, or none") {
            Resolution::Failed(f) => f.error,
            other => panic!("a reference with no read must fail: {other:?}"),
        }
    };
    let (no_transform, _) = fixture::step(doc.clone(), DocEdit::DeleteNode { id: moved });
    assert_eq!(
        gone_read(&no_transform, read),
        ResolveError::ReadGone {
            name: cap.clone(),
            read,
            edit: editor_core::ReadEdit::Deleted,
        }
    );
    let never = editor_core::VarId::new(0, 0);
    assert!(matches!(
        gone_read(&doc, never),
        ResolveError::ReadGone {
            edit: editor_core::ReadEdit::Foreign,
            ..
        }
    ));
    let (neither, _) = fixture::step(no_transform, DocEdit::DeleteNode { id: cube });
    assert!(
        matches!(gone_read(&neither, read), ResolveError::ReadGone { .. }),
        "the site read answers ahead of the name's node"
    );
    // A loss on a key read there is the same rung, and enriches to it.
    let (painted, _) = fixture::step(
        doc.clone(),
        DocEdit::SetAppearance {
            body: read,
            name: cap.clone(),
            attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(1, 2, 3)),
        },
    );
    let (painted, _) = fixture::step(painted, DocEdit::DeleteNode { id: moved });
    let ev = fixture::run(&painted, &Default::default());
    let [loss] = ev.appearance.losses.as_slice() else {
        panic!("one loss: {:?}", ev.appearance.losses);
    };
    assert_eq!(loss.cause, editor_core::AppearanceLossCause::ReadGone);
    let enriched = editor_core::enrich_appearance_loss(
        RunCtx {
            doc: &painted,
            eval: &ev,
        },
        loss,
    );
    assert!(matches!(
        enriched,
        Resolution::Failed(editor_core::ResolutionFailure {
            error: ResolveError::ReadGone { .. },
            ..
        })
    ));
}

/// **A read that holds no body is no question to ask**: a live scalar
/// variable, and a datum's output, refuse `ReadNotAnOutput` — typed,
/// never a `Resolution`. A node handle whose node went, holding a name
/// it did not mint, refuses `NodeNotHeld`; one holding a name it minted
/// answers `NodeGone`, about that node.
#[test]
fn a_read_that_holds_no_body_is_refused_and_a_gone_node_handle_says_so() {
    let (doc, cube, moved, cap) = cube_and_transform();
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DefineVar {
            var: editor_core::VarName::from_static("depth").into(),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(
                editor_core::Dimension::Length,
                0.9,
            )),
            fresh: Vec::new(),
        },
    );
    let depth = doc.var_named("depth").expect("the scalar");
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let ev = fixture::run(&doc, &Default::default());
    let ctx = RunCtx {
        doc: &doc,
        eval: &ev,
    };
    for read in [depth, fixture::out(&doc, plane)] {
        assert_eq!(
            resolve(ctx, read, &cap),
            Err(editor_core::ResolveRefusal::ReadNotAnOutput { read })
        );
    }
    let (gone, _) = fixture::step(doc.clone(), DocEdit::DeleteNode { id: moved });
    assert_eq!(
        editor_core::stranded_at(&gone, moved, &cap),
        Some(Err(editor_core::ResolveRefusal::NodeNotHeld {
            node: moved
        }))
    );
    let (gone, _) = fixture::step(gone, DocEdit::DeleteNode { id: cube });
    assert!(matches!(
        editor_core::stranded_at(&gone, cube, &cap),
        Some(Ok(Resolution::Failed(editor_core::ResolutionFailure {
            error: ResolveError::NodeGone { .. },
            ..
        })))
    ));
    assert_eq!(editor_core::stranded_at(&doc, cube, &cap), None);
}
