//! **A sphere cut in a cap names its lens's entities.** Two revolved
//! balls, the unit ball and radius 0.8 at height 1.4, intersect in a
//! lens; a 0.4 × 0.4 × 0.3 brick, standing turned so its near face lies 0.985 from
//! the origin toward latitude 68°, azimuth 50°, meets the lens's top
//! face in a circle no edge reaches. The kernel cuts that face along its
//! chart's meridian through the circle before the op runs
//! (`topo::boolean::ops::apply_cut_ins`): the cut's new piece descends
//! from the face it divided, and the cut itself is no edge of the lens.
//!
//! The output stage closes the cut again (`topo::boolean::slit`, closing
//! a cut), so every op builds, and the circle keeps one vertex with no
//! identity of its own, named for the circle.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{failure, run};
use crate::fixture::{against, ang, axes_at, insert, len, len2, on_frame, scl};
use editor_core::{
    LoopProgram, Node, ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget,
    RecipeNodeId,
};
use geom_core::Tol;

/// A ball of radius `r` centred `c` up the y axis: a bulge-1 semicircle
/// revolved a full turn about y.
pub(crate) fn ball(doc: ProfileDoc, r: f64, c: f64) -> (ProfileDoc, RecipeNodeId) {
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
        doc,
        crate::fixture::revolve_about(
            p,
            (0.0, 0.0),
            crate::fixture::along_y(),
            ang(std::f64::consts::TAU),
        ),
    )
}

/// The unit direction at latitude `lat` (from the xz plane toward +y)
/// and azimuth `az` (from +x toward +z), in degrees.
pub(crate) fn dir(lat: f64, az: f64) -> [f64; 3] {
    let (lat, az) = (lat.to_radians(), az.to_radians());
    [lat.cos() * az.cos(), lat.sin(), lat.cos() * az.sin()]
}

/// `a × b`.
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// A `2w × 2w × t` brick on its own xy, centred on its own origin, and
/// the frame it stands on in the world: its near face in the plane
/// `n·p = s`, centred on its foot, its material beyond the plane — `y`
/// turned onto `n` about `y × n`, as `sweep`'s `snowman` rows turn
/// theirs. The brick is the root a combine with it is spelled in, and
/// [`world_in_brick`] places a world-drawn body beside it.
pub(crate) fn brick_toward(
    doc: ProfileDoc,
    n: [f64; 3],
    s: f64,
    w: f64,
    t: f64,
) -> (ProfileDoc, RecipeNodeId, editor_core::Frame) {
    let k = cross([0.0, 1.0, 0.0], n);
    let kn = k.iter().map(|x| x * x).sum::<f64>().sqrt();
    let k = k.map(|x| x / kn);
    let th = kn.atan2(n[1]);
    let rot = |v: [f64; 3]| {
        let (s_, c) = th.sin_cos();
        let kv = cross(k, v);
        let kd: f64 = k.iter().zip(v).map(|(a, b)| a * b).sum();
        [0, 1, 2].map(|i| v[i] * c + kv[i] * s_ + k[i] * kd * (1.0 - c))
    };
    let (u, v) = (rot([0.0, 0.0, 1.0]), rot([1.0, 0.0, 0.0]));
    let (doc, p) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(-w, -w), (w, -w), (w, w), (-w, w)]],
    );
    let (doc, brick) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(t),
        },
    );
    (doc, brick, axes_at(n.map(|x| x * s), u, v))
}

/// **`body`, drawn in world coordinates, placed against the `brick`
/// standing on `stands`**: its copy in the brick's own coordinates.
pub(crate) fn world_in_brick(
    doc: ProfileDoc,
    brick: RecipeNodeId,
    stands: &editor_core::Frame,
    body: editor_core::Operand,
) -> (ProfileDoc, RecipeNodeId) {
    let at = editor_core::Frame::from_affine(stands.affine().inverse());
    against(doc, brick, body, &at)
}

/// The document: the lens, the brick, and `op` over them, lens first or
/// not.
fn document(op: &str, lens_first: bool) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let doc = ProfileDoc::empty_derived("sphere_cut_in_cap", Tol::witness());
    let (doc, b1) = ball(doc, 1.0, 0.0);
    let (doc, b2) = ball(doc, 0.8, 1.4);
    let (doc, brick, stands) = brick_toward(doc, dir(68.0, 50.0), 0.985, 0.2, 0.3);
    let (doc, b1) = world_in_brick(doc, brick, &stands, editor_core::Operand::output(b1, 0));
    let (doc, b2) = world_in_brick(doc, brick, &stands, editor_core::Operand::output(b2, 0));
    let (doc, lens) = insert(
        doc,
        Node::Intersect {
            members: editor_core::Bodies::spelled(vec![b1.into(), b2.into()]),
        },
    );
    let (x, y) = if lens_first {
        (lens, brick)
    } else {
        (brick, lens)
    };
    let node = match op {
        "subtract" => Node::Subtract {
            from: x.into(),
            tool: y.into(),
        },
        "union" => Node::Union {
            members: editor_core::Bodies::spelled(vec![x.into(), y.into()]),
        },
        _ => Node::Intersect {
            members: editor_core::Bodies::spelled(vec![x.into(), y.into()]),
        },
    };
    let (doc, out) = insert(doc, node);
    (doc, [lens, brick, out])
}

/// **Every op over the lens and the brick names**, in both orders. The
/// circle the brick's near face meets the lens's top face in keeps one
/// conventional vertex, named for the circle alone: a seam between a
/// face of the lens and one of the brick, so its name cites both
/// operands and nothing else of the op (`names/README.md`, "A
/// conventional vertex").
#[test]
fn every_op_over_a_sphere_cut_in_names_the_circles_vertex_for_the_circle() {
    for (op, lens_first) in [
        ("intersect", true),
        ("intersect", false),
        ("subtract", false),
        ("union", true),
        ("union", false),
        ("subtract", true),
    ] {
        let what = format!("{op}, lens first: {lens_first}");
        let (doc, [lens, brick, out]) = document(op, lens_first);
        let ev = run(&doc);
        assert!(failure(&ev, lens).is_none(), "{what}: the lens names");
        assert!(failure(&ev, brick).is_none(), "{what}: the brick names");
        if let Some(e) = failure(&ev, out) {
            panic!("{what}: the op names: {e}");
        }
        circle_vertex_cites(&doc, &ev, out, [lens, brick], &what);
    }
}

/// `out`'s table names one conventional vertex, for a seam edge whose
/// name derives from both of `operands`, the nodes minting the names of
/// the faces it runs between; the vertex's name, and a pick at the
/// vertex, resolve to that edge.
pub(crate) fn circle_vertex_cites(
    doc: &ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
    out: RecipeNodeId,
    operands: [RecipeNodeId; 2],
    what: &str,
) {
    use editor_core::{EntityKey, Entry, RoleSeg};
    let t = crate::fixture::table(ev, out);
    let vertices: Vec<_> = t
        .iter()
        .filter_map(|(n, entry)| match (n.path.as_slice(), entry) {
            ([RoleSeg::ConventionalVertex(edge)], Entry::Unique(v)) => {
                Some((n.clone(), edge.clone(), v.key))
            }
            _ => None,
        })
        .collect();
    let [(name, edge, EntityKey::Vertex(vertex))] = vertices.as_slice() else {
        panic!("{what}: one conventional vertex, not {vertices:?}");
    };
    let Some(Entry::Unique(on)) = t.lookup(edge) else {
        panic!("{what}: the vertex's edge is one row of the table: {edge:?}");
    };
    let from = editor_core::derivation_nodes(edge);
    assert!(
        matches!(edge.path.as_slice(), [RoleSeg::Seam { .. }])
            && operands.iter().all(|o| from.contains(o)),
        "{what}: the circle is the seam between the two operands: {edge:?}"
    );
    match crate::fixture::resolve_at_mint(editor_core::RunCtx { doc, eval: ev }, name) {
        editor_core::Resolution::Resolved(r) => assert_eq!(
            r.entity.key, on.key,
            "{what}: the vertex's name resolves to its edge"
        ),
        other => panic!("{what}: the vertex's name resolves, not {other:?}"),
    }
    assert_eq!(
        editor_core::vertex_name(ev, out, 0, *vertex).expect("a pick at the vertex names"),
        &**edge,
        "{what}: a pick at the vertex is a pick of its edge"
    );
}

/// **The cap states its meridian crossings as meetings**: the kernel
/// cuts the lens's top face along a meridian before the op, and the
/// cap decides each point where the section circle crosses that
/// meridian on the brick's near face. Read back to the lens, the point
/// is the face the meridian runs in, so the row is the lens's face and
/// the brick's sharing that point (`Relation::Meet`), never the lens's
/// face lying on the brick's. Every other row names cells of its inputs
/// that hold the point decided: a vertex on a face lies on that face's
/// surface. Red if a crossing is published face on face, or dropped.
#[test]
fn the_cap_states_its_meridian_crossings_as_meetings() {
    for lens_first in [true, false] {
        let (doc, [lens, brick, _]) = document("intersect", lens_first);
        let ev = run(&doc);
        let operand = |id| {
            sweep::test_support::finished(
                "operand",
                crate::corpus::body_of(&ev, id).clone(),
                Tol::witness(),
            )
        };
        let (lens, brick) = (operand(lens), operand(brick));
        let (a, b, brick_is) = if lens_first {
            (&lens, &brick, topo::Operand::B)
        } else {
            (&brick, &lens, topo::Operand::A)
        };
        let Ok(topo::BooleanResult::Body(cap)) = topo::intersect(a, b, Tol::witness()) else {
            panic!("lens first {lens_first}: the cap builds");
        };
        let body = |input| if input == topo::Operand::A { a } else { b };
        let point = |input, v| {
            body(input)
                .vertex_points()
                .find(|&(k, _)| k == v)
                .map(|(_, p)| p)
        };
        // Whether `p` lies on face `f`'s surface, read for the planes and
        // spheres this fixture holds.
        let on_face = |input, f, p: geom_core::Point3<f64>| {
            let b = body(input);
            match b.face_surface(f) {
                Some(topo::Surface::Plane { frame, .. }) => {
                    let (origin, normal) = (frame.origin(), frame.axis());
                    (p - origin).dot(normal).abs() < 1e-9
                }
                Some(topo::Surface::Sphere { frame, radius, .. }) => {
                    let center = frame.origin();
                    ((p - center).norm() - radius).abs() < 1e-9
                }
                _ => false,
            }
        };
        for row in &cap.coincidences {
            let holds = match row.cells {
                [
                    topo::RowCell::Input {
                        input: x,
                        cell: topo::Cell::Vertex(v),
                    },
                    topo::RowCell::Input {
                        input: y,
                        cell: topo::Cell::Face(f),
                    },
                ] => point(x, v).is_some_and(|p| on_face(y, f, p)),
                [
                    topo::RowCell::Input {
                        input: x,
                        cell: topo::Cell::Vertex(v),
                    },
                    topo::RowCell::Input {
                        input: y,
                        cell: topo::Cell::Vertex(w),
                    },
                ] => {
                    matches!((point(x, v), point(y, w)), (Some(p), Some(q)) if (p - q).norm() < 1e-9)
                }
                [
                    topo::RowCell::Input {
                        cell: topo::Cell::Face(_),
                        ..
                    },
                    topo::RowCell::Input {
                        cell: topo::Cell::Face(_),
                        ..
                    },
                ] => row.relation != topo::Relation::OnCarrier,
                _ => false,
            };
            assert!(
                holds,
                "lens first {lens_first}: a row whose cells hold its point: {row:?}"
            );
        }
        let met: Vec<&topo::Coincidence> = cap
            .coincidences
            .iter()
            .filter(|r| r.relation == topo::Relation::Meet)
            .collect();
        let face_of = |cell: topo::RowCell, brick: bool| {
            matches!(cell, topo::RowCell::Input { input, cell: topo::Cell::Face(f) }
                if (input == brick_is) == brick && body(input).faces().any(|(k, _)| k == f))
        };
        assert!(
            met.len() == 2
                && met
                    .iter()
                    .all(|r| r.site == topo::DecisionSite::VertexFusion
                        && r.cells.iter().any(|&c| face_of(c, false))
                        && r.cells.iter().any(|&c| face_of(c, true))),
            "lens first {lens_first}: both crossings, the lens's face meeting the brick's: {met:?}"
        );
    }
}

/// The body node `id` evaluated to, with its frame.
fn framed_at(
    ev: &editor_core::Evaluation<f64>,
    id: RecipeNodeId,
    what: &str,
) -> editor_core::Framed<f64> {
    match &ev.value(id).expect("the node evaluated").payload {
        editor_core::ValuePayload::Body(b)
        | editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Body { body: b, .. }) => {
            b.clone()
        }
        other => panic!("{what}: expected a body, got {other:?}"),
    }
}

/// Every face of `result` holds a member face, and every member face
/// `holdings` names is a face of the body at its place in `members`.
fn names_member_faces(
    holdings: &topo::Holdings,
    result: &topo::Body<f64>,
    members: [&topo::Body<f64>; 2],
    what: &str,
) {
    for (f, _) in result.faces() {
        assert!(
            holdings.of(f).is_some(),
            "{what}: finished face {f:?} has holdings"
        );
    }
    let rows = holdings
        .faces()
        .map(|(_, h)| h)
        .chain(holdings.lost_rows())
        .chain(holdings.uncut_rows());
    for held in rows {
        let faces = held
            .faces()
            .unwrap_or_else(|| panic!("{what}: a holding refused: {held:?}"));
        for &(m, f) in faces {
            assert!(
                members[m].get_face(f).is_some(),
                "{what}: the holdings name member {m}'s face {f:?}, which it does not hold"
            );
        }
    }
    for (f, held) in holdings.faces() {
        assert!(
            held.faces().is_some_and(|s| !s.is_empty()),
            "{what}: finished face {f:?} holds a member face"
        );
    }
}

/// **The holdings of every op over the lens and the brick name only
/// faces of their operands**: the kernel cuts the lens's top face along
/// a meridian before the op, and rewrites the op's division rows after
/// it, so the holdings it publishes are read from the rows as rewritten,
/// the cut's new piece read back to the face it divided. The pair door
/// and the n-ary door's one step, each op, each operand order.
#[test]
fn the_holdings_over_a_cut_in_name_only_their_operands_faces() {
    let tol = Tol::witness();
    let (lens, brick) = operands();
    let (mut cut_in, mut built) = (0, 0);
    for (x, y, order) in [
        (&lens, &brick, "lens first"),
        (&brick, &lens, "brick first"),
    ] {
        for op in [
            topo::BooleanOp::Union,
            topo::BooleanOp::Intersect,
            topo::BooleanOp::Subtract,
        ] {
            let what = format!("{op:?}, {order}");
            let out = match topo::boolean_op_with(
                op,
                x,
                y,
                &topo::BooleanInputs::default(),
                topo::SweepStrategy::Realized,
                tol,
            ) {
                Ok(topo::BooleanResult::Body(out)) => out,
                other => {
                    println!("{what}: no body to read: {other:?}");
                    continue;
                }
            };
            cut_in += usize::from(closes_a_cut(&out));
            built += 1;
            names_member_faces(&out.holdings, &out.body, [x, y], &what);
        }
        for op in [topo::NaryOp::Union, topo::NaryOp::Intersect] {
            let what = format!("n-ary {op:?}, {order}");
            let inputs = topo::MemberInputs::default();
            let members: [topo::NaryMember<'_, f64>; 2] = [
                topo::NaryMember {
                    body: x,
                    inputs: &inputs,
                },
                topo::NaryMember {
                    body: y,
                    inputs: &inputs,
                },
            ];
            let run = topo::nary_boolean(op, &members, topo::SweepStrategy::Realized, tol);
            if let Some(step) = run.steps.first() {
                names_member_faces(&step.holdings, &step.body.body, [x, y], &what);
                built += 1;
            }
        }
    }
    assert!(cut_in > 0, "some op cuts the lens's face in before it runs");
    assert!(built >= 6, "most of the ten runs build: {built}");
}

/// Whether the op closed a chart cut on a sphere face: the output stage
/// deletes a cut's piece between two pieces of the face it divided with
/// one of them, a substitution row from that piece onto the face that
/// keeps it.
fn closes_a_cut(out: &topo::BooleanBody<f64>) -> bool {
    out.naming.substitutions.rows().iter().any(|&(dead, kept)| {
        matches!((dead, kept), (topo::Cell::Face(_), topo::Cell::Face(k))
            if matches!(out.body.face_surface(k), Some(topo::Surface::Sphere { .. })))
    })
}

/// The lens and the brick as the kernel's ops read them: the brick in the
/// lens's coordinates.
fn operands() -> (topo::AtRestBody<f64>, topo::AtRestBody<f64>) {
    let tol = Tol::witness();
    let (doc, [lens, brick, _]) = document("union", true);
    let ev = run(&doc);
    let lens = framed_at(&ev, lens, "the lens");
    let brick = framed_at(&ev, brick, "the brick");
    let brick = match lens.relative(&brick).expect("the frames compose") {
        None => brick.body().clone(),
        Some(map) => topo::transform::transform_rigid(brick.body(), &map)
            .expect("the brick maps into the lens's frame"),
    };
    (
        topo::test_support::finished("the lens", lens.body().clone(), tol),
        topo::test_support::finished("the brick", brick, tol),
    )
}

/// **Every op over the lens and the brick builds a maximal body whose
/// volumes add up**: the kernel cuts the lens's face in and closes the
/// cut again, so each sphere holds no more faces than the lens brought,
/// and the union, the two differences and the cap partition the two
/// operands' volumes, in either order.
#[test]
fn every_op_over_the_lens_and_the_brick_builds_maximal_and_additive() {
    let tol = Tol::witness();
    let (lens, brick) = operands();
    let vol = |b: &topo::AtRestBody<f64>, what: &str| {
        topo::test_support::pinned_volume(what, b, tol).volume
    };
    let spheres = |b: &topo::Body<f64>| {
        b.faces()
            .filter(|&(f, _)| matches!(b.face_surface(f), Some(topo::Surface::Sphere { .. })))
            .count()
    };
    let built = |out: Result<topo::BooleanResult<f64>, topo::BooleanError>, what: &str| {
        let Ok(topo::BooleanResult::Body(out)) = out else {
            panic!("{what}: the op builds: {out:?}");
        };
        let body = out.body.clone();
        assert!(
            spheres(&body) <= spheres(&lens),
            "{what}: no sphere holds more faces than the lens brought: {} against {}",
            spheres(&body),
            spheres(&lens)
        );
        vol(&body, what)
    };
    let (vl, vb) = (vol(&lens, "the lens"), vol(&brick, "the brick"));
    let lens_less = built(topo::subtract(&lens, &brick, tol), "lens ∖ brick");
    let brick_less = built(topo::subtract(&brick, &lens, tol), "brick ∖ lens");
    let slack = 1e-9 * (vl + vb);
    for (x, y, order) in [
        (&lens, &brick, "lens first"),
        (&brick, &lens, "brick first"),
    ] {
        let cap = built(topo::intersect(x, y, tol), &format!("∩, {order}"));
        let union = built(topo::union(x, y, tol), &format!("∪, {order}"));
        assert!(cap > 0.0, "{order}: the cap has volume: {cap}");
        for (what, got, want) in [
            ("∪", union, vl + vb - cap),
            ("lens ∖ brick", lens_less, vl - cap),
            ("brick ∖ lens", brick_less, vb - cap),
        ] {
            assert!(
                (got - want).abs() <= slack,
                "{order}, {what}: volume {got}, the operands' less the cap {want}"
            );
        }
    }
}
