//! LIB-PLACEDUNION — the group boolean as an index and a union
//! (`placed-union-places-and-fuses-in-one-node`, D10 `[repetition]`).
//!
//! The heat sink's fin group is one placement over an index and one
//! union reading its family whole; the die tour's tool is six
//! placements and one union over them. What is here is what the corpus
//! rows do not say: the twin equality against the chain the family
//! replaces, the memoized recompute, and that copies whose
//! arrangement the certificate cannot separate fuse rather than refuse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use editor_core::{DocEdit, Node, ProfileDoc, RecipeNodeId, TargetAxis, ValuePayload, apply};
use fixture::len;

use corpus::{body_of, documents, eval, failures};
use geom_core::Tol;

/// The fin prototype the heat-sink documents share, in a fresh
/// document: the extrude alone, no base, no group.
fn fin_only() -> (ProfileDoc, RecipeNodeId) {
    let mut r = corpus::Recorder::new();
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.25, 0.125),
            (0.4375, 0.125),
            (0.4375, 0.875),
            (0.25, 0.875),
        ]],
    );
    let fin = r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(0.8125),
    });
    (r.doc, fin)
}

/// The corpus document by name.
fn doc_named(name: &str) -> corpus::CorpusDoc {
    documents()
        .into_iter()
        .find(|d| d.name == name)
        .unwrap_or_else(|| panic!("corpus document {name}"))
}

/// `doc`'s union and placement nodes, counted.
fn unions_and_places(doc: &ProfileDoc) -> (usize, usize) {
    let mut out = (0, 0);
    for id in doc.ids() {
        match doc.node(id) {
            Some(Node::Union { .. }) => out.0 += 1,
            Some(Node::Place { .. }) => out.1 += 1,
            _ => {}
        }
    }
    out
}

/// The union of `members`, inserted.
fn union_of(
    doc: ProfileDoc,
    members: editor_core::Bodies<editor_core::BodyRead<editor_core::Formula>>,
) -> (ProfileDoc, RecipeNodeId) {
    let applied = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Union { members }),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("insert union");
    let id = applied.record.minted.expect("minted");
    (applied.doc, id)
}

// ---------------------------------------------------------------------
// The heat sink's fin union, in the document
// ---------------------------------------------------------------------

/// **One placement, one union, one body.** The grouped document says
/// with one placement over `k = index(fins)` and one union reading its
/// family what `heat_sink` says the same way beside its base — and the
/// union's value is an ordinary `Body`. Five disjoint fins are five
/// solids. Red if the fins need a placement each, or the union of a
/// family is not a body.
#[test]
fn the_fin_group_is_one_placement_and_one_union() {
    let d = doc_named("heat_sink_fins");
    // The fins' placement and the world placement of their union.
    assert_eq!(
        unions_and_places(&d.doc),
        (1, 2),
        "one union over one family"
    );
    let ev = eval::<f64>(&d.doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let root = d.result.expect("the group is the document's result");
    let body = body_of(&ev, root);
    assert_eq!(body.solids().count(), 5, "one solid per fin");
    assert_eq!(body.shells().count(), 5, "five shells, one per member");
    assert!(
        topo::validate_geometric(&body, Tol::witness()).is_ok(),
        "the group validates"
    );
}

/// **The twin equality.** The union of the family and the placement +
/// union chain it replaces are the same solid, bit for bit on both mass
/// oracles, against a chain built here from the same prototype. Red if
/// the family's members are not the chain's copies.
#[test]
fn the_fin_group_equals_the_placement_union_chain() {
    const PITCH: f64 = 0.3125;
    let (base_doc, fin) = fin_only();

    // The chain: five placed copies, unioned pairwise. The fins are
    // separated, so every union takes the disjoint path.
    let mut chain = base_doc.clone();
    let mut acc: Option<RecipeNodeId> = None;
    for i in 0..5 {
        let (doc, placed) = fixture::xform(
            chain,
            fin,
            [f64::from(i) * PITCH, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            0.0,
        );
        chain = doc;
        acc = Some(match acc {
            None => placed,
            Some(a) => {
                let (doc, id) = union_of(
                    chain,
                    editor_core::Bodies::Spelled(vec![a.into(), placed.into()]),
                );
                chain = doc;
                id
            }
        });
    }

    // The family: one placement over an index, one union reading it.
    let (doc, _, row) = fixture::row(base_doc, fin, ("k", 5), TargetAxis::X, PITCH);
    let (grouped, group) = union_of(doc, editor_core::Bodies::Family(row.into()));

    let ev_chain = eval::<f64>(&chain);
    let ev_group = eval::<f64>(&grouped);
    assert!(failures(&ev_chain).is_empty(), "{:?}", failures(&ev_chain));
    assert!(failures(&ev_group).is_empty(), "{:?}", failures(&ev_group));
    assert!(matches!(
        ev_group.value(group).map(|v| &v.payload),
        Some(ValuePayload::Boolean(_))
    ));
    let a = topo::test_support::finished(
        "a",
        topo::Body::clone(&(body_of(&ev_chain, acc.expect("the chain's root")))),
        Tol::witness(),
    );
    let b = topo::test_support::finished(
        "b",
        topo::Body::clone(&(body_of(&ev_group, group))),
        Tol::witness(),
    );
    let mass = |body| {
        topo::mass_properties(
            body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .expect("mass")
    };
    let (ma, mb) = (mass(&a), mass(&b));
    assert_eq!(
        ma.enclosure.volume, mb.enclosure.volume,
        "same volume, bit for bit"
    );
    assert_eq!(
        ma.enclosure.surface_area, mb.enclosure.surface_area,
        "same area, bit for bit"
    );
    assert_eq!(
        (a.faces().count(), a.edges().count(), a.vertices().count()),
        (b.faces().count(), b.edges().count(), b.vertices().count()),
        "same census"
    );
    assert_eq!(a.solids().count(), b.solids().count(), "same solid count");
}

/// **The memoized recompute survives the collapse.** A parameter edit
/// upstream of the group recomputes the group and nothing else; an
/// edit that touches nothing the group depends on reuses it. The
/// family's content key covers its values, so neither answer is luck.
#[test]
fn the_group_recomputes_exactly_when_its_inputs_move() {
    let d = doc_named("heat_sink_fins");
    let first = eval::<f64>(&d.doc);
    // Nothing changed: every node is reused.
    let again = editor_core::evaluate::<f64>(
        &d.doc,
        Some(&first),
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions::default(),
        Tol::witness(),
    );
    assert_eq!(
        again.recomputed, 0,
        "an unchanged document recomputes nothing"
    );
    // The fin's distance moves: the fin and the group recompute, the
    // profile does not.
    let bumped = d.bumped();
    let after = editor_core::evaluate::<f64>(
        &bumped,
        Some(&first),
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions::default(),
        Tol::witness(),
    );
    assert!(failures(&after).is_empty(), "{:?}", failures(&after));
    assert!(after.reused > 0, "the profile is reused across the bump");
    assert!(
        after.recomputed >= 2,
        "the fin extrude and the group both recompute"
    );
}

// ---------------------------------------------------------------------
// The die tour's tool
// ---------------------------------------------------------------------

/// **The tool is six placements and one union, and still cuts.** Each
/// pip says where it is (D10 `[repetition]`: an irregular family is its
/// members written out), and the union of the six is a legal boolean
/// operand: the document subtracts it from the cube. Red if the tool
/// needs a union per pip.
#[test]
fn the_die_tool_is_one_union_and_still_cuts() {
    let d = doc_named("die_tool");
    let (unions, places) = unions_and_places(&d.doc);
    assert_eq!(unions, 1, "one union over the six pips");
    assert!(places >= 6, "a placement per pip: {places}");
    let ev = eval::<f64>(&d.doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let pipped = body_of(&ev, d.result.expect("the pipped die"));
    assert!(topo::validate_geometric(&pipped, Tol::witness()).is_ok());
    assert_eq!(pipped.solids().count(), 1);
    // Every pip cuts its cap: the die is the cube less six caps of
    // height `PIP_H` off balls of radius `PIP_R`.
    let (l, r, h) = (
        corpus::die_pips::DIE_L,
        corpus::die_pips::PIP_R,
        corpus::die_pips::PIP_H,
    );
    let cap = std::f64::consts::PI * h * h * (3.0 * r - h) / 3.0;
    let volume = topo::mass_properties(
        &topo::test_support::finished("pipped", topo::Body::clone(&pipped), Tol::witness()),
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .expect("mass")
    .enclosure
    .volume;
    let want = l * l * l - 6.0 * cap;
    assert!(
        (volume - want).abs() < 1e-3 * cap,
        "six caps cut: {volume} against {want}"
    );
}

// ---------------------------------------------------------------------
// An arrangement the certificate cannot separate
// ---------------------------------------------------------------------

/// **Overlapping copies fuse.** Two unit boxes half a unit apart are a
/// pair the separation certificate cannot hold apart, so the union
/// judges them and fuses them: one solid of volume 1.5. Red if an
/// uncertified pair refuses, as the group boolean's certificate did.
#[test]
fn overlapping_copies_fuse_rather_than_refuse() {
    let mut r = corpus::Recorder::new();
    let p = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let cube = r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    let (doc, _, row) = fixture::row(r.doc, cube, ("k", 2), TargetAxis::X, 0.5);
    let (doc, fused) = union_of(doc, editor_core::Bodies::Family(row.into()));
    let ev = eval::<f64>(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let body = body_of(&ev, fused);
    assert_eq!(body.solids().count(), 1, "the two boxes fuse");
    let volume = topo::mass_properties(
        &topo::test_support::finished("fused", topo::Body::clone(&body), Tol::witness()),
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .expect("mass")
    .enclosure
    .volume;
    assert!((volume - 1.5).abs() < 1e-9, "{volume}");
}
