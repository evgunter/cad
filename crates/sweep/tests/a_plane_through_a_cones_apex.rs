//! **A plane through a full cone's apex** cuts it along two rulings,
//! grazes it along one, or meets it at the apex alone. Either way each
//! side is the cone over the part of its base disc on that side, so its
//! volume is `h·A/3`, `A` the disc's segment beyond the line the plane
//! cuts the base's plane in.
//!
//! The cone: profile `(0, 0), (r, 0), (0, h)` revolved whole about `y`,
//! half-angle `α` (`r = h·tan α`), at 30°, 45° and 60°. Its apex vertex
//! has two sectors, each a half-turn of the wall, between the two seam
//! meridians. The poses hold the ruling at azimuth `θ`: grazed along it
//! at every azimuth of `THETAS` with either normal, and turned off
//! tangency about it by each tilt of `TILTS` at `θ ∈ {0, 0.3, 2}`, the
//! seam meridian and two azimuths off it.
//!
//! Each side lands at its closed form and passes tier 3′ (which the
//! split's own gate does not read); a graze lands whole on its material
//! side. The `t = 10⁻³` family turns the plane a
//! sliver `≈ t²/2` deep off the cone, which at ε 1e-6 lies within the
//! band: it lands or refuses typed as measured per ε (`near_tangent`),
//! never an invariant or a wrong body.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use topo::ContactRecords;
use topo::splitting::{SplitError, SplitReduceError, split};

/// The azimuths each graze is posed at (`split_tangent_edge_curved.rs`'s).
const THETAS: [f64; 10] = [
    0.0,
    FRAC_PI_2,
    PI,
    3.0 * FRAC_PI_2,
    0.3,
    1.1,
    2.0,
    2.9,
    4.0,
    5.5,
];

/// The tilts off tangency about the held ruling.
const TILTS: [f64; 9] = [
    NEAR_TANGENT,
    0.05,
    0.4,
    1.0,
    FRAC_PI_2,
    2.0,
    3.0,
    -0.4,
    -1.2,
];

/// The tilt whose sliver lies within the band at ε 1e-6.
const NEAR_TANGENT: f64 = 1e-3;

/// The azimuths each tilt is posed at: the seam meridian and two off it.
const SECANT_AZIMUTHS: [f64; 3] = [0.0, 0.3, 2.0];

/// The cone of half-angle `alpha` and height 1, as a finished body, with
/// its base radius.
fn cone(alpha: f64) -> (f64, topo::AtRestBody<f64>) {
    let r = alpha.tan();
    let lp = bulge_loop(
        [(0.0, 0.0), (r, 0.0), (0.0, 1.0)]
            .iter()
            .map(|&(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
    );
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let body = sweep::revolve(
        &vp,
        sweep::RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        sweep::Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    (
        r,
        sweep::test_support::finished("the cone", body, Tol::witness()),
    )
}

/// One plane through the apex.
struct Pose {
    label: String,
    /// The held ruling's foot on the base circle.
    foot: Point3<f64>,
    normal: Vec3<f64>,
    near_tangent: bool,
    theta: f64,
}

/// Every pose (module docs) on the cone of base radius `r`.
fn poses(r: f64) -> Vec<Pose> {
    let at = |a: f64, t: f64, s: f64| {
        let axis = Vec3::new(0.0, 1.0, 0.0);
        let radial = Vec3::new(a.cos(), 0.0, a.sin());
        let ruling = (axis - radial * r).normalize();
        let outward = (radial + axis * r).normalize();
        Pose {
            label: format!("θ = {a}, tilt {t}, normal {s}·n"),
            foot: Point3::new(0.0, 0.0, 0.0) + radial * r,
            normal: (outward * t.cos() + ruling.cross(outward) * t.sin()) * s,
            near_tangent: t == NEAR_TANGENT,
            theta: a,
        }
    };
    let grazes = THETAS.iter().map(|&a| (a, 0.0));
    let secants = SECANT_AZIMUTHS
        .iter()
        .flat_map(|&a| TILTS.iter().map(move |&t| (a, t)));
    grazes
        .chain(secants)
        .flat_map(|(a, t)| [1.0, -1.0].map(|s| at(a, t, s)))
        .collect()
}

/// Each side's closed form, Above then Below: the cone over the base
/// disc's part on that side, and how deep that part reaches in from the
/// disc's rim (zero where the side is empty).
fn closed_forms(r: f64, pose: &Pose) -> [(f64, f64); 2] {
    let n = pose.normal;
    let across = Vec3::new(n.x, 0.0, n.z);
    // The base's points `X` above the plane are those with
    // `n̂_xz · X > δ`, the line's signed distance from the centre.
    let delta = (pose.foot - Point3::new(0.0, 0.0, 0.0)).dot(across) / across.norm();
    let segment = |d: f64| {
        let d = d.clamp(-r, r);
        r * r * (d / r).acos() - d * (r * r - d * d).sqrt()
    };
    let above = segment(delta);
    [
        (above / 3.0, (r - delta).max(0.0)),
        ((PI * r * r - above) / 3.0, (r + delta).max(0.0)),
    ]
}

/// What the `t = 10⁻³` family does on the cone of half-angle `deg`°
/// holding the ruling at azimuth `theta`, at the run's ε, as measured:
/// `Some(None)` where it lands, `Some(Some(name))` where it refuses
/// that variant, `None` at an ε no row measured.
///
/// - ε 1e-6: the sliver lies within the band. Off the seam it lands
///   whole on its material side; along the seam the reduction reads the
///   seam edge and the extremal ruling beside it both On
///   (`ConsecutiveOnSectors`,
///   `a-plane-a-milliradian-off-a-cones-seam-through-its-apex-refuses-consecutive-ons-at-1e-6`);
///   at 60° the base circle reads the near graze in band first
///   (`CrossingEscalated`,
///   `a-sixty-degree-cones-base-circle-reads-a-milliradian-graze-in-band-at-1e-6`).
/// - ε 1e-9 and 1e-12: the apex lane's pair reach reads in band
///   (`SectionGermsEscalated`) and then off the apex (`ApexSection`,
///   `a-plane-a-milliradian-off-a-cones-ruling-through-its-apex-reads-its-section-off-the-apex`).
fn near_tangent(deg: f64, theta: f64, eps: f64) -> Option<Option<&'static str>> {
    Some(if eps == 1e-6 {
        if deg == 60.0 {
            Some("CrossingEscalated")
        } else if theta == 0.0 {
            Some("ConsecutiveOnSectors")
        } else {
            None
        }
    } else if eps == 1e-9 {
        Some("SectionGermsEscalated")
    } else if eps == 1e-12 {
        Some("ApexSection")
    } else {
        return None;
    })
}

/// The variant name of a refusal [`near_tangent`] names.
fn refusal_name(e: &SplitError) -> Option<&'static str> {
    match e {
        SplitError::Reduce(SplitReduceError::ConsecutiveOnSectors { .. }) => {
            Some("ConsecutiveOnSectors")
        }
        SplitError::Reduce(SplitReduceError::CrossingEscalated { .. }) => Some("CrossingEscalated"),
        SplitError::Reduce(SplitReduceError::SectionGermsEscalated { .. }) => {
            Some("SectionGermsEscalated")
        }
        SplitError::Reduce(SplitReduceError::ApexSection { .. }) => Some("ApexSection"),
        _ => None,
    }
}

/// Splits the cone of half-angle `deg`° at every pose and checks each
/// side against its closed form and tier 3′.
fn every_pose_answers(deg: f64) {
    let tol = Tol::witness();
    let eps = tol.eps();
    let alpha = deg.to_radians();
    let (r, body) = cone(alpha);
    let whole = PI * r * r / 3.0;
    // The pinned resolution's displacement over the cone's whole boundary.
    let slack = 2048.0 * eps * PI * r * (r + (1.0 + r * r).sqrt());
    for pose in poses(r) {
        let what = format!("α = {deg}°, {}", pose.label);
        let plane = topo::test_support::split_plane(pose.foot, pose.normal, tol);
        let measured = if pose.near_tangent {
            near_tangent(deg, pose.theta, eps)
        } else {
            Some(None)
        };
        let result = match (split(&body, &plane, tol), measured) {
            (Err(e), Some(Some(name))) => {
                assert_eq!(refusal_name(&e), Some(name), "{what}: refused {e:?}");
                continue;
            }
            (Err(e), None) => {
                assert!(refusal_name(&e).is_some(), "{what}: refused {e:?}");
                continue;
            }
            (Err(e), Some(None)) => panic!("{what}: refused {e:?}"),
            (Ok(_), Some(Some(name))) => panic!("{what}: landed, where {name} was measured"),
            (Ok(result), _) => result,
        };
        let forms = closed_forms(r, &pose);
        for ((side, part), (want, depth)) in [("above", &result.above), ("below", &result.below)]
            .into_iter()
            .zip(forms)
        {
            let what = format!("{what}, {side}");
            let Some(b) = part.body() else {
                assert!(
                    depth <= 10.0 * eps,
                    "{what}: empty where the closed form is {want} reaching {depth} in"
                );
                continue;
            };
            let got = topo::test_support::pinned_volume(&what, b, tol).volume;
            // A sliver within the band lands with the other side whole.
            let other = whole - want;
            let landed_whole = (got - whole).abs() <= slack && want.min(other) <= slack;
            assert!(
                (got - want).abs() <= slack || landed_whole,
                "{what}: volume {got}, closed form {want}"
            );
            assert_eq!(
                topo::validate_pseudomanifold(b, &ContactRecords::default(), tol),
                Ok(()),
                "{what}: tier 3′"
            );
        }
    }
}

/// **A plane grazing a conical pit along a ruling through its apex
/// refuses the knife edge.** A unit cylinder with a 30° pit sunk from
/// its top (its rulings leave the apex up the axis) or from its bottom
/// (down it, the other nappe): the pit's wall bends away from the
/// material round it, so the plane tangent to it along a ruling leaves
/// material on both sides of the contact, which the split cannot
/// declare. It refuses at the apex in the reduction, naming the pit's
/// wall, at every azimuth of `THETAS` with either normal.
#[test]
fn a_plane_grazing_a_conical_pit_through_its_apex_refuses_the_knife_edge() {
    let tol = Tol::witness();
    let (rim, depth) = (0.5, 0.5 / 30f64.to_radians().tan());
    for (pit, profile, apex_y, rim_y) in [
        (
            "from the top",
            vec![
                (0.0, 0.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (rim, 1.0),
                (0.0, 1.0 - depth),
            ],
            1.0 - depth,
            1.0,
        ),
        (
            "from the bottom",
            vec![(0.0, depth), (rim, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            depth,
            0.0,
        ),
    ] {
        let lp = bulge_loop(
            profile
                .iter()
                .map(|&(x, y)| (Point2::new(x, y), 0.0))
                .collect(),
        );
        let vp = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol)
            .unwrap();
        let body = sweep::revolve(
            &vp,
            sweep::RevolveAxis {
                origin: Point2::new(0.0, 0.0),
                dir: Vec2::new(0.0, 1.0),
            },
            sweep::Revolution::Full,
            tol,
        )
        .unwrap()
        .body;
        let body = sweep::test_support::finished("the pitted cylinder", body, tol);
        let apex = Point3::new(0.0, apex_y, 0.0);
        for a in THETAS {
            let radial = Vec3::new(a.cos(), 0.0, a.sin());
            let ruling = (Point3::new(0.0, rim_y, 0.0) + radial * rim) - apex;
            let round = Vec3::new(-a.sin(), 0.0, a.cos());
            for s in [1.0, -1.0] {
                let what = format!("a pit {pit}, θ = {a}, normal {s}·n");
                let plane =
                    topo::test_support::split_plane(apex, ruling.cross(round).normalize() * s, tol);
                let e = match split(&body, &plane, tol) {
                    Ok(r) => panic!(
                        "{what}: answered ({:?}, {:?})",
                        r.above.body().is_some(),
                        r.below.body().is_some()
                    ),
                    Err(e) => e,
                };
                let knife = e
                    .knife_edge()
                    .unwrap_or_else(|| panic!("{what}: refused {e:?}, not the knife edge"));
                assert!(
                    matches!(e, SplitError::Reduce(_)),
                    "{what}: read in the reduction: {e:?}"
                );
                let wall = body
                    .get_face(knife.wall)
                    .and_then(|f| body.get_surface(f.surface))
                    .map(geom::Surface::kind);
                assert_eq!(
                    wall,
                    Some(geom::SurfaceKind::Cone),
                    "{what}: names the pit's wall"
                );
            }
        }
    }
}

/// The 30° cone: the two seam meridians meet at 60°, under a half-turn.
#[test]
fn a_plane_through_a_30_degree_cones_apex_cuts_its_closed_forms() {
    every_pose_answers(30.0);
}

/// The 45° cone: the two seam meridians meet at exactly a right angle.
#[test]
fn a_plane_through_a_45_degree_cones_apex_cuts_its_closed_forms() {
    every_pose_answers(45.0);
}

/// The 60° cone: the two seam meridians meet at 120°.
#[test]
fn a_plane_through_a_60_degree_cones_apex_cuts_its_closed_forms() {
    every_pose_answers(60.0);
}
