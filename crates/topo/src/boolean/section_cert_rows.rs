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
use crate::{Body, FaceKey, FaceSurface, MefSite, MevSite};
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

fn classify(f: &Surface<f64>, g: &Surface<f64>) -> Section<f64> {
    super::classify(f, g, LEVER, band())
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

/// **A near-axial plane** (`s ≈ 1e-9`) at the tube's top: tangent or
/// decided, never a wrong class.
#[test]
fn a_near_axial_plane_is_decided_or_tangent_never_wrong() {
    let n = v(1e-9, 0.0, 1.0);
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
    // Apart.
    let s = classify(&a, &cylinder(p(0.0, 2.0, 0.0), Vec3::unit_x(), 0.5));
    assert_eq!(shape(&s).0, 0);
    // The tangent pose |δ₀| = r₁ + r₂.
    let s = classify(&a, &cylinder(p(0.0, 1.5, 0.0), Vec3::unit_x(), 0.5));
    assert_eq!(tangent(&s), "section_cylinder_pair_reach");
}

/// **The `|d₁ × d₂|` decision**: two walls `1e-9` rad off parallel,
/// offset so the long saddle loop exists, refuse R-tan. Dropping the
/// decision would call them parallel (rulings, W1) and clear the loop.
#[test]
fn near_parallel_walls_refuse_as_a_tangency() {
    let a = cylinder(p(0.0, 0.0, 0.0), Vec3::unit_z(), 1.0);
    let b = cylinder(p(1.3, 0.0, 0.0), v(0.0, 1e-9, 1.0), 0.5);
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
    assert_eq!(shape(&s).3, vec![true, true]);
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
