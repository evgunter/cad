//! **A union over a union publishes FLAT sets** (N3,
//! `crates/editor-core/src/names/README.md`): a merged face or a joined
//! edge of the outer union that takes in one of the inner union's lists
//! the inner set's constituents, each re-wrapped as the outer union's
//! `From(inner's read, c)`, never `From(inner's read, Merged{…})` as one
//! constituent — in every member order of both unions.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use crate::blocks::{block_on, failure, run};
use crate::emit_shared_rim_several::{Bx, document, permutations};
use crate::emit_union_rim_piece_ranks::signature;
use crate::fixture::{insert, out, table};

use editor_core::{EntityKind, Node, RoleSeg, StableName, VarId};

/// `a` and `b` overlap and are flush in the inner union, so its
/// two y-walls and two caps are merged faces and its four x-running rims
/// joined edges; `c` overlaps `b` past `a`'s end and is flush with
/// those merged faces in the outer union. `g`, a pillar standing
/// across the top's y = 0 rim past `b`'s end, is in the outer union too:
/// it notches the top and that wall without dividing either, and crosses
/// the rim, so the fold meets a joined edge of all three blocks where
/// `g` crosses it and reads its sense along it.
const A: Bx = ((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
const B: Bx = ((0.5, 1.5), (0.0, 1.0), (0.0, 1.0));
const C: Bx = ((1.0, 2.0), (0.0, 1.0), (0.0, 1.0));
const G: Bx = ((1.6, 1.7), (-0.5, 0.5), (0.5, 3.0));

/// Whether `n`, read through every `From` wrapper, is a bare merged
/// name: the constituent N3 forbids.
fn bare_merge_through_wrappers(n: &StableName) -> bool {
    match n.path.as_slice() {
        [RoleSeg::Merged(_)] => true,
        [RoleSeg::From { of: inner, .. }] => bare_merge_through_wrappers(inner),
        _ => false,
    }
}

/// One order's label, the outer union's signature, the member reads and
/// the inner union's read.
type Order = (String, BTreeMap<StableName, String>, Vec<VarId>, VarId);

/// Every order of both unions, the member ids being `[a, b, c, g]`.
/// `c` and `g` are made after the inner union, so its member-keyed names
/// order before theirs and a set's reader meets its constituents first.
fn orders() -> Vec<Order> {
    let (doc, ab) = document(&[A, B], &[0, 1]);
    let mut found = Vec::new();
    for inner in permutations(&[0, 1]) {
        let io: Vec<_> = inner.iter().map(|&i| ab[i]).collect();
        let (d1, u1) = crate::fixture::union_over(doc.clone(), &io);
        let ((cx, cy, cz), (gx, gy, gz)) = (C, G);
        let (d1, c) = block_on(d1, ab[0], cx, cy, cz.0, cz.1);
        let (d1, g) = block_on(d1, ab[0], gx, gy, gz.0, gz.1);
        let ids: Vec<VarId> = [ab[0], ab[1], c, g]
            .into_iter()
            .map(|m| out(&d1, m))
            .collect();
        let u1_read = out(&d1, u1);
        let ev1 = run(&d1);
        assert!(
            failure(&ev1, u1).is_none(),
            "{inner:?}: the inner union refused"
        );
        for outer in permutations(&[0, 1, 2]) {
            let members: Vec<_> = outer.iter().map(|&i| [u1, c, g][i]).collect();
            let (docx, top) = crate::fixture::union_over(d1.clone(), &members);
            let ev = run(&docx);
            let at = format!("{inner:?}{outer:?}");
            assert!(
                failure(&ev, top).is_none(),
                "{at}: the outer union refused: {:?}",
                failure(&ev, top)
            );
            found.push((at, signature(&ev, top), ids.clone(), u1_read));
        }
    }
    found
}

/// **The outer union's merged faces and joined edges list faces and
/// edges, never the inner union's sets, and its table is one table in
/// every order.** Each of the four flush families is one parent face of
/// the outer union, `Merged` of `a`'s, `b`'s and `c`'s faces, the first
/// two through the inner union; each of the four x-running rims is one
/// set, `Merged` of the three blocks' rims, so (`g` cuts the y = 0 top
/// rim, and its piece along all three blocks is a piece of that set).
///
/// Red while a member's `From` is not read through: those sets list
/// `From(inner's read, Merged{a's, b's})` and `c`'s, two constituents.
#[test]
fn a_union_over_a_union_publishes_flat_sets_in_every_order() {
    let orders = orders();
    assert_eq!(orders.len(), 12, "two inner orders by six outer orders");
    for (at, sig, ids, u1) in &orders {
        let mut sets = BTreeMap::<EntityKind, BTreeSet<&[StableName]>>::new();
        for name in sig.keys() {
            for seg in &name.path {
                let RoleSeg::Merged(set) = seg else { continue };
                for c in set {
                    assert!(
                        !bare_merge_through_wrappers(c),
                        "{at}: {name:?} lists a merged name as one constituent: {c:?}"
                    );
                }
                let of = |m: VarId| {
                    set.iter()
                        .filter(|c| matches!(c.path.as_slice(), [RoleSeg::From { read, .. }] if *read == m))
                        .count()
                };
                if set.len() == 3 && of(*u1) == 2 && of(ids[2]) == 1 {
                    sets.entry(name.kind).or_default().insert(set);
                }
            }
        }
        let count = |kind| sets.get(&kind).map_or(0, BTreeSet::len);
        assert_eq!(
            count(EntityKind::Face),
            4,
            "{at}: the four flush families are each one face of all three blocks"
        );
        assert_eq!(
            count(EntityKind::Edge),
            4,
            "{at}: the four x-running rims are each one edge set of all three blocks"
        );
    }
    let (first_at, first, ..) = &orders[0];
    for (at, sig, ..) in &orders[1..] {
        assert_eq!(first, sig, "{first_at} against {at}");
    }
}

/// **A union over a two-member union publishes flat sets too.** `a`
/// and `b` in a two-member `Union`, its merged faces and joined
/// rims `Merged{From(a's read, …), From(b's read, …)}`, and that union
/// in a union with `c` flush with them: in both member orders,
/// each of the four flush families is one face set and each x-running
/// rim one edge set of all three blocks, listing `a`'s and `b`'s through
/// the inner union, never `From(inner's read, Merged{…})` as one
/// constituent, and the table is one table.
#[test]
fn a_union_over_a_pair_boolean_publishes_flat_sets_in_both_orders() {
    let (doc, ab) = document(&[A, B], &[0, 1]);
    let (d1, inner) = insert(
        doc.clone(),
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![ab[0].into(), ab[1].into()]),
        },
    );
    let (cx, cy, cz) = C;
    let (d1, c) = block_on(d1, ab[0], cx, cy, cz.0, cz.1);
    let ev1 = run(&d1);
    assert!(failure(&ev1, inner).is_none(), "the pair boolean refused");
    let (inner_read, c_read) = (out(&d1, inner), out(&d1, c));
    let mut sigs = Vec::new();
    for members in [[inner, c], [c, inner]] {
        let (docx, top) = crate::fixture::union_over(d1.clone(), &members);
        let ev = run(&docx);
        let at = format!("{members:?}");
        assert!(
            failure(&ev, top).is_none(),
            "{at}: the union refused: {:?}",
            failure(&ev, top)
        );
        let sig = signature(&ev, top);
        let mut sets = BTreeMap::<EntityKind, BTreeSet<&[StableName]>>::new();
        for name in sig.keys() {
            for seg in &name.path {
                let RoleSeg::Merged(set) = seg else { continue };
                for k in set {
                    assert!(
                        !bare_merge_through_wrappers(k),
                        "{at}: {name:?} lists a merged name as one constituent: {k:?}"
                    );
                }
                let of = |m: VarId| {
                    set.iter()
                        .filter(|k| matches!(k.path.as_slice(), [RoleSeg::From { read, .. }] if *read == m))
                        .count()
                };
                if set.len() == 3 && of(inner_read) == 2 && of(c_read) == 1 {
                    sets.entry(name.kind).or_default().insert(set);
                }
            }
        }
        let count = |kind| sets.get(&kind).map_or(0, BTreeSet::len);
        assert_eq!(
            count(EntityKind::Face),
            4,
            "{at}: four face sets of all three blocks"
        );
        assert_eq!(
            count(EntityKind::Edge),
            4,
            "{at}: four edge sets of all three blocks"
        );
        sigs.push(sig.clone());
    }
    assert_eq!(sigs[0], sigs[1], "the two member orders publish one table");
}

/// Whether `n`, read through every descent wrapper a set is read
/// through (`merged::peel`: an operand's `From`, whether a boolean's
/// member or a one-operand door's target, and the shell's `Inner`), is
/// a bare merged name.
fn bare_merge_through_every_wrapper(n: &StableName) -> bool {
    match n.path.as_slice() {
        [RoleSeg::Merged(_)] => true,
        [RoleSeg::From { of: inner, .. } | RoleSeg::Inner(inner)] => {
            bare_merge_through_every_wrapper(inner)
        }
        _ => false,
    }
}

/// The edge of `body` whose two ends both sit at `(x, y)` in plan: a
/// vertical edge of a block.
fn vertical_edge_at(body: &topo::Body<f64>, x: f64, y: f64) -> topo::EdgeKey {
    let at: BTreeMap<_, _> = body.vertex_points().collect();
    let on = |v| {
        let p: &geom_core::Point3<f64> = &at[&v];
        (p.x - x).abs() < 1e-9 && (p.y - y).abs() < 1e-9
    };
    let found: Vec<_> = body
        .edges()
        .filter(|(_, e)| {
            [e.he_plus, e.he_minus]
                .iter()
                .all(|h| on(body.get_half_edge(*h).expect("a live half-edge").start))
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(found.len(), 1, "one vertical edge at ({x}, {y})");
    found[0]
}

/// **A union over a filleted body whose rims are already sets lists
/// edges, never sets** (N3; `merged::peel` reads a blend's target
/// `From` as it reads a boolean's wrappers). The inner union of `a` and
/// `b` joins its four x-running rims into sets; a fillet on the vertical
/// edge at `a`'s `(0, 0)` corner trims the two y = 0 rims and carries
/// the two y = 1 rims untouched, as `From(target read, Merged{a's,
/// b's})`. The outer union over the fillet and `c`, flush with the
/// carried merged faces, joins each y = 1 rim with `c`'s: one set of
/// the three blocks' rims, `a`'s and `b`'s each read out through
/// `From(fillet's read, From(target read, …))`.
///
/// Red while the target's `From` is not peeled: those sets list
/// `From(fillet's read, From(target read, Merged{a's, b's}))` and `c`'s,
/// two constituents.
#[test]
fn a_union_over_a_filleted_body_whose_rims_are_sets_publishes_flat_sets() {
    let (doc, ab) = document(&[A, B], &[0, 1]);
    let (d1, u1) = crate::fixture::union_over(doc.clone(), &ab);
    let ev1 = run(&d1);
    assert!(failure(&ev1, u1).is_none(), "the inner union refused");
    let corner = vertical_edge_at(&(crate::corpus::body_of(&ev1, u1)), 0.0, 0.0);
    let corner_name = table(&ev1, u1)
        .name_of(&editor_core::EntityRef {
            body: 0,
            key: editor_core::EntityKey::Edge(corner),
        })
        .expect("the corner is named")
        .clone();
    let (d1, fillet) = insert(
        d1,
        Node::Fillet {
            radius: crate::fixture::len(0.1),
            selection: editor_core::Operand::select(u1, vec![corner_name]),
        },
    );
    let ((cx, cy, cz), _) = (C, G);
    let (d1, c) = block_on(d1, ab[0], cx, cy, cz.0, cz.1);
    let ev1 = run(&d1);
    assert!(failure(&ev1, fillet).is_none(), "the fillet refused");
    let (u1_read, fillet_read) = (out(&d1, u1), out(&d1, fillet));
    let carried_rims = table(&ev1, fillet)
        .iter()
        .filter(|(n, _)| {
            n.kind == EntityKind::Edge
                && matches!(n.path.as_slice(), [RoleSeg::From { read, of: inner }]
                    if *read == u1_read && matches!(inner.path.as_slice(), [RoleSeg::Merged(_)]))
        })
        .count();
    assert_eq!(carried_rims, 2, "the y = 1 rims are carried as sets");
    for members in [[fillet, c], [c, fillet]] {
        let (docx, top) = crate::fixture::union_over(d1.clone(), &members);
        let ev = run(&docx);
        assert!(
            failure(&ev, top).is_none(),
            "{members:?}: the outer union refused: {:?}",
            failure(&ev, top)
        );
        let mut three = 0;
        for (name, _) in table(&ev, top).iter() {
            for seg in &name.path {
                let RoleSeg::Merged(set) = seg else { continue };
                for k in set {
                    assert!(
                        !bare_merge_through_every_wrapper(k),
                        "{members:?}: {name:?} lists a merged name as one constituent: {k:?}"
                    );
                }
                let through_fillet = set
                    .iter()
                    .filter(|k| {
                        matches!(k.path.as_slice(),
                        [RoleSeg::From { read, of }]
                            if *read == fillet_read
                                && matches!(of.path.as_slice(), [RoleSeg::From { read: target, .. }] if *target == u1_read))
                    })
                    .count();
                if name.kind == EntityKind::Edge && set.len() == 3 && through_fillet == 2 {
                    three += 1;
                }
            }
        }
        assert_eq!(
            three, 2,
            "{members:?}: each carried rim and `c`'s is one edge set of all three blocks"
        );
    }
}
