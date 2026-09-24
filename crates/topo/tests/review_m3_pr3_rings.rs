//! PR 3 adversarial review — target 6 (join mirror sites, HARDER ring
//! fixture): a two-hole box whose divided top/bottom faces carry TWO
//! rings each, so `laringmv` must execute BOTH verdicts (keep one
//! ring In, move one ring Out) in a single re-homing sweep; plus a
//! split THROUGH one hole (the ring is divided, not re-homed —
//! two section polygons from one plane), and the genus-1 fixture on
//! the interval lane (which the acceptance interval test omits).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::holed_block;
use geom_core::Tol;
use geom_core::{Point3, Real, Vec3};
use topo::readback::euler_counts;
use topo::{Body, SplitPart, SplitPlane, mass_properties, split, validate_closed};

fn body_of<T: Real>(part: &SplitPart<T>) -> &Body<T> {
    part.body().expect("side has material")
}

fn plane_x<T: geom_core::Decide>(c: f64) -> SplitPlane<T> {
    SplitPlane {
        origin: Point3::new(T::from_f64(c), T::from_f64(0.0), T::from_f64(0.0)),
        normal: Vec3::new(T::from_f64(1.0), T::from_f64(0.0), T::from_f64(0.0)),
    }
}

/// The multi-ring `laringmv` sweep: two holes, split between them —
/// the divided top/bottom faces each hold two rings at re-homing
/// time, one In (stays) and one Out (moves). Both verdicts execute
/// in one sweep; each side ends up an independent genus-1 body.
#[test]
fn two_hole_box_split_between_rehomes_both_ways() {
    let body = holed_block::<f64>(6.0, &[1.0, 5.0], Tol::witness());
    assert_eq!(validate_closed(&body), Ok(()));
    let rings = |b: &Body<f64>| euler_counts(b).r;
    assert_eq!(rings(&body), 4, "top and bottom carry two rings each");
    let r = split(&body, &plane_x(3.0), Tol::witness()).unwrap();
    let (above, below) = (body_of(&r.above), body_of(&r.below));
    assert_eq!(validate_closed(above), Ok(()));
    assert_eq!(validate_closed(below), Ok(()));
    // One hole each: each side keeps exactly ONE ring pair (top +
    // bottom of its hole = 2 ring loops per side).
    assert_eq!(rings(below), 2, "below keeps its own hole's rings");
    assert_eq!(rings(above), 2, "above got the moved rings");
    assert_eq!(below.shells().count(), 1);
    assert_eq!(above.shells().count(), 1);
    // Volumes: total 6·2·2 − 2·(1·1·2) = 20, split 10/10.
    let (va, vb) = (
        mass_properties(above, Tol::witness()).unwrap().volume,
        mass_properties(below, Tol::witness()).unwrap().volume,
    );
    assert!((va - 10.0).abs() < 1e-12, "above {va}");
    assert!((vb - 10.0).abs() < 1e-12, "below {vb}");
}

/// Split THROUGH a hole (x = 1 bisects the first hole): the ring is
/// cut, not re-homed — the plane meets the body in TWO disjoint
/// section polygons (above and below the hole's channel), and each
/// side comes out genus-0-with-a-channel plus the other side's hole.
#[test]
fn split_through_hole_two_section_polygons() {
    let body = holed_block::<f64>(6.0, &[1.0, 5.0], Tol::witness());
    let s = topo::plane_section(&body, &plane_x::<f64>(1.0), Tol::witness()).unwrap();
    assert_eq!(s.polygons.len(), 2, "channel splits the section in two");
    let r = split(&body, &plane_x(1.0), Tol::witness()).unwrap();
    let (above, below) = (body_of(&r.above), body_of(&r.below));
    assert_eq!(validate_closed(above), Ok(()));
    assert_eq!(validate_closed(below), Ok(()));
    // Below: [0,1] slab with an open channel notch — no rings left.
    // Above: [1,6] slab with the intact second hole (2 ring loops).
    let rings = |b: &Body<f64>| euler_counts(b).r;
    assert_eq!(rings(below), 0);
    assert_eq!(rings(above), 2);
    let (va, vb) = (
        mass_properties(above, Tol::witness()).unwrap().volume,
        mass_properties(below, Tol::witness()).unwrap().volume,
    );
    // Total 20; below = 1·2·2 − (0.5·1·2 channel half) = 3.
    assert!((vb - 3.0).abs() < 1e-12, "below {vb}");
    assert!((va - 17.0).abs() < 1e-12, "above {va}");
}

/// The genus-1 ring re-homing on the INTERVAL lane — the acceptance
/// interval test never exercises `laringmv`/`point_in_loop` there.
/// Either it works like f64 (lane agreement) or it must refuse typed
/// (the documented interval posture) — a silent wrong answer or a
/// panic is the only failure. Executed to find out which.
#[cfg(feature = "interval")]
#[test]
fn interval_lane_ring_rehoming() {
    use geom_core::Interval;
    let body = holed_block::<Interval>(6.0, &[1.0, 5.0], Tol::witness());
    assert_eq!(validate_closed(&body), Ok(()));
    match split(&body, &plane_x::<Interval>(3.0), Tol::witness()) {
        Ok(r) => {
            let (above, below) = (body_of(&r.above), body_of(&r.below));
            assert_eq!(validate_closed(above), Ok(()));
            assert_eq!(validate_closed(below), Ok(()));
            let rings = |b: &Body<Interval>| euler_counts(b).r;
            assert_eq!((rings(below), rings(above)), (2, 2), "same as f64 lane");
        }
        Err(e) => {
            eprintln!("interval ring re-homing refused typed: {e}");
        }
    }
}
