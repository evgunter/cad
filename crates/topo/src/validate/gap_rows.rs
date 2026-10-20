//! Check 9's gap read against the real gap, on stored points far from
//! the origin (`lines_meet`'s [`super::gaps_apart`]). Its own file so
//! that its `Interval` oracle's bracket read stays out of
//! `validate.rs`, whose bracket reads the `Bounds` scope rule counts
//! (`tests/r1_lane1_bracket_read_census.rs`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Real};

use super::gaps_apart;

/// **A gap read apart is a real gap, on stored points far from the
/// origin** (the non-exact companion of `validate::tests`' T-junction
/// row, `an_end_touching_the_middle_of_a_square_segment_is_never_read_apart_by_the_gap`).
/// Two segments in one plane whose coordinates are
/// arbitrary `f64`s up to `10⁶` m out, `b` starting a step of
/// `10⁻¹³` … `10⁻⁷` m (either sign) along `a`'s line past its end and
/// in band across it. Wherever [`gaps_apart`] reads them apart at the
/// fine band, the `Interval` enclosure of the real gap (the stored
/// points against `a`'s real unit chord) is positive: the segments
/// are apart in ℝ.
///
/// **The lengths reach `3·10⁶` m because the charge is only visible
/// there.** For stored points near one another `p − a0` is nearly
/// exact, so a projection rounds by about `u` of the segment's
/// length, not `u·|p|`; below about `10⁵` m that stays under the
/// fine band, and a dropped charge would pass. Red where the gap's
/// charge is dropped.
#[test]
fn a_gap_read_apart_is_a_real_gap_on_points_far_from_the_origin() {
    use geom_core::{Bounds, Interval};
    let band = Band::new(1e-12, 1e-11).unwrap();
    let mut rng = test_utils::fuzz::start("check-9-gap-far");
    let mut fired = 0usize;
    let unit = |rng: &mut test_utils::fuzz::Rng| {
        geom_core::Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        )
        .normalize()
    };
    for _ in 0..test_utils::fuzz::scaled(20_000) {
        let far = 10f64.powf(rng.range(0.0, 6.0));
        let a0 = Point3::new(
            far * rng.range(-1.0, 1.0),
            far * rng.range(-1.0, 1.0),
            far * rng.range(-1.0, 1.0),
        );
        let d = unit(&mut rng);
        let n = d.cross(unit(&mut rng)).normalize();
        let len = 10f64.powf(rng.range(-2.0, 6.5));
        let a1 = a0 + d * len;
        let step = rng.range(-1.0, 1.0) * 10f64.powf(rng.range(-13.0, -7.0));
        let across = n.cross(d) * (rng.range(-1.0, 1.0) * 5e-12);
        let b0 = a1 + d * step + across;
        let turn = (d + n.cross(d) * rng.range(-1.0, 1.0)).normalize();
        let b1 = b0 + turn * len;
        let ea = (a1 - a0).normalize();
        if !gaps_apart(ea, (a0, a1), (b0, b1), band) {
            continue;
        }
        fired += 1;
        let iv = |p: Point3<f64>| {
            Point3::new(
                Interval::from_f64(p.x),
                Interval::from_f64(p.y),
                Interval::from_f64(p.z),
            )
        };
        let (ia0, ia1) = (iv(a0), iv(a1));
        let chord = ia1 - ia0;
        let real_unit = chord * (Interval::one() / chord.norm());
        let along = |q: Point3<Interval>| (q - ia0).dot(real_unit);
        let (t0, t1) = (along(iv(b0)), along(iv(b1)));
        let gap = t0.min(t1) - along(ia1);
        assert!(
            gap.lo() > 0.0,
            "read apart, but the real gap encloses {gap:?}: a {a0:?}→{a1:?}, \
             b {b0:?}→{b1:?} — {}",
            test_utils::fuzz::replay()
        );
    }
    assert!(
        fired > 100,
        "only {fired} draws read apart: the sweep does not reach the gap"
    );
}
