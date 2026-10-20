//! **INTENT stage 3 PR D — a construction is a root, placed by a mate**
//! (`docs/INTENT-STAGE3-SPEC.md` §5, fork-log rows 159 and 161).
//!
//! A construction computes in its own coordinates and reads no frame.
//! These rows pin the placements §5's migration writes: a copy standing
//! where its body stands (the identity bundle, no mate and the constant
//! `0` on every freedom), a bundle with no mate and its door, the chart
//! a world mate's
//! values are read on (from the world's own frame, so a datum's in-plane
//! offset rides on the values), and a loft's stations.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, ang, insert, len};
use editor_core::pose::{PoseDef, PoseRole};
use editor_core::{
    BodyRead, Coord, EvalOptions, Formula, LoftSection, Mate, Node, NodeResult, Operand,
    ProfileDoc, RecipeNodeId, StationMove, Target, TargetAxis, TubeWindow, ValuePayload, WorldRead,
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

/// Every point of `want` has one of `got` within rounding, and the two
/// hold as many: a placed copy's vertices, read where the motion's
/// rounding puts them.
fn assert_near(got: &[[f64; 3]], want: &[[f64; 3]]) {
    assert_eq!(got.len(), want.len(), "{got:?} vs {want:?}");
    for q in want {
        assert!(
            got.iter()
                .any(|p| (0..3).all(|k| (p[k] - q[k]).abs() < 1e-12)),
            "{q:?} is not among {got:?}"
        );
    }
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
            (Coord::Slide(TargetAxis::X), len(x)),
            (Coord::Slide(TargetAxis::Y), len(y)),
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
            Coord::Slide(TargetAxis::X),
            Formula::zero(editor_core::Dimension::Length),
        ),
        (
            Coord::Slide(TargetAxis::Y),
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

/// **Every copy standing where its body stands is the identity
/// bundle** ([`editor_core::stand`], D10 `[placement.values]`): no mate,
/// and the constant `0` on the three slides and the three spins, so an
/// extrude, a full revolve whose axis is written in numbers (`0 m`, a
/// typed value a mate could not share), and a full ring each stand as
/// their bodies do, bit for bit, and the solve reads each placement off
/// its recipe as at the identity.
///
/// *Breaks if* `stand` reads the copy's geometry again (a mate, an
/// anchor face, an axis), writes a value that is not the constant `0`,
/// or the fold places the identity bundle by a map.
#[test]
fn every_copy_stands_by_the_identity_bundle() {
    use editor_core::TargetAxis::{X, Y, Z};
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, b) = unit_box(doc);
    let (doc, profile) = insert(
        doc,
        Node::Profile(fixture::desc(vec![vec![
            (0.0, -1.5),
            (1.0, -1.5),
            (1.0, -1.0),
            (0.0, -1.0),
        ]])),
    );
    let mut revolve = fixture::revolve_about(
        profile,
        (0.0, 0.0),
        fixture::along_x(),
        Formula::turn_over(1).expect("a turn"),
    );
    if let Node::Revolve { axis_through, .. } = &mut revolve {
        *axis_through = [len(0.0), len(0.0)];
    }
    let (doc, rev) = insert(doc, revolve);
    let (doc, ring) = insert(
        doc,
        Node::Tube {
            major_radius: len(2.0),
            window: TubeWindow::Full,
            minor_radius: len(0.5),
        },
    );
    let mut doc = doc;
    for body in [b, rev, ring] {
        let own = vertices(&doc, body);
        let read = fixture::body_read(&doc, body.into());
        let stood = editor_core::stand(&doc, &read, &EvalOptions::default(), Tol::witness())
            .unwrap_or_else(|refusal| panic!("{body} stands: {refusal}"));
        assert_eq!(stood.by, editor_core::PlacedBy::Identity, "{body}");
        assert_eq!(stood.mate, None, "{body}: the identity bundle has no mate");
        let coords: Vec<Coord> = stood.values.iter().map(|(coord, _)| *coord).collect();
        assert_eq!(
            coords,
            [
                Coord::Slide(X),
                Coord::Slide(Y),
                Coord::Slide(Z),
                Coord::SpinAbout(X),
                Coord::SpinAbout(Y),
                Coord::SpinAbout(Z),
            ],
            "{body}: every freedom is written"
        );
        assert!(
            stood.values.iter().all(|(_, by)| by.is_zero()),
            "{body}: each the constant zero: {:?}",
            stood.values
        );
        let (next, placement) = insert(doc, Node::place(vec![BodyRead::plain(read)]));
        doc = fixture::written(next, placement, stood);
        assert_eq!(
            vertices(&doc, placement),
            own,
            "{body}: the copy is its body"
        );
        assert!(
            fixture::solve(&doc, &EvalOptions::default(), Tol::witness()).at_identity(placement),
            "{body}: the identity bundle is read as the identity"
        );
    }
}

/// The unit box's copy placed by `values` and no mate, in the world.
fn unmated(values: Vec<(Coord, Formula)>) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, b) = unit_box(doc);
    let (doc, placement) = insert(doc, Node::place(vec![BodyRead::plain(Operand::from(b))]));
    let applied = doc
        .apply(
            &editor_core::DocEdit::unmated(placement, values),
            Tol::witness(),
        )
        .unwrap_or_else(|refusal| panic!("the values are admitted: {refusal}"));
    (applied.doc, b, placement)
}

/// **A bundle with no mate places its copy by its values, the spins in
/// the order written** (D10 `[placement.values]`): each spin turns the
/// copy about the world's own axis, the first written first, and the
/// slides set its origin. A quarter turn about `z` then one about `x`
/// carries the box's `(0, 1, 0)` corner to `(−1, 0, 0)`; written the
/// other way round, to `(0, 0, 1)`. Typed zeros place the copy where it
/// stands, yet the bundle is no identity bundle: what is placed by no
/// map is read off the recipe, not off the map's value.
///
/// *Breaks if* the chain is applied in an order the kernel chooses,
/// about the copy's axes rather than the world's, or a bundle is read
/// as the identity from its values.
#[test]
fn a_bundle_with_no_mate_places_by_its_chain_in_written_order() {
    use editor_core::TargetAxis::{X, Y, Z};
    let quarter = || Formula::turn_over(4).expect("a quarter turn");
    let slides = || {
        vec![
            (Coord::Slide(X), len(1.0)),
            (Coord::Slide(Y), len(2.0)),
            (Coord::Slide(Z), len(3.0)),
        ]
    };
    let turned = |first: editor_core::TargetAxis, then: editor_core::TargetAxis| {
        let mut values = slides();
        values.push((Coord::SpinAbout(first), quarter()));
        values.push((Coord::SpinAbout(then), quarter()));
        values.push((Coord::SpinAbout(first), ang(0.0)));
        values
    };
    // R = Rx·Rz: (x, y, z) ↦ (−y, −z, x), then slid by (1, 2, 3).
    let (doc, b, p) = unmated(turned(Z, X));
    let zx: Vec<[f64; 3]> = vertices(&doc, b)
        .iter()
        .map(|&[x, y, z]| [1.0 - y, 2.0 - z, 3.0 + x])
        .collect();
    assert_near(&vertices(&doc, p), &zx);
    // R = Rz·Rx: (x, y, z) ↦ (z, x, y), then slid.
    let (doc, b, p) = unmated(turned(X, Z));
    let xz: Vec<[f64; 3]> = vertices(&doc, b)
        .iter()
        .map(|&[x, y, z]| [1.0 + z, 2.0 + x, 3.0 + y])
        .collect();
    assert_near(&vertices(&doc, p), &xz);
    assert!(
        !fixture::solve(&doc, &EvalOptions::default(), Tol::witness()).at_identity(p),
        "a turned copy is not at the identity"
    );
    let typed = [X, Y, Z]
        .into_iter()
        .map(|axis| (Coord::Slide(axis), len(0.0)))
        .chain(
            [X, Y, Z]
                .into_iter()
                .map(|axis| (Coord::SpinAbout(axis), ang(0.0))),
        )
        .collect();
    let (doc, b, p) = unmated(typed);
    assert_eq!(
        vertices(&doc, p),
        vertices(&doc, b),
        "typed zeros move nothing"
    );
    assert!(
        !fixture::solve(&doc, &EvalOptions::default(), Tol::witness()).at_identity(p),
        "typed zeros are numbers, so the bundle is no identity bundle and runs its map"
    );
}

/// **The door admits a bundle with no mate only whole** (D10
/// `[placement.placement]`, `[placement.values]`): with no mate, its
/// values state the whole motion, so a value alone leaves freedoms
/// unwritten (`Unpinned`), and so does taking one out; a repeat in a
/// row, a fourth spin and a mate beside the six values fix a freedom
/// twice (`Overconstrained`); a spin about a mated direction has no
/// direction with no mate, and a value naming no mate beside one has no
/// chart (`NoChart`).
///
/// *Breaks if* the door lets a bundle with no mate stand part-written,
/// counts a chain by axis rather than by position, or charts an unmated
/// value beside a mate.
#[test]
fn the_door_admits_a_bundle_with_no_mate_only_whole() {
    use editor_core::TargetAxis::{X, Y, Z};
    use editor_core::{DocEdit, EditError, MateFault, NoChartWhy};
    let fault =
        |result: Result<editor_core::Applied<editor_core::ProfileProgram>, EditError>| match result
        {
            Err(EditError::MateRefused { fault, .. }) => *fault,
            other => panic!("the door refuses with a mate fault: {other:?}"),
        };
    let six = |spins: &[editor_core::TargetAxis]| {
        [X, Y, Z]
            .into_iter()
            .map(|axis| {
                (
                    Coord::Slide(axis),
                    Formula::zero(editor_core::Dimension::Length),
                )
            })
            .chain(spins.iter().map(|&axis| {
                (
                    Coord::SpinAbout(axis),
                    Formula::zero(editor_core::Dimension::Angle),
                )
            }))
            .collect::<Vec<_>>()
    };
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, b) = unit_box(doc);
    let (doc, p) = insert(doc, Node::place(vec![BodyRead::plain(Operand::from(b))]));
    let tol = Tol::witness();
    let one = DocEdit::AddValue {
        placement: p,
        value: Box::new(editor_core::Value {
            mate: None,
            coord: Coord::Slide(X),
            by: len(1.0),
        }),
        fresh: Vec::new(),
    };
    assert!(
        matches!(
            fault(doc.apply(&one, tol)),
            MateFault::Unpinned { free: 5, .. }
        ),
        "a value alone leaves five freedoms"
    );
    assert!(
        matches!(
            fault(doc.apply(&DocEdit::unmated(p, six(&[X, X, Y])), tol)),
            MateFault::Overconstrained { ref constraints, free: 1, .. } if constraints.len() == 2
        ),
        "two spins in a row about x"
    );
    assert!(
        matches!(
            fault(doc.apply(&DocEdit::unmated(p, six(&[X, Y, Z, X])), tol)),
            MateFault::Overconstrained { ref constraints, .. } if constraints.len() == 4
        ),
        "a fourth spin"
    );
    let mut spun = six(&[X, Y, Z]);
    spun[3] = (Coord::Spin, Formula::zero(editor_core::Dimension::Angle));
    assert!(
        matches!(
            fault(doc.apply(&DocEdit::unmated(p, spun), tol)),
            MateFault::NoChart {
                why: NoChartWhy::NoMatedDirection,
                ..
            }
        ),
        "a spin about a mated direction, with no mate"
    );
    let stood = doc
        .apply(&DocEdit::unmated(p, six(&[Z, X, Z])), tol)
        .expect("a ZXZ chain writes every freedom");
    let first = stood.record.minted_constraints[0];
    assert!(
        matches!(
            fault(stood.doc.apply(
                &DocEdit::RemoveConstraint {
                    placement: p,
                    constraint: first,
                },
                tol
            )),
            MateFault::Unpinned { free: 1, .. }
        ),
        "taking a value out leaves a freedom"
    );
    let mate = Mate {
        on: start_cap(b),
        to: Target::World(WorldRead::through(PoseRole::Xy)),
        class: editor_core::ContactClass::Rest,
    };
    assert!(
        matches!(
            fault(stood.doc.apply(
                &DocEdit::AddConstraint {
                    placement: p,
                    mate: Box::new(mate.clone()),
                    fresh: Vec::new(),
                },
                tol
            )),
            MateFault::Overconstrained { .. }
        ),
        "a mate beside the six values fixes what they fix"
    );
    let (mated, _) = fixture::constrain_at(doc, p, mate);
    assert!(
        matches!(
            fault(mated.apply(&one, tol)),
            MateFault::NoChart {
                why: NoChartWhy::BesideAMate,
                ..
            }
        ),
        "a value naming no mate, beside a mate"
    );
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
        .flat_map(|z| [(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)].map(|(x, y)| [x, y, z]))
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
            (p[0].hypot(p[1]) - r).abs() < 1e-12 && (p[0].abs() < 1e-12 || p[1].abs() < 1e-12),
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
        Operand::Var(
            doc.output(split, side.port())
                .expect("a split defines its halves"),
        )
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
    assert_eq!(
        top_z(&raised),
        2.5,
        "the boss rides up with the face it names"
    );
}

/// A full revolve about its own y, the bore of a ring: no face of it lies
/// on its own `xy`.
fn full_revolve(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = insert(
        doc,
        Node::Profile(fixture::desc(vec![vec![
            (1.0, 0.0),
            (1.5, 0.0),
            (1.5, 1.0),
            (1.0, 1.0),
        ]])),
    );
    insert(
        doc,
        fixture::revolve_about(
            profile,
            (0.0, 0.0),
            fixture::along_y(),
            Formula::turn_over(1).expect("a turn"),
        ),
    )
}

/// **A copy is placed against a target with no plane of its own**: a
/// frame read off the revolve's geometry stands where the copy's start
/// cap does, and the copy lands at `at` in the revolve's coordinates.
#[test]
fn a_copy_stands_against_a_plane_less_target() {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, rev) = full_revolve(doc);
    let (doc, b) = unit_box(doc);
    let own = vertices(&doc, b);
    let (doc, placed) = fixture::slid(doc, Operand::output(rev, 0), b, [2.0, 0.5, -0.25]);
    assert_near(&vertices(&doc, placed), &shifted(&own, [2.0, 0.5, -0.25]));
}

/// **A copy with neither a plane nor an axis of its own** (an arc tube:
/// its wedge caps stand off its own `xy`) is placed by a frame read off
/// it, equated with one read off its target.
#[test]
fn an_arc_tube_stands_against_its_target_by_frames() {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, b) = unit_box(doc);
    let (doc, bend) = insert(
        doc,
        Node::Tube {
            major_radius: len(2.0),
            window: TubeWindow::Arc {
                t0: ang(0.25),
                t1: ang(1.5),
            },
            minor_radius: len(0.25),
        },
    );
    let own = vertices(&doc, bend);
    let (doc, placed) = fixture::slid(doc, b, bend, [0.5, 0.5, 3.0]);
    assert_near(&vertices(&doc, placed), &shifted(&own, [0.5, 0.5, 3.0]));
}

/// **An instance stands where its part's world stands**: by the
/// identity bundle, which reads nothing of the part, so the instance's
/// copies stand where the part's product does at every value of the
/// part's variables.
///
/// *Breaks if* `stand` reads the part's geometry (a face of the part's
/// product, which a part grown taller would move).
#[test]
fn an_instance_stands_by_the_identity_bundle() {
    use fixture::resolver::{PartStore, with_resolver};
    let mut store = PartStore::new();
    let part = ProfileDoc::empty(editor_core::DocumentId::derive("d-post"), Tol::witness());
    let (part, post) = unit_box(part);
    let (post_ref, _) = store.insert_part((part, post), Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(editor_core::DocumentId::derive("d-host"), Tol::witness());
    let (doc, instance) = insert(doc, Node::instantiate_part(post_ref));
    let copies = Operand::Var(
        doc.output(instance, 0)
            .expect("an instance defines its copies"),
    );
    let stood =
        editor_core::stand(&doc, &copies, &opts, Tol::witness()).expect("the instance stands");
    assert_eq!(stood.by, editor_core::PlacedBy::Identity);
    assert_eq!(stood.mate, None, "nothing of the part is read");
    let doc = fixture::written(doc, instance, stood);
    assert!(
        fixture::solve(&doc, &opts, Tol::witness()).at_identity(instance),
        "the instance is placed by no map"
    );
}

/// **A copy stands against a copy already placed**, at a frame in the
/// placed copy's own coordinates: the target is read where its
/// placement moved it, the frame carried there with it, so the second
/// box sits on the first's top wherever the first was turned and slid.
/// Placing the placed copy itself again still refuses: a copy is
/// defined by its one placement.
#[test]
fn a_copy_stands_against_a_copy_already_placed() {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, a) = unit_box(doc);
    let turned = editor_core::Frame::rotate_then_translate(
        [0.0, 0.0, 1.0],
        std::f64::consts::FRAC_PI_2,
        [5.0, 0.0, 0.0],
        fixture::band(),
    )
    .expect("a quarter turn about z");
    let (doc, first) = fixture::in_world(doc, a, &turned);
    let (doc, b) = unit_box(doc);
    let own = vertices(&doc, b);
    let (doc, second) = fixture::slid(doc, first, b, [0.0, 0.0, 1.0]);
    // The first box's motion, applied to the second's own vertices one
    // up its z: (x, y, z) → (5 − y, x, z + 1).
    let want: Vec<[f64; 3]> = own.iter().map(|p| [5.0 - p[1], p[0], p[2] + 1.0]).collect();
    assert_near(&vertices(&doc, second), &want);
    let again = editor_core::place_against(
        &doc,
        &Operand::from(b),
        &Operand::from(first),
        &editor_core::Frame::IDENTITY,
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert!(
        matches!(again, Err(editor_core::PinRefusal::AlreadyPlaced { .. })),
        "a placed copy is never placed again: {:?}",
        again.map(|p| p.by)
    );
}

/// Every vertex of the body `node` evaluates to at scalar `T`, each
/// coordinate as its `(lo, hi)` bounds, sorted.
fn vertex_bounds<T: editor_core::EvalScalar>(
    doc: &ProfileDoc,
    node: RecipeNodeId,
) -> Vec<[(f64, f64); 3]> {
    use geom_core::Bounds;
    let ev = editor_core::evaluate::<T>(
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
        ValuePayload::Copies(copies) if copies.bodies.len() == 1 => copies.bodies[0].clone(),
        other => panic!("{node} is a body: {other:?}"),
    };
    let mut points: Vec<[(f64, f64); 3]> = body
        .vertices()
        .map(|(v, _)| {
            let p = topo::readback::vertex_point(&body, v).expect("a vertex point");
            [p.x, p.y, p.z].map(|c| (c.lo(), c.hi()))
        })
        .collect();
    points.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    points
}

/// **A block placed upright by a written `turn/4` stands on the world's
/// planes bit for bit, in every lane** (D10 `[variables.literals]`: a
/// rational fraction of a turn is a constant; D8, the recipe keeps the
/// form written). The unit box is turned a quarter turn about the
/// world's `x` by a bundle with no mate, about its plane mate's
/// direction by a mated spin, and about the world's `x` by a world read
/// spun by `turn/4`; each vertex lands on its exact integer coordinates
/// at `f64`, `Interval`, `Dual64` and `Sym`, each coordinate a point
/// with no width. A spin is read through its sine and cosine, taken from
/// the fraction (`geom_core::Angle`), never from the rounded radian,
/// whose cosine is `6.1e-17`.
///
/// *Breaks if* a rotation by a written fraction of a turn reads the
/// sine and cosine of its value: a coordinate then carries `6.1e-17`
/// at `f64` and an enclosure of that width at `Interval`.
#[test]
fn a_block_placed_upright_by_a_written_quarter_turn_stands_on_the_worlds_planes() {
    use editor_core::TargetAxis::{X, Y, Z};
    let quarter = || Formula::turn_over(4).expect("a quarter turn");
    let zero = |d| Formula::zero(d);
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, b) = unit_box(doc);
    let corners = |f: fn([f64; 3]) -> [f64; 3]| {
        let mut want: Vec<[(f64, f64); 3]> = vertices(&doc, b)
            .into_iter()
            .map(|p| f(p).map(|c| (c + 0.0, c + 0.0)))
            .collect();
        want.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
        want
    };
    // Upright: a quarter turn about x, (x, y, z) ↦ (x, −z, y).
    let upright = corners(|[x, y, z]| [x, -z, y]);
    // A quarter turn about z, (x, y, z) ↦ (−y, x, z).
    let about_z = corners(|[x, y, z]| [-y, x, z]);

    let mut unmated = [X, Y, Z]
        .into_iter()
        .map(|axis| (Coord::Slide(axis), zero(editor_core::Dimension::Length)))
        .collect::<Vec<_>>();
    unmated.push((Coord::SpinAbout(X), quarter()));
    unmated.push((Coord::SpinAbout(Y), zero(editor_core::Dimension::Angle)));
    unmated.push((Coord::SpinAbout(Z), zero(editor_core::Dimension::Angle)));
    let (by_values, p_values) = insert(
        doc.clone(),
        Node::place(vec![BodyRead::plain(Operand::from(b))]),
    );
    let by_values = by_values
        .apply(
            &editor_core::DocEdit::unmated(p_values, unmated),
            Tol::witness(),
        )
        .unwrap_or_else(|refusal| panic!("the values are admitted: {refusal}"))
        .doc;

    let mate = |to: WorldRead<Formula>| Mate {
        on: start_cap(b),
        to: Target::World(to),
        class: editor_core::ContactClass::Rest,
    };
    let (by_spin, p_spin) = placed(
        doc.clone(),
        b,
        mate(WorldRead::through(PoseRole::Xy)),
        vec![
            (Coord::Slide(X), zero(editor_core::Dimension::Length)),
            (Coord::Slide(Y), zero(editor_core::Dimension::Length)),
            (Coord::Spin, quarter()),
        ],
    );
    let stood = WorldRead::through(PoseRole::Xy).spin(WorldRead::through(PoseRole::X), quarter());
    let (by_read, p_read) = placed(
        doc,
        b,
        mate(stood),
        vec![
            (Coord::Slide(X), zero(editor_core::Dimension::Length)),
            (Coord::Slide(Y), zero(editor_core::Dimension::Length)),
            (Coord::Spin, zero(editor_core::Dimension::Angle)),
        ],
    );

    let rows = [
        ("no mate, spun about x", &by_values, p_values, &upright),
        ("a mated spin", &by_spin, p_spin, &about_z),
        ("a world read spun about x", &by_read, p_read, &upright),
    ];
    for (what, doc, p, want) in rows {
        assert_eq!(&vertex_bounds::<f64>(doc, p), want, "{what} at f64");
        assert_eq!(
            &vertex_bounds::<geom_core::Interval>(doc, p),
            want,
            "{what} at Interval"
        );
        assert_eq!(
            &vertex_bounds::<geom_core::Dual64>(doc, p),
            want,
            "{what} at Dual64"
        );
        assert_eq!(
            &vertex_bounds::<geom_core::Sym<f64>>(doc, p),
            want,
            "{what} at Sym"
        );
    }
}
