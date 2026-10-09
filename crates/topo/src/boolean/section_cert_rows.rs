//! **The section certificate's verdict rows**: the classifier on
//! surfaces, the per-pair rule on constructed classifications, and the
//! scan over real faces where a row needs a face that no public door
//! builds (a seamless band, a lone-vertex ring). The op-level rows are
//! `crates/sweep/tests/germ_interior_oval.rs`.
//!
//! Every row names the certificate it pins and the mutant that turns it
//! red.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

use super::*;
use crate::boolean::ops;
use crate::test_support_fixtures::{CylFrame, brick, cyl_wall_sheet};
use crate::{
    Body, BooleanDeclarations, FaceKey, FacePairDeclaration, FaceSurface, MefSite, MevSite, Operand,
};
use core::f64::consts::{PI, TAU};
use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Band, Point3, Tol, Vec3};

pub(super) fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

pub(super) const LEVER: f64 = 5.0;

pub(super) fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

pub(super) fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

pub(super) fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
    let normal = normal.normalize();
    Surface::Plane {
        origin,
        normal,
        u_ref: normal.orthonormal_basis().0,
    }
}

pub(super) fn cylinder(origin: Point3<f64>, axis: Vec3<f64>, radius: f64) -> Surface<f64> {
    let axis = axis.normalize();
    Surface::Cylinder {
        origin,
        axis,
        radius,
        u_ref: axis.orthonormal_basis().0,
    }
}

pub(super) fn sphere(center: Point3<f64>, radius: f64) -> Surface<f64> {
    Surface::Sphere {
        center,
        radius,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

pub(super) fn torus(center: Point3<f64>, major: f64, minor: f64) -> Surface<f64> {
    Surface::Torus {
        center,
        axis: Vec3::unit_z(),
        major_radius: major,
        minor_radius: minor,
        u_ref: Vec3::unit_x(),
    }
}

/// The donut of the torus doors, about `z` here: `R = 2`, `r = 0.5`.
fn donut() -> Surface<f64> {
    torus(p(0.0, 0.0, 0.0), 2.0, 0.5)
}

/// The rows' reach: a ball of diameter [`LEVER`] about the origin,
/// where every row's torus and walls stand.
pub(super) fn classify(f: &Surface<f64>, g: &Surface<f64>) -> Section<f64> {
    super::classify(
        f,
        g,
        super::Reach {
            centre: p(0.0, 0.0, 0.0),
            radius: LEVER / 2.0,
        },
        band(),
    )
}

/// `(count, single, essential on F, essential on G, unbounded)`, per
/// component.
pub(super) type Shape = (usize, bool, Vec<bool>, Vec<bool>, Vec<bool>);

pub(super) fn shape(s: &Section<f64>) -> Shape {
    match s {
        Section::Components { parts, single } => (
            parts.len(),
            *single,
            parts.iter().map(|c| c.essential_f).collect(),
            parts.iter().map(|c| c.essential_g).collect(),
            parts.iter().map(|c| c.unbounded).collect(),
        ),
        other => panic!("not classified: {other:?}"),
    }
}

/// The lone component's witness.
pub(super) fn witness(s: &Section<f64>) -> Point3<f64> {
    match s {
        Section::Components {
            parts,
            single: true,
        } if parts.len() == 1 => parts[0]
            .witness
            .expect("a lone component carries a witness"),
        other => panic!("not a lone component: {other:?}"),
    }
}

/// The witness lies on both carriers.
pub(super) fn on_both(w: Point3<f64>, f: &Surface<f64>, g: &Surface<f64>) {
    for s in [f, g] {
        let r = geom_brep::implicit_residual(s, w);
        assert!(r.abs() < 1e-12, "witness {w:?} is {r} off {s:?}");
    }
}

pub(super) fn tangent(s: &Section<f64>) -> &'static str {
    match s {
        Section::Tangent(name) => name,
        other => panic!("not a tangency: {other:?}"),
    }
}

fn touch(s: &Section<f64>) -> Touch<f64> {
    match s {
        Section::Touch(t) => *t,
        other => panic!("not a touch: {other:?}"),
    }
}

// -------------------------------------------------------------------
// Torus × plane
// -------------------------------------------------------------------

/// **Two `(0,1)` ovals: a plane through the hole cuts the tube on both
/// sides** (the bar threading the hole). Essential on the torus, so W2
/// carries it; there is no witness, so without W2 it refuses.
#[test]
fn a_plane_through_the_hole_cuts_two_essential_ovals() {
    let pl = plane(p(0.3, 0.0, 0.0), v(1.0, 0.0, 0.0));
    let s = classify(&donut(), &pl);
    assert_eq!(
        shape(&s),
        (
            2,
            false,
            vec![true, true],
            vec![false, false],
            vec![false, false]
        )
    );
    // The mirror order: the flags swap with the faces.
    let s = classify(&pl, &donut());
    assert_eq!(
        (shape(&s).2, shape(&s).3),
        (vec![false, false], vec![true, true])
    );
}

/// **Two `(1,0)` curves: a plane cutting both tube circles** — the
/// pin's `y = ±0.3` faces against the half donut (axis `y` there, `z`
/// here: the plane `z = 0.3`).
#[test]
fn a_plane_across_the_tube_cuts_two_parallels() {
    let s = classify(&donut(), &plane(p(0.0, 0.0, 0.3), v(0.0, 0.0, 1.0)));
    assert_eq!((shape(&s).0, shape(&s).2), (2, vec![true, true]));
}

/// **The scrape oval, and its side.** `x = 2.45` scrapes the near tube
/// circle and `x = −2.45` the far one: a single null oval whose witness
/// lies on BOTH carriers, on the plane's own side. A classifier that
/// swaps `σ` puts the witness on the other tube circle, which the plane
/// misses: red.
#[test]
fn a_scrape_oval_is_one_null_component_witnessed_on_its_own_side() {
    for x in [2.45, -2.45, 2.2, -1.7] {
        let pl = plane(p(x, 0.0, 0.0), v(1.0, 0.0, 0.0));
        let s = classify(&donut(), &pl);
        assert_eq!(
            shape(&s),
            (1, true, vec![false], vec![false], vec![false]),
            "x = {x}"
        );
        let w = witness(&s);
        on_both(w, &donut(), &pl);
        assert!(
            w.x * x > 0.0,
            "x = {x}: the witness sits on the plane's side"
        );
    }
    // A tilted plane scraping the outer wall off the midplane.
    let pl = plane(p(2.4, 0.0, 0.2), v(1.0, 0.3, 0.4));
    on_both(witness(&classify(&donut(), &pl)), &donut(), &pl);
}

/// **A pinch refuses (R-tan)**: a plane on the inner equator, a saddle
/// point, where the section is a figure eight; and a plane on the top
/// circle, tangent along it. Read as `Positive`, the first would be a
/// scrape and the second two parallels; read as a touch, either would
/// clear on one point of a section that is not a small loop about it.
#[test]
fn a_plane_tangent_to_the_tube_off_its_outer_half_refuses_as_a_tangency() {
    let s = classify(&donut(), &plane(p(1.5, 0.0, 0.0), v(1.0, 0.0, 0.0)));
    assert_eq!(tangent(&s), "section_torus_plane_near_tube");
    let s = classify(&donut(), &plane(p(-1.5, 0.0, 0.0), v(1.0, 0.0, 0.0)));
    assert_eq!(tangent(&s), "section_torus_plane_far_tube");
    let s = classify(&donut(), &plane(p(0.0, 0.0, 0.5), v(0.0, 0.0, 1.0)));
    assert!(tangent(&s).starts_with("section_torus_plane_"));
}

/// A length strictly inside the run's band sliver — past `zero`, short
/// of `escalate` — where a margin decides neither way, at whatever
/// eps the run carries.
pub(super) fn in_sliver() -> f64 {
    let b = band();
    0.5 * (b.zero() + b.escalate())
}

/// **A near-axial plane** at the tube's top: tangent or decided, never
/// a wrong class. Its tilt `s` is set so the near-tube margin `s·R` at
/// `h = r` sits in the band's sliver, so the top circle's pinch refuses
/// R-tan at every eps; just inside and just outside, both tube circles
/// are cut or both missed.
#[test]
fn a_near_axial_plane_is_decided_or_tangent_never_wrong() {
    let n = v(in_sliver() / 2.0, 0.0, 1.0);
    for (h, want) in [(0.5, None), (0.4, Some(2)), (0.6, Some(0))] {
        let s = classify(&donut(), &plane(p(0.0, 0.0, h), n));
        match (want, &s) {
            (None, Section::Tangent(_)) => {}
            (Some(k), Section::Components { parts, .. }) => {
                assert_eq!(parts.len(), k, "h = {h}");
            }
            _ => panic!("h = {h}: {s:?}"),
        }
    }
}

// -------------------------------------------------------------------
// Torus × sphere
// -------------------------------------------------------------------

#[test]
fn torus_and_ball_every_class() {
    // A ball on the axis cutting the tube's top: two parallels, W2.
    let s = classify(&donut(), &sphere(p(0.0, 0.0, 1.0), 2.2));
    assert_eq!(shape(&s).2, vec![true, true]);
    // A ball swallowing an arc of the donut: two `(0,1)` loops, W2.
    let s = classify(&donut(), &sphere(p(2.0, 0.0, 0.0), 1.0));
    assert_eq!(shape(&s).2, vec![true, true]);
    // A ball scraping the outer wall: one null loop, on both carriers.
    let ball = sphere(p(2.9, 0.3, 0.1), 0.6);
    on_both(witness(&classify(&donut(), &ball)), &donut(), &ball);
    // The same scrape from the hole's side, operands swapped.
    let ball = sphere(p(1.2, 0.0, 0.0), 0.4);
    on_both(witness(&classify(&ball, &donut())), &donut(), &ball);
    // A ball clear of the tube, and one inside it.
    assert_eq!(
        shape(&classify(&donut(), &sphere(p(4.0, 0.0, 0.0), 0.5))).0,
        0
    );
    assert_eq!(
        shape(&classify(&donut(), &sphere(p(2.0, 0.0, 0.0), 0.2))).0,
        0
    );
    // A ball in the hole tangent to the inner equator, a saddle point:
    // R-tan.
    let s = classify(&donut(), &sphere(p(1.0, 0.0, 0.0), 0.5));
    assert_eq!(tangent(&s), "section_torus_sphere_near_tube");
}

// -------------------------------------------------------------------
// Torus × coaxial and parallel-axis partners
// -------------------------------------------------------------------

#[test]
fn coaxial_partners_meet_the_torus_in_parallels() {
    let s = classify(&donut(), &cylinder(p(0.0, 0.0, -3.0), Vec3::unit_z(), 2.2));
    assert_eq!(
        shape(&s),
        (
            2,
            false,
            vec![true, true],
            vec![true, true],
            vec![false, false]
        )
    );
    let s = classify(&donut(), &torus(p(0.0, 0.0, 0.3), 2.3, 0.5));
    assert_eq!((shape(&s).0, shape(&s).3), (2, vec![true, true]));
    // A coaxial wall on the tube's outer equator: tangent.
    let s = classify(&donut(), &cylinder(p(0.0, 0.0, -3.0), Vec3::unit_z(), 2.5));
    assert_eq!(tangent(&s), "section_torus_coaxial_wall");
}

#[test]
fn a_parallel_axis_cylinder_every_class() {
    // A pin through the tube: two loops encircling the pin, W2 on it.
    let s = classify(&donut(), &cylinder(p(2.0, 0.0, -1.0), Vec3::unit_z(), 0.2));
    assert_eq!(
        shape(&s),
        (
            2,
            false,
            vec![false, false],
            vec![true, true],
            vec![false, false]
        )
    );
    // A pin straddling the outer equator: the null loop at θ = π.
    let pin = cylinder(p(2.5, 0.0, -1.0), Vec3::unit_z(), 0.2);
    let w = witness(&classify(&donut(), &pin));
    on_both(w, &donut(), &pin);
    assert!((w.x - 2.3).abs() < 1e-12, "{w:?}");
    // A pin straddling the inner equator: the null loop at θ = 0.
    let pin = cylinder(p(1.5, 0.1, -1.0), Vec3::unit_z(), 0.2);
    on_both(witness(&classify(&pin, &donut())), &donut(), &pin);
    // A fat offset cylinder crossing the tube twice: two `(0,1)` loops.
    let s = classify(&donut(), &cylinder(p(2.0, 0.0, -1.0), Vec3::unit_z(), 1.0));
    assert_eq!(shape(&s).2, vec![true, true]);
}

/// **R-reach**: an oblique cylinder (the backstop's tilted rod,
/// `union-backstop-catches-a-suspect-body-from-a-tilted-rod-in-a-half-donut`),
/// a non-coaxial torus; a cone against an oblique cylinder, a tilted
/// cone, a parallel-axis cone, a non-coaxial torus and a spline.
#[test]
fn intractable_poses_refuse_on_reach() {
    let rod = cylinder(
        p(1.8, 0.0, 0.0),
        v(-(0.5f64).sin(), 0.0, -(0.5f64).cos()),
        0.15,
    );
    assert!(matches!(classify(&donut(), &rod), Section::Intractable));
    assert!(matches!(
        classify(&donut(), &torus(p(1.0, 0.0, 0.0), 2.0, 0.5)),
        Section::Intractable
    ));
    let cone = Surface::Cone {
        apex: p(0.0, 0.0, 3.0),
        axis: Vec3::unit_z(),
        half_angle: 0.4,
        u_ref: Vec3::unit_x(),
    };
    let tilted = Surface::Cone {
        apex: p(0.5, 0.0, -1.0),
        axis: v(0.3, 0.0, 1.0).normalize(),
        half_angle: 0.3,
        u_ref: Vec3::unit_y(),
    };
    let beside = Surface::Cone {
        apex: p(0.5, 0.0, -1.0),
        axis: Vec3::unit_z(),
        half_angle: 0.4,
        u_ref: Vec3::unit_x(),
    };
    for (what, partner) in [
        ("an oblique cylinder", rod),
        ("a tilted cone", tilted),
        ("a parallel-axis cone", beside),
        ("a non-coaxial torus", torus(p(1.0, 0.0, 0.0), 2.0, 0.5)),
        ("a spline", bump()),
    ] {
        assert!(
            matches!(classify(&cone, &partner), Section::Intractable),
            "the cone and {what}"
        );
        assert!(
            matches!(classify(&partner, &cone), Section::Intractable),
            "{what} and the cone"
        );
    }
}

/// A bicubic bump over `[−2, 2]²`: boundary rows in `z = 0`, the inner
/// four control points at `z = 2`.
fn bump() -> Surface<f64> {
    use geom_core::spline::KnotVector;
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let mut control = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            let inner = (1..3).contains(&i) && (1..3).contains(&j);
            control.push(p(
                -2.0 + 4.0 * f64::from(i) / 3.0,
                -2.0 + 4.0 * f64::from(j) / 3.0,
                if inner { 2.0 } else { 0.0 },
            ));
        }
    }
    Surface::Nurbs(std::sync::Arc::new(
        geom::NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 16]).unwrap(),
    ))
}

/// **NURBS × plane: W0 on the control net, else R-reach**, in both
/// orders. A plane clear of every control point, on either side and
/// either orientation, is apart; a plane that cuts the net, one that
/// touches its top row, and a placeholder's poison net have no arm. Red
/// against W0 reading any one point clear rather than all (the cut and
/// touching planes), and against W0 dropped (the clear ones).
#[test]
fn a_nurbs_patch_and_a_plane_are_apart_by_the_net_or_refuse_on_reach() {
    let placeholder = Surface::Nurbs(std::sync::Arc::new(geom::NurbsSurface::placeholder()));
    let apart =
        |s: &Section<f64>| matches!(s, Section::Components { parts, .. } if parts.is_empty());
    for (what, pl, want_apart) in [
        ("above", plane(p(0.0, 0.0, 2.5), v(0.0, 0.0, 1.0)), true),
        (
            "above, flipped",
            plane(p(0.0, 0.0, 2.5), v(0.0, 0.0, -1.0)),
            true,
        ),
        ("below", plane(p(0.0, 0.0, -0.1), v(0.0, 0.0, 1.0)), true),
        ("oblique", plane(p(0.0, 0.0, 2.4), v(-0.3, 0.0, 1.0)), true),
        ("cutting", plane(p(0.0, 0.0, 0.8), v(0.0, 0.0, 1.0)), false),
        ("touching", plane(p(0.0, 0.0, 2.0), v(0.0, 0.0, 1.0)), false),
    ] {
        for s in [classify(&bump(), &pl), classify(&pl, &bump())] {
            if want_apart {
                assert!(apart(&s), "{what}: {s:?}");
            } else {
                assert!(matches!(s, Section::Intractable), "{what}: {s:?}");
            }
        }
    }
    let pl = plane(p(0.0, 0.0, 2.5), v(0.0, 0.0, 1.0));
    assert!(matches!(classify(&placeholder, &pl), Section::Intractable));
    assert!(matches!(classify(&bump(), &bump()), Section::Intractable));
}

// -------------------------------------------------------------------
// Cylinder × cylinder and × plane
// -------------------------------------------------------------------

/// **The P0 saddle loop**: `r₁ = 1` about `z`, `r₂ = 0.5` about `x` at
/// `y = 1.3`. One null loop, witnessed at `(±0.6, 0.8, 0)`. Reading the
/// middle row as the first (two thin-essential loops) would let W2
/// clear it on the arc face.
#[test]
fn the_p0_saddle_loop_is_one_null_component() {
    let a = cylinder(p(0.0, 0.0, 0.0), Vec3::unit_z(), 1.0);
    let b = cylinder(p(-3.0, 1.3, 0.0), Vec3::unit_x(), 0.5);
    let s = classify(&a, &b);
    assert_eq!(shape(&s), (1, true, vec![false], vec![false], vec![false]));
    let w = witness(&s);
    on_both(w, &a, &b);
    assert!((w.y - 0.8).abs() < 1e-12 && w.z.abs() < 1e-12, "{w:?}");
    // Tilted 0.15 rad about y: the same class, the angle never read.
    let b = cylinder(
        p(0.0, 1.3, 0.0),
        v(0.15f64.cos(), 0.0, -(0.15f64).sin()),
        0.5,
    );
    on_both(witness(&classify(&a, &b)), &a, &b);
}

/// **The saddle witness's side is decided, per sign of `δ₀`.** Cylinder
/// 2 crosses cylinder 1's axis at `y = ±1.3`, running along `+x` or
/// `−x`; `m = d₁ × d₂` turns with the run, so the four poses give `δ₀`
/// of each sign on each side of the axis, taken with either wall the
/// thinner. The witness is on the arc's middle ruling on cylinder 1's
/// side of cylinder 2: on both carriers, at `y` of the offset's sign,
/// `|y| = |offset| − r₂`. The far ruling never meets cylinder 1
/// (`|δ₀| + r₂ > r₁` wherever the loop is a saddle), so a side chosen
/// against `δ₀`'s sign reds every pose.
#[test]
fn the_saddle_witness_lies_on_cylinder_1s_side_for_either_sign() {
    for (r1, r2) in [(1.0, 0.5), (0.5, 1.0)] {
        let a = cylinder(p(0.0, 0.0, 0.0), Vec3::unit_z(), r1);
        for oy in [1.3, -1.3] {
            for run in [1.0, -1.0] {
                let (o2, d2) = (p(-3.0 * run, oy, 0.0), v(run, 0.0, 0.0));
                let b = cylinder(o2, d2, r2);
                let delta = (o2 - p(0.0, 0.0, 0.0)).dot(Vec3::unit_z().cross(d2));
                let pose = format!("r₁ = {r1}, r₂ = {r2}, offset y = {oy}, δ₀ = {delta}");
                let w = witness(&classify(&a, &b));
                on_both(w, &a, &b);
                let near = oy.signum() * (oy.abs() - r2);
                assert!(
                    (w.y - near).abs() < 1e-12 && w.z.abs() < 1e-12,
                    "{pose}: the witness {w:?} is not on the near ruling y = {near}"
                );
            }
        }
    }
}

#[test]
fn cylinder_pairs_every_class() {
    let a = cylinder(p(0.0, 0.0, 0.0), Vec3::unit_z(), 1.0);
    let rod = cylinder(p(0.0, 0.3, 0.0), Vec3::unit_x(), 0.2);
    // A thin rod through the fat wall: two loops encircling the rod.
    let s = classify(&a, &rod);
    assert_eq!(
        shape(&s),
        (
            2,
            false,
            vec![false, false],
            vec![true, true],
            vec![false, false]
        )
    );
    assert_eq!(shape(&classify(&rod, &a)).2, vec![true, true]);
    // Parallel walls: rulings.
    let s = classify(&a, &cylinder(p(1.2, 0.0, 0.0), Vec3::unit_z(), 0.5));
    assert_eq!(shape(&s).4, vec![true]);
    // Coaxial walls of different radii: parallel, nothing shared.
    let s = classify(&a, &cylinder(p(0.0, 0.0, 3.0), Vec3::unit_z(), 0.5));
    assert_eq!(shape(&s).4, vec![true]);
    // ONE carrier (same axis, same radius): the section is the surface
    // itself, not a curve, so it refuses as a tangency rather than
    // clearing as rulings.
    let s = classify(&a, &cylinder(p(0.0, 0.0, 3.0), v(0.0, 0.0, -1.0), 1.0));
    assert_eq!(tangent(&s), "section_cylinder_pair_coincident");
    // Apart.
    let s = classify(&a, &cylinder(p(0.0, 2.0, 0.0), Vec3::unit_x(), 0.5));
    assert_eq!(shape(&s).0, 0);
    // The tangent pose |δ₀| = r₁ + r₂: a touch at (0, 1, 0).
    let t = touch(&classify(
        &a,
        &cylinder(p(0.0, 1.5, 0.0), Vec3::unit_x(), 0.5),
    ));
    assert_eq!(t.name, "section_cylinder_pair_reach");
    assert!(
        (t.at - p(0.0, 1.0, 0.0)).norm() < 1e-12,
        "touch at {:?}",
        t.at
    );
}

/// **The `|d₁ × d₂|` decision**: two walls whose tilt, levered by the
/// pair's extent, sits in the band's sliver — neither parallel nor
/// definitely tilted, at whatever eps the run carries — offset so the
/// long saddle loop exists, refuse R-tan. A decision that let the
/// undecided tilt through would classify a loop the band cannot place.
#[test]
fn near_parallel_walls_refuse_as_a_tangency() {
    let a = cylinder(p(0.0, 0.0, 0.0), Vec3::unit_z(), 1.0);
    let b = cylinder(p(1.3, 0.0, 0.0), v(0.0, in_sliver() / LEVER, 1.0), 0.5);
    assert_eq!(tangent(&classify(&a, &b)), "section_cylinder_axes_tilt");
}

/// A plane section of a wall is essential or unbounded on the wall, and
/// its count is never certified single (two rulings or one ellipse).
#[test]
fn a_wall_and_a_plane_are_essential_on_the_wall_and_never_single() {
    let a = cylinder(p(0.0, 0.0, 0.0), Vec3::unit_z(), 1.0);
    for n in [v(0.2, 0.1, 1.0), v(1.0, 0.0, 0.0)] {
        let s = classify(&a, &plane(p(0.0, 0.0, 0.5), n));
        assert_eq!(shape(&s), (1, false, vec![true], vec![false], vec![false]));
    }
}

// -------------------------------------------------------------------
// The sphere
// -------------------------------------------------------------------

#[test]
fn sphere_pairs_every_class() {
    let ball = sphere(p(0.0, 0.0, 0.0), 1.0);
    // Plane and sphere: one circle, on both carriers, either order.
    let pl = plane(p(0.0, 0.0, 0.4), v(0.1, 0.2, 1.0));
    on_both(witness(&classify(&ball, &pl)), &ball, &pl);
    on_both(witness(&classify(&pl, &ball)), &ball, &pl);
    let other = sphere(p(0.9, 0.5, 0.2), 0.7);
    on_both(witness(&classify(&ball, &other)), &ball, &other);
    assert_eq!(shape(&classify(&ball, &sphere(p(0.1, 0.0, 0.0), 0.3))).0, 0);
    // Cylinder: one loop off the axis, witnessed on the nearest ruling.
    let wall = cylinder(p(0.8, 0.0, -2.0), Vec3::unit_z(), 0.5);
    on_both(witness(&classify(&ball, &wall)), &ball, &wall);
    on_both(witness(&classify(&wall, &ball)), &ball, &wall);
    // Two loops, each encircling the cylinder: essential on it.
    let s = classify(&ball, &cylinder(p(0.1, 0.0, -2.0), Vec3::unit_z(), 0.3));
    assert_eq!(
        (shape(&s).2, shape(&s).3),
        (vec![false, false], vec![true, true]),
        "essential on the cylinder, and NOT claimed on the sphere"
    );
    // Viviani's pinch: tangent.
    let s = classify(&ball, &cylinder(p(0.5, 0.0, -2.0), Vec3::unit_z(), 0.5));
    assert_eq!(tangent(&s), "section_sphere_cylinder_girdle");
}

// -------------------------------------------------------------------
// Touches
// -------------------------------------------------------------------

/// The touch `classify` gives `f` against `g`, and `g` against `f`:
/// one and the same point, on both carriers within the band.
fn touch_both_ways(f: &Surface<f64>, g: &Surface<f64>, name: &str) -> Touch<f64> {
    let t = touch(&classify(f, g));
    let u = touch(&classify(g, f));
    assert_eq!((t.name, u.name), (name, name), "{f:?} against {g:?}");
    assert!(
        (t.at - u.at).norm() < 1e-12,
        "{:?} against {:?}",
        t.at,
        u.at
    );
    for s in [f, g] {
        let r = geom_brep::implicit_residual(s, t.at);
        assert!(
            r.abs() <= band().escalate(),
            "touch {:?} is {r} off {s:?}",
            t.at
        );
    }
    t
}

/// `at` lies at the centre of the sampled loop `section`: the loop's
/// centroid (each loop here is symmetric about its centre, and sampled
/// symmetrically) stands within a hundredth of the loop's reach from
/// `at`, so `at` is inside it.
fn centres(label: &str, t: &Touch<f64>, section: &[Point3<f64>]) {
    assert!(
        section.len() > 8,
        "{label}: the sampled loop is {} points",
        section.len()
    );
    let n = section.len() as f64;
    let centroid = section
        .iter()
        .fold(Vec3::new(0.0, 0.0, 0.0), |acc, q| acc + (*q - t.at))
        / n;
    let reach = section
        .iter()
        .map(|q| (*q - t.at).norm())
        .fold(0.0, f64::max);
    assert!(
        centroid.norm() <= 0.01 * reach,
        "{label}: the loop's centroid is {} from the touch, against a loop reach of {reach}",
        centroid.norm()
    );
}

/// The angles `0, 2π/n, …`.
fn turns(n: usize) -> impl Iterator<Item = f64> {
    (0..n).map(move |i| TAU * i as f64 / n as f64)
}

/// The circle about `c` of radius `rad` in the plane normal to `k`.
fn circle_points(c: Point3<f64>, k: Vec3<f64>, rad: f64) -> Vec<Point3<f64>> {
    let (b1, b2) = k.orthonormal_basis();
    turns(64)
        .map(|a| c + b1 * (rad * a.cos()) + b2 * (rad * a.sin()))
        .collect()
}

/// `n + 1` angles evenly across `[−max, max]`.
fn span(max: f64, n: usize) -> impl Iterator<Item = f64> {
    (0..=n).map(move |i| max * (2.0 * i as f64 / n as f64 - 1.0))
}

/// The section of a ball `(e, 0, 0)`, radius `rho`, with the wall of
/// radius `rc` about z: per ruling `θ`, `t² = ρ² − |c − foot(θ)|²`,
/// over the rulings it reaches.
fn ball_wall_loop(rc: f64, e: f64, rho: f64) -> Vec<Point3<f64>> {
    let reach = (1.0 - (rho * rho - (e - rc).powi(2)) / (2.0 * e * rc)).acos();
    span(reach, 512)
        .flat_map(|th| {
            let t2 = rho * rho - (e * e + rc * rc - 2.0 * e * rc * th.cos());
            let h = t2.max(0.0).sqrt();
            [h, -h]
                .into_iter()
                .filter(move |_| t2 >= 0.0)
                .map(move |h| p(rc * th.cos(), rc * th.sin(), h))
        })
        .collect()
}

/// The section of wall 1 (radius 1 about z) with wall 2 (radius 0.5
/// along `u` through `(0, δ, 0)`), per ruling of wall 2 that reaches
/// wall 1.
fn walls_loop(u: Vec3<f64>, delta: f64) -> Vec<Point3<f64>> {
    let w = u.cross(Vec3::unit_y());
    span((1.0 - 2.0 * (1.5 - delta)).acos(), 512)
        .flat_map(|ph| {
            let q = p(0.0, delta, 0.0) + (w * ph.sin() - Vec3::unit_y() * ph.cos()) * 0.5;
            let reach = 1.0 - q.y * q.y;
            let root = reach.max(0.0).sqrt();
            [root, -root]
                .into_iter()
                .filter(move |_| reach >= 0.0)
                .map(move |x| q + u * ((x - q.x) / u.x))
        })
        .collect()
}

/// **A touch reads the point the clearance argument needs**: on both
/// carriers, and at the centre of the loop the carriers share in a pose
/// the decided margin admits where they cross — here pushed `0.9·zero`
/// deep, with each loop solved in closed form or per ruling. In the
/// exact pose it is the tangent point. The poses include a ball deep in
/// a wall (`ρc/e = 50`) and walls 5.7° apart. Mutants: the far ruling,
/// a point on the partner's axis, the wrong foot all leave the loop or
/// the carriers.
#[test]
fn a_touch_is_the_centre_of_every_loop_its_margin_admits() {
    let push = 0.9 * band().zero();
    let k = v(0.3, -0.4, 0.5).normalize();
    let origin = p(0.0, 0.0, 0.0);
    let ball = sphere(origin, 1.0);
    // Sphere × plane.
    let t = touch_both_ways(&ball, &plane(origin + k, k), "section_sphere_plane_reach");
    assert!(
        (t.at - (origin + k)).norm() < 1e-12,
        "tangent at {:?}",
        t.at
    );
    let t = touch_both_ways(
        &ball,
        &plane(origin + k * (1.0 - push), k),
        "section_sphere_plane_reach",
    );
    let rad = (1.0 - (1.0 - push).powi(2)).sqrt();
    centres(
        "sphere × plane",
        &t,
        &circle_points(origin + k * (1.0 - push), k, rad),
    );
    // Sphere × sphere, outside and inside, the ball as F and as the
    // larger or the smaller.
    for (label, r2, d, name) in [
        ("outside", 0.3, 1.3, "section_sphere_pair_reach"),
        ("inside", 0.4, 0.6, "section_sphere_pair_nest"),
    ] {
        let t = touch_both_ways(&ball, &sphere(origin + k * d, r2), name);
        assert!(
            (t.at - (origin + k)).norm() < 1e-12,
            "{label}: tangent at {:?}",
            t.at
        );
        let d = if name.ends_with("reach") {
            d - push
        } else {
            d + push
        };
        let t = touch_both_ways(&ball, &sphere(origin + k * d, r2), name);
        let x = (d * d + 1.0 - r2 * r2) / (2.0 * d);
        centres(
            label,
            &t,
            &circle_points(origin + k * x, k, (1.0 - x * x).sqrt()),
        );
    }
    // Sphere × cylinder, beside, inside, and deep inside the wall.
    for (label, rc, e, rho) in [
        ("ball beside a wall", 0.5, 1.5, 1.0),
        ("ball in a wall", 1.0, 0.3, 0.7),
        ("ball deep in a wall", 1.0, 0.02, 0.98),
    ] {
        let wall = cylinder(origin, Vec3::unit_z(), rc);
        let t = touch_both_ways(
            &sphere(p(e, 0.0, 0.0), rho),
            &wall,
            "section_sphere_cylinder_reach",
        );
        assert!(
            (t.at - p(rc, 0.0, 0.0)).norm() < 1e-12,
            "{label}: tangent at {:?}",
            t.at
        );
        let t = touch_both_ways(
            &sphere(p(e, 0.0, 0.0), rho + push),
            &wall,
            "section_sphere_cylinder_reach",
        );
        centres(label, &t, &ball_wall_loop(rc, e, rho + push));
    }
    // Skew walls outside one another, square and 5.7° apart.
    for tilt in [0.0, 0.1_f64.acos()] {
        let u = v(tilt.cos(), 0.0, tilt.sin());
        let one = cylinder(origin, Vec3::unit_z(), 1.0);
        let pair = |delta: f64| (one.clone(), cylinder(p(0.0, delta, 0.0), u, 0.5));
        let (a, b) = pair(1.5);
        let t = touch_both_ways(&a, &b, "section_cylinder_pair_reach");
        assert!(
            (t.at - p(0.0, 1.0, 0.0)).norm() < 1e-12,
            "walls {tilt}: tangent at {:?}",
            t.at
        );
        let (a, b) = pair(1.5 - push);
        let t = touch_both_ways(&a, &b, "section_cylinder_pair_reach");
        centres(
            &format!("walls tilted {tilt}"),
            &t,
            &walls_loop(u, 1.5 - push),
        );
    }
}

/// **Only a decided `Zero` on a touching arm is a touch.** An undecided
/// reach margin, a girdle pinch, an inner skew tangency (where the
/// thinner wall leaves the fatter on both sides of the touch), a
/// coincident wall pair, a ball about a wall's axis and two nested
/// spheres about one centre stay R-tan.
#[test]
fn pinches_and_undecided_tangencies_are_not_touches() {
    let ball = sphere(p(0.0, 0.0, 0.0), 1.0);
    let pl = plane(p(0.0, 0.0, 1.0 - in_sliver()), Vec3::unit_z());
    assert_eq!(tangent(&classify(&ball, &pl)), "section_sphere_plane_reach");
    let girdle = cylinder(p(0.5, 0.0, -2.0), Vec3::unit_z(), 0.5);
    assert_eq!(
        tangent(&classify(&ball, &girdle)),
        "section_sphere_cylinder_girdle"
    );
    let fat = cylinder(p(0.0, 0.0, 0.0), Vec3::unit_z(), 1.0);
    let inner = cylinder(p(0.0, 0.8, 0.0), Vec3::unit_x(), 0.2);
    assert_eq!(
        tangent(&classify(&fat, &inner)),
        "section_cylinder_pair_nest"
    );
    let same = cylinder(p(0.0, 0.0, 3.0), Vec3::unit_z(), 1.0);
    assert_eq!(
        tangent(&classify(&fat, &same)),
        "section_cylinder_pair_coincident"
    );
    // A ball about a wall's axis touches it all round, and two spheres
    // one inside the other about one centre stand within the band of
    // each other everywhere: neither is the extreme of a level clear of
    // the next.
    let off = 0.3 * band().zero();
    assert_eq!(
        tangent(&classify(&sphere(p(off, 0.0, 0.0), 1.0), &fat)),
        "section_sphere_cylinder_reach"
    );
    assert_eq!(
        tangent(&classify(&ball, &sphere(p(off, 0.0, 0.0), 1.0 - off))),
        "section_sphere_pair_nest"
    );
}

/// The rows' donut at `(u, v)`.
fn donut_at(u: f64, v: f64) -> Point3<f64> {
    let rho = 2.0 + 0.5 * v.cos();
    p(rho * u.cos(), rho * u.sin(), 0.5 * v.sin())
}

/// The donut's section with `partner` about `at`, on 64 rays out of
/// `at`'s parameters, evenly in angle at unit speed: each ray's first
/// change of the partner's residual from its sign at `at`, by bisection.
fn donut_loop(partner: &Surface<f64>, at: Point3<f64>) -> Vec<Point3<f64>> {
    let (u0, v0) = (at.y.atan2(at.x), at.z.atan2(at.x.hypot(at.y) - 2.0));
    let g = |q| geom_brep::implicit_residual(partner, q).signum();
    let deep = g(donut_at(u0, v0));
    turns(64)
        .map(|phi| {
            let at_t = |t: f64| {
                donut_at(
                    u0 + t * phi.cos() / (2.0 + 0.5 * v0.cos()),
                    v0 + t * phi.sin() / 0.5,
                )
            };
            let (mut lo, mut hi) = (0.0, 1e-10);
            while g(at_t(hi)) == deep {
                (lo, hi) = (hi, 2.0 * hi);
                assert!(hi < 0.5, "no crossing on the ray at {phi} from {at:?}");
            }
            for _ in 0..100 {
                let mid = 0.5 * (lo + hi);
                if g(at_t(mid)) == deep {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            at_t(0.5 * (lo + hi))
        })
        .collect()
}

/// **A torus touch is at the elliptic extreme, and at the centre of
/// every loop its margin admits**: a plane on the outer equator on
/// either tube circle and tilted onto the tube's outer half, a ball
/// outside the tube, one about the whole torus and one inside the tube,
/// and a wall parallel to the axis beside the torus and about it. In
/// the exact pose `at` is the tangent point; pushed `0.9·zero` across,
/// it centres the section loop, sampled on the torus. Mutants: the foot
/// on the other side of the tube circle, the farthest point for the
/// nearest, the wall's far ruling.
#[test]
fn a_torus_touch_is_the_centre_of_every_loop_its_margin_admits() {
    let push = 0.9 * band().zero();
    let x = Vec3::unit_x();
    let z = Vec3::unit_z();
    let k = v(0.6f64.cos(), 0.0, 0.6f64.sin());
    let foot = p(2.0, 0.0, 0.0) + k * 0.5;
    let about = p(0.5, 0.0, 0.3);
    let d_far = 2.5f64.hypot(0.3);
    let in_tube = p(2.2, 0.0, 0.1);
    let d_near = 0.2f64.hypot(0.1);
    const PLANE_NEAR: &str = "section_torus_plane_near_tube";
    const SPHERE_NEAR: &str = "section_torus_sphere_near_tube";
    const WALL: &str = "section_torus_offset_wall_near_outer";
    // (label, the margin, the exact pose, the pose pushed across, the
    // tangent point)
    type Row = (
        &'static str,
        &'static str,
        Surface<f64>,
        Surface<f64>,
        Point3<f64>,
    );
    let rows: [Row; 8] = [
        (
            "plane on the outer equator",
            PLANE_NEAR,
            plane(p(2.5, 0.0, 0.0), x),
            plane(p(2.5 - push, 0.0, 0.0), x),
            p(2.5, 0.0, 0.0),
        ),
        (
            "plane on the far tube's outer equator",
            "section_torus_plane_far_tube",
            plane(p(-2.5, 0.0, 0.0), x),
            plane(p(-2.5 + push, 0.0, 0.0), x),
            p(-2.5, 0.0, 0.0),
        ),
        (
            "tilted plane",
            PLANE_NEAR,
            plane(foot, k),
            plane(foot - k * push, k),
            foot,
        ),
        (
            "ball outside the tube",
            SPHERE_NEAR,
            sphere(foot + k * 0.7, 0.7),
            sphere(foot + k * 0.7, 0.7 + push),
            foot,
        ),
        (
            "ball about the torus",
            "section_torus_sphere_far_tube",
            sphere(about, d_far + 0.5),
            sphere(about, d_far + 0.5 - push),
            p(-2.0 - 0.5 * 2.5 / d_far, 0.0, -0.5 * 0.3 / d_far),
        ),
        (
            "ball inside the tube",
            SPHERE_NEAR,
            sphere(in_tube, 0.5 - d_near),
            sphere(in_tube, 0.5 - d_near + push),
            p(2.0 + 0.5 * 0.2 / d_near, 0.0, 0.5 * 0.1 / d_near),
        ),
        (
            "wall beside the torus",
            WALL,
            cylinder(p(3.0, 0.0, -1.0), z, 0.5),
            cylinder(p(3.0, 0.0, -1.0), z, 0.5 + push),
            p(2.5, 0.0, 0.0),
        ),
        (
            "wall about the torus",
            WALL,
            cylinder(p(0.3, 0.0, -1.0), z, 2.8),
            cylinder(p(0.3, 0.0, -1.0), z, 2.8 - push),
            p(-2.5, 0.0, 0.0),
        ),
    ];
    for (label, name, exact, pushed, tangent_at) in rows {
        let t = touch_both_ways(&donut(), &exact, name);
        assert!(
            (t.at - tangent_at).norm() < 1e-12,
            "{label}: tangent at {:?}",
            t.at
        );
        let t = touch_both_ways(&donut(), &pushed, name);
        centres(label, &t, &donut_loop(&pushed, t.at));
    }
}

/// **A torus tangency that is not an elliptic extreme stays R-tan**: a
/// plane touching the tube near its top circle with the elliptic margin
/// undecided and decided `Zero`; a ball centred on the axis on the outer
/// equator all round; a ball centred on the core circle, on a whole
/// meridian; a wall in the hole on the inner equator; a wall crossing
/// the tube on the outer equator. Mutants: the elliptic margin, the
/// extreme margin, or the nearest-end margin dropped; any wall pinch
/// read as a touch.
#[test]
fn torus_tangencies_off_an_elliptic_extreme_are_not_touches() {
    for (label, s) in [
        ("undecided", in_sliver() / 0.5),
        ("zero", 0.5 * band().zero() / 0.5),
    ] {
        let n = v(s, 0.0, (1.0 - s * s).sqrt());
        let pl = plane(p(0.0, 0.0, 0.0) + n * (2.0 * s + 0.5), n);
        assert_eq!(
            tangent(&classify(&donut(), &pl)),
            "section_torus_plane_near_tube",
            "{label}"
        );
    }
    for ball in [sphere(p(0.0, 0.0, 0.0), 2.5), sphere(p(2.0, 0.0, 0.0), 0.5)] {
        assert_eq!(
            tangent(&classify(&donut(), &ball)),
            "section_torus_sphere_near_tube",
            "{ball:?}"
        );
    }
    let z = Vec3::unit_z();
    assert_eq!(
        tangent(&classify(&donut(), &cylinder(p(1.0, 0.0, -1.0), z, 0.5))),
        "section_torus_offset_wall_far_inner"
    );
    assert_eq!(
        tangent(&classify(&donut(), &cylinder(p(2.2, 0.0, -1.0), z, 0.3))),
        "section_torus_offset_wall_far_outer"
    );
}

/// **A touch clears only on a pair with no event, and only out of a
/// face.** `Out` of either face clears on that side; a touch on both
/// faces, or one no face places, refuses R-tan; and an event on the
/// pair refuses whatever the placement — the clearance argument needs
/// the pair's own silence (mutant: reading the placement on an evented
/// pair).
#[test]
fn a_touch_clears_only_out_of_a_face_on_a_silent_pair() {
    let s = Section::Touch(Touch {
        name: "section_sphere_plane_reach",
        at: p(0.0, 0.0, 1.0),
    });
    let tan = Err(Refusal::Tangent("section_sphere_plane_reach"));
    let rule = |evented: bool, place: [Option<FaceContainment>; 2]| {
        certify(&s, evented, |_| false, |_| place)
    };
    assert_eq!(rule(false, [OUT, IN]), Ok(vec![Cleared::TouchOut(Side::F)]));
    assert_eq!(rule(false, [IN, OUT]), Ok(vec![Cleared::TouchOut(Side::G)]));
    assert_eq!(rule(false, [IN, IN]), tan);
    assert_eq!(rule(false, [None, None]), tan);
    assert_eq!(rule(true, [OUT, OUT]), tan);
    assert_eq!(rule(true, [OUT, IN]), tan);
}

// -------------------------------------------------------------------
// The per-pair rule
// -------------------------------------------------------------------

fn lone_at(w: Point3<f64>) -> Section<f64> {
    Section::Components {
        parts: vec![Component {
            unbounded: false,
            essential_f: false,
            essential_g: false,
            witness: Some(w),
        }],
        single: true,
    }
}

pub(super) fn at(
    f: Option<FaceContainment>,
    g: Option<FaceContainment>,
) -> impl FnMut(Point3<f64>) -> [Option<FaceContainment>; 2] {
    move |_| [f, g]
}

pub(super) const IN: Option<FaceContainment> = Some(FaceContainment::In);
pub(super) const OUT: Option<FaceContainment> = Some(FaceContainment::Out);

/// **The no-event decision.** A lone component with no event: `Out` of
/// either face clears (W3, whichever face); `In` both is R-loop; a
/// point with no verdict or on a boundary is R-undec.
#[test]
fn the_no_event_decision_reads_one_point() {
    let s = lone_at(p(0.0, 0.0, 0.0));
    let ok = |r| certify(&s, false, |_| false, r);
    assert_eq!(ok(at(OUT, IN)), Ok(vec![Cleared::Out(Side::F)]));
    assert_eq!(ok(at(IN, OUT)), Ok(vec![Cleared::Out(Side::G)]));
    assert_eq!(ok(at(IN, IN)), Err(Refusal::Loop));
    assert_eq!(ok(at(IN, None)), Err(Refusal::Undecided));
    assert_eq!(
        ok(at(
            IN,
            Some(FaceContainment::OnVertex(crate::VertexKey::default()))
        )),
        Err(Refusal::Undecided)
    );
}

/// **W4 needs a CERTIFIED single component.** With an event, a lone
/// component clears whatever its point says; a two-component section
/// with an event and no other witness refuses, and so does one whose
/// count is not certified. No tractable arm reaches the two-component
/// case on faces that describe (every multi-component section is
/// essential or unbounded), so this row is on the rule.
#[test]
fn w4_clears_only_a_certified_single_component() {
    let s = lone_at(p(0.0, 0.0, 0.0));
    assert_eq!(
        certify(&s, true, |_| false, at(IN, IN)),
        Ok(vec![Cleared::LoneEvented])
    );
    let c = Component {
        unbounded: false,
        essential_f: false,
        essential_g: false,
        witness: Some(p(0.0, 0.0, 0.0)),
    };
    let two = Section::Components {
        parts: vec![c, c],
        single: false,
    };
    assert_eq!(
        certify(&two, true, |_| false, at(IN, IN)),
        Err(Refusal::Undecided)
    );
    let uncounted = Section::Components {
        parts: vec![c],
        single: false,
    };
    assert_eq!(
        certify(&uncounted, true, |_| false, at(IN, IN)),
        Err(Refusal::Undecided)
    );
}

/// **W2 asks the face, and only for the side the component is
/// essential on.**
#[test]
fn w2_clears_only_where_the_face_describes() {
    let s = classify(
        &cylinder(p(0.0, 0.0, 0.0), Vec3::unit_z(), 1.0),
        &plane(p(0.0, 0.0, 0.5), v(0.2, 0.0, 1.0)),
    );
    assert_eq!(
        certify(&s, false, |side| side == Side::F, at(None, None)),
        Ok(vec![Cleared::Essential(Side::F)])
    );
    assert_eq!(
        certify(&s, false, |side| side == Side::G, at(None, None)),
        Err(Refusal::Undecided)
    );
}

// -------------------------------------------------------------------
// The scan over real faces
// -------------------------------------------------------------------

/// The unit wall's full circle at height `z`, as the intersection of
/// `cyl` and the rim plane there, whose normal is `normal`; and that
/// plane's key, for the cap the circle bounds.
fn full_rim(
    body: &mut Body<f64>,
    cyl: crate::geometry::SurfaceKey,
    z: f64,
    normal: Vec3<f64>,
) -> (EdgeCurveSpec<f64>, crate::geometry::SurfaceKey) {
    let rim_plane = body.add_surface(plane(p(0.0, 0.0, z), normal));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            s1: cyl,
            s2: rim_plane,
            witness: p(-1.0, 0.0, z),
        },
        carrier: Curve3::Circle {
            center: p(0.0, 0.0, z),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        },
        param_start: 0.0,
        param_end: TAU,
    };
    (spec, rim_plane)
}

/// **A seamless band**: one face on the unit cylinder over
/// `z ∈ [z0, z1]`, bounded by its two full rim circles and carrying no
/// meridian edge. Each rim circle splits off a cap disc, a strut joins
/// the two, and the strut's kill leaves the band's boundary as two
/// loops.
fn seamless_band(z0: f64, z1: f64) -> (Body<f64>, FaceKey) {
    let tol = Tol::witness();
    let frame = CylFrame::canonical(1.0);
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(frame.at(0.0, z0), true).unwrap();
    let cyl = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: frame.surface(),
                sense: true,
            },
        )
        .unwrap();
    let (bottom, bottom_plane) = full_rim(&mut body, cyl, z0, v(0.0, 0.0, -1.0));
    let cap_b = body
        .mef(
            MefSite::Lone {
                r#loop: seed.r#loop,
            },
            bottom,
            FaceSurface::Shared {
                key: bottom_plane,
                sense: true,
            },
            tol,
        )
        .unwrap();
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: cap_b.he_plus,
                he2: cap_b.he_plus,
            },
            frame.at(0.0, z1),
            tol,
        )
        .unwrap();
    let (top, top_plane) = full_rim(&mut body, cyl, z1, v(0.0, 0.0, 1.0));
    body.mef(
        MefSite::Chords {
            he1: strut.he_minus,
            he2: strut.he_minus,
        },
        top,
        FaceSurface::Shared {
            key: top_plane,
            sense: true,
        },
        tol,
    )
    .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    (body, seed.face)
}

/// The verdict of every pair of `a`'s `face` against `b`'s faces on the
/// crossings path, with no events.
pub(super) fn scan(
    a: &Body<f64>,
    face: FaceKey,
    b: &Body<f64>,
) -> Vec<Result<Vec<Cleared>, Refusal>> {
    ops::section_pairs(
        a,
        b,
        band(),
        ops::SectionPath::Crossings,
        ops::Exempt::Nothing,
        |_, _| false,
        false,
    )
    .unwrap()
    .into_iter()
    .filter(|pv| pv.a_face == face)
    .map(|pv| pv.verdict)
    .collect()
}

/// A slab tilted by 0.2 rad about `y`, crossing the unit wall about `z`
/// in ellipses through `z ∈ [−0.3, 0.3]`.
pub(super) fn tilted_slab() -> Body<f64> {
    let b = brick::<f64>((-3.0, 3.0), (-3.0, 3.0), (-0.1, 0.1), Tol::witness());
    let tilt = geom_core::Affine3::rotation_about_axis(p(0.0, 0.0, 0.0), Vec3::unit_y(), 0.2);
    crate::transform::transform_rigid(&b, &tilt, Tol::witness()).unwrap()
}

/// **W2 reads `chart_boundary` per face, never the kind.** A seamless
/// band cut by a tilted slab in ellipses inside both faces refuses:
/// nothing about the kind forbids such a face, and the meridian-edge
/// premise the old wall gate assumed fails on it. A W2 that took
/// "cylinder" for "describes" clears it: red. The banded wall of the
/// same carrier, which carries its meridians, clears by W2.
#[test]
fn w2_refuses_a_seamless_band_and_clears_a_banded_wall() {
    let (band_body, band_face) = seamless_band(-1.0, 1.0);
    let surface = band_body
        .get_surface(band_body.get_face(band_face).unwrap().surface)
        .unwrap()
        .clone();
    assert!(
        matches!(
            crate::pcurves::chart_boundary(&band_body, band_face, &surface, band()),
            Err(crate::PcurveMintError::LoopWraps { .. })
        ),
        "the seamless band's loops each close a period off"
    );
    let slab = tilted_slab();
    let verdicts = scan(&band_body, band_face, &slab);
    assert!(
        verdicts.contains(&Err(Refusal::Undecided)),
        "the band's ellipse pairs refuse: {verdicts:?}"
    );
    let mut wall_body = Body::<f64>::new();
    let wall = cyl_wall_sheet(
        &mut wall_body,
        CylFrame::canonical(1.0),
        None,
        (0.0, PI),
        (-1.0, 1.0),
        Tol::witness(),
    );
    let verdicts = scan(&wall_body, wall, &slab);
    assert!(!verdicts.is_empty());
    assert!(
        verdicts
            .iter()
            .all(|v| *v == Ok(vec![Cleared::Essential(Side::F)])),
        "{verdicts:?}"
    );
}

/// **A face carrying a lone-vertex loop refuses every pair it enters.**
/// The sweep never examines a lone vertex (it is edge-driven), so the
/// certificate refuses rather than rest an answer on the argument that
/// such a vertex hides no component (module docs). The same wall
/// without the ring clears. The strut that leaves the ring is a secant
/// of the cylinder, so `mev` leaves the wall storing no row; the wall is
/// re-minted once the ring is made, so both scans read a minted wall.
#[test]
fn a_lone_vertex_ring_refuses_the_pair() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let wall = cyl_wall_sheet(
        &mut body,
        CylFrame::canonical(1.0),
        None,
        (0.0, PI),
        (-1.0, 1.0),
        tol,
    );
    let slab = tilted_slab();
    let before = scan(&body, wall, &slab);
    assert!(!before.is_empty());
    assert!(
        before
            .iter()
            .all(|v| *v == Ok(vec![Cleared::Essential(Side::F)]))
    );
    let crate::LoopBoundary::Cycle { first } = body
        .get_loop(body.get_face(wall).unwrap().outer)
        .unwrap()
        .boundary
    else {
        panic!("the wall's outer loop is a cycle")
    };
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: first,
                he2: first,
            },
            CylFrame::canonical(1.0).at(0.5, 0.0),
            tol,
        )
        .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    crate::pcurves::mint_pcurves(&mut body, tol).unwrap();
    assert!(
        body.get_face(wall).unwrap().rings.iter().any(|&l| matches!(
            body.get_loop(l).unwrap().boundary,
            crate::LoopBoundary::Empty { .. }
        )),
        "the kill leaves a lone-vertex ring"
    );
    let after = scan(&body, wall, &slab);
    assert_eq!(after.len(), before.len());
    assert!(
        after.iter().all(|v| *v == Err(Refusal::LoneVertex)),
        "{after:?}"
    );
}

// -------------------------------------------------------------------
// The lever's pivot: the same carrier line, whatever its stored origin
// -------------------------------------------------------------------

/// The classification of a torus pair whose region is the ball
/// `(centre, radius)`: the overlap box's centre and half-diagonal.
pub(super) fn classify_near(
    f: &Surface<f64>,
    g: &Surface<f64>,
    centre: Point3<f64>,
    radius: f64,
) -> Section<f64> {
    super::classify(f, g, super::Reach { centre, radius }, band())
}

/// A carrier line through `through` with direction `d`, stored with its
/// origin `far` metres along `d` from there.
fn far_cylinder(through: Point3<f64>, d: Vec3<f64>, radius: f64, far: f64) -> Surface<f64> {
    let d = d.normalize();
    cylinder(through + d * far, d, radius)
}

/// **Case A: a pin's offset is read near the torus, not at its stored
/// origin.** A pin of radius `0.2` whose axis passes `x = 2.7 − 8e-8`
/// at the torus's midplane — so `ρ_min = 2.5 − 8e-8`, just inside the
/// outer equator: one null loop, the scrape — tilted `1.9e-10` rad
/// (decided `Zero` at the pair's lever) with its origin 1 km along the
/// axis, where the axis has drifted `1.9e-7` further out. Read at the
/// origin, the offset puts `ρ_min` beyond the tube and the pair would
/// clear W0 with a true loop in it. Where the band takes in the `8e-8`
/// (ε = 1e-6), the pin touches the outer equator, and the touch is the
/// nearest ruling read at the pivot, not `1.9e-7` out at the origin.
#[test]
fn a_far_origin_does_not_move_a_pins_offset() {
    let tilt = 1.9e-10;
    let pin = far_cylinder(p(2.7 - 8e-8, 0.0, 0.0), v(tilt, 0.0, 1.0), 0.2, 1000.0);
    let s = classify_near(&donut(), &pin, p(2.5, 0.0, 0.0), 2.5);
    match &s {
        Section::Components { parts, .. } => {
            assert!(!parts.is_empty(), "a true scrape loop cleared as apart")
        }
        Section::Tangent(_) | Section::Intractable => {}
        Section::Touch(t) => assert!(
            (t.at - p(2.5 - 8e-8, 0.0, 0.0)).norm() < 1e-9,
            "the touch is read off the pivot: {t:?}"
        ),
    }
}

/// **Case B: a wall's coaxiality is read near the torus.** A wall of
/// radius `2.5 − 1e-7`, its axis tilted `1.5e-10` rad and meeting the
/// torus axis 1000 m up, where its origin is stored: at the torus the
/// axes stand `1.5e-7` apart, so the wall pokes out past the outer
/// equator on one side and the section is ONE null loop. Read at the
/// origin the pair is coaxial, and two essential parallels would clear
/// it by W2.
#[test]
fn a_far_origin_does_not_make_a_wall_coaxial() {
    let tilt = 1.5e-10;
    let wall = cylinder(p(0.0, 0.0, 1000.0), v(-tilt, 0.0, -1.0), 2.5 - 1e-7);
    let s = classify_near(&donut(), &wall, p(0.0, 0.0, 0.0), 3.0);
    if let Section::Components { parts, .. } = &s {
        assert!(
            !(parts.len() == 2 && parts.iter().all(|c| c.essential_f && c.essential_g)),
            "one null loop classified as two essential parallels: {s:?}"
        );
    }
}

// -------------------------------------------------------------------
// The scan's per-face plumbing
// -------------------------------------------------------------------

/// The verdict of every pair of `b`'s `face` against `a`'s faces.
pub(super) fn scan_b(
    a: &Body<f64>,
    b: &Body<f64>,
    face: FaceKey,
) -> Vec<Result<Vec<Cleared>, Refusal>> {
    ops::section_pairs(
        a,
        b,
        band(),
        ops::SectionPath::Crossings,
        ops::Exempt::Nothing,
        |_, _| false,
        false,
    )
    .unwrap()
    .into_iter()
    .filter(|pv| pv.b_face == face)
    .map(|pv| pv.verdict)
    .collect()
}

/// **W2 asks the face the component is essential ON, from its own
/// operand.** The seamless band as operand B against a slab as A: the
/// ellipses are essential on the band (G), which does not describe, so
/// every such pair refuses. A W2 that asked F's (the slab plane's)
/// chart for a G claim clears them: red.
#[test]
fn w2_asks_the_partner_operands_face_for_a_g_claim() {
    let (band_body, band_face) = seamless_band(-1.0, 1.0);
    let verdicts = scan_b(&tilted_slab(), &band_body, band_face);
    assert!(!verdicts.is_empty());
    assert!(verdicts.contains(&Err(Refusal::Undecided)), "{verdicts:?}");
    assert!(
        !verdicts
            .iter()
            .any(|v| matches!(v, Ok(c) if c.contains(&Cleared::Essential(Side::G)))),
        "{verdicts:?}"
    );
}

/// **The chart cache is keyed by operand.** The two arenas mint keys
/// independently: a describing face of A and the seamless band of B can
/// share a key, and the band must not inherit A's answer.
#[test]
fn the_chart_cache_does_not_share_a_key_across_operands() {
    let (band_body, band_face) = seamless_band(-1.0, 1.0);
    let mut wall_body = Body::<f64>::new();
    let _ = cyl_wall_sheet(
        &mut wall_body,
        CylFrame::canonical(1.0),
        None,
        (0.0, PI),
        (-1.0, 1.0),
        Tol::witness(),
    );
    // The sheet's seed face is the band's key in its own arena.
    assert!(wall_body.get_face(band_face).is_some(), "the keys coincide");
    let surface = |b: &Body<f64>, f| {
        b.get_surface(b.get_face(f).unwrap().surface)
            .unwrap()
            .clone()
    };
    let mut cache = ops::ChartCache::default();
    assert!(cache.describes(
        Operand::A,
        &wall_body,
        band_face,
        &surface(&wall_body, band_face),
        band()
    ));
    assert!(!cache.describes(
        Operand::B,
        &band_body,
        band_face,
        &surface(&band_body, band_face),
        band()
    ));
}

/// **A witness placed OFF a face's carrier is no verdict, never `Out`.**
/// Every witness is built on both carriers, so a definite off-carrier
/// answer contradicts its construction; reading it as `Out` would clear
/// a component by W3 on a point that is not on the component at all.
#[test]
fn an_off_carrier_witness_places_nowhere() {
    let mut body = Body::<f64>::new();
    let wall = cyl_wall_sheet(
        &mut body,
        CylFrame::canonical(1.0),
        None,
        (0.0, PI),
        (-1.0, 1.0),
        Tol::witness(),
    );
    let surface = body
        .get_surface(body.get_face(wall).unwrap().surface)
        .unwrap()
        .clone();
    assert_eq!(
        ops::place_witness(&body, wall, &surface, p(0.0, 1.5, 0.0), band()),
        None,
        "0.5 off the unit wall"
    );
    // On the carrier and outside the half-wall's window: a real `Out`.
    assert_eq!(
        ops::place_witness(&body, wall, &surface, p(0.0, -1.0, 0.0), band()),
        Some(FaceContainment::Out)
    );
}

/// **A lone-vertex ring on operand B's face refuses too.** The wall is
/// re-minted once the ring is made, as in the A-side row.
#[test]
fn a_lone_vertex_ring_on_the_b_side_refuses_the_pair() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let wall = cyl_wall_sheet(
        &mut body,
        CylFrame::canonical(1.0),
        None,
        (0.0, PI),
        (-1.0, 1.0),
        tol,
    );
    let crate::LoopBoundary::Cycle { first } = body
        .get_loop(body.get_face(wall).unwrap().outer)
        .unwrap()
        .boundary
    else {
        panic!("the wall's outer loop is a cycle")
    };
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: first,
                he2: first,
            },
            CylFrame::canonical(1.0).at(0.5, 0.0),
            tol,
        )
        .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    crate::pcurves::mint_pcurves(&mut body, tol).unwrap();
    let verdicts = scan_b(&tilted_slab(), &body, wall);
    assert!(!verdicts.is_empty());
    assert!(
        verdicts.iter().all(|v| *v == Err(Refusal::LoneVertex)),
        "{verdicts:?}"
    );
}

/// **A declaration speaks for its own pair only.** A face of A declared
/// against B's face `Y` meets B's other faces undeclared, and B's `Y`
/// meets A's other faces undeclared.
#[test]
fn a_declaration_exempts_its_own_pair_only() {
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let b = brick::<f64>((0.0, 1.0), (0.0, 1.0), (1.0, 2.0), Tol::witness());
    let fa: Vec<FaceKey> = a.faces().map(|(k, _)| k).collect();
    let fb: Vec<FaceKey> = b.faces().map(|(k, _)| k).collect();
    let decls = BooleanDeclarations {
        coincident_faces: vec![FacePairDeclaration {
            a: fa[0],
            b: fb[0],
            class: crate::contact::BooleanCoincidence::REST,
        }],
        ..BooleanDeclarations::default()
    };
    assert!(ops::declares_pair(&decls, fa[0], fb[0]));
    assert!(!ops::declares_pair(&decls, fa[0], fb[1]));
    assert!(!ops::declares_pair(&decls, fa[1], fb[0]));
}

// -------------------------------------------------------------------
// W2 on apex-closed cone faces: the apex closure
// -------------------------------------------------------------------

/// The unit cone about `z`, apex at the origin, half-angle π/4: the
/// rim at `z = 1` has radius 1.
pub(super) fn unit_cone() -> Surface<f64> {
    Surface::Cone {
        apex: p(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        half_angle: PI / 4.0,
        u_ref: Vec3::unit_x(),
    }
}

/// The rim point at azimuth `t`.
fn rim_at(t: f64) -> Point3<f64> {
    p(t.cos(), t.sin(), 1.0)
}

/// **A bow-tie**: one face of the unit cone whose outline runs through
/// the apex twice, bounding the sectors `[π/2, 3π/4]` and `[0, π/4]`,
/// beside the two single-sector faces cut from it. Returns the body,
/// the bow-tie and one sector.
fn cone_bow_tie() -> (Body<f64>, FaceKey, FaceKey) {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(0.0, 0.0, 0.0), true).unwrap();
    let cone = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: unit_cone(),
                sense: true,
            },
        )
        .unwrap();
    let arc = |body: &mut Body<f64>, t0: f64, t1: f64| {
        let rim_plane = body.add_surface(plane(p(0.0, 0.0, 1.0), Vec3::unit_z()));
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1: cone,
                s2: rim_plane,
                witness: rim_at(0.5 * (t0 + t1)),
            },
            carrier: Curve3::Circle {
                center: p(0.0, 0.0, 1.0),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            param_start: t0,
            param_end: t1,
        }
    };
    let e1 = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            rim_at(0.0),
            tol,
        )
        .unwrap();
    let spec = arc(&mut body, 0.0, PI / 4.0);
    let e2 = body
        .mev(
            MevSite::Fan {
                he1: e1.he_minus,
                he2: e1.he_minus,
            },
            rim_at(PI / 4.0),
            spec,
            tol,
        )
        .unwrap();
    let e3 = body
        .mev_line(
            MevSite::Fan {
                he1: e1.he_plus,
                he2: e1.he_plus,
            },
            rim_at(PI / 2.0),
            tol,
        )
        .unwrap();
    let spec = arc(&mut body, PI / 2.0, 0.75 * PI);
    let e4 = body
        .mev(
            MevSite::Fan {
                he1: e3.he_minus,
                he2: e3.he_minus,
            },
            rim_at(0.75 * PI),
            spec,
            tol,
        )
        .unwrap();
    let sector = body
        .mef(
            MefSite::Chords {
                he1: e2.he_minus,
                he2: e3.he_plus,
            },
            EdgeCurveSpec::line_between(rim_at(PI / 4.0), p(0.0, 0.0, 0.0)),
            FaceSurface::Shared {
                key: cone,
                sense: true,
            },
            tol,
        )
        .unwrap()
        .face;
    body.mef(
        MefSite::Chords {
            he1: e4.he_minus,
            he2: e1.he_plus,
        },
        EdgeCurveSpec::line_between(rim_at(0.75 * PI), p(0.0, 0.0, 0.0)),
        FaceSurface::Shared {
            key: cone,
            sense: true,
        },
        tol,
    )
    .unwrap();
    (body, seed.face, sector)
}

/// **A face through the apex twice has no single lift, and does not
/// describe.** Its two sectors' gap lies between them on the cone, and
/// no one window can exclude it. A single sector cut from the same
/// sheet does describe. The mutant that relaxes "exactly one apex
/// visit" lifts the bow-tie from its first visit and clears W2 on it.
#[test]
fn a_bow_tie_through_the_apex_twice_does_not_describe() {
    use crate::chord_join::{ApexClosure, cone_apex_closure};
    let (body, bow_tie, sector) = cone_bow_tie();
    let cone = unit_cone();
    let crate::LoopBoundary::Cycle { first } = body
        .get_loop(body.get_face(bow_tie).unwrap().outer)
        .unwrap()
        .boundary
    else {
        panic!("the bow-tie's outline is a cycle");
    };
    let apex_visits = body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .filter(|&he| {
            let v = body.get_half_edge(he).unwrap().start;
            (crate::readback::vertex_point(&body, v).unwrap() - p(0.0, 0.0, 0.0)).norm() == 0.0
        })
        .count();
    assert_eq!(
        apex_visits, 2,
        "the fixture's outline visits the apex twice"
    );
    assert!(matches!(
        cone_apex_closure(&body, &cone, bow_tie, band()).unwrap(),
        ApexClosure::Open
    ));
    let mut charts = ops::ChartCache::default();
    assert!(
        !charts.describes(Operand::A, &body, bow_tie, &cone, band()),
        "the bow-tie must not describe"
    );
    assert!(
        matches!(
            cone_apex_closure(&body, &cone, sector, band()).unwrap(),
            ApexClosure::Closed { .. }
        ),
        "one sector visits the apex once"
    );
    assert!(
        charts.describes(Operand::A, &body, sector, &cone, band()),
        "a single sector describes"
    );
}

/// One step of a [`cone_sheet`] chain.
#[derive(Clone, Copy)]
pub(super) enum Step {
    /// A straight edge to the point.
    Line(Point3<f64>),
    /// A rim arc at height `h` (radius `|h|`), azimuth `t0` to `t1`,
    /// either way round.
    Arc(f64, f64, f64),
}

/// The point at height `h` and azimuth `t` on [`unit_cone`] (either
/// nappe: `h < 0` is the mirror one).
pub(super) fn cone_at(h: f64, t: f64) -> Point3<f64> {
    p(h.abs() * t.cos(), h.abs() * t.sin(), h)
}

/// **One face of `surface`, bounded by a chain of `n` edges grown by
/// `mev` from `start` and closed back to it by `mef`.** `step(body,
/// key, i)` is the `i`-th edge: its end point, and its spec (`None` for
/// a straight edge). `close(body, key, end)` is the closing edge, which
/// runs from `start` to the chain's last vertex `end`, as `mef` states
/// it. The face returned is the one the chain bounds in its own order.
fn chain_sheet(
    surface: Surface<f64>,
    start: Point3<f64>,
    n: usize,
    mut step: impl FnMut(
        &mut Body<f64>,
        crate::geometry::SurfaceKey,
        usize,
    ) -> (Point3<f64>, Option<EdgeCurveSpec<f64>>),
    close: impl FnOnce(&mut Body<f64>, crate::geometry::SurfaceKey, Point3<f64>) -> EdgeCurveSpec<f64>,
) -> (Body<f64>, FaceKey) {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(start, true).unwrap();
    let key = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface,
                sense: true,
            },
        )
        .unwrap();
    let mut edges: Vec<crate::MevCreated> = Vec::new();
    for i in 0..n {
        let site = match edges.last() {
            None => MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Some(e) => MevSite::Fan {
                he1: e.he_minus,
                he2: e.he_minus,
            },
        };
        let (to, spec) = step(&mut body, key, i);
        edges.push(match spec {
            None => body.mev_line(site, to, tol).unwrap(),
            Some(spec) => body.mev(site, to, spec, tol).unwrap(),
        });
    }
    let (first, last) = (edges[0], edges[edges.len() - 1]);
    let end = crate::readback::vertex_point(&body, last.vertex).unwrap();
    let spec = close(&mut body, key, end);
    let face = body
        .mef(
            MefSite::Chords {
                he1: first.he_plus,
                he2: last.he_minus,
            },
            spec,
            FaceSurface::Shared { key, sense: true },
            tol,
        )
        .unwrap()
        .face;
    (body, face)
}

/// **One face of the unit cone, bounded by a chain of [`Step`]s from
/// `start` closed back to it by a straight edge** ([`chain_sheet`]).
pub(super) fn cone_sheet(start: Point3<f64>, steps: &[Step]) -> (Body<f64>, FaceKey) {
    chain_sheet(
        unit_cone(),
        start,
        steps.len(),
        |body, cone, i| match steps[i] {
            Step::Line(to) => (to, None),
            Step::Arc(h, t0, t1) => {
                let rim_plane = body.add_surface(plane(p(0.0, 0.0, h), Vec3::unit_z()));
                let spec = EdgeCurveSpec {
                    description: EdgeDescriptionSpec::Intersection {
                        s1: cone,
                        s2: rim_plane,
                        witness: cone_at(h, 0.5 * (t0 + t1)),
                    },
                    // A descending arc runs forward on the reversed
                    // axis, its seam moved to `t0`.
                    carrier: Curve3::Circle {
                        center: p(0.0, 0.0, h),
                        axis: if t1 > t0 {
                            Vec3::unit_z()
                        } else {
                            -Vec3::unit_z()
                        },
                        radius: h.abs(),
                        u_ref: if t1 > t0 {
                            Vec3::unit_x()
                        } else {
                            v(t0.cos(), t0.sin(), 0.0)
                        },
                    },
                    param_start: if t1 > t0 { t0 } else { 0.0 },
                    param_end: if t1 > t0 { t1 } else { t0 - t1 },
                };
                (cone_at(h, t1), Some(spec))
            }
        },
        |_, _, end| EdgeCurveSpec::line_between(start, end),
    )
}

pub(super) fn contain_at(
    body: &Body<f64>,
    face: FaceKey,
    q: Point3<f64>,
) -> Option<FaceContainment> {
    crate::curved_face_containment(body, face, q, band()).unwrap()
}

/// **An L-shaped cone face refuses, never trims by its hull.** The face
/// covers `z < 1` over `[0, w]` and `z < 2` over `[w, 2w]`; the notch
/// `1 < z < 2` over `[0, w]` has the same hull, so a window read off the
/// hull answers `In` there. `bool_cone_chart_box` sees the notch (the
/// notch's inner sides lie inside its box) and the face door answers
/// `None`. Narrow (`w = π/8`, the nearest-branch walk's class) and wide
/// (`w = 3π/4`, the apex closure's), and one clear of the apex (the
/// notch cut from a frustum band).
#[test]
fn an_l_shaped_cone_face_refuses_rather_than_trim_by_its_hull() {
    let o = p(0.0, 0.0, 0.0);
    for w in [PI / 8.0, 0.75 * PI] {
        let (body, face) = cone_sheet(
            o,
            &[
                Step::Line(cone_at(1.0, 0.0)),
                Step::Arc(1.0, 0.0, w),
                Step::Line(cone_at(2.0, w)),
                Step::Arc(2.0, w, 2.0 * w),
            ],
        );
        assert_eq!(
            contain_at(&body, face, cone_at(1.5, 0.5 * w)),
            None,
            "w {w}: the notch"
        );
    }
    let (body, face) = cone_sheet(
        cone_at(0.5, 0.0),
        &[
            Step::Arc(0.5, 0.0, PI / 2.0),
            Step::Line(cone_at(2.0, PI / 2.0)),
            Step::Arc(2.0, PI / 2.0, PI / 4.0),
            Step::Line(cone_at(1.0, PI / 4.0)),
            Step::Arc(1.0, PI / 4.0, 0.0),
        ],
    );
    assert_eq!(
        contain_at(&body, face, cone_at(1.5, 0.1)),
        None,
        "the frustum L's notch"
    );
    // The same frustum band without its notch trims: the rectangle.
    let (body, face) = cone_sheet(
        cone_at(0.5, 0.0),
        &[
            Step::Arc(0.5, 0.0, PI / 2.0),
            Step::Line(cone_at(2.0, PI / 2.0)),
            Step::Arc(2.0, PI / 2.0, 0.0),
        ],
    );
    assert_eq!(
        contain_at(&body, face, cone_at(1.5, 0.1)),
        Some(FaceContainment::In),
        "the rectangle holds the point"
    );
    assert_eq!(
        contain_at(&body, face, cone_at(1.5, PI / 2.0 + 0.1)),
        Some(FaceContainment::Out),
        "the rectangle trims past its window"
    );
}

/// The face door's answer at `q`, an escalation included: the
/// small-notch rows require a definite refusal, not an in-band one.
fn door(
    body: &Body<f64>,
    face: FaceKey,
    q: Point3<f64>,
) -> Result<Option<FaceContainment>, crate::ContainError> {
    crate::curved_face_containment(body, face, q, band())
}

/// The notch sizes the small-notch rows cut, in metres: three decades
/// up from `10·K·ε`, a notch the band reads definitely, plus the sizes
/// the review measured wherever they clear that floor. The run's ε
/// picks the row.
fn notch_sizes() -> Vec<f64> {
    let tol = Tol::witness();
    let floor = 10.0 * tol.k() * tol.eps();
    let mut sizes: Vec<f64> = [1.0, 10.0, 100.0].iter().map(|k| k * floor).collect();
    for s in [1e-6, 1e-5, 1e-4] {
        if s >= floor && sizes.iter().all(|&t| (t / s).log10().abs() > 0.1) {
            sizes.push(s);
        }
    }
    sizes
}

/// The point at chart `(u, t)` on the ring torus `(R, r)` about `z`:
/// major angle `u` from `x`, minor angle `t` from the outer equator
/// towards `+z`.
fn torus_at((major, minor): (f64, f64), (u, t): (f64, f64)) -> Point3<f64> {
    let rho = major + minor * t.cos();
    p(rho * u.cos(), rho * u.sin(), minor * t.sin())
}

/// The iso arc of the ring torus `(R, r)` from chart point `a` to chart
/// point `b`, which share a coordinate: a parallel (a horizontal
/// section) when they share `t`, a meridian (an axial section) when they
/// share `u`. Either runs forward from `a` on the carrier whose `u_ref`
/// points at `a`.
fn torus_iso(
    body: &mut Body<f64>,
    torus: crate::geometry::SurfaceKey,
    (major, minor): (f64, f64),
    a: (f64, f64),
    b: (f64, f64),
) -> EdgeCurveSpec<f64> {
    let mid = (0.5 * (a.0 + b.0), 0.5 * (a.1 + b.1));
    let (center, axis, radius, u_ref, sweep, cut) = if a.1 == b.1 {
        let h = minor * a.1.sin();
        let axis = if b.0 > a.0 {
            Vec3::unit_z()
        } else {
            -Vec3::unit_z()
        };
        (
            p(0.0, 0.0, h),
            axis,
            major + minor * a.1.cos(),
            v(a.0.cos(), a.0.sin(), 0.0),
            (b.0 - a.0).abs(),
            plane(p(0.0, 0.0, h), Vec3::unit_z()),
        )
    } else {
        assert_eq!(a.0, b.0, "an iso arc shares a coordinate");
        let r_hat = v(a.0.cos(), a.0.sin(), 0.0);
        let n = r_hat.cross(Vec3::unit_z());
        (
            p(0.0, 0.0, 0.0) + r_hat * major,
            if b.1 > a.1 { n } else { -n },
            minor,
            r_hat * a.1.cos() + Vec3::unit_z() * a.1.sin(),
            (b.1 - a.1).abs(),
            plane(p(0.0, 0.0, 0.0), n),
        )
    };
    let cut = body.add_surface(cut);
    EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            s1: torus,
            s2: cut,
            witness: torus_at((major, minor), mid),
        },
        carrier: Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        },
        param_start: 0.0,
        param_end: sweep,
    }
}

/// **One face of the ring torus `(R, r)` bounded by iso arcs through
/// the chart corners**, in order ([`chain_sheet`]): the torus face a
/// boolean mints where parallels and meridians cut the tube.
fn torus_sheet(ring: (f64, f64), corners: &[(f64, f64)]) -> (Body<f64>, FaceKey) {
    chain_sheet(
        torus(p(0.0, 0.0, 0.0), ring.0, ring.1),
        torus_at(ring, corners[0]),
        corners.len() - 1,
        |body, key, i| {
            let spec = torus_iso(body, key, ring, corners[i], corners[i + 1]);
            (torus_at(ring, corners[i + 1]), Some(spec))
        },
        |body, key, _| torus_iso(body, key, ring, corners[0], corners[corners.len() - 1]),
    )
}

/// **An L-shaped torus face refuses, never trims by its hull.** The face
/// covers `t ∈ [−a, 0]` over `u ∈ [0, a]` and `t ∈ [−a, a]` over
/// `u ∈ [a, 2a]`; the notch `t ∈ (0, a)` over `u ∈ (0, a)` has the same
/// hull, and the L, monotone in both channels, the same total variation.
/// `bool_torus_chart_box` sees the notch's sides inside the box and the
/// face door answers `None` for the whole face. The U, notched from the
/// middle of its top side, refuses too; the rectangle trims.
#[test]
fn an_l_shaped_torus_face_refuses_rather_than_trim_by_its_hull() {
    let ring = (2.0, 0.5);
    let a = PI / 4.0;
    let l = [
        (0.0, -a),
        (2.0 * a, -a),
        (2.0 * a, a),
        (a, a),
        (a, 0.0),
        (0.0, 0.0),
    ];
    let notch = torus_at(ring, (0.5 * a, 0.5 * a));
    let (body, face) = torus_sheet(ring, &l);
    assert_eq!(contain_at(&body, face, notch), None, "the L's notch");
    assert_eq!(
        contain_at(&body, face, torus_at(ring, (1.5 * a, 0.5 * a))),
        None,
        "the L refuses as a face, its arm too"
    );
    let u = [
        (0.0, -a),
        (3.0 * a, -a),
        (3.0 * a, a),
        (2.0 * a, a),
        (2.0 * a, 0.0),
        (a, 0.0),
        (a, a),
        (0.0, a),
    ];
    let (body, face) = torus_sheet(ring, &u);
    assert_eq!(
        contain_at(&body, face, torus_at(ring, (1.5 * a, 0.5 * a))),
        None,
        "the U's notch"
    );
    let (body, face) = torus_sheet(ring, &[(0.0, -a), (2.0 * a, -a), (2.0 * a, a), (0.0, a)]);
    assert_eq!(
        contain_at(&body, face, notch),
        Some(FaceContainment::In),
        "the rectangle holds the point"
    );
    assert_eq!(
        contain_at(&body, face, torus_at(ring, (2.5 * a, 0.5 * a))),
        Some(FaceContainment::Out),
        "the rectangle trims past its window"
    );
}

/// The ring tori the small-notch rows cut: `(R, r)`, a fat and a thin
/// tube, a near-horn one and a wide one.
const RINGS: [(f64, f64); 4] = [(2.0, 0.5), (10.0, 0.1), (1.0, 0.9), (100.0, 1.0)];

/// The chart rectangle `[0, π/2] × [−π/4, π/4]` less a `du × dv` notch
/// cut from its top-right corner (`corner`, an L) or from the middle of
/// its top side (a U), and the notch's centre.
fn notched_chart(du: f64, dv: f64, corner: bool) -> (Vec<(f64, f64)>, (f64, f64)) {
    let (w, a) = (PI / 2.0, PI / 4.0);
    if corner {
        (
            vec![
                (0.0, -a),
                (w, -a),
                (w, a - dv),
                (w - du, a - dv),
                (w - du, a),
                (0.0, a),
            ],
            (w - 0.5 * du, a - 0.5 * dv),
        )
    } else {
        let m = 0.5 * w;
        (
            vec![
                (0.0, -a),
                (w, -a),
                (w, a),
                (m + 0.5 * du, a),
                (m + 0.5 * du, a - dv),
                (m - 0.5 * du, a - dv),
                (m - 0.5 * du, a),
                (0.0, a),
            ],
            (m, a - 0.5 * dv),
        )
    }
}

/// **A small notch refuses at every size the band reads, on every
/// ring.** Each notch is `s` metres on a side (`s/ρ` of major angle at
/// its own parallel's radius `ρ`, `s/r` of minor angle), cut as an L
/// and as a U from `[0, π/2] × [−π/4, π/4]`, and each is asked at its
/// centre, which is `s/2` from the face: the answer is a definite
/// refusal, never `In` (the hull's answer) and never an escalation (a
/// margin that shrinks faster than the notch). Two thin notches ride
/// along, `s` by `π/8` each way.
#[test]
fn a_small_notch_in_a_torus_face_refuses_at_every_size() {
    let eps = Tol::witness().eps();
    let mut bad = Vec::new();
    let a = PI / 4.0;
    for ring in RINGS {
        let rho = ring.0 + ring.1 * a.cos();
        for s in notch_sizes() {
            let (du, dv) = (s / rho, s / ring.1);
            for (what, du, dv, corner) in [
                ("L", du, dv, true),
                ("U", du, dv, false),
                ("thin deep L", du, PI / 8.0, true),
                ("thin wide L", PI / 8.0, dv, true),
            ] {
                let (corners, centre) = notched_chart(du, dv, corner);
                let (body, face) = torus_sheet(ring, &corners);
                let got = door(&body, face, torus_at(ring, centre));
                if !matches!(got, Ok(None)) {
                    bad.push(format!("ring {ring:?}, {what} notch {s:e} m: {got:?}"));
                }
            }
        }
    }
    assert!(
        bad.is_empty(),
        "ε {eps:e}: {} cells answer other than a refusal:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// **The rectangle still trims on every ring**: `In` at its centre,
/// `Out` a tenth of a radian past its major window.
#[test]
fn a_torus_rectangle_trims_on_every_ring() {
    let a = PI / 4.0;
    for ring in RINGS {
        let (body, face) = torus_sheet(ring, &[(0.0, -a), (2.0 * a, -a), (2.0 * a, a), (0.0, a)]);
        assert_eq!(
            door(&body, face, torus_at(ring, (a, 0.0))).unwrap(),
            Some(FaceContainment::In),
            "ring {ring:?}: the centre"
        );
        assert_eq!(
            door(&body, face, torus_at(ring, (2.0 * a + 0.1, 0.0))).unwrap(),
            Some(FaceContainment::Out),
            "ring {ring:?}: past the window"
        );
    }
}

/// **A gap anywhere in the torus walk refuses.** The rectangle
/// `[−π/2, 0] × [−π/4, π/4]` windows; with any one of its four edges
/// made null scaffolding (which the walk steps over), the other three
/// still lie on the box's sides, so only the continuity check (an edge
/// in the middle of the walk) or the closure check (the walk's first or
/// last) can see the gap. Each answers `PartialTorusFace`.
#[test]
fn a_gap_in_the_torus_walk_refuses() {
    use crate::null::{CurveGeom, NullEdge};
    let ring = (2.0, 0.5);
    let (w, a) = (PI / 2.0, PI / 4.0);
    let (body, face) = torus_sheet(ring, &[(-w, -a), (0.0, -a), (0.0, a), (-w, a)]);
    let windows = |body: &Body<f64>| {
        super::super::solid_contain::torus_face_windows(body, face, ring.0, ring.1, band())
    };
    assert!(windows(&body).is_ok(), "the whole rectangle windows");
    let crate::LoopBoundary::Cycle { first } = body
        .get_loop(body.get_face(face).unwrap().outer)
        .unwrap()
        .boundary
    else {
        panic!("a cycle")
    };
    let cycle = body.loop_cycle(first).unwrap();
    assert_eq!(cycle.len(), 4, "four edges");
    let mut bad = Vec::new();
    for (i, &he) in cycle.iter().enumerate() {
        let mut gapped = body.clone();
        let edge = gapped
            .get_edge(gapped.get_half_edge(he).unwrap().edge)
            .unwrap();
        let (curve, he_plus, he_minus) = (edge.curve, edge.he_plus, edge.he_minus);
        let ends = (
            gapped.get_half_edge(he_plus).unwrap().start,
            gapped.get_half_edge(he_minus).unwrap().start,
        );
        gapped.curves[curve] = CurveGeom::NullScaffold(NullEdge {
            below_end: ends.0,
            above_end: ends.1,
        });
        let got = windows(&gapped);
        if !matches!(
            got,
            Err(super::super::solid_contain::PointInSolidError::PartialTorusFace { .. })
        ) {
            bad.push(format!("edge {i} of the walk skipped: {got:?}"));
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The frusta the small-notch cone rows cut: `(h0, h1)` on the unit
/// cone.
const FRUSTA: [(f64, f64); 2] = [(1.0, 2.0), (10.0, 20.0)];

/// **A small notch in a cone face refuses at every size the band
/// reads.** Each notch is `s` metres on a side (`s/√2` of height on the
/// unit cone, whose slant is `h·√2`; `s/h1` of azimuth at the top
/// rim), cut from the frustum `[h0, h1] × [0, π/4]` as an L at its
/// top-right corner and as a U from the middle of its top rim, and from
/// the apex sector `[0, h1] × [0, π/4]` as an L, which the apex closure
/// walks. Each is asked at the notch's centre: a definite refusal,
/// never `In`, never an escalation.
#[test]
fn a_small_notch_in_a_cone_face_refuses_at_every_size() {
    let eps = Tol::witness().eps();
    let mut bad = Vec::new();
    let w = PI / 4.0;
    for (h0, h1) in FRUSTA {
        for s in notch_sizes() {
            let (dh, dt) = (s / 2f64.sqrt(), s / h1);
            let centre = cone_at(h1 - 0.5 * dh, w - 0.5 * dt);
            let l = cone_sheet(
                cone_at(h0, 0.0),
                &[
                    Step::Arc(h0, 0.0, w),
                    Step::Line(cone_at(h1 - dh, w)),
                    Step::Arc(h1 - dh, w, w - dt),
                    Step::Line(cone_at(h1, w - dt)),
                    Step::Arc(h1, w - dt, 0.0),
                ],
            );
            let m = 0.5 * w;
            let u = cone_sheet(
                cone_at(h0, 0.0),
                &[
                    Step::Arc(h0, 0.0, w),
                    Step::Line(cone_at(h1, w)),
                    Step::Arc(h1, w, m + 0.5 * dt),
                    Step::Line(cone_at(h1 - dh, m + 0.5 * dt)),
                    Step::Arc(h1 - dh, m + 0.5 * dt, m - 0.5 * dt),
                    Step::Line(cone_at(h1, m - 0.5 * dt)),
                    Step::Arc(h1, m - 0.5 * dt, 0.0),
                ],
            );
            let apex = cone_sheet(
                p(0.0, 0.0, 0.0),
                &[
                    Step::Line(cone_at(h1, 0.0)),
                    Step::Arc(h1, 0.0, w - dt),
                    Step::Line(cone_at(h1 - dh, w - dt)),
                    Step::Arc(h1 - dh, w - dt, w),
                ],
            );
            for (what, (body, face), q) in [
                ("L", l, centre),
                ("U", u, cone_at(h1 - 0.5 * dh, m)),
                ("apex L", apex, centre),
            ] {
                let got = door(&body, face, q);
                if !matches!(got, Ok(None)) {
                    bad.push(format!("frustum {h0}..{h1}, {what} notch {s:e} m: {got:?}"));
                }
            }
        }
        let (body, face) = cone_sheet(
            cone_at(h0, 0.0),
            &[
                Step::Arc(h0, 0.0, w),
                Step::Line(cone_at(h1, w)),
                Step::Arc(h1, w, 0.0),
            ],
        );
        let mid = 0.5 * (h0 + h1);
        assert_eq!(
            door(&body, face, cone_at(mid, 0.5 * w)).unwrap(),
            Some(FaceContainment::In),
            "frustum {h0}..{h1}: the rectangle holds its centre"
        );
        assert_eq!(
            door(&body, face, cone_at(mid, w + 0.1)).unwrap(),
            Some(FaceContainment::Out),
            "frustum {h0}..{h1}: the rectangle trims past its window"
        );
    }
    assert!(
        bad.is_empty(),
        "ε {eps:e}: {} cells answer other than a refusal:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

/// **The smallest notch the builders mint pins the arms' direction.**
/// A notch `1.5·K·ε` on a side, where each arm is the exact rate: on the
/// torus at the outer equator (a parallel's radius there is `R + r`, the
/// major arm itself; a meridian's is `r`), and on the cone at its top
/// rim (the azimuth arm is that rim's radius; slant is exact). The
/// notch's inner sides are then `1.5·K·ε` from the box's in metres, past
/// the band, so the TRIM refuses definitely (`PartialTorusFace`,
/// `PartialConeFace`). An arm that understated its rate would bring that
/// margin into the band (an escalation) or under it (a window), and
/// this row reads either as a failure. The face door at the notch's
/// centre, `0.75·K·ε` from the face, may escalate on that graze before
/// it reaches the trim; it must never answer `In`.
#[test]
fn the_smallest_notch_refuses_definitely() {
    use super::super::solid_contain::{PointInSolidError, cone_face_trim, torus_face_windows};
    let tol = Tol::witness();
    let s = 1.5 * tol.k() * tol.eps();
    let mut bad = Vec::new();
    let mut door_reads =
        |what: String, got: Result<Option<FaceContainment>, crate::ContainError>| {
            if matches!(got, Ok(Some(FaceContainment::In))) {
                bad.push(format!("{what}: the door answers In"));
            }
        };
    let mut trims = Vec::new();
    for ring in RINGS {
        let (du, dv) = (s / (ring.0 + ring.1), s / ring.1);
        let (w, a) = (PI / 2.0, PI / 4.0);
        let corners = [
            (0.0, -a),
            (w, -a),
            (w, -dv),
            (w - du, -dv),
            (w - du, 0.0),
            (0.0, 0.0),
        ];
        let (body, face) = torus_sheet(ring, &corners);
        door_reads(
            format!("ring {ring:?}"),
            door(&body, face, torus_at(ring, (w - 0.5 * du, -0.5 * dv))),
        );
        let got = torus_face_windows(&body, face, ring.0, ring.1, band());
        if !matches!(got, Err(PointInSolidError::PartialTorusFace { .. })) {
            trims.push(format!("ring {ring:?}: the trim answers {got:?}"));
        }
    }
    let w = PI / 4.0;
    for (h0, h1) in FRUSTA {
        let (dh, dt) = (s / 2f64.sqrt(), s / h1);
        let (body, face) = cone_sheet(
            cone_at(h0, 0.0),
            &[
                Step::Arc(h0, 0.0, w),
                Step::Line(cone_at(h1 - dh, w)),
                Step::Arc(h1 - dh, w, w - dt),
                Step::Line(cone_at(h1, w - dt)),
                Step::Arc(h1, w - dt, 0.0),
            ],
        );
        door_reads(
            format!("frustum {h0}..{h1}"),
            door(&body, face, cone_at(h1 - 0.5 * dh, w - 0.5 * dt)),
        );
        let got = cone_face_trim(
            &body,
            face,
            p(0.0, 0.0, 0.0),
            Vec3::unit_z(),
            PI / 4.0,
            band(),
        );
        if !matches!(got, Err(PointInSolidError::PartialConeFace { .. })) {
            trims.push(format!("frustum {h0}..{h1}: the trim answers {got:?}"));
        }
    }
    bad.extend(trims);
    assert!(
        bad.is_empty(),
        "notch {s:e} m at ε {:e}:\n{}",
        tol.eps(),
        bad.join("\n")
    );
}

/// **A ringed apex-closed face has no single lift.** The quarter sector
/// with a lone-vertex ring closes `Open`; without the ring it closes.
#[test]
fn a_ringed_apex_face_does_not_close() {
    use crate::chord_join::{ApexClosure, cone_apex_closure};
    let (mut body, face) = cone_sheet(
        p(0.0, 0.0, 0.0),
        &[Step::Line(cone_at(1.0, 0.0)), Step::Arc(1.0, 0.0, PI / 2.0)],
    );
    let cone = unit_cone();
    assert!(matches!(
        cone_apex_closure(&body, &cone, face, band()).unwrap(),
        ApexClosure::Closed { .. }
    ));
    let crate::LoopBoundary::Cycle { first } = body
        .get_loop(body.get_face(face).unwrap().outer)
        .unwrap()
        .boundary
    else {
        panic!("a cycle")
    };
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: first,
                he2: first,
            },
            cone_at(0.5, PI / 4.0),
            Tol::witness(),
        )
        .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    assert!(
        !body.get_face(face).unwrap().rings.is_empty(),
        "the kill leaves a ring"
    );
    assert!(matches!(
        cone_apex_closure(&body, &cone, face, band()).unwrap(),
        ApexClosure::Open
    ));
    assert!(!ops::ChartCache::default().describes(Operand::A, &body, face, &cone, band()));
}

/// **An apex face reaching both nappes has no single lift.** Its
/// outline leaves the apex on the upper nappe, crosses to the mirror
/// nappe along one straight line through the apex (no vertex there), and
/// returns: one apex visit, vertices on both nappes. It closes `Open`
/// and does not describe.
#[test]
fn an_apex_face_on_both_nappes_does_not_close() {
    use crate::chord_join::{ApexClosure, cone_apex_closure};
    let (body, face) = cone_sheet(
        p(0.0, 0.0, 0.0),
        &[
            Step::Line(cone_at(1.0, 0.0)),
            Step::Arc(1.0, 0.0, PI / 4.0),
            Step::Line(cone_at(-1.0, 1.25 * PI)),
            Step::Arc(-1.0, 1.25 * PI, 1.5 * PI),
        ],
    );
    let cone = unit_cone();
    assert!(matches!(
        cone_apex_closure(&body, &cone, face, band()).unwrap(),
        ApexClosure::Open
    ));
    assert!(!ops::ChartCache::default().describes(Operand::A, &body, face, &cone, band()));
}

/// **A face wound past a full turn neither describes nor trims.** Its
/// rim runs `[0, 5π/2]` in two arcs, so the closure lifts it to a window
/// wider than a period: `describes` refuses it (`bool_cone_closure_period`)
/// and the face door answers `None`.
#[test]
fn an_apex_face_wound_past_a_turn_neither_describes_nor_trims() {
    use crate::chord_join::{ApexClosure, cone_apex_closure};
    let (body, face) = cone_sheet(
        p(0.0, 0.0, 0.0),
        &[
            Step::Line(cone_at(1.0, 0.0)),
            Step::Arc(1.0, 0.0, 1.5 * PI),
            Step::Arc(1.0, 1.5 * PI, 2.5 * PI),
        ],
    );
    let cone = unit_cone();
    let ApexClosure::Closed { window, .. } = cone_apex_closure(&body, &cone, face, band()).unwrap()
    else {
        panic!("one apex visit, no ring: the closure lifts it");
    };
    assert!(window.1 - window.0 > TAU, "{window:?}");
    assert!(!ops::ChartCache::default().describes(Operand::A, &body, face, &cone, band()));
    assert_eq!(contain_at(&body, face, cone_at(0.5, 0.25 * PI)), None);
}

// -------------------------------------------------------------------
// Torus touches against a sampling oracle
// -------------------------------------------------------------------

/// A ring torus about any axis, with the frame its sampling reads.
#[derive(Clone, Copy)]
struct Tor {
    c: Point3<f64>,
    e1: Vec3<f64>,
    e2: Vec3<f64>,
    a: Vec3<f64>,
    big_r: f64,
    r: f64,
}

impl Tor {
    fn new(c: Point3<f64>, a: Vec3<f64>, big_r: f64, r: f64) -> Self {
        let a = a.normalize();
        let (e1, e2) = a.orthonormal_basis();
        Self {
            c,
            e1,
            e2,
            a,
            big_r,
            r,
        }
    }

    fn at(&self, u: f64, v: f64) -> Point3<f64> {
        let rho = self.big_r + self.r * v.cos();
        self.c + (self.e1 * u.cos() + self.e2 * u.sin()) * rho + self.a * (self.r * v.sin())
    }

    fn normal(&self, u: f64, v: f64) -> Vec3<f64> {
        (self.e1 * u.cos() + self.e2 * u.sin()) * v.cos() + self.a * v.sin()
    }

    fn uv(&self, x: Point3<f64>) -> (f64, f64) {
        let w = x - self.c;
        let (px, py, z) = (w.dot(self.e1), w.dot(self.e2), w.dot(self.a));
        (py.atan2(px), z.atan2(px.hypot(py) - self.big_r))
    }

    /// The distance from `x` to the torus.
    fn off(&self, x: Point3<f64>) -> f64 {
        let (u, v) = self.uv(x);
        (self.at(u, v) - x).norm()
    }

    fn surface(&self) -> Surface<f64> {
        Surface::Torus {
            center: self.c,
            axis: self.a,
            major_radius: self.big_r,
            minor_radius: self.r,
            u_ref: self.e1,
        }
    }
}

/// A partner of the torus arms, with its signed level `g`: the plane's
/// height, or the distance from the sphere's centre or the wall's axis
/// less the radius.
#[derive(Clone, Copy, Debug)]
enum Partner {
    Plane(Point3<f64>, Vec3<f64>),
    Sphere(Point3<f64>, f64),
    Wall(Point3<f64>, Vec3<f64>, f64),
}

impl Partner {
    fn g(&self, x: Point3<f64>) -> f64 {
        match *self {
            Self::Plane(o, n) => n.dot(x - o),
            Self::Sphere(c, rho) => (x - c).norm() - rho,
            Self::Wall(o, d, rc) => {
                let w = x - o;
                (w - d * w.dot(d)).norm() - rc
            }
        }
    }

    fn surface(&self) -> Surface<f64> {
        match *self {
            Self::Plane(o, n) => plane(o, n),
            Self::Sphere(c, rho) => sphere(c, rho),
            Self::Wall(o, d, rc) => cylinder(o, d, rc),
        }
    }
}

/// A pattern-search descent of `f` from `(u, v)` at step `h`.
fn descend(f: &impl Fn(f64, f64) -> f64, mut u: f64, mut v: f64, mut h: f64) -> (f64, f64, f64) {
    let mut best = f(u, v);
    while h > 1e-15 {
        let step = [
            (h, 0.0),
            (-h, 0.0),
            (0.0, h),
            (0.0, -h),
            (h, h),
            (-h, -h),
            (h, -h),
            (-h, h),
        ]
        .into_iter()
        .map(|(du, dv)| (u + du, v + dv))
        .find(|&(x, y)| f(x, y) < best);
        match step {
            Some((x, y)) => {
                (u, v, best) = (x, y, f(x, y));
            }
            None => h *= 0.5,
        }
    }
    (u, v, best)
}

/// **The oracle on one touch**, from the torus and the partner alone:
/// `at` stands on both carriers within `k`, on the tube's outer half,
/// and on one side of the partner `g` has no other local minimum at or
/// below `k` over the torus (refined from an `n × n/2` grid) that a
/// barrier above `k` parts from `at`, and none deeper near `at` — the
/// partner touches there and nowhere else. A minimum the straight path
/// in `(u, v)` joins to `at` below `k` and `g`'s own rounding is the
/// touch's region, not a second one: near the top parallel the touch's
/// region is long and flat below the rounding at ×1e3, where descent
/// stalls short of `at`.
fn touch_holds(
    t: &Tor,
    partner: &Partner,
    at: Point3<f64>,
    k: f64,
    n: usize,
) -> Result<(), String> {
    let scale = t.big_r + t.r;
    let slack = k + 1e-12 * scale;
    let (off_t, off_p) = (t.off(at), partner.g(at).abs());
    if off_t > slack || off_p > slack {
        return Err(format!(
            "at off a carrier: torus {off_t:e}, partner {off_p:e}"
        ));
    }
    let (ua, va) = t.uv(at);
    if va.cos() <= 0.0 {
        return Err(format!("at on the hyperbolic half: v = {va}"));
    }
    let noise = 64.0 * f64::EPSILON * ((t.c - Point3::origin()).norm() + scale);
    let (nu, nv) = (n, n / 2);
    let (du, dv) = (TAU / nu as f64, TAU / nv as f64);
    let mut why = Vec::new();
    for side in [1.0, -1.0] {
        let f = |u: f64, v: f64| side * partner.g(t.at(u, v));
        let (_, _, here) = descend(&f, ua, va, 1e-3);
        if here < -k {
            why.push(format!("side {side}: {here:e} below the touch"));
            continue;
        }
        let grid: Vec<f64> = (0..nu * nv)
            .map(|i| f((i % nu) as f64 * du, (i / nu) as f64 * dv))
            .collect();
        let at_grid = |i: usize, j: usize| grid[(j % nv) * nu + i % nu];
        let other = (0..nv)
            .flat_map(|j| (0..nu).map(move |i| (i, j)))
            .filter(|&(i, j)| {
                let y = at_grid(i, j);
                [
                    (nu - 1, 0),
                    (1, 0),
                    (0, nv - 1),
                    (0, 1),
                    (1, 1),
                    (nu - 1, nv - 1),
                    (1, nv - 1),
                    (nu - 1, 1),
                ]
                .into_iter()
                .all(|(a, b)| at_grid(i + a, j + b) >= y)
            })
            .map(|(i, j)| descend(&f, i as f64 * du, j as f64 * dv, du))
            .find(|&(u, v, m)| {
                let du = (u - ua + PI).rem_euclid(TAU) - PI;
                let dv = (v - va + PI).rem_euclid(TAU) - PI;
                let parted = (1..200).any(|i| {
                    let w = f64::from(i) / 200.0;
                    f(ua + du * w, va + dv * w) > k + noise
                });
                m <= k && parted && (t.at(u, v) - at).norm() > 1e-4 * scale
            });
        match other {
            Some((u, v, m)) => why.push(format!(
                "side {side}: another minimum {m:e} at {:?}",
                t.at(u, v)
            )),
            None => return Ok(()),
        }
    }
    Err(why.join("; "))
}

/// **A plane touch near the top parallel stands on both carriers**
/// (`Touch::at`'s contract): a plane tangent to the donut at
/// `v = π/2 − t`, `t` from 1e-4 down to 1e-8 rad, about a generic axis,
/// at ×1 and ×1e3. Its `at` is read in the meridian half-plane through
/// the plane's normal, whose direction is that normal's small part
/// square to the axis. Mutant: that part taken as `n − a·(a·n)`, whose
/// component along the axis is `n`'s rounding over `s` and stands `at`
/// 8.9ε off each carrier at ×1e3 and 1e-5 rad, 1.2e4ε at 1e-8 rad.
#[test]
fn a_plane_touch_near_the_top_parallel_stands_on_both_carriers() {
    let band = band();
    let k = band.zero();
    for scale in [1.0, 1e3] {
        let mut touches = 0;
        for (u, axis) in [
            (0.9, v(0.0, 0.0, 1.0)),
            (0.9, v(1.0, 2.0, 3.0)),
            (2.3, v(-0.3, 0.2, 1.0)),
        ] {
            let tor = Tor::new(p(0.0, 0.0, 0.0), axis, 2.0 * scale, 0.5 * scale);
            for t in [1e-4, 1e-5, 1e-6, 1e-7, 1e-8] {
                let x = tor.at(u, PI / 2.0 - t);
                let pl = Partner::Plane(x, tor.normal(u, PI / 2.0 - t));
                let reach = Reach {
                    centre: tor.c,
                    radius: 5.0 * scale,
                };
                let label = format!("×{scale:e}, axis {axis:?}, u {u}, t {t:e}");
                match super::classify(&tor.surface(), &pl.surface(), reach, band) {
                    Section::Touch(touch) => {
                        touches += 1;
                        let (off_t, off_p) = (tor.off(touch.at), pl.g(touch.at).abs());
                        assert!(
                            off_t <= k && off_p <= k,
                            "{label}: at {:?} stands {off_t:e} off the torus, {off_p:e} off \
                             the plane (ε = {k:e})",
                            touch.at
                        );
                    }
                    Section::Tangent(_) => {}
                    other => panic!("{label}: neither a touch nor a pinch: {other:?}"),
                }
            }
        }
        assert!(touches > 0, "×{scale:e}: no touch decided");
    }
}

/// **A ball touching a wall from inside, nearly coaxial, stands its
/// touch on both carriers** (`Touch::at`'s contract): the ball of
/// radius `ρc − e` about a centre `e` off the wall's axis, `e` from 10ε
/// to 1e3ε, about 16 axes spread over the sphere, the wall's origin at
/// the ball or 1e3 along the axis, at ×1e-3, ×1 and ×1e3, both operand
/// orders. `at` is read on the ruling through the centre's part square
/// to the axis, of norm `e`: it stands within ε of each carrier, plus 8
/// ulps of the coordinates the distances are read in. Mutant: that part
/// taken as `cs − (o + d·(w·d))`, whose component along the axis is
/// `w`'s rounding over `e`: at ε = 1e-9 it stands `at` 1.2ε off the
/// wall at ×1e3, and at 1e-12 4.5e3ε off it at ×1e-3.
#[test]
fn a_ball_touching_a_wall_near_its_axis_stands_on_both_carriers() {
    let band = band();
    let eps = band.zero();
    for scale in [1e-3, 1.0, 1e3] {
        let rc = scale;
        let base = Point3::origin() + v(0.3, -0.2, 0.1) * scale;
        let reach = Reach {
            centre: base,
            radius: 2.0 * scale,
        };
        let mut touches = 0;
        for i in 0..16 {
            let z = 1.0 - (f64::from(i) + 0.5) / 8.0;
            let phi = 2.4 * f64::from(i);
            let s = (1.0 - z * z).sqrt();
            let d = v(s * phi.cos(), s * phi.sin(), z);
            let (b1, b2) = d.orthonormal_basis();
            let q = b1 * phi.cos() + b2 * phi.sin();
            for far in [0.0, 1e3] {
                let o = base - d * far;
                let wall = Partner::Wall(o, d, rc);
                let ulps = 8.0 * f64::EPSILON * (far + 2.0 * scale);
                for e in [10.0, 100.0, 1e3].map(|k| k * eps) {
                    let ball = Partner::Sphere(base + q * e, rc - e);
                    let label = format!("×{scale:e}, axis {i}, origin {far:e} along, e {e:e}");
                    for section in [
                        super::classify(&ball.surface(), &wall.surface(), reach, band),
                        super::classify(&wall.surface(), &ball.surface(), reach, band),
                    ] {
                        let Section::Touch(touch) = section else {
                            panic!("{label}: not a touch: {section:?}");
                        };
                        touches += 1;
                        let (off_b, off_w) = (ball.g(touch.at).abs(), wall.g(touch.at).abs());
                        assert!(
                            off_b <= eps + ulps && off_w <= eps + ulps,
                            "{label}: at {:?} stands {:.3}ε off the ball, {:.3}ε off the wall \
                             (8 ulps {:.3}ε)",
                            touch.at,
                            off_b / eps,
                            off_w / eps,
                            ulps / eps
                        );
                    }
                }
            }
        }
        assert_eq!(touches, 192, "×{scale:e}: every pose a touch");
    }
}

/// **Every torus touch holds against a sampling oracle** (a bounded cut
/// of the review's 900-torus fuzz): random ring tori, any axis, `r/R`
/// from 0.02 to 0.98, at ×1e-3, ×1 and ×1e3, against a plane, three
/// spheres (outside, inside the tube, about the torus) and three walls
/// parallel to the axis (beside, about, in the hole), each tangent at a
/// random point biased to the equators and the top and bottom parallels
/// and pushed by up to ±100ε; a push that would take a radius to zero
/// or below, off the carriers' convention, is not drawn. Every `Touch`
/// either operand order answers passes [`touch_holds`]. Mutants, each
/// red here: the plane's or the sphere's elliptic margin taken as `abs`
/// (a touch on the hyperbolic half), the sphere's extreme margin
/// dropped, and [`super::square_to`] as `v − a·(a·v)`.
#[test]
fn every_torus_touch_holds_against_a_sampling_oracle() {
    use super::super::conic_oracle::unit;
    use core::f64::consts::FRAC_PI_2;
    use test_utils::fuzz;
    let band = band();
    let eps = band.zero();
    let k = band.escalate();
    let mut rng = fuzz::start("section_cert::every_torus_touch_holds_against_a_sampling_oracle");
    let (mut touches, mut bad) = (0, Vec::new());
    for scale in [1e-3, 1.0, 1e3] {
        for case in 0..fuzz::scaled(60) {
            let big_r = rng.range(0.5, 3.0) * scale;
            let ratio = match rng.below(3) {
                0 => rng.range(0.02, 0.1),
                1 => rng.range(0.85, 0.98),
                _ => rng.range(0.1, 0.85),
            };
            let c = p(
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            );
            let t = Tor::new(
                Point3::origin() + (c - Point3::origin()) * scale,
                unit(&mut rng),
                big_r,
                ratio * big_r,
            );
            let u = rng.range(-3.2, 3.2);
            let jitter = [0.0, eps, 10.0 * eps, 1e-6, 1e-4, 1e-2][rng.below(6)];
            let jitter = if rng.below(2) == 0 { jitter } else { -jitter };
            let vv = match rng.below(6) {
                0 => jitter,
                1 => FRAC_PI_2 + jitter,
                2 => -FRAC_PI_2 + jitter,
                3 => PI + jitter,
                _ => rng.range(-3.2, 3.2),
            };
            let delta = [0.0, 1.0, -1.0, 10.0, -10.0, 100.0, -100.0, 0.5, -0.5][rng.below(9)] * eps;
            let (x, nrm) = (t.at(u, vv), t.normal(u, vv));
            let mut partners = vec![Partner::Plane(x + nrm * delta, nrm)];
            for (side, rho) in [
                (1.0, rng.range(0.05, 3.0) * scale),
                (-1.0, rng.range(0.05, 0.99) * t.r),
                (-1.0, rng.range(1.0, 4.0) * (t.big_r + t.r)),
            ] {
                if rho + delta > 0.0 {
                    partners.push(Partner::Sphere(x + nrm * (side * rho), rho + delta));
                }
            }
            for v_eq in [0.0, PI] {
                let (x, nrm) = (t.at(u, v_eq), t.normal(u, v_eq));
                for (side, rc) in [
                    (1.0, rng.range(0.05, 2.0) * scale),
                    (-1.0, rng.range(1.0, 3.0) * (t.big_r + t.r)),
                    (-1.0, rng.range(0.05, 0.99) * (t.big_r - t.r)),
                ] {
                    let o = x + nrm * (side * rc) + t.a * (rng.range(-1.0, 1.0) * scale);
                    if rc + delta > 0.0 {
                        partners.push(Partner::Wall(o, t.a, rc + delta));
                    }
                }
            }
            let reach = Reach {
                centre: t.c,
                radius: 2.0 * (t.big_r + t.r),
            };
            for partner in partners {
                for section in [
                    super::classify(&t.surface(), &partner.surface(), reach, band),
                    super::classify(&partner.surface(), &t.surface(), reach, band),
                ] {
                    if let Section::Touch(touch) = section {
                        touches += 1;
                        if let Err(why) = touch_holds(&t, &partner, touch.at, k, 120) {
                            bad.push(format!(
                                "×{scale:e} case {case} v {vv} δ {delta:e} {}: {why} :: {partner:?}",
                                touch.name
                            ));
                        }
                    }
                }
            }
        }
    }
    assert!(touches > 0, "no touch drawn: {}", fuzz::replay());
    assert!(
        bad.is_empty(),
        "{} of {touches} touches fail:\n{}\n{}",
        bad.len(),
        bad.join("\n"),
        fuzz::replay()
    );
}
