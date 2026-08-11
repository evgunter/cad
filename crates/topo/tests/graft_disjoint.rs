//! **The disjoint-graft door on real geometry** (R1 MINOR-2 for
//! PR #325): `topo::graft_disjoint`'s structural contract is checked
//! in the module's own unit rows; these are the rows that need a body
//! with CERTIFIED geometry — the import's actual call shape, where a
//! component is mapped through `transform_rigid` and the placed copy
//! is grafted in, and where the `RemapKeys` description bridge is the
//! thing under test.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{describe_as_intersections, geometric_cube};
use geom_core::{Affine3, Vec3};

/// A placed copy grafts in whole, and the union is tier-1/2/3 valid:
/// the descriptions the copy carries land under the destination's own
/// surface keys, with the source's certificates travelling verbatim.
#[test]
fn a_placed_copy_grafts_and_the_union_certifies() {
    // `describe_as_intersections` is the upgrade pass `geometric_cube`
    // documents: a cube's edges are definitely-transverse plane-pair
    // corners, and tier 3 names un-upgraded chords by design. It is run
    // on the SOURCE, so the descriptions the graft has to bridge are
    // `Intersection` ones holding surface keys — which is the point.
    let mut src = geometric_cube::<f64>().body;
    describe_as_intersections(&mut src);
    let mut dst = geometric_cube::<f64>().body;
    describe_as_intersections(&mut dst);
    // 10 mm clear of the unit cube: disjoint, so the union is a body.
    let map = Affine3::translation(Vec3::new(10.0, 0.0, 0.0));
    let placed = topo::transform_rigid(&src, &map).expect("a rigid map");
    let key = topo::graft_disjoint(&mut dst, &placed).expect("a placed graft");

    assert_eq!(dst.solids().count(), 2, "two solids, not one fused");
    assert_eq!(dst.faces().count(), 12);
    assert_eq!(dst.edges().count(), 24);
    assert_eq!(dst.vertices().count(), 16);
    assert_eq!(
        topo::validate_geometric(&dst),
        Ok(()),
        "tiers 1-3 on the union"
    );

    // The copy sits where the map put it, and the original did not
    // move: x runs 0..1 for one solid and 10..11 for the other.
    let extent = |solid| {
        dst.solids()
            .find(|(k, _)| *k == solid)
            .map(|(_, s)| {
                s.shells
                    .iter()
                    .flat_map(|&sh| dst.get_shell(sh).unwrap().faces.clone())
                    .flat_map(|f| {
                        let face = dst.get_face(f).unwrap();
                        std::iter::once(face.outer).chain(face.rings.iter().copied())
                    })
                    .filter_map(|l| match dst.get_loop(l).unwrap().boundary {
                        topo::LoopBoundary::Cycle { first } => dst.loop_cycle(first),
                        topo::LoopBoundary::Empty { .. } => None,
                    })
                    .flatten()
                    .map(|he| {
                        let v = dst.get_half_edge(he).unwrap().start;
                        dst.get_point(dst.get_vertex(v).unwrap().point).unwrap().x
                    })
                    .fold((f64::MAX, f64::MIN), |(lo, hi), x| (lo.min(x), hi.max(x)))
            })
            .unwrap()
    };
    let (glo, ghi) = extent(key);
    assert!(
        (glo - 10.0).abs() < 1e-12 && (ghi - 11.0).abs() < 1e-12,
        "{glo}..{ghi}"
    );
    let other = dst.solids().map(|(k, _)| k).find(|k| *k != key).unwrap();
    let (olo, ohi) = extent(other);
    assert!(
        (olo - 0.0).abs() < 1e-12 && (ohi - 1.0).abs() < 1e-12,
        "{olo}..{ohi}"
    );
}

/// The same source grafted TWICE under two different maps: three
/// solids, three positions, one arena — the N-instance shape, at the
/// kernel door rather than through the reader.
#[test]
fn two_placed_copies_of_one_source_are_two_independent_solids() {
    let mut src = geometric_cube::<f64>().body;
    describe_as_intersections(&mut src);
    let mut dst = geometric_cube::<f64>().body;
    describe_as_intersections(&mut dst);
    for dx in [10.0, 20.0] {
        let placed = topo::transform_rigid(&src, &Affine3::translation(Vec3::new(dx, 0.0, 0.0)))
            .expect("a rigid map");
        topo::graft_disjoint(&mut dst, &placed).expect("a graft");
    }
    assert_eq!(dst.solids().count(), 3);
    assert_eq!(dst.faces().count(), 18);
    assert_eq!(topo::validate_geometric(&dst), Ok(()), "tiers 1-3");
    // Three separated x bands, and no point between them.
    let mut xs: Vec<f64> = dst.points().map(|(_, p)| p.x).collect();
    xs.sort_by(f64::total_cmp);
    xs.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    assert_eq!(xs.len(), 6, "0,1,10,11,20,21 — got {xs:?}");
}

// ---------------------------------------------------------------------
// ASM-2K: the N-solid door (`graft_disjoint_all`). A multi-solid source
// is an assembly's instantiated part; the door's claim is that grafting
// it is the SAME transplant as grafting its solids one by one.
// ---------------------------------------------------------------------

/// Two described cubes, `dx` apart, as one two-solid body — the
/// multi-solid source these rows graft.
fn two_solid_source(dx: f64) -> topo::Body<f64> {
    let mut a = geometric_cube::<f64>().body;
    describe_as_intersections(&mut a);
    let mut b = geometric_cube::<f64>().body;
    describe_as_intersections(&mut b);
    let b = topo::transform_rigid(&b, &Affine3::translation(Vec3::new(dx, 0.0, 0.0)))
        .expect("a rigid map");
    topo::graft_disjoint(&mut a, &b).expect("a two-solid source");
    a
}

/// The census a graft is compared by: every arena's size.
fn census(b: &topo::Body<f64>) -> [usize; 8] {
    [
        b.solids().count(),
        b.shells().count(),
        b.faces().count(),
        b.edges().count(),
        b.vertices().count(),
        b.points().count(),
        b.surfaces().count(),
        b.solids().map(|(_, s)| s.shells.len()).sum(),
    ]
}

/// **The D-1 equivalence.** Grafting an N-solid source is N sequential
/// single-solid grafts in the source's solid order: same census, same
/// per-solid shape, bit-equal volume, and the same point coordinates in
/// the same arena order — the transplant is one pass over shared
/// arenas, not a re-derivation.
#[test]
fn a_multi_solid_graft_equals_sequential_single_solid_grafts() {
    let multi = two_solid_source(10.0);
    let pieces: Vec<topo::Body<f64>> = {
        // The same two solids as separate bodies (what the sequential
        // path grafts): rebuilt exactly as `two_solid_source` built
        // them, before they were joined.
        let mut first = geometric_cube::<f64>().body;
        describe_as_intersections(&mut first);
        let mut second = geometric_cube::<f64>().body;
        describe_as_intersections(&mut second);
        let second =
            topo::transform_rigid(&second, &Affine3::translation(Vec3::new(10.0, 0.0, 0.0)))
                .expect("a rigid map");
        vec![first, second]
    };

    let mut at_once = geometric_cube::<f64>().body;
    describe_as_intersections(&mut at_once);
    let keys = topo::graft_disjoint_all(&mut at_once, &multi).expect("the N-solid graft");
    assert_eq!(keys.len(), 2, "one key per source solid");

    let mut one_by_one = geometric_cube::<f64>().body;
    describe_as_intersections(&mut one_by_one);
    let seq: Vec<_> = pieces
        .iter()
        .map(|p| topo::graft_disjoint(&mut one_by_one, p).expect("a single graft"))
        .collect();

    assert_eq!(census(&at_once), census(&one_by_one), "same census");
    assert_eq!(
        at_once
            .points()
            .map(|(_, p)| p.x.to_bits())
            .collect::<Vec<_>>(),
        one_by_one
            .points()
            .map(|(_, p)| p.x.to_bits())
            .collect::<Vec<_>>(),
        "the same coordinates in the same arena order"
    );
    assert_eq!(
        topo::mass_properties(&at_once)
            .expect("volume")
            .volume
            .to_bits(),
        topo::mass_properties(&one_by_one)
            .expect("volume")
            .volume
            .to_bits(),
        "bit-equal volume"
    );
    // Solid ORDER is preserved: the k-th returned key is the k-th
    // source solid, and it sits where that solid sat.
    let x_of = |b: &topo::Body<f64>, k: topo::SolidKey| {
        b.get_solid(k)
            .unwrap()
            .shells
            .iter()
            .flat_map(|&sh| b.get_shell(sh).unwrap().faces.clone())
            .map(|f| {
                let l = b.get_face(f).unwrap().outer;
                match b.get_loop(l).unwrap().boundary {
                    topo::LoopBoundary::Cycle { first } => {
                        let v = b.get_half_edge(first).unwrap().start;
                        b.get_point(b.get_vertex(v).unwrap().point).unwrap().x
                    }
                    topo::LoopBoundary::Empty { .. } => f64::MAX,
                }
            })
            .fold(f64::MAX, f64::min)
    };
    for (a, b) in keys.iter().zip(seq.iter()) {
        assert_eq!(x_of(&at_once, *a), x_of(&one_by_one, *b), "solid order");
    }
    assert!(
        x_of(&at_once, keys[0]) < x_of(&at_once, keys[1]),
        "source order"
    );
    assert_eq!(at_once.solids().count(), 3, "the destination plus both");
    assert_eq!(
        topo::validate_geometric(&at_once),
        Ok(()),
        "tiers 1-3 on the aggregate"
    );
}

/// **Fresh-key discipline across grafts of ONE source.** The collision
/// class the import's `SolidSpec` note names — two copies of one source
/// in one arena — cannot arise: each graft re-creates every entity
/// under a key of its own, so two grafts of the same multi-solid source
/// occupy disjoint key ranges.
#[test]
fn two_grafts_of_one_multi_solid_source_share_no_key() {
    let multi = two_solid_source(10.0);
    let far = topo::transform_rigid(&multi, &Affine3::translation(Vec3::new(0.0, 30.0, 0.0)))
        .expect("a rigid map");
    let mut dst = geometric_cube::<f64>().body;
    describe_as_intersections(&mut dst);
    let base: std::collections::BTreeSet<_> = dst.faces().map(|(k, _)| k).collect();

    let first = topo::graft_disjoint_all(&mut dst, &multi).expect("graft one");
    let after_first: std::collections::BTreeSet<_> = dst.faces().map(|(k, _)| k).collect();
    let second = topo::graft_disjoint_all(&mut dst, &far).expect("graft two");
    let all: std::collections::BTreeSet<_> = dst.faces().map(|(k, _)| k).collect();

    assert_eq!(first.len(), 2);
    assert_eq!(second.len(), 2);
    assert!(
        first.iter().all(|k| !second.contains(k)),
        "a solid key was reused"
    );
    assert_eq!(dst.solids().count(), 5, "1 + 2 + 2 solids");
    assert_eq!(all.len(), base.len() * 5, "every face is its own key");
    // The second graft touched nothing the first produced, and neither
    // touched the destination's own.
    assert!(after_first.is_superset(&base) && all.is_superset(&after_first));
    assert_eq!(all.len(), 5 * 6, "6 faces per cube, no key collapsed");
    assert_eq!(
        topo::validate_geometric(&dst),
        Ok(()),
        "five disjoint solids are a body"
    );
}

/// **Per-solid and aggregate gates catch different things, and the
/// N-solid graft changes neither.** The per-solid gate is what names
/// WHICH solid is bad (it is asked about that solid alone); the
/// aggregate gate is the only one that can see a relation BETWEEN
/// solids. A raw transplant asserts neither — validity stays the
/// caller's, exactly as at the single door.
#[test]
fn per_solid_and_aggregate_gates_both_still_bite_on_a_multi_solid_source() {
    // A source whose SECOND solid is un-upgraded (tier 3 names its
    // chords): the per-solid gate refuses that piece, by itself, and
    // the caller knows which one because it is the subject.
    let mut good = geometric_cube::<f64>().body;
    describe_as_intersections(&mut good);
    let raw = topo::transform_rigid(
        &geometric_cube::<f64>().body,
        &Affine3::translation(Vec3::new(10.0, 0.0, 0.0)),
    )
    .expect("a rigid map");
    let pieces = [good.clone(), raw.clone()];
    let verdicts: Vec<bool> = pieces
        .iter()
        .map(|p| topo::validate_geometric(p).is_ok())
        .collect();
    assert_eq!(
        verdicts,
        vec![true, false],
        "the per-solid gate names solid 1"
    );

    // Grafted anyway, the aggregate gate still bites — the defect
    // travelled with the entity, it was not laundered by the graft.
    let mut bad = good.clone();
    topo::graft_disjoint(&mut bad, &raw).expect("a graft is not a gate");
    let mut dst = geometric_cube::<f64>().body;
    describe_as_intersections(&mut dst);
    topo::graft_disjoint_all(&mut dst, &bad).expect("the transplant itself succeeds");
    assert!(
        topo::validate_geometric(&dst).is_err(),
        "the aggregate gate must still refuse"
    );

    // On success the aggregate gate still RUNS, over the whole
    // transplanted union and not merely the pieces: a second graft of
    // clean solids into an already-gated destination is asked again.
    let mut clean = good.clone();
    let far = topo::transform_rigid(&good, &Affine3::translation(Vec3::new(20.0, 0.0, 0.0)))
        .expect("a rigid map");
    topo::graft_disjoint(&mut clean, &far).expect("a graft");
    let clean = topo::transform_rigid(&clean, &Affine3::translation(Vec3::new(0.0, 40.0, 0.0)))
        .expect("a rigid map");
    let mut dst = geometric_cube::<f64>().body;
    describe_as_intersections(&mut dst);
    topo::graft_disjoint_all(&mut dst, &clean).expect("the N-solid door");
    assert_eq!(dst.solids().count(), 3);
    assert_eq!(topo::validate_geometric(&dst), Ok(()), "the aggregate gate");
}

/// The N-solid door's own refusals: a source with no solid at all is
/// `JoinDesync` and writes nothing, and the single-solid door still
/// refuses an N-solid source (its one returned key could name only one
/// of them) — the flipped case is `graft_disjoint_all`'s.
#[test]
fn the_n_solid_door_refuses_an_empty_source_and_the_single_door_still_refuses_n() {
    let mut dst = geometric_cube::<f64>().body;
    let err = topo::graft_disjoint_all(&mut dst, &topo::Body::<f64>::new())
        .expect_err("no solid to graft");
    assert!(format!("{err:?}").contains("JoinDesync"), "{err:?}");
    assert_eq!(dst.solids().count(), 1, "and nothing was written");

    let multi = two_solid_source(10.0);
    let err = topo::graft_disjoint(&mut dst, &multi).expect_err("N solids at the single door");
    assert!(format!("{err:?}").contains("JoinDesync"), "{err:?}");
    assert_eq!(dst.solids().count(), 1, "and nothing was written");
    // The same source, at the door that is FOR it, succeeds.
    assert_eq!(
        topo::graft_disjoint_all(&mut dst, &multi)
            .expect("the N-solid door")
            .len(),
        2
    );
}

/// Provenance rides per solid: each grafted solid carries ITS source
/// solid's record, verbatim and in order (a graft is not a re-birth,
/// and D-3 mints no new vocabulary for the N-solid case).
#[test]
fn each_grafted_solid_carries_its_own_source_provenance() {
    let multi = two_solid_source(10.0);
    let want: Vec<String> = multi
        .solids()
        .map(|(k, _)| format!("{:?}", multi.provenance(topo::EntityId::Solid(k)).unwrap()))
        .collect();
    let mut dst = topo::Body::<f64>::new();
    let keys = topo::graft_disjoint_all(&mut dst, &multi).expect("the N-solid door");
    let got: Vec<String> = keys
        .iter()
        .map(|&k| format!("{:?}", dst.provenance(topo::EntityId::Solid(k)).unwrap()))
        .collect();
    assert_eq!(got, want, "per-solid provenance, in source order");
}
