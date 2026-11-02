//! **A union's holdings over a curved run only several members open**:
//! a plate, a boss sunk through its top whose wall is two half-cylinders,
//! and a bar over the boss. The bar cuts the wall the plate leaves into
//! two pieces, each holding area of both half-cylinders, which no fold
//! step glued by itself (`topo::Holdings`, `topo::overlap`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use geom_core::{Point2, Tol};
use sweep::test_support::{brick, cylinder_of_arcs_at, finished};
use topo::{
    AtRestBody, Body, BooleanResult, FaceKey, Held, MemberInputs, NaryEnd, NaryMember, NaryOp,
    Operand, Surface, SweepStrategy,
};

fn tol() -> Tol {
    Tol::witness()
}

fn curved(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| !matches!(body.get_surface(f.surface), Some(Surface::Plane { .. })))
        .map(|(k, _)| k)
        .collect()
}

type Table = BTreeMap<FaceKey, BTreeSet<(usize, FaceKey)>>;

/// The union of `bodies` in `order`: the finished body and what each of
/// its faces holds, each member face read back to its place in
/// `bodies`.
fn union_in(bodies: &[AtRestBody<f64>], order: &[usize]) -> (AtRestBody<f64>, Table) {
    let inputs: Vec<MemberInputs> = order
        .iter()
        .map(|&i| MemberInputs {
            rank: u32::try_from(i).unwrap(),
            ..MemberInputs::default()
        })
        .collect();
    let members: Vec<NaryMember<'_, f64>> = order
        .iter()
        .zip(&inputs)
        .map(|(&i, inputs)| NaryMember {
            body: &bodies[i],
            inputs,
        })
        .collect();
    let run = topo::nary_boolean(NaryOp::Union, &members, SweepStrategy::Realized, tol());
    assert!(matches!(run.end, NaryEnd::Built), "order {order:?}: {:?}", run.end);
    let step = run.steps.into_iter().last().unwrap();
    let table = step
        .holdings
        .faces()
        .map(|(f, held)| {
            let Held::Faces(set) = held else {
                panic!("order {order:?}: face {f:?} refused its holdings: {held:?}");
            };
            (f, set.iter().map(|&(m, k)| (order[m], k)).collect())
        })
        .collect();
    (step.body.body, table)
}

/// Each member rejoined to the finished body, wherever that builds:
/// what its substitution rows say each finished face holds of it must
/// be what the holdings say. Returns how many rejoins built.
fn rejoin_oracle(
    bodies: &[AtRestBody<f64>],
    order: &[usize],
    result: &AtRestBody<f64>,
    table: &Table,
) -> usize {
    let mut built = 0;
    for (i, member) in bodies.iter().enumerate() {
        let Ok(BooleanResult::Body(rejoin)) = topo::union(result, member, tol()) else {
            continue;
        };
        built += 1;
        let n = &rejoin.naming;
        let read: BTreeSet<(FaceKey, FaceKey)> = n
            .substituted
            .iter()
            .map(|s| {
                let (a, b) = match s.operand {
                    Operand::A => (s.face, s.by),
                    Operand::B => (s.by, s.face),
                };
                (n.face_fragments_a.root(a), n.face_fragments_b.root(b))
            })
            .collect();
        let holds: BTreeSet<(FaceKey, FaceKey)> = table
            .iter()
            .flat_map(|(&f, set)| {
                set.iter()
                    .filter(move |(m, _)| *m == i)
                    .map(move |&(_, k)| (f, k))
            })
            .collect();
        assert_eq!(
            holds, read,
            "order {order:?}: what the finished faces hold of member {i} is what its rejoin \
             substitutes"
        );
    }
    built
}

/// **Every member order reads one holdings table**, in which each piece
/// of the boss's wall holds both half-cylinders; the rejoin of each
/// member that builds reads the same (the plate's refuses: it lies on
/// the finished boundary and never crosses it).
#[test]
fn a_bar_over_a_sunk_boss_holds_both_wall_halves_in_every_order() {
    let t = tol();
    let bodies = [
        finished("plate", brick((0.0, 3.0), (0.0, 3.0), (0.0, 1.0), t), t),
        finished(
            "boss",
            cylinder_of_arcs_at(2, 0.5, Point2::new(1.5, 1.5), 0.5, 1.0, t),
            t,
        ),
        finished("bar", brick((1.4, 1.6), (0.5, 2.5), (0.8, 2.0), t), t),
    ];
    let walls: BTreeSet<(usize, FaceKey)> = curved(&bodies[1]).into_iter().map(|k| (1, k)).collect();
    assert_eq!(walls.len(), 2, "the boss's wall is two half-cylinders");
    let mut tables = BTreeSet::new();
    for order in [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]] {
        let (result, table) = union_in(&bodies, &order);
        let pieces = curved(&result);
        assert_eq!(pieces.len(), 2, "order {order:?}: the bar cuts the wall in two");
        for f in &pieces {
            assert_eq!(
                table[f], walls,
                "order {order:?}: wall piece {f:?} holds both half-cylinders"
            );
        }
        let built = rejoin_oracle(&bodies, &order, &result, &table);
        assert!(built >= 2, "order {order:?}: the boss's and the bar's rejoins build");
        let mut rows: Vec<_> = table.into_values().collect();
        rows.sort();
        tables.insert(rows);
    }
    assert_eq!(tables.len(), 1, "one holdings table in every member order");
}
