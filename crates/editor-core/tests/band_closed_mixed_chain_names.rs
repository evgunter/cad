//! **A rounded outline fillets whole through the document**: a rounded
//! plate, every edge by name, and a rounded pocket cut by
//! `Node::Subtract` — its mouth by name, and every edge of the pocketed
//! block. Each removes Pappus's closed form, and each junction is named
//! once, as `BandSection { vertex }`, a name an unrelated edit keeps.
//! The fixtures are review lane band-chain-r2's for PR 6520.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{insert, len, len2, slid, table};
use editor_core::{
    CancelToken, DocEdit, EntityKey, EntityKind, EntityRef, EvalOptions, Evaluation, LoopProgram,
    Node, NodeResult, ProfileDoc, ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId,
    RoleSeg, SlotId, StableName, apply, evaluate,
};
use geom_core::Tol;
use std::f64::consts::PI;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A rounded rectangle `[x0, x0 + w] × [y0, y0 + h]`, corner radius `r`.
fn rounded(x0: f64, y0: f64, w: f64, h: f64, r: f64) -> LoopProgram<editor_core::Formula> {
    let p = |x: f64, y: f64| ProgramTarget::Point(len2([x, y]));
    let t = || ProgramStep::Tangent;
    LoopProgram::Chain(vec![
        ProgramStep::At(len2([x0 + r, y0])),
        ProgramStep::LineTo(p(x0 + w - r, y0)),
        t(),
        ProgramStep::TangentArcTo(p(x0 + w, y0 + r)),
        t(),
        ProgramStep::Line(len(h - 2.0 * r)),
        t(),
        ProgramStep::TangentArcTo(p(x0 + w - r, y0 + h)),
        t(),
        ProgramStep::Line(len(w - 2.0 * r)),
        t(),
        ProgramStep::TangentArcTo(p(x0, y0 + h - r)),
        t(),
        ProgramStep::Line(len(h - 2.0 * r)),
        t(),
        ProgramStep::TangentArcTo(ProgramTarget::StartArriving),
    ])
}

fn prism(
    doc: ProfileDoc,
    lp: LoopProgram<editor_core::Formula>,
    d: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![lp],
            ids: Vec::new(),
            on: None,
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(d),
        },
    )
}

fn body(ev: &Evaluation<f64>, node: RecipeNodeId) -> &topo::Body<f64> {
    match ev.nodes.get(&node) {
        Some(NodeResult::Ok(v)) => v.payload.output_body(0).expect("one body").body(),
        other => panic!("node {node:?}: {other:?}"),
    }
}

/// A body's volume and the pad of its certified enclosure.
fn volume(ev: &Evaluation<f64>, node: RecipeNodeId) -> (f64, f64) {
    let p = topo::mass_properties(
        &topo::test_support::finished("m", body(ev, node).clone(), Tol::witness()),
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .expect("props");
    (p.enclosure.volume, p.enclosure.volume_pad)
}

fn sections(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<StableName> {
    let mut v: Vec<StableName> = table(ev, node)
        .iter()
        .filter(|(n, _)| {
            n.kind == EntityKind::Edge
                && n.node == node
                && matches!(n.path.first(), Some(RoleSeg::BandSection { .. }))
        })
        .map(|(n, _)| n.clone())
        .collect();
    v.sort();
    v
}

/// The section a round of radius `r` removes or adds, and its centroid's
/// inset from each face.
fn section(r: f64) -> (f64, f64) {
    (
        r * r * (1.0 - PI / 4.0),
        r * (10.0 - 3.0 * PI) / (3.0 * (4.0 - PI)),
    )
}

/// The perimeter of a `w × h` rounded rectangle of corner radius `rr`.
fn perimeter(w: f64, h: f64, rr: f64) -> f64 {
    2.0 * (w - 2.0 * rr) + 2.0 * (h - 2.0 * rr) + 2.0 * PI * rr
}

fn assert_removed(
    what: &str,
    ev0: &Evaluation<f64>,
    a: RecipeNodeId,
    ev: &Evaluation<f64>,
    b: RecipeNodeId,
    truth: f64,
) {
    let ((v0, w0), (v1, w1)) = (volume(ev0, a), volume(ev, b));
    assert!(
        w0 + w1 < 1e-7,
        "{what}: the enclosure is a claim: ±{}",
        w0 + w1
    );
    assert!(
        ((v0 - v1) - truth).abs() <= w0 + w1 + 1e-12,
        "{what}: ΔV {} is the closed form {truth}",
        v0 - v1
    );
}

#[test]
fn a_rounded_plate_filleted_by_name_names_each_section_once() {
    let doc = ProfileDoc::empty_derived("band_mixed_plate", Tol::witness());
    let (doc, plate) = prism(doc, rounded(0.0, 0.0, 2.0, 1.2, 0.3), 0.5);
    let ev0 = run(&doc);
    let every = editor_core::all_edges(&ev0, plate);
    let (doc, f) = insert(doc, Node::fillet(plate, len(0.1), every));
    let ev = run(&doc);
    let (a, d) = section(0.1);
    let truth = 2.0 * a * (perimeter(2.0, 1.2, 0.3) - 2.0 * PI * d);
    assert_removed("plate", &ev0, plate, &ev, f, truth);
    let named = sections(&ev, f);
    let mut unique = named.clone();
    unique.dedup();
    assert_eq!(
        (named.len(), unique.len()),
        (16, 16),
        "one name per junction of both rims"
    );
    let bumped = apply(
        &doc,
        &DocEdit::SetParam {
            node: plate,
            slot: SlotId::Distance,
            value: len(0.6).into(),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("the bump applies");
    assert_eq!(
        named,
        sections(&run(&bumped.doc), f),
        "the section names survive the bump"
    );
}

/// A `3 × 2 × 1` block less the rounded rectangle `[0.5, 2.5] × [0.4,
/// 1.6]`, corner radius `0.3`, from `z = 0.5`.
fn pocketed() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("band_mixed_pocket", Tol::witness());
    let (doc, block) = prism(
        doc,
        LoopProgram::polygon([(0.0, 0.0), (3.0, 0.0), (3.0, 2.0), (0.0, 2.0)]).unwrap(),
        1.0,
    );
    let (doc, tool) = prism(doc, rounded(0.5, 0.4, 2.0, 1.2, 0.3), 1.0);
    let (doc, tool) = slid(doc, block, tool, [0.0, 0.0, 0.5]);
    insert(
        doc,
        Node::Subtract {
            from: block.into(),
            tool: tool.into(),
        },
    )
}

#[test]
fn a_rounded_pockets_mouth_filleted_by_name_removes_pappuss_volume() {
    let (doc, cut) = pocketed();
    let ev0 = run(&doc);
    let b = body(&ev0, cut);
    let t = table(&ev0, cut);
    let mouth: Vec<StableName> = b
        .edges()
        .filter(|(k, _)| {
            let at = |v| *b.get_point(b.get_vertex(v).unwrap().point).unwrap();
            let (s, t) = b.edge_vertices(*k).unwrap();
            let (p, q) = (at(s), at(t));
            let inside = |p: geom_core::Point3<f64>| {
                p.x > 0.45 && p.x < 2.55 && p.y > 0.35 && p.y < 1.65 && (p.z - 1.0).abs() < 1e-9
            };
            inside(p) && inside(q)
        })
        .map(|(k, _)| {
            t.name_of(&EntityRef {
                body: 0,
                key: EntityKey::Edge(k),
            })
            .cloned()
            .expect("a mouth edge's name")
        })
        .collect();
    assert_eq!(mouth.len(), 8, "the mouth: four lines and four arcs");
    let (doc, f) = insert(doc, Node::fillet(cut, len(0.1), mouth));
    let ev = run(&doc);
    let (a, d) = section(0.1);
    assert_removed(
        "mouth",
        &ev0,
        cut,
        &ev,
        f,
        a * (perimeter(2.0, 1.2, 0.3) + 2.0 * PI * d),
    );
    assert_eq!(sections(&ev, f).len(), 8, "one section per junction");
}

/// **Every edge of the pocketed block**: the box's twelve rounded with
/// their eight corner balls, the mouth's band removing and the floor's
/// adding. The box rounds to its shrunk box swept by the ball.
#[test]
fn every_edge_of_a_rounded_pocket_fillets_at_its_closed_form() {
    let (doc, cut) = pocketed();
    let ev0 = run(&doc);
    let every = editor_core::all_edges(&ev0, cut);
    let r = 0.1;
    let (doc, f) = insert(doc, Node::fillet(cut, len(r), every));
    let ev = run(&doc);
    let (x, y, z) = (3.0 - 2.0 * r, 2.0 - 2.0 * r, 1.0 - 2.0 * r);
    let rounded_box = x * y * z
        + 2.0 * r * (x * y + y * z + x * z)
        + PI * r * r * (x + y + z)
        + 4.0 / 3.0 * PI * r.powi(3);
    let (a, d) = section(r);
    let p = perimeter(2.0, 1.2, 0.3);
    let truth = (3.0 * 2.0 * 1.0 - rounded_box) + a * (p + 2.0 * PI * d) - a * (p - 2.0 * PI * d);
    assert_removed("every edge", &ev0, cut, &ev, f, truth);
    assert_eq!(
        sections(&ev, f).len(),
        16,
        "the mouth's and the floor's junctions"
    );
}
