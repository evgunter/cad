//! **The section value** (`geom_brep::pair_section`): the cylinder ×
//! sphere pose decided once, the components every reader takes, and the
//! carrying that marches the general rung.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{
    CarryError, ClassCarrier, Coincident, ComponentShape, PairSection, SectionError, SectionRead,
    SsiError, SsiLane, ZeroKind,
};
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A cylinder of radius `r` about `z` through the origin.
fn cylinder(r: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, -3.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: r,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// A sphere of radius `big` about `(d, 0, 0.2)`.
fn sphere(d: f64, big: f64) -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(d, 0.0, 0.2),
        radius: big,
        axis: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn built(a: &Surface<f64>, b: &Surface<f64>) -> PairSection<f64> {
    match geom_brep::pair_section(a, b, None, band()) {
        Ok(SectionRead::Built(section)) => section,
        other => panic!("expected the section, got {other:?}"),
    }
}

fn on_both(p: Point3<f64>, a: &Surface<f64>, b: &Surface<f64>, what: &str) {
    for s in [a, b] {
        let r = geom_brep::implicit_residual(s, p);
        assert!(r.abs() < 1e-12, "{what}: {p:?} is {r:e} off {s:?}");
    }
}

/// **The coaxial pose**: two circles of the cylinder's radius, each on
/// both surfaces, winding about the cylinder and not the sphere, told
/// apart by their side of the centre's plane; the coincidence that
/// chose the arm is the record the Boolean keeps. Carried without a
/// march door: both carriers are closed forms.
#[test]
fn the_coaxial_pose_is_two_circles_and_records_its_coincidence() {
    let (cyl, sph) = (cylinder(1.0), sphere(0.0, 2.0));
    let section = built(&cyl, &sph);
    let (decided, _) = section.coincidence().expect("the coaxiality is recorded");
    assert_eq!(
        (decided.name(), decided),
        ("cs_coaxial", Coincident::CylinderSphere)
    );
    assert!(section.tangency().is_none(), "no tangency");
    assert_eq!(
        section.counted().expect("a counted section").len(),
        2,
        "two circles"
    );
    for (k, c) in section
        .counted()
        .expect("a counted section")
        .iter()
        .enumerate()
    {
        assert_eq!(
            c.winds,
            [true, false],
            "circle {k} winds about the cylinder"
        );
        let ClassCarrier::Closed(curve @ Curve3::Circle { .. }) = &c.carrier else {
            panic!("circle {k}: a closed-form circle, got {:?}", c.carrier);
        };
        for i in 0..6 {
            on_both(curve.eval(f64::from(i)), &cyl, &sph, "a coaxial circle");
        }
    }
    let swapped = built(&sph, &cyl);
    assert_eq!(
        swapped.counted().expect("a counted section")[0].winds,
        [false, true],
        "the order asked of"
    );
    let carried = section.carry(None, band()).expect("closed forms carry");
    let p = match &section.counted().expect("a counted section")[1].carrier {
        ClassCarrier::Closed(curve) => curve.eval(0.4),
        _ => unreachable!(),
    };
    let foot = carried.foot(p, band()).unwrap();
    assert_eq!(foot.component, 1, "the foot's component");
}

/// **A tangency is classification data**, decided once: the coaxial
/// tangent circle (`cs_wall_reach`), an off-axis sphere touching the wall
/// from outside or inside (`cs_near_reach`, one point on the nearest
/// ruling), and the figure-eight's node (`cs_far_reach`). None carries.
#[test]
fn a_tangency_is_classification_data_and_does_not_carry() {
    for (label, d, r, big, decision, shape, at) in [
        (
            "tangent circle",
            0.0,
            1.0,
            1.0,
            "cs_wall_reach",
            ComponentShape::Loop,
            None,
        ),
        (
            "touch outside",
            1.5,
            0.5,
            1.0,
            "cs_near_reach",
            ComponentShape::Point,
            Some(Point3::new(0.5, 0.0, 0.2)),
        ),
        (
            "touch inside",
            0.3,
            1.0,
            0.7,
            "cs_near_reach",
            ComponentShape::Point,
            Some(Point3::new(1.0, 0.0, 0.2)),
        ),
        (
            "node",
            0.5,
            0.5,
            1.0,
            "cs_far_reach",
            ComponentShape::Loop,
            None,
        ),
    ] {
        let (cyl, sph) = (cylinder(r), sphere(d, big));
        let section = built(&cyl, &sph);
        let zero = section.tangency().expect("a tangency");
        assert_eq!(
            (zero.decision.name(), zero.decision.kind()),
            (decision, ZeroKind::Tangency),
            "{label}"
        );
        let [c] = section.counted().expect("a counted section") else {
            panic!("{label}: one component");
        };
        assert_eq!(c.shape, shape, "{label}");
        assert!(matches!(c.carrier, ClassCarrier::Unminted(_)), "{label}");
        if let Some(at) = at {
            assert!(
                (c.witness.unwrap() - at).norm() < 1e-12,
                "{label}: {:?}",
                c.witness
            );
        }
        assert!(
            matches!(section.carry(None, band()), Err(CarryError::Unminted(_))),
            "{label}: a tangency does not carry"
        );
    }
}

/// **Off the axis the reach counts the loops**: none short of the wall,
/// one past its nearest ruling, two past its farthest, each loop on the
/// general rung and two winding about the cylinder; the lone loop's
/// witness lies on both surfaces.
#[test]
fn off_the_axis_the_reach_counts_the_loops() {
    let none = built(&cylinder(0.5), &sphere(2.0, 0.5));
    assert!(
        none.counted().expect("a counted section").is_empty(),
        "the sphere misses the wall"
    );
    let (cyl, sph) = (cylinder(0.5), sphere(0.5, 0.3));
    let one = built(&cyl, &sph);
    let [c] = one.counted().expect("a counted section") else {
        panic!("one loop");
    };
    assert_eq!(c.winds, [false, false], "null on both carriers");
    assert!(matches!(c.carrier, ClassCarrier::General));
    on_both(c.witness.unwrap(), &cyl, &sph, "the lone loop's witness");
    let two = built(&cylinder(0.5), &sphere(0.1, 0.7));
    assert_eq!(
        two.counted().expect("a counted section").len(),
        2,
        "two loops"
    );
    for c in two.counted().expect("a counted section") {
        assert_eq!(c.winds, [true, false], "each winds about the cylinder");
        assert!(matches!(c.carrier, ClassCarrier::General));
    }
}

/// **An axis offset in the band escalates** (`cs_coaxial`), and a reach
/// in the band too: the pose never folds a conditioning verdict into an
/// arm's absence.
#[test]
fn a_pose_in_the_band_escalates() {
    let b = band();
    let mid = (b.zero() + b.escalate()) / 2.0;
    for (label, d, big, want) in [
        ("offset", mid, 0.75, "cs_coaxial"),
        ("far reach", 0.25, 0.75 + mid, "cs_far_reach"),
        ("wall reach", 0.0, 0.5 + mid, "cs_wall_reach"),
    ] {
        match geom_brep::pair_section(&cylinder(0.5), &sphere(d, big), None, b) {
            Err(SectionError::Escalated(diag)) => {
                assert_eq!(diag.predicate, Some(want), "{label}");
            }
            other => panic!("{label}: expected an escalation, got {other:?}"),
        }
    }
}

/// **A radius not definitely positive is a degenerate operand on every
/// reading of the pair**, coaxial or not, either radius, zero or
/// negative, and in either order: the section value, the march and the
/// C5 table's pose question.
#[test]
fn a_radius_not_definitely_positive_is_degenerate_on_every_reading() {
    let reach = geom_brep::Reach::Measured {
        at: Point3::new(0.0, 0.0, 0.0),
        lever: 1.0,
    };
    for (label, cyl, sph) in [
        (
            "negative cylinder, coaxial",
            cylinder(-0.5),
            sphere(0.0, 1.0),
        ),
        (
            "negative cylinder, off the axis",
            cylinder(-0.5),
            sphere(0.6, 0.3),
        ),
        ("negative sphere, coaxial", cylinder(0.5), sphere(0.0, -0.7)),
        (
            "negative sphere, off the axis",
            cylinder(0.5),
            sphere(0.4, -0.3),
        ),
        ("zero sphere, off the axis", cylinder(0.5), sphere(0.6, 0.0)),
        (
            "zero cylinder, off the axis",
            cylinder(0.0),
            sphere(0.4, 0.3),
        ),
    ] {
        for (a, b) in [(&cyl, &sph), (&sph, &cyl)] {
            assert!(
                matches!(
                    geom_brep::pair_section(a, b, None, band()),
                    Err(SectionError::DegenerateOperand { .. })
                ),
                "{label}: the section value"
            );
            let slab = geom_brep::SsiDomain {
                center: Point3::new(0.0, 0.0, 0.0),
                half_extent: 2.0,
                extent: 2.0,
                floor_scale: 1.0,
            };
            assert!(
                matches!(
                    geom_brep::cylinder_sphere_ssi(a, b, slab, band()),
                    Err(SsiError::RadiusNotPositive { .. })
                ),
                "{label}: the march"
            );
            let served = geom_brep::route_pose(a, b, &reach, band())
                .unwrap_or_else(|e| panic!("{label}: {e:?}"));
            assert!(!served.implemented, "{label}: the pose question");
        }
    }
}

/// **A radius in the band escalates by its name**, in either order: a
/// sphere `2.865·zero` in radius whose centre lies that far outside the
/// wall, so it touches it, reads its radius undecided before any reach
/// or girdle (the fuzzer's pose 1562 at ε = 1e-6, where the retired
/// guard, which never read the radius, called it a touch); and a
/// cylinder that thin.
#[test]
fn a_radius_in_the_band_escalates_by_its_name() {
    let z = band().zero();
    let (r, small) = (0.190_833_873_299_757_15, 2.865_155_564_080_179_6 * z);
    for (label, cyl, sph, want, predicate) in [
        (
            "sphere",
            cylinder(r),
            sphere(r + small, small),
            geom_brep::SectionRadius::Sphere,
            "cs_sphere_radius",
        ),
        (
            "cylinder",
            cylinder(small),
            sphere(0.4, 0.3),
            geom_brep::SectionRadius::Cylinder,
            "cs_cylinder_radius",
        ),
    ] {
        for (a, b) in [(&cyl, &sph), (&sph, &cyl)] {
            match geom_brep::pair_section(a, b, None, band()) {
                Err(SectionError::RadiusEscalated { radius, diag }) => {
                    assert_eq!(radius, want, "{label}");
                    assert_eq!(diag.predicate, Some(predicate), "{label}");
                }
                other => panic!("{label}: expected its radius escalated, got {other:?}"),
            }
        }
    }
}

/// **The general rung carries through its door**: the lone loop marched
/// over the sphere's box at `f64`, one closed fitted component with its
/// certificate, the witness's foot on it; without the door the carrying
/// refuses by the scalar's name. Seated at an interior parameter it is
/// the same locus run from that point round to it, certified again; at
/// its start it is kept.
#[test]
fn the_general_rung_carries_through_its_door() {
    let (cyl, sph) = (cylinder(0.5), sphere(0.5, 0.3));
    let section = built(&cyl, &sph);
    assert!(
        matches!(
            section.carry(None, band()),
            Err(CarryError::NoLane { scalar: "f64" })
        ),
        "no door, no march"
    );
    let carried = section
        .carry(Some(SsiLane::marched()), band())
        .expect("the lone loop marches");
    let [c] = carried.components() else {
        panic!("one carried component");
    };
    assert!(
        c.closed() && c.certificate.is_some(),
        "a closed certified loop"
    );
    let witness = section.counted().expect("a counted section")[0]
        .witness
        .unwrap();
    assert_eq!(carried.foot(witness, band()).unwrap().component, 0);
    // Seated at an interior parameter: the loop opened there, certified
    // again, the same locus run from that point round to it.
    let (lo, hi) = c.span;
    let at = lo + 0.37 * (hi - lo);
    assert!(
        matches!(
            section.seat(None, c, at, band()),
            Err(CarryError::NoLane { scalar: "f64" })
        ),
        "no door, no seat"
    );
    let seated = section
        .seat(Some(SsiLane::marched()), c, at, band())
        .expect("the loop seats");
    assert!(
        seated.closed() && seated.certificate.is_some(),
        "the seated loop is closed and certified again"
    );
    let (s0, s1) = seated.span;
    let start = seated.carrier.eval(s0);
    assert!(
        (start - c.carrier.eval(at)).norm() < band().zero(),
        "the seated loop starts at the seat"
    );
    assert!(
        (seated.carrier.eval(s1) - start).norm() < band().zero(),
        "the seated loop ends where it starts"
    );
    for k in 1..64 {
        let p = seated.carrier.eval(s0 + (s1 - s0) * f64::from(k) / 64.0);
        let foot = c.carrier.foot(p, at);
        assert!(
            (c.carrier.eval(foot.t()) - p).norm() < band().zero(),
            "the seated loop's point {k} lies on the loop"
        );
    }
    // At the domain's end the loop starts there already.
    let kept = section
        .seat(Some(SsiLane::marched()), c, lo, band())
        .expect("a seat at the start");
    assert_eq!(kept.span, c.span, "a loop seated at its start is kept");
}

/// **A seated loop is the loop, point for point, and its certificate is
/// its own**: over five poses, one loop and two, every component seated
/// at nineteen interior parameters and at six within rounding of its
/// ends is closed and certified, and every point of it is the parent's
/// point a whole period back where it runs past the parent's end, to
/// 1e-12. A loop opened elsewhere than at its start carries the
/// certificate the general rung's certifying door gives its own net,
/// not the parent's copied (read again on a few seats).
#[test]
fn a_seated_loop_is_the_loop_point_for_point() {
    for (d, big) in [
        (0.5, 0.3),
        (0.35, 0.3),
        (0.2, 0.699),
        (0.1, 0.7),
        (0.62, 0.15),
    ] {
        let (cyl, sph) = (cylinder(0.5), sphere(d, big));
        let section = built(&cyl, &sph);
        let carried = section.carry(Some(SsiLane::marched()), band()).unwrap();
        for c in carried.components() {
            let (lo, hi) = c.span;
            let period = hi - lo;
            let mut ats: Vec<f64> = (1..20)
                .map(|k| lo + period * f64::from(k) / 20.0 + 1e-3)
                .collect();
            ats.extend([
                lo + 1e-15,
                lo + 1e-12,
                lo + 1e-9,
                hi - 1e-9,
                hi - 1e-12,
                hi - 1e-15,
            ]);
            for (i, at) in ats.into_iter().enumerate() {
                let seated = section
                    .seat(Some(SsiLane::marched()), c, at, band())
                    .unwrap_or_else(|e| panic!("d {d}, R {big}: seat {at}: {e:?}"));
                assert!(
                    seated.closed() && seated.certificate.is_some(),
                    "d {d}, R {big}: seat {at} is closed and certified"
                );
                let (s0, s1) = seated.span;
                for k in 0..=4000 {
                    let u = s0 + (s1 - s0) * f64::from(k) / 4000.0;
                    let v = if u <= hi { u } else { u - period };
                    let gap = (seated.carrier.eval(u) - c.carrier.eval(v)).norm();
                    assert!(gap < 1e-12, "d {d}, R {big}: seat {at}: {gap:e}");
                }
                if i % 7 == 0 && seated.span != c.span {
                    let Curve3::Nurbs(net) = &seated.carrier else {
                        panic!("a seated loop's carrier is a spline");
                    };
                    let own = geom_brep::certify_rung3(
                        net,
                        None,
                        &geom_brep::SsiOperand::Analytic(&cyl),
                        &geom_brep::SsiOperand::Analytic(&sph),
                        big * 2.0,
                        band(),
                    )
                    .unwrap();
                    let held = seated.certificate.unwrap();
                    assert_eq!(
                        (held.hull_sup, held.on_locus_max, held.tube_boxes),
                        (own.hull_sup, own.on_locus_max, own.tube_boxes),
                        "d {d}, R {big}: seat {at} holds its own net's certificate"
                    );
                }
            }
        }
    }
}

/// **Every other kind pair is read elsewhere**, with `route`'s own arm.
#[test]
fn every_other_pair_is_read_elsewhere() {
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    for (a, b) in [
        (&plane, &sphere(0.0, 1.0)),
        (&sphere(0.0, 1.0), &sphere(0.5, 1.0)),
    ] {
        match geom_brep::pair_section(a, b, None, band()) {
            Ok(SectionRead::Elsewhere(route)) => {
                assert_eq!(route, geom_brep::route(a.kind(), b.kind()));
            }
            other => panic!(
                "{:?} × {:?}: read elsewhere, got {other:?}",
                a.kind(),
                b.kind()
            ),
        }
    }
}

// ---------------------------------------------------------------------
// The pose against the formulas it retired, and the sites near the node
// (the dual review of SECT step 1a; R1's and R2's rows, deduplicated).
// ---------------------------------------------------------------------

/// A unit vector square to `a`.
fn square_to(a: Vec3<f64>) -> Vec3<f64> {
    let t = if a.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let u = t - a * t.dot(a);
    u * (1.0 / u.norm())
}

/// A cylinder of radius `r` about `axis` through `origin`.
fn cylinder_about(origin: Point3<f64>, axis: Vec3<f64>, r: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin,
        axis,
        radius: r,
        u_ref: square_to(axis),
    }
}

/// A sphere of radius `big` about `center`.
fn sphere_at(center: Point3<f64>, big: f64) -> Surface<f64> {
    Surface::Sphere {
        center,
        radius: big,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn sign(m: f64) -> Option<geom_core::Sign> {
    geom_core::k_stats::decide("retired", geom_core::Margin::of(m), band()).ok()
}

/// What the interior-loop guard reads off a pair.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Class {
    Empty,
    Lone,
    Pair,
    Touch,
    Tangent,
}

/// **The retired guard arm** (`section_cert::sphere_cylinder` as it
/// stood before step 1a), transcribed: the reach `R − |e − r|` read
/// first, then the girdle `e + r − R`, at the centre's offset `e` from
/// the axis. A `Zero` reach beside a `Positive` girdle is the touch;
/// any other `Zero` or undecided row is the tangency.
fn retired_guard(e: f64, r: f64, big: f64) -> Class {
    use geom_core::Sign;
    match (sign(big - (e - r).abs()), sign(e + r - big)) {
        (None, _) => Class::Tangent,
        (Some(Sign::Zero), Some(Sign::Positive)) => Class::Touch,
        (Some(Sign::Zero), _) | (Some(_), None | Some(Sign::Zero)) => Class::Tangent,
        (Some(Sign::Negative), Some(_)) => Class::Empty,
        (Some(Sign::Positive), Some(Sign::Positive)) => Class::Lone,
        (Some(Sign::Positive), Some(Sign::Negative)) => Class::Pair,
    }
}

/// **The retired coincidence row** (`glue::frame_rows` over
/// `cylinder_sphere_section`): written where the offset decided `Zero`,
/// both radii decided positive and `R − r` decided.
fn retired_row(e: f64, r: f64, big: f64) -> bool {
    use geom_core::Sign;
    sign(e) == Some(Sign::Zero)
        && sign(r) == Some(Sign::Positive)
        && sign(big) == Some(Sign::Positive)
        && sign(big - r).is_some()
}

/// The offset as the pose reads it: an offset decided `Zero` is the
/// coaxial pose, whose reach and girdle are `±(R − r)`. The retired arm
/// read `|e − r|` with the measured residual `e`, so where `R − r` lies
/// within `e ≤ zero` of a band edge the two can read different signs;
/// the pose's reading is the coaxial one it decided.
fn decided_offset(e: f64) -> f64 {
    if sign(e) == Some(geom_core::Sign::Zero) {
        0.0
    } else {
        e
    }
}

/// The value's reading of a pair: its guard class, and whether it
/// records a coincidence.
fn read_class(a: &Surface<f64>, b: &Surface<f64>) -> (Class, bool) {
    match geom_brep::pair_section(a, b, None, band()) {
        Ok(SectionRead::Built(s)) => {
            let class = if s.tangency().is_some() {
                match s.counted().expect("a counted section") {
                    [c] if c.shape == ComponentShape::Point && c.witness.is_some() => Class::Touch,
                    _ => Class::Tangent,
                }
            } else {
                match s.counted().expect("a counted section").len() {
                    0 => Class::Empty,
                    1 => Class::Lone,
                    2 => Class::Pair,
                    n => panic!("{n} components"),
                }
            };
            (class, s.coincidence().is_some())
        }
        Err(SectionError::Escalated(_) | SectionError::RadiusEscalated { .. }) => {
            (Class::Tangent, false)
        }
        other => panic!("positive radii build or escalate, got {other:?}"),
    }
}

/// **The boundary enumeration**: every offset and radius boundary of the
/// pose, at and either side of the band's edges, in both orders. Off the
/// band the value reads the retired arm's class and writes the retired
/// coincidence row exactly; an offset inside the band (`zero < e <
/// escalate`) is the one ruling step 1a moved: the pose escalates the
/// offset itself, so the guard reads a tangency where the retired arm
/// read the reach and girdle of an off-axis pose. An offset decided
/// `Zero` but not exactly zero is read as the coaxial pose
/// ([`decided_offset`]).
#[test]
fn the_pose_reads_the_retired_formulas_at_every_boundary() {
    let b = band();
    let (z, k) = (b.zero(), b.escalate());
    let r = 0.5;
    let offsets = [
        0.0,
        0.3 * z,
        z,
        2.0 * z,
        0.5 * (z + k),
        k,
        1.5 * k,
        3.0 * k,
        1e-3,
        0.1,
        0.2,
        0.5,
        0.7,
        1.2,
    ];
    let nudges = [
        -3.0 * k,
        -k,
        -0.5 * (z + k),
        -z,
        -0.3 * z,
        0.0,
        0.3 * z,
        z,
        0.5 * (z + k),
        k,
        3.0 * k,
        0.05,
        -0.05,
    ];
    let (mut n, mut in_band) = (0, 0);
    for e in offsets {
        let sph_centre = Point3::new(e, 0.0, 0.2);
        for base in [(e - r).abs(), e + r, r, 0.3, 0.9] {
            for nudge in nudges {
                let big = base + nudge;
                if big <= 3.0 * k {
                    continue;
                }
                let (cyl, sph) = (cylinder(r), sphere_at(sph_centre, big));
                let old = (
                    retired_guard(decided_offset(e), r, big),
                    retired_row(e, r, big),
                );
                for (x, y) in [(&cyl, &sph), (&sph, &cyl)] {
                    n += 1;
                    let new = read_class(x, y);
                    let what = format!(
                        "e {e:e}, R−|e−r| {:e}, R−(e+r) {:e}, R−r {:e}",
                        big - (e - r).abs(),
                        big - (e + r),
                        big - r
                    );
                    if e > z && e < k {
                        in_band += 1;
                        assert_eq!(new, (Class::Tangent, false), "in the band: {what}");
                    } else {
                        assert_eq!(new, old, "off the band: {what}");
                    }
                }
            }
        }
    }
    assert!(
        n > 1000 && in_band > 100,
        "{n} readings, {in_band} in the band"
    );
}

/// **The counterexample search**: random poses, any axis and origin,
/// pushed onto their coaxiality, their tangencies and the band's edge,
/// in both orders. Off the band the value writes the retired coincidence
/// row exactly where it was written and reads the retired guard's class.
#[test]
fn no_pose_off_the_band_moves_from_the_retired_formulas() {
    use test_utils::fuzz;
    let mut g = fuzz::start("pair_section::retired_formulas");
    let b = band();
    let (mut rows, mut checked) = (0, 0);
    let poses = fuzz::scaled(5_000);
    for k in 0..poses {
        let origin = Point3::new(g.unit() - 0.5, g.unit() - 0.5, g.unit() - 0.5);
        let axis = loop {
            let v = Vec3::new(g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0));
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                break v * (1.0 / n);
            }
        };
        let r = 0.05 + g.unit();
        let off = match k % 3 {
            0 => 0.0,
            1 => b.escalate() * 3.0 * g.unit(),
            _ => 1.5 * g.unit(),
        };
        let big = match k % 7 {
            0 => r,
            1 => (off - r).abs(),
            2 => off + r,
            3 => (off - r).abs() + b.zero() * 0.5,
            _ => 0.05 + 2.0 * g.unit(),
        };
        // The retired formulas read a radius decided positive; one in the
        // band escalates by its name instead
        // ([`a_radius_in_the_band_escalates_by_its_name`]).
        if big <= b.escalate() * 1.001 || (off > b.zero() * 0.999 && off < b.escalate() * 1.001) {
            continue;
        }
        let c = origin + axis * g.range(-1.0, 1.0) + square_to(axis) * off;
        let (cyl, sph) = (cylinder_about(origin, axis, r), sphere_at(c, big));
        // The retired formulas read the offset as measured, not as drawn.
        let q = c - origin;
        let e = (q - axis * q.dot(axis)).norm();
        let old = (
            retired_guard(decided_offset(e), r, big),
            retired_row(e, r, big),
        );
        for (x, y) in [(&cyl, &sph), (&sph, &cyl)] {
            let new = read_class(x, y);
            assert_eq!(
                new,
                old,
                "pose {k} (off {off:e}, r {r}, R {big}); {}",
                fuzz::replay()
            );
            rows += usize::from(new.1);
            checked += 1;
        }
    }
    assert!(
        rows > poses / 10 && checked > poses,
        "{rows} rows, {checked} readings; {}",
        fuzz::replay()
    );
}

/// The wall point at azimuth `θ` of the section of [`cylinder`]`(r)` and
/// [`sphere`]`(d, big)`, on the upper (`+1`) or lower (`−1`) sheet;
/// `None` where the azimuth has none.
fn wall_point(r: f64, d: f64, big: f64, theta: f64, sheet: f64) -> Option<Point3<f64>> {
    let h2 = big * big - r * r - d * d + 2.0 * r * d * theta.cos();
    (h2 >= 0.0).then(|| Point3::new(r * theta.cos(), r * theta.sin(), 0.2 + sheet * h2.sqrt()))
}

/// Whether the cyclic sequence `ts` on a closed span of length `period`
/// runs one way round exactly once.
fn turns_once(ts: &[f64], period: f64) -> bool {
    let steps: Vec<f64> = ts
        .iter()
        .zip(ts.iter().cycle().skip(1))
        .map(|(a, b)| (b - a).rem_euclid(period))
        .collect();
    let fwd: f64 = steps.iter().sum();
    let back: f64 = steps.iter().map(|s| period - s).sum();
    (fwd - period).abs() < 1e-6 * period || (back - period).abs() < 1e-6 * period
}

/// **Two loops keep their sites and their order, down to the node**:
/// `R` past `d + r` by a wide margin and by hairs, so the loops nearly
/// meet at the far ruling. In either order every analytic site of one
/// sheet has its foot on one component, the two sheets' on two, and the
/// feet run round each loop in the azimuth's order, once. Marches: in
/// the slow set.
#[test]
fn two_loops_keep_their_sites_and_order_near_the_node() {
    // Pinned red at 1e-6 since PR 6011: the closest pair escalates at
    // the straddle
    // (`work/sect/main-red-five-torus-section-rows-refuse-at-1e-6-since-components-by-foot.md`).
    test_utils::pinned_red::row(
        Tol::witness().eps(),
        &[1e-6],
        "two loops near the node",
        &[
            "d 0.2, R 0.701, θ 2.7842143000352855: Escalated(Indeterminate { margin: MarginDiag(Enclosure { lo: 0.10006928966127418, hi: 0.2063515605648144, tag: None, zero: ZeroWord(0) }), terminal_sliver: false, band: Band { zero: 0.10979844554160123, escalate: 0.10980744554160123 }, predicate: Some(\"curve_carrier_straddle\") })",
        ],
        two_loops_keep_their_sites_and_order_near_the_node_row,
    );
}

/// The row [`two_loops_keep_their_sites_and_order_near_the_node`] runs, under its pin.
fn two_loops_keep_their_sites_and_order_near_the_node_row() {
    let r = 0.5;
    for (d, big) in [(0.1, 0.7), (0.2, 0.71), (0.2, 0.701)] {
        let (cyl, sph) = (cylinder(r), sphere(d, big));
        for (x, y) in [(&cyl, &sph), (&sph, &cyl)] {
            let s = built(x, y);
            let c = s
                .carry(Some(SsiLane::marched()), band())
                .unwrap_or_else(|e| panic!("d {d}, R {big}: {e:?}"));
            assert_eq!(c.components().len(), 2, "d {d}, R {big}");
            let mut seen = Vec::new();
            for sheet in [1.0, -1.0] {
                let mut ts = Vec::new();
                let mut want = None;
                for i in 0..97 {
                    // Densest near the far ruling, θ = π.
                    let u = f64::from(i) / 97.0;
                    let theta = std::f64::consts::PI * (1.0 + (2.0 * u - 1.0).powi(3));
                    let p = wall_point(r, d, big, theta, sheet).unwrap();
                    let at = format!("d {d}, R {big}, θ {theta}");
                    let f = c.foot(p, band()).unwrap_or_else(|e| panic!("{at}: {e:?}"));
                    assert_eq!(*want.get_or_insert(f.component), f.component, "{at}");
                    ts.push(f.t);
                }
                let want = want.expect("a site on the sheet");
                assert!(
                    !seen.contains(&want),
                    "d {d}, R {big}: each sheet its own loop"
                );
                seen.push(want);
                let (lo, hi) = c.components()[want].span;
                assert!(turns_once(&ts, hi - lo), "d {d}, R {big}, sheet {sheet}");
            }
        }
    }
}

/// **One loop keeps its order, down to the node**: the centre on the
/// wall, and `R` short of `d + r` by hairs, so the loop's two lobes pass
/// close at the far side. In either order every site's foot lies on the
/// one component and the feet run round it once in the loop's own order.
/// Marches: in the slow set.
#[test]
fn one_loop_keeps_its_order_near_the_node() {
    let r = 0.5;
    for (d, big) in [(0.5, 0.3), (0.2, 0.69), (0.2, 0.699)] {
        let (cyl, sph) = (cylinder(r), sphere(d, big));
        let t0 = ((r * r + d * d - big * big) / (2.0 * r * d)).acos();
        for (x, y) in [(&cyl, &sph), (&sph, &cyl)] {
            let c = built(x, y)
                .carry(Some(SsiLane::marched()), band())
                .unwrap_or_else(|e| panic!("d {d}, R {big}: {e:?}"));
            assert_eq!(c.components().len(), 1, "d {d}, R {big}");
            let mut ts = Vec::new();
            for i in 0..200 {
                let tau = 4.0 * t0 * (f64::from(i) + 0.5) / 200.0;
                let (theta, sheet) = if tau < 2.0 * t0 {
                    (tau - t0, 1.0)
                } else {
                    (3.0 * t0 - tau, -1.0)
                };
                let Some(p) = wall_point(r, d, big, theta, sheet) else {
                    continue;
                };
                let f = c
                    .foot(p, band())
                    .unwrap_or_else(|e| panic!("d {d}, R {big}, τ {tau}: {e:?}"));
                assert_eq!(f.component, 0);
                ts.push(f.t);
            }
            let (lo, hi) = c.components()[0].span;
            assert!(turns_once(&ts, hi - lo), "d {d}, R {big}");
        }
    }
}
