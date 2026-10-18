//! **A sphere cut in a cap names its lens's entities.** Two revolved
//! balls, the unit ball and radius 0.8 at height 1.4, intersect in a
//! lens; a 0.4 × 0.4 × 0.3 brick, on a frame turned so its near face lies 0.985 from
//! the origin toward latitude 68°, azimuth 50°, meets the lens's top
//! face in a circle no edge reaches. The kernel cuts that face along its
//! chart's meridian through the circle before the op runs
//! (`topo::boolean::ops::apply_cut_ins`): the cut's new piece descends
//! from the face it divided, and the cut itself is no edge of the lens.
//!
//! Intersected (the cap), and the brick minus the lens, the cut is glued
//! away and the circle keeps one vertex with no identity of its own,
//! which the table refuses typed until its edge-derived name is built
//! (`work/emit/a-conventional-vertex-mints-no-edge-derived-name.md`).
//! The union and the lens minus the brick keep the cut as two edges
//! between pieces of one face ([`CUT_LEFT`]).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{failure, run};
use crate::fixture::{ang, insert, len, len2, on_frame, scl, xy_frame};
use editor_core::{
    LoopProgram, Node, ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep,
    ProgramTarget, RecipeNodeId,
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

/// A `2w × 2w × t` brick whose near face lies in the plane `n·p = s`,
/// centred on its foot, its material beyond the plane: `y` turned onto
/// `n` about `y × n`, as `sweep`'s `snowman` rows turn theirs.
pub(crate) fn brick_toward(
    doc: ProfileDoc,
    n: [f64; 3],
    s: f64,
    w: f64,
    t: f64,
) -> (ProfileDoc, RecipeNodeId) {
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
        n.map(|x| x * s),
        u,
        v,
        vec![vec![(-w, -w), (w, -w), (w, w), (-w, w)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(t),
        },
    )
}

/// Where the cut survives, the result holds two pieces of one sphere
/// face across it.
const CUT_LEFT: &str =
    "work/hone/a-sphere-cut-in-survives-unglued-in-the-union-of-a-lens-and-a-tilted-brick.md";

/// The document: the lens, the brick, and `op` over them, lens first or
/// not.
fn document(op: &str, lens_first: bool) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let doc = ProfileDoc::empty_derived("sphere_cut_in_cap", Tol::witness());
    let (doc, b1) = ball(doc, 1.0, 0.0);
    let (doc, b2) = ball(doc, 0.8, 1.4);
    let (doc, lens) = insert(
        doc,
        Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![
                editor_core::Operand::output(b1, 0).into(),
                editor_core::Operand::output(b2, 0).into(),
            ]),
        },
    );
    let (doc, brick) = brick_toward(doc, dir(68.0, 50.0), 0.985, 0.2, 0.3);
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
            members: editor_core::Bodies::Spelled(vec![x.into(), y.into()]),
        },
        _ => Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![x.into(), y.into()]),
        },
    };
    let (doc, out) = insert(doc, node);
    (doc, [lens, brick, out])
}

/// **The lens and the brick name, and every op over them refuses only
/// where a row is filed**: the cap and the brick minus the lens at the
/// circle's conventional vertex, which the totality check reaches only
/// once every face and edge is named, and the union and the lens minus
/// the brick at the cut they keep ([`CUT_LEFT`]). No op over them
/// publishes a table yet, so none is read here for the lens entities
/// its names cite; the conventional-vertex row's fix makes the first
/// three name, and asserts that here.
#[test]
fn a_sphere_cut_in_refuses_only_where_its_rows_are_filed() {
    for (op, lens_first, want) in [
        ("intersect", true, "ConventionalVertex"),
        ("intersect", false, "ConventionalVertex"),
        ("subtract", false, "ConventionalVertex"),
        ("union", true, CUT_LEFT),
        ("union", false, CUT_LEFT),
        ("subtract", true, CUT_LEFT),
    ] {
        let what = format!("{op}, lens first: {lens_first}");
        let (doc, [lens, brick, out]) = document(op, lens_first);
        let ev = run(&doc);
        assert!(failure(&ev, lens).is_none(), "{what}: the lens names");
        assert!(failure(&ev, brick).is_none(), "{what}: the brick names");
        let e = format!("{:?}", failure(&ev, out).expect("refuses"));
        if want == CUT_LEFT {
            assert!(
                e.contains("a chart cut survives in the result"),
                "{what}: refused other than at the cut ({CUT_LEFT}): {e}"
            );
        } else {
            assert!(
                e.starts_with("Naming(ConventionalVertex"),
                "{what}: refused other than at the conventional vertex: {e}"
            );
        }
    }
}
