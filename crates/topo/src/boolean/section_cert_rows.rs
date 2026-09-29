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

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

const LEVER: f64 = 5.0;

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
    let normal = normal.normalize();
    Surface::Plane {
        origin,
        normal,
        u_ref: normal.orthonormal_basis().0,
    }
}

fn cylinder(origin: Point3<f64>, axis: Vec3<f64>, radius: f64) -> Surface<f64> {
    let axis = axis.normalize();
    Surface::Cylinder {
        origin,
        axis,
        radius,
        u_ref: axis.orthonormal_basis().0,
    }
}

fn sphere(center: Point3<f64>, radius: f64) -> Surface<f64> {
    Surface::Sphere {
        center,
        radius,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

fn torus(center: Point3<f64>, major: f64, minor: f64) -> Surface<f64> {
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
fn classify(f: &Surface<f64>, g: &Surface<f64>) -> Section<f64> {
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
type Shape = (usize, bool, Vec<bool>, Vec<bool>, Vec<bool>);

fn shape(s: &Section<f64>) -> Shape {
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
fn witness(s: &Section<f64>) -> Point3<f64> {
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
fn on_both(w: Point3<f64>, f: &Surface<f64>, g: &Surface<f64>) {
    for s in [f, g] {
        let r = geom_brep::implicit_residual(s, w);
        assert!(r.abs() < 1e-12, "witness {w:?} is {r} off {s:?}");
    }
}

fn tangent(s: &Section<f64>) -> &'static str {
    match s {
        Section::Tangent(name) => name,
        other => panic!("not a tangency: {other:?}"),
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

/// **Tangency refuses (R-tan)**: a plane on the outer equator and a
/// plane on the top circle. Read as `Positive`, the first would be a
/// scrape and the second two parallels.
#[test]
fn a_plane_tangent_to_the_tube_refuses_as_a_tangency() {
    let s = classify(&donut(), &plane(p(2.5, 0.0, 0.0), v(1.0, 0.0, 0.0)));
    assert_eq!(tangent(&s), "section_torus_plane_near_tube");
    let s = classify(&donut(), &plane(p(0.0, 0.0, 0.5), v(0.0, 0.0, 1.0)));
    assert!(tangent(&s).starts_with("section_torus_plane_"));
}

/// A length strictly inside the run's band sliver — past `zero`, short
/// of `escalate` — where a margin decides neither way, at whatever
/// eps the run carries.
fn in_sliver() -> f64 {
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
    // A ball tangent to the tube: R-tan.
    let s = classify(&donut(), &sphere(p(3.0, 0.0, 0.0), 0.5));
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
/// a non-coaxial torus, and any cone.
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
    assert!(matches!(
        classify(&cone, &plane(p(0.0, 0.0, 1.0), v(0.1, 0.0, 1.0))),
        Section::Intractable
    ));
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
    // The tangent pose |δ₀| = r₁ + r₂.
    let s = classify(&a, &cylinder(p(0.0, 1.5, 0.0), Vec3::unit_x(), 0.5));
    assert_eq!(tangent(&s), "section_cylinder_pair_reach");
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

fn at(
    f: Option<FaceContainment>,
    g: Option<FaceContainment>,
) -> impl FnMut(Point3<f64>) -> [Option<FaceContainment>; 2] {
    move |_| [f, g]
}

const IN: Option<FaceContainment> = Some(FaceContainment::In);
const OUT: Option<FaceContainment> = Some(FaceContainment::Out);

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
/// `cyl` and the rim plane there.
fn full_rim(body: &mut Body<f64>, cyl: crate::geometry::SurfaceKey, z: f64) -> EdgeCurveSpec<f64> {
    let rim_plane = body.add_surface(Surface::Plane {
        origin: p(0.0, 0.0, z),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    });
    EdgeCurveSpec {
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
    }
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
    let seed = body.mvfs(frame.at(0.0, z0)).unwrap();
    let cyl = body
        .set_face_surface(seed.face, FaceSurface::New(frame.surface()))
        .unwrap();
    let bottom = full_rim(&mut body, cyl, z0);
    let cap_b = body
        .mef(
            MefSite::Lone {
                r#loop: seed.r#loop,
            },
            bottom,
            FaceSurface::New(plane(p(0.0, 0.0, z0), v(0.0, 0.0, -1.0))),
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
    let top = full_rim(&mut body, cyl, z1);
    body.mef(
        MefSite::Chords {
            he1: strut.he_minus,
            he2: strut.he_minus,
        },
        top,
        FaceSurface::New(plane(p(0.0, 0.0, z1), v(0.0, 0.0, 1.0))),
        tol,
    )
    .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    (body, seed.face)
}

/// The verdict of every pair of `a`'s `face` against `b`'s faces on the
/// crossings path, with no events.
fn scan(a: &Body<f64>, face: FaceKey, b: &Body<f64>) -> Vec<Result<Vec<Cleared>, Refusal>> {
    ops::section_pairs(
        a,
        b,
        band(),
        ops::SectionPath::Crossings,
        |_, _| false,
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
fn tilted_slab() -> Body<f64> {
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
/// without the ring clears.
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
fn classify_near(
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
/// clear W0 with a true loop in it.
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
fn scan_b(a: &Body<f64>, b: &Body<f64>, face: FaceKey) -> Vec<Result<Vec<Cleared>, Refusal>> {
    ops::section_pairs(
        a,
        b,
        band(),
        ops::SectionPath::Crossings,
        |_, _| false,
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

/// **A lone-vertex ring on operand B's face refuses too.**
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
            class: crate::contact::ContactClass::Rest,
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
fn unit_cone() -> Surface<f64> {
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
    let seed = body.mvfs(p(0.0, 0.0, 0.0)).unwrap();
    let cone = body
        .set_face_surface(seed.face, FaceSurface::New(unit_cone()))
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
            FaceSurface::Shared(cone),
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
        FaceSurface::Shared(cone),
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
            (crate::readback::vertex_point_ref(&body, v).unwrap() - p(0.0, 0.0, 0.0)).norm() == 0.0
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
enum Step {
    /// A straight edge to the point.
    Line(Point3<f64>),
    /// A rim arc at height `h` (radius `|h|`), azimuth `t0` to `t1`,
    /// either way round.
    Arc(f64, f64, f64),
}

/// The point at height `h` and azimuth `t` on [`unit_cone`] (either
/// nappe: `h < 0` is the mirror one).
fn cone_at(h: f64, t: f64) -> Point3<f64> {
    p(h.abs() * t.cos(), h.abs() * t.sin(), h)
}

/// **One face of the unit cone, bounded by a chain of edges from
/// `start` closed back to it by a straight edge.** The chain is grown by
/// `mev` from the seed vertex and closed by `mef`; the face returned is
/// the one the chain bounds in its own order.
fn cone_sheet(start: Point3<f64>, steps: &[Step]) -> (Body<f64>, FaceKey) {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(start).unwrap();
    let cone = body
        .set_face_surface(seed.face, FaceSurface::New(unit_cone()))
        .unwrap();
    let mut edges: Vec<crate::MevCreated> = Vec::new();
    for &step in steps {
        let site = match edges.last() {
            None => MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Some(e) => MevSite::Fan {
                he1: e.he_minus,
                he2: e.he_minus,
            },
        };
        edges.push(match step {
            Step::Line(to) => body.mev_line(site, to, tol).unwrap(),
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
                body.mev(site, cone_at(h, t1), spec, tol).unwrap()
            }
        });
    }
    let (first, last) = (edges[0], edges[edges.len() - 1]);
    let end = crate::readback::vertex_point_ref(&body, last.vertex).unwrap();
    let face = body
        .mef(
            MefSite::Chords {
                he1: first.he_plus,
                he2: last.he_minus,
            },
            EdgeCurveSpec::line_between(start, end),
            FaceSurface::Shared(cone),
            tol,
        )
        .unwrap()
        .face;
    (body, face)
}

fn contain_at(body: &Body<f64>, face: FaceKey, q: Point3<f64>) -> Option<FaceContainment> {
    crate::curved_face_containment(body, face, q, band()).unwrap()
}

/// **An L-shaped cone face refuses, never trims by its hull.** The face
/// covers `z < 1` over `[0, w]` and `z < 2` over `[w, 2w]`; the notch
/// `1 < z < 2` over `[0, w]` has the same hull, so a window read off the
/// hull answers `In` there. `bool_cone_chart_box` sees the notch (the
/// polygon's area falls short of its box) and the face door answers
/// `None`. Narrow (`w = π/8`, the nearest-branch walk's class, wrong on
/// main too) and wide (`w = 3π/4`, the apex closure's), and one clear of
/// the apex (the notch cut from a frustum band).
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

/// The point at chart `(u, t)` on [`donut`]: major angle `u` from `x`,
/// minor angle `t` from the outer equator towards `+z`.
fn donut_at(u: f64, t: f64) -> Point3<f64> {
    let rho = 2.0 + 0.5 * t.cos();
    p(rho * u.cos(), rho * u.sin(), 0.5 * t.sin())
}

/// The iso arc of [`donut`] from chart point `a` to chart point `b`,
/// which share a coordinate: a parallel (a horizontal section) when they
/// share `t`, a meridian (an axial section) when they share `u`. Either
/// runs forward from `a` on the carrier whose `u_ref` points at `a`.
fn donut_iso(
    body: &mut Body<f64>,
    torus: crate::geometry::SurfaceKey,
    a: (f64, f64),
    b: (f64, f64),
) -> EdgeCurveSpec<f64> {
    let mid = (0.5 * (a.0 + b.0), 0.5 * (a.1 + b.1));
    let (center, axis, radius, u_ref, sweep, cut) = if a.1 == b.1 {
        let h = 0.5 * a.1.sin();
        let axis = if b.0 > a.0 {
            Vec3::unit_z()
        } else {
            -Vec3::unit_z()
        };
        (
            p(0.0, 0.0, h),
            axis,
            2.0 + 0.5 * a.1.cos(),
            v(a.0.cos(), a.0.sin(), 0.0),
            (b.0 - a.0).abs(),
            plane(p(0.0, 0.0, h), Vec3::unit_z()),
        )
    } else {
        assert_eq!(a.0, b.0, "an iso arc shares a coordinate");
        let r_hat = v(a.0.cos(), a.0.sin(), 0.0);
        let n = r_hat.cross(Vec3::unit_z());
        (
            p(0.0, 0.0, 0.0) + r_hat * 2.0,
            if b.1 > a.1 { n } else { -n },
            0.5,
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
            witness: donut_at(mid.0, mid.1),
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

/// **One face of [`donut`] bounded by iso arcs through the chart
/// corners**, in order, closed back to the first by `mef`: the torus
/// face a boolean mints where parallels and meridians cut the tube.
fn donut_sheet(corners: &[(f64, f64)]) -> (Body<f64>, FaceKey) {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(donut_at(corners[0].0, corners[0].1)).unwrap();
    let torus = body
        .set_face_surface(seed.face, FaceSurface::New(donut()))
        .unwrap();
    let mut edges: Vec<crate::MevCreated> = Vec::new();
    for pair in corners.windows(2) {
        let site = match edges.last() {
            None => MevSite::Lone {
                r#loop: seed.r#loop,
            },
            Some(e) => MevSite::Fan {
                he1: e.he_minus,
                he2: e.he_minus,
            },
        };
        let spec = donut_iso(&mut body, torus, pair[0], pair[1]);
        edges.push(
            body.mev(site, donut_at(pair[1].0, pair[1].1), spec, tol)
                .unwrap(),
        );
    }
    let (first, last) = (edges[0], edges[edges.len() - 1]);
    let spec = donut_iso(&mut body, torus, corners[0], corners[corners.len() - 1]);
    let face = body
        .mef(
            MefSite::Chords {
                he1: first.he_plus,
                he2: last.he_minus,
            },
            spec,
            FaceSurface::Shared(torus),
            tol,
        )
        .unwrap()
        .face;
    (body, face)
}

/// **An L-shaped torus face refuses, never trims by its hull.** The face
/// covers `t ∈ [−a, 0]` over `u ∈ [0, a]` and `t ∈ [−a, a]` over
/// `u ∈ [a, 2a]`; the notch `t ∈ (0, a)` over `u ∈ (0, a)` has the same
/// hull. The L is monotone in both channels, so a variation-to-span
/// comparison passes it; `bool_torus_chart_box` compares areas and sees
/// the notch, and the face door answers `None`. The U (two notches'
/// worth, not monotone) refuses too, and the rectangle trims.
#[test]
fn an_l_shaped_torus_face_refuses_rather_than_trim_by_its_hull() {
    let a = PI / 4.0;
    let l = [
        (0.0, -a),
        (2.0 * a, -a),
        (2.0 * a, a),
        (a, a),
        (a, 0.0),
        (0.0, 0.0),
    ];
    let notch = donut_at(0.5 * a, 0.5 * a);
    let (body, face) = donut_sheet(&l);
    assert_eq!(
        contain_at(&body, face, donut_at(1.5 * a, 0.5 * a)),
        Some(FaceContainment::In),
        "the L holds its arm"
    );
    assert_eq!(contain_at(&body, face, notch), None, "the L's notch");
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
    let (body, face) = donut_sheet(&u);
    assert_eq!(
        contain_at(&body, face, donut_at(1.5 * a, 0.5 * a)),
        None,
        "the U's notch"
    );
    let (body, face) = donut_sheet(&[(0.0, -a), (2.0 * a, -a), (2.0 * a, a), (0.0, a)]);
    assert_eq!(
        contain_at(&body, face, notch),
        Some(FaceContainment::In),
        "the rectangle holds the point"
    );
    assert_eq!(
        contain_at(&body, face, donut_at(2.5 * a, 0.5 * a)),
        Some(FaceContainment::Out),
        "the rectangle trims past its window"
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
