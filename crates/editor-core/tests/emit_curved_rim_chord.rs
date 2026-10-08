//! **A chord on a curved rim cut into pieces is named for the piece it
//! lies within.**
//!
//! A 270° arc prism — an arc of radius 1 from 0° to 270° about the
//! origin, closed through the centre, extruded 1 — is notched through
//! its top rim arc, which leaves the top cap and the arc wall sharing
//! two edges: two pieces of one circle. A post standing across one
//! piece cuts a chord out of it, and the union names that chord as a
//! piece of the member's rim, in either member order. The same union
//! over a straight rim in two pieces is `resolve_cited_line`'s.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::{edge_of, frame, insert, len, len2, table};

use editor_core::{
    BooleanOp, EntityKind, ExtrudeSide, Formula, LoopProgram, Node, ProfileDoc, ProfileProgram,
    ProgramArcData, ProgramStep, ProgramTarget, Qualifier, RecipeNodeId, RoleSeg, StableName,
};
use geom_core::Tol;
use topo::EdgeKey;

fn to(x: f64, y: f64) -> ProgramTarget<Formula> {
    ProgramTarget::Point(len2([x, y]))
}

/// The 270° arc prism: the unit arc from (1, 0) through (−1, 0) to
/// (0, −1), closed through the origin, extruded 1 from z = 0.
fn arc_prism(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane) = insert(doc, frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Chain(vec![
                ProgramStep::At(len2([0.0, 0.0])),
                ProgramStep::LineTo(to(1.0, 0.0)),
                ProgramStep::ArcTo(ProgramArcData::Via {
                    q: len2([-1.0, 0.0]),
                    target: to(0.0, -1.0),
                }),
                ProgramStep::LineTo(ProgramTarget::Start),
            ])],
            ids: Vec::new(),
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    )
}

/// The arc prism notched through its top rim arc near 200°: the
/// document and the notched prism.
fn notched_prism() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("emit_curved_rim_chord", Tol::witness());
    let (doc, prism) = arc_prism(doc);
    let (doc, notch) = block(doc, (-1.5, -0.6), (-0.38, -0.30), 0.8, 1.0);
    insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: prism,
            b: notch,
            declare: Vec::new(),
        },
    )
}

/// Every edge `union` names for an edge of member `prism` lies within
/// that edge — on a straight source, both ends on its segment; on an
/// arc, both ends on its circle within its angular span — and the
/// count of those on the top rim arc (z = 1, radius 1). The prism's arc
/// sweeps 0..270° about the origin, so a piece's span is the interval
/// between its ends' angles, read in [0, 2π).
fn within_sources(
    ev: &editor_core::Evaluation<f64>,
    union: RecipeNodeId,
    prism: RecipeNodeId,
    at: &str,
) -> usize {
    let point = |b: &topo::Body<f64>, he| {
        let v = b.get_half_edge(he).unwrap().start;
        *b.get_point(b.get_vertex(v).unwrap().point).unwrap()
    };
    let ends = |b: &topo::Body<f64>, k| {
        let e = b.get_edge(k).unwrap();
        [point(b, e.he_plus), point(b, e.he_minus)]
    };
    let angle = |p: geom_core::Point3<f64>| {
        let a = p.y.atan2(p.x);
        if a < -1e-9 {
            a + std::f64::consts::TAU
        } else {
            a.max(0.0)
        }
    };
    let body = body_of(ev, union);
    let src_body = body_of(ev, prism);
    let src_table = table(ev, prism);
    let mut on_top_arc = 0;
    for (name, entry) in table(ev, union).iter() {
        let (editor_core::Entry::Unique(e), EntityKind::Edge) = (entry, name.kind) else {
            continue;
        };
        let editor_core::EntityKey::Edge(k) = e.key else {
            continue;
        };
        let Some(RoleSeg::FromMember { member, of }) = name.path.first() else {
            continue;
        };
        if *member != prism {
            continue;
        }
        // The source rows: `of` itself, or, where `of` cites a line,
        // the rows lying on it (the line + `Fragment(Ends)`).
        let line_of = |n: &StableName| {
            let mut n = n.clone();
            while matches!(n.path.last(), Some(RoleSeg::Fragment(Qualifier::Ends(_)))) {
                n.path.pop();
            }
            n
        };
        let sources: Vec<EdgeKey> = match src_table.lookup(of) {
            Some(_) => vec![edge_of(src_table, "the member's edge", of)],
            None => src_table
                .iter()
                .filter(|(n, _)| n.kind == EntityKind::Edge && line_of(n) == **of)
                .map(|(n, _)| edge_of(src_table, "a row on the cited line", n))
                .collect(),
        };
        let here = ends(body, k);
        let within = |src: EdgeKey| {
            let [q0, q1] = ends(src_body, src);
            match topo::query::edge_carrier_kind(src_body, src) {
                Some(topo::query::CurveKind::Line) => {
                    let d = q1 - q0;
                    here.iter().all(|&p| {
                        let off = (p - q0).cross(d).norm() / d.norm();
                        let s = (p - q0).dot(d) / d.dot(d);
                        off < 1e-9 && s > -1e-9 && s < 1.0 + 1e-9
                    })
                }
                Some(topo::query::CurveKind::Circle) => {
                    let (lo, hi) = (angle(q0).min(angle(q1)), angle(q0).max(angle(q1)));
                    here.iter().all(|&p| {
                        ((p.x * p.x + p.y * p.y).sqrt() - 1.0).abs() < 1e-9
                            && (p.z - q0.z).abs() < 1e-9
                            && angle(p) > lo - 1e-9
                            && angle(p) < hi + 1e-9
                    })
                }
                other => panic!("{at}: {name:?} descends from an edge of kind {other:?}"),
            }
        };
        assert!(
            sources.iter().any(|&src| within(src)),
            "{at}: {name:?} with ends {here:?} lies within none of its sources {sources:?}"
        );
        if topo::query::edge_carrier_kind(body, k) == Some(topo::query::CurveKind::Circle)
            && here.iter().all(|p| (p.z - 1.0).abs() < 1e-9)
        {
            on_top_arc += 1;
        }
    }
    on_top_arc
}

/// The notched prism united with a post `x × y`, z ∈ [0.5, 1.5], in
/// both member orders: each union evaluates, every edge it names for
/// a member edge lies on that edge, and a piece of the prism's top rim
/// arc survives on each side of the post, named for that rim.
fn unites_in_both_orders(x: (f64, f64), y: (f64, f64), at: &str) {
    for flip in [false, true] {
        let (doc, notched) = notched_prism();
        let (doc, post) = block(doc, x, y, 0.5, 1.0);
        let members = if flip {
            vec![post, notched]
        } else {
            vec![notched, post]
        };
        let (doc, u) = insert(
            doc,
            Node::Union {
                members: members.clone(),
                declare: Vec::new(),
            },
        );
        let ev = run(&doc);
        let at = format!("{at}, members {members:?}");
        assert!(
            failure(&ev, u).is_none(),
            "{at}: the union evaluates: {:?}",
            failure(&ev, u)
        );
        assert!(
            within_sources(&ev, u, notched, &at) >= 3,
            "{at}: the top rim arc survives in at least three named pieces \
             (the notch and the post each cut it)"
        );
    }
}

/// **A post across the first piece of the notched rim arc.**
#[test]
fn a_post_across_the_first_piece_of_a_notched_rim_arc_unites() {
    unites_in_both_orders((0.6, 0.8), (0.4, 1.2), "post across the first piece");
}

/// **A post across the second piece**, between the notch and 270°.
#[test]
fn a_post_across_the_second_piece_of_a_notched_rim_arc_unites() {
    unites_in_both_orders((-0.4, -0.2), (-1.2, -0.7), "post across the second piece");
}
