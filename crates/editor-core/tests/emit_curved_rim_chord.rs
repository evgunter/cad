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

use crate::blocks::{block_on, failure, run};
use crate::corpus::body_of;
use crate::emit_shared_rim_several::Bx;
use crate::fixture::{edge_of, insert, len, len2, out, table};

use editor_core::{
    EntityKind, Formula, LoopProgram, NameRef, NameTable, Node, ProfileDoc, ProfileProgram,
    ProgramArcData, ProgramStep, ProgramTarget, Qualifier, RecipeNodeId, RoleSeg, StableName,
    VarId,
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
            loops: vec![LoopProgram::Chain(steps)],
            ids: Vec::new(),
            on: None,
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
        },
    )
}

/// `rim`'s prism with its top rim notched into pieces, each notch a
/// block placed against the prism: the document, the notched prism and
/// the prism, the root a later block is placed against.
fn notched_prism(rim: Rim) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("emit_curved_rim_chord", Tol::witness());
    let (doc, root) = prism(doc, rim);
    let mut body = root;
    let notches: &[Bx] = match rim {
        Rim::Arc => &[NEAR_200],
        Rim::Circle => &[NEAR_200, NEAR_90],
    };
    let mut doc = doc;
    for &(x, y, z) in notches {
        let (d, notch) = block_on(doc, root, x, y, z.0, z.1);
        (doc, body) = insert(
            d,
            Node::Subtract {
                from: body.into(),
                tool: notch.into(),
            },
        );
    }
    (doc, body, root)
}

/// A notch through the top rim near 200°.
const NEAR_200: Bx = ((-1.5, -0.6), (-0.38, -0.30), (0.8, 1.0));
/// A notch through the top rim near 90°.
const NEAR_90: Bx = ((-0.04, 0.04), (0.6, 1.5), (0.8, 1.0));

/// The line edge `n` lies on: `n` with every piece qualifier removed,
/// through the `From` wrappers a boolean adds (names README,
/// "Edge pieces").
fn line_of(n: &StableName) -> StableName {
    let mut n = n.clone();
    while matches!(n.path.last(), Some(RoleSeg::Fragment(Qualifier::Ends(_)))) {
        n.path.pop();
    }
    if let [RoleSeg::From { of: inner, .. }] = n.path.as_mut_slice() {
        *inner = NameRef::new(line_of(inner));
    }
    n
}

/// Every edge `union` names for an edge of member `notched` lies
/// within exactly one row of the line its name cites — on a straight
/// row, both ends on its segment; on an arc about the z axis, both ends
/// on its circle within the angular span its ends and middle bound —
/// and its name is that row's: the row bare, or its line + `Ends` where
/// the edge is part of it. Returns the
/// count of the curved edges named so on the top rim (z = 1).
fn within_sources(
    ev: &editor_core::Evaluation<f64>,
    union: RecipeNodeId,
    published: &NameTable,
    notched: RecipeNodeId,
    notched_read: VarId,
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
    for (name, entry) in published.iter() {
        let (editor_core::Entry::Unique(e), EntityKind::Edge) = (entry, name.kind) else {
            continue;
        };
        let editor_core::EntityKey::Edge(k) = e.key else {
            continue;
        };
        let Some(RoleSeg::From { read, of }) = name.path.first() else {
            continue;
        };
        if *read != notched_read {
            continue;
        }
        // Every row of `notched` on the line `of` lies on: `of`
        // itself where it is a row, and its siblings.
        let line = line_of(of);
        let rows: Vec<(StableName, EdgeKey)> = src_table
            .iter()
            .filter(|(n, _)| n.kind == EntityKind::Edge && line_of(n) == line)
            .map(|(n, _)| (n.clone(), edge_of(src_table, "a row on the cited line", n)))
            .collect();
        let here = ends(&body, k);
        let within = |src: EdgeKey| {
            let [q0, q1] = ends(&src_body, src);
            match topo::query::edge_carrier_kind(&src_body, src) {
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
        // The piece the edge descends from is the one row it lies
        // within; the pieces are disjoint, so there is at most one.
        let holding: Vec<&(StableName, EdgeKey)> =
            rows.iter().filter(|(_, src)| within(*src)).collect();
        let [(piece, src)] = holding.as_slice() else {
            panic!(
                "{at}: {name:?} with ends {here:?} lies within {holding:?}, not one row of its line"
            );
        };
        let whole = {
            let q = ends(&src_body, *src);
            let same = |a: geom_core::Point3<f64>, b: geom_core::Point3<f64>| (a - b).norm() < 1e-9;
            (same(here[0], q[0]) && same(here[1], q[1]))
                || (same(here[0], q[1]) && same(here[1], q[0]))
        };
        // A bare name cites the piece the edge descends from; a piece
        // spelled on its line is part of its row, never all of it
        // (N2). A bare name on part of its row is
        // `a-lone-cut-piece-of-an-operand-edge-keeps-its-bare-name`,
        // not asserted here.
        match src_table.lookup(of) {
            Some(_) => assert!(
                **of == *piece,
                "{at}: {name:?} cites the row {of:?}, but the edge {here:?} lies within {piece:?}"
            ),
            None => assert!(
                !whole,
                "{at}: {name:?} is spelled as a piece of its line, but the edge {here:?} is \
                 all of {piece:?}"
            ),
        }
        if topo::query::edge_carrier_kind(&body, k) == Some(topo::query::CurveKind::Circle)
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
        let (doc, notched, root) = notched_prism(rim);
        let (doc, post) = block_on(doc, root, x, y, 0.5, 1.0);
        let members = if flip {
            vec![post, notched]
        } else {
            vec![notched, post]
        };
        let (doc, u) = insert(
            doc,
            Node::Union {
                members: editor_core::Bodies::Spelled(members.iter().map(|&m| m.into()).collect()),
            },
        );
        let ev = run(&doc);
        let at = format!("{at}, members {members:?}");
        assert!(
            failure(&ev, u).is_none(),
            "{at}: the union evaluates: {:?}",
            failure(&ev, u)
        );
        let named = within_sources(&ev, u, table(&ev, u), notched, out(&doc, notched), &at);
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

/// **A cavity the notched prism cuts through a slotted box**: the
/// prism is the subtract's tool, so its rim pieces reach the result
/// through the graft, and the slot kills the stretch of the first piece
/// inside it. The pieces either side are named for the first piece.
#[test]
fn a_cavity_cut_by_the_notched_prism_names_its_rim_pieces() {
    let (doc, notched, root) = notched_prism(Rim::Arc);
    let (doc, boxed) = block_on(doc, root, (-1.5, 1.5), (-1.5, 1.5), 0.5, 1.0);
    let (doc, slot) = block_on(doc, root, (0.6, 0.8), (0.4, 1.2), 0.4, 1.2);
    let (doc, holed) = insert(
        doc,
        Node::Subtract {
            from: boxed.into(),
            tool: slot.into(),
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: holed.into(),
            tool: notched.into(),
        },
    );
    let ev = run(&doc);
    assert!(
        failure(&ev, cut).is_none(),
        "the cavity evaluates: {:?}",
        failure(&ev, cut)
    );
    let named = within_sources(
        &ev,
        cut,
        table(&ev, cut),
        notched,
        out(&doc, notched),
        "cavity",
    );
    assert!(
        named >= 3,
        "the top rim survives in at least three named pieces, found {named}"
    );
}

/// **The edge check pins the piece.** The first-piece union's table,
/// doctored the two ways a derivation that picked the wrong piece of
/// the notched rim publishes it, fails [`within_sources`]: an edge
/// named bare for one piece renamed for the other piece on its line;
/// and an edge that is all of its piece respelled as a piece of the
/// line, as it is when the wrong pick puts a second edge in its group.
#[test]
fn the_edge_check_refuses_a_name_for_the_other_piece() {
    let (doc, notched, root) = notched_prism(Rim::Arc);
    let (doc, post) = block_on(doc, root, (0.6, 0.8), (0.4, 1.2), 0.5, 1.0);
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![notched.into(), post.into()]),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, u).is_none(), "the union evaluates");
    let read = out(&doc, notched);
    let src = table(&ev, notched);
    // The other row on `of`'s line, where there is exactly one.
    let other = |of: &StableName| {
        let line = line_of(of);
        let rows: Vec<&StableName> = src
            .iter()
            .map(|(n, _)| n)
            .filter(|n| n.kind == EntityKind::Edge && line_of(n) == line && *n != of)
            .collect();
        match rows.as_slice() {
            [one] if src.lookup(of).is_some() => Some((*one).clone()),
            _ => None,
        }
    };
    for respell in [false, true] {
        let mut doctored = NameTable::new();
        let mut swapped = 0;
        for (name, entry) in table(&ev, u).iter() {
            let mut name = name.clone();
            if let [RoleSeg::From { read: r, of }] = name.path.as_mut_slice()
                && *r == read
                && let Some(o) = other(of)
            {
                if respell {
                    *of = NameRef::new(line_of(of));
                    name.path
                        .push(RoleSeg::Fragment(Qualifier::Ends(Vec::new())));
                } else {
                    *of = NameRef::new(o);
                }
                swapped += 1;
            }
            match entry {
                editor_core::Entry::Unique(e) => doctored.insert(name, *e).unwrap(),
                editor_core::Entry::Tied(es) => doctored.insert_tied(name, es.clone()).unwrap(),
            }
        }
        assert!(
            swapped > 0,
            "some edge is named bare for a piece of the notched rim"
        );
        let caught = std::panic::catch_unwind(|| {
            within_sources(&ev, u, &doctored, notched, read, "doctored");
        });
        assert!(
            caught.is_err(),
            "respelled {respell}: the check passes {swapped} names for the other piece"
        );
    }
}
