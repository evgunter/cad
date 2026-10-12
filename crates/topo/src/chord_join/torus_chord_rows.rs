//! **The plane × torus chord's section** ([`super::section_case`]'s
//! torus arm, [`super::section_loop`]): a plane through the axis or
//! square to it cuts the torus in two circles, and a chord is an arc of
//! the one its two sites lie on, picked by the boolean join's own split
//! of the section.

use super::*;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The ring torus `(R, r)` about `y` through the origin.
fn torus(big_r: f64, r: f64) -> geom::Surface<f64> {
    geom::Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_y(),
        major_radius: big_r,
        minor_radius: r,
        u_ref: Vec3::unit_x(),
    }
}

/// The plane through `origin` with unit normal `normal`.
fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> geom::Surface<f64> {
    geom::Surface::Plane {
        origin,
        normal,
        u_ref: normal.cross(Vec3::unit_z()).normalize(),
    }
}

/// The table's two loops for `cut` against `wall`, levered at `lever`.
fn loops(
    cut: &geom::Surface<f64>,
    wall: &geom::Surface<f64>,
    lever: f64,
) -> ([SectionConic<f64>; 2], LoopSplit<f64>) {
    let f = FaceKey::default();
    match section_case(
        (f, geom::SurfaceKind::Plane),
        band(),
        cut,
        wall,
        SectionReach::Torus(lever),
    ) {
        Ok(SectionCase::Loops { loops, split }) => (loops, split),
        Ok(_) => panic!("a two-loop section"),
        Err(e) => panic!("the torus arm refused {e:?}"),
    }
}

/// The loop the chord between `sites` takes.
fn pick(
    (loops, split): &([SectionConic<f64>; 2], LoopSplit<f64>),
    sites: [Point3<f64>; 2],
) -> Result<SectionConic<f64>, SplitJoinError> {
    let [a, b] = loops;
    section_loop(
        FaceKey::default(),
        [clone(a), clone(b)],
        *split,
        sites,
        band(),
    )
}

fn clone(c: &SectionConic<f64>) -> SectionConic<f64> {
    SectionConic {
        center: c.center,
        normal: c.normal,
        major: c.major,
        sa: c.sa,
        sb: c.sb,
        carrier: c.carrier.clone(),
    }
}

/// **A chord whose sites lie on `c2` is an arc of `c2`**: on the
/// meridian plane `x = 0` against the donut (`R = 2`, `r = 0.5` about
/// `y`), whose circles are centred at `(0, 0, ±2)`, a chord's sites on
/// either circle take that circle, whichever circle the table names
/// first; on the plane `y = 0.2`, square to the axis, a chord's sites on
/// the inner circle take the inner one. Red against the mutant "the
/// first circle always".
#[test]
fn a_chord_whose_sites_lie_on_c2_is_an_arc_of_c2() {
    let d = torus(2.0, 0.5);
    for n in [Vec3::unit_x(), -Vec3::unit_x()] {
        let two = loops(&plane(Point3::origin(), n), &d, 3.0);
        for z in [2.0, -2.0] {
            let on = |t: f64| Point3::new(0.0, 0.5 * t.sin(), z + 0.5 * t.cos());
            let got = pick(&two, [on(0.3), on(2.1)]).unwrap();
            assert!(
                (got.center - Point3::new(0.0, 0.0, z)).norm() < 1e-12,
                "normal {n:?}: the chord's sites on the circle about z = {z} took the one about \
                 {:?}",
                got.center
            );
        }
    }
    let h = 0.2;
    let w = (0.25_f64 - h * h).sqrt();
    let two = loops(&plane(Point3::new(0.0, h, 0.0), Vec3::unit_y()), &d, 3.0);
    for radius in [2.0 + w, 2.0 - w] {
        let on = |t: f64| Point3::new(radius * t.cos(), h, radius * t.sin());
        let got = pick(&two, [on(0.4), on(-2.5)]).unwrap();
        assert!(
            (got.sa - radius).abs() < 1e-12,
            "the chord's sites on the circle of radius {radius} took radius {}",
            got.sa
        );
    }
}

/// **A chord's sites on two loops, or on neither, are an invariant, and
/// in the escalation band they escalate**: one site on each meridian
/// circle; a site on the axis's trace in the plane, which no point of
/// the section comes near (its loop margin is Zero); and a site whose
/// margin lies between the band's two thresholds.
#[test]
fn a_chords_sites_on_two_loops_or_neither_are_refused() {
    let d = torus(2.0, 0.5);
    let two = loops(&plane(Point3::origin(), Vec3::unit_x()), &d, 3.0);
    let (c1, c2) = (Point3::new(0.0, 0.5, 2.0), Point3::new(0.0, 0.5, -2.0));
    let invariant = |got: Result<SectionConic<f64>, SplitJoinError>, what: &str| {
        assert!(
            matches!(got, Err(SplitJoinError::SectionInvariant { .. })),
            "{what}: {:?}",
            got.map(|c| c.center)
        );
    };
    invariant(pick(&two, [c1, c2]), "a site on each circle");
    invariant(
        pick(&two, [c1, Point3::new(0.0, 0.3, 0.0)]),
        "a site on the axis trace",
    );
    let b = band();
    let between = Point3::new(0.0, 0.3, (b.zero() + b.escalate()) / 2.0);
    assert!(
        matches!(
            pick(&two, [between, c1]),
            Err(SplitJoinError::Escalated { .. })
        ),
        "a site in the escalation band escalates"
    );
}

/// The largest distance off `cut` and off `wall`'s torus of points
/// round `c`.
fn off_both(c: &SectionConic<f64>, cut: &geom::Surface<f64>, (big_r, r): (f64, f64)) -> (f64, f64) {
    let geom::Surface::Plane { origin, normal, .. } = *cut else {
        unreachable!()
    };
    (0..720)
        .map(|k| {
            c.carrier
                .eval(f64::from(k) * core::f64::consts::TAU / 720.0)
        })
        .map(|p| {
            let tube = (p.x.hypot(p.z) - big_r).hypot(p.y) - r;
            ((p - origin).dot(normal).abs(), tube.abs())
        })
        .fold((0.0, 0.0), |(a, b), (x, y)| (a.max(x), b.max(y)))
}

/// **The minted circles lie on the plane and on the torus within the
/// band, at the thinnest face reach among the rows.** That reach is
/// `0.112 m`, the lily's weld cap read from a site on the arch's rim
/// (measured over the `germ_plane_torus_chord` and lily rows). A
/// meridian circle is minted on the torus and turned off the cut by the
/// axis tilt, so it stands off the cut by up to `r × tilt`; a tilt the
/// table decides Zero at the lever `L` is under `ε/L`. So the arm levers
/// at the tube radius where the reach is shorter: a plane whose tilt sits
/// just inside the band at that lever is served circles within the band
/// of both surfaces, and one whose tilt is in band only at the shorter
/// reach is not served circles at all. The lily's arch (`R = 1.1`,
/// `r = 0.052`, shorter than the reach) and the donut (`r = 0.5`, longer
/// than it). The concentric circles' tilt is levered at `R + r`, so
/// both stand within the band of the cut too. Red against the mutants
/// "lever at the reach alone", whose meridian circles stand `4ε` off the
/// cut on the donut, and "the concentric tilt levered at `R`", whose
/// outer circle stands `1.1ε` off it.
#[test]
fn the_minted_circles_stand_within_the_band_at_the_thinnest_reach() {
    let b = band();
    let reach = 0.112;
    for (big_r, r) in [(1.1, 0.052), (2.0, 0.5)] {
        let wall = torus(big_r, r);
        let lever = reach.max(r);
        let cut = |tilt: f64| plane(Point3::origin(), Vec3::new(tilt.cos(), tilt.sin(), 0.0));
        for tilt in [0.0, 0.9 * b.zero() / lever] {
            let (two, _) = loops(&cut(tilt), &wall, reach);
            for c in &two {
                let (plane_off, torus_off) = off_both(c, &cut(tilt), (big_r, r));
                assert!(
                    plane_off <= b.zero() && torus_off <= b.zero(),
                    "R = {big_r}, r = {r}, tilt {tilt:e}: a minted circle stands {plane_off:e} \
                     off the cut and {torus_off:e} off the torus, against the band's {:e}",
                    b.zero()
                );
            }
        }
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
            let (two, _) = loops(&square(tilt), &wall, reach);
            for c in &two {
                let (plane_off, torus_off) = off_both(c, &square(tilt), (big_r, r));
                assert!(
                    plane_off <= b.zero() && torus_off <= b.zero(),
                    "R = {big_r}, r = {r}, tilt {tilt:e} off square: a concentric circle of \
                     radius {} stands {plane_off:e} off the cut and {torus_off:e} off the torus",
                    c.sa
                );
            }
        }
        let tilt = 0.99 * b.zero() / big_r;
        assert!(
            !matches!(
                section_case(
                    (FaceKey::default(), geom::SurfaceKind::Plane),
                    b,
                    &square(tilt),
                    &wall,
                    SectionReach::Torus(reach),
                ),
                Ok(SectionCase::Loops { .. })
            ),
            "R = {big_r}, r = {r}: a tilt {tilt:e} off square, in band only at R, is served circles"
        );
        if lever > reach {
            let tilt = 0.9 * b.zero() / reach;
            let got = section_case(
                (FaceKey::default(), geom::SurfaceKind::Plane),
                b,
                &cut(tilt),
                &wall,
                SectionReach::Torus(reach),
            );
            assert!(
                matches!(
                    got,
                    Err(SplitJoinError::Section { .. } | SplitJoinError::Escalated { .. })
                ),
                "R = {big_r}, r = {r}: a tilt {tilt:e} in band at the reach alone is served"
            );
        }
    }
}
