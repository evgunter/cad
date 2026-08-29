//! **M10-2 R1 probes** — an independent reviewer's consumer suite for
//! the measurement vocabulary (PR #1213, frozen head e0cc0b20).
//!
//! Every oracle below is derived from AUTHORED numbers (coordinates
//! and radii written into the fixtures), never from a previous run of
//! the code under test. Rows marked EVIDENCE-ONLY record observed
//! behavior for the review record and would be candidates to retire
//! per `memories/review-and-dependency-policy.md`.
//!
//! No fuzzing here — every row is a written-down witness (shape 2 of
//! `memories/test-suite-cost.md`), so no seeds and no effort dial.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "fixture/mod.rs"]
mod fixture;

use editor_core::{
    AssertionDir, AssertionVerdict, CancelToken, Dimension, DocEdit, DocParam, DocParamValue,
    DocumentId, EditError, EntityKind, EvalOptions, Evaluation, Expr, GeomPred, LoopProgram,
    MeasureExpr, MeasurePrimitive, NamePat, Node, NodeErrorKind, NodeResult, ParamName,
    PersistError, ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget,
    RecipeNodeId, Selector, SnapshotError, StableName, SurfaceKindSet, ValuePayload, apply,
    evaluate, face_frame, load, save, select_where, vertex_position,
};
use fixture::{ang, len, scl};
use geom_core::Tol;
use profile::SketchPlane;

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn push(doc: &ProfileDoc, edit: &DocEdit<ProfileProgram>) -> ProfileDoc {
    apply(doc, edit, Tol::witness())
        .unwrap_or_else(|e| panic!("edit refused: {e}"))
        .doc
}

fn insert(doc: &ProfileDoc, node: Node<ProfileProgram>) -> (ProfileDoc, RecipeNodeId) {
    let applied = apply(doc, &DocEdit::InsertNode { node }, Tol::witness())
        .unwrap_or_else(|e| panic!("insert refused: {e}"));
    (applied.doc, applied.record.minted.expect("insert mints"))
}

fn no_params() -> editor_core::ParamEnv<f64> {
    ProfileDoc::empty_derived("m10-2-r1-noparams", Tol::witness()).param_env::<f64>()
}

fn faces_of_kind(
    ev: &Evaluation<f64>,
    body: RecipeNodeId,
    kind: geom_brep::SurfaceKind,
) -> Vec<StableName> {
    let mut faces = select_where(
        ev,
        body,
        &Selector::of(NamePat::of_kind(EntityKind::Face)),
        &[GeomPred::SurfaceKind(SurfaceKindSet::just(kind))],
        &no_params(),
        Tol::witness(),
    )
    .expect("surface-kind selection is exact");
    faces.sort();
    faces
}

fn measured(ev: &Evaluation<f64>, id: RecipeNodeId) -> (f64, Dimension) {
    match ev.nodes.get(&id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, dim } => (*value, *dim),
            other => panic!("node {id:?} is a {}", other.kind_name()),
        },
        other => panic!("node {id:?} did not evaluate: {other:?}"),
    }
}

fn verdict(ev: &Evaluation<f64>, id: RecipeNodeId) -> AssertionVerdict<f64> {
    match ev.nodes.get(&id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Assertion(verdict) => verdict.clone(),
            other => panic!("node {id:?} is a {}", other.kind_name()),
        },
        other => panic!("node {id:?} did not evaluate: {other:?}"),
    }
}

fn failed_kind(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NodeErrorKind {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(e)) => &e.kind,
        other => panic!("node {id:?} was expected to fail, got {other:?}"),
    }
}

/// A `w × h` rectangular slab extruded by the `depth` PARAMETER
/// (0.35 by default — deliberately NOT the spec plate's numbers).
/// Caps at z = 0 and z = depth; sides at x = ±0.6, y = ±0.4.
const DEPTH: f64 = 0.35;

fn slab() -> (ProfileDoc, RecipeNodeId) {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-2-r1-slab"), Tol::witness());
    doc = push(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("depth"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: DEPTH,
                distribution: None,
            },
        },
    );
    let outer = LoopProgram::Chain(vec![
        ProgramStep::At([len(-0.6), len(-0.4)]),
        ProgramStep::LineTo(ProgramTarget::Point([len(0.6), len(-0.4)])),
        ProgramStep::LineTo(ProgramTarget::Point([len(0.6), len(0.4)])),
        ProgramStep::LineTo(ProgramTarget::Point([len(-0.6), len(0.4)])),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let (doc, profile) = insert(
        &doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops: vec![outer],
        }),
    );
    let (doc, slab) = insert(
        &doc,
        Node::Extrude {
            profile,
            distance: Expr::param(ParamName::new("depth"), Dimension::Length),
        },
    );
    (doc, slab)
}

/// The slab's two z-normal caps, identified through the PUBLIC frame
/// door: the faces whose carrier axis is ±ẑ. Returned bottom (z≈0)
/// first.
fn caps(ev: &Evaluation<f64>, slab: RecipeNodeId) -> [StableName; 2] {
    let mut z_faces: Vec<(f64, StableName)> = faces_of_kind(ev, slab, geom_brep::SurfaceKind::Plane)
        .into_iter()
        .filter_map(|name| {
            let pose = face_frame(ev, slab, &name).expect("a plane face has a frame");
            (pose.axis.z.abs() > 0.99).then(|| (pose.origin.z, name))
        })
        .collect();
    z_faces.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    assert_eq!(z_faces.len(), 2, "a slab has two z-normal caps");
    let mut it = z_faces.into_iter().map(|(_, n)| n);
    [it.next().unwrap(), it.next().unwrap()]
}

// ---- distance: the arms the PR's suite left without an oracle ----

/// `distance(plane, plane)` between the two caps IS the extrude depth
/// — the authored parameter value, exactly.
#[test]
fn r1_plane_plane_distance_is_the_extrude_depth() {
    let (doc, slab) = slab();
    let ev = eval(&doc);
    let [bottom, top] = caps(&ev, slab);
    let (doc, m) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![bottom, top],
        )
        .expect("indices in range"),
    );
    let (d, dim) = measured(&eval(&doc), m);
    assert_eq!(dim, Dimension::Length);
    assert!(
        (d - DEPTH).abs() < 1e-12,
        "cap separation is the authored depth {DEPTH}, got {d}"
    );
}

/// `distance(vertex, plane)`: every bottom vertex is at z = 0, so its
/// distance to the TOP cap is the authored depth.
#[test]
fn r1_vertex_plane_distance_is_the_depth() {
    let (doc, slab) = slab();
    let ev = eval(&doc);
    let [_, top] = caps(&ev, slab);
    let mut verts = select_where(
        &ev,
        slab,
        &Selector::of(NamePat::of_kind(EntityKind::Vertex)),
        &[],
        &no_params(),
        Tol::witness(),
    )
    .expect("vertex selection");
    verts.sort();
    let bottom_vert = verts
        .iter()
        .find(|v| {
            vertex_position(&ev, slab, v)
                .map(|p| p.z.abs() < 1e-12)
                .unwrap_or(false)
        })
        .expect("the slab has bottom vertices")
        .clone();
    let (doc, m) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![bottom_vert, top],
        )
        .expect("indices in range"),
    );
    let (d, _) = measured(&eval(&doc), m);
    assert!(
        (d - DEPTH).abs() < 1e-12,
        "a z=0 vertex is {DEPTH} from the top cap, got {d}"
    );
}

/// `distance(vertex, vertex)`: two vertices whose positions are read
/// through the PUBLIC door and whose separation is re-derived with
/// this suite's own arithmetic.
#[test]
fn r1_vertex_vertex_distance_matches_the_authored_corners() {
    let (doc, slab) = slab();
    let ev = eval(&doc);
    let mut verts = select_where(
        &ev,
        slab,
        &Selector::of(NamePat::of_kind(EntityKind::Vertex)),
        &[],
        &no_params(),
        Tol::witness(),
    )
    .expect("vertex selection");
    verts.sort();
    assert!(verts.len() >= 2, "a slab has corners");
    let (a, b) = (verts[0].clone(), verts[1].clone());
    let (pa, pb) = (
        vertex_position(&ev, slab, &a).expect("a vertex has a position"),
        vertex_position(&ev, slab, &b).expect("a vertex has a position"),
    );
    // The corners are authored at x∈{±0.6}, y∈{±0.4}, z∈{0, depth}:
    // check the read positions really are corners, then re-derive.
    for p in [pa, pb] {
        assert!((p.x.abs() - 0.6).abs() < 1e-12, "corner x, got {}", p.x);
        assert!((p.y.abs() - 0.4).abs() < 1e-12, "corner y, got {}", p.y);
        assert!(
            p.z.abs() < 1e-12 || (p.z - DEPTH).abs() < 1e-12,
            "corner z, got {}",
            p.z
        );
    }
    let expect =
        ((pb.x - pa.x).powi(2) + (pb.y - pa.y).powi(2) + (pb.z - pa.z).powi(2)).sqrt();
    let (doc, m) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![a, b],
        )
        .expect("indices in range"),
    );
    let (d, _) = measured(&eval(&doc), m);
    assert!(
        (d - expect).abs() < 1e-12,
        "vertex separation {expect}, got {d}"
    );
}

// ---- angle: real oracles (the PR's own row asserts only [0, π]) ----

/// The two caps of a prism face APART: the angle between their chart
/// normals is π exactly — and two perpendicular side walls give π/2.
#[test]
fn r1_plane_angles_have_the_authored_values() {
    let (doc, slab) = slab();
    let ev = eval(&doc);
    let [bottom, top] = caps(&ev, slab);
    let all_planes = faces_of_kind(&ev, slab, geom_brep::SurfaceKind::Plane);
    let x_wall = all_planes
        .iter()
        .find(|n| {
            face_frame(&ev, slab, n)
                .map(|p| p.axis.x.abs() > 0.99)
                .unwrap_or(false)
        })
        .expect("an x-normal wall")
        .clone();
    let y_wall = all_planes
        .iter()
        .find(|n| {
            face_frame(&ev, slab, n)
                .map(|p| p.axis.y.abs() > 0.99)
                .unwrap_or(false)
        })
        .expect("a y-normal wall")
        .clone();

    let (doc, opposed) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Angle { a: 0, b: 1 }),
            vec![bottom, top],
        )
        .expect("indices in range"),
    );
    let (doc, square) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Angle { a: 0, b: 1 }),
            vec![x_wall, y_wall],
        )
        .expect("indices in range"),
    );
    let ev = eval(&doc);
    let (a_pi, dim) = measured(&ev, opposed);
    assert_eq!(dim, Dimension::Angle);
    assert!(
        (a_pi - std::f64::consts::PI).abs() < 1e-9,
        "opposed caps subtend π, got {a_pi}"
    );
    let (a_half, _) = measured(&ev, square);
    assert!(
        (a_half - std::f64::consts::FRAC_PI_2).abs() < 1e-9,
        "perpendicular walls subtend π/2, got {a_half}"
    );
}

// ---- gap: the plane arm, and what its sign actually depends on ----

/// The plane gap is `(o_i − o_o)·n̂_o` — verified against the SAME
/// formula computed from the public frame door, and its sign is shown
/// to ride the OUTER face's chart-normal orientation (the fact a
/// consumer must know: C5's "g > 0 clearance" is meaningful for the
/// plane arm only relative to that normal).
#[test]
fn r1_plane_gap_matches_its_formula_and_rides_the_outer_chart_normal() {
    let (doc, slab) = slab();
    let ev = eval(&doc);
    let [bottom, top] = caps(&ev, slab);
    let (o_b, n_b) = {
        let p = face_frame(&ev, slab, &bottom).unwrap();
        (p.origin, p.axis)
    };
    let (o_t, n_t) = {
        let p = face_frame(&ev, slab, &top).unwrap();
        (p.origin, p.axis)
    };
    let dot = |o: geom_core::Point3<f64>, oo: geom_core::Point3<f64>, n: geom_core::Vec3<f64>| {
        (o.x - oo.x) * n.x + (o.y - oo.y) * n.y + (o.z - oo.z) * n.z
    };
    let expect_bt = dot(o_t, o_b, n_b); // gap(bottom, top)
    let expect_tb = dot(o_b, o_t, n_t); // gap(top, bottom)

    let (doc, g_bt) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
            vec![bottom.clone(), top.clone()],
        )
        .expect("indices in range"),
    );
    let (doc, g_tb) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
            vec![top, bottom],
        )
        .expect("indices in range"),
    );
    let ev = eval(&doc);
    let (bt, dim) = measured(&ev, g_bt);
    assert_eq!(dim, Dimension::Length);
    assert!((bt - expect_bt).abs() < 1e-12, "gap(bottom, top): formula {expect_bt}, got {bt}");
    let (tb, _) = measured(&ev, g_tb);
    assert!((tb - expect_tb).abs() < 1e-12, "gap(top, bottom): formula {expect_tb}, got {tb}");
    // The magnitude is the cap separation either way.
    assert!((bt.abs() - DEPTH).abs() < 1e-12);
    assert!((tb.abs() - DEPTH).abs() < 1e-12);
    // EVIDENCE-ONLY, recorded for the review: with ANTI-parallel chart
    // normals (a prism's caps), swapping the roles does NOT negate the
    // gap — g(a,b) = g(b,a). The sign convention for the plane arm is
    // a fact about chart orientation, not about containment roles.
    assert!(
        (bt - tb).abs() < 1e-12 || (bt + tb).abs() < 1e-12,
        "role swap either preserves or negates: {bt} vs {tb}"
    );
}

// ---- gap: concentric spheres (untested in the PR) ----

/// A ball of radius `r` centred `c` up the y-axis: the natural
/// meridian (bulge-1 semicircle) revolved 2π about y.
fn ball(doc: &ProfileDoc, r: f64, c: f64) -> (ProfileDoc, RecipeNodeId) {
    let p2 = |x: f64, y: f64| [len(x), len(y)];
    let meridian = LoopProgram::Chain(vec![
        ProgramStep::At(p2(0.0, c - r)),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(p2(0.0, c + r)),
            b: scl(1.0),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::from_frame(
                geom_core::Point3::new(0.0, 0.0, 0.0),
                geom_core::Vec3::new(1.0, 0.0, 0.0),
                geom_core::Vec3::new(0.0, 1.0, 0.0),
            ),
            loops: vec![meridian],
        }),
    );
    let (doc, axis) = insert(
        &doc,
        Node::Datum(editor_core::Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    insert(
        &doc,
        Node::Revolve {
            profile: p,
            axis,
            angle: ang(std::f64::consts::TAU),
        },
    )
}

/// `gap(socket, ball) = R − r − ‖Δc‖` in all three regimes, on real
/// revolved sphere faces: R = 1 socket at the origin, r = 0.25 ball
/// offset `c` up the axis — clearance at c = 0.3, contact at c = 0.75,
/// interference at c = 0.9.
#[test]
fn r1_sphere_gap_three_regimes_on_revolved_balls() {
    for (c, expect) in [(0.3, 0.45), (0.75, 0.0), (0.9, -0.15)] {
        let doc = ProfileDoc::empty(DocumentId::derive("m10-2-r1-balls"), Tol::witness());
        let (doc, socket) = ball(&doc, 1.0, 0.0);
        let (doc, pin) = ball(&doc, 0.25, c);
        let ev = eval(&doc);
        let socket_face = faces_of_kind(&ev, socket, geom_brep::SurfaceKind::Sphere)
            .first()
            .expect("the socket revolve mints a sphere face")
            .clone();
        let ball_face = faces_of_kind(&ev, pin, geom_brep::SurfaceKind::Sphere)
            .first()
            .expect("the ball revolve mints a sphere face")
            .clone();
        let (doc, m) = insert(
            &doc,
            Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
                vec![socket_face, ball_face],
            )
            .expect("indices in range"),
        );
        let (g, dim) = measured(&eval(&doc), m);
        assert_eq!(dim, Dimension::Length);
        assert!(
            (g - expect).abs() < 1e-9,
            "R − r − Δc = 1 − 0.25 − {c} = {expect}, got {g}"
        );
    }
}

// ---- gap: coaxial cylinders, three regimes on real geometry ----

/// Two circles extruded into cylinders: `bore_r` at the origin,
/// `pin_r` offset by `off` along x.
fn cylinders(bore_r: f64, pin_r: f64, off: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive("m10-2-r1-cyl"), Tol::witness());
    let circle = |cx: f64, r: f64| {
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops: vec![LoopProgram::Circle {
                centre: [len(cx), len(0.0)],
                radius: len(r),
            }],
        })
    };
    let (doc, p1) = insert(&doc, circle(0.0, bore_r));
    let (doc, bore) = insert(
        &doc,
        Node::Extrude {
            profile: p1,
            distance: len(0.1),
        },
    );
    let (doc, p2) = insert(&doc, circle(off, pin_r));
    let (doc, pin) = insert(
        &doc,
        Node::Extrude {
            profile: p2,
            distance: len(0.1),
        },
    );
    let _ = p2;
    (doc, bore, pin)
}

fn wall(ev: &Evaluation<f64>, node: RecipeNodeId) -> StableName {
    faces_of_kind(ev, node, geom_brep::SurfaceKind::Cylinder)
        .first()
        .expect("a circular extrude mints a cylinder wall")
        .clone()
}

/// `gap(bore, pin) = r_b − r_p − d` across clearance / contact /
/// interference, with the axis offset exercised too.
#[test]
fn r1_cylinder_gap_three_regimes_on_real_geometry() {
    for (bore_r, pin_r, off, expect) in [
        (0.3, 0.2, 0.0, 0.1),    // clearance, structural coaxial
        (0.3, 0.3, 0.0, 0.0),    // contact, exact arithmetic
        (0.3, 0.35, 0.0, -0.05), // interference
        (0.3, 0.2, 0.04, 0.06),  // clearance eaten by axis offset
    ] {
        let (doc, bore, pin) = cylinders(bore_r, pin_r, off);
        let ev = eval(&doc);
        let (doc, m) = insert(
            &doc,
            Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
                vec![wall(&ev, bore), wall(&ev, pin)],
            )
            .expect("indices in range"),
        );
        let (g, _) = measured(&eval(&doc), m);
        assert!(
            (g - expect).abs() < 1e-12,
            "gap({bore_r}, {pin_r}, d={off}) = {expect}, got {g}"
        );
        if expect == 0.0 {
            assert_eq!(g, 0.0, "the contact regime is exact arithmetic");
        }
    }
}

/// Swapping the gap's roles NEGATES it for cylinders at zero offset
/// (`r_p − r_b − d`), which is why the roles are authored: an
/// interference fit read with swapped roles would silently claim
/// clearance.
#[test]
fn r1_cylinder_gap_role_swap_negates() {
    let (doc, bore, pin) = cylinders(0.3, 0.2, 0.0);
    let ev = eval(&doc);
    let (b_wall, p_wall) = (wall(&ev, bore), wall(&ev, pin));
    let (doc, fwd) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
            vec![b_wall.clone(), p_wall.clone()],
        )
        .expect("indices in range"),
    );
    let (doc, rev) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
            vec![p_wall, b_wall],
        )
        .expect("indices in range"),
    );
    let ev = eval(&doc);
    let (f, _) = measured(&ev, fwd);
    let (r, _) = measured(&ev, rev);
    assert!((f - 0.1).abs() < 1e-12 && (r + 0.1).abs() < 1e-12, "{f} vs {r}");
}

// ---- skew axes refuse (untested in the PR) ----

/// Perpendicular cylinder axes: both `gap` and `distance` refuse
/// typed. The pin is extruded on the yz plane, so its axis is x̂
/// against the bore's ẑ.
#[test]
fn r1_skew_cylinder_axes_refuse_typed() {
    let doc = ProfileDoc::empty(DocumentId::derive("m10-2-r1-skew"), Tol::witness());
    let (doc, p1) = insert(
        &doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(0.0)],
                radius: len(0.3),
            }],
        }),
    );
    let (doc, bore) = insert(
        &doc,
        Node::Extrude {
            profile: p1,
            distance: len(0.1),
        },
    );
    let (doc, p2) = insert(
        &doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::yz(),
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(1.0)],
                radius: len(0.2),
            }],
        }),
    );
    let (doc, pin) = insert(
        &doc,
        Node::Extrude {
            profile: p2,
            distance: len(0.1),
        },
    );
    let ev = eval(&doc);
    let (b_wall, p_wall) = (wall(&ev, bore), wall(&ev, pin));
    let (doc, g) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
            vec![b_wall.clone(), p_wall.clone()],
        )
        .expect("indices in range"),
    );
    let (doc, d) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![b_wall, p_wall],
        )
        .expect("indices in range"),
    );
    let ev = eval(&doc);
    for id in [g, d] {
        let err = failed_kind(&ev, id);
        assert!(
            matches!(err, NodeErrorKind::MeasureUnsupported(_)),
            "skew axes must refuse typed, got {err:?}"
        );
    }
}

// ---- Dual64: the open door (no seeding door exists until M10-4) ----

/// A measure and its verdict evaluate at `Dual64` with the value
/// channel BIT-identical to the f64 run and a zero tangent (nothing
/// seeds a parameter through the public door — that is M10-4's).
#[test]
fn r1_measure_at_dual64_value_channel_is_bit_identical_tangent_zero() {
    use geom_core::Dual64;
    let (doc, slab) = slab();
    let ev = eval(&doc);
    let [bottom, top] = caps(&ev, slab);
    let (doc, m) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![bottom, top],
        )
        .expect("indices in range"),
    );
    let (doc, a) = insert(
        &doc,
        Node::Assertion {
            measure: m,
            bound: Expr::literal(0.1, Dimension::Length).expect("finite"),
            dir: AssertionDir::AtLeast,
        },
    );
    let at_f64 = measured(&eval(&doc), m).0;
    let ev_d = evaluate::<Dual64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match ev_d.nodes.get(&m) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => {
                assert_eq!(
                    value.value.to_bits(),
                    at_f64.to_bits(),
                    "the Dual value channel must be bit-identical to f64"
                );
                assert_eq!(value.deriv, 0.0, "unseeded tangent is zero");
            }
            other => panic!("expected a measure, got {}", other.kind_name()),
        },
        other => panic!("the measure did not evaluate at Dual64: {other:?}"),
    }
    match ev_d.nodes.get(&a) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Assertion(verdict) => {
                assert_eq!(verdict.holds(), Some(true), "0.35 >= 0.1 holds at Dual64");
            }
            other => panic!("expected an assertion, got {}", other.kind_name()),
        },
        other => panic!("the assertion did not evaluate at Dual64: {other:?}"),
    }
}

// ---- Interval: containment for gap and angle too ----

/// The gap and the angle contain their f64 values at `Interval` (the
/// PR's own containment row covers only the web distance).
#[cfg(feature = "interval")]
#[test]
fn r1_gap_and_angle_at_interval_contain_the_f64_values() {
    use geom_core::{Bounds, Interval};
    let (doc, bore, pin) = cylinders(0.3, 0.2, 0.04);
    let ev = eval(&doc);
    let (doc, m) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 1 }),
            vec![wall(&ev, bore), wall(&ev, pin)],
        )
        .expect("indices in range"),
    );
    let at_f64 = measured(&eval(&doc), m).0;
    let ev_i = evaluate::<Interval>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match ev_i.nodes.get(&m) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => {
                assert!(
                    value.lo() <= at_f64 && at_f64 <= value.hi(),
                    "[{}, {}] must contain {at_f64}",
                    value.lo(),
                    value.hi()
                );
            }
            other => panic!("expected a measure, got {}", other.kind_name()),
        },
        other => panic!("the gap did not evaluate at Interval: {other:?}"),
    }
}

// ---- the assertion's three states (Unevaluated untested in the PR) ----

/// A bound the run's band cannot separate from the measurement yields
/// `Unevaluated` — the third state, which no PR row produces; a bound
/// hit EXACTLY yields `Holds` (non-strict).
#[test]
fn r1_assertion_at_the_bound_holds_and_in_the_band_is_unevaluated() {
    let (doc, slab) = slab();
    let ev = eval(&doc);
    let [bottom, top] = caps(&ev, slab);
    let (doc, m) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![bottom, top],
        )
        .expect("indices in range"),
    );
    // Exactly at the bound: comparand 0, non-strict holds.
    let (doc_eq, a_eq) = insert(
        &doc,
        Node::Assertion {
            measure: m,
            bound: Expr::literal(DEPTH, Dimension::Length).expect("finite"),
            dir: AssertionDir::AtLeast,
        },
    );
    match verdict(&eval(&doc_eq), a_eq) {
        AssertionVerdict::Holds { measured, bound } => {
            assert_eq!(measured, bound, "at the bound exactly");
        }
        other => panic!("at the bound a non-strict relation holds, got {other:?}"),
    }
    // In the sliver band: eps < |comparand| < K*eps at the witness
    // tolerance — the run must refuse to pick a side.
    let eps = Tol::witness().eps();
    let (doc_band, a_band) = insert(
        &doc,
        Node::Assertion {
            measure: m,
            bound: Expr::literal(DEPTH - 5.0 * eps, Dimension::Length).expect("finite"),
            dir: AssertionDir::AtLeast,
        },
    );
    match verdict(&eval(&doc_band), a_band) {
        AssertionVerdict::Unevaluated { .. } => {}
        other => panic!("a 5ε margin lies in the band (ε..10ε), got {other:?}"),
    }
}

// ---- report-only: attack through the op vocabulary ----

/// A verdict (or a measure) fed to a body-consuming op is a TYPED
/// refusal, never a silent skip or a panic.
#[test]
fn r1_ops_refuse_measurement_operands_typed() {
    let (doc, slab) = slab();
    let ev = eval(&doc);
    let [bottom, top] = caps(&ev, slab);
    let (doc, m) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![bottom, top],
        )
        .expect("indices in range"),
    );
    let (doc, a) = insert(
        &doc,
        Node::Assertion {
            measure: m,
            bound: Expr::literal(0.1, Dimension::Length).expect("finite"),
            dir: AssertionDir::AtLeast,
        },
    );
    // Boolean over the ASSERTION's id.
    let (doc, bool_over_verdict) = insert(
        &doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Subtract,
            a: slab,
            b: a,
            declare: None,
        },
    );
    // Transform of the MEASURE's id.
    let (doc, moved_measure) = insert(
        &doc,
        Node::Transform {
            input: m,
            translation: [len(0.1), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let ev = eval(&doc);
    for id in [bool_over_verdict, moved_measure] {
        match ev.nodes.get(&id) {
            Some(NodeResult::Failed(_)) => {}
            other => panic!("an op over a measurement value must fail typed, got {other:?}"),
        }
    }
}

// ---- deviation 3: what a reference to a MOVED entity reads ----

/// EVIDENCE-ONLY (deviation 3, "carriers as minted"): a wall selected
/// FROM THE TRANSFORM node's own selection door measures at its
/// UNMOVED position. The pin sits at x = 0.5 and the transform moves
/// it to x = 0.75, yet the measure over the name the transform's door
/// hands out reports 0.5 — because the transform re-emits the
/// EXTRUDE's name (the observed mint below) and resolution reads the
/// minting node's value.
///
/// The deviation's own escape hatch — "measuring the moved one means
/// referencing the moving node's own emission" — is checked here too:
/// this row also asserts the transform's table carries NO
/// transform-minted cylinder-face name, i.e. the escape hatch does
/// not exist through the public selection door. If either half goes
/// red, the semantics changed and the review record should be
/// revisited.
#[test]
fn r1_a_wall_selected_from_a_transform_measures_the_unmoved_carrier() {
    let (doc, bore, pin) = cylinders(0.3, 0.2, 0.5);
    let (doc, moved) = insert(
        &doc,
        Node::Transform {
            input: pin,
            translation: [len(0.25), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    );
    let ev = eval(&doc);
    let bore_wall = wall(&ev, bore);
    let transform_walls = faces_of_kind(&ev, moved, geom_brep::SurfaceKind::Cylinder);
    assert!(
        !transform_walls.is_empty(),
        "the moved body still has its wall"
    );
    // The escape-hatch check: every name the transform's door hands
    // out is minted by the EXTRUDE, so there is no "moving node's own
    // emission" to reference.
    for w in &transform_walls {
        assert_eq!(
            w.node, pin,
            "the transform re-emits the extrude's names; a transform-minted \
             name would be the deviation's escape hatch, and it appeared: {w:?}"
        );
    }
    let moved_wall = transform_walls[0].clone();
    let (doc, m) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![bore_wall, moved_wall],
        )
        .expect("indices in range"),
    );
    let (d, _) = measured(&eval(&doc), m);
    assert!(
        (d - 0.5).abs() < 1e-12,
        "pinned observed semantics: the measure reads the carrier as \
         MINTED (0.5), not as placed (0.75); got {d}"
    );
}

// ---- corrupt v16 files: the load-door re-checks the PR left untested ----

/// A measured document whose assertion bound carries a distinctive
/// literal, for byte-corruption probes.
fn corruptible() -> ProfileDoc {
    let (doc, slab) = slab();
    let ev = eval(&doc);
    let [bottom, top] = caps(&ev, slab);
    let (doc, m) = insert(
        &doc,
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![bottom, top],
        )
        .expect("indices in range"),
    );
    let (doc, _) = insert(
        &doc,
        Node::Assertion {
            measure: m,
            bound: Expr::literal(0.777, Dimension::Length).expect("finite"),
            dir: AssertionDir::AtLeast,
        },
    );
    doc
}

/// The corruption rows: each plants bytes no door of this build can
/// write and demands the typed load-door refusal.
#[test]
fn r1_corrupt_v16_files_refuse_typed_at_the_load_door() {
    let doc = corruptible();
    let text = save(&doc, &[], Tol::witness()).expect("saves");

    // (a) The bound's dimension: Length → Angle. The bound literal is
    // distinctive, so locate its dim line relative to it.
    let target = "\"value\": 0.777,\n              \"dim\": \"Length\"";
    let (target, replacement) = if text.contains(target) {
        (target.to_string(), target.replace("Length", "Angle"))
    } else {
        // Fall back to a whitespace-insensitive locate: find the literal,
        // then the next "Length" after it.
        let at = text.find("0.777").expect("the bound literal is in the file");
        let dim_at = text[at..].find("\"Length\"").expect("its dim follows") + at;
        let t = &text[at..dim_at + 8];
        (t.to_string(), t.replace("Length", "Angle"))
    };
    assert_eq!(text.matches(&target).count(), 1);
    let corrupt = text.replace(&target, &replacement);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::AssertionBound {
            measured: Some(Dimension::Length),
            bound: Dimension::Angle,
            ..
        })) => {}
        other => panic!("a mismatched bound dim must refuse AssertionBound, got {other:?}"),
    }

    // (b) The assertion's target: point it at the profile (node 0).
    // The slab is profile 0 + extrude 1, the measure is 2.
    let target = "\"measure\": 2";
    assert_eq!(text.matches(target).count(), 1, "{target:?} must be unique");
    let corrupt = text.replace(target, "\"measure\": 0");
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::AssertionBound {
            measured: None, ..
        })) => {}
        other => panic!("a non-measure target must refuse AssertionBound, got {other:?}"),
    }

    // (c) A reference whose minting node does not exist. The refs are
    // minted by the extrude (node 1).
    let target = "\"node\": 1,";
    let n = text.matches(target).count();
    assert!(n >= 1, "the measure's refs name node 1");
    let corrupt = text.replacen(target, "\"node\": 77,", 1);
    match load(&corrupt, Tol::witness()) {
        // Two typed gates can own this corruption: the id-counter
        // check (77 was never minted) or the dangling-input walk.
        // Either is a loud load-door refusal, which is the claim.
        Err(PersistError::Snapshot(
            SnapshotError::DanglingInput { .. } | SnapshotError::IdBeyondCounter { .. },
        )) => {}
        other => panic!("a dangling minting node must refuse typed at load, got {other:?}"),
    }
}

// ---- the edit door checks payload params (claim 3 rounding-out) ----

/// A measured expression referencing an UNDECLARED parameter refuses
/// at the edit door with the payload-specific vocabulary.
#[test]
fn r1_an_unknown_payload_param_refuses_at_the_edit_door() {
    let (doc, slab) = slab();
    let ev = eval(&doc);
    let [bottom, top] = caps(&ev, slab);
    let expr = MeasureExpr::sub(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::value(Expr::param(ParamName::new("ghost"), Dimension::Length)),
    )
    .expect("Length - Length");
    let err = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(expr, vec![bottom, top]).expect("indices in range"),
        },
        Tol::witness(),
    )
    .expect_err("an undeclared parameter refuses");
    assert!(
        matches!(err, EditError::UnknownPayloadParam { .. }),
        "got {err:?}"
    );
}

// ---- a parameter edit under a measured bound moves the verdict ----

/// The e2e re-derivation the dispatch asks for, on THIS suite's own
/// geometry: web = distance − 2r with authored numbers, flipped by a
/// parameter edit, both numbers on the Violated verdict.
#[test]
fn r1_own_document_web_and_flip() {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-2-r1-web"), Tol::witness());
    doc = push(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("r"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.1,
                distribution: None,
            },
        },
    );
    let circle = |cx: f64| {
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops: vec![LoopProgram::Circle {
                centre: [len(cx), len(0.0)],
                radius: Expr::param(ParamName::new("r"), Dimension::Length),
            }],
        })
    };
    let (d2, p1) = insert(&doc, circle(-0.25));
    let (d3, e1) = insert(
        &d2,
        Node::Extrude {
            profile: p1,
            distance: len(0.05),
        },
    );
    let (d4, p2) = insert(&d3, circle(0.25));
    let (d5, e2) = insert(
        &d4,
        Node::Extrude {
            profile: p2,
            distance: len(0.05),
        },
    );
    let _ = p2;
    let ev = eval(&d5);
    let r = || MeasureExpr::value(Expr::param(ParamName::new("r"), Dimension::Length));
    let web = MeasureExpr::sub(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::add(r(), r()).expect("Length + Length"),
    )
    .expect("Length - Length");
    let (d6, m) = insert(
        &d5,
        Node::measure(web, vec![wall(&ev, e1), wall(&ev, e2)]).expect("indices in range"),
    );
    let (d7, a) = insert(
        &d6,
        Node::Assertion {
            measure: m,
            bound: Expr::literal(0.05, Dimension::Length).expect("finite"),
            dir: AssertionDir::AtLeast,
        },
    );
    // Web = 0.5 − 0.2 = 0.3 ≥ 0.05: Holds.
    let ev = eval(&d7);
    let (w, _) = measured(&ev, m);
    assert!((w - 0.3).abs() < 1e-12, "web = 0.5 − 0.2 = 0.3, got {w}");
    assert_eq!(verdict(&ev, a).holds(), Some(true));
    // r → 0.24: web = 0.5 − 0.48 = 0.02 < 0.05: Violated, both numbers.
    let d8 = push(
        &d7,
        &DocEdit::SetDocParamValue {
            name: ParamName::new("r"),
            value: DocParamValue::Continuous(0.24),
        },
    );
    match verdict(&eval(&d8), a) {
        AssertionVerdict::Violated { measured, bound } => {
            assert!((measured - 0.02).abs() < 1e-12, "web 0.02, got {measured}");
            assert!((bound - 0.05).abs() < 1e-15, "bound 0.05, got {bound}");
        }
        other => panic!("0.02 < 0.05 must violate, got {other:?}"),
    }
}
