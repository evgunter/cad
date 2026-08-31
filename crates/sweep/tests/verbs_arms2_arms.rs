//! **VERBS-ARMS-2 arm rows: the curved-support constant-radius arms,
//! at their own closed form, in BOTH material configurations.**
//!
//! The rolling ball of radius `r` tangent to two supports is defined by
//! two equations and nothing else: its centre is at distance exactly `r`
//! from each support, on the side its material is. These rows check that
//! specification directly — for every arm the table carries, with the
//! stored sense bit set both ways — rather than re-deriving each arm's
//! algebra a second time and comparing two spellings of one mistake.
//!
//! What makes each row go red:
//!
//! - **The defining equations** — the signed distance from the ball
//!   centre to each support, measured in that support's own closed form,
//!   must be `−r·side`. Red if a trace's normal, its material fold, or
//!   the crossing itself is wrong by any amount.
//! - **Both configurations, per arm** — the same pair with the sense bit
//!   flipped must move the centre to the OTHER side and satisfy the
//!   flipped equations. Red if an arm silently assumes one configuration
//!   (the ARMS-1 lesson: `plane_sphere_blend` assumed the pocket and only
//!   the dome exposed it).
//! - **The trimlines are the contact circles** — each lies ON its
//!   support (signed distance zero) at distance exactly `r` from the
//!   centre, and the setback is its Euclidean displacement from the rim.
//! - **The branch** — as `r → 0` the centre returns the RIM POINT, which
//!   is the structural answer to "which of the two circles the offset
//!   surfaces meet in is my edge". Red if a `√` picks the far crossing.
//! - **The spine** — a coaxial arm's torus is centred on the axis at the
//!   ball centre's own level with major radius the centre's own radial
//!   coordinate; a ruled arm's cylinder is centred ON the ball centre.
//! - **The plane–sphere cross-check** — the shared reduction, run on the
//!   pair that already had its own arm, agrees with `plane_sphere_blend`
//!   to the last bit of the tolerance. One derivation, two doors.
//! - **The refusal roster** — every `BlendArm::name` appears in the
//!   `SpineUnsupported` payload the front door hands back.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Point3, Vec3};
use sweep::blend::arms::{BlendArm, EdgeBlend, Meridian, Ruling, plane_sphere_blend};

const EPS: f64 = 1e-12;

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// The signed distance from `p` to `s`, POSITIVE on the chart normal's
/// side — each surface's own closed form, written independently of the
/// arms so the check is a specification and not a restatement.
fn signed_dist(s: &Surface<f64>, p: Point3<f64>) -> f64 {
    match *s {
        Surface::Plane { origin, normal, .. } => (p - origin).dot(normal),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let d = p - origin;
            (d - axis * d.dot(axis)).norm() - radius
        }
        Surface::Sphere { center, radius, .. } => (p - center).norm() - radius,
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let d = p - apex;
            let v = d.dot(axis);
            let rho = (d - axis * v).norm();
            rho * half_angle.cos() - v.abs() * half_angle.sin()
        }
        _ => panic!("no closed form for this support kind"),
    }
}

/// One arm fixture: the two supports, the rim point they share, and the
/// sheet the pair's symmetry gives.
struct Row {
    arm: BlendArm,
    a: Surface<f64>,
    b: Surface<f64>,
    rim: Point3<f64>,
    /// The coaxial sheet, when the pair is one; `None` for a ruled pair.
    axis: Option<(Point3<f64>, Vec3<f64>)>,
    /// The ruling, when the pair is one.
    tau: Option<Vec3<f64>>,
}

fn cone(apex_y: f64, half_angle: f64) -> Surface<f64> {
    Surface::Cone {
        apex: p3(0.0, apex_y, 0.0),
        axis: v3(0.0, 1.0, 0.0),
        half_angle,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

fn cyl(origin: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>) -> Surface<f64> {
    Surface::Cylinder {
        origin,
        axis,
        radius,
        u_ref,
    }
}

fn sphere(center: Point3<f64>, radius: f64) -> Surface<f64> {
    Surface::Sphere {
        center,
        radius,
        axis: v3(0.0, 1.0, 0.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

fn plane(origin: Point3<f64>, normal: Vec3<f64>, u_ref: Vec3<f64>) -> Surface<f64> {
    Surface::Plane {
        origin,
        normal,
        u_ref,
    }
}

/// Every curved row, each on the SAME rim point `(0.8, 0.6, 0)` so the
/// fixtures differ only in what meets there.
///
/// The coaxial rows sit on the y-axis; the ruled two on the ruling `+z`
/// through the same point (two unit cylinders 1.6 apart, and one of them
/// cut by a plane through the rim containing the ruling).
fn rows() -> Vec<Row> {
    let up = v3(0.0, 1.0, 0.0);
    let rim = p3(0.8, 0.6, 0.0);
    // The cone through the rim whose generator falls three units of
    // radius per unit of axis: apex at 0.6 + 0.8/3.
    let steep = cone(0.6 + 0.8 / 3.0, 3.0f64.atan());
    // A second cone through the same rim, opening the other way at 45°.
    let shallow = cone(0.6 - 0.8, core::f64::consts::FRAC_PI_4);
    let unit_sphere = sphere(p3(0.0, 0.0, 0.0), 1.0);
    // The second sphere of the sphere–sphere pair: the unit sphere
    // through the same rim from the other side, centred at `(0, 1.2)`.
    let mate_sphere = sphere(p3(0.0, 1.2, 0.0), 1.0);
    let coaxial_cyl = cyl(p3(0.0, 0.0, 0.0), up, 0.8, v3(1.0, 0.0, 0.0));
    let flat = plane(p3(0.0, 0.6, 0.0), up, v3(1.0, 0.0, 0.0));
    let sheet = Some((p3(0.0, 0.6, 0.0), up));
    let tau = v3(0.0, 0.0, 1.0);
    let ruled_a = cyl(p3(0.0, 0.0, 0.0), tau, 1.0, v3(1.0, 0.0, 0.0));
    let ruled_b = cyl(p3(1.6, 0.0, 0.0), tau, 1.0, v3(1.0, 0.0, 0.0));
    let ruled_plane = plane(p3(0.0, 0.6, 0.0), up, tau);
    vec![
        Row {
            arm: BlendArm::SphereConeTorus,
            a: unit_sphere.clone(),
            b: steep.clone(),
            rim,
            axis: sheet,
            tau: None,
        },
        Row {
            arm: BlendArm::ConePlaneTorus,
            a: steep.clone(),
            b: flat.clone(),
            rim,
            axis: sheet,
            tau: None,
        },
        Row {
            arm: BlendArm::ConeConeTorus,
            a: steep.clone(),
            b: shallow,
            rim,
            axis: sheet,
            tau: None,
        },
        Row {
            arm: BlendArm::CylinderConeTorus,
            a: coaxial_cyl.clone(),
            b: steep,
            rim,
            axis: sheet,
            tau: None,
        },
        Row {
            arm: BlendArm::CylinderSphereTorus,
            a: coaxial_cyl.clone(),
            b: unit_sphere.clone(),
            rim,
            axis: sheet,
            tau: None,
        },
        Row {
            arm: BlendArm::SphereSphereTorus,
            a: unit_sphere.clone(),
            b: mate_sphere,
            rim,
            axis: sheet,
            tau: None,
        },
        Row {
            arm: BlendArm::CylinderPlaneTorus,
            a: coaxial_cyl,
            b: flat,
            rim,
            axis: sheet,
            tau: None,
        },
        Row {
            arm: BlendArm::CylinderCylinderCylinder,
            a: ruled_a.clone(),
            b: ruled_b,
            rim,
            axis: None,
            tau: Some(tau),
        },
        Row {
            arm: BlendArm::CylinderPlaneCylinder,
            a: ruled_a,
            b: ruled_plane,
            rim,
            axis: None,
            tau: Some(tau),
        },
    ]
}

impl Row {
    /// Build this row's blend at `radius` with the two stored sense bits.
    fn blend(&self, radius: f64, senses: (bool, bool)) -> (EdgeBlend<f64>, Point3<f64>) {
        if let Some((origin, axis)) = self.axis {
            let sheet = Meridian {
                origin,
                axis,
                rim: self.rim,
            };
            let (ta, da) = sheet.trace(&self.a, senses.0).expect("a coaxial trace");
            let (tb, db) = sheet.trace(&self.b, senses.1).expect("a coaxial trace");
            assert!(
                da.abs() < EPS && db.abs() < EPS,
                "{}: the fixture is coaxial, departures {da}/{db}",
                self.arm.name()
            );
            let b = sheet.blend(ta, tb, radius);
            let center =
                sweep::blend::arms::sheet_center(self.rim, sheet.sheet_normal(), ta, tb, radius);
            (b, center)
        } else {
            let sheet = Ruling {
                tau: self.tau.expect("a ruled row carries its ruling"),
                rim: self.rim,
                lever: 1.0,
            };
            let (ta, da) = sheet.trace(&self.a, senses.0).expect("a ruled trace");
            let (tb, db) = sheet.trace(&self.b, senses.1).expect("a ruled trace");
            assert!(
                da.abs() < EPS && db.abs() < EPS,
                "{}: the fixture shares its ruling, departures {da}/{db}",
                self.arm.name()
            );
            let b = sheet.blend(ta, tb, radius);
            let center = sweep::blend::arms::sheet_center(self.rim, sheet.tau, ta, tb, radius);
            (b, center)
        }
    }
}

/// **The defining equations, both configurations, every arm.** The ball
/// centre is at distance exactly `r` from each support on its material
/// side, and each trimline is the contact circle there.
#[test]
fn every_curved_arm_solves_the_rolling_ball_equations_in_both_configurations() {
    let radius = 0.05;
    for row in rows() {
        let mut centers = Vec::new();
        for senses in [(true, true), (false, false), (true, false), (false, true)] {
            let (blend, center) = row.blend(radius, senses);
            let side = |b: bool| if b { 1.0 } else { -1.0 };
            for (s, sense, which) in [(&row.a, senses.0, "first"), (&row.b, senses.1, "second")] {
                let d = signed_dist(s, center);
                assert!(
                    (d + radius * side(sense)).abs() < EPS,
                    "{} {senses:?}: the ball centre is {d} from its {which} support, \
                     wanted {}",
                    row.arm.name(),
                    -radius * side(sense)
                );
            }
            // The trimlines are the contact loci: on the support, at
            // distance `r` from the centre, and the setback is the
            // Euclidean displacement from the rim.
            let radial = match row.axis {
                Some((o, k)) => {
                    let d = row.rim - o;
                    (d - k * d.dot(k)).normalize()
                }
                None => v3(0.0, 0.0, 0.0),
            };
            for (s, trim, which) in [
                (&row.a, &blend.trim_a, "first"),
                (&row.b, &blend.trim_b, "second"),
            ] {
                let q = match trim.0 {
                    Curve3::Circle { center, radius, .. } => center + radial * radius,
                    Curve3::Line { origin, .. } => origin,
                    _ => panic!("a blend trimline is a circle or a line"),
                };
                assert!(
                    signed_dist(s, q).abs() < EPS,
                    "{} {senses:?}: the {which} trimline is {} off its support",
                    row.arm.name(),
                    signed_dist(s, q)
                );
                assert!(
                    ((q - center).norm() - radius).abs() < EPS,
                    "{} {senses:?}: the {which} contact is {} from the centre, wanted {radius}",
                    row.arm.name(),
                    (q - center).norm()
                );
                assert!(
                    ((q - row.rim).norm() - trim.1).abs() < EPS,
                    "{} {senses:?}: the {which} setback {} is not the displacement {}",
                    row.arm.name(),
                    trim.1,
                    (q - row.rim).norm()
                );
            }
            centers.push(center);
        }
        // The fold is LIVE: no two configurations of one arm put the
        // ball in the same place.
        for i in 0..centers.len() {
            for j in (i + 1)..centers.len() {
                assert!(
                    (centers[i] - centers[j]).norm() > 1e-6,
                    "{}: configurations {i} and {j} put the ball centre in one place — \
                     the material-side fold is not being read",
                    row.arm.name()
                );
            }
        }
    }
}

/// **The spine each arm mints.** A coaxial arm's torus sits on the axis
/// at the ball centre's own level, major radius the centre's own radial
/// coordinate and minor the requested one; a ruled arm's cylinder is
/// centred on the ball centre itself and its spine never folds.
#[test]
fn every_curved_arm_mints_its_spine_from_the_ball_centre() {
    let radius = 0.05;
    for row in rows() {
        let (blend, center) = row.blend(radius, (true, true));
        match (row.axis, blend.surface.clone()) {
            (
                Some((o, k)),
                Surface::Torus {
                    center: tc,
                    axis,
                    major_radius,
                    minor_radius,
                    ..
                },
            ) => {
                let d = center - o;
                let along = k * d.dot(k);
                assert!((tc - (o + along)).norm() < EPS, "{}", row.arm.name());
                assert!((axis - k).norm() < EPS, "{}", row.arm.name());
                assert!(
                    (major_radius - (d - along).norm()).abs() < EPS,
                    "{}: major {major_radius} is not the centre's radial coordinate",
                    row.arm.name()
                );
                assert!((minor_radius - radius).abs() < EPS, "{}", row.arm.name());
                assert!(
                    (blend.spine_curvature - 1.0 / major_radius).abs() < EPS,
                    "{}: the spine curvature is 1/s",
                    row.arm.name()
                );
                assert!(
                    major_radius > minor_radius,
                    "{}: the fixture's spine clears its tube",
                    row.arm.name()
                );
            }
            (
                None,
                Surface::Cylinder {
                    origin,
                    axis,
                    radius: cr,
                    ..
                },
            ) => {
                assert!((origin - center).norm() < EPS, "{}", row.arm.name());
                assert!((axis - row.tau.unwrap()).norm() < EPS, "{}", row.arm.name());
                assert!((cr - radius).abs() < EPS, "{}", row.arm.name());
                assert!(
                    blend.spine_curvature == 0.0,
                    "{}: a straight spine has no curvature",
                    row.arm.name()
                );
            }
            (_, other) => panic!("{}: unexpected blend surface {other:?}", row.arm.name()),
        }
    }
}

/// **The branch is the rim's own.** Two offset surfaces of a coaxial
/// pair meet in TWO circles and the blend belongs to exactly one of
/// them — the one that collapses onto the requested edge as the ball
/// shrinks. Red if any closed form picks the far crossing.
#[test]
fn the_ball_centre_returns_the_rim_as_the_radius_vanishes() {
    for row in rows() {
        let mut previous = f64::INFINITY;
        for radius in [1e-2, 1e-4, 1e-6, 1e-8] {
            let (_, center) = row.blend(radius, (true, true));
            let gap = (center - row.rim).norm();
            assert!(
                gap < 100.0 * radius,
                "{}: at r = {radius} the ball centre is {gap} from the rim — the far \
                 crossing, not this edge",
                row.arm.name()
            );
            assert!(gap < previous, "{}: the gap must shrink", row.arm.name());
            previous = gap;
        }
    }
}

/// **One derivation, two doors.** The shared sheet reduction, run on the
/// plane–sphere pair that already had its own closed form, agrees with
/// [`plane_sphere_blend`] — spine, tube, and both trim circles. Red if
/// either spelling drifts from the other.
#[test]
fn the_shared_reduction_agrees_with_the_plane_sphere_arm() {
    let radius = 0.05;
    // The dome's own configuration: the unit sphere's equator on a flat
    // base, material inside both.
    let sphere_c = p3(0.0, 0.0, 0.0);
    let rim = p3(1.0, 0.0, 0.0);
    let n = v3(0.0, -1.0, 0.0);
    for sphere_sense in [true, false] {
        let old = plane_sphere_blend(
            p3(0.0, 0.0, 0.0),
            n,
            v3(1.0, 0.0, 0.0),
            sphere_c,
            1.0,
            radius,
            sphere_sense,
        );
        let sheet = Meridian {
            origin: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 1.0, 0.0),
            rim,
        };
        // The plane's stored normal IS `n` and its material side is the
        // one the outward normal already names, so its sense is `true`.
        let (tp, _) = sheet
            .trace(&plane(p3(0.0, 0.0, 0.0), n, v3(1.0, 0.0, 0.0)), true)
            .unwrap();
        let (ts, _) = sheet.trace(&sphere(sphere_c, 1.0), sphere_sense).unwrap();
        let new = sheet.blend(tp, ts, radius);
        let (
            Surface::Torus {
                center: c0,
                major_radius: m0,
                minor_radius: n0,
                ..
            },
            Surface::Torus {
                center: c1,
                major_radius: m1,
                minor_radius: n1,
                ..
            },
        ) = (old.surface.clone(), new.surface.clone())
        else {
            panic!("both doors mint a torus");
        };
        assert!((c0 - c1).norm() < 1e-12, "sphere_sense = {sphere_sense}");
        assert!((m0 - m1).abs() < 1e-12, "sphere_sense = {sphere_sense}");
        assert!((n0 - n1).abs() < 1e-12, "sphere_sense = {sphere_sense}");
        for (o, w) in [(&old.trim_a, &new.trim_a), (&old.trim_b, &new.trim_b)] {
            let (
                Curve3::Circle {
                    center: a,
                    radius: ra,
                    ..
                },
                Curve3::Circle {
                    center: b,
                    radius: rb,
                    ..
                },
            ) = (o.0.clone(), w.0.clone())
            else {
                panic!("both doors mint circles");
            };
            assert!((a - b).norm() < 1e-12, "sphere_sense = {sphere_sense}");
            assert!((ra - rb).abs() < 1e-12, "sphere_sense = {sphere_sense}");
            // The setbacks agree in MAGNITUDE: the plane–sphere arm keeps
            // a signed one on the plane (a pocket's widening reads
            // positive), the shared reduction the unsigned Euclidean
            // displacement, which is the conservative reading for
            // predicate 2's `gap − setback − setback`.
            assert!(
                (o.1.abs() - w.1).abs() < 1e-12,
                "sphere_sense = {sphere_sense}: setbacks {} vs {}",
                o.1,
                w.1
            );
        }
    }
}

/// **The refusal advertises every arm.** `SpineUnsupported`'s payload is
/// a `&'static str`, so the roster is hand-written; this row is what
/// makes it a checked claim rather than a stale one.
#[test]
fn the_refusal_roster_names_every_arm_and_nothing_else() {
    let roster = sweep::blend::battery::arm_roster();
    // The roster lists PAIRS, so the pair half of each arm's name is
    // what must appear; the chamfer's strip shares the plane–plane row.
    let mut want: Vec<&str> = BlendArm::ALL
        .iter()
        .map(|a| a.name().split(" →").next().unwrap())
        .collect();
    want.sort_unstable();
    want.dedup();
    // The roster's own rows, read back out of the payload text: the
    // parenthesised list, split on the separator it is written with.
    let inner = roster
        .split_once('(')
        .and_then(|(_, rest)| rest.rsplit_once(')'))
        .map(|(inner, _)| inner)
        .expect("the roster payload is a parenthesised list");
    let mut have: Vec<&str> = inner.split('/').map(str::trim).collect();
    have.sort_unstable();
    have.dedup();
    // BOTH directions: a new arm that is not advertised reds, and a
    // roster row whose arm has been retired reds too.
    assert_eq!(
        have, want,
        "the SpineUnsupported roster and the arm table disagree: roster {have:?}, arms \
         {want:?}"
    );
}
