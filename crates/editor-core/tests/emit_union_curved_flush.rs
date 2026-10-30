//! **A union names a curved flush stretch from the finished body, in
//! every member order** (`emit_union::Flush`, `names/README.md`, "Flush
//! edges at a union").
//!
//! `h` is the half disc over the unit arc from 0° to 180°, and `q` the
//! half disc over the arc from 90° to 270°, both extruded z ∈ [0, 1].
//! Their arc walls lie on one cylinder and overlap over 90°–180°, so
//! each rim arc of `h` runs flush with `q`'s over that stretch. `w` is a
//! wall y ∈ [0.9, 2] standing through the top rim, which it crosses at
//! about 64° and 116°.
//!
//! - `h` and `q` alone: the output stage joins each rim into one arc
//!   from 0° to 270°, within neither member's arc and along both, so it
//!   is the `Merged` set of the two.
//! - With `w`: the top rim is cut at both crossings. The piece from 0°
//!   to 64° lies within `h`'s arc alone, the piece from 116° to 270°
//!   within `q`'s alone (and along `h`'s for part of it), and the
//!   crossing at 116° is `q`'s arc crossed by `w`'s wall.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::blocks::{block_on, failure, run};
use crate::corpus::body_of;
use crate::emit_shared_rim_several::permutations;
use crate::emit_union_rim_piece_ranks::signature;
use crate::fixture::{insert, len, len2, member_of, slid, table, union_over};

use editor_core::{
    EntityKey, EntityKind, Entry, Evaluation, Formula, LoopProgram, Node, ProfileDoc,
    ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, Qualifier, RecipeNodeId, RoleSeg,
    StableName,
};
use geom_core::{Point3, Tol};

/// The point at `deg` degrees on the unit circle in the plane z.
fn on_circle(deg: f64, z: f64) -> Point3<f64> {
    let r = deg.to_radians();
    Point3::new(r.cos(), r.sin(), z)
}

/// The half disc over the unit arc from `a0` through `am` to `a1`
/// degrees, closed by its diameter, extruded z ∈ [0, 1].
fn half_disc(doc: ProfileDoc, a0: f64, am: f64, a1: f64) -> (ProfileDoc, RecipeNodeId) {
    let pt = |a: f64| {
        let p = on_circle(a, 0.0);
        len2([p.x, p.y])
    };
    let steps: Vec<ProgramStep<Formula>> = vec![
        ProgramStep::At(pt(a0)),
        ProgramStep::ArcTo(ProgramArcData::Via {
            q: pt(am),
            target: ProgramTarget::Point(pt(a1)),
        }),
        ProgramStep::LineTo(ProgramTarget::Start),
    ];
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

/// `h`, `q` and `w`, created in that order: `q` and `w` copies placed
/// against `h`.
fn document() -> (ProfileDoc, [RecipeNodeId; 3]) {
    let doc = ProfileDoc::empty_derived("emit_union_curved_flush", Tol::witness());
    let (doc, h) = half_disc(doc, 0.0, 90.0, 180.0);
    let (doc, q) = half_disc(doc, 90.0, 180.0, 270.0);
    let (doc, q) = slid(doc, h, q, [0.0; 3]);
    let (doc, w) = block_on(doc, h, (-2.0, 2.0), (0.9, 2.0), 0.5, 1.0);
    (doc, [h, q, w])
}

/// One evaluated union: its member order, the document and the
/// evaluation, and the union.
type Run = (String, ProfileDoc, Evaluation<f64>, RecipeNodeId);

/// The union of `members` in every order, each evaluated.
fn every_order(doc: &ProfileDoc, members: &[RecipeNodeId]) -> Vec<Run> {
    let idx: Vec<usize> = (0..members.len()).collect();
    permutations(&idx)
        .into_iter()
        .map(|order| {
            let ids: Vec<RecipeNodeId> = order.iter().map(|&i| members[i]).collect();
            let (doc, union) = union_over(doc.clone(), &ids);
            let ev = run(&doc);
            let at = format!("order {order:?}");
            assert!(
                failure(&ev, union).is_none(),
                "{at}: {:?}",
                failure(&ev, union)
            );
            (at, doc, ev, union)
        })
        .collect()
}

/// Every order's published table is the first order's, name for name
/// and entity for entity ([`signature`]).
fn one_table(runs: &[Run]) {
    let [(first_at, _, first_ev, first_union), rest @ ..] = runs else {
        panic!("no orders");
    };
    let first = signature(first_ev, *first_union);
    for (at, _, ev, union) in rest {
        assert_eq!(first, signature(ev, *union), "{first_at} against {at}");
    }
}

/// The path of a member's entity `of` in its construction, which a
/// placed copy carries verbatim.
fn own_path(of: &StableName) -> &[RoleSeg] {
    of.path.as_slice()
}

/// Whether `name` is a member's rim edge carried bare.
fn is_rim(name: &StableName) -> bool {
    match name.path.as_slice() {
        [RoleSeg::From { of, .. }] => matches!(own_path(of), [RoleSeg::RimEdge(..)]),
        _ => false,
    }
}

/// The published arcs of the unit circle at height `z`, by name: each
/// its two ends' azimuths in degrees, least first.
fn rim_arcs(ev: &Evaluation<f64>, union: RecipeNodeId, z: f64) -> BTreeMap<StableName, (f64, f64)> {
    let body = body_of(ev, union);
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let on = |p: Point3<f64>| (p.z - z).abs() < 1e-9 && (p.x.hypot(p.y) - 1.0).abs() < 1e-9;
    let az = |p: Point3<f64>| p.y.atan2(p.x).to_degrees().rem_euclid(360.0);
    let mut out = BTreeMap::new();
    for (name, entry) in table(ev, union).iter() {
        let Entry::Unique(e) = entry else { continue };
        let EntityKey::Edge(k) = e.key else { continue };
        if topo::query::edge_carrier_kind(&body, k) != Some(topo::query::CurveKind::Circle) {
            continue;
        }
        let edge = body.get_edge(k).unwrap();
        let [p0, p1] =
            [edge.he_plus, edge.he_minus].map(|he| point(body.get_half_edge(he).unwrap().start));
        if on(p0) && on(p1) {
            let (a, b) = (az(p0), az(p1));
            out.insert(name.clone(), (a.min(b), a.max(b)));
        }
    }
    out
}

/// **Two arcs flush over part of their length join into one rim named
/// for both, in either order.** Each rim of `h ∪ q` is one arc from 0°
/// to 270°, published as `Merged` of `h`'s rim arc and `q`'s, and the
/// two orders publish one table.
#[test]
fn a_rim_joined_along_two_flush_arcs_is_named_for_both_in_either_order() {
    let (doc, [h, q, _]) = document();
    let runs = every_order(&doc, &[h, q]);
    one_table(&runs);
    for (at, doc, ev, union) in &runs {
        for z in [0.0, 1.0] {
            let rims = rim_arcs(ev, *union, z);
            let [(name, &(a, b))] = rims.iter().collect::<Vec<_>>()[..] else {
                panic!("{at} z = {z}: one rim arc, got {rims:?}");
            };
            assert!(
                a.abs() < 1e-6 && (b - 270.0).abs() < 1e-6,
                "{at} z = {z}: the rim runs from 0° to 270°, got {a}°–{b}°"
            );
            let [RoleSeg::Merged(set)] = name.path.as_slice() else {
                panic!("{at} z = {z}: the joined rim is a set, got {name:?}");
            };
            let held: Vec<_> = set.iter().map(|c| (member_of(doc, c), is_rim(c))).collect();
            assert_eq!(
                held,
                vec![(Some(h), true), (Some(q), true)],
                "{at} z = {z}: the set is h's rim arc and q's, got {set:?}"
            );
        }
    }
}

/// **A wall across the shared rim leaves each piece named for the arc
/// it lies within, and the crossing on the flush stretch for that arc,
/// in every order.** The top rim's piece from 0° to 64° is a piece of
/// `h`'s arc, the piece from 116° to 270° a piece of `q`'s, and the
/// vertex at 116° is the `Crossing` of `q`'s arc and `w`'s wall.
#[test]
fn a_wall_across_two_flush_arcs_publishes_one_table_in_every_order() {
    let (doc, [h, q, w]) = document();
    let runs = every_order(&doc, &[h, q, w]);
    assert_eq!(runs.len(), 6, "every order of three members");
    one_table(&runs);
    let cross = 0.9f64.asin().to_degrees();
    let deg = |x: f64| x.round() as i64;
    for (at, doc, ev, union) in &runs {
        let piece_of = |name: &StableName| match name.path.as_slice() {
            [
                RoleSeg::From { of, .. },
                RoleSeg::Fragment(Qualifier::Ends(_)),
            ] if matches!(own_path(of), [RoleSeg::RimEdge(..)]) => member_of(doc, name),
            _ => None,
        };
        let mut pieces: Vec<_> = rim_arcs(ev, *union, 1.0)
            .iter()
            .map(|(n, &(a, b))| ((deg(a), deg(b)), piece_of(n)))
            .collect();
        pieces.sort();
        assert_eq!(
            pieces,
            vec![
                ((0, deg(cross)), Some(h)),
                ((deg(180.0 - cross), 270), Some(q)),
            ],
            "{at}: the top rim's two pieces, each a piece of the arc it lies within"
        );
        let body = body_of(ev, *union);
        let crossing = table(ev, *union)
            .iter()
            .filter(|(n, _)| n.kind == EntityKind::Vertex)
            .find_map(|(n, entry)| {
                let Entry::Unique(e) = entry else { return None };
                let EntityKey::Vertex(v) = e.key else {
                    return None;
                };
                let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                ((p - on_circle(180.0 - cross, 1.0)).norm() < 1e-9).then(|| n.clone())
            })
            .unwrap_or_else(|| panic!("{at}: no vertex at the crossing on the flush stretch"));
        let [RoleSeg::Crossing { edge, face, .. }] = crossing.path.as_slice() else {
            panic!("{at}: the crossing on the flush stretch is one Crossing, got {crossing:?}");
        };
        assert!(
            is_rim(edge) && member_of(doc, edge) == Some(q),
            "{at}: the crossing cites q's rim arc, got {edge:?}"
        );
        assert_eq!(
            member_of(doc, face),
            Some(w),
            "{at}: the crossing face is w's wall, got {face:?}"
        );
    }
}
