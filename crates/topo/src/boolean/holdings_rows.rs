//! Rows for the booleans' [`Holdings`] and the [`overlap`] door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use geom_core::{Band, Tol};

use super::holdings::{Held, Holdings};
use super::nary::{MemberInputs, NaryEnd, NaryMember, NaryOp, nary_boolean};
use super::overlap::{OverlapRefusal, overlap};
use super::{BooleanResult, Operand, SweepStrategy};
use crate::AtRestBody;
use crate::body::Body;
use crate::entity::FaceKey;
use crate::test_support::meeting::orders;
use crate::test_support::{brick, finished, prism_z};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// The top face of a prism over `profile`, z 0 to 1, in its own body.
fn top(profile: &[(f64, f64)]) -> (Body<f64>, FaceKey) {
    let p = prism_z::<f64>(profile, 0.0, 1.0, tol());
    (p.body, p.top_face)
}

fn rect(x: (f64, f64), y: (f64, f64)) -> Vec<(f64, f64)> {
    vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]
}

/// The corners of a regular polygon about the origin, radius 1, at
/// every `step`-th of `n` angles, counterclockwise.
fn regular(n: usize, step: usize) -> Vec<(f64, f64)> {
    (0..n)
        .step_by(step)
        .map(|i| {
            let a = core::f64::consts::TAU * i as f64 / n as f64;
            (a.cos(), a.sin())
        })
        .collect()
}

fn overlaps(x: &[(f64, f64)], y: &[(f64, f64)]) -> Result<bool, OverlapRefusal> {
    let (bx, fx) = top(x);
    let (by, fy) = top(y);
    let there = overlap((&bx, fx), (&by, fy), band())?;
    let back = overlap((&by, fy), (&bx, fx), band())?;
    assert_eq!(there, back, "overlap reads alike in either order");
    Ok(there)
}

/// **`overlap` finds each kind of witness, and none where the regions
/// share no area.**
#[test]
fn overlap_reads_each_witness_kind_and_no_area_as_false() {
    let yes = |what: &str, x: Vec<(f64, f64)>, y: Vec<(f64, f64)>| {
        assert!(overlaps(&x, &y).unwrap(), "{what}: the faces meet in area");
    };
    let no = |what: &str, x: Vec<(f64, f64)>, y: Vec<(f64, f64)>| {
        assert!(
            !overlaps(&x, &y).unwrap(),
            "{what}: the faces share no area"
        );
    };
    yes(
        "a vertex strictly inside",
        rect((0.0, 2.0), (0.0, 2.0)),
        rect((1.0, 3.0), (1.0, 3.0)),
    );
    yes(
        "crossings alone: a cross, no vertex of either inside the other",
        rect((0.0, 3.0), (1.0, 2.0)),
        rect((1.0, 2.0), (0.0, 3.0)),
    );
    yes(
        "a triangle inscribed in a hexagon: no vertex inside, no crossing",
        regular(6, 2),
        regular(6, 1),
    );
    yes(
        "a piece along its source face's edges from inside",
        rect((0.0, 0.8), (0.0, 2.0)),
        rect((0.0, 1.0), (0.0, 2.0)),
    );
    yes(
        "one face",
        rect((0.0, 1.0), (0.0, 1.0)),
        rect((0.0, 1.0), (0.0, 1.0)),
    );
    no(
        "apart",
        rect((0.0, 1.0), (0.0, 1.0)),
        rect((2.0, 3.0), (0.0, 1.0)),
    );
    no(
        "touching at a corner",
        rect((0.0, 1.0), (0.0, 1.0)),
        rect((1.0, 2.0), (1.0, 2.0)),
    );
    no(
        "sharing an edge from either side",
        rect((0.0, 1.0), (0.0, 1.0)),
        rect((1.0, 2.0), (0.0, 1.0)),
    );
    no(
        "a vertex on the other's edge, from outside",
        rect((0.0, 1.0), (0.0, 1.0)),
        vec![(1.0, 0.5), (2.0, 0.0), (2.0, 1.0)],
    );
}

/// **A margin in band refuses typed**: a face whose vertex lies in the
/// band off the other's edge, neither on it nor clear of it.
#[test]
fn overlap_refuses_a_vertex_in_band_of_the_others_edge() {
    let t = tol();
    let gap = t.eps() * (1.0 + t.k()) / 2.0;
    let got = overlaps(
        &rect((0.0, 1.0), (0.0, 1.0)),
        &rect((1.0 + gap, 2.0), (0.0, 1.0)),
    );
    assert!(
        matches!(got, Err(OverlapRefusal::Placement(_))),
        "a vertex {gap} off the other face's edge is in band: {got:?}"
    );
}

/// What `held` holds, refusing the row where it could not be read.
fn faces(held: &Held, what: &str) -> BTreeSet<(usize, FaceKey)> {
    match held {
        Held::Faces(f) => f.clone(),
        Held::Refused(r) => panic!("{what} refused its holdings: {r:?}"),
    }
}

/// The face of `body` on the plane z = `z` facing +z.
fn tops_at(body: &Body<f64>, z: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(crate::Surface::Plane { origin, normal, .. })
                    if normal.z * if f.sense { 1.0 } else { -1.0 } > 0.5
                        && (origin.z - z).abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .collect()
}

/// **The pair door reads a sunk coplanar tile's top as held through the
/// substitution**: the union of a brick and a tile sunk under its top
/// has one top face, which holds both tops, A's and B's.
#[test]
fn the_pair_door_holds_a_sunk_tiles_top_through_its_substitution() {
    let t = tol();
    let p = finished("P", brick::<f64>((0.0, 1.0), (0.0, 2.0), (0.0, 1.0), t), t);
    let tile = finished("T", brick::<f64>((0.2, 0.6), (0.5, 1.5), (0.5, 1.0), t), t);
    let Ok(BooleanResult::Body(out)) = super::union(&p, &tile, t) else {
        panic!("the union builds");
    };
    assert!(
        out.naming
            .substituted
            .iter()
            .any(|s| s.operand == Operand::B
                && tops_at(&tile, 1.0).contains(&out.naming.face_fragments_b.root(s.face))),
        "the tile's top reaches the result through a substitution row"
    );
    let finished_tops = tops_at(&out.body, 1.0);
    assert_eq!(finished_tops.len(), 1, "one top face");
    let expected = BTreeSet::from([(0, tops_at(&p, 1.0)[0]), (1, tops_at(&tile, 1.0)[0])]);
    assert_eq!(
        faces(out.holdings.of(finished_tops[0]).unwrap(), "the top"),
        expected,
        "the top holds the brick's top and the tile's"
    );
    for (f, held) in out.holdings.faces() {
        assert!(
            !faces(held, "a face").is_empty(),
            "face {f:?} holds some operand face"
        );
    }
    assert_eq!(
        out.holdings.lost_rows().len(),
        out.naming.discards.len(),
        "one lost region per discard row"
    );
}

/// What each finished face holds, each member face by its place in the
/// list the union was given.
type Table = BTreeMap<FaceKey, BTreeSet<(usize, FaceKey)>>;

/// Runs the n-ary union of `bodies` in `order`: its finished body, the
/// last step's holdings, and every member face some step's discarded
/// face held, each member face read back to its place in `bodies`.
fn union_in(
    bodies: &[AtRestBody<f64>],
    order: &[usize],
) -> (AtRestBody<f64>, Table, BTreeSet<(usize, FaceKey)>) {
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
    let run = nary_boolean(NaryOp::Union, &members, SweepStrategy::Realized, tol());
    assert!(
        matches!(run.end, NaryEnd::Built),
        "order {order:?}: {:?}",
        run.end
    );
    let mut lost = BTreeSet::new();
    for step in &run.steps {
        for (row, held) in step.holdings.lost_rows().iter().enumerate() {
            let what = format!("order {order:?}'s step {}'s discard row {row}", step.member);
            lost.extend(faces(held, &what).into_iter().map(|(m, k)| (order[m], k)));
        }
    }
    let mut steps = run.steps;
    let step = steps.pop().unwrap();
    let holdings: &Holdings = &step.holdings;
    let held = holdings
        .faces()
        .map(|(f, h)| {
            let set = faces(h, &format!("order {order:?}'s face {f:?}"))
                .into_iter()
                .map(|(m, k)| (order[m], k))
                .collect();
            (f, set)
        })
        .collect();
    (step.body.body, held, lost)
}

/// **A union's holdings read alike in every member order**, and match
/// the finished body: the planar probe's left top face holds P's top
/// and the sunk tile's, the right Q's alone.
#[test]
fn a_unions_holdings_read_alike_in_every_member_order() {
    let t = tol();
    let b = |x, y, z| finished("b", brick::<f64>(x, y, z, t), t);
    let bodies = vec![
        b((0.0, 1.0), (0.0, 2.0), (0.0, 1.0)),
        b((1.0, 2.0), (0.0, 2.0), (0.0, 1.0)),
        b((0.8, 1.2), (-1.0, 3.0), (0.5, 2.0)),
        b((0.2, 0.6), (0.5, 1.5), (0.5, 1.0)),
    ];
    let top_of = |i: usize| (i, tops_at(&bodies[i], 1.0)[0]);
    let mut tables = BTreeSet::new();
    let mut losses = BTreeSet::new();
    for order in orders(bodies.len()) {
        let (result, held, lost) = union_in(&bodies, &order);
        assert!(
            lost.contains(&top_of(0)) && lost.contains(&top_of(1)),
            "order {order:?}: P's top and Q's lose the strip under the bar"
        );
        assert!(
            !lost.contains(&top_of(3)),
            "order {order:?}: the tile's top loses no area, though the bar cuts a face that \
             holds it"
        );
        losses.insert(lost);
        let mut tops: Vec<BTreeSet<(usize, FaceKey)>> = tops_at(&result, 1.0)
            .into_iter()
            .map(|f| held[&f].clone())
            .collect();
        tops.sort();
        assert_eq!(
            tops,
            vec![
                BTreeSet::from([top_of(0), top_of(3)]),
                BTreeSet::from([top_of(1)]),
            ],
            "order {order:?}: the left top holds P's top and T's, the right Q's"
        );
        rejoin_oracle(&bodies, &order, &result, &held, true);
        let mut table: Vec<BTreeSet<(usize, FaceKey)>> = held.into_values().collect();
        table.sort();
        tables.insert(table);
    }
    assert_eq!(tables.len(), 1, "one holdings table in every member order");
    assert_eq!(
        losses.len(),
        1,
        "the same member faces lose area in every member order"
    );
}

/// **The rejoin oracle**: each member rejoined to the finished body
/// holds, through its substitution rows, exactly the member faces the
/// holdings say each finished face holds of it, wherever the rejoin
/// builds. With `all`, every rejoin must build.
fn rejoin_oracle(
    bodies: &[AtRestBody<f64>],
    order: &[usize],
    result: &AtRestBody<f64>,
    held: &Table,
    all: bool,
) -> usize {
    let mut built = 0;
    for i in 0..bodies.len() {
        let rejoin = match super::union(result, &bodies[i], tol()) {
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
                let (af, bf) = match s.operand {
                    Operand::A => (s.face, s.by),
                    Operand::B => (s.by, s.face),
                };
                (n.face_fragments_a.root(af), n.face_fragments_b.root(bf))
            })
            .collect();
        let holds: BTreeSet<(FaceKey, FaceKey)> = held
            .iter()
            .flat_map(|(&f, set)| {
                set.iter()
                    .filter(move |(m, _)| *m == i)
                    .map(move |&(_, k)| (f, k))
            })
            .collect();
        assert_eq!(
            holds, read,
            "order {order:?}: what each finished face holds of member {i} is what the rejoin \
             substitutes"
        );
    }
    built
}
