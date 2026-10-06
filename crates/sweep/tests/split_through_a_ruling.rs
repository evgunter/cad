//! A plane through a full revolve's seam ruling splits the body as it
//! does through any other ruling: the seam edge lies on the section
//! boundary, and the section face that leaves the wall's chart takes
//! it with an image on the wall that keeps the chart, without the seam
//! claim one side can no longer make. Each pose answers with its
//! closed-form volumes and halves that pass tiers 1, 2, 3 and 3′.
//!
//! The walls are revolved about y, so the seam ruling is the one at
//! azimuth 0, `(r, y, 0)`. A plane through the ruling at azimuth `a`
//! with its normal turned `t` off the wall's outward normal there cuts
//! a vertical wall's cross-section along a line at signed distance
//! `s·r·cos t` from the axis. At `t = π/2` the plane holds the axis, so
//! it runs along every wall's seam ruling at once, a bore's included.
//!
//! A plane holding a frustum's ruling holds its apex, and a plane
//! parallel to a cylinder's axis runs along two of its rulings: either
//! way the wall's section is a pair of rulings, and the split pairs the
//! wall's crossings along each, on the seam or off it, about the y axis
//! or about one tilted off the coordinate axes.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use topo::splitting::{SplitPart, SplitPlane, split};
use topo::{AtRestBody, Body, ContactRecords, mass_properties};

/// Tilts off tangency, both senses and both sides of the axis.
const TILTS: [f64; 8] = [0.05, 0.4, 1.0, FRAC_PI_2, 2.0, 3.0, -0.4, -1.2];

fn revolved(points: &[(f64, f64)]) -> AtRestBody<f64> {
    revolved_about(points, Vec2::new(0.0, 1.0))
}

/// The profile `(ρ, s)` — `ρ` off the axis, `s` along it — revolved
/// fully about the axis through the origin along `dir` in the xy plane.
/// Its seam lies on the side of the axis at `dir × ẑ`.
fn revolved_about(points: &[(f64, f64)], dir: Vec2<f64>) -> AtRestBody<f64> {
    let d = dir / dir.norm();
    let lp = bulge_loop(
        points
            .iter()
            .map(|&(rho, s)| (Point2::new(d.x * s + d.y * rho, d.y * s - d.x * rho), 0.0))
            .collect(),
    );
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let body = sweep::revolve(
        &vp,
        sweep::RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: d,
        },
        sweep::Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    sweep::test_support::finished("operand", body, Tol::witness())
}

/// The axis `dir` in the xy plane as a 3-D unit vector, with the unit
/// radial direction at azimuth `phi` about it, measured from the seam's
/// side `dir × ẑ` towards `ẑ`.
fn axis_frame(dir: Vec2<f64>, phi: f64) -> (Vec3<f64>, Vec3<f64>) {
    let d = dir / dir.norm();
    let axis = Vec3::new(d.x, d.y, 0.0);
    let seam = Vec3::new(d.y, -d.x, 0.0);
    (axis, seam * phi.cos() + Vec3::new(0.0, 0.0, phi.sin()))
}

/// The area of the disc of radius `r` on the far side of a line at
/// signed distance `d` from its centre.
fn segment(r: f64, d: f64) -> f64 {
    if d >= r {
        0.0
    } else if d <= -r {
        PI * r * r
    } else {
        r * r * (d / r).acos() - d * (r * r - d * d).sqrt()
    }
}

/// A stack of annular slabs `(height, inner radius, outer radius)`: the
/// volume on the normal's side of a plane parallel to the axis at
/// signed distance `d` from it, and the whole.
fn slabs_above(slabs: &[(f64, f64, f64)], d: f64) -> (f64, f64) {
    let above = slabs
        .iter()
        .map(|&(h, r_in, r_out)| h * (segment(r_out, d) - segment(r_in, d)))
        .sum();
    let whole = slabs
        .iter()
        .map(|&(h, r_in, r_out)| h * PI * (r_out * r_out - r_in * r_in))
        .sum();
    (above, whole)
}

fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> SplitPlane<f64> {
    topo::test_support::split_plane(origin, normal / normal.norm(), Tol::witness())
}

/// One side holds the volume wanted and passes every tier.
fn holds(label: &str, side: &str, part: &SplitPart<f64>, want: f64) {
    let tol = Tol::witness();
    let b: &Body<f64> = part
        .body()
        .unwrap_or_else(|| panic!("{label}: no {side} side, want {want}"));
    topo::validate(b).unwrap_or_else(|e| panic!("{label}: {side}: tier 1: {e:?}"));
    topo::validate_closed(b).unwrap_or_else(|e| panic!("{label}: {side}: tier 2: {e:?}"));
    topo::validate_geometric(b, tol).unwrap_or_else(|e| panic!("{label}: {side}: tier 3: {e:?}"));
    topo::validate_pseudomanifold(b, &ContactRecords::default(), tol)
        .unwrap_or_else(|e| panic!("{label}: {side}: tier 3′: {e:?}"));
    let v = mass_properties(b, tol)
        .unwrap_or_else(|e| panic!("{label}: {side}: mass properties: {e:?}"))
        .volume;
    assert!(
        (v - want).abs() <= 1e-9 * want.max(1.0),
        "{label}: {side} volume {v}, want {want}"
    );
}

/// `body` split by `p` answers `(above, below)`.
fn answers(label: &str, body: &AtRestBody<f64>, p: &SplitPlane<f64>, want: (f64, f64)) {
    let r = split(body, p, Tol::witness()).unwrap_or_else(|e| panic!("{label}: {e:?}"));
    holds(label, "above", &r.above, want.0);
    holds(label, "below", &r.below, want.1);
}

/// Every tilt, both normals, of a vertical wall of radius `r` (height
/// spanning y = 0.5) cut through its seam ruling.
fn seam_rulings(name: &str, body: &AtRestBody<f64>, slabs: &[(f64, f64, f64)], r: f64) {
    for t in TILTS {
        for s in [1.0, -1.0] {
            let n = Vec3::new(s * t.cos(), 0.0, s * t.sin());
            let (above, whole) = slabs_above(slabs, s * r * t.cos());
            answers(
                &format!("{name}: r = {r}, t = {t}, s = {s}"),
                body,
                &plane(Point3::new(r, 0.5, 0.0), n),
                (above, whole - above),
            );
        }
    }
}

/// The pose the row was found on — a solid cylinder (r = 1, height 1)
/// cut through its seam ruling 0.4 rad off tangency — at every tilt.
#[test]
fn a_solid_cylinder_split_through_its_seam_ruling_answers() {
    let cylinder = revolved(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
    seam_rulings("solid cylinder", &cylinder, &[(1.0, 0.0, 1.0)], 1.0);
}

/// A tube's outer wall cut through its seam ruling at every tilt.
#[test]
fn a_tube_split_through_its_seam_ruling_answers() {
    let tube = revolved(&[(0.5, 0.0), (1.0, 0.0), (1.0, 1.0), (0.5, 1.0)]);
    seam_rulings("tube", &tube, &[(1.0, 0.5, 1.0)], 1.0);
}

/// A counterbored ring's outer wall cut through its seam ruling at
/// every tilt.
#[test]
fn a_counterbore_split_through_its_seam_ruling_answers() {
    let counterbore = revolved(&[
        (0.3, 0.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (0.6, 1.0),
        (0.6, 0.5),
        (0.3, 0.5),
    ]);
    seam_rulings(
        "counterbore",
        &counterbore,
        &[(0.5, 0.3, 1.0), (0.5, 0.6, 1.0)],
        1.0,
    );
}

/// The y axis, whose seam ruling is the one at azimuth 0.
const Y_AXIS: Vec2<f64> = Vec2::new(0.0, 1.0);

/// Axis directions in the sketch plane tilted off every coordinate
/// axis, so no section ruling runs along a coordinate of the plane's
/// in-plane frame.
const TILTED_AXES: [Vec2<f64>; 2] = [Vec2::new(1.0, 1.0), Vec2::new(1.0, 0.2)];

/// Azimuths of the ruling: the seam's and three off it.
const AZIMUTHS: [f64; 4] = [0.0, 0.3, 2.0, 4.0];

/// A frustum of radius `r0` at `s = 0` and `r1` at `s = 1` about each
/// of `axes`, cut through its ruling at each azimuth at every tilt, both
/// normals. The plane holds the cone's apex, so the wall's section is
/// two rulings, and each side is the cone over its part of the larger
/// disc less the similar cone over its part of the smaller.
fn frustum_rulings(name: &str, r0: f64, r1: f64, axes: &[Vec2<f64>]) {
    let apex = r0 / (r0 - r1);
    let (big, small, s_big) = if r0 > r1 {
        (r0, r1, 0.0)
    } else {
        (r1, r0, 1.0)
    };
    let k = (apex - s_big).abs() / 3.0 * (1.0 - (small / big).powi(3));
    for &dir in axes {
        let body = revolved_about(&[(0.0, 0.0), (r0, 0.0), (r1, 1.0), (0.0, 1.0)], dir);
        for a in AZIMUTHS {
            let (axis, radial) = axis_frame(dir, a);
            let ruling = (axis + radial * (r1 - r0)).normalize();
            let outward = (radial - axis * (r1 - r0)).normalize();
            for t in TILTS {
                for s in [1.0, -1.0] {
                    let n = (outward * t.cos() + ruling.cross(outward) * t.sin()) * s;
                    let across = n - axis * n.dot(axis);
                    let above = k * segment(big, big * n.dot(radial) / across.norm());
                    answers(
                        &format!("{name}: axis {dir:?}, a = {a}, t = {t}, s = {s}"),
                        &body,
                        &plane(Point3::new(0.0, 0.0, 0.0) + radial * r0, n),
                        (above, k * PI * big * big - above),
                    );
                }
            }
        }
    }
}

/// A frustum (radii 1 → 1/2 over height 1) cut through a wall ruling,
/// on its seam or off it, at every tilt: 7/12 of the base segment.
#[test]
fn a_frustum_split_through_a_ruling_answers() {
    frustum_rulings("frustum", 1.0, 0.5, &[Y_AXIS]);
}

/// The same frustum widening along its axis (radii 1/2 → 1, the apex
/// behind the narrow end) cut through a wall ruling at every tilt.
#[test]
fn a_flared_frustum_split_through_a_ruling_answers() {
    frustum_rulings("flared frustum", 0.5, 1.0, &[Y_AXIS]);
}

/// Both frusta about axes tilted off the coordinate axes.
#[test]
fn a_frustum_about_a_tilted_axis_split_through_a_ruling_answers() {
    frustum_rulings("frustum", 1.0, 0.5, &TILTED_AXES);
    frustum_rulings("flared frustum", 0.5, 1.0, &TILTED_AXES);
}

/// A solid cylinder (r = 1, length 1) about a tilted axis, cut by a
/// plane parallel to that axis at distance `c`: the wall's section is
/// two rulings, and the side above is a slab over the disc's segment.
#[test]
fn a_cylinder_cut_parallel_to_its_tilted_axis_answers() {
    for dir in TILTED_AXES {
        let cylinder = revolved_about(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], dir);
        for phi in AZIMUTHS {
            let (_, radial) = axis_frame(dir, phi);
            for c in [0.0, 0.5, 0.9, -0.7] {
                for s in [1.0, -1.0] {
                    let n = radial * s;
                    let above = segment(1.0, c * s);
                    answers(
                        &format!("tilted cylinder: axis {dir:?}, phi = {phi}, c = {c}, s = {s}"),
                        &cylinder,
                        &plane(Point3::new(0.0, 0.0, 0.0) + radial * c, n),
                        (above, PI - above),
                    );
                }
            }
        }
    }
}

/// The section of the frustum through a ruling is the trapezoid of the
/// two rulings and the two cap chords: 3/4 of the triangle the apex
/// makes with the base chord.
#[test]
fn a_frustum_section_through_a_ruling_is_its_trapezoid() {
    let frustum = revolved(&[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)]);
    let apex = Point3::new(0.0, 2.0, 0.0);
    for a in AZIMUTHS {
        let (axis, radial) = axis_frame(Y_AXIS, a);
        let ruling = (axis - radial * 0.5).normalize();
        let outward = (radial + axis * 0.5).normalize();
        for t in TILTS {
            for s in [1.0, -1.0] {
                let label = format!("frustum section: a = {a}, t = {t}, s = {s}");
                let n = (outward * t.cos() + ruling.cross(outward) * t.sin()) * s;
                let across = (n - axis * n.dot(axis)).normalize();
                let d = n.dot(radial) / (n - axis * n.dot(axis)).norm();
                let chord = axis.cross(across) * (1.0 - d * d).sqrt();
                let foot = Point3::new(0.0, 0.0, 0.0) + across * d;
                let want = 0.375 * ((foot + chord) - apex).cross((foot - chord) - apex).norm();
                let section = topo::splitting::plane_section(
                    &frustum,
                    &plane(Point3::new(0.0, 0.0, 0.0) + radial, n),
                    Tol::witness(),
                )
                .unwrap_or_else(|e| panic!("{label}: {e:?}"));
                assert_eq!(section.regions.len(), 1, "{label}: regions");
                let area = section.regions[0].area();
                assert!(
                    (area - want).abs() <= 1e-9,
                    "{label}: area {area}, want {want}"
                );
            }
        }
    }
}

/// The area of the unit disc's segment cut off by a chord subtending
/// `theta`, `(θ − sin θ)/2`, by its series where the difference
/// cancels.
fn thin_segment(theta: f64) -> f64 {
    if theta < 0.1 {
        let t2 = theta * theta;
        theta * t2 / 12.0 * (1.0 - t2 / 20.0 * (1.0 - t2 / 42.0 * (1.0 - t2 / 72.0)))
    } else {
        (theta - theta.sin()) / 2.0
    }
}

/// Near tangency the sliver falls inside the band and a cut may refuse;
/// on the seam ruling or off it, a pose that answers holds both sides
/// at their closed forms, the sliver included. Each side is held to
/// the float floor of a volume integrated over the whole operand's
/// scale, so the sliver is read wherever it is above that floor
/// (t ≥ 1e-4). Which poses answer depends on ε.
#[test]
fn a_near_tangent_cut_along_a_cylinder_ruling_never_answers_wrongly() {
    let tol = Tol::witness();
    let cylinder = revolved(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
    let floor = 64.0 * f64::EPSILON * PI;
    let mut answered = 0;
    for a in [0.0_f64, 0.3] {
        for t in [1e-5_f64, 1e-4, 1e-3] {
            for s in [1.0, -1.0] {
                let label = format!("near tangent: a = {a}, t = {t}, s = {s}");
                let n = Vec3::new(s * (a + t).cos(), 0.0, s * (a + t).sin());
                let p = plane(Point3::new(a.cos(), 0.5, a.sin()), n);
                let Ok(r) = split(&cylinder, &p, tol) else {
                    continue;
                };
                answered += 1;
                let sliver = thin_segment(2.0 * t);
                let (above, below) = if s > 0.0 {
                    (sliver, PI - sliver)
                } else {
                    (PI - sliver, sliver)
                };
                for (side, part, want) in [("above", &r.above, above), ("below", &r.below, below)] {
                    let b = part
                        .body()
                        .unwrap_or_else(|| panic!("{label}: {side} came back empty, want {want}"));
                    topo::validate(b).unwrap_or_else(|e| panic!("{label}: {side}: tier 1: {e:?}"));
                    topo::validate_closed(b)
                        .unwrap_or_else(|e| panic!("{label}: {side}: tier 2: {e:?}"));
                    topo::validate_geometric(b, tol)
                        .unwrap_or_else(|e| panic!("{label}: {side}: tier 3: {e:?}"));
                    topo::validate_pseudomanifold(b, &ContactRecords::default(), tol)
                        .unwrap_or_else(|e| panic!("{label}: {side}: tier 3′: {e:?}"));
                    let m = mass_properties(b, tol)
                        .unwrap_or_else(|e| panic!("{label}: {side}: mass properties: {e:?}"));
                    assert!(
                        (m.volume - want).abs() <= 1e-9 * want + m.volume_pad + floor,
                        "{label}: {side} volume {}, want {want}",
                        m.volume
                    );
                }
            }
        }
    }
    if answered == 0 {
        println!("SKIPPED at this ε: no near-tangent pose answers, so no volume is read");
    }
}

/// The frustum cut through a ruling near tangency, on its seam or off
/// it: a pose that answers holds both sides at their closed forms, the
/// sliver read as 7/12 of the base disc's thin segment, at the floor
/// the cylinder's near-tangent rows read. Which poses answer depends
/// on ε.
#[test]
fn a_near_tangent_cut_through_a_frustum_ruling_never_answers_wrongly() {
    let tol = Tol::witness();
    let frustum = revolved(&[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0), (0.0, 1.0)]);
    let k = 7.0 / 12.0;
    let floor = 64.0 * f64::EPSILON * PI;
    let mut answered = 0;
    for a in AZIMUTHS {
        let (axis, radial) = axis_frame(Y_AXIS, a);
        let ruling = (axis - radial * 0.5).normalize();
        let outward = (radial + axis * 0.5).normalize();
        for t in [1e-3_f64, 1e-4, 1e-5] {
            for s in [1.0, -1.0] {
                let label = format!("near tangent frustum: a = {a}, t = {t}, s = {s}");
                let n = (outward * t.cos() + ruling.cross(outward) * t.sin()) * s;
                let d = n.dot(radial) / (n - axis * n.dot(axis)).norm();
                let sliver = k * thin_segment(2.0 * d.abs().min(1.0).acos());
                let (above, below) = if d > 0.0 {
                    (sliver, k * PI - sliver)
                } else {
                    (k * PI - sliver, sliver)
                };
                let p = plane(Point3::new(0.0, 0.0, 0.0) + radial, n);
                let Ok(r) = split(&frustum, &p, tol) else {
                    continue;
                };
                answered += 1;
                for (side, part, want) in [("above", &r.above, above), ("below", &r.below, below)] {
                    let b = part
                        .body()
                        .unwrap_or_else(|| panic!("{label}: {side} came back empty, want {want}"));
                    topo::validate(b).unwrap_or_else(|e| panic!("{label}: {side}: tier 1: {e:?}"));
                    topo::validate_closed(b)
                        .unwrap_or_else(|e| panic!("{label}: {side}: tier 2: {e:?}"));
                    topo::validate_geometric(b, tol)
                        .unwrap_or_else(|e| panic!("{label}: {side}: tier 3: {e:?}"));
                    topo::validate_pseudomanifold(b, &ContactRecords::default(), tol)
                        .unwrap_or_else(|e| panic!("{label}: {side}: tier 3′: {e:?}"));
                    let m = mass_properties(b, tol)
                        .unwrap_or_else(|e| panic!("{label}: {side}: mass properties: {e:?}"));
                    assert!(
                        (m.volume - want).abs() <= 1e-9 * want + m.volume_pad + floor,
                        "{label}: {side} volume {}, want {want}",
                        m.volume
                    );
                }
            }
        }
    }
    if answered == 0 {
        println!("SKIPPED at this ε: no near-tangent pose answers, so no volume is read");
    }
}
