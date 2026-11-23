//! **The n-ary union's holdings, read back for a row**: a run in a given
//! member order, its holdings with each member face named by its place
//! in the list the row built, and the rejoin oracle that reads the same
//! holdings off each member rejoined to the finished body.

#![allow(clippy::panic, clippy::unwrap_used)]

use std::collections::{BTreeMap, BTreeSet};

use geom_core::Tol;

use crate::AtRestBody;
use crate::boolean::{
    BooleanResult, Held, MemberFace, MemberInputs, NaryEnd, NaryMember, NaryOp, Operand,
    SweepStrategy, nary_boolean,
};
use crate::entity::FaceKey;

/// What each finished face holds, each member face by its place in the
/// list the row built.
pub type Table = BTreeMap<FaceKey, BTreeSet<MemberFace>>;

/// **The union of `bodies` in `order`**, each member ranked by its place
/// in `bodies`: the finished body, what each of its faces holds, and
/// every member face some step's discarded face held.
///
/// # Panics
///
/// Where the union does not build, or any holding refuses.
pub fn union_in(
    bodies: &[AtRestBody<f64>],
    order: &[usize],
    tol: Tol,
) -> (AtRestBody<f64>, Table, BTreeSet<MemberFace>) {
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
    let run = nary_boolean(NaryOp::Union, &members, SweepStrategy::Realized, tol);
    assert!(
        matches!(run.end, NaryEnd::Built),
        "order {order:?}: {:?}",
        run.end
    );
    let read = |held: &Held, what: String| -> BTreeSet<MemberFace> {
        match held {
            Held::Faces(f) => f.iter().map(|&(m, k)| (order[m], k)).collect(),
            Held::Refused(r) => panic!("order {order:?}: {what} refused its holdings: {r:?}"),
        }
    };
    let mut lost = BTreeSet::new();
    for step in &run.steps {
        for (row, held) in step.holdings.lost_rows().iter().enumerate() {
            lost.extend(read(
                held,
                format!("step {}'s discard row {row}", step.member),
            ));
        }
    }
    let step = run.steps.into_iter().last().unwrap();
    let table = step
        .holdings
        .faces()
        .map(|(f, held)| (f, read(held, format!("face {f:?}"))))
        .collect();
    (step.body.body, table, lost)
}

/// **The rejoin oracle**: each member rejoined to the finished body,
/// wherever that builds, holds through its substitution rows exactly the
/// member faces `table` says each finished face holds of it. Returns how
/// many rejoins built; with `all`, every one must.
///
/// # Panics
///
/// Where a rejoin that builds reads otherwise, or, with `all`, one does
/// not build.
pub fn rejoin_oracle(
    bodies: &[AtRestBody<f64>],
    order: &[usize],
    result: &AtRestBody<f64>,
    table: &Table,
    (all, tol): (bool, Tol),
) -> usize {
    let mut built = 0;
    for (i, member) in bodies.iter().enumerate() {
        let rejoin = match crate::union(result, member, tol) {
            Ok(BooleanResult::Body(rj)) => rj,
            other => {
                assert!(
                    !all,
                    "order {order:?}: rejoining member {i} builds: {other:?}"
                );
                continue;
            }
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
            "order {order:?}: what each finished face holds of member {i} is what its rejoin \
             substitutes"
        );
    }
    built
}
