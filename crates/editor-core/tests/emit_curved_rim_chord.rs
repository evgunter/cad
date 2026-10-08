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
use crate::emit_shared_rim_several::Bx;
use crate::fixture::{edge_of, frame, insert, len, len2, table};

use editor_core::{
    BooleanOp, EntityKind, ExtrudeSide, Formula, LoopProgram, NameRef, Node, ProfileDoc,
    ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, Qualifier, RecipeNodeId, RoleSeg,
    StableName,
};
use geom_core::Tol;
use topo::EdgeKey;

fn to(x: f64, y: f64) -> ProgramTarget<Formula> {
    ProgramTarget::Point(len2([x, y]))
}

/// Which rim the notches cut.
#[derive(Clone, Copy)]
enum Rim {
    /// The 270° arc prism: the unit arc from (1, 0) through (−1, 0) to
    /// (0, −1), closed through the origin, notched near 200°.
    Arc,
    /// The unit cylinder, drawn as two half arcs meeting tangent at 0°
    /// and 180°, notched near 90° and near 200°: one piece of the top
    /// rim runs from the 200° notch through 270° to 0°.
    Circle,
}

/// The prism of `rim`, extruded 1 from z = 0.
fn prism(doc: ProfileDoc, rim: Rim) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane) = insert(doc, frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let steps = match rim {
        Rim::Arc => vec![
            ProgramStep::At(len2([0.0, 0.0])),
            ProgramStep::LineTo(to(1.0, 0.0)),
            ProgramStep::ArcTo(ProgramArcData::Via {
                q: len2([-1.0, 0.0]),
                target: to(0.0, -1.0),
            }),
            ProgramStep::LineTo(ProgramTarget::Start),
        ],
        Rim::Circle => vec![
            ProgramStep::At(len2([1.0, 0.0])),
            ProgramStep::ArcTo(ProgramArcData::Via {
                q: len2([0.0, 1.0]),
                target: to(-1.0, 0.0),
            }),
            ProgramStep::Tangent,
            ProgramStep::TangentArcTo(ProgramTarget::StartArriving),
        ],
    };
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Chain(steps)],
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

/// `rim`'s prism with its top rim notched into pieces: the document
/// and the notched prism.
fn notched_prism(rim: Rim) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("emit_curved_rim_chord", Tol::witness());
    let (doc, mut body) = prism(doc, rim);
    let notches: &[Bx] = match rim {
        Rim::Arc => &[NEAR_200],
        Rim::Circle => &[NEAR_200, NEAR_90],
    };
    let mut doc = doc;
    for &(x, y, z) in notches {
        let (d, notch) = block(doc, x, y, z.0, z.1);
        (doc, body) = insert(
            d,
            Node::Boolean {
                op: BooleanOp::Subtract,
                a: body,
                b: notch,
                declare: Vec::new(),
            },
        );
    }
    (doc, body)
}

/// A notch through the top rim near 200°.
const NEAR_200: Bx = ((-1.5, -0.6), (-0.38, -0.30), (0.8, 1.0));
/// A notch through the top rim near 90°.
const NEAR_90: Bx = ((-0.04, 0.04), (0.6, 1.5), (0.8, 1.0));

/// The line edge `n` lies on: `n` with every piece qualifier removed,
/// through the `FromA`/`FromB` wrappers a boolean adds (names README,
/// "Edge pieces").
fn line_of(n: &StableName) -> StableName {
    let mut n = n.clone();
    while matches!(n.path.last(), Some(RoleSeg::Fragment(Qualifier::Ends(_)))) {
        n.path.pop();
    }
    if let [RoleSeg::FromA(inner) | RoleSeg::FromB(inner)] = n.path.as_mut_slice() {
        *inner = NameRef::new(line_of(inner));
    }
    n
}

/// Every edge `union` names for an edge of member `notched` lies
/// within that edge — on a straight source, both ends on its segment;
/// on an arc about the z axis, both ends on its circle within the
/// angular span its ends and middle bound — and the count of the
/// union's curved edges named so on the top rim (z = 1).
fn within_sources(
    ev: &editor_core::Evaluation<f64>,
    union: RecipeNodeId,
    notched: RecipeNodeId,
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
    let angle = |p: geom_core::Point3<f64>| p.y.atan2(p.x);
    let body = body_of(ev, union);
    let src_body = body_of(ev, notched);
    let src_table = table(ev, notched);
    let source_mid = |k: EdgeKey| {
        let c = src_body
            .get_edge(k)
            .and_then(|d| src_body.get_curve_geom(d.curve))
            .and_then(topo::CurveGeom::certified)
            .expect("a source arc has a certified curve");
        let (t0, t1) = c.params();
        c.carrier().mid_point(t0, t1)
    };
    let ends_closed = |k: EdgeKey| {
        let e = src_body.get_edge(k).unwrap();
        let start = |he| src_body.get_half_edge(he).unwrap().start;
        start(e.he_plus) == start(e.he_minus)
    };
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
        if *member != notched {
            continue;
        }
        // The source rows: `of` itself, or, where `of` cites a line,
        // the rows lying on it.
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
                    // Counter-clockwise turn from `q0`, in [0, 2π): the
                    // source runs that way when its middle lies within
                    // its span that way, and clockwise otherwise.
                    let turn = |p| (angle(p) - angle(q0)).rem_euclid(std::f64::consts::TAU);
                    let closed = ends_closed(src);
                    let span = turn(q1);
                    let ccw = turn(source_mid(src)) < span;
                    here.iter().all(|&p| {
                        let t = turn(p);
                        ((p.x * p.x + p.y * p.y).sqrt() - 1.0).abs() < 1e-9
                            && (p.z - q0.z).abs() < 1e-9
                            && if closed {
                                true
                            } else if ccw {
                                t < span + 1e-9 || t > std::f64::consts::TAU - 1e-9
                            } else {
                                t > span - 1e-9 || t < 1e-9
                            }
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

/// `rim`'s notched prism united with a post `x × y`, z ∈ [0.5, 1.5],
/// in both member orders: each union evaluates, every edge it names for
/// a member edge lies within that edge, and at least `pieces` curved
/// edges on the top rim are named so.
fn unites_in_both_orders(rim: Rim, x: (f64, f64), y: (f64, f64), pieces: usize, at: &str) {
    for flip in [false, true] {
        let (doc, notched) = notched_prism(rim);
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
        let named = within_sources(&ev, u, notched, &at);
        assert!(
            named >= pieces,
            "{at}: the top rim survives in at least {pieces} named pieces \
             (the notches and the post each cut it), found {named}"
        );
    }
}

/// **A post across the first piece of the notched rim arc.**
#[test]
fn a_post_across_the_first_piece_of_a_notched_rim_arc_unites() {
    unites_in_both_orders(
        Rim::Arc,
        (0.6, 0.8),
        (0.4, 1.2),
        3,
        "post across the first piece",
    );
}

/// **A post across the second piece**, between the notch and 270°.
#[test]
fn a_post_across_the_second_piece_of_a_notched_rim_arc_unites() {
    unites_in_both_orders(
        Rim::Arc,
        (-0.4, -0.2),
        (-1.2, -0.7),
        3,
        "post across the second piece",
    );
}

/// **A post across the piece of a notched full circle that runs
/// through 0°**, where the profile's two arcs meet.
#[test]
fn a_post_across_the_piece_of_a_notched_circle_through_0_unites() {
    unites_in_both_orders(Rim::Circle, (0.8, 1.5), (-0.2, 0.2), 3, "post across 0°");
}

/// **A post across the other piece of the notched full circle**, near
/// 130°.
#[test]
fn a_post_across_the_other_piece_of_a_notched_circle_unites() {
    unites_in_both_orders(Rim::Circle, (-0.8, -0.4), (0.6, 1.2), 3, "post near 130°");
}
