//! Designer probe: three plates in a row, neighbours offset in z by
//! `d`, as one `Union` node, in every member order.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use crate::fixture;
use editor_core::eval::NodeResult;
use editor_core::{DocumentId, EvalOptions, Node, ProfileDoc, RecipeNodeId};
use fixture::{insert, len, on_frame, run};
use geom_core::Tol;

fn plate(doc: ProfileDoc, i: usize, _d: f64) -> (ProfileDoc, RecipeNodeId) {
    let x = i as f64;
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x, 0.0), (x + 1.0, 0.0), (x + 1.0, 1.0), (x, 1.0)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
        },
    )
}

#[test]
fn pairwise_fold_of_three_plates_in_every_order() {
    let eps = Tol::witness().eps();
    for frac in [0.3, 0.6, 0.8] {
        for order in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            let doc = ProfileDoc::empty(DocumentId::derive("probe"), Tol::witness());
            let (doc, a) = plate(doc, 0, frac * eps);
            let (doc, b) = plate(doc, 1, frac * eps);
            let (doc, c) = plate(doc, 2, frac * eps);
            let (doc, b) = fixture::slid(doc, a, b, [0.0, 0.0, frac * eps]);
            let (doc, c) = fixture::slid(doc, a, c, [0.0, 0.0, 2.0 * frac * eps]);
            let ids = [a, b, c];
            let (doc, u) = insert(
                doc,
                Node::Union {
                    members: editor_core::Bodies::Spelled(
                        order.iter().map(|&k| ids[k].into()).collect(),
                    ),
                },
            );
            let ev = run(&doc, &EvalOptions::default());
            let out = match ev.result(u) {
                Some(NodeResult::Ok(_)) => "built".to_string(),
                Some(NodeResult::Failed(e)) => format!("failed {e:?}")
                    .split("refusal:")
                    .nth(1)
                    .unwrap_or("")
                    .chars()
                    .take(200)
                    .collect(),
                other => format!("{:?}", other.map(|_| ())),
            };
            println!("d={frac}ε order {order:?}: {out}");
        }
    }
}

fn box_at(
    doc: ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
    );
    let (doc, e) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
        },
    );
    if z0 == 0.0 { (doc, e) } else { (doc, e) }
}

#[test]
fn pairwise_fold_of_a_covered_in_band_gap_in_every_order() {
    let g = 3.0 * Tol::witness().eps();
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let doc = ProfileDoc::empty(DocumentId::derive("probe-covered"), Tol::witness());
        let (doc, a) = box_at(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
        let (doc, b) = box_at(doc, (1.0, 2.0 - g), (0.0, 1.0), 0.0, 1.0);
        let (doc, b) = fixture::slid(doc, a, b, [g, 0.0, 0.0]);
        let (doc, c) = box_at(doc, (0.5, 1.5), (-0.5, 1.5), 0.0, 2.0);
        let (doc, c) = fixture::slid(doc, a, c, [0.0, 0.0, -0.5]);
        let ids = [a, b, c];
        let (doc, u) = insert(
            doc,
            Node::Union {
                members: editor_core::Bodies::Spelled(
                    order.iter().map(|&k| ids[k].into()).collect(),
                ),
            },
        );
        let ev = run(&doc, &EvalOptions::default());
        let out = match ev.result(u) {
            Some(NodeResult::Ok(_)) => "built".to_string(),
            Some(NodeResult::Failed(e)) => format!("failed {e:?}").chars().take(260).collect(),
            other => format!("{:?}", other.map(|_| ())),
        };
        println!("covered order {order:?}: {out}");
    }
}

#[test]
fn pairwise_fold_of_a_rolled_plate_in_every_order() {
    let eps = Tol::witness().eps();
    for frac in [0.2, 0.4, 3.0] {
        let th: f64 = frac * eps;
        for order in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            let doc = ProfileDoc::empty(DocumentId::derive("probe-roll"), Tol::witness());
            let (doc, a) = box_at(doc, (0.0, 10.0), (0.0, 1.0), 0.0, 1.0);
            let (doc, b) = box_at(doc, (10.0, 11.0), (0.0, 1.0), 0.0, 1.0);
            let (doc, c) = box_at(doc, (11.0, 12.0), (0.0, 1.0), 0.0, 1.0);
            let roll = editor_core::Frame::rotate_then_translate(
                [1.0, 0.0, 0.0],
                th,
                [0.0, th.sin(), 1.0 - th.cos()],
                geom_core::Band::linear(Tol::witness()).unwrap(),
            )
            .unwrap();
            let (doc, c) = fixture::against(doc, a, c, &roll);
            let (doc, b) = fixture::slid(doc, a, b, [0.0, 0.0, 0.0]);
            let ids = [a, b, c];
            let (doc, u) = insert(
                doc,
                Node::Union {
                    members: editor_core::Bodies::Spelled(
                        order.iter().map(|&k| ids[k].into()).collect(),
                    ),
                },
            );
            let ev = run(&doc, &EvalOptions::default());
            let out = match ev.result(u) {
                Some(NodeResult::Ok(_)) => "built".to_string(),
                Some(NodeResult::Failed(e)) => {
                    let s = format!("{e:?}");
                    let k = s.find("kind:").map_or(0, |i| i);
                    let m = s.find("margin").map_or("", |i| &s[i..]);
                    format!(
                        "failed {} | {}",
                        s[k..].chars().take(110).collect::<String>(),
                        m.chars().take(50).collect::<String>()
                    )
                }
                other => format!("{:?}", other.map(|_| ())),
            };
            println!("roll {frac}ε order {order:?}: {out}");
        }
    }
}
