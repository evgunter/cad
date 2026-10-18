//! **INTENT stage 3 PR D — a construction is a root, placed by a mate**
//! (`docs/INTENT-STAGE3-SPEC.md` §5, fork-log rows 159 and 161).
//!
//! A construction computes in its own coordinates and reads no frame.
//! These rows pin the placements §5's migration writes: a copy standing
//! where its body stands (a plane mate on a face no variable reaches, or
//! a plane-less construction's own axis), the chart a world mate's
//! values are read on (from the world's own frame, so a datum's in-plane
//! offset rides on the values), and a loft's stations.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, ang, insert, len};
use editor_core::pose::{PoseDef, PoseRole};
use editor_core::{
    BodyRead, Coord, EvalOptions, Formula, LoftSection, Mate, Node, NodeResult, Operand,
    ProfileDoc, RecipeNodeId, SlideAxis, StationMove, Target, TubeWindow, ValuePayload, WorldRead,
};
use geom_core::Tol;

/// Every vertex of the body `node` evaluates to, sorted, at `f64`.
fn vertices(doc: &ProfileDoc, node: RecipeNodeId) -> Vec<[f64; 3]> {
    let ev = editor_core::evaluate::<f64>(
        doc,
        None,
        &editor_core::CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let Some(NodeResult::Ok(value)) = ev.result(node) else {
        panic!("{node} evaluates: {:?}", ev.result(node));
    };
    let body = match &value.payload {
        ValuePayload::Body(b) => b.clone(),
        ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. }) => body.clone(),
        ValuePayload::Copies(copies) if copies.bodies.len() == 1 => copies.bodies[0].clone(),
        other => panic!("{node} is a body: {other:?}"),
    };
    let mut points: Vec<[f64; 3]> = body
        .vertices()
        .map(|(v, _)| {
            let p = topo::readback::vertex_point(&body, v).expect("a vertex point");
            [p.x, p.y, p.z]
        })
        .collect();
    points.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    points
}

/// A unit box on its own xy, its start cap on its own `z = 0`.
fn unit_box(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = insert(
        doc,
        Node::Profile(fixture::desc(vec![vec![
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.0, 1.0),
        ]])),
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// A placement of `body` constrained by `mate` and `values`.
fn placed(
    doc: ProfileDoc,
    body: RecipeNodeId,
    mate: Mate<Formula>,
    values: Vec<(Coord, Formula)>,
) -> (ProfileDoc, RecipeNodeId) {
    let read = fixture::body_read(&doc, body.into());
    let (doc, placement) = insert(doc, Node::place(vec![BodyRead::plain(read)]));
    (
        fixture::with_values(doc, placement, mate, values),
        placement,
    )
}

/// The box's start cap, read as a plane, flipped so it looks up its
/// own `+z` as the world's `xy` does.
fn start_cap(box_: RecipeNodeId) -> Operand {
    Operand::Pose(Box::new(PoseDef::Flip {
        pose: editor_core::pose::RoleRead::plain(fixture::plane(Operand::select(
            box_,
            vec![fixture::fname(
                box_,
                editor_core::RoleSeg::Cap(editor_core::CapEnd::Start),
            )],
        ))),
    }))
}

fn shifted(points: &[[f64; 3]], by: [f64; 3]) -> Vec<[f64; 3]> {
    points
        .iter()
        .map(|p| [p[0] + by[0], p[1] + by[1], p[2] + by[2]])
        .collect()
}

/// **A datum's in-plane offset rides on the values** (§5, row 161's
/// finding): a world mate's values are charted from the world's own
/// frame, not from the target read's moved one. A construction that
/// sat on a datum at (3, 4, 0), normal z, is placed by a plane mate to
/// the world's `xy` with `Slide X = 3`, `Slide Y = 4` and spin 0, and
/// stands at (3, 4, 0). The spelling §5 first wrote — the chain on the
/// world read, the constant `0` on every freedom — measures the slides
/// from the world's origin, so it stands at the origin, not at the
/// datum.
#[test]
fn a_datums_in_plane_offset_rides_on_the_values() {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, b) = unit_box(doc);
    let own = vertices(&doc, b);

    let values = |x: f64, y: f64| {
        vec![
            (Coord::Slide(SlideAxis::X), len(x)),
            (Coord::Slide(SlideAxis::Y), len(y)),
            (Coord::Spin, Formula::zero(editor_core::Dimension::Angle)),
        ]
    };
    let mate = |to: WorldRead<Formula>| Mate {
        on: start_cap(b),
        to: Target::World(to),
        class: editor_core::ContactClass::Rest,
    };

    let (on_values, p) = placed(
        doc.clone(),
        b,
        mate(WorldRead::through(PoseRole::Xy)),
        values(3.0, 4.0),
    );
    assert_eq!(vertices(&on_values, p), shifted(&own, [3.0, 4.0, 0.0]));

    let slid_read = WorldRead::through(PoseRole::Xy)
        .slide(WorldRead::through(PoseRole::X), len(3.0))
        .slide(WorldRead::through(PoseRole::Y), len(4.0));
    let zeros = vec![
        (
            Coord::Slide(SlideAxis::X),
            Formula::zero(editor_core::Dimension::Length),
        ),
        (
            Coord::Slide(SlideAxis::Y),
            Formula::zero(editor_core::Dimension::Length),
        ),
        (Coord::Spin, Formula::zero(editor_core::Dimension::Angle)),
    ];
    let (on_read, p) = placed(doc, b, mate(slid_read), zeros);
    assert_eq!(
        vertices(&on_read, p),
        own,
        "a world read's in-plane moves are measured away by values charted from the world's \
         frame"
    );
}

/// **A copy standing where its body stands** ([`editor_core::stand`]):
/// a plane mate on the extrude's start cap, which reads nothing, and
/// three constant zeros; the copy is its body, bit for bit.
#[test]
fn an_extrude_stands_on_its_start_cap() {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, b) = unit_box(doc);
    let stood = editor_core::stand(
        &doc,
        &Operand::from(b),
        &EvalOptions::default(),
        Tol::witness(),
    )
    .expect("the box stands");
    assert_eq!(stood.by, editor_core::PlacedBy::PlanarFace);
    assert!(
        stood.values.iter().all(|(_, by)| by.is_zero()),
        "{:?}",
        stood.values
    );
    let own = vertices(&doc, b);
    let (doc, p) = placed(doc, b, stood.mate, stood.values);
    assert_eq!(vertices(&doc, p), own);
}

/// **A full revolve stands on its own axis** (row 161, Q1): no face of
/// it lies on its own `xy`, so [`editor_core::stand`] writes an `Axis`
/// mate from the revolve's `axis` output to the world's `x` axis (the
/// heading the constant `0`, the axis through the constant zero), and
/// constant zeros on the slide along it and the spin about it.
#[test]
fn a_full_revolve_stands_on_its_own_axis() {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(fixture::desc(vec![vec![
            (0.0, -1.5),
            (1.0, -1.5),
            (1.0, -1.0),
            (0.0, -1.0),
        ]])),
    );
    let (doc, rev) = insert(
        doc,
        fixture::revolve_about(
            profile,
            (0.0, 0.0),
            fixture::along_x(),
            Formula::turn_over(1).expect("a turn"),
        ),
    );
    let own = vertices(&doc, rev);
    let read = fixture::body_read(&doc, rev.into());
    let stood = editor_core::stand(&doc, &read, &EvalOptions::default(), Tol::witness())
        .expect("the revolve stands");
    assert_eq!(stood.by, editor_core::PlacedBy::Axis);
    assert_eq!(stood.mate.on, Operand::output(rev, 1));
    assert_eq!(
        stood.mate.to,
        Target::World(WorldRead::through(PoseRole::X)),
        "a constant axis writes no move"
    );
    let (doc, p) = placed(doc, rev, stood.mate, stood.values);
    assert_eq!(vertices(&doc, p), own);
}

/// **A full ring stands on its torus's axis**, which reads nothing,
/// whatever its radii read.
#[test]
fn a_full_ring_stands_on_its_carrier_axis() {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, ring) = insert(
        doc,
        Node::Tube {
            major_radius: len(2.0),
            window: TubeWindow::Full,
            minor_radius: len(0.5),
        },
    );
    let stood = editor_core::stand(
        &doc,
        &Operand::from(ring),
        &EvalOptions::default(),
        Tol::witness(),
    )
    .expect("the ring stands");
    assert_eq!(stood.by, editor_core::PlacedBy::Axis);
    assert_eq!(
        stood.mate.to,
        Target::World(WorldRead::through(PoseRole::Z))
    );
    let own = vertices(&doc, ring);
    let (doc, p) = placed(doc, ring, stood.mate, stood.values);
    assert_eq!(vertices(&doc, p), own);
}

/// **A loft's later section stands at its station** (row 161, Q2): a
/// `Slide(Z)` of 1 carries the second square one up the first's own
/// `z`, and a spin a quarter turn about it turns it.
#[test]
fn a_loft_section_stands_at_its_station() {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let square = || Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 0.5)]));
    let (doc, a) = insert(doc, square());
    let (doc, b) = insert(doc, square());
    let section = |station| LoftSection {
        profile: Operand::from(b),
        station,
    };
    let (slid, loft) = insert(
        doc.clone(),
        Node::Loft {
            first: a.into(),
            then: vec![section(vec![StationMove::Slide {
                along: editor_core::Axis3::Z,
                by: len(1.0),
            }])],
            v_degree: Formula::count(1),
        },
    );
    let points = vertices(&slid, loft);
    let mut expect: Vec<[f64; 3]> = [0.0, 1.0]
        .into_iter()
        .flat_map(|z| {
            [(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)].map(|(x, y)| [x, y, z])
        })
        .collect();
    expect.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    assert_eq!(points, expect);

    let (spun, loft) = insert(
        doc,
        Node::Loft {
            first: a.into(),
            then: vec![section(vec![
                StationMove::Slide {
                    along: editor_core::Axis3::Z,
                    by: len(1.0),
                },
                StationMove::Spin {
                    about: editor_core::Axis3::Z,
                    by: ang(core::f64::consts::FRAC_PI_4),
                },
            ])],
            v_degree: Formula::count(1),
        },
    );
    let top: Vec<[f64; 3]> = vertices(&spun, loft)
        .into_iter()
        .filter(|p| p[2] > 0.5)
        .collect();
    let r = 0.5 * core::f64::consts::SQRT_2;
    for p in &top {
        assert!((p[2] - 1.0).abs() < 1e-12, "{p:?}");
        assert!(
            (p[0].hypot(p[1]) - r).abs() < 1e-12
                && (p[0].abs() < 1e-12 || p[1].abs() < 1e-12),
            "the spun square's corners lie on the axes: {p:?}"
        );
    }
}

/// **A copy slid up its target's own `z` stands up it**: the target's
/// own-xy face is its start cap, which looks DOWN its `z`, so the mate's
/// target plane is slid along that normal by the negated height.
#[test]
fn a_copy_slid_up_its_target_stands_up_it() {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, a) = unit_box(doc);
    let (doc, b) = unit_box(doc);
    let own = vertices(&doc, b);
    let (doc, placed) = fixture::slid(doc, a, b, [0.25, 0.5, 0.75]);
    assert_eq!(vertices(&doc, placed), shifted(&own, [0.25, 0.5, 0.75]));
}

/// **A boolean's copy has its body's reach**: a value on a copy of a
/// union is decided over the union's extent, so a copy of two split
/// halves fused stands where its body stands.
#[test]
fn a_boolean_copy_stands_over_its_bodys_reach() {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, b) = unit_box(doc);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: b.into(),
            tool: fixture::start_cap_at(b, len(0.5)),
        },
    );
    let half = |side: editor_core::SplitHalf| {
        Operand::Var(doc.output(split, side.port()).expect("a split defines its halves"))
    };
    let members = vec![
        BodyRead::plain(half(editor_core::SplitHalf::Above)),
        BodyRead::plain(half(editor_core::SplitHalf::Below)),
    ];
    let (doc, whole) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(members),
        },
    );
    let own = vertices(&doc, whole);
    let (doc, placement) = fixture::place(doc, whole);
    assert_eq!(vertices(&doc, placement), own);
}

/// **A construction is the root of its own space, and no loose copy**:
/// the loose copies an evaluation lists are placements whose bundle pins
/// less than their body needs, never a construction.
#[test]
fn a_construction_is_no_loose_copy() {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, b) = unit_box(doc);
    let (doc, _) = fixture::place(doc, b);
    let (doc, loose) = insert(doc, Node::place(vec![BodyRead::plain(Operand::from(b))]));
    let ev = editor_core::evaluate::<f64>(
        &doc,
        None,
        &editor_core::CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert_eq!(ev.space(b), editor_core::Space::Loose { root: b });
    assert_eq!(ev.loose_roots(), vec![loose]);
}

/// **"Sketch on this face" is one action** (spec §5, DM1): a boss drawn
/// on its own plane, extruded, placed on the box's top face by a mate
/// that names the face, and fused with the box. The boss stands on the
/// face, and raising the box raises it.
#[test]
fn a_sketch_on_a_face_stands_on_the_face_it_names() {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, b) = unit_box(doc);
    let top = fixture::fname(b, editor_core::RoleSeg::Cap(editor_core::CapEnd::End));
    let boss = fixture::desc(vec![fixture::square(0.5, 0.5, 0.25)]);
    let sketched = editor_core::sketch_on_face(
        &doc,
        Operand::from(b),
        top,
        boss,
        len(0.5),
        Formula::zero(editor_core::Dimension::Angle),
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the gesture lands");
    let doc = sketched.doc;
    let union = sketched.sketched.union;
    let top_z = |doc: &ProfileDoc| {
        vertices(doc, union)
            .into_iter()
            .map(|p| p[2])
            .fold(f64::NEG_INFINITY, f64::max)
    };
    assert_eq!(top_z(&doc), 1.5, "the boss stands on the top face");
    let raised = fixture::step(
        doc,
        editor_core::DocEdit::SetParam {
            node: b,
            slot: editor_core::SlotId::Distance,
            value: len(2.0).into(),
            fresh: Vec::new(),
        },
    )
    .0;
    assert_eq!(top_z(&raised), 2.5, "the boss rides up with the face it names");
}
