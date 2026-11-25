//! **`geom_brep::plane_torus_section` at a lever its caller hands it**:
//! the section value levers the closed form's fork over the torus's ball
//! (`plane_torus_closed_form`); a caller with its own extent may lever it
//! shorter, and each served row decides its circles' displacement off
//! the cut beside its own datum, so the circles stand within the band at
//! any lever.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom_core::{Point3, Vec3};

/// The ring torus `(R, r)` about `y` through the origin.
fn torus(big_r: f64, r: f64) -> geom::Surface<f64> {
    geom::Surface::Torus {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_y(), Vec3::unit_x()),
        major_radius: big_r,
        minor_radius: r,
    }
}

/// The plane through `origin` with unit normal `normal`.
fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> geom::Surface<f64> {
    geom::Surface::Plane {
        frame: geom::Frame::new(origin, normal, normal.cross(Vec3::unit_z()).normalize()),
    }
}

/// The largest distance off `cut` and off `wall`'s torus of points
/// round `c`.
fn off_both(c: &geom::Curve3<f64>, cut: &geom::Surface<f64>, (big_r, r): (f64, f64)) -> (f64, f64) {
    let geom::Surface::Plane { frame, .. } = *cut else {
        unreachable!()
    };
    let (origin, normal) = (frame.origin(), frame.axis());
    (0..720)
        .map(|k| c.eval(f64::from(k) * core::f64::consts::TAU / 720.0))
        .map(|p| {
            let tube = (p.x.hypot(p.z) - big_r).hypot(p.y) - r;
            ((p - origin).dot(normal).abs(), tube.abs())
        })
        .fold((0.0, 0.0), |(a, b), (x, y)| (a.max(x), b.max(y)))
}

/// **The served circles lie on the plane and on the torus within the
/// band at a thin lever**: `0.112 m`, the lily's weld cap read from a
/// site on the arch's rim, which a reader levering the section at its
/// own extent (the offset doors) may hand it. A meridian circle is minted on the torus about a centre `d`
/// off the cut, and the axis tilt leans its points off the cut by up to
/// `r` times the tilt more, so the table decides the two as one sum, at
/// any lever: a tilt just inside the band at the tube radius (or the
/// reach, where it is longer) is served circles within the band of both
/// surfaces, and so is a pose whose stand-off and lean are each `0.4` of
/// the band; one whose stand-off and lean are each `0.9` of it is not
/// served circles (decided separately, they stood `1.8ε` off the cut on
/// the donut, `1.32ε` on the arch). The lily's arch (`R = 1.1`,
/// `r = 0.052`, shorter than the reach) and the donut (`r = 0.5`, longer
/// than it). The concentric circles' tilt is levered at `R + r`, so both
/// stand within the band of the cut too. Red against the mutants "the
/// stand-off and the lean decided separately" and "the concentric tilt
/// levered at `R`", whose outer circle stands `1.1ε` off the cut.
#[test]
fn its_served_circles_stand_within_the_band_at_a_thin_lever() {
    let b = band();
    let reach = 0.112;
    let within_band = |two: [&geom::Curve3<f64>; 2],
                       cut: &geom::Surface<f64>,
                       (big_r, r): (f64, f64),
                       what: &str| {
        for c in two {
            let (plane_off, torus_off) = off_both(c, cut, (big_r, r));
            assert!(
                plane_off <= b.zero() && torus_off <= b.zero(),
                "R = {big_r}, r = {r}, {what}: a circle stands {plane_off:e} off the cut and \
                 {torus_off:e} off the torus, against the band's {:e}",
                b.zero()
            );
        }
    };
    let case = |cut: &geom::Surface<f64>, wall: &geom::Surface<f64>| {
        geom_brep::plane_torus_section(cut, wall, geom_core::test_support::fixture_lever(reach), b)
    };
    let circles =
        |got: &Result<geom_brep::PlaneTorusSection<f64>, geom_brep::SectionError>| match got {
            Ok(
                geom_brep::PlaneTorusSection::MeridianCircles { c1, c2 }
                | geom_brep::PlaneTorusSection::ConcentricCircles { c1, c2 },
            ) => Some([c1.clone(), c2.clone()]),
            _ => None,
        };
    for (big_r, r) in [(1.1, 0.052), (2.0, 0.5)] {
        let wall = torus(big_r, r);
        // The lean is read at the tube radius or the reach, whichever is
        // longer, so the tilt is decided Zero at the reach.
        let lever = r.max(reach);
        // The meridian plane `d` off the axis along its normal, the
        // normal turned toward the axis by `tilt`.
        let cut = |d: f64, tilt: f64| {
            let n = Vec3::new(tilt.cos(), tilt.sin(), 0.0);
            plane(Point3::origin() + n * d, n)
        };
        for (d, lean) in [(0.0, 0.0), (0.0, 0.9), (0.4, 0.4)] {
            let cut = cut(d * b.zero(), lean * b.zero() / lever);
            let got = case(&cut, &wall);
            let Some([c1, c2]) = circles(&got) else {
                panic!("R = {big_r}, r = {r}, d {d}, lean {lean}: two circles, got {got:?}");
            };
            within_band(
                [&c1, &c2],
                &cut,
                (big_r, r),
                &format!("d {d}, lean {lean} bands"),
            );
        }
        let split = cut(0.9 * b.zero(), 0.9 * b.zero() / lever);
        let got = case(&split, &wall);
        assert!(
            got.is_err(),
            "R = {big_r}, r = {r}: a stand-off and a lean of 0.9 bands each are served: {got:?}"
        );
        // The concentric circles: the table levers their axis tilt at
        // `R + r`, the largest radius either can have, which each
        // stands off the cut by its radius times. One tilt just inside
        // the band at that lever is served circles within it; one inside
        // the band only at `R` is not served circles.
        let square = |tilt: f64| {
            plane(
                Point3::new(0.0, 0.4 * r, 0.0),
                Vec3::new(tilt.sin(), tilt.cos(), 0.0),
            )
        };
        for tilt in [0.0, 0.9 * b.zero() / (big_r + r)] {
            let got = case(&square(tilt), &wall);
            let Some([c1, c2]) = circles(&got) else {
                panic!("R = {big_r}, r = {r}, tilt {tilt:e}: two circles, got {got:?}");
            };
            within_band(
                [&c1, &c2],
                &square(tilt),
                (big_r, r),
                &format!("tilt {tilt:e} off square"),
            );
        }
        let tilt = 0.99 * b.zero() / big_r;
        assert!(
            circles(&case(&square(tilt), &wall)).is_none(),
            "R = {big_r}, r = {r}: a tilt {tilt:e} off square, in band only at R, is served circles"
        );
    }
}
