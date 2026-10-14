//! probe
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{failure, run};
use crate::fixture::{ang, insert, len, len2, on_frame, scl, xy_frame};
use editor_core::{
    ExtrudeSide, LoopProgram, Node, ProfileDoc, ProfileProgram, ProgramArcData, ProgramStep,
    ProgramTarget, RecipeNodeId,
};
use geom_core::Tol;

fn ball(doc: ProfileDoc, r: f64, c: f64) -> (ProfileDoc, RecipeNodeId) {
    let meridian = LoopProgram::Chain(vec![
        ProgramStep::At(len2([0.0, c - r])),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(len2([0.0, c + r])),
            b: scl(1.0),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let (doc, xy) = insert(doc, xy_frame());
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: xy.into(),
            loops: vec![meridian],
            ids: Vec::new(),
        }),
    );
    insert(
        doc,
        crate::fixture::revolve_about(p, (0.0, 0.0), (0.0, 1.0), ang(std::f64::consts::TAU)),
    )
}

fn dir(lat: f64, az: f64) -> [f64; 3] {
    let (lat, az) = (lat.to_radians(), az.to_radians());
    [lat.cos() * az.cos(), lat.sin(), lat.cos() * az.sin()]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn brick_toward(
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
            side: ExtrudeSide::Along,
        },
    )
}

fn document(op: &str, ball_first: bool) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let doc = ProfileDoc::empty_derived("ball_cut_by_brick", Tol::witness());
    let (doc, b) = ball(doc, 1.0, 0.0);
    let (doc, brick) = brick_toward(doc, dir(68.0, 50.0), 0.985, 0.2, 0.3);
    let ball_body = editor_core::Operand::output(b, 0);
    let brick_body = editor_core::Operand::from(brick);
    let (x, y) = if ball_first {
        (ball_body, brick_body)
    } else {
        (brick_body, ball_body)
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
    (doc, [b, brick, out])
}

#[test]
fn probe() {
    for op in ["intersect", "union", "subtract"] {
        for bf in [true, false] {
            let (doc, [b, brick, out]) = document(op, bf);
            let ev = run(&doc);
            eprintln!("PROBE {op} ball_first={bf}: ball {:?} brick {:?} out {:?}", failure(&ev, b), failure(&ev, brick), failure(&ev, out));
        }
    }
}
