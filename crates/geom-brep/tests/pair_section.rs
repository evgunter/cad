//! **The section value** (`geom_brep::pair_section`): the cylinder ×
//! sphere pose decided once, the components every reader takes, and the
//! carrying that marches the general rung.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{
    CarryError, ClassCarrier, ComponentShape, PairSection, SectionError, SectionRead, SsiError,
    SsiLane, ZeroKind,
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
    match geom_brep::pair_section(a, b, band()) {
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
    let zero = section.coincidence().expect("the coaxiality is recorded");
    assert_eq!(
        (zero.decision, zero.kind),
        ("cs_coaxial", ZeroKind::Coincidence)
    );
    assert!(section.tangency().is_none(), "no tangency");
    assert_eq!(section.components().len(), 2, "two circles");
    for (k, c) in section.components().iter().enumerate() {
        assert_eq!(
            c.winds,
            [true, false],
            "circle {k} winds about the cylinder"
        );
        let ClassCarrier::Closed(curve @ Curve3::Circle { .. }) = &c.carrier else {
            panic!("circle {k}: a closed-form circle, got {:?}", c.carrier);
        };
        for i in 0..6 {
            let p = curve.eval(f64::from(i));
            on_both(p, &cyl, &sph, "a coaxial circle");
            assert_eq!(
                section.component_of(p, band()),
                Ok(k),
                "circle {k} at {p:?}"
            );
        }
    }
    let swapped = built(&sph, &cyl);
    assert_eq!(
        swapped.components()[0].winds,
        [false, true],
        "the order asked of"
    );
    let carried = section.carry(None, band()).expect("closed forms carry");
    let p = match &section.components()[1].carrier {
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
            (zero.decision, zero.kind),
            (decision, ZeroKind::Tangency),
            "{label}"
        );
        let [c] = section.components() else {
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
    assert!(none.components().is_empty(), "the sphere misses the wall");
    let (cyl, sph) = (cylinder(0.5), sphere(0.5, 0.3));
    let one = built(&cyl, &sph);
    let [c] = one.components() else {
        panic!("one loop");
    };
    assert_eq!(c.winds, [false, false], "null on both carriers");
    assert!(matches!(c.carrier, ClassCarrier::General));
    on_both(c.witness.unwrap(), &cyl, &sph, "the lone loop's witness");
    let two = built(&cylinder(0.5), &sphere(0.1, 0.7));
    assert_eq!(two.components().len(), 2, "two loops");
    for c in two.components() {
        assert_eq!(c.winds, [true, false], "each winds about the cylinder");
        assert!(matches!(c.carrier, ClassCarrier::General));
    }
    assert_eq!(two.component_of(Point3::new(0.5, 0.0, 0.8), band()), Ok(0));
    assert_eq!(two.component_of(Point3::new(0.5, 0.0, -0.4), band()), Ok(1));
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
        match geom_brep::pair_section(&cylinder(0.5), &sphere(d, big), b) {
            Err(SectionError::Escalated(diag)) => {
                assert_eq!(diag.predicate, Some(want), "{label}");
            }
            other => panic!("{label}: expected an escalation, got {other:?}"),
        }
    }
}

/// **A radius not definitely positive is a degenerate operand on every
/// reading of the pair**, coaxial or not and in either order: the
/// section value, the C5 coaxial arm and the march.
#[test]
fn a_radius_not_definitely_positive_is_degenerate_on_every_reading() {
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
        ("zero sphere, off the axis", cylinder(0.5), sphere(0.6, 0.0)),
    ] {
        for (a, b) in [(&cyl, &sph), (&sph, &cyl)] {
            assert!(
                matches!(
                    geom_brep::pair_section(a, b, band()),
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
        }
        assert!(
            matches!(
                geom_brep::cylinder_sphere_section(&cyl, &sph, band()),
                Err(SectionError::DegenerateOperand { .. })
            ),
            "{label}: the coaxial arm"
        );
    }
}

/// **The general rung carries through its door**: the lone loop marched
/// over the sphere's box at `f64`, one closed fitted component with its
/// certificate, the witness's foot on it; without the door the carrying
/// refuses by the scalar's name.
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
        c.closed && c.certificate.is_some(),
        "a closed certified loop"
    );
    let witness = section.components()[0].witness.unwrap();
    assert_eq!(carried.foot(witness, band()).unwrap().component, 0);
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
        (&cylinder(1.0), &cylinder(0.5)),
    ] {
        match geom_brep::pair_section(a, b, band()) {
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
