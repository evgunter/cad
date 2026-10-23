//! **Placements**: every copy of a body the tour puts somewhere is a
//! `Node::place` of it, its bundle the mates and values the scene
//! means ([`Answer`]) — nothing read back off an evaluation:
//!
//! - [`place_in_world`]: the copy in the world where it stands
//!   ([`stand`]: the identity bundle, no mate and the constant `0` on
//!   every freedom), so it follows every variable its body reads;
//! - [`in_world`]: the copy in the world at a pose, by values on the
//!   world's axes ([`moves`]: a quarter turn written `turn/4`);
//! - [`against`]: the copy placed into another body's space at a frame
//!   in that body's own coordinates — how a construction combined with
//!   another joins its space;
//! - [`on_axis`]: a revolve's copy mated by its own axis to a frame's
//!   `z`.
//!
//! A placement is one edit, the insert of the placement holding its
//! whole bundle; [`Placed::edits`] is that edit, so a scene that
//! exports its edit log replays exactly what it applied.

use pncad::document::pose::{PoseDef, PoseRole, RoleRead};
use pncad::document::{
    BodyRead, CancelToken, Coord, Doc, DocEdit, EvalOptions, Evaluation, Formula, Frame, FrameMove,
    Mate, Operand, Placed as Answer, ProfileProgram, RecipeNodeId, TargetAxis, apply, evaluate,
    moved_frame, solve_document, stand,
};
use pncad::geom_core::Tol;
use pncad::prelude::Dimension;

/// What a placement left: the document, the placement it minted, and
/// the edits it applied, in order.
pub struct Placed {
    pub doc: Doc<ProfileProgram>,
    pub placement: RecipeNodeId,
    pub edits: Vec<DocEdit<ProfileProgram>>,
}

/// One copy of `body` in the world where it stands ([`stand`]).
///
/// # Panics
///
/// If an edit refuses.
pub fn place_in_world(doc: &Doc<ProfileProgram>, body: impl Into<Operand>, tol: Tol) -> Placed {
    written(doc, body.into(), world(), stand(), tol)
}

/// One copy of `body` in the world at `at`: its own frame moved there
/// by values on the world's axes ([`moves`]; at the identity,
/// [`place_in_world`]).
///
/// # Panics
///
/// If an edit refuses.
pub fn in_world(
    doc: &Doc<ProfileProgram>,
    body: impl Into<Operand>,
    at: &Frame,
    tol: Tol,
) -> Placed {
    if *at == Frame::IDENTITY {
        return place_in_world(doc, body, tol);
    }
    in_world_by(doc, body, moves(at), tol)
}

/// One copy of `body` in the world placed by `values` on the world's
/// axes as written — the spins a chain of turns about the world's `x`,
/// `y` and `z` through the copy's origin, the slides where they carry
/// it — each freedom no value names written `0`.
///
/// # Panics
///
/// If an edit refuses.
pub fn in_world_by(
    doc: &Doc<ProfileProgram>,
    body: impl Into<Operand>,
    values: Vec<(Coord, Formula)>,
    tol: Tol,
) -> Placed {
    written(doc, body.into(), world(), Answer::unmated(values), tol)
}

/// A copy of `body` placed into `target`'s space at `at` — the copy's
/// own frame in `target`'s own coordinates, by values on the space's
/// axes ([`moves`]). The placement's output is the copy, in `target`'s
/// space; a target that is itself a placed copy carries `at` by the
/// pose it is solved to there.
///
/// # Panics
///
/// If an edit refuses.
pub fn against(
    doc: &Doc<ProfileProgram>,
    target: impl Into<Operand>,
    body: impl Into<Operand>,
    at: &Frame,
    tol: Tol,
) -> Placed {
    let (target, body) = (target.into(), body.into());
    let space = space_of(doc, &target);
    let node = match &target {
        Operand::Node(node) | Operand::Output { node, .. } => Some(*node),
        Operand::Var(var) => doc.operation_of(*var),
        _ => None,
    };
    let posed =
        node.and_then(|node| solve_document(doc, &EvalOptions::default(), tol).relative(node));
    let at = match posed {
        Some(pose) => Frame::from_affine(pose.affine() * at.affine()),
        None => *at,
    };
    written(doc, body, space, Answer::unmated(moves(&at)), tol)
}

/// **The values standing a copy at `at`** on its target space's axes:
/// the chain of turns about `x`, `y` and `z` its axes make — each a
/// quarter turn written exactly (`turn/4`) where they are a signed
/// permutation of the space's, else the angle `atan2` takes off them
/// (`linear = Rz(γ)·Ry(β)·Rx(α)`) — and the slides its origin makes,
/// each turn and slide that is not zero.
pub fn moves(at: &Frame) -> Vec<(Coord, Formula)> {
    // Entry `r(i, j)`: row `i` of column `j`.
    let r = |i: usize, j: usize| at.columns[j][i];
    let exact = at.columns.iter().all(|column| {
        column.iter().filter(|&&c| c != 0.0).count() == 1
            && column.iter().all(|&c| c == 0.0 || c.abs() == 1.0)
    });
    let turn = |den| Formula::turn_over(den).expect("a turn is an angle");
    let quarter = |angle: f64| match (angle / std::f64::consts::FRAC_PI_2).round() as i64 {
        1 => turn(4),
        2 | -2 => turn(2),
        _ => Formula::neg(turn(4)).expect("an angle negates"),
    };
    let cos_beta = r(0, 0).hypot(r(1, 0));
    let beta = (-r(2, 0)).atan2(cos_beta);
    let (alpha, gamma) = if cos_beta > f64::EPSILON.sqrt() {
        (r(2, 1).atan2(r(2, 2)), r(1, 0).atan2(r(0, 0)))
    } else {
        ((-r(1, 2)).atan2(r(1, 1)), 0.0)
    };
    let spins = [
        (TargetAxis::X, alpha),
        (TargetAxis::Y, beta),
        (TargetAxis::Z, gamma),
    ]
    .into_iter()
    .filter(|&(_, angle)| angle != 0.0)
    .map(|(axis, angle)| {
        let by = if exact {
            quarter(angle)
        } else {
            Formula::literal(angle, Dimension::Angle).expect("a finite angle")
        };
        (Coord::Spin(axis), by)
    });
    let slides = TargetAxis::ALL
        .into_iter()
        .zip(at.translation)
        .filter(|&(_, by)| by != 0.0)
        .map(|(axis, by)| {
            let by = Formula::literal(by, Dimension::Length).expect("a finite length");
            (Coord::Slide(axis), by)
        });
    spins.chain(slides).collect()
}

/// **The frame at `origin` with axes `u` and `v`** (`v` made orthogonal
/// to `u`, both normalised; the normal `u × v`).
///
/// # Panics
///
/// If `u` and `v` span no plane.
pub fn frame(origin: [f64; 3], u: [f64; 3], v: [f64; 3]) -> Frame {
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let unit = |a: [f64; 3]| {
        let n = dot(a, a).sqrt();
        assert!(n > 0.0, "the frame's axes span a plane");
        a.map(|c| c / n)
    };
    let x = unit(u);
    let along = dot(v, x);
    let y = unit([
        v[0] - x[0] * along,
        v[1] - x[1] * along,
        v[2] - x[2] * along,
    ]);
    let z = [
        x[1] * y[2] - x[2] * y[1],
        x[2] * y[0] - x[0] * y[2],
        x[0] * y[1] - x[1] * y[0],
    ];
    Frame {
        columns: [x, y, z],
        translation: origin,
    }
}

/// **The plane through `origin` with normal `normal`, read off `body`'s
/// own geometry** ([`frame_off`] through its `xy`): a plane written in
/// the body's own coordinates, as a split's tool reads one.
///
/// # Panics
///
/// If `normal` is zero or `body` offers no frame to read.
pub fn plane_on(
    doc: &Doc<ProfileProgram>,
    body: impl Into<Operand>,
    origin: [f64; 3],
    normal: [f64; 3],
    tol: Tol,
) -> RoleRead<Operand> {
    let seed = if normal[0].abs() <= normal[1].abs().max(normal[2].abs()) {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let nn = normal.iter().map(|c| c * c).sum::<f64>();
    let k = (seed[0] * normal[0] + seed[1] * normal[1] + seed[2] * normal[2]) / nn;
    let u = [0, 1, 2].map(|i| seed[i] - normal[i] * k);
    let v = [
        normal[1] * u[2] - normal[2] * u[1],
        normal[2] * u[0] - normal[0] * u[2],
        normal[0] * u[1] - normal[1] * u[0],
    ];
    let at = frame(origin, u, v);
    RoleRead::through(frame_off(doc, &body.into(), &at, tol), PoseRole::Xy)
}

/// **The axis through `origin` along `dir`, read off `body`'s own
/// geometry** ([`frame_off`] through its `z`): a pattern's axis written in
/// the body's own coordinates.
///
/// # Panics
///
/// As [`plane_on`]'s.
pub fn axis_on(
    doc: &Doc<ProfileProgram>,
    body: impl Into<Operand>,
    origin: [f64; 3],
    dir: [f64; 3],
    tol: Tol,
) -> RoleRead<Operand> {
    RoleRead::through(plane_on(doc, body, origin, dir, tol).read, PoseRole::Z)
}

/// **A copy of the full revolve `revolve` standing on the `z` of
/// `frame`** — a frame read off the body it joins and stated by exact
/// moves ([`pncad::document::moved_frame`]), so a quarter turn is
/// `turn/4` and the axis is exactly where it is meant: an `Axis` mate
/// from the revolve's own axis, slid `along` it (on the target space's
/// axes swung onto it), its spin about it the constant `0`.
///
/// # Panics
///
/// If an edit refuses.
pub fn on_axis(
    doc: &Doc<ProfileProgram>,
    revolve: RecipeNodeId,
    frame: Operand,
    along: Formula,
    tol: Tol,
) -> Placed {
    use pncad::prelude::ContactClass;
    let space = space_of(doc, &frame);
    let to = Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::through(frame.clone(), PoseRole::Z),
        along: RoleRead::through(frame, PoseRole::Z),
        by: Formula::zero(Dimension::Length),
    }));
    let mate = Mate {
        on: Operand::Output {
            node: revolve,
            port: 1,
        },
        to,
        class: ContactClass::Rest,
    };
    // The slide an axis mate leaves, written; its spin is the constant 0.
    let placed = Answer::of(doc, vec![mate], vec![(Coord::Slide(TargetAxis::Z), along)]);
    written(doc, Operand::output(revolve, 0), space, placed, tol)
}

/// **A frame read off `body`'s own geometry, standing at `at` in its
/// own coordinates**: `Through` the first edge by name with an axis and
/// the first vertex by name off its line, moved to `at` by the moves
/// its axes and origin make there ([`moves`], [`moved_frame`]). `body`
/// is a construction, read in its own coordinates.
///
/// # Panics
///
/// If `body` does not evaluate, or offers no such edge and vertex.
fn frame_off(doc: &Doc<ProfileProgram>, body: &Operand, at: &Frame, tol: Tol) -> Operand {
    use pncad::select::{
        At, CurveKind, all_edges, all_vertices, edge_carrier_kind, edge_frame, vertex_position,
    };
    let ev: Evaluation<f64> =
        evaluate(doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
    let at_body = || -> At {
        match body {
            Operand::Var(var) => (*var).into(),
            Operand::Output { node, port } => doc
                .output(*node, *port)
                .unwrap_or_else(|| panic!("{node} defines port {port}"))
                .into(),
            Operand::Node(node) => (*node).into(),
            other => panic!("a frame is read off a body, not {other:?}"),
        }
    };
    let sub = |a: [f64; 3], b: [f64; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    let picked = all_edges(&ev, at_body()).into_iter().find_map(|edge| {
        let kind = edge_carrier_kind(&ev, at_body(), &edge).ok()?;
        matches!(
            kind,
            CurveKind::Line | CurveKind::Circle | CurveKind::Ellipse
        )
        .then_some(())?;
        let line = edge_frame(&ev, at_body(), &edge).ok()?;
        let o = [line.origin.x, line.origin.y, line.origin.z];
        let d = [line.axis.x, line.axis.y, line.axis.z];
        let n = dot(d, d).sqrt();
        (n > 0.0).then_some(())?;
        let d = d.map(|c| c / n);
        all_vertices(&ev, at_body()).into_iter().find_map(|vertex| {
            let p = vertex_position(&ev, at_body(), &vertex).ok()?;
            let off = sub([p.x, p.y, p.z], o);
            let along = dot(off, d);
            let across = sub(off, d.map(|c| c * along));
            let m = dot(across, across).sqrt();
            (m > 1e-9 * (1.0 + dot(off, off).sqrt())).then_some(())?;
            let foot = [0, 1, 2].map(|i| o[i] + d[i] * along);
            Some((edge.clone(), vertex, foot, d, across.map(|c| c / m)))
        })
    });
    let (edge, vertex, foot, z, x) = picked
        .unwrap_or_else(|| panic!("{body:?} offers an edge and a vertex to read a frame off"));
    let y = [
        z[1] * x[2] - z[2] * x[1],
        z[2] * x[0] - z[0] * x[2],
        z[0] * x[1] - z[1] * x[0],
    ];
    let read = Frame {
        columns: [x, y, z],
        translation: foot,
    };
    let rel = Frame::from_affine(read.affine::<f64>().inverse() * at.affine());
    let base = Operand::Pose(Box::new(PoseDef::Through {
        axis: RoleRead::plain(Operand::Pose(Box::new(PoseDef::Axis {
            of: Operand::select(body.clone(), vec![edge]),
        }))),
        point: RoleRead::plain(Operand::Pose(Box::new(PoseDef::Point {
            of: Operand::select(body.clone(), vec![vertex]),
        }))),
    }));
    let steps: Vec<FrameMove> = moves(&rel)
        .into_iter()
        .map(|(coord, by)| match coord {
            Coord::Spin(axis) => FrameMove::Spin(axis, by),
            Coord::Slide(axis) => FrameMove::Slide(axis, by),
        })
        .collect();
    moved_frame(base, &steps)
}

/// The world's space, as a placement's target reads it.
fn world() -> Operand {
    Operand::Var(pncad::document::VarId::WORLD)
}

/// **The space `target` is in**, as a placement's target reads it: a
/// copy placed against a body joins that body's space.
fn space_of(doc: &Doc<ProfileProgram>, target: &Operand) -> Operand {
    Operand::Var(
        doc.operand_space(target)
            .unwrap_or_else(|| panic!("the target is a body in a space: {target:?}")),
    )
}

/// The placement of `body` into `space` written as one edit: the insert
/// of the placement holding what the door answered
/// ([`pncad::document::stand`] and the rest), its whole bundle.
fn written(
    doc: &Doc<ProfileProgram>,
    body: Operand,
    space: Operand,
    placed: Answer,
    tol: Tol,
) -> Placed {
    let edit = DocEdit::place(vec![BodyRead::plain(body)], space, placed.bundle());
    let applied =
        apply(doc, &edit, tol).unwrap_or_else(|err| panic!("the placement applies: {err}"));
    let placement = applied.record.minted.expect("an insert mints an id");
    let doc = applied.doc;
    let edits = vec![edit];
    Placed {
        doc,
        placement,
        edits,
    }
}
