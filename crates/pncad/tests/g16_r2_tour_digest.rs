//! REVIEW PROBE (lib-g16-r2): the TOUR's composed-die document —
//! `demos/tour/src/diefillet.rs::build`, transcribed verbatim minus the
//! render plumbing — evaluated and its name tables digested, so the
//! `emit_fillet` re-shape's behaviour-preservation claim is measured on
//! the richest fillet-minting tour scene rather than only the two
//! registered corpus documents. Three `Node::fillet` sites: the blank
//! (all twelve edges), the box edges of the 21-pip die, and the 21
//! pip rims at a second radius, through twenty union nodes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use pncad::document::BooleanOp;
use pncad::geom_core::Tol;
use pncad::prelude::{
    CancelToken, CurveKind, CurveKindSet, Datum, Dimension, Doc, DocEdit, EntityKind, EvalOptions,
    Evaluation, Expr, GeomPred, LoopProgram, NamePat, Node, Point3, ProfileProgram, ProgramArcData,
    ProgramStep, ProgramTarget, RecipeNodeId, Selector, SketchPlane, SurfaceKind, SurfaceKindSet,
    Vec3, all_edges, apply, evaluate, select_where,
};

const L: f64 = 1.0;
const R: f64 = 0.12;
const RIM_R: f64 = 0.02;
const PIP_R: f64 = 0.09;
const PIP_H: f64 = 0.05;
const PIP_D: f64 = 0.22;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a length")
}
fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).expect("an angle")
}
fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("a scalar")
}

fn layout(n: u32) -> Vec<(f64, f64)> {
    let c = vec![(0.0, 0.0)];
    let diag = vec![(-1.0, -1.0), (1.0, 1.0)];
    let anti = vec![(-1.0, 1.0), (1.0, -1.0)];
    let sides = vec![(-1.0, 0.0), (1.0, 0.0)];
    match n {
        1 => c,
        2 => diag,
        3 => [diag.clone(), c].concat(),
        4 => [diag.clone(), anti.clone()].concat(),
        5 => [diag.clone(), anti.clone(), c].concat(),
        _ => [diag, anti, sides].concat(),
    }
}

struct Placement {
    centre: [f64; 3],
    axis: [f64; 3],
    angle: f64,
}

const X: [f64; 3] = [1.0, 0.0, 0.0];
const Y: [f64; 3] = [0.0, 1.0, 0.0];
const Z: [f64; 3] = [0.0, 0.0, 1.0];
const NEG_X: [f64; 3] = [-1.0, 0.0, 0.0];
const NEG_Y: [f64; 3] = [0.0, -1.0, 0.0];
const NEG_Z: [f64; 3] = [0.0, 0.0, -1.0];

fn placements() -> Vec<Placement> {
    let h = L / 2.0;
    let faces = [
        (1u32, Z, X, Y, Z, 0.0),
        (6, NEG_Z, X, Y, X, PI),
        (2, X, Y, Z, Y, FRAC_PI_2),
        (5, NEG_X, Y, Z, Y, -FRAC_PI_2),
        (3, Y, Z, X, X, -FRAC_PI_2),
        (4, NEG_Y, Z, X, X, FRAC_PI_2),
    ];
    let mut out = Vec::new();
    for (n, normal, ex, ey, axis, angle) in faces {
        let d = h + (PIP_R - PIP_H);
        for (u, w) in layout(n) {
            let centre = core::array::from_fn(|i| {
                h + normal[i] * d + ex[i] * (u * PIP_D) + ey[i] * (w * PIP_D)
            });
            out.push(Placement {
                centre,
                axis,
                angle,
            });
        }
    }
    out
}

fn half_disc() -> LoopProgram {
    let p = |x: f64, y: f64| [len(x), len(y)];
    LoopProgram::Chain(vec![
        ProgramStep::At(p(0.0, -PIP_R)),
        ProgramStep::ArcTo(ProgramArcData::Bulge {
            target: ProgramTarget::Point(p(0.0, PIP_R)),
            b: scl(1.0),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ])
}

fn eval(doc: &Doc<ProfileProgram>, tol: Tol) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), tol)
}

fn insert(doc: &mut Doc<ProfileProgram>, node: Node<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the edit applies");
    *doc = applied.doc;
    applied.record.minted.expect("insert mints an id")
}

fn cube_node(doc: &mut Doc<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let cube_p = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::xy(),
            loops: vec![LoopProgram::polygon([(0.0, 0.0), (L, 0.0), (L, L), (0.0, L)]).unwrap()],
        }),
        tol,
    );
    insert(
        doc,
        Node::Extrude {
            profile: cube_p,
            distance: len(L),
        },
        tol,
    )
}

fn pipped_node(doc: &mut Doc<ProfileProgram>, cube: RecipeNodeId, tol: Tol) -> RecipeNodeId {
    let axis = insert(
        doc,
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(0.0), scl(1.0)],
        }),
        tol,
    );
    let ball_p = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: SketchPlane::from_frame(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            loops: vec![half_disc()],
        }),
        tol,
    );
    let ball = insert(
        doc,
        Node::Revolve {
            profile: ball_p,
            axis,
            angle: ang(TAU),
        },
        tol,
    );
    let mut tool: Option<RecipeNodeId> = None;
    for p in placements() {
        let pip = insert(
            doc,
            Node::Transform {
                input: ball,
                translation: p.centre.map(len),
                rotation_axis: p.axis.map(scl),
                rotation_angle: ang(p.angle),
            },
            tol,
        );
        tool = Some(match tool {
            None => pip,
            Some(acc) => insert(
                doc,
                Node::Boolean {
                    op: BooleanOp::Union,
                    a: acc,
                    b: pip,
                    declare: None,
                },
                tol,
            ),
        });
    }
    insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: cube,
            b: tool.expect("21 pips"),
            declare: None,
        },
        tol,
    )
}

/// FNV-1a 64 over the tables (`m4_pr3_names_ci.rs::digest`).
fn digest(ev: &Evaluation<f64>) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |s: &str| {
        for b in s.bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    };
    for id in &ev.order {
        feed(&format!("#{id:?}"));
        if let Some(v) = ev.value(*id) {
            for (n, e) in v.name_table.iter() {
                feed(&format!("{n:?}={e:?};"));
            }
        }
    }
    h
}

#[test]
fn g16_r2_tour_die_name_digest() {
    let tol = Tol::witness();
    let mut doc: Doc<ProfileProgram> = Doc::empty_derived("die", tol);
    let edges = Selector::of(NamePat::of_kind(EntityKind::Edge));

    let cube = cube_node(&mut doc, tol);
    let all_twelve = all_edges(&eval(&doc, tol), cube);
    let blank = insert(&mut doc, Node::fillet(cube, len(R), all_twelve), tol);

    let pipped = pipped_node(&mut doc, cube, tol);

    let params = doc.param_env::<f64>();
    let box_edges = select_where(
        &eval(&doc, tol),
        pipped,
        &edges,
        &[GeomPred::CurveKind(CurveKindSet::just(CurveKind::Line))],
        &params,
        tol,
    )
    .expect("EXACT atoms are total");
    let blanked = insert(
        &mut doc,
        Node::fillet(pipped, len(R), box_edges.clone()),
        tol,
    );

    let rims = select_where(
        &eval(&doc, tol),
        blanked,
        &edges,
        &[GeomPred::AdjacentKinds(
            SurfaceKindSet::just(SurfaceKind::Plane),
            SurfaceKindSet::just(SurfaceKind::Sphere),
        )],
        &params,
        tol,
    )
    .expect("EXACT atoms are total");
    let composed = insert(
        &mut doc,
        Node::fillet(blanked, len(RIM_R), rims.clone()),
        tol,
    );

    let ev = eval(&doc, tol);
    assert!(ev.value(blank).is_some(), "the blank evaluated");
    assert!(ev.value(composed).is_some(), "the composed die evaluated");
    println!(
        "G16R2 tour_die box_edges={} rims={} digest={:016x}",
        box_edges.len(),
        rims.len(),
        digest(&ev)
    );
}
