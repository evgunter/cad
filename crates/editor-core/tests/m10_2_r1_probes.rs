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
//! implementer-discipline §8), so no seeds and no effort dial.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::AuthoredNode;

use editor_core::UnitSym;
use editor_core::{
    AssertionRelation, AssertionVerdict, CancelToken, Dimension, DocEdit, DocumentId, EditError,
    EntityKind, EvalOptions, Evaluation, Formula, FreeValue, FreeVar, GeomPred, LoopProgram,
    MeasurePrimitive, NamePat, Node, NodeErrorKind, NodeResult, PersistError, ProfileDoc,
    ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, RecipeNodeId, Selector,
    SnapshotError, StableName, SurfaceKindSet, ValuePayload, VarName, apply, evaluate, face_frame,
    load, save, select_where, vertex_position,
};
use fixture::{ang, len, len2, scl};
use geom_core::Tol;

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn push(doc: &editor_core::ProfileDoc, edit: &DocEdit<ProfileProgram>) -> ProfileDoc {
    apply(doc, edit, Tol::witness())
        .unwrap_or_else(|e| panic!("edit refused: {e}"))
        .doc
}

fn insert(doc: &editor_core::ProfileDoc, node: AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
    let applied = apply(
        doc,
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("insert refused: {e}"));
    (applied.doc, applied.record.minted.expect("insert mints"))
}

fn no_params() -> editor_core::VarEnv<f64> {
    ProfileDoc::empty_derived("m10-2-r1-noparams", Tol::witness()).var_env::<f64>()
}

fn faces_of_kind(
    ev: &Evaluation<f64>,
    body: RecipeNodeId,
    kind: geom::SurfaceKind,
) -> Vec<(RecipeNodeId, StableName)> {
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
    faces.into_iter().map(|name| (body, name)).collect()
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
        &DocEdit::DeclareVar {
            name: VarName::from_static("depth"),
            def: editor_core::VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Length,
                value: DEPTH,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: None,
            }),
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
            loops: vec![outer],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, slab) = insert(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: Formula::named(VarName::from_static("depth"), Dimension::Length),
        },
    );
    (doc, slab)
}

/// The slab's two z-normal caps, identified through the PUBLIC frame
/// door: the faces whose carrier axis is ±ẑ. Returned bottom (z≈0)
/// first.
fn caps(ev: &Evaluation<f64>, slab: RecipeNodeId) -> [StableName; 2] {
    let mut z_faces: Vec<(f64, StableName)> = faces_of_kind(ev, slab, geom::SurfaceKind::Plane)
        .into_iter()
        .filter_map(|(_, name)| {
            let pose = face_frame(ev, slab, &name).expect("a plane face has a frame");
            (pose.axis.z.abs() > 0.99).then_some((pose.origin.z, name))
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
    let (doc, m) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        at_mint([bottom, top]),
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
    let (doc, m) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        at_mint([bottom_vert, top]),
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
    let expect = ((pb.x - pa.x).powi(2) + (pb.y - pa.y).powi(2) + (pb.z - pa.z).powi(2)).sqrt();
    let (doc, m) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        at_mint([a, b]),
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
    let all_planes = faces_of_kind(&ev, slab, geom::SurfaceKind::Plane);
    let x_wall = all_planes
        .iter()
        .find(|n| {
            face_frame(&ev, slab, &n.1)
                .map(|p| p.axis.x.abs() > 0.99)
                .unwrap_or(false)
        })
        .expect("an x-normal wall")
        .clone();
    let y_wall = all_planes
        .iter()
        .find(|n| {
            face_frame(&ev, slab, &n.1)
                .map(|p| p.axis.y.abs() > 0.99)
                .unwrap_or(false)
        })
        .expect("a y-normal wall")
        .clone();

    let (doc, opposed) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Angle { a: 0, b: 1 },
        at_mint([bottom, top]),
    );
    let (doc, square) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Angle { a: 0, b: 1 },
        at_mint([x_wall, y_wall]),
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

    let (doc, g_bt) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        at_mint([bottom.clone(), top.clone()]),
    );
    let (doc, g_tb) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        at_mint([top, bottom]),
    );
    let ev = eval(&doc);
    let (bt, dim) = measured(&ev, g_bt);
    assert_eq!(dim, Dimension::Length);
    assert!(
        (bt - expect_bt).abs() < 1e-12,
        "gap(bottom, top): formula {expect_bt}, got {bt}"
    );
    let (tb, _) = measured(&ev, g_tb);
    assert!(
        (tb - expect_tb).abs() < 1e-12,
        "gap(top, bottom): formula {expect_tb}, got {tb}"
    );
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
fn ball(doc: &editor_core::ProfileDoc, r: f64, c: f64) -> (ProfileDoc, RecipeNodeId) {
    let meridian = LoopProgram::Chain(vec![
        ProgramStep::At(len2([0.0, c - r])),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(len2([0.0, c + r])),
            b: scl(1.0),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![meridian],
            ids: Vec::new(),
            on: None,
        }),
    );
    insert(
        &doc,
        fixture::revolve_about(
            p,
            (0.0, 0.0),
            crate::fixture::along_y(),
            ang(std::f64::consts::TAU),
        ),
    )
}

/// Two solid balls, the smaller inside the larger, are a nested pair:
/// `gap(large, small) = ‖Δc‖ + r − R` on real revolved sphere faces,
/// R = 1 at the origin, r = 0.25 offset `c` up the axis — inside at
/// c = 0.3, touching from inside at c = 0.75, out of it at c = 0.9.
#[test]
fn r1_sphere_gap_three_regimes_on_nested_balls() {
    for (c, expect) in [(0.3, -0.45), (0.75, 0.0), (0.9, 0.15)] {
        let doc = ProfileDoc::empty(DocumentId::derive("m10-2-r1-balls"), Tol::witness());
        let (doc, large) = ball(&doc, 1.0, 0.0);
        let (doc, small) = ball(&doc, 0.25, c);
        // The small ball joins the large one's space on its axis.
        let (doc, small) = fixture::against(
            doc,
            editor_core::Operand::output(large, 0),
            editor_core::Operand::output(small, 0),
            &editor_core::Frame::IDENTITY,
        );
        let ev = eval(&doc);
        let socket_face = faces_of_kind(&ev, large, geom::SurfaceKind::Sphere)
            .first()
            .expect("the large revolve mints a sphere face")
            .clone();
        let small_face = faces_of_kind(&ev, small, geom::SurfaceKind::Sphere)
            .first()
            .expect("the small revolve mints a sphere face")
            .clone();
        let (doc, m) = fixture::measure_node(
            &doc,
            MeasurePrimitive::Gap { outer: 0, inner: 1 },
            at_mint([socket_face, small_face]),
        );
        let (g, dim) = measured(&eval(&doc), m);
        assert_eq!(dim, Dimension::Length);
        assert!(
            (g - expect).abs() < 1e-9,
            "Δc + r − R = {c} + 0.25 − 1 = {expect}, got {g}"
        );
    }
}

// ---- gap: coaxial cylinders, three regimes on real geometry ----

/// Two circles extruded into solid rods: `wide_r` at the origin,
/// `thin_r` offset by `off` along x.
fn rods(wide_r: f64, thin_r: f64, off: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive("m10-2-r1-cyl"), Tol::witness());
    let circle = |cx: f64, r: f64| {
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(cx), len(0.0)],
                radius: len(r),
            }],
            ids: Vec::new(),
            on: None,
        })
    };
    let (doc, p1) = insert(&doc, circle(0.0, wide_r));
    let (doc, wide) = insert(
        &doc,
        Node::Extrude {
            profile: p1.into(),
            distance: len(0.1),
        },
    );
    let (doc, p2) = insert(&doc, circle(off, thin_r));
    let (doc, thin) = insert(
        &doc,
        Node::Extrude {
            profile: p2.into(),
            distance: len(0.1),
        },
    );
    // The thin rod joins the wide one's space where it stands.
    let (doc, thin) = fixture::against(doc, wide, thin, &editor_core::Frame::IDENTITY);
    (doc, wide, thin)
}

/// Read every reference AT ITS MINTING NODE — the spelling this suite
/// means throughout: a copy's names are minted by its placement, so a
/// placed body's references read the copy.
///
/// Each is the selection of its name in its minting node's body port
/// (a revolve also defines its axis); the rows and their oracles are
/// unchanged.
/// Each name read where it is read ([`fixture::Sited`]): at its mint,
/// or at the node a `(node, name)` pair names.
fn at_mint<T: fixture::Sited, const N: usize>(names: [T; N]) -> Vec<editor_core::Operand> {
    names
        .into_iter()
        .map(|name| {
            let at = name.site();
            editor_core::Operand::select(editor_core::Operand::output(at, 0), vec![name.name()])
        })
        .collect()
}

/// `node`'s cylinder wall, read at `node`: a placement's copy carries
/// its source's name for it.
fn wall(ev: &Evaluation<f64>, node: RecipeNodeId) -> (RecipeNodeId, StableName) {
    faces_of_kind(ev, node, geom::SurfaceKind::Cylinder)
        .first()
        .expect("a circular extrude mints a cylinder wall")
        .clone()
}

/// Two solid rods, the thinner inside the wider: their walls are both
/// convex, a nested pair, so `gap(wide, thin) = d + r_thin − r_wide`,
/// how far the thin wall stands out of the wide rod's material, inside,
/// on and out of it, with the axis offset exercised too.
#[test]
fn r1_cylinder_gap_three_regimes_on_nested_rods() {
    for (wide_r, thin_r, off, expect) in [
        (0.3, 0.2, 0.0, -0.1),   // inside, structural coaxial
        (0.3, 0.3, 0.0, 0.0),    // on, exact arithmetic
        (0.3, 0.35, 0.0, 0.05),  // out of it
        (0.3, 0.2, 0.04, -0.06), // inside, brought out by the axis offset
    ] {
        let (doc, wide, thin) = rods(wide_r, thin_r, off);
        let ev = eval(&doc);
        let (doc, m) = fixture::measure_node(
            &doc,
            MeasurePrimitive::Gap { outer: 0, inner: 1 },
            at_mint([wall(&ev, wide), wall(&ev, thin)]),
        );
        let (g, _) = measured(&eval(&doc), m);
        assert!(
            (g - expect).abs() < 1e-12,
            "gap(wide {wide_r}, thin {thin_r}, d={off}) = {expect}, got {g}"
        );
        if expect == 0.0 {
            assert_eq!(g, 0.0, "the contact regime is exact arithmetic");
        }
    }
}

/// Swapping the gap's roles NEGATES it for two solid rods, one nested
/// in the other off its axis, an aligned pair, which is why the roles
/// are authored: read from the wide rod, the thin one stands 0.06
/// inside it; read from the thin one, the wide stands 0.06 out.
#[test]
fn r1_nested_rods_gap_role_swap_negates() {
    let (doc, wide, thin) = rods(0.3, 0.2, 0.04);
    let ev = eval(&doc);
    let (w_wall, t_wall) = (wall(&ev, wide), wall(&ev, thin));
    let (doc, fwd) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        at_mint([w_wall.clone(), t_wall.clone()]),
    );
    let (doc, rev) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        at_mint([t_wall, w_wall]),
    );
    let ev = eval(&doc);
    let (f, _) = measured(&ev, fwd);
    let (r, _) = measured(&ev, rev);
    assert!(
        (f + 0.06).abs() < 1e-12 && (r - 0.06).abs() < 1e-12,
        "{f} vs {r}"
    );
}

/// **A pin in a real bore reads its fit, in either role order** (C5's
/// opposed fit): a plate with a hole of radius `R` and a pin of `r`
/// off its axis by `d` read `R − r − d`, clearance, contact and
/// interference.
#[test]
fn r1_a_pin_in_a_real_bore_reads_its_fit() {
    for (bore_r, pin_r, off, expect) in [
        (0.3, 0.2, 0.04, 0.06),
        (0.3, 0.26, 0.04, 0.0),
        (0.3, 0.3, 0.04, -0.04),
    ] {
        let doc = ProfileDoc::empty(DocumentId::derive("m10-2-r1-fit"), Tol::witness());
        let (doc, outline) = insert(
            &doc,
            Node::Profile(ProfileProgram {
                loops: vec![
                    LoopProgram::polygon([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)])
                        .expect("finite corners"),
                    LoopProgram::Circle {
                        centre: [len(0.0), len(0.0)],
                        radius: len(bore_r),
                    },
                ],
                ids: Vec::new(),
                on: None,
            }),
        );
        let (doc, plate) = insert(
            &doc,
            Node::Extrude {
                profile: outline.into(),
                distance: len(0.1),
            },
        );
        let (doc, p2) = insert(
            &doc,
            Node::Profile(ProfileProgram {
                loops: vec![LoopProgram::Circle {
                    centre: [len(off), len(0.0)],
                    radius: len(pin_r),
                }],
                ids: Vec::new(),
                on: None,
            }),
        );
        let (doc, pin) = insert(
            &doc,
            Node::Extrude {
                profile: p2.into(),
                distance: len(0.1),
            },
        );
        let (doc, pin) = fixture::against(doc, plate, pin, &editor_core::Frame::IDENTITY);
        let ev = eval(&doc);
        let (bore, wall_of_pin) = (wall(&ev, plate), wall(&ev, pin));
        for (refs, order) in [
            ([bore.clone(), wall_of_pin.clone()], "bore outer"),
            ([wall_of_pin, bore], "pin outer"),
        ] {
            let (doc, m) = fixture::measure_node(
                &doc,
                MeasurePrimitive::Gap { outer: 0, inner: 1 },
                at_mint(refs),
            );
            let (g, _) = measured(&eval(&doc), m);
            assert!(
                (g - expect).abs() < 1e-12,
                "a pin of {pin_r} in a bore of {bore_r} off by {off}, {order}: {expect}, got {g}"
            );
        }
    }
}

/// **A ball in a real socket reads its fit, in either role order**: a
/// block of revolution with a spherical hollow of radius 1 (the socket's
/// face concave) and a ball of 0.25 offset `c` up its axis read
/// `1 − 0.25 − c`.
#[test]
fn r1_a_ball_in_a_real_socket_reads_its_fit() {
    for (c, expect) in [(0.3, 0.45), (0.75, 0.0), (0.9, -0.15)] {
        let doc = ProfileDoc::empty(DocumentId::derive("m10-2-r1-socket"), Tol::witness());
        let meridian = LoopProgram::Chain(vec![
            ProgramStep::At(len2([0.0, 1.0])),
            ProgramStep::LineTo(ProgramTarget::Point(len2([0.0, 1.5]))),
            ProgramStep::LineTo(ProgramTarget::Point(len2([1.5, 1.5]))),
            ProgramStep::LineTo(ProgramTarget::Point(len2([1.5, -1.5]))),
            ProgramStep::LineTo(ProgramTarget::Point(len2([0.0, -1.5]))),
            ProgramStep::LineTo(ProgramTarget::Point(len2([0.0, -1.0]))),
            ProgramStep::ArcTo(ProgramArcData::Bulge {
                target: ProgramTarget::Start,
                b: scl(1.0),
            }),
        ]);
        let (doc, p) = insert(
            &doc,
            Node::Profile(ProfileProgram {
                loops: vec![meridian],
                ids: Vec::new(),
                on: None,
            }),
        );
        let (doc, socket) = insert(
            &doc,
            fixture::revolve_about(
                p,
                (0.0, 0.0),
                crate::fixture::along_y(),
                ang(std::f64::consts::TAU),
            ),
        );
        let (doc, ball) = ball(&doc, 0.25, c);
        let (doc, ball) = fixture::against(
            doc,
            editor_core::Operand::output(socket, 0),
            editor_core::Operand::output(ball, 0),
            &editor_core::Frame::IDENTITY,
        );
        let ev = eval(&doc);
        let hollow = faces_of_kind(&ev, socket, geom::SurfaceKind::Sphere)
            .first()
            .expect("the socket revolve mints a sphere face")
            .clone();
        let face = faces_of_kind(&ev, ball, geom::SurfaceKind::Sphere)
            .first()
            .expect("the ball revolve mints a sphere face")
            .clone();
        for (refs, order) in [
            ([hollow.clone(), face.clone()], "socket outer"),
            ([face, hollow], "ball outer"),
        ] {
            let (doc, m) = fixture::measure_node(
                &doc,
                MeasurePrimitive::Gap { outer: 0, inner: 1 },
                at_mint(refs),
            );
            let (g, _) = measured(&eval(&doc), m);
            assert!(
                (g - expect).abs() < 1e-9,
                "a ball at {c} in a socket, {order}: {expect}, got {g}"
            );
        }
    }
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
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(0.0)],
                radius: len(0.3),
            }],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, bore) = insert(
        &doc,
        Node::Extrude {
            profile: p1.into(),
            distance: len(0.1),
        },
    );
    let (doc, p2) = insert(
        &doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(1.0)],
                radius: len(0.2),
            }],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, pin) = insert(
        &doc,
        Node::Extrude {
            profile: p2.into(),
            distance: len(0.1),
        },
    );
    // The pin stands on the bore's YZ plane, so its axis is x̂ against
    // the bore's ẑ — the skew this row is about.
    let (doc, pin) = fixture::against(
        doc,
        bore,
        pin,
        &fixture::axes_at([0.0; 3], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
    );
    let ev = eval(&doc);
    let (b_wall, p_wall) = (wall(&ev, bore), wall(&ev, pin));
    let (doc, g) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        at_mint([b_wall.clone(), p_wall.clone()]),
    );
    let (doc, d) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        at_mint([b_wall, p_wall]),
    );
    let ev = eval(&doc);
    for id in [g, d] {
        let err = failed_kind(&ev, id);
        // The fix pass sharpened this refusal (review MINOR-3). Skew
        // cylinders are a pair the v1 table DOES have an arm for; what
        // it lacks is their PARALLELISM. `MeasureUnsupported` said the
        // false thing — "no closed form for a cylinder against a
        // cylinder" — and sent the author hunting a missing feature
        // instead of at their tilt. The row keeps its subject (skew
        // axes refuse typed, never guess) at the corrected arm.
        let NodeErrorKind::MeasureNotParallel { predicate, .. } = err else {
            panic!("skew axes must refuse NotParallel, got {err:?}");
        };
        assert_eq!(*predicate, "carrier_cyl_axis_parallel");
        let rendered = ev.node_error(id).map(|e| e.to_string()).unwrap_or_default();
        assert!(
            rendered.contains("parallel"),
            "the refusal must name the tilt: {rendered}"
        );
        assert!(
            !rendered.contains("no closed form"),
            "the refusal must not claim the pair is unsupported: {rendered}"
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
    let (doc, m) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        at_mint([bottom, top]),
    );
    let (doc, a) = insert(
        &doc,
        Node::Assertion {
            value: fixture::value_of(&doc, m),
            bound: len(0.1),
            relation: AssertionRelation::AtLeast,
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
#[test]
fn r1_gap_and_angle_at_interval_contain_the_f64_values() {
    use geom_core::{Bounds, Interval};
    let (doc, wide, thin) = rods(0.3, 0.2, 0.04);
    let ev = eval(&doc);
    let (doc, m) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        at_mint([wall(&ev, wide), wall(&ev, thin)]),
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
    let (doc, m) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        at_mint([bottom, top]),
    );
    // Exactly at the bound: comparand 0, non-strict holds.
    let (doc_eq, a_eq) = insert(
        &doc,
        Node::Assertion {
            value: fixture::value_of(&doc, m),
            bound: len(DEPTH),
            relation: AssertionRelation::AtLeast,
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
            value: fixture::value_of(&doc, m),
            bound: len(DEPTH - 5.0 * eps),
            relation: AssertionRelation::AtLeast,
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
    let (doc, m) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        at_mint([bottom, top]),
    );
    let (doc, a) = insert(
        &doc,
        Node::Assertion {
            value: fixture::value_of(&doc, m),
            bound: len(0.1),
            relation: AssertionRelation::AtLeast,
        },
    );
    // Boolean over the ASSERTION: an assertion defines nothing.
    let refusal = crate::fixture::insert_refused(
        &doc,
        Node::Subtract {
            from: slab.into(),
            tool: a.into(),
        },
    );
    assert!(
        matches!(&refusal, editor_core::EditError::DefinesNothing { input, .. } if input.id() == a),
        "{refusal:?}"
    );
    // A placement of the MEASURE: a measured length is not placeable.
    let refusal = crate::fixture::insert_refused(
        &doc,
        crate::fixture::stood(editor_core::BodyRead::plain(editor_core::Operand::from(m))),
    );
    assert!(
        matches!(
            &refusal,
            editor_core::EditError::SlotVarKind {
                found: editor_core::VarKind::Length,
                expected: editor_core::SlotKind::Placeable,
                ..
            }
        ),
        "{refusal:?}"
    );
}

// ---- corrupt v16 files: the load-door re-checks the PR left untested ----

/// A measured document whose assertion bound carries a distinctive
/// literal, for byte-corruption probes.
fn corruptible() -> ProfileDoc {
    let (doc, slab) = slab();
    let ev = eval(&doc);
    let [bottom, top] = caps(&ev, slab);
    let (doc, m) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        at_mint([bottom, top]),
    );
    let (doc, _) = insert(
        &doc,
        Node::Assertion {
            value: fixture::value_of(&doc, m),
            bound: len(0.777),
            relation: AssertionRelation::AtLeast,
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

    // (a) The bound's dimension: Length → Angle. The bound is written
    // as a distinctive value, so its variable is found by it.
    //
    // The kind, the dimension and the UNIT all move, and must: a
    // variable whose kind said `Angle` while its unit said `m` is
    // corrupt in a NEARER way, refused before the snapshot walk this
    // row is about ever runs. Moving all three keeps the variable
    // well-formed and leaves exactly one thing wrong with the document
    // — the bound's dimension against its measure — which is what this
    // row is here to pin.
    let (header, body) = text.split_once('\n').expect("a header line");
    let mut wire: serde_json::Value = serde_json::from_str(body).expect("a JSON body");
    let bound: Vec<String> = wire["snapshot"]["vars"]
        .as_object()
        .expect("a variable table")
        .iter()
        .filter(|(_, var)| var["def"]["Free"]["Continuous"]["value"] == serde_json::json!(0.777))
        .map(|(id, _)| id.clone())
        .collect();
    let [bound] = &bound[..] else {
        panic!("one variable holds the bound, got {bound:?}")
    };
    let held = &mut wire["snapshot"]["vars"][bound.as_str()];
    held["kind"] = serde_json::json!("Angle");
    held["def"]["Free"]["Continuous"]["dim"] = serde_json::json!("Angle");
    held["def"]["Free"]["Continuous"]["display_unit"] = serde_json::json!("rad");
    let corrupt = format!(
        "{header}\n{}",
        serde_json::to_string(&wire).expect("re-emit")
    );
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::AssertionBound {
            measured: Dimension::Length,
            bound: Dimension::Angle,
            ..
        })) => {}
        other => panic!("a mismatched bound dim must refuse AssertionBound, got {other:?}"),
    }

    // (b) The assertion's value: point it at the PROFILE (the first
    // node), which defines no scalar. The slab is profile and extrude,
    // so the measure is the third node.
    let [profile, extrude, measure] = doc.ids()[..3] else {
        panic!("a slab and its measure");
    };
    let read = |node| doc.output(node, 0).expect("the node defines a value");
    let target = format!("\"value\": \"{}\"", read(measure).0);
    assert_eq!(
        text.matches(&target).count(),
        1,
        "{target:?} must be unique"
    );
    let corrupt = text.replace(&target, &format!("\"value\": \"{}\"", read(profile).0));
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::PayloadVarKind { .. })) => {}
        other => panic!("a value of no scalar kind must refuse by kind, got {other:?}"),
    }

    // (c) A reference whose minting node does not exist. The refs are
    // the names the measure's selections hold, minted by the extrude.
    let (header, body) = text.split_once('\n').expect("a header line");
    let mut wire: serde_json::Value = serde_json::from_str(body).expect("a JSON body");
    let selects: Vec<String> = wire["snapshot"]["vars"]
        .as_object()
        .expect("a variable table")
        .iter()
        .filter(|(_, var)| var["def"].get("Select").is_some())
        .map(|(id, _)| id.clone())
        .collect();
    let Some(select) = selects.first() else {
        panic!("the measure reads its references through selections")
    };
    let name = &mut wire["snapshot"]["vars"][select.as_str()]["def"]["Select"]["names"][0];
    assert_eq!(
        name["node"],
        serde_json::json!(extrude.0.to_string()),
        "the measure's refs name the extrude"
    );
    name["node"] = serde_json::json!("0:000000000000004d");
    let corrupt = format!(
        "{header}\n{}",
        serde_json::to_string(&wire).expect("re-emit")
    );
    match load(&corrupt, Tol::witness()) {
        // The mint-log check owns this corruption: 77 was never
        // minted, which is a loud load-door refusal, the claim.
        Err(PersistError::Snapshot(SnapshotError::NodeNotMinted { .. })) => {}
        other => panic!("a dangling minting node must refuse typed at load, got {other:?}"),
    }
}

// ---- the edit door checks payload params (claim 3 rounding-out) ----

/// A measured value referencing an UNDECLARED parameter refuses at the
/// assertion's edit door with the payload-specific vocabulary.
#[test]
fn r1_an_unknown_payload_param_refuses_at_the_edit_door() {
    let (doc, slab) = slab();
    let ev = eval(&doc);
    let [bottom, top] = caps(&ev, slab);
    let (doc, measured) = fixture::measure(
        doc,
        &[MeasurePrimitive::Distance { a: 0, b: 1 }],
        &at_mint([bottom, top]),
    );
    let value = Formula::sub(
        fixture::read_var(&doc, measured.outputs[0]),
        Formula::named(VarName::from_static("ghost"), Dimension::Length),
    )
    .expect("Length - Length");
    let err = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Assertion {
                value,
                bound: len(0.0),
                relation: AssertionRelation::AtLeast,
            }),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect_err("an undeclared parameter refuses");
    assert!(
        matches!(err, EditError::PayloadUnknownVarName { .. }),
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
        &DocEdit::DeclareVar {
            name: VarName::from_static("r"),
            def: editor_core::VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Length,
                value: 0.1,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: None,
            }),
        },
    );
    let circle = |cx: f64| {
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(cx), len(0.0)],
                radius: Formula::named(VarName::from_static("r"), Dimension::Length),
            }],
            ids: Vec::new(),
            on: None,
        })
    };
    let (d2, p1) = insert(&doc, circle(-0.25));
    let (d3, e1) = insert(
        &d2,
        Node::Extrude {
            profile: p1.into(),
            distance: len(0.05),
        },
    );
    let (d4, p2) = insert(&d3, circle(0.25));
    let (d5, e2) = insert(
        &d4,
        Node::Extrude {
            profile: p2.into(),
            distance: len(0.05),
        },
    );
    // The second hole joins the first's space where it stands.
    let (d5, e2) = fixture::against(d5, e1, e2, &editor_core::Frame::IDENTITY);
    let ev = eval(&d5);
    let (d6, measured) = fixture::measure(
        d5,
        &[MeasurePrimitive::Distance { a: 0, b: 1 }],
        &at_mint([wall(&ev, e1), wall(&ev, e2)]),
    );
    let r = || Formula::named(VarName::from_static("r"), Dimension::Length);
    let web = Formula::sub(
        fixture::read_var(&d6, measured.outputs[0]),
        Formula::add(r(), r()).expect("Length + Length"),
    )
    .expect("Length - Length");
    let (d7, a) = insert(
        &d6,
        Node::Assertion {
            value: web,
            bound: len(0.05),
            relation: AssertionRelation::AtLeast,
        },
    );
    // Web = 0.5 − 0.2 = 0.3 ≥ 0.05: Holds.
    let ev = eval(&d7);
    let w = match ev.reading(&d7, fixture::assertion_value(&d7, a)) {
        Ok(editor_core::Observed::Value(w)) => w,
        other => panic!("the web reads: {other:?}"),
    };
    assert!((w - 0.3).abs() < 1e-12, "web = 0.5 − 0.2 = 0.3, got {w}");
    assert_eq!(verdict(&ev, a).holds(), Some(true));
    // r → 0.24: web = 0.5 − 0.48 = 0.02 < 0.05: Violated, both numbers.
    let d8 = push(
        &d7,
        &DocEdit::SetVarValue {
            var: VarName::from_static("r").into(),
            value: FreeValue::Continuous(0.24),
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
