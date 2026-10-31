//! **INTENT stage 3 PR D — a construction is a root, placed by a mate**
//! (`docs/INTENT-STAGE3-SPEC.md` §5, fork-log rows 159 and 161).
//!
//! A construction computes in its own coordinates and reads no frame.
//! These rows pin the placements §5's migration writes: a copy standing
//! where its body stands (the identity bundle, no mate and the constant
//! `0` on every freedom), a bundle with no mate and its door, the chart
//! a world mate's
//! values are read on (from the world's own frame, so an in-plane
//! offset rides on the values), and a loft's stations.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, ang, insert, len};
use editor_core::Dimension;
use editor_core::pose::PoseDef;
use editor_core::{
    BodyRead, Coord, EvalOptions, Formula, LoftSection, Mate, Node, NodeResult, Operand,
    ProfileDoc, RecipeNodeId, StationMove, TubeWindow, ValuePayload,
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
        ValuePayload::Body(b) => crate::fixture::in_space(b),
        ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. }) => {
            crate::fixture::in_space(body)
        }
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
    let (doc, placement) = insert(doc, crate::fixture::stood(BodyRead::plain(read)));
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

/// **A copy of `box_` standing in the world by `values` and no mate**:
/// a floor a mate reads geometry off, since no space has roles a mate
/// can read (D10 `[placement.mates]`).
fn floor(
    doc: ProfileDoc,
    box_: RecipeNodeId,
    values: Vec<(Coord, Formula)>,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, at) = insert(
        doc,
        crate::fixture::stood(BodyRead::plain(Operand::from(box_))),
    );
    let placed = editor_core::Placed::of(&doc, Vec::new(), values);
    (fixture::written(doc, at, placed), at)
}

fn shifted(points: &[[f64; 3]], by: [f64; 3]) -> Vec<[f64; 3]> {
    points
        .iter()
        .map(|p| [p[0] + by[0], p[1] + by[1], p[2] + by[2]])
        .collect()
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
        let stood = editor_core::stand();
        assert!(
            stood.mates.is_empty(),
            "{body}: the identity bundle has no mate"
        );
        let coords: Vec<Coord> = stood.values.iter().map(|(coord, _)| *coord).collect();
        assert_eq!(
            coords,
            [
                Coord::Slide(X),
                Coord::Slide(Y),
                Coord::Slide(Z),
                Coord::Spin(Some(X)),
                Coord::Spin(Some(Y)),
                Coord::Spin(Some(Z)),
            ],
            "{body}: every freedom is written"
        );
        assert!(
            stood.values.iter().all(|(_, by)| by.is_zero()),
            "{body}: each the constant zero: {:?}",
            stood.values
        );
        let (next, placement) = insert(doc, crate::fixture::stood(BodyRead::plain(read)));
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

/// **`values` written onto `placement` as its whole bundle, with no
/// mate** ([`editor_core::DocEdit::SetConstraints`]).
fn unmated_whole(
    placement: RecipeNodeId,
    values: Vec<(Coord, Formula)>,
) -> editor_core::DocEdit<editor_core::ProfileProgram> {
    editor_core::Placed {
        mates: Vec::new(),
        values,
    }
    .write(placement)
}

/// The unit box's copy placed by `values` and no mate, in the world.
fn unmated(values: Vec<(Coord, Formula)>) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, b) = unit_box(doc);
    let (doc, placement) = insert(
        doc,
        crate::fixture::stood(BodyRead::plain(Operand::from(b))),
    );
    let applied = doc
        .apply(&unmated_whole(placement, values), Tol::witness())
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
        values.push((Coord::Spin(Some(first)), quarter()));
        values.push((Coord::Spin(Some(then)), quarter()));
        values.push((Coord::Spin(Some(first)), Formula::zero(Dimension::Angle)));
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
                .map(|axis| (Coord::Spin(Some(axis)), ang(0.0))),
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
/// unwritten (`Unpinned`); a repeat in a row and a fourth spin fix a
/// freedom twice (`Overconstrained`). Taking a value out writes its
/// freedom back to the constant `0`, and adding a mate keeps the values
/// whose freedoms it leaves and drops the rest, so both leave the copy
/// pinned (`DocEdit::RemoveConstraint`, `DocEdit::AddConstraint`).
///
/// *Breaks if* the door lets a bundle with no mate stand part-written,
/// counts a chain by axis rather than by position, or an increment
/// leaves a bundle unpinned or overconstrained.
#[test]
fn the_door_admits_a_bundle_with_no_mate_only_whole() {
    use editor_core::TargetAxis::{X, Y, Z};
    use editor_core::{DocEdit, EditError, MateFault};
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
                    Coord::Spin(Some(axis)),
                    Formula::zero(editor_core::Dimension::Angle),
                )
            }))
            .collect::<Vec<_>>()
    };
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, b) = unit_box(doc);
    let (doc, base) = insert(
        doc,
        crate::fixture::stood(BodyRead::plain(Operand::from(b))),
    );
    let (doc, p) = insert(
        doc,
        crate::fixture::stood(BodyRead::plain(Operand::from(b))),
    );
    let tol = Tol::witness();
    let one = unmated_whole(p, vec![(Coord::Slide(X), len(1.0))]);
    assert!(
        matches!(
            fault(doc.apply(&one, tol)),
            MateFault::Unpinned { free: 5, .. }
        ),
        "a value alone leaves five freedoms"
    );
    assert!(
        matches!(
            fault(doc.apply(&unmated_whole(p, six(&[X, X, Y])), tol)),
            MateFault::Overconstrained { ref constraints, free: 1, .. } if constraints.len() == 2
        ),
        "two spins in a row about x"
    );
    assert!(
        matches!(
            fault(doc.apply(&unmated_whole(p, six(&[X, Y, Z, X])), tol)),
            MateFault::Overconstrained { ref constraints, .. } if constraints.len() == 4
        ),
        "a fourth spin"
    );
    let stood = doc
        .apply(&unmated_whole(p, six(&[Z, X, Z])), tol)
        .expect("a ZXZ chain writes every freedom");
    let first = stood.record.minted_constraints[0];
    let removed = stood
        .doc
        .apply(
            &DocEdit::RemoveConstraint {
                placement: p,
                constraint: first,
            },
            tol,
        )
        .expect("taking a value out writes its freedom back to 0");
    let held = fixture::bundle(&removed.doc, p);
    assert_eq!(held.len(), 6, "six values still pin the copy: {held:?}");
    assert!(
        !held.iter().any(|c| c.id == first),
        "the value taken out is gone"
    );
    // A mate on the box's start cap to the top of a copy standing in
    // the world: a plane fixes the tilts and the lift, so the chain's
    // tilt and the slide along z go, and the in-plane slides and the
    // first spin about z stay, now on the plane's chart.
    let top = fixture::fname(b, editor_core::RoleSeg::Cap(editor_core::CapEnd::End));
    let mate = Mate {
        on: start_cap(b),
        to: fixture::plane(Operand::select(Operand::output(base, 0), vec![top])),
        orient: editor_core::Orientation::IDENTITY,
    };
    let mated = stood
        .doc
        .apply(
            &DocEdit::AddConstraint {
                placement: p,
                mate: Box::new(mate),
                fresh: Vec::new(),
            },
            tol,
        )
        .expect("a mate added to a pinned bundle keeps the values it leaves");
    let coords: Vec<Coord> = fixture::bundle(&mated.doc, p)
        .iter()
        .filter_map(|c| c.value().map(|v| v.coord))
        .collect();
    assert_eq!(
        coords,
        [Coord::Slide(X), Coord::Slide(Y), Coord::SPIN],
        "the values a plane mate leaves"
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
/// a construction's body is in its own space, the one its `space`
/// output defines, and a copy of it placed into the world is in the
/// world: each derived from the recipe.
#[test]
fn a_construction_is_in_its_own_space() {
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, b) = unit_box(doc);
    let (doc, copy) = fixture::place(doc, b);
    let own = doc.output(b, 1).expect("a construction defines its space");
    assert_eq!(
        doc.var(own).map(editor_core::Var::kind),
        Some(editor_core::VarKind::Space)
    );
    assert_eq!(doc.node_space(b), Some(own));
    assert_eq!(doc.node_space(copy), Some(editor_core::VarId::WORLD));
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
        None,
        &EvalOptions::default(),
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

/// **A copy is placed against a target with no plane of its own** by
/// the moves its author writes, and lands at `at` in the revolve's
/// coordinates.
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
/// its wedge caps stand off its own `xy`) is placed by the moves its
/// author writes.
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
    let (doc, instance) = insert(doc, crate::fixture::mated_instance(post_ref));
    let stood = editor_core::stand();
    assert!(stood.mates.is_empty(), "nothing of the part is read");
    let doc = fixture::written(doc, instance, stood);
    assert!(
        fixture::solve(&doc, &opts, Tol::witness()).at_identity(instance),
        "the instance is placed by no map"
    );
}

/// **A copy mated against a copy already placed charts its values on
/// the target space** (row 190): the second box's start cap rests on the
/// first's top, its slides measured from the world's origin along the
/// world's axes, so where the first was turned and slid moves the second
/// only through the face it rests on. A placed copy's own coordinates
/// are not the chart.
#[test]
fn a_copy_mated_against_a_copy_already_placed_charts_on_the_space() {
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
    // On the first's top, at z = 1, its slides and spin zero on the
    // world's axes: (x, y, z) → (x, y, z + 1).
    let want: Vec<[f64; 3]> = own.iter().map(|p| [p[0], p[1], p[2] + 1.0]).collect();
    assert_near(&vertices(&doc, second), &want);
}

/// Every vertex of the body `node` evaluates to at scalar `T`, each
/// coordinate as its `(lo, hi)` bounds, sorted.
fn vertex_bounds<T: editor_core::EvalScalar>(
    doc: &ProfileDoc,
    node: RecipeNodeId,
) -> Vec<[(f64, f64); 3]> {
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
        ValuePayload::Body(b) => crate::fixture::in_space(b),
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
/// direction by a mated spin, and mated to the start cap of a copy
/// turned about the world's `x` by `turn/4`; each vertex lands on its exact integer coordinates
/// at `f64`, `Interval`, `Dual64` and `Sym`, each coordinate a point
/// with no width. A spin is read through its sine and cosine, taken from
/// the fraction (`geom_core::Angle`), never from the rounded radian,
/// whose cosine is `6.1e-17`.
///
/// An axis mate off the origin turns about its line: the box's edge on
/// the edge of a copy slid to `(1, 2)`, spun `turn/4`. There the copy's
/// measured spin is undone before the written one is made, and at
/// `Interval` an exact zero measured spin is the exact zero, not the
/// backend `atan2`'s dust, whose cosine is an ulp wide and would widen
/// the copy's translation by the line's lever.
///
/// *Breaks if* a rotation by a written fraction of a turn reads the
/// sine and cosine of its value: a coordinate then carries `6.1e-17`
/// at `f64` and an enclosure of that width at `Interval`; or if an
/// exactly zero measured spin is undone through a widened zero.
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
    unmated.push((Coord::Spin(Some(X)), quarter()));
    unmated.push((Coord::Spin(Some(Y)), zero(editor_core::Dimension::Angle)));
    unmated.push((Coord::Spin(Some(Z)), zero(editor_core::Dimension::Angle)));
    let (by_values, p_values) = insert(
        doc.clone(),
        crate::fixture::stood(BodyRead::plain(Operand::from(b))),
    );
    let by_values = by_values
        .apply(&unmated_whole(p_values, unmated), Tol::witness())
        .unwrap_or_else(|refusal| panic!("the values are admitted: {refusal}"))
        .doc;

    let mate = |to: Operand| Mate {
        on: start_cap(b),
        to,
        orient: editor_core::Orientation::IDENTITY,
    };
    let (level, flat, _) = fixture::floor(doc.clone(), Vec::new());
    let (by_spin, p_spin) = placed(
        level,
        b,
        mate(flat),
        vec![
            (Coord::Slide(X), zero(editor_core::Dimension::Length)),
            (Coord::Slide(Y), zero(editor_core::Dimension::Length)),
            (Coord::SPIN, quarter()),
        ],
    );
    let (stood, upended, _) = fixture::floor(doc.clone(), vec![(Coord::Spin(Some(X)), quarter())]);
    let (by_read, p_read) = placed(
        stood,
        b,
        mate(upended),
        vec![
            (Coord::Slide(X), zero(editor_core::Dimension::Length)),
            (Coord::Slide(Y), zero(editor_core::Dimension::Length)),
            (Coord::SPIN, zero(editor_core::Dimension::Angle)),
        ],
    );

    // An axis mate off the origin: the box's lateral edge at its own
    // origin on the world's `z` slid to (1, 2), spun a quarter turn about
    // it, (x, y, z) ↦ (1 − y, 2 + x, z).
    let edge = fixture::axis(Operand::select(
        b,
        vec![fixture::ename(
            b,
            editor_core::RoleSeg::LateralEdge(fixture::vpiece(&doc, b, 0, 0)),
        )],
    ));
    let (lined, post) = floor(
        doc.clone(),
        b,
        vec![(Coord::Slide(X), len(1.0)), (Coord::Slide(Y), len(2.0))],
    );
    let line = fixture::axis(Operand::select(
        Operand::output(post, 0),
        vec![fixture::ename(
            b,
            editor_core::RoleSeg::LateralEdge(fixture::vpiece(&doc, b, 0, 0)),
        )],
    ));
    let (by_axis, p_axis) = placed(
        lined,
        b,
        Mate {
            on: edge,
            to: line,
            orient: editor_core::Orientation::IDENTITY,
        },
        vec![
            (Coord::Slide(Z), zero(editor_core::Dimension::Length)),
            (Coord::SPIN, quarter()),
        ],
    );
    let about_the_line = corners(|[x, y, z]| [1.0 - y, 2.0 + x, z]);
    let rows = [
        ("no mate, spun about x", &by_values, p_values, &upright),
        ("a mated spin", &by_spin, p_spin, &about_z),
        ("a plane spun about x", &by_read, p_read, &upright),
        (
            "an axis mate off the origin",
            &by_axis,
            p_axis,
            &about_the_line,
        ),
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

/// What a run makes of the copy `node`: its vertices' bounds, whether it
/// is placed by a map, and its content key.
fn read_copy<T: editor_core::EvalScalar>(
    ev: &editor_core::Evaluation<T>,
    node: RecipeNodeId,
) -> (Vec<[(f64, f64); 3]>, bool, editor_core::ContentKey) {
    let Some(NodeResult::Ok(value)) = ev.result(node) else {
        panic!("{node} evaluates: {:?}", ev.result(node));
    };
    let bodies = match &value.payload {
        ValuePayload::Body(b) => vec![crate::fixture::in_space(b)],
        other => panic!("{node} is a placement: {other:?}"),
    };
    let mut points: Vec<[(f64, f64); 3]> = bodies
        .iter()
        .flat_map(|body| {
            body.vertices().map(|(v, _)| {
                let p = topo::readback::vertex_point(body, v).expect("a vertex point");
                [p.x, p.y, p.z].map(|c| (c.lo(), c.hi()))
            })
        })
        .collect();
    points.sort_by(|a, b| a.partial_cmp(b).expect("finite"));
    (points, value.motion.is_some(), value.content_key)
}

/// `doc` with the slot `slot` of `node` set to `value`.
fn retyped(
    doc: &ProfileDoc,
    node: RecipeNodeId,
    slot: editor_core::SlotId,
    value: editor_core::SlotValue,
) -> ProfileDoc {
    doc.apply(
        &editor_core::DocEdit::SetParam {
            node,
            slot,
            value,
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .unwrap_or_else(|refusal| panic!("{slot:?} takes the retyped value: {refusal}"))
    .doc
}

/// The memo-primed run of `after` against `before`'s, and a fresh one,
/// agree on the copy `p` at the lane `T`, and the edit moved its key.
fn primed_agrees_with_fresh<T: editor_core::EvalScalar>(
    what: &str,
    before: &ProfileDoc,
    after: &ProfileDoc,
    p: RecipeNodeId,
) -> (Vec<[(f64, f64); 3]>, bool) {
    let run = |doc, prior| {
        editor_core::evaluate::<T>(
            doc,
            prior,
            &editor_core::CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        )
    };
    let prior = run(before, None);
    let primed = read_copy(&run(after, Some(&prior)), p);
    let fresh = read_copy(&run(after, None), p);
    assert_eq!(
        (&primed.0, primed.1),
        (&fresh.0, fresh.1),
        "{what}: the memo-primed run is the fresh one"
    );
    assert_ne!(
        read_copy(&prior, p).2,
        fresh.2,
        "{what}: the retyped form moves the copy's key"
    );
    (fresh.0, fresh.1)
}

/// **A content key keys the form the evaluator reads off the recipe,
/// not only the value's bits** (D8 `[recipe.forms]`). A spin written
/// `turn/4` and one typed `π/2 rad` have the same bits, yet the
/// rotation reads its sine and cosine off the first's fraction; a value
/// written as the constant `0` and one typed `0 rad` have the same bits,
/// yet only the first makes the identity bundle, placed by no map. So
/// retyping either moves the placement's key, and a run primed with the
/// one before (as the viewer primes every edit's) builds what a fresh
/// run builds: at a mateless spin, a mated spin, a spun target's spin,
/// a pose definition's spin on the copy's side, and the identity bundle.
///
/// *Breaks if* a slot's or a pose definition's turn form, or a value's
/// constant-zero form, is left out of the key: the primed run then
/// serves the exact turn's vertices for the typed number, or a copy
/// "placed by no map" for a bundle that is no identity bundle.
#[test]
fn a_retyped_form_moves_the_key_and_the_memo_serves_the_fresh_run() {
    use editor_core::TargetAxis::{X, Y, Z};
    use editor_core::{MateSide, OperandSlot, SlotId, SlotValue};
    let quarter = || Formula::turn_over(4).expect("a quarter turn");
    let zero = |d| Formula::zero(d);
    let typed_quarter = || SlotValue::Formula(ang(core::f64::consts::FRAC_PI_2));
    let doc = ProfileDoc::empty_derived("d", Tol::witness());
    let (doc, b) = unit_box(doc);
    let value_slot = |doc: &ProfileDoc, p, coord| {
        let c = fixture::bundle(doc, p)
            .into_iter()
            .find(|c| c.value().is_some_and(|v| v.coord == coord))
            .expect("the value");
        match coord {
            Coord::Slide(_) => SlotId::ValueSlide { constraint: c.id },
            Coord::SPIN | Coord::Spin(_) => SlotId::ValueSpin { constraint: c.id },
        }
    };
    let mate_id = |doc: &ProfileDoc, p| {
        fixture::bundle(doc, p)
            .into_iter()
            .find(|c| c.value().is_none())
            .expect("the mate")
            .id
    };
    let slides = || {
        vec![
            (Coord::Slide(X), zero(editor_core::Dimension::Length)),
            (Coord::Slide(Y), zero(editor_core::Dimension::Length)),
        ]
    };
    let mate = |on: Operand, to: Operand| Mate {
        on,
        to,
        orient: editor_core::Orientation::IDENTITY,
    };
    let (level, flat, _) = fixture::floor(doc.clone(), Vec::new());
    let mut cases: Vec<(&str, ProfileDoc, ProfileDoc, RecipeNodeId)> = Vec::new();

    // A mateless spin, `turn/4` about the world's `x`.
    let mut values: Vec<(Coord, Formula)> = [X, Y, Z]
        .into_iter()
        .map(|axis| (Coord::Slide(axis), zero(editor_core::Dimension::Length)))
        .collect();
    values.push((Coord::Spin(Some(X)), quarter()));
    values.push((Coord::Spin(Some(Y)), zero(editor_core::Dimension::Angle)));
    values.push((Coord::Spin(Some(Z)), zero(editor_core::Dimension::Angle)));
    let (before, p) = insert(
        doc.clone(),
        crate::fixture::stood(BodyRead::plain(Operand::from(b))),
    );
    let before = before
        .apply(&unmated_whole(p, values), Tol::witness())
        .expect("the values are admitted")
        .doc;
    let slot = value_slot(&before, p, Coord::Spin(Some(X)));
    let after = retyped(&before, p, slot, typed_quarter());
    cases.push(("a mateless spin", before, after, p));

    // A mated spin.
    let mut values = slides();
    values.push((Coord::SPIN, quarter()));
    let (before, p) = placed(level.clone(), b, mate(start_cap(b), flat.clone()), values);
    let slot = value_slot(&before, p, Coord::SPIN);
    let after = retyped(&before, p, slot, typed_quarter());
    cases.push(("a mated spin", before, after, p));

    // A plane read off a copy spun `turn/4` about the world's `x`: the
    // copy's spin retyped moves the mated copy's key.
    let mut values = slides();
    values.push((Coord::SPIN, zero(editor_core::Dimension::Angle)));
    let (stood, upended_top, upended) =
        fixture::floor(doc.clone(), vec![(Coord::Spin(Some(X)), quarter())]);
    let (before, p) = placed(stood, b, mate(start_cap(b), upended_top), values);
    let slot = value_slot(&before, upended, Coord::Spin(Some(X)));
    let after = retyped(&before, upended, slot, typed_quarter());
    cases.push(("a spun target's spin", before, after, p));

    // The copy's side read through a pose definition spun a sixth of a
    // turn about the rim of the box's start cap (a quarter turn's copy lands on
    // the world's axes either way, which would hide a stale memo).
    let edge = || {
        fixture::axis(Operand::select(
            b,
            vec![fixture::rim_edge(
                b,
                editor_core::CapEnd::Start,
                fixture::piece(&doc, b, 0, 0),
            )],
        ))
    };
    let spun = |by: Formula| {
        Operand::Pose(Box::new(PoseDef::Spin {
            pose: editor_core::pose::RoleRead::plain(start_cap(b)),
            about: editor_core::pose::RoleRead::plain(edge()),
            by,
        }))
    };
    let mut values = slides();
    values.push((Coord::SPIN, zero(editor_core::Dimension::Angle)));
    let (before, p) = placed(
        level,
        b,
        mate(
            spun(Formula::turn_over(6).expect("a sixth of a turn")),
            flat,
        ),
        values,
    );
    let slot = SlotId::Operand(OperandSlot::Side {
        constraint: mate_id(&before, p),
        side: MateSide::On,
    });
    let after = retyped(
        &before,
        p,
        slot,
        SlotValue::Read(spun(ang(core::f64::consts::TAU / 6.0))),
    );
    cases.push(("a pose definition's spin", before, after, p));

    for (what, before, after, p) in &cases {
        let (exact, _) = primed_agrees_with_fresh::<f64>(what, after, before, *p);
        let (typed, _) = primed_agrees_with_fresh::<f64>(what, before, after, *p);
        assert_ne!(
            exact, typed,
            "{what}: the two spellings build apart, so the row can see a stale memo"
        );
        primed_agrees_with_fresh::<geom_core::Interval>(what, before, after, *p);
    }

    // The identity bundle with one spin retyped to a typed `0 rad`: no
    // longer the identity bundle, so placed by a map (its turn form keys
    // it apart).
    let read = fixture::body_read(&doc, b.into());
    let stood = editor_core::stand();
    let (next, p) = insert(doc.clone(), crate::fixture::stood(BodyRead::plain(read)));
    let before = fixture::written(next, p, stood);
    let slot = value_slot(&before, p, Coord::Spin(Some(Z)));
    let after = retyped(&before, p, slot, SlotValue::Formula(ang(0.0)));
    let (_, mapped) = primed_agrees_with_fresh::<f64>("the identity bundle", &before, &after, p);
    assert!(mapped, "a typed `0 rad` is placed by its map");
    let (_, mapped) = primed_agrees_with_fresh::<f64>("the identity bundle", &after, &before, p);
    assert!(!mapped, "the identity bundle is placed by no map");
    primed_agrees_with_fresh::<geom_core::Interval>("the identity bundle", &before, &after, p);
    // A slide retyped to a typed `0 mm`: a length has no turn form, so
    // only the constant-zero form tells the two apart.
    let slot = value_slot(&before, p, Coord::Slide(Y));
    let after = retyped(&before, p, slot, SlotValue::Formula(len(0.0)));
    let (_, mapped) =
        primed_agrees_with_fresh::<f64>("the identity bundle's slide", &before, &after, p);
    assert!(mapped, "a typed `0 mm` is placed by its map");
}
