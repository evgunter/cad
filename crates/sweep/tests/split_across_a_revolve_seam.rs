//! **A plane across a full revolve's single seam cuts each wall it
//! crosses in a whole conic.** A profile off the axis revolves a full
//! turn into walls of one face each, closed on one seam meridian. A
//! plane across the axis crosses that seam once per wall, so each
//! wall's section is a one-corner loop: a self-loop chord on the whole
//! circle or ellipse. The split builds both halves of a tube, a
//! counterbored tube and a cone socket at their closed-form volumes,
//! each section an annulus (an outline and the bore's ring), and
//! `plane_section` reports the same annulus.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::bores::{halves_at_rest, section_faces};
use core::f64::consts::PI;
use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use topo::splitting::{SectionEdge, SplitPlane, plane_section};
use topo::{Body, mass_properties};

fn tol() -> Tol {
    Tol::witness()
}

/// `pts` (straight edges) revolved a full turn about the sketch's `v`
/// axis: world `y` on the xy sketch, world `z` on the yz sketch.
fn revolved(sketch: SketchPlane<f64>, pts: &[(f64, f64)]) -> Body<f64> {
    let lp = bulge_loop(pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect());
    let profile = Profile::new(sketch, vec![lp]).validate(tol()).unwrap();
    let axis = sweep::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    sweep::revolve(&profile, axis, sweep::Revolution::Full, tol())
        .unwrap()
        .body
}

const TUBE: [(f64, f64); 4] = [(0.5, 0.0), (1.0, 0.0), (1.0, 1.0), (0.5, 1.0)];
/// A bore of r = 0.3 under a counterbore of r = 0.6 from y = 0.6.
const COUNTERBORE: [(f64, f64); 6] = [
    (0.3, 0.0),
    (1.0, 0.0),
    (1.0, 1.0),
    (0.6, 1.0),
    (0.6, 0.6),
    (0.3, 0.6),
];
/// A conical bore widening from r = 0.3 at y = 0 to r = 0.6 at y = 1.
const SOCKET: [(f64, f64); 4] = [(0.3, 0.0), (1.0, 0.0), (1.0, 1.0), (0.6, 1.0)];

/// The plane through `axis · c` whose normal leans `tilt` off `axis`
/// toward azimuth `az` of `axis`'s own basis, negated when `s < 0`.
fn across(axis: Vec3<f64>, c: f64, tilt: f64, az: f64, s: f64) -> SplitPlane<f64> {
    let (e1, e2) = axis.orthonormal_basis();
    let n = (axis * tilt.cos() + (e1 * az.cos() + e2 * az.sin()) * tilt.sin()) * s;
    topo::test_support::split_plane(Point3::origin() + axis * c, n, tol())
}

/// Splits `body` by `plane`, asserting tiers 1, 3 and 3′ on both halves
/// and one section face per half whose one ring is the bore's section;
/// returns the `[below, above]` volumes.
fn halves(label: &str, body: &Body<f64>, plane: &SplitPlane<f64>) -> [f64; 2] {
    let halves: [Body<f64>; 2] = halves_at_rest(label, body, plane);
    halves.map(|h| {
        assert_eq!(
            topo::validate_pseudomanifold(&h, &Default::default(), tol()),
            Ok(()),
            "{label}: tier 3′"
        );
        let rings: Vec<usize> = section_faces(&h, plane)
            .into_iter()
            .map(|f| h.get_face(f).unwrap().rings.len())
            .collect();
        assert_eq!(rings, [1], "{label}: one annular section face");
        mass_properties(&h, tol()).unwrap().volume
    })
}

fn near(label: &str, got: f64, want: f64, by: f64) {
    assert!((got - want).abs() <= by, "{label}: {got}, want {want}");
}

/// **A tube cut across its axis splits into two annular halves.** The
/// tube about y, cut through its centre `(0, 0.5, 0)` square, mirrored
/// and leaning at azimuths off the seam: the point reflection through
/// the centre maps the tube and each plane to themselves and swaps the
/// sides, so each half is `3π/8`. The tube about z is cut leaning.
#[test]
fn a_tube_cut_across_its_axis_splits_into_two_annular_halves() {
    let half = 0.375 * PI;
    let y = Vec3::new(0.0, 1.0, 0.0);
    let tube = revolved(SketchPlane::xy(), &TUBE);
    let poses = [
        (0.0, 0.0, 1.0),
        (0.0, 0.0, -1.0),
        (0.1f64.atan(), PI, 1.0),
        (0.25, 2.0, -1.0),
    ];
    for (tilt, az, s) in poses {
        let label = format!("about y, tilt {tilt}, azimuth {az}, s = {s}");
        for v in halves(&label, &tube, &across(y, 0.5, tilt, az, s)) {
            near(&label, v, half, 1e-8);
        }
    }
    let z = Vec3::new(0.0, 0.0, 1.0);
    let tube = revolved(SketchPlane::yz(), &TUBE);
    let label = "about z, tilt 0.25, azimuth 0.7, s = -1";
    for v in halves(label, &tube, &across(z, 0.5, 0.25, 0.7, -1.0)) {
        near(label, v, half, 1e-8);
    }
}

/// **A counterbored tube and a cone socket split across their axes.**
/// Square cuts at closed-form volumes; a leaning cut through the
/// counterbore's narrow bore holds the same volumes, since the plane's
/// lean integrates to nothing over the axisymmetric annulus it crosses
/// between two cylinders. The counterbore also splits flush with its
/// step.
#[test]
fn a_counterbore_and_a_cone_socket_split_across_their_axes() {
    let y = Vec3::new(0.0, 1.0, 0.0);
    let cb = revolved(SketchPlane::xy(), &COUNTERBORE);
    let (bore, wide) = (PI * (1.0 - 0.09), PI * (1.0 - 0.36));
    let whole = bore * 0.6 + wide * 0.4;
    for (c, below, tilt) in [
        (0.3, bore * 0.3, 0.0),
        (0.3, bore * 0.3, 0.1),
        (0.8, bore * 0.6 + wide * 0.2, 0.0),
        (0.6, bore * 0.6, 0.0),
    ] {
        for s in [1.0, -1.0] {
            let label = format!("counterbore at y = {c}, tilt {tilt}, s = {s}");
            let p = across(y, c, tilt, 2.0, s);
            let [b, a] = if c == 0.6 {
                // The step face lies in the plane beside the section.
                halves_at_rest(&label, &cb, &p).map(|h| mass_properties(&h, tol()).unwrap().volume)
            } else {
                halves(&label, &cb, &p)
            };
            let (b, a) = if s > 0.0 { (b, a) } else { (a, b) };
            near(&label, b, below, 1e-8);
            near(&label, a, whole - below, 1e-8);
        }
    }
    let socket = revolved(SketchPlane::xy(), &SOCKET);
    let frustum = |h: f64| {
        let r = 0.3 + 0.3 * h;
        PI * h / 3.0 * (0.09 + 0.3 * r + r * r)
    };
    let (whole, below) = (PI - frustum(1.0), 0.5 * PI - frustum(0.5));
    for s in [1.0, -1.0] {
        let label = format!("socket, s = {s}");
        let [b, a] = halves(&label, &socket, &across(y, 0.5, 0.0, 0.0, s));
        let (b, a) = if s > 0.0 { (b, a) } else { (a, b) };
        near(&label, b, below, 1e-9);
        near(&label, a, whole - below, 1e-9);
    }
}

/// **`plane_section` of a revolved body across its axis is an
/// annulus**: one region whose outline and one hole are each a single
/// corner on a whole-turn arc, the area the closed form.
#[test]
fn plane_section_across_a_revolve_seam_is_an_annulus() {
    let y = Vec3::new(0.0, 1.0, 0.0);
    let z = Vec3::new(0.0, 0.0, 1.0);
    for (label, body, axis, c, area) in [
        (
            "tube about y",
            revolved(SketchPlane::xy(), &TUBE),
            y,
            0.5,
            0.75 * PI,
        ),
        (
            "tube about z",
            revolved(SketchPlane::yz(), &TUBE),
            z,
            0.5,
            0.75 * PI,
        ),
        (
            "counterbore",
            revolved(SketchPlane::xy(), &COUNTERBORE),
            y,
            0.8,
            0.64 * PI,
        ),
        (
            "socket",
            revolved(SketchPlane::xy(), &SOCKET),
            y,
            0.5,
            PI * (1.0 - 0.45 * 0.45),
        ),
    ] {
        for s in [1.0, -1.0] {
            let label = format!("{label}, s = {s}");
            let body = sweep::test_support::finished("the revolve", body.clone(), tol());
            let section = plane_section(&body, &across(axis, c, 0.0, 0.0, s), tol())
                .unwrap_or_else(|e| panic!("{label}: {e:?}"));
            let [region] = &section.regions[..] else {
                panic!("{label}: {} regions", section.regions.len());
            };
            assert_eq!(region.holes.len(), 1, "{label}: the bore is the hole");
            for polygon in std::iter::once(&region.outline).chain(&region.holes) {
                let [SectionEdge::Arc { start, end, .. }] = polygon.edges() else {
                    panic!("{label}: {:?}", polygon.edges());
                };
                near(&label, (end - start).abs(), 2.0 * PI, 1e-12);
            }
            near(&label, region.area(), area, 1e-9);
        }
    }
}

/// **A plane whose section touches a rim splits at the closed form.**
/// The tube about y, cut by planes leaning `t = 0.2` whose outer
/// ellipse touches the top rim (or the bottom one) at one point: at the
/// seam vertex itself (azimuth 0), where the whole conic's one corner
/// is also a vertex of the cap, and at two rim points between vertices.
/// The plane stays inside the band of the tube elsewhere, so the cap's
/// side holds `A·tan t` of the annulus `A = 3π/4`. On main every pose
/// refuses, `DegenerateSection` at the seam vertex and
/// `Finish(Corrupt)` at the others.
#[test]
fn a_section_touching_a_rim_splits_at_the_closed_form() {
    let t = 0.2f64;
    let (whole, cap_side) = (0.75 * PI, 0.75 * PI * t.tan());
    let tube = revolved(SketchPlane::xy(), &TUBE);
    let y = Vec3::new(0.0, 1.0, 0.0);
    for az in [0.0, 0.5 * PI, PI] {
        let d = Vec3::new(az.cos(), 0.0, az.sin());
        // Top rim: the cap's side is above; bottom rim: below.
        for (rim, lean) in [(1.0, 1.0), (0.0, -1.0)] {
            let n = y * t.cos() - d * (t.sin() * lean);
            for s in [1.0, -1.0] {
                let label = format!("rim {rim}, azimuth {az}, s = {s}");
                let plane =
                    topo::test_support::split_plane(Point3::new(d.x, rim, d.z), n * s, tol());
                let [below, above] = halves_at_rest(&label, &tube, &plane)
                    .map(|h| mass_properties(&h, tol()).unwrap().volume);
                let cap = if (rim == 1.0) == (s > 0.0) {
                    above
                } else {
                    below
                };
                near(&label, cap, cap_side, 1e-8);
                near(&label, below + above, whole, 1e-8);
            }
        }
    }
}
