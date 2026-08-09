//! **LIB-SEL1 — geometric selectors** (`docs/SELECT-DESIGN.md` §§1-2).
//!
//! The LB7 fence comes down here, and what replaces it is a SPLIT, not
//! a uniform "geometric predicate" bucket. Three things are pinned.
//!
//! 1. **EXACT atoms are total.** Carrier kind, surface kind and the
//!    unordered adjacent-kind pair are tag reads: no funnel, no
//!    margin, no refusal — a wrong-kind name or a missing carrier is
//!    an honest NO. `select_where` with an empty conjunction is
//!    `select`, exactly.
//! 2. **The DECIDED atom refuses rather than lies.** A datum-distance
//!    margin inside the ambiguity band does not silently include OR
//!    exclude its candidate; it comes back as
//!    `SelectRefusal::InBand` naming the candidate. That is the G1
//!    NOTE-2 lesson — the escalation path is a tested row, not a
//!    documented intention.
//! 3. **The query's static faults are refusals too**: a value
//!    expression that is not a length, a `datum` reference that is not
//!    an evaluated datum.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    CancelToken, CapEnd, Cmp, CurveKind, CurveKindSet, Datum, Dimension, EntityKind,
    EvalOptions, Expr, GeomPred, NamePat, Node, ParamEnv, ProfileDoc, RecipeNodeId, SegPat, SegTag,
    SelectRefusal, Selector, SurfaceKindSet, evaluate, select, select_where,
};
use geom_brep::SurfaceKind;

use fixture::{insert, len};

fn eval(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default())
}

/// A unit box as an extruded square, plus a datum PLANE on z = 0 with
/// its normal along +z — the frame every position row below measures
/// against, referenced as a node exactly like any other input (GS-Q6).
fn box_doc() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, p) = insert(
        ProfileDoc::empty(),
        Node::Profile(fixture::desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let (doc, cube) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    );
    let (doc, datum) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.0)],
            normal: [
                fixture::scl(0.0),
                fixture::scl(0.0),
                fixture::scl(1.0),
            ],
        }),
    );
    (doc, cube, datum)
}

fn no_params() -> ParamEnv<f64> {
    ProfileDoc::empty().param_env::<f64>()
}

fn all(kind: EntityKind) -> Selector {
    Selector::of(NamePat::of_kind(kind))
}

// ------------------------------------------------------------------
// 1. EXACT atoms: tag reads, total, no funnel.
// ------------------------------------------------------------------

/// An empty conjunction IS the structural selector — the same names,
/// in the same canonical order, and infallibly.
#[test]
fn empty_geometry_is_plain_select() {
    let (doc, cube, _) = box_doc();
    let ev = eval(&doc);
    for kind in [
        EntityKind::Face,
        EntityKind::Edge,
        EntityKind::Vertex,
        EntityKind::Body,
    ] {
        let structural = select(&ev, cube, &all(kind));
        let filtered = select_where(&ev, cube, &all(kind), &[], &no_params()).unwrap();
        assert_eq!(structural, filtered, "{kind:?}");
    }
}

/// Every edge of a box is a LINE, and no face is: an exact atom
/// answers NO for the wrong entity kind rather than refusing.
#[test]
fn carrier_kind_selects_the_box_edges() {
    let (doc, cube, _) = box_doc();
    let ev = eval(&doc);
    let lines = [GeomPred::CurveKind(CurveKindSet::just(CurveKind::Line))];

    let edges = select_where(&ev, cube, &all(EntityKind::Edge), &lines, &no_params()).unwrap();
    assert_eq!(edges, select(&ev, cube, &all(EntityKind::Edge)));
    assert_eq!(edges.len(), 12);

    // A curve atom against FACE names: total, and totally false.
    assert!(
        select_where(&ev, cube, &all(EntityKind::Face), &lines, &no_params())
            .unwrap()
            .is_empty()
    );
    // An EMPTY kind set matches nothing, like an empty Selector.
    assert!(
        select_where(
            &ev,
            cube,
            &all(EntityKind::Edge),
            &[GeomPred::CurveKind(CurveKindSet::default())],
            &no_params(),
        )
        .unwrap()
        .is_empty()
    );
}

/// Six planar faces, and the structural narrowing still composes: the
/// geometric stage filters SURVIVORS of the pattern, never the table.
#[test]
fn surface_kind_selects_the_box_faces() {
    let (doc, cube, _) = box_doc();
    let ev = eval(&doc);
    let planes = [GeomPred::SurfaceKind(SurfaceKindSet::just(
        SurfaceKind::Plane,
    ))];
    let faces = select_where(&ev, cube, &all(EntityKind::Face), &planes, &no_params()).unwrap();
    assert_eq!(faces.len(), 6);

    let top_cap = Selector::of(
        NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(CapEnd::Top)),
    );
    assert_eq!(
        select_where(&ev, cube, &top_cap, &planes, &no_params()).unwrap(),
        select(&ev, cube, &top_cap),
    );
    // No sphere anywhere on a box.
    assert!(
        select_where(
            &ev,
            cube,
            &all(EntityKind::Face),
            &[GeomPred::SurfaceKind(SurfaceKindSet::just(
                SurfaceKind::Sphere
            ))],
            &no_params(),
        )
        .unwrap()
        .is_empty()
    );
}

/// The adjacent pair is UNORDERED, which is what makes it equivariant
/// under a reflection that swaps the two sides: `(Plane, Plane)` finds
/// every box edge whichever half-edge carries which face.
#[test]
fn adjacent_kinds_is_unordered() {
    let (doc, cube, _) = box_doc();
    let ev = eval(&doc);
    let plane = SurfaceKindSet::just(SurfaceKind::Plane);
    let sphere = SurfaceKindSet::just(SurfaceKind::Sphere);

    let both = select_where(
        &ev,
        cube,
        &all(EntityKind::Edge),
        &[GeomPred::AdjacentKinds(plane, plane)],
        &no_params(),
    )
    .unwrap();
    assert_eq!(both.len(), 12);

    for pair in [(plane, sphere), (sphere, plane)] {
        assert!(
            select_where(
                &ev,
                cube,
                &all(EntityKind::Edge),
                &[GeomPred::AdjacentKinds(pair.0, pair.1)],
                &no_params(),
            )
            .unwrap()
            .is_empty(),
            "no plane/sphere rim on a box, either way round",
        );
    }
}

/// A CONJUNCTION: both atoms must hold, and a contradictory pair is
/// empty rather than surprising.
#[test]
fn atoms_conjoin() {
    let (doc, cube, _) = box_doc();
    let ev = eval(&doc);
    let plane = SurfaceKindSet::just(SurfaceKind::Plane);
    let conj = [
        GeomPred::CurveKind(CurveKindSet::just(CurveKind::Line)),
        GeomPred::AdjacentKinds(plane, plane),
    ];
    assert_eq!(
        select_where(&ev, cube, &all(EntityKind::Edge), &conj, &no_params())
            .unwrap()
            .len(),
        12
    );
    let contradiction = [
        GeomPred::CurveKind(CurveKindSet::just(CurveKind::Line)),
        GeomPred::CurveKind(CurveKindSet::just(CurveKind::Circle)),
    ];
    assert!(
        select_where(
            &ev,
            cube,
            &all(EntityKind::Edge),
            &contradiction,
            &no_params()
        )
        .unwrap()
        .is_empty()
    );
}

// ------------------------------------------------------------------
// 2. The DECIDED atom: datum-relative position through the funnel.
// ------------------------------------------------------------------

fn at(datum: RecipeNodeId, cmp: Cmp, v: f64) -> [GeomPred; 1] {
    [GeomPred::DatumDistance {
        datum,
        cmp,
        value: len(v),
    }]
}

/// The three comparison arms partition the box's vertices about the
/// z = 0 datum plane: four ON it, four a metre above, none below.
#[test]
fn datum_distance_partitions_by_sign() {
    let (doc, cube, datum) = box_doc();
    let ev = eval(&doc);
    let v = all(EntityKind::Vertex);
    let n = |cmp, value| {
        select_where(&ev, cube, &v, &at(datum, cmp, value), &no_params())
            .unwrap()
            .len()
    };
    assert_eq!(n(Cmp::Approx, 0.0), 4, "the bottom cap sits ON the datum");
    assert_eq!(n(Cmp::Approx, 1.0), 4, "the top cap is one metre up");
    assert_eq!(n(Cmp::Greater, 0.0), 4);
    assert_eq!(n(Cmp::Less, 0.0), 0, "the box is entirely above");
    assert_eq!(n(Cmp::Less, 1.0), 4);
    assert_eq!(n(Cmp::Greater, 1.0), 0);
    assert_eq!(select(&ev, cube, &v).len(), 8);
}

/// The face-side reading of "where is it": a face answers with its
/// carrier frame origin (LIB-U5's own point), so the top cap is the
/// one plane at distance 1.
#[test]
fn datum_distance_reads_face_frames() {
    let (doc, cube, datum) = box_doc();
    let ev = eval(&doc);
    let top = select_where(
        &ev,
        cube,
        &all(EntityKind::Face),
        &at(datum, Cmp::Approx, 1.0),
        &no_params(),
    )
    .unwrap();
    let named = select(
        &ev,
        cube,
        &Selector::of(
            NamePat::of_kind(EntityKind::Face).seg(SegPat::tag(SegTag::Cap).side(CapEnd::Top)),
        ),
    );
    assert_eq!(top, named, "the decided atom finds what the role path does");
}

/// **The escalation path is a tested row.** A datum plane placed so
/// that a candidate's margin lands strictly inside the ambiguity band
/// refuses and NAMES the candidate — it does not quietly include it
/// (a razor-thin selection cliff) or quietly drop it (the
/// staleness-shaped sin).
#[test]
fn an_in_band_margin_refuses_typed() {
    let (doc, cube, datum) = box_doc();
    let ev = eval(&doc);
    let tol = geom_core::Tolerance::get();
    // Inside (eps, k*eps): definite neither way.
    let sliver = tol.eps * (1.0 + tol.k) / 2.0;
    let refusal = select_where(
        &ev,
        cube,
        &all(EntityKind::Vertex),
        &at(datum, Cmp::Approx, sliver),
        &no_params(),
    )
    .expect_err("an in-band margin is a refusal, not a filter outcome");
    match refusal {
        SelectRefusal::InBand {
            name, predicate, ..
        } => {
            assert_eq!(predicate, editor_core::SEL_DATUM_DISTANCE);
            assert_eq!(name.kind, EntityKind::Vertex);
        }
        other => panic!("expected InBand, got {other:?}"),
    }
}

/// A datum-distance atom against a whole-BODY name has no point to
/// measure, and says so rather than dropping the name.
#[test]
fn an_unreadable_candidate_refuses_typed() {
    let (doc, cube, datum) = box_doc();
    let ev = eval(&doc);
    let refusal = select_where(
        &ev,
        cube,
        &all(EntityKind::Body),
        &at(datum, Cmp::Approx, 0.0),
        &no_params(),
    )
    .expect_err("a body has no position");
    assert!(matches!(refusal, SelectRefusal::Unreadable { .. }));
}

// ------------------------------------------------------------------
// 3. Static faults of the query itself.
// ------------------------------------------------------------------

/// The comparand of a distance must BE a distance.
#[test]
fn a_non_length_value_refuses() {
    let (doc, cube, datum) = box_doc();
    let ev = eval(&doc);
    let bad = [GeomPred::DatumDistance {
        datum,
        cmp: Cmp::Approx,
        value: Expr::literal(1.0, Dimension::Angle).unwrap(),
    }];
    assert!(matches!(
        select_where(&ev, cube, &all(EntityKind::Vertex), &bad, &no_params()),
        Err(SelectRefusal::NotALength {
            dim: Dimension::Angle
        })
    ));
}

/// A `datum` reference that names something else is a refusal, and the
/// refusal says what the node actually produced.
#[test]
fn a_non_datum_reference_refuses() {
    let (doc, cube, _) = box_doc();
    let ev = eval(&doc);
    let bad = at(cube, Cmp::Approx, 0.0);
    match select_where(&ev, cube, &all(EntityKind::Vertex), &bad, &no_params()) {
        Err(SelectRefusal::NotADatum { datum, found }) => {
            assert_eq!(datum, cube);
            assert_ne!(found, "datum");
        }
        other => panic!("expected NotADatum, got {other:?}"),
    }
    // An unevaluated node id, same door.
    let ghost = at(RecipeNodeId(9999), Cmp::Approx, 0.0);
    assert!(matches!(
        select_where(&ev, cube, &all(EntityKind::Vertex), &ghost, &no_params()),
        Err(SelectRefusal::NotADatum { .. })
    ));
}

/// A valueless node materializes EMPTY, not an error — the structural
/// contract, unchanged.
#[test]
fn a_valueless_node_is_empty_not_an_error() {
    let (doc, _, _) = box_doc();
    let ev = eval(&doc);
    assert!(
        select_where(
            &ev,
            RecipeNodeId(9999),
            &all(EntityKind::Edge),
            &[GeomPred::CurveKind(CurveKindSet::just(CurveKind::Line))],
            &no_params(),
        )
        .unwrap()
        .is_empty()
    );
}

// ------------------------------------------------------------------
// 4. The mirrors cannot drift.
// ------------------------------------------------------------------

/// `ALL_SURFACE_KINDS` is the iteration order of a `SurfaceKindSet`,
/// and the bitset's exhaustive `surface_bit` match is the compile-time
/// tripwire. This pins the pair: every listed kind is a member of the
/// set built from the whole list, exactly once.
#[test]
fn the_surface_kind_mirror_is_complete() {
    let all = SurfaceKindSet::of(editor_core::ALL_SURFACE_KINDS);
    assert_eq!(
        all.iter().count(),
        editor_core::ALL_SURFACE_KINDS.len(),
        "a kind added to SurfaceKind must be added to ALL_SURFACE_KINDS",
    );
    for k in editor_core::ALL_SURFACE_KINDS {
        assert!(all.contains(k));
        assert!(!SurfaceKindSet::default().contains(k));
        assert_eq!(SurfaceKindSet::just(k).iter().count(), 1);
    }
    let curves = CurveKindSet::of(CurveKind::ALL);
    assert_eq!(curves.iter().count(), CurveKind::ALL.len());
    for k in CurveKind::ALL {
        assert!(curves.contains(k));
        assert_eq!(CurveKindSet::just(k).iter().next(), Some(k));
    }
    // Sets are canonical values: order and repetition of the
    // constructor's input cannot make two equal sets differ.
    assert_eq!(
        CurveKindSet::of([CurveKind::Line, CurveKind::Circle, CurveKind::Line]),
        CurveKindSet::of([CurveKind::Circle, CurveKind::Line]),
    );
}
