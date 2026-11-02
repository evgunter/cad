//! **`Gap` reads a vertex or a line edge against a sphere or cylinder
//! face** (`topo::Round`, the carrier's side as the face's material).
//!
//! The bodies are posed off the axes: a pin and a plate's bore turned
//! an eighth of a turn about a skew axis and moved off the origin, and a
//! ball centred off it. A cube's vertical edge crosses each at a slant,
//! so an edge's least lies inside it (a pin) or at an end (a bore), and
//! no row is the axis-parallel ruling the old arm read. Each value is
//! held to a dense walk of the written-down distance, and the four
//! analysis lanes are run over the pin's and the bore's toleranced
//! radius: the Interval build encloses the f64 value, the Dual pass
//! reads `∂g/∂r = ∓1`, the box drive certifies a bound, and every Monte
//! Carlo draw measures.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::{in_world, insert, len, step, vname, vpiece};

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use editor_core::mc::{McConfig, monte_carlo};
use editor_core::stackup::{SensitivityOutcome, sensitivities};
use editor_core::{
    AssertionRelation, CapEnd, Dimension, Distribution, DocEdit, EntityKind, Formula, Frame,
    FreeVar, GeomPred, LoopProgram, MeasurePrimitive, NamePat, Node, NodeResult, ProfileDoc,
    ProfileProgram, RecipeNodeId, RoleSeg, Selector, SitedRef, SurfaceKindSet, UnitSym,
    ValuePayload, VarDecl, VarName, select_where,
};
use geom_core::{Bounds, Interval, Tol, Turns};

const R: f64 = 0.5;
/// The radius's tolerance, either way.
const H: f64 = 1e-4;

type V = [f64; 3];

fn sub(a: V, b: V) -> V {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn dot(a: V, b: V) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn add(a: V, b: V, s: f64) -> V {
    [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s]
}
fn norm(a: V) -> f64 {
    dot(a, a).sqrt()
}

/// The turn every round body is posed by, about a skew unit axis.
fn tilt() -> (V, Turns) {
    let a = [1.0, 0.3, 0.2];
    let n = norm(a);
    (
        [a[0] / n, a[1] / n, a[2] / n],
        Turns::new(1, 8).expect("an eighth"),
    )
}

/// `w` turned by [`tilt`] (right-handed, as a frame turns it).
fn turned(w: V) -> V {
    let (a, _) = tilt();
    let (s, c) = std::f64::consts::FRAC_PI_4.sin_cos();
    let x = [
        a[1] * w[2] - a[2] * w[1],
        a[2] * w[0] - a[0] * w[2],
        a[0] * w[1] - a[1] * w[0],
    ];
    let k = dot(a, w) * (1.0 - c);
    [
        w[0] * c + x[0] * s + a[0] * k,
        w[1] * c + x[1] * s + a[1] * k,
        w[2] * c + x[2] * s + a[2] * k,
    ]
}

const AT: V = [0.2, -0.1, 0.4];

fn posed() -> Frame {
    let (a, t) = tilt();
    Frame::rotate_by_turns_then_translate(a, t, AT, crate::fixture::band()).expect("a unit axis")
}

fn poly(pts: &[(f64, f64)]) -> LoopProgram<Formula> {
    LoopProgram::polygon(pts.to_vec()).expect("finite corners")
}

fn extrude(
    doc: ProfileDoc,
    loops: Vec<LoopProgram<Formula>>,
    h: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops,
            ids: Vec::new(),
            on: None,
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(h),
        },
    )
}

/// The first face of `placed` on a carrier of `kind`.
fn face_of(doc: &ProfileDoc, placed: RecipeNodeId, kind: geom::SurfaceKind) -> SitedRef {
    let run = corpus::eval::<f64>(doc);
    let mut faces = select_where(
        &run,
        placed,
        &Selector::of(NamePat::of_kind(EntityKind::Face)),
        &[GeomPred::SurfaceKind(SurfaceKindSet::just(kind))],
        &ProfileDoc::empty_derived("np", Tol::witness()).var_env::<f64>(),
        Tol::witness(),
    )
    .expect("exact");
    faces.sort();
    SitedRef::new(placed, faces.remove(0))
}

/// The unit cube's lateral edge at its profile vertex 0, which runs up
/// `z` from `corner`, and the vertex at its foot.
fn cube_at(doc: ProfileDoc, corner: V) -> (ProfileDoc, SitedRef, SitedRef) {
    let (doc, b) = extrude(
        doc,
        vec![poly(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])],
        1.0,
    );
    let (doc, pb) = in_world(doc, b, &Frame::translation(corner));
    let c0 = vpiece(&doc, b, 0, 0);
    let edge = SitedRef::new(pb, crate::fixture::ename(b, RoleSeg::LateralEdge(c0)));
    let foot = SitedRef::new(pb, vname(b, RoleSeg::CapVertex(CapEnd::Start, c0)));
    (doc, edge, foot)
}

/// A gap measure: the document after it and the measure.
fn gap_node(doc: &ProfileDoc, outer: &SitedRef, inner: &SitedRef) -> (ProfileDoc, RecipeNodeId) {
    crate::fixture::measure_node(
        doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        vec![outer.clone(), inner.clone()],
    )
}

fn value<T: editor_core::EvalScalar>(doc: &ProfileDoc, m: RecipeNodeId) -> Result<T, String> {
    let run = corpus::eval::<T>(doc);
    match run.nodes.get(&m) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => Ok(*value),
            other => panic!("a measure, not {}", other.kind_name()),
        },
        other => Err(format!("{other:?}")),
    }
}

/// The least over a dense walk of the edge from `x0` up `z` by one of
/// `f`, and its value at `x0`.
fn walk(x0: V, f: impl Fn(V) -> f64) -> f64 {
    (0..=20_000)
        .map(|i| f(add(x0, [0.0, 0.0, 1.0], f64::from(i) / 20_000.0)))
        .fold(f64::INFINITY, f64::min)
}

/// The pin's or the bore's wall: the signed distance from its turned
/// axis, positive away from its material.
fn wall(bore: bool, r: f64) -> impl Fn(V) -> f64 {
    let a = turned([0.0, 0.0, 1.0]);
    move |q: V| {
        let rel = sub(q, AT);
        let d = norm(add(rel, a, -dot(rel, a))) - r;
        if bore { -d } else { d }
    }
}

/// **A pin and a bore, posed off the axes, the radius `r` toleranced**,
/// and the cube's edge crossing the wall at a slant: the document, the
/// wall, the edge and its foot vertex.
fn round_wall(bore: bool, corner: V) -> (ProfileDoc, SitedRef, SitedRef, SitedRef) {
    let doc = ProfileDoc::empty_derived("gap-round-wall", Tol::witness());
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("r"),
            def: VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Length,
                value: R,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(Distribution::Uniform { lo: -H, hi: H }),
            }),
        },
    );
    let circle = LoopProgram::Circle {
        centre: [len(0.0), len(0.0)],
        radius: Formula::named(VarName::from_static("r"), Dimension::Length),
    };
    let loops = if bore {
        vec![
            poly(&[(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)]),
            circle,
        ]
    } else {
        vec![circle]
    };
    let (doc, c) = extrude(doc, loops, 2.0);
    let (doc, pc) = in_world(doc, c, &posed());
    let wall = face_of(&doc, pc, geom::SurfaceKind::Cylinder);
    let (doc, edge, foot) = cube_at(doc, corner);
    (doc, wall, edge, foot)
}

/// Where the cube's edge stands for each wall: across the pin, passing
/// within the radius of its axis, and across the bore's hollow with its
/// ends in the plate.
fn corner(bore: bool) -> V {
    let mid = add(AT, turned([0.0, 0.0, 1.0]), 1.0);
    if bore {
        [mid[0] + 0.2, mid[1] - 0.1, mid[2] - 0.5]
    } else {
        [mid[0] + 0.3, mid[1], mid[2] - 0.5]
    }
}

/// **A tilted pin and bore read a slanted edge and its foot vertex at
/// the walk's least, in either seat, and the Interval build encloses
/// each.** Red if the edge reads at an end of a pin (the held least
/// missing), at the least of a bore (a sense flipped), refuses as no
/// ruling, or if the enclosure misses.
#[test]
fn a_tilted_pin_and_bore_read_a_slanted_edge_and_a_vertex() {
    for bore in [false, true] {
        let (doc, wall_ref, edge, foot) = round_wall(bore, corner(bore));
        let f = wall(bore, R);
        let x0 = corner(bore);
        for (cell, want, what) in [(&edge, walk(x0, &f), "edge"), (&foot, f(x0), "vertex")] {
            for (o, i, seat) in [
                (&wall_ref, cell, "wall outer"),
                (cell, &wall_ref, "cell outer"),
            ] {
                let (doc, m) = gap_node(&doc, o, i);
                let got = value::<f64>(&doc, m)
                    .unwrap_or_else(|e| panic!("bore {bore} {what} {seat}: {e}"));
                assert!(
                    (got - want).abs() < 1e-7 && got <= want + 1e-12,
                    "bore {bore} {what} {seat}: reads {got}, the walk {want}"
                );
                let iv =
                    value::<Interval>(&doc, m).unwrap_or_else(|e| panic!("interval refused: {e}"));
                assert!(
                    iv.lo() <= got && got <= iv.hi(),
                    "bore {bore} {what} {seat}: [{}, {}] misses {got}",
                    iv.lo(),
                    iv.hi()
                );
            }
        }
        // The pin's least lies inside the edge, the bore's at an end.
        let inside = walk(x0, &f) < f(x0).min(f(add(x0, [0.0, 0.0, 1.0], 1.0))) - 1e-3;
        assert_eq!(inside, !bore, "bore {bore}: where the least lies");
    }
}

/// **The Dual pass, the box drive and the Monte Carlo lane read the
/// edge's gap over the toleranced radius.** The least is `d − r` on a
/// pin and `r − d` at a bore's end, so `∂g/∂r` is `−1` and `+1`; the
/// box drive certifies the gap within `2H` of its nominal over the
/// radius's box; every draw measures, inside `[g(R − H), g(R + H)]`.
/// Red if a lane refuses or reads outside.
#[test]
fn the_analysis_lanes_read_a_round_gap_over_its_radius() {
    for bore in [false, true] {
        let (doc, wall_ref, edge, _) = round_wall(bore, corner(bore));
        let (doc, m) = gap_node(&doc, &wall_ref, &edge);
        let nominal = value::<f64>(&doc, m).expect("f64 reads");
        let var = doc.output(m, 0).expect("a measure defines its value");
        let entries = sensitivities(&doc, var, None, None, false, None, Tol::witness())
            .unwrap_or_else(|e| panic!("bore {bore}: the Dual pass refused {e:?}"));
        assert_eq!(entries.len(), 1, "bore {bore}: one toleranced input");
        match entries[0].outcome {
            SensitivityOutcome::Derivative { value, .. } => {
                let want = if bore { 1.0 } else { -1.0 };
                assert!(
                    (value - want).abs() < 1e-9,
                    "bore {bore}: ∂g/∂r reads {value}"
                );
            }
            ref other => panic!("bore {bore}: {other:?}"),
        }
        // The box drive, over `|g − nominal| ≤ 2H`.
        let read = crate::fixture::value_of(&doc, m);
        let (doc, _) = insert(
            doc,
            Node::Assertion {
                value: read.clone(),
                bound: len(nominal + 2.0 * H),
                relation: AssertionRelation::AtMost,
            },
        );
        let (doc, _) = insert(
            doc,
            Node::Assertion {
                value: read,
                bound: len(nominal - 2.0 * H),
                relation: AssertionRelation::AtLeast,
            },
        );
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let verdict = drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: 16,
                ..DriveConfig::default()
            },
            Tol::witness(),
        )
        .expect("the drive runs");
        let receipt = verdict.receipt();
        assert!(
            receipt.certified > 0 && receipt.refused == 0,
            "bore {bore}: {receipt:?} {:?}",
            verdict.refused().first().map(|l| &l.reason)
        );
        let report = monte_carlo(
            &doc,
            &analyzed,
            &McConfig {
                samples: 32,
                parallel: false,
                ..McConfig::default()
            },
            Tol::witness(),
        )
        .expect("the nominal builds");
        let row = report
            .measures
            .iter()
            .find(|r| r.node == m)
            .expect("the gap's row");
        let lo = walk(corner(bore), wall(bore, R + H)).min(walk(corner(bore), wall(bore, R - H)));
        let hi = walk(corner(bore), wall(bore, R + H)).max(walk(corner(bore), wall(bore, R - H)));
        assert!(
            row.measured == 32
                && row.unmeasured == 0
                && row.min >= lo - 1e-7
                && row.max <= hi + 1e-7,
            "bore {bore}: {} measured, [{}, {}] against [{lo}, {hi}]",
            row.measured,
            row.min,
            row.max
        );
    }
}

/// **A ball off the origin reads an edge resting on it, lifted and
/// pressed.** The ball (radius 0.6) is turned and moved off the
/// origin; the cube's edge stands upright beside it at `0.6 + lift`
/// from its centre, so its least is at the middle and reads `lift`, in
/// either seat, and the Interval build encloses it. Red if the edge
/// reads at an end (`√(0.36 + 0.25) − 0.6`, far above), or a seat flips.
#[test]
fn an_off_centre_ball_reads_an_edge_resting_on_it() {
    let r = 0.6;
    let centre = AT;
    for lift in [0.0, 0.01, -0.01] {
        let doc = ProfileDoc::empty_derived("gap-round-ball", Tol::witness());
        let (doc, b) = crate::emit_sphere_cut_in_cap::ball(doc, r, 0.0);
        let (doc, pb) = in_world(doc, editor_core::Operand::output(b, 0), &posed());
        let ball = face_of(&doc, pb, geom::SurfaceKind::Sphere);
        let (doc, edge, _) = cube_at(doc, [centre[0] + r + lift, centre[1], centre[2] - 0.5]);
        for (o, i, seat) in [(&ball, &edge, "ball outer"), (&edge, &ball, "edge outer")] {
            let (doc, m) = gap_node(&doc, o, i);
            let got = value::<f64>(&doc, m).unwrap_or_else(|e| panic!("lift {lift} {seat}: {e}"));
            assert!((got - lift).abs() < 1e-9, "lift {lift} {seat}: reads {got}");
            let iv = value::<Interval>(&doc, m).unwrap_or_else(|e| panic!("interval refused: {e}"));
            assert!(
                iv.lo() <= got && got <= iv.hi(),
                "lift {lift} {seat}: misses"
            );
        }
    }
}

/// **The Dual pass reads `∂g/∂r = −1` where a cell meets the pin's
/// axis** (the review's probe of #5827): a vertex on the axis, an edge
/// along it, and an edge crossing it (the pin turned a quarter turn
/// about `x`). The cell does not move, so the distance from the axis
/// has a derivative of zero even where it is zero, and only the radius
/// moves the gap. Red if a NaN tangent is minted there
/// (`TangentDegraded`), or the pass refuses.
#[test]
fn the_dual_pass_reads_the_radius_where_a_cell_meets_the_axis() {
    let base = |frame: Frame, corner: V| {
        let doc = ProfileDoc::empty_derived("gap-axis-dual", Tol::witness());
        let (doc, _) = step(
            doc,
            DocEdit::DeclareVar {
                name: VarName::from_static("r"),
                def: VarDecl::Free(FreeVar::Continuous {
                    dim: Dimension::Length,
                    value: R,
                    display_unit: UnitSym::canonical_for(Dimension::Length),
                    distribution: Some(Distribution::Uniform { lo: -H, hi: H }),
                }),
            },
        );
        let circle = LoopProgram::Circle {
            centre: [len(0.0), len(0.0)],
            radius: Formula::named(VarName::from_static("r"), Dimension::Length),
        };
        let (doc, c) = extrude(doc, vec![circle], 2.0);
        let (doc, pc) = in_world(doc, c, &frame);
        let wall = face_of(&doc, pc, geom::SurfaceKind::Cylinder);
        let (doc, edge, foot) = cube_at(doc, corner);
        (doc, wall, edge, foot)
    };
    let quarter = Frame::rotate_by_turns_then_translate(
        [1.0, 0.0, 0.0],
        Turns::new(1, 4).expect("a quarter"),
        [0.0, 0.0, 0.0],
        crate::fixture::band(),
    )
    .expect("a unit axis");
    let still = Frame::translation([0.0, 0.0, 0.0]);
    for (name, frame, corner, use_edge) in [
        ("vertex on the axis", still, [0.0, 0.0, 0.5], false),
        ("edge along the axis", still, [0.0, 0.0, 0.5], true),
        ("edge across the axis", quarter, [0.0, -1.0, -0.5], true),
    ] {
        let (doc, wall, edge, foot) = base(frame, corner);
        let cell = if use_edge { &edge } else { &foot };
        let (doc, m) = gap_node(&doc, &wall, cell);
        let var = doc.output(m, 0).expect("a measure defines its value");
        let entries = sensitivities(&doc, var, None, None, false, None, Tol::witness())
            .unwrap_or_else(|e| panic!("{name}: the Dual pass refused {e:?}"));
        match entries.first().map(|e| &e.outcome) {
            Some(SensitivityOutcome::Derivative { value, .. }) => {
                assert!((value + 1.0).abs() < 1e-9, "{name}: ∂g/∂r reads {value}");
            }
            other => panic!("{name}: {other:?}"),
        }
    }
}
