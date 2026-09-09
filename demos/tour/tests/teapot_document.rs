//! **The teapot's document, tabulated** — the class behind the scene's
//! sixth finding, and the guard its two-request roll owes.
//!
//! `demos/tour/src/teapot.rs` pins the finding LIVE, on the lid's own
//! three rims: the one-request `Node::Fillet` is attempted on every
//! pass and its `Naming(Duplicate)` asserted. A pin on one selection
//! says the door refused THAT selection. This file is the other half —
//! the claim the scene's prose makes about the CLASS, executed:
//!
//! 1. **The collision is not "any two ADJACENT rims".** A band slits
//!    ONE support's seam meridian, so two rims collide exactly when
//!    their bands slit the SAME one. On this lid that is the pair
//!    `{1, 2}` and nothing else among the rims that roll: `{1, 3}`,
//!    `{1, 4}`, `{2, 3}`, `{2, 4}` and `{3, 4}` all compose, and two
//!    of those — `{2, 3}` and `{3, 4}` — are ADJACENT. Adjacency is
//!    necessary and not sufficient, which is what the filed issue's
//!    first cut got wrong.
//! 2. **The two requests build what the kernel's one request builds.**
//!    The scene splits the roll because the document layer cannot NAME
//!    the one-request output, and the whole weight of that deviation
//!    rests on the two spellings being the same body. So they are
//!    compared: same census, the three bands' stored `(station, major,
//!    minor)` bit for bit, and the mass properties to a relative
//!    1e-14. What they do NOT share is the face ORDER, which is the
//!    only thing the conversion moved and is recorded here rather than
//!    left to the tess-budget diff to imply.
//!
//! The meridian and the constants are re-spelled here because a demo
//! binary's module cannot be imported by an integration test — the
//! same reason `verbs_teapot.rs` carries its own `genus`. They are
//! copied from `src/teapot.rs` and nothing here may be edited without
//! editing it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use pncad::document::{
    CancelToken, Datum, Dimension, Doc, DocEdit, EvalOptions, Evaluation, Expr, LoopProgram, Node,
    ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, ValuePayload, apply,
    evaluate,
};
use pncad::geom::Surface;
use pncad::geom_core::Tol;
use pncad::prelude::{StableName, fillet_edges, query};
use pncad::profile::ArcSweep;
use pncad::select::{band_rim, carried, edge_name};
use pncad::topo::{Body, EdgeKey};

// ---- the lid's stations, from `src/teapot.rs` ----
const R_NECK: f64 = 3.0 / 64.0;
const Y_MOUTH: f64 = 8.0 / 64.0;
const LIFT: f64 = 1.0 / 32.0;
const LID_BASE: f64 = Y_MOUTH + LIFT;
const R_FLANGE: f64 = 14.0 / 256.0;
const Y_FLANGE: f64 = LID_BASE + 6.0 / 256.0;
const DOME_C: f64 = LID_BASE + 1.0 / 256.0;
const R_KNOB: f64 = 5.0 / 256.0;
const Y_KNOB: f64 = LID_BASE + 13.0 / 256.0;
const Y_TOP: f64 = LID_BASE + 18.0 / 256.0;
const R_VENT: f64 = 1.0 / 256.0;
const ROLL: f64 = 2.0 / 256.0;

/// The lid's three rolled rims, as the meridian vertex each stands at.
const ROLLED: [u32; 3] = [1, 2, 4];

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a finite length")
}
fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("a finite scalar")
}
fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).expect("a finite angle")
}
fn lpt(x: f64, y: f64) -> [Expr; 2] {
    [len(x), len(y)]
}
fn line_to(x: f64, y: f64) -> ProgramStep {
    ProgramStep::LineTo(ProgramTarget::Point(lpt(x, y)))
}

fn lid_meridian() -> LoopProgram {
    LoopProgram::Chain(vec![
        ProgramStep::At(lpt(R_VENT, LID_BASE)),
        line_to(R_FLANGE, LID_BASE),
        line_to(R_NECK, Y_FLANGE),
        ProgramStep::ArcTo(ProgramArcData::Center {
            c: lpt(0.0, DOME_C),
            winding: ArcSweep::Ccw,
            target: ProgramTarget::Point(lpt(R_KNOB, Y_KNOB)),
        }),
        line_to(R_KNOB, Y_TOP),
        line_to(R_VENT, Y_TOP),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

fn insert(doc: &mut Doc<ProfileProgram>, node: Node<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the edit applies");
    *doc = applied.doc;
    applied.record.minted.expect("insert mints an id")
}

/// A fresh document carrying the sharp lid, and that node's id.
fn sharp_lid(tol: Tol) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("teapot-lid", tol);
    let plane = insert(
        &mut doc,
        Node::Datum(Datum::Frame {
            origin: [len(0.0), len(0.0), len(0.0)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }),
        tol,
    );
    let axis = insert(
        &mut doc,
        Node::Datum(Datum::AxisInPlane {
            plane,
            origin: [len(0.0), len(0.0)],
            direction: [scl(0.0), scl(1.0)],
        }),
        tol,
    );
    let profile = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![lid_meridian()],
        }),
        tol,
    );
    let lid = insert(
        &mut doc,
        Node::Revolve {
            profile,
            axis,
            angle: ang(TAU),
        },
        tol,
    );
    (doc, lid)
}

fn eval(doc: &Doc<ProfileProgram>, tol: Tol) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), tol)
}

fn body_at(ev: &Evaluation<f64>, id: RecipeNodeId) -> Body<f64> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => (**b).clone(),
        other => panic!("expected a body, got {other:?}"),
    }
}

fn census(b: &Body<f64>) -> (usize, usize, usize) {
    (b.vertices().count(), b.edges().count(), b.faces().count())
}

/// Every torus band's stored `(station, major, minor)`, as BITS —
/// which is what "the same body" has to mean for a stored radius.
fn bands(b: &Body<f64>) -> Vec<(u64, u64, u64)> {
    let mut out: Vec<(u64, u64, u64)> = b
        .faces()
        .filter_map(|(_, f)| match b.get_surface(f.surface) {
            Some(Surface::Torus {
                major_radius,
                minor_radius,
                center,
                ..
            }) => Some((
                center.y.to_bits(),
                major_radius.to_bits(),
                minor_radius.to_bits(),
            )),
            _ => None,
        })
        .collect();
    out.sort_unstable();
    out
}

/// One `Node::Fillet` request over the rims at `vs`: the census it
/// built, or the refusal it answered.
fn roll_once(vs: &[u32], tol: Tol) -> Result<(usize, usize, usize), String> {
    let (mut doc, lid) = sharp_lid(tol);
    let sel: Vec<StableName> = vs.iter().map(|&v| band_rim(lid, v)).collect();
    let rolled = insert(&mut doc, Node::fillet(lid, len(ROLL), sel), tol);
    let ev = eval(&doc, tol);
    match ev.node_error(rolled) {
        Some(e) => Err(format!("{:?}", e.kind)),
        None => Ok(census(&body_at(&ev, rolled))),
    }
}

/// **Which rim pairs one request can name, and which it cannot.**
///
/// The filed issue's first cut said any two ADJACENT latitude rims
/// collide. They do not: a band slits ONE support's seam, so the test
/// is whether two bands slit the SAME meridian. Two of the five pairs
/// below that COMPOSE are adjacent.
#[test]
fn the_slit_collision_is_per_meridian_and_not_per_adjacency() {
    let tol = Tol::witness();
    // The scene's own three rims, in one request: the refusal the
    // scene's two-request grain exists for.
    let all = roll_once(&ROLLED, tol).expect_err("all three in one request must refuse");
    assert!(
        all.starts_with("Naming(Duplicate")
            && all.contains("BandSlit")
            && all.contains("Meridian(Seam, ProfileEdgeRef { loop_index: 0, segment: 1 })"),
        "the three-rim request refuses at the SLIT's name, on the flange cone's own \
         seam meridian (segment 1): {all}"
    );
    // The pair that collides, and it is the only one among the rolled
    // rims: the flange's rim and the dome's foot are the two ends of
    // segment 1, so both bands slit its seam.
    let pair = roll_once(&[1, 2], tol).expect_err("the flange rim and the dome foot collide");
    assert!(
        pair.starts_with("Naming(Duplicate") && pair.contains("segment: 1"),
        "{pair}"
    );
    // Every other pair over the rims the scene rolls composes, ADJACENT
    // or not — which is the half the issue's first cut denied. `{2, 3}`
    // and `{3, 4}` are adjacent and build.
    for pair in [[1u32, 3], [1, 4], [2, 3], [2, 4], [3, 4]] {
        assert_eq!(
            roll_once(&pair, tol),
            Ok((8, 16, 8)),
            "rims {pair:?} slit two different meridians, so ONE request names their \
             output: two annulus bands over the sharp lid's 6/12/6"
        );
    }
}

/// **The two requests build the body the kernel's one request builds.**
///
/// The scene splits the roll because the document layer cannot name
/// the one-request output, and the deviation is only honest if the
/// geometry is untouched by it. Same census, the same three bands bit
/// for bit, the same mass. The face ORDER differs, and that difference
/// is the whole of what the conversion moved in the tess-budget rows.
#[test]
fn two_requests_build_the_kernels_one_request_body() {
    let tol = Tol::witness();
    let (mut doc, lid) = sharp_lid(tol);
    let first = insert(
        &mut doc,
        Node::fillet(lid, len(ROLL), vec![band_rim(lid, ROLLED[0])]),
        tol,
    );
    let second = insert(
        &mut doc,
        Node::fillet(
            first,
            len(ROLL),
            vec![
                carried(first, band_rim(lid, ROLLED[1])),
                carried(first, band_rim(lid, ROLLED[2])),
            ],
        ),
        tol,
    );
    let ev = eval(&doc, tol);
    assert!(
        ev.node_error(second).is_none(),
        "the two-request roll builds: {:?}",
        ev.node_error(second)
    );
    let sharp = body_at(&ev, lid);
    let two = body_at(&ev, second);

    // The kernel's ONE request over the same three rims — their keys
    // found through the NAMES, so the two spellings are asked for the
    // same edges and not merely for three edges each.
    let keys: Vec<EdgeKey> = ROLLED
        .iter()
        .map(|&v| {
            query::all_edges(&sharp)
                .into_iter()
                .find(|&k| edge_name(&ev, lid, 0, k).ok() == Some(&band_rim(lid, v)))
                .expect("each rolled rim's key, by its name")
        })
        .collect();
    let one = fillet_edges(&sharp, &keys, ROLL, tol)
        .expect("the kernel door rolls all three in one request")
        .body;

    assert_eq!(census(&one), (9, 18, 9));
    assert_eq!(census(&two), (9, 18, 9));
    assert_eq!(
        bands(&one),
        bands(&two),
        "the three bands' STORED (station, major, minor) must be identical bits — a \
         difference here is the two-request grain changing the part"
    );
    let p1 = pncad::topo::mass_properties(&one, tol).expect("one-request props");
    let p2 = pncad::topo::mass_properties(&two, tol).expect("two-request props");
    assert!(
        ((p1.volume - p2.volume) / p1.volume).abs() < 1e-14
            && ((p1.surface_area - p2.surface_area) / p1.surface_area).abs() < 1e-14,
        "one request V={} A={}; two requests V={} A={}",
        p1.volume,
        p1.surface_area,
        p2.volume,
        p2.surface_area
    );

    // What DOES differ, recorded rather than implied: the order the
    // faces come out in. This is the whole of the tess-budget move the
    // conversion carries — three `teapotlid` rows permuting their
    // triangle counts among themselves.
    let order = |b: &Body<f64>| -> Vec<String> {
        b.faces()
            .map(|(_, f)| match b.get_surface(f.surface) {
                Some(Surface::Torus { center, .. }) => format!("torus@{}", center.y),
                Some(Surface::Plane { .. }) => "plane".to_owned(),
                Some(Surface::Cylinder { .. }) => "cylinder".to_owned(),
                Some(Surface::Cone { .. }) => "cone".to_owned(),
                Some(Surface::Sphere { .. }) => "sphere".to_owned(),
                _ => "?".to_owned(),
            })
            .collect()
    };
    let (a, b) = (order(&one), order(&two));
    println!("   one request: {a:?}\n   two requests: {b:?}");
    let mut sorted = (a.clone(), b.clone());
    sorted.0.sort();
    sorted.1.sort();
    assert_eq!(
        sorted.0, sorted.1,
        "the two spellings carry the same faces; only their order may differ"
    );
    assert_ne!(
        a, b,
        "the face ORDER is the same after all — then the tess-budget rows the conversion \
         moved should not have moved, and the baseline wants re-cutting back"
    );
}
