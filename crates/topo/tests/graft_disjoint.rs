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
