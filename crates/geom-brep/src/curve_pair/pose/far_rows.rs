//! **The conic-pair readers of a far-centred conic's height**, read to
//! the belly's own digits: `conic_conic`'s parallel screen
//! ([`super::clear_all_round`]) and the first harmonic `transverse` and
//! `line_conic` solve in ([`super::Range::first_harmonic`]), each fed a
//! far ellipse's height over a plane grazing it at any angle round the
//! turn ([`super::height`], [`geom::Conic::belly_along`]) and judged
//! exactly on the stored data
//! ([`crate::curve_pair::far_graze_rows::judge`]).

#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::Sign;

use crate::curve_pair::far_graze_rows::{band, graze, judge};

/// A graze's stored placement, its plane, and its height read through
/// the pose's own terms, `(c₀, swing, belly)`.
type Read = (
    geom_core::StoredHarmonic<f64>,
    (geom_core::Point3<f64>, geom_core::Vec3<f64>),
    (f64, f64, f64),
);

/// One far graze's plane and its height ([`Read`]).
fn read(rng: &mut test_utils::fuzz::Rng, far: f64, arc_held: bool) -> Read {
    let g = graze(rng, far, arc_held);
    let tilt = rng.range(-1.2, 1.2);
    let n = (g.outward * tilt.cos() + g.axis * tilt.sin()).normalize();
    let w = n.cross(g.along).normalize();
    let gap = rng.range(-2.0, 2.0) * band().escalate();
    let o = g.crest + n * gap + w * rng.range(-3.0, 3.0);
    let k = g.conic.conic().unwrap();
    let (c0, cp, sp) = super::height(&k, o, n);
    (g.stored, (o, n), (c0, cp.hypot(sp), k.belly_along(o, n)))
}

/// `(wrong, beyond, definite)` over 2000 grazes at `far`, `decide`
/// answering a definite meets-or-not from `(c₀, swing, belly)`.
fn sweep(
    far: f64,
    label: &str,
    decide: impl Fn(f64, f64, f64) -> Option<bool>,
) -> (usize, usize, usize) {
    let mut rng = test_utils::fuzz::start(label);
    let (mut wrong, mut beyond, mut definite) = (0, 0, 0);
    for k in 0..test_utils::fuzz::scaled(2000) {
        let (stored, plane, (c0, swing, belly)) = read(&mut rng, far, k % 2 == 1);
        if let Some(meets) = decide(c0, swing, belly) {
            definite += 1;
            let (w, b) = judge(stored, plane, meets, band());
            wrong += usize::from(w);
            beyond += usize::from(b);
        }
    }
    (wrong, beyond, definite)
}

/// **`conic_conic`'s parallel screen clears a far-centred conic only
/// where it is clear**: the screen answers `Apart` where the height's
/// extremes are definitely one-signed, so wherever it does the exact
/// belly must be definitely negative.
#[test]
fn the_parallel_screen_clears_a_far_centred_conic_only_where_it_is_clear() {
    let got: Vec<_> = [1e4, 1e5, 1e6]
        .map(|far| {
            (
                far,
                sweep(far, "curve_pair::far_rows::screen", |c0, swing, belly| {
                    super::clear_all_round(c0, swing, belly, band()).then_some(false)
                }),
            )
        })
        .to_vec();
    println!("(far, (wrong, beyond the escalation, definite)): {got:?}");
    assert!(
        got.iter().all(|g| g.1.0 == 0),
        "read wrong: {got:?}; {}",
        test_utils::fuzz::replay()
    );
}

/// **The first harmonic `transverse` and `line_conic` solve reads its
/// extreme nearer zero to the belly's digits**: of its extremes
/// ([`super::Range::extremes`]), the one nearer zero (below the offset's
/// sign) definite says whether the conic crosses, and must say it as the
/// exact belly does.
#[test]
fn the_harmonics_near_extreme_reads_a_far_centred_conics_belly() {
    let got: Vec<_> = [1e4, 1e5, 1e6]
        .map(|far| {
            (
                far,
                sweep(far, "curve_pair::far_rows::range", |c0, swing, belly| {
                    let range = super::Range::first_harmonic(c0, swing, belly, 0.0);
                    let (lo, hi) = range.extremes(band());
                    let (near, crossed) = if c0 > 0.0 {
                        (lo, Sign::Negative)
                    } else {
                        (hi, Sign::Positive)
                    };
                    match near {
                        Ok(Sign::Zero) | Err(_) => None,
                        Ok(sign) => Some(sign == crossed),
                    }
                }),
            )
        })
        .to_vec();
    println!("(far, (wrong, beyond the escalation, definite)): {got:?}");
    assert!(
        got.iter().all(|g| g.1.0 == 0),
        "read wrong: {got:?}; {}",
        test_utils::fuzz::replay()
    );
}

/// `(Apart, poses)` of 2000 at `far`: an ellipse of semi-axes `far` and
/// `k·far` about `z`, its centre `(−far, 0, 0)` and seam `x`, so its seam
/// vertex is the origin exactly, against a unit circle centred
/// `(g, 1, 0)` with seam `−y` in a plane through `y` tilted out of the
/// ellipse's plane. The circle passes exactly through `(g, 0, 0)`, its
/// point at its seam, and that plane grazes the ellipse at its vertex.
/// With `|g| ≤ zero/2`, the two curves come within `zero/2` of each other,
/// so `Apart`, which says the distance is past the band, is wrong on
/// every pose. The fact is by construction, with no reading of the code.
fn grazing_circle_aparts(far: f64) -> (usize, usize) {
    let band = band();
    let mut rng = test_utils::fuzz::start("curve_pair::far_rows::grazing_circle");
    let mut apart = 0;
    let poses = test_utils::fuzz::scaled(2000);
    for _ in 0..poses {
        let k = rng.range(0.2, 1.0);
        let tilt = rng.range(0.1, 1.4) * if rng.unit() < 0.5 { -1.0 } else { 1.0 };
        let g = rng.range(-0.5, 0.5) * band.zero();
        let ellipse = geom::Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(
                geom_core::Point3::new(-far, 0.0, 0.0),
                geom_core::Vec3::unit_z(),
                geom_core::Vec3::unit_x(),
            ),
            far,
            k * far,
        ));
        let normal = geom_core::Vec3::new(tilt.cos(), 0.0, tilt.sin());
        let circle = geom::Curve3::circle(
            geom_core::Point3::new(g, 1.0, 0.0),
            normal,
            1.0,
            geom_core::Vec3::new(0.0, -1.0, 0.0),
        );
        let read = super::pose(
            &crate::curve_pair::Span::new(&ellipse, (-0.5, 0.5)),
            &crate::curve_pair::Span::new(&circle, (-0.5, 0.5)),
            band,
        );
        if matches!(read, Ok(super::Pose::Apart)) {
            apart += 1;
        }
    }
    (apart, poses)
}

/// **A far-centred conic grazing a circle's plane through a point of the
/// circle is never read apart** (`conic_conic`'s parallel screen at its
/// call site, read end to end): the curves meet within the zero band, so
/// no pose may answer `Apart`.
#[test]
fn a_far_conic_grazing_a_circle_through_its_vertex_is_never_apart() {
    let got: Vec<_> = [1e4, 1e5, 1e6]
        .map(|far| (far, grazing_circle_aparts(far)))
        .to_vec();
    println!("(far, (Apart, poses)): {got:?}");
    assert!(
        got.iter().all(|g| g.1.0 == 0),
        "read apart: {got:?}; {}",
        test_utils::fuzz::replay()
    );
}
