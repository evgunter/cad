//! **The die** (M5 PR 12, acceptance shape (v)): constant-radius
//! rolling-ball fillets, made visible.
//!
//! Three stops, and since LIB-SEL1 **ONE recipe document** behind all
//! three — the die is a `Doc`, its stops are three of its nodes, and
//! every edge set it blends is a `select_where` call rather than a
//! hand-rolled loop over a kernel body. That is the whole point of the
//! scene now: the die is the tour's fillet showcase AND its
//! geometric-selector showcase, because the two are the same model.
//!
//! - **the blank** — a unit cube with all twelve edges filleted. Six
//!   shrunk planar faces, twelve quarter-cylinder blends, eight
//!   sphere-octant corner patches: 26 faces, 48 edges, 24 vertices.
//!   Every open chain terminates in a three-convex-edge corner (OQ6's
//!   one in-scope configuration), every trimline is stored
//!   `TangentIntersection` from birth, and the corner trimlines are
//!   CIRCLES — a jet-certificate class this unit had to retire into
//!   the lane before it could store them honestly. The selection is
//!   `all_edges`, the every-edge materializer.
//! - **the pips** — 21 spherical dimples on the six faces of a sharp
//!   cube, cut in ONE certified group operation (S13's closed-group
//!   arm), each ball charted with its pole along the face it is cut
//!   by.
//! - **the composed die** (M6 unit 1) — the pipped cube's twelve box
//!   edges blended IN PLACE (the pip rims carried through as rings)
//!   and all 21 rims replaced by torus bands: one body, tier-3
//!   certified, closed-form volume, watertight. The M5 frontier rows
//!   (`deviation_1_*`) are flipped in
//!   `crates/sweep/tests/m5_pr12_die.rs`; the full ladder lives in
//!   `crates/sweep/tests/m6_surgery.rs`. Its two selections are the
//!   geometric vocabulary's EXACT atoms, said as calls.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use pncad::document::{BooleanOp, BooleanValue};
use pncad::prelude::{
    CancelToken, CurveKind, CurveKindSet, Datum, Dimension, Doc, DocEdit, EntityKind, EvalOptions,
    Evaluation, Expr, GeomPred, LoopProgram, NamePat, Node, Point3, ProfileProgram, ProgramArcData,
    ProgramStep, ProgramTarget, RecipeNodeId, Selector, SketchPlane, SurfaceKind, SurfaceKindSet,
    ValuePayload, Vec3, all_edges, apply, evaluate, select_where,
};
use pncad::topo::Body;

use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::geom_core::Tol;

const L: f64 = 1.0;
const R: f64 = 0.12;
/// The pip-rim blend radius (the composed stop's second call).
const RIM_R: f64 = 0.02;
const PIP_R: f64 = 0.09;
const PIP_H: f64 = 0.05;
const PIP_D: f64 = 0.22;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a length")
}
fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).expect("an angle")
}
fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("a scalar")
}

fn layout(n: u32) -> Vec<(f64, f64)> {
    let c = vec![(0.0, 0.0)];
    let diag = vec![(-1.0, -1.0), (1.0, 1.0)];
    let anti = vec![(-1.0, 1.0), (1.0, -1.0)];
    let sides = vec![(-1.0, 0.0), (1.0, 0.0)];
    match n {
        1 => c,
        2 => diag,
        3 => [diag.clone(), c].concat(),
        4 => [diag.clone(), anti.clone()].concat(),
        5 => [diag.clone(), anti.clone(), c].concat(),
        _ => [diag, anti, sides].concat(),
    }
}

/// One pip's placement: where its centre goes, and the EXACT rotation
/// carrying the master ball's `+Z` pole onto the face normal it is cut
/// by (the chart discipline the plane×sphere section needs).
///
/// The six rotations are AUTHORED, one per face, rather than derived
/// by a runtime `cross`/`acos` branch: `Node::Transform` is one
/// unconditional fused `(R, t)` pass, so the placement wants to be
/// data. Every angle here is `0`, `±π/2` or `π` about a coordinate
/// axis — the same six placements the runtime branch used to compute.
struct Placement {
    centre: [f64; 3],
    axis: [f64; 3],
    angle: f64,
}

const X: [f64; 3] = [1.0, 0.0, 0.0];
const Y: [f64; 3] = [0.0, 1.0, 0.0];
const Z: [f64; 3] = [0.0, 0.0, 1.0];
const NEG_X: [f64; 3] = [-1.0, 0.0, 0.0];
const NEG_Y: [f64; 3] = [0.0, -1.0, 0.0];
const NEG_Z: [f64; 3] = [0.0, 0.0, -1.0];

fn placements() -> Vec<Placement> {
    let h = L / 2.0;
    // pip count, outward normal, the two in-plane axes the layout
    // offsets ride, then the axis and angle carrying +Z onto that
    // normal.
    let faces = [
        (1u32, Z, X, Y, Z, 0.0),
        (6, NEG_Z, X, Y, X, PI),
        (2, X, Y, Z, Y, FRAC_PI_2),
        (5, NEG_X, Y, Z, Y, -FRAC_PI_2),
        (3, Y, Z, X, X, -FRAC_PI_2),
        (4, NEG_Y, Z, X, X, FRAC_PI_2),
    ];
    let mut out = Vec::new();
    for (n, normal, ex, ey, axis, angle) in faces {
        // The centre stands `PIP_R - PIP_H` OUTSIDE the face plane, so
        // the cavity it cuts is exactly a cap of height `PIP_H`.
        let d = h + (PIP_R - PIP_H);
        for (u, w) in layout(n) {
            let centre = core::array::from_fn(|i| {
                h + normal[i] * d + ex[i] * (u * PIP_D) + ey[i] * (w * PIP_D)
            });
            out.push(Placement {
                centre,
                axis,
                angle,
            });
        }
    }
    out
}

/// The pip ball's meridian: one bulge-1 semicircle from pole to pole,
/// closed by the on-axis diameter. Every vertex is ON the revolve
/// axis, which is what a sphere's meridian IS — the revolve names its
/// poles from the sweep's construction record (M9-D1), so the natural
/// three-step program is what the document authors.
fn half_disc() -> LoopProgram {
    let p = |x: f64, y: f64| [len(x), len(y)];
    LoopProgram::Chain(vec![
        ProgramStep::At(p(0.0, -PIP_R)),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(p(0.0, PIP_R)),
            b: scl(1.0),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

/// The die's recipe, and the three nodes the three stops read.
struct Die {
    doc: Doc<ProfileProgram>,
    blank: RecipeNodeId,
    pipped: RecipeNodeId,
    composed: RecipeNodeId,
    /// What the two geometric selections actually materialized, kept
    /// for the captions: the twelve box edges and the 42 rim arcs.
    box_edges: usize,
    rims: usize,
}

fn eval(doc: &Doc<ProfileProgram>, tol: Tol) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), tol)
}

/// Builds the die document, materializing each blend's edge set
/// against the evaluation of the recipe SO FAR — evaluate, select,
/// store, which is the whole authoring loop the document layer has.
///
/// The selections are frozen at authoring time on purpose: a stored
/// "every plane–sphere rim" that re-ran on every edit would silently
/// grow under an upstream change, which is the staleness the freeze
/// exists to prevent (`pncad::select` module docs).
fn build(tol: Tol) -> Die {
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("die", tol);
    let insert = |doc: &mut Doc<ProfileProgram>, node| -> RecipeNodeId {
        let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the edit applies");
        *doc = applied.doc;
        applied.record.minted.expect("insert mints an id")
    };
    // Every geometric selection in this file runs over EDGES.
    let edges = Selector::of(NamePat::of_kind(EntityKind::Edge));

    // ---- the sharp cube, [0, L]³ ----
    let cube_p = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops: vec![LoopProgram::polygon([(0.0, 0.0), (L, 0.0), (L, L), (0.0, L)]).unwrap()],
        }),
    );
    let cube = insert(
        &mut doc,
        Node::Extrude {
            profile: cube_p,
            distance: len(L),
        },
    );

    // ---- the blank: EVERY edge of the cube, at one radius ----
    // No predicate needed — "all of them" has its own door.
    let all_twelve = all_edges(&eval(&doc, tol), cube);
    let blank = insert(&mut doc, Node::fillet(cube, len(R), all_twelve));

    // ---- the master ball, poled along +Z ----
    let axis = insert(
        &mut doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let ball_p = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            // u = +X, v = +Z: the sketch's revolve axis lands on the
            // world +Z axis, which is the pole `placements` rotates.
            plane: SketchPlane::from_frame(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            loops: vec![half_disc()],
        }),
    );
    let ball = insert(
        &mut doc,
        Node::Revolve {
            profile: ball_p,
            axis,
            angle: ang(TAU),
        },
    );

    // ---- 21 placements, unioned into ONE cutting tool ----
    // The group discipline is load-bearing, not a flourish: cutting
    // the pips one at a time would present a body already carrying a
    // TRIMMED sphere face as the next operand, which S13 refuses (the
    // extent certificate needs the closed group).
    //
    // NAMED GAP (2026-08-14): the recipe layer's only way to assemble
    // a multi-shell body is a PAIRWISE `Node::Boolean(Union)`, so a
    // 21-shell tool costs twenty union nodes and leaves the last
    // ball's names twenty `FromA`/`FromB` segments deep. A group-union
    // node (or a union that takes a list) would say this in one.
    // Recorded in `docs/M8-LOG.md` beside the meridian gap.
    let mut tool: Option<RecipeNodeId> = None;
    for p in placements() {
        let pip = insert(
            &mut doc,
            Node::Transform {
                input: ball,
                translation: p.centre.map(len),
                rotation_axis: p.axis.map(scl),
                rotation_angle: ang(p.angle),
            },
        );
        tool = Some(match tool {
            None => pip,
            Some(acc) => insert(
                &mut doc,
                Node::Boolean {
                    op: BooleanOp::Union,
                    a: acc,
                    b: pip,
                    declare: None,
                },
            ),
        });
    }
    let pipped = insert(
        &mut doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: cube,
            b: tool.expect("21 pips"),
            declare: None,
        },
    );

    // ---- the surgery, both blends selected GEOMETRICALLY ----
    //
    // P10's alphabet, said in the ratified vocabulary as `select_where`
    // CALLS (SELECT-DESIGN §§1-2). Both atoms are EXACT — they read the
    // carrier's enum tag — so they are total: no funnel, no margin, and
    // `expect` here is a statement about the atoms, not optimism.
    let params = doc.param_env::<f64>();

    // The twelve box edges: the only LINES in the pipped cube. Every
    // pip cavity contributes circles (two rim arcs, two meridian
    // seams), so the carrier tag alone separates them.
    let box_edges = select_where(
        &eval(&doc, tol),
        pipped,
        &edges,
        &[GeomPred::CurveKind(CurveKindSet::just(CurveKind::Line))],
        &params,
        tol,
    )
    .expect("EXACT atoms are total");
    let blanked = insert(&mut doc, Node::fillet(pipped, len(R), box_edges.clone()));

    // The 21 pip rims: plane against sphere, UNORDERED across the edge
    // (the atom takes two SETS, not a left and a right). What this
    // EXCLUDES is the point — the two cavity meridians of each pip read
    // `(Sphere, Sphere)` and match neither set, and `fillet_edges`
    // would refuse them `TangentialEdge` at a margin of exactly zero,
    // correctly, at any radius: two half-cap faces sharing ONE sphere
    // surface have no dihedral wedge to roll a ball into. The
    // exclusion is by DESCRIPTION, not by a hand-maintained omission.
    let rims = select_where(
        &eval(&doc, tol),
        blanked,
        &edges,
        &[GeomPred::AdjacentKinds(
            SurfaceKindSet::just(SurfaceKind::Plane),
            SurfaceKindSet::just(SurfaceKind::Sphere),
        )],
        &params,
        tol,
    )
    .expect("EXACT atoms are total");
    let composed = insert(&mut doc, Node::fillet(blanked, len(RIM_R), rims.clone()));

    Die {
        doc,
        blank,
        pipped,
        composed,
        box_edges: box_edges.len(),
        rims: rims.len(),
    }
}

/// The recipe's bodies at any tour scalar — the scene's geometry is
/// the EVALUATION now, so this is where the generic lives (the
/// `heatsink` precedent).
fn body_at<S: Scalar>(ev: &Evaluation<S>, id: RecipeNodeId) -> Body<S> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => (**body).clone(),
        other => panic!("expected a body, got {other:?}"),
    }
}

/// The blank's closed-form volume: core + 6 slabs + 12
/// quarter-cylinders + 8 octants (which sum to one whole ball).
fn blank_volume() -> f64 {
    let core = L - 2.0 * R;
    core.powi(3)
        + 6.0 * R * core.powi(2)
        + 12.0 * (PI * R * R / 4.0) * core
        + (4.0 / 3.0) * PI * R.powi(3)
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let die = build(tol);
    let ev = evaluate::<f64>(
        &die.doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );

    // The two geometric selections ARE the composed stop's claim, so
    // they are pinned as counts: twelve lines, and 21 rims × two arcs
    // (a full revolve's band is two half-faces split at the seam
    // meridians, so each rim is two arcs rather than one circle).
    assert_eq!(
        die.box_edges, 12,
        "CurveKind(Line) is exactly the box edges"
    );
    assert_eq!(
        die.rims, 42,
        "AdjacentKinds(Plane, Sphere) is exactly the 21 pip rims, two arcs each"
    );

    let blank = body_at(&ev, die.blank);
    let vol = pncad::topo::mass_properties(&blank, tol).unwrap().volume;
    let want = blank_volume();
    assert!(
        (vol - want).abs() < 1e-9 * want,
        "the blank's volume is a closed form: {vol} vs {want}"
    );
    let (f, e, v) = (
        blank.faces().count(),
        blank.edges().count(),
        blank.vertices().count(),
    );
    assert_eq!((f, e, v), (26, 48, 24));
    let pipped = body_at(&ev, die.pipped);
    let pip_vol = pncad::topo::mass_properties(&pipped, tol).unwrap().volume;
    let composed = body_at(&ev, die.composed);
    let comp = pncad::topo::mass_properties(&composed, tol).unwrap();
    assert_eq!(
        pncad::topo::validate_geometric(&composed, tol),
        Ok(()),
        "the composed die is tier-3 valid"
    );
    let (cf, ce, cv) = (
        composed.faces().count(),
        composed.edges().count(),
        composed.vertices().count(),
    );
    assert_eq!((cf, ce, cv), (26 + 21 * 3, 48 + 21 * 7, 24 + 21 * 5));

    vec![
        Stop {
            name: "diefillet",
            caption: "the die blank (rolling-ball fillets)".to_string(),
            // Standalone since the montage-v2 curation (Evan, #218
            // follow-up): the two PARTIAL dice — the blank and the
            // pipped cube — are interesting for how they work (the
            // battery, the closed-group cut), but without that context
            // they read as near-duplicates of the composed die, which
            // remains the sheet's die. Scenes and narration stay alive.
            montage: false,
            story: "every edge of a cube blended at one radius — twelve quarter-cylinders \
                    and eight sphere-octant corners",
            ops: "Node::Fillet(cube, all_edges, r = 0.12): battery first, then \
                  plane–plane → cylinder blends with sphere-octant corner patches",
            delta: 5e-3,
            note: Some(format!(
                "the validity battery runs over the INPUTS before a surface exists — six \
                 named Q1 trileans (radius headroom, face consumption, spine regularity, \
                 chain G1, convexity sign, corner configuration); the result is {f} faces, \
                 {e} edges, {v} vertices, every trimline stored TangentIntersection from \
                 birth (the corner ones on CIRCLE carriers, a jet-certificate class this \
                 unit retired into the lane by equivariance), and V = {vol:.6} m³ on the \
                 closed form"
            )),
            view: View {
                elev: 26.0,
                azim: -50.0,
                up: 'z',
            },
            bodies: vec![SceneBody::plain("diefillet", [0.80, 0.72, 0.55], blank)],
        },
        Stop {
            name: "diepips",
            caption: "the die's pips (one group cut)".to_string(),
            // Standalone with `diefillet` (see the note there).
            montage: false,
            story: "21 spherical dimples on six faces, subtracted as a single 21-shell \
                    operand",
            ops: "cube ∖ (21 disjoint balls): S13's closed-group extent arm, each ball \
                  charted with its pole along the face it is cut by",
            delta: 5e-3,
            note: Some(format!(
                "cutting the pips one at a time would present a TRIMMED sphere face as the \
                 next operand, which S13 refuses typed (the extent certificate needs the \
                 closed-group discipline); charting a ball with a tilted pole would make the \
                 plane×sphere section non-polar, which the split-join refuses typed. Doing \
                 both right makes it one certified operation: V = {pip_vol:.6} m³. At M5 \
                 this and the blank were the die's two halves; the next stop is M6's \
                 composition surgery joining them"
            )),
            view: View {
                elev: 26.0,
                azim: -50.0,
                up: 'z',
            },
            bodies: vec![SceneBody::plain("diepips", [0.62, 0.66, 0.78], pipped)],
        },
        Stop {
            name: "diecomposed",
            caption: "THE COMPOSED DIE (in-place surgery)".to_string(),
            montage: true,
            story: "the filleted blank, the 21 pips and the filleted pip rims in ONE body \
                    — M6's in-place edge-blend composition surgery, both blends selected \
                    by GEOMETRY",
            ops: "cube ∖ pips, then Node::Fillet twice IN PLACE: select_where(CurveKind = \
                  Line) → the twelve box edges (faces split along stored trimlines, rings \
                  carried through, octant corners grafted), then \
                  select_where(AdjacentKinds = {Plane} × {Sphere}) → all 21 rims as closed \
                  chains → torus bands (donut-style slit-seamed annuli)",
            delta: 5e-3,
            note: Some(format!(
                "M5 exited with the die as two bodies pinned at two frontiers; both are \
                 retired — door B by the surgery, door A by the circle-carrier \
                 definite-miss rider. The composed body: {cf} faces, {ce} edges, {cv} \
                 vertices, tier-3 certified, V = {:.6} m³ on the closed form \
                 (Steiner blank − 21·(cap + rim-torus term)) at zero enclosure pad. \
                 Neither blend names an edge by hand: {} lines and {} plane–sphere arcs \
                 come back from `select_where`, and the 42 co-surface meridians it does \
                 NOT return are the ones `fillet_edges` would refuse at margin zero",
                comp.volume, die.box_edges, die.rims
            )),
            view: View {
                elev: 26.0,
                azim: -50.0,
                up: 'z',
            },
            bodies: vec![SceneBody::plain(
                "diecomposed",
                [0.85, 0.63, 0.46],
                composed,
            )],
        },
    ]
}
