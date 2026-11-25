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

/// The table's two loops for `cut` against `wall`.
fn loops(
    cut: &geom::Surface<f64>,
    wall: &geom::Surface<f64>,
) -> ([SectionCurve<f64>; 2], LoopSplit<f64>) {
    let f = FaceKey::default();
    match section_case(f, band(), cut, wall, SectionReach::Unmeasured) {
        Ok(SectionCase::Loops { loops, split }) => (loops, split),
        Ok(_) => panic!("a two-loop section"),
        Err(e) => panic!("the torus arm refused {e:?}"),
    }
}

/// The loop the chord between `sites` takes.
fn pick(
    (loops, split): &([SectionCurve<f64>; 2], LoopSplit<f64>),
    sites: [Point3<f64>; 2],
) -> Result<SectionCurve<f64>, JoinError> {
    section_loop(
        FaceKey::default(),
        loops.clone(),
        (*split, None),
        sites,
        band(),
    )
}

/// The circle a loop of a meridian or concentric section is.
fn circle(c: SectionCurve<f64>) -> SectionConic<f64> {
    match c {
        SectionCurve::Conic(c) => c,
        SectionCurve::Oval(o) => panic!("a circle's loop came back an oval: {:?}", o.carrier),
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
        let two = loops(&plane(Point3::origin(), n), &d);
        for z in [2.0, -2.0] {
            let on = |t: f64| Point3::new(0.0, 0.5 * t.sin(), z + 0.5 * t.cos());
            let got = circle(pick(&two, [on(0.3), on(2.1)]).unwrap());
            assert!(
                (got.conic().frame().origin() - Point3::new(0.0, 0.0, z)).norm() < 1e-12,
                "normal {n:?}: the chord's sites on the circle about z = {z} took the one about \
                 {:?}",
                got.conic().frame().origin()
            );
        }
    }
    let h = 0.2;
    let w = (0.25_f64 - h * h).sqrt();
    let two = loops(&plane(Point3::new(0.0, h, 0.0), Vec3::unit_y()), &d);
    for radius in [2.0 + w, 2.0 - w] {
        let on = |t: f64| Point3::new(radius * t.cos(), h, radius * t.sin());
        let got = circle(pick(&two, [on(0.4), on(-2.5)]).unwrap());
        assert!(
            (got.conic().semi_axes().0 - radius).abs() < 1e-12,
            "the chord's sites on the circle of radius {radius} took radius {}",
            got.conic().semi_axes().0
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
    let two = loops(&plane(Point3::origin(), Vec3::unit_x()), &d);
    let (c1, c2) = (Point3::new(0.0, 0.5, 2.0), Point3::new(0.0, 0.5, -2.0));
    let invariant = |got: Result<SectionCurve<f64>, JoinError>, what: &str| {
        assert!(
            matches!(got, Err(JoinError::SectionInvariant { .. })),
            "{what}: {:?}",
            got.map(|c| c.carrier().clone())
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
        matches!(pick(&two, [between, c1]), Err(JoinError::Escalated { .. })),
        "a site in the escalation band escalates"
    );
}

/// **A chord on a spiric oval is an arc of it on its own parameter, the
/// one its departure leaves along**: the plane `x = 0.3` against the
/// donut cuts two ovals, one each side of the axis's trace. For sites
/// on either oval at pairs of minor angles, short arcs and long ones and
/// arcs across the parameter's seam, with the departure along the oval
/// either way: the chord's loop is that oval, its arc starts at the
/// first site and ends at the second, and its midpoint is the oval's
/// own point half way along the parameter the departure runs into (the
/// closed form, `Curve3::Spiric`). Red against the mutant "the arc's
/// parameter read as the angle about the plane's foot", which lies
/// outside both ovals.
#[test]
fn a_chord_on_an_oval_is_an_arc_on_its_own_parameter() {
    let d = torus(2.0, 0.5);
    let cut = plane(Point3::new(0.3, 0.0, 0.0), Vec3::unit_x());
    let two = loops(&cut, &d);
    let geom_brep::PlaneTorusSection::SpiricOvals { s1, s2 } = geom_brep::plane_torus_section(
        &cut,
        &d,
        geom_core::test_support::fixture_lever(3.0),
        band(),
    )
    .unwrap() else {
        panic!("x = 0.3 cuts two ovals");
    };
    let tau = core::f64::consts::TAU;
    for (name, oval) in [("s1", s1), ("s2", s2)] {
        for (v1, v2) in [
            (0.2, 0.9),
            (0.9, 0.2),
            (-2.5, 2.8),
            (2.8, -2.5),
            (1.0, 4.0),
            (-0.3, 3.0),
        ] {
            let (p1, p2) = (oval.eval(v1), oval.eval(v2));
            let curve = pick(&two, [p1, p2]).unwrap();
            assert!(
                matches!(&curve, SectionCurve::Oval(o) if format!("{:?}", o.carrier) == format!("{oval:?}")),
                "{name}, {v1} → {v2}: the chord's loop is its own oval"
            );
            for along in [1.0, -1.0] {
                let leave = Departure {
                    dir: oval.deriv(v1).normalize() * along,
                    datum: Datum::Germ,
                };
                let what = format!("{name}, {v1} → {v2}, along {along}");
                let ccw =
                    arc_leaving_as(FaceKey::default(), band(), &curve, &d, p1, leave).unwrap();
                assert_eq!(
                    ccw,
                    along > 0.0,
                    "{what}: the arc leaves along its departure"
                );
                let (carrier, t0, t1) = oriented_arc(
                    &curve,
                    FaceKey::default(),
                    curve.param(p1),
                    ArcEnd::At(curve.param(p2)),
                    ccw,
                )
                .unwrap();
                assert!(t1 > t0, "{what}: the arc runs forward");
                assert!(
                    (carrier.eval(t0) - p1).norm() < 1e-12,
                    "{what}: starts at p1"
                );
                assert!((carrier.eval(t1) - p2).norm() < 1e-12, "{what}: ends at p2");
                let forward = (v2 - v1).rem_euclid(tau);
                let mid = if ccw {
                    v1 + forward / 2.0
                } else {
                    v1 - (tau - forward) / 2.0
                };
                assert!(
                    (carrier.mid_point(t0, t1) - oval.eval(mid)).norm() < 1e-12,
                    "{what}: the arc's midpoint is the oval's at {mid}"
                );
            }
        }
    }
}
