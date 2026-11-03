//! **A cylinder, sphere or torus against a NURBS wall, read whole on
//! the 1×2 trace** (`geom_brep::ImplicitSection`): every open branch
//! between the boundary pass's crossings, every closed loop, and the
//! accounting that they are all of them.
//!
//! Every wall here is a chart that is not a graph over its own
//! parameters: its `u` and `v` run along other world axes than the
//! obvious ones, one reversed, so a pcurve whose feet were swapped or
//! read off the wrong chart sends its image off the carrier, and the
//! rows read the image back against the carrier.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, NurbsSurface, Surface};
use geom_brep::ssi::BranchEnd;
use geom_brep::{ChartAxis, ImplicitSection, SectionReading, SsiDomain, SsiError, SsiOutcome};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

use crate::shared::tol::{band, eps};

/// A clamped single-span knot vector of degree `p` over `[0, 1]`.
fn span(p: usize) -> KnotVector {
    let mut k = vec![0.0; p + 1];
    k.extend(vec![1.0; p + 1]);
    KnotVector::clamped(k, p).unwrap()
}

/// The single-span wall of degrees `(p, q)` whose control point `(i, j)`
/// is `c(i, j)`, unit weights.
fn patch(p: usize, q: usize, c: impl Fn(usize, usize) -> Point3<f64>) -> NurbsSurface<f64> {
    let control = (0..=p)
        .flat_map(|i| (0..=q).map(move |j| (i, j)))
        .map(|(i, j)| c(i, j))
        .collect();
    NurbsSurface::new(span(p), span(q), control, vec![1.0; (p + 1) * (q + 1)]).unwrap()
}

/// **The strip**: `x = 4v − 2` along `v`, `y = ½ − u` along `u`
/// (reversed), and `z = ½ − 0.8v + 0.8v²`, so `z = 0.3 + 0.05x²`: a
/// gently bent strip `4` long in `x` and `1` wide in `y`.
fn strip() -> NurbsSurface<f64> {
    let (x, y, z) = ([-2.0, 0.0, 2.0], [0.5, -0.5], [0.5, 0.1, 0.5]);
    patch(1, 2, |i, j| Point3::new(x[j], y[i], z[j]))
}

/// **The bumped plate**: `x = 1 − 2v`, `y = 2u − 1` over `[−1, 1]²`, at
/// height `0.2`, its middle control point raised to `0.6`.
fn bumped_plate() -> NurbsSurface<f64> {
    patch(2, 2, |i, j| {
        let z = if i == 1 && j == 1 { 0.6 } else { 0.2 };
        Point3::new(1.0 - f64::from(j as u32), f64::from(i as u32) - 1.0, z)
    })
}

/// **The flat plate** `z = h` over `[−2, 2]²`, `x = 2 − 4v`, `y = 4u − 2`.
fn flat_plate(h: f64) -> NurbsSurface<f64> {
    patch(1, 1, |i, j| {
        Point3::new(2.0 - 4.0 * j as f64, 4.0 * i as f64 - 2.0, h)
    })
}

fn sphere(center: Point3<f64>, radius: f64) -> Surface<f64> {
    Surface::Sphere {
        center,
        radius,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

fn cylinder(radius: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
        radius,
    }
}

fn torus(major: f64, minor: f64) -> Surface<f64> {
    Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
        major_radius: major,
        minor_radius: minor,
    }
}

fn domain() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.0, 0.0, 0.3),
        half_extent: 3.0,
        extent: 1.0,
        floor_scale: 1.0,
    }
}

/// The section read whole.
fn whole(partner: &Surface<f64>, wall: &NurbsSurface<f64>) -> SsiOutcome {
    ImplicitSection::of(partner, wall, domain(), band())
        .expect("the section")
        .all()
        .expect("every branch")
}

/// **Every branch lies on both surfaces, and its wall pcurve is its
/// carrier's chart path.** Dense samples of each carrier lie within a
/// few ε of the partner (its residual) and of the wall (the wall's image
/// of the pcurve at the same parameter); the branch carries no pcurve on
/// the partner, which has no chart. Red against swapped feet (the image
/// lands across the strip from the carrier) and against a pcurve on the
/// partner's side.
fn on_both(partner: &Surface<f64>, wall: &NurbsSurface<f64>, out: &SsiOutcome, what: &str) {
    for (k, b) in out.branches.iter().enumerate() {
        assert!(
            b.pcurve_a.is_none(),
            "{what} branch {k}: a pcurve on the partner"
        );
        let pc = b.pcurve_b.as_ref().expect("the wall's pcurve");
        let (t0, t1) = b.params;
        let (mut off_partner, mut off_wall) = (0.0_f64, 0.0_f64);
        for i in 0..=200 {
            let t = t0 + (t1 - t0) * f64::from(i) / 200.0;
            let p = b.carrier.eval(t);
            off_partner = off_partner.max(geom_brep::implicit_residual(partner, p).abs());
            let uv = pc.eval(t);
            off_wall = off_wall.max(wall.eval(uv.x, uv.y).distance(p));
        }
        assert!(
            off_partner <= 4.0 * eps(),
            "{what} branch {k}: {off_partner:e} m off the partner"
        );
        assert!(
            off_wall <= 4.0 * eps(),
            "{what} branch {k}: its pcurve's image is {off_wall:e} m off its carrier"
        );
    }
    assert!(
        out.exhaustiveness.accounted > 0,
        "{what}: the accounting banked no tube: {}",
        out.exhaustiveness
    );
}

/// **A sphere cuts the strip in two open arcs, each from one long side
/// to the other.** The unit sphere about the origin meets the bent strip
/// where `x ≈ ±0.93`, each arc running across the strip's width, from
/// its `u = 0` side to its `u = 1` side: two branches, each a rim branch
/// whose ends are on those two sides, one on each side of `x = 0`. Red
/// against the march's crossing pairing scrambled (an arc joining two
/// crossings of one side, or both ends on one arc's side), and against
/// the rim branch's ends read on the wrong axis.
#[test]
fn a_sphere_cuts_the_strip_in_two_arcs_across_it() {
    let (s, w) = (sphere(Point3::new(0.0, 0.0, 0.0), 1.0), strip());
    let out = whole(&s, &w);
    assert_eq!(out.branches.len(), 2, "two arcs: {:?}", out.branches.len());
    assert!(out.boundary.is_empty(), "no boundary region");
    on_both(&s, &w, &out, "sphere × strip");
    let mut sides = Vec::new();
    for b in &out.branches {
        let BranchEnd::Rim { from, to } = b.end else {
            panic!("an open arc ends on the rim: {:?}", b.end);
        };
        for end in [from, to] {
            assert_eq!(end.side.fixed, ChartAxis::U, "an end on a u side: {end:?}");
        }
        assert_ne!(from.side.end, to.side.end, "the arc runs across the strip");
        sides.push(b.carrier.eval(b.params.0).x.signum());
    }
    assert!(
        sides[0] * sides[1] < 0.0,
        "one arc each side of x = 0: {sides:?}"
    );
}

/// **A vertical cylinder cuts the bumped plate in one interior loop.**
/// The cylinder of radius ½ about the `z` axis meets the plate over the
/// circle `x² + y² = ¼`, inside the plate's rectangle, its height rising
/// and falling with the bump: one closed branch, found from the seeds,
/// no crossing and no region. Red against the accounting excluding cells
/// it has not read clear, or skipping the tube check: the loop goes
/// unread and the accounting does not close.
#[test]
fn a_cylinder_cuts_a_bumped_plate_in_one_loop() {
    let (c, w) = (cylinder(0.5), bumped_plate());
    let out = whole(&c, &w);
    assert_eq!(out.branches.len(), 1, "one loop");
    assert!(
        matches!(out.branches[0].end, BranchEnd::Closed),
        "the loop is closed: {:?}",
        out.branches[0].end
    );
    on_both(&c, &w, &out, "cylinder × plate");
    let Curve3::Nurbs(n) = &out.branches[0].carrier else {
        panic!("a fitted carrier");
    };
    let rho = n
        .control()
        .iter()
        .map(|p| (p.x.hypot(p.y) - 0.5).abs())
        .fold(0.0, f64::max);
    assert!(rho < 0.05, "the loop rides the cylinder: {rho}");
}

/// **A torus cuts a flat plate in two loops.** The ring torus `(1,
/// 0.3)` about the `z` axis meets the plate `z = 0.1` in the circles of
/// radii `1 ± √0.08`: two closed branches, one about the other. Red
/// against limb 3's tube read without `‖∇f‖`, whose quartic form is far
/// from a unit here.
#[test]
fn a_torus_cuts_a_flat_plate_in_two_loops() {
    let (t, w) = (torus(1.0, 0.3), flat_plate(0.1));
    let out = whole(&t, &w);
    assert_eq!(out.branches.len(), 2, "two loops");
    let mut radii: Vec<f64> = out
        .branches
        .iter()
        .map(|b| {
            assert!(matches!(b.end, BranchEnd::Closed), "{:?}", b.end);
            let p = b.carrier.eval(b.params.0);
            p.x.hypot(p.y)
        })
        .collect();
    radii.sort_by(f64::total_cmp);
    let r = 0.08_f64.sqrt();
    assert!(
        (radii[0] - (1.0 - r)).abs() < 1e-6 && (radii[1] - (1.0 + r)).abs() < 1e-6,
        "the loops' radii {radii:?}"
    );
    on_both(&t, &w, &out, "torus × plate");
}

/// **`branch_at` answers the arc its seed settles on, and `all` keeps
/// it.** A seed beside the strip's `x > 0` arc returns a branch whose
/// carrier lies at `x > 0`; reading the rest whole gives the two arcs,
/// that one among them. Red against a seed settled on the wrong chart
/// coordinate (it leaves the strip and refuses).
#[test]
fn branch_at_answers_the_arc_its_seed_settles_on() {
    let (s, w) = (sphere(Point3::new(0.0, 0.0, 0.0), 1.0), strip());
    let mut section = ImplicitSection::of(&s, &w, domain(), band()).expect("the section");
    let SectionReading::Branch(b) = section
        .branch_at(Point3::new(0.95, 0.1, 0.35))
        .expect("the seed settles on an arc")
    else {
        panic!("a branch, not a region");
    };
    let at = b.carrier.eval(b.params.0);
    assert!(at.x > 0.5, "the arc at x > 0: {at:?}");
    let out = section.all().expect("every branch");
    assert_eq!(out.branches.len(), 2, "the two arcs");
}

/// **A sphere of radius 100 cuts the strip in two arcs near its ends.**
/// The sphere about `(0, 0, −99.5)`, its top `0.2` above the strip's
/// middle, meets the strip where its fall `≈ x²/200` meets the strip's
/// rise, at `x ≈ ±1.9`: two arcs across the strip, each on both
/// surfaces. The canonical form's divisor here is `2R = 200`, so every
/// reading that crosses the form to metres is read far from a unit: red
/// against limb 2 reading the sphere's composite with no divisor (its
/// hull over-reads the fit's deviation two hundred fold).
#[test]
fn a_large_sphere_cuts_the_strip_in_two_arcs() {
    let (s, w) = (sphere(Point3::new(0.0, 0.0, -99.5), 100.0), strip());
    let out = whole(&s, &w);
    assert_eq!(out.branches.len(), 2, "two arcs");
    on_both(&s, &w, &out, "large sphere × strip");
    for b in &out.branches {
        let x = b.carrier.eval(b.params.0).x.abs();
        assert!(
            (1.5..2.0).contains(&x),
            "an arc near an end of the strip: {x}"
        );
    }
}

/// **A sphere of radius 10⁴ cuts the strip in two arcs.** The sphere
/// about `(0, 0, 0.45 − 10⁴)`, its top `0.15` above the strip's middle
/// and nearly flat across the strip, meets it near `x = ±√3`: two arcs
/// across the strip, each on both surfaces. The canonical form's divisor
/// here is `2·10⁴`: red against limb 2 reading the sphere's composite
/// with no divisor (its hull over-reads the fit's deviation twenty
/// thousand fold, past the band).
///
/// Where `f64` at the sphere's own reach cannot settle a state well
/// inside ε (at ε = 1e-12, the spacing of floats near 10⁴ is 1.8e-12),
/// the section refuses at its door, the binding reach the partner's:
/// the march's tolerance reads the wall written about the sphere's
/// centre, not the wall alone. Red against the tolerance read on the
/// wall's coordinates alone (the march then refuses mid-trace, unable to
/// settle a step).
#[test]
fn a_very_large_sphere_cuts_the_strip_in_two_arcs() {
    let r = 1.0e4;
    let (s, w) = (sphere(Point3::new(0.0, 0.0, 0.45 - r), r), strip());
    match ImplicitSection::of(&s, &w, domain(), band()) {
        Ok(section) => {
            let out = section.all().expect("every branch");
            assert_eq!(out.branches.len(), 2, "two arcs");
            on_both(&s, &w, &out, "very large sphere × strip");
        }
        Err(SsiError::SettlingUnresolvable(why)) => {
            assert!(why.reach >= r, "the partner's reach binds: {why:?}");
            assert!(
                why.settle > 0.5 * why.tolerance,
                "refused only where it cannot settle inside ε: {why:?}"
            );
        }
        Err(e) => panic!("the section: {e:?}"),
    }
}
