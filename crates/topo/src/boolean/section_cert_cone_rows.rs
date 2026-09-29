//! **The section certificate's cone rows**: the classifier on a cone
//! against each partner it has an arm for, every row both ways round
//! (the cone as `F` and as `G`), and the per-pair rule over real cone
//! faces for each component class.
//!
//! The cone is [`unit_cone`]: apex at the origin, axis `z`, half-angle
//! `π/4`, so its double carrier is `ρ = |z|`. Every row names the
//! mutant that turns it red.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

use super::section_cert_rows::{
    IN, OUT, Shape, Step, at, band, classify, cone_at, cone_sheet, cylinder, on_both, p, plane,
    shape, sphere, tangent, unit_cone, v, witness,
};
use super::*;
use crate::boolean::ops;
use crate::test_support_fixtures::brick;
use crate::{Body, FaceKey, FaceSurface, MefSite, MevSite};
use core::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_4, PI, TAU};
use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Point3, Tol, Vec3};

/// The classification with the cone as `F`, after checking that the
/// cone as `G` gives the same one with its roles exchanged.
fn both(cone: &Surface<f64>, partner: &Surface<f64>) -> Section<f64> {
    let s = classify(cone, partner);
    let t = classify(partner, cone);
    match (&s, &t) {
        (
            Section::Components {
                parts: a,
                single: sa,
            },
            Section::Components {
                parts: b,
                single: sb,
            },
        ) => {
            assert_eq!(sa, sb, "single, both ways");
            assert_eq!(a.len(), b.len(), "the count, both ways");
            for (x, y) in a.iter().zip(b) {
                assert_eq!(
                    (x.unbounded, x.essential_f, x.essential_g),
                    (y.unbounded, y.essential_g, y.essential_f),
                    "a component's flags, both ways"
                );
                assert_eq!(
                    x.witness.map(|w| [w.x, w.y, w.z]),
                    y.witness.map(|w| [w.x, w.y, w.z]),
                    "a component's witness, both ways"
                );
            }
        }
        (Section::Tangent(a), Section::Tangent(b)) => assert_eq!(a, b),
        (Section::Intractable, Section::Intractable) => {}
        _ => panic!("the two orders disagree: {s:?} against {t:?}"),
    }
    s
}

/// Every component's witness lies on both carriers.
fn witnesses_on_both(s: &Section<f64>, f: &Surface<f64>, g: &Surface<f64>) -> Vec<Point3<f64>> {
    let Section::Components { parts, .. } = s else {
        panic!("not classified: {s:?}")
    };
    parts
        .iter()
        .map(|c| {
            let w = c
                .witness
                .expect("every cone component here carries a witness");
            on_both(w, f, g);
            w
        })
        .collect()
}

fn shape_of(n: usize, single: bool, ef: bool, eg: bool, unbounded: bool) -> Shape {
    (n, single, vec![ef; n], vec![eg; n], vec![unbounded; n])
}

// -------------------------------------------------------------------
// Cone × plane
// -------------------------------------------------------------------

/// **An oblique plane cuts one ellipse**, essential on the cone and
/// certified single, witnessed at its vertex nearer the apex: the plane
/// `0.3x + z = ±1` meets the generator `x = z` at `z = ±1/1.3` (the far
/// vertex is at `x = −z = ±1/0.7`). The same plane stored with either
/// normal gives the same witness, on the upper nappe and on the mirror
/// one. The mutant that drops the facing decision (the generator read
/// off the stored normal) witnesses the reversed plane at its far
/// vertex: red.
#[test]
fn an_oblique_plane_cuts_one_ellipse_witnessed_at_its_near_vertex() {
    let cone = unit_cone();
    for side in [1.0, -1.0] {
        let near = p(side / 1.3, 0.0, side / 1.3);
        for n in [v(0.3, 0.0, 1.0), v(-0.3, 0.0, -1.0)] {
            let pl = plane(p(0.0, 0.0, side), n);
            let s = both(&cone, &pl);
            assert_eq!(shape(&s), shape_of(1, true, true, false, false), "{n:?}");
            let w = witness(&s);
            on_both(w, &cone, &pl);
            assert!(
                (w - near).norm() < 1e-12,
                "side {side}, normal {n:?}: the witness {w:?} is not the near vertex {near:?}"
            );
        }
    }
}

/// **An axis-normal plane cuts a parallel**, the ellipse's round case:
/// the plane's normal has no meridian, so the witness is on the
/// generator line along the cone's seam direction — past the apex, on
/// the mirror nappe, for the parallel below it.
#[test]
fn an_axis_normal_plane_cuts_a_parallel_witnessed_on_the_seam() {
    let cone = unit_cone();
    for z in [0.7, -0.7] {
        let pl = plane(p(0.0, 0.0, z), Vec3::unit_z());
        let s = both(&cone, &pl);
        assert_eq!(shape(&s), shape_of(1, true, true, false, false));
        let w = witness(&s);
        on_both(w, &cone, &pl);
        assert!((w - p(z, 0.0, z)).norm() < 1e-12, "{w:?}");
    }
}

/// **A plane the axis direction lies in, or nearly, cuts a hyperbola**:
/// one branch per nappe, both unbounded. Through the apex it cuts two
/// lines, unbounded all the same. The mutant reading the hyperbola as an
/// ellipse makes it single: red.
#[test]
fn a_steep_plane_cuts_two_unbounded_branches() {
    let cone = unit_cone();
    for (origin, n) in [
        (p(0.3, 0.0, 0.0), v(1.0, 0.0, 0.2)),
        (p(0.0, 0.0, 0.0), v(1.0, 0.0, 0.0)),
        (p(0.0, 0.0, 0.0), v(1.0, 1.0, 0.5)),
    ] {
        let s = both(&cone, &plane(origin, n));
        assert_eq!(shape(&s), shape_of(2, false, false, false, true), "{n:?}");
    }
}

/// **A plane at the parabola's aperture is essential with no witness**:
/// the aperture margin is Zero, and the section is a parabola or an
/// ellipse or hyperbola near one, every component of which is unbounded
/// or essential; nothing counts them. Through the apex it is the double
/// line, read the same way. A hundredth of a radian either way decides:
/// the ellipse, the hyperbola. The mutants: the Zero read as unbounded
/// (W1 clears on any face), or as the ellipse (a witness, and single).
#[test]
fn a_plane_at_the_parabola_is_essential_with_no_witness() {
    let cone = unit_cone();
    let tilted = |b: f64| v(b.sin(), 0.0, b.cos());
    for origin in [p(0.0, 0.0, 1.0), p(0.0, 0.0, 0.0)] {
        let s = both(&cone, &plane(origin, tilted(FRAC_PI_4)));
        assert_eq!(shape(&s), shape_of(1, false, true, false, false));
        let Section::Components { parts, .. } = &s else {
            unreachable!()
        };
        assert!(parts[0].witness.is_none(), "{s:?}");
    }
    let origin = p(0.0, 0.0, 1.0);
    let ellipse = both(&cone, &plane(origin, tilted(FRAC_PI_4 - 0.01)));
    assert_eq!(shape(&ellipse), shape_of(1, true, true, false, false));
    let hyperbola = both(&cone, &plane(origin, tilted(FRAC_PI_4 + 0.01)));
    assert_eq!(shape(&hyperbola), shape_of(2, false, false, false, true));
}

/// **The apex's offset from the plane does not bound the ellipse.** A
/// plane `m = zero/2` from the apex — a Zero offset — and short of the
/// parabola by an aperture margin decided Positive meets the cone in a
/// real ellipse whose far vertex runs `m·cos α / μ` along a generator:
/// a closed curve many bands long, not a point. The class is the
/// ellipse's, witnessed. The mutant that reads a Zero offset as the
/// apex alone (`none()`, W0) clears a loop that can lie inside a
/// seamless band and a plane face: red.
#[test]
fn the_apex_offset_does_not_bound_the_ellipse() {
    let cone = unit_cone();
    let b = band();
    let (s, c) = FRAC_PI_4.sin_cos();
    let m = 0.5 * b.zero();
    // The rows' lever is 5 m about the apex; μ·5 = 2·escalate.
    let mu = 0.4 * b.escalate();
    let beta = (s + mu).acos();
    let n = v(beta.sin(), 0.0, beta.cos());
    let pl = plane(p(0.0, 0.0, 0.0) - n * m, n);
    let sec = both(&cone, &pl);
    assert_eq!(shape(&sec), shape_of(1, true, true, false, false));
    on_both(witness(&sec), &cone, &pl);
    let far_gen = v(-s, 0.0, c);
    let far = p(0.0, 0.0, 0.0) + far_gen * (-m / far_gen.dot(n));
    for surf in [&cone, &pl] {
        let r = geom_brep::implicit_residual(surf, far);
        assert!(r.abs() < b.zero(), "the far vertex {far:?} is {r} off");
    }
    assert!(
        (far - p(0.0, 0.0, 0.0)).norm() > 1e3 * b.zero(),
        "the ellipse is real: its far vertex is {far:?}"
    );
}

// -------------------------------------------------------------------
// Cone × sphere
// -------------------------------------------------------------------

/// The ball touching the lateral face at `(0.5, 0, ±0.5)`: its centre
/// `0.05` out along the outward normal, radius `0.1`. It meets that
/// nappe in one null loop.
fn ball_on_the_face(side: f64) -> Surface<f64> {
    let foot = p(0.5, 0.0, 0.5 * side);
    sphere(foot + v(1.0, 0.0, -side) * (0.05 * FRAC_1_SQRT_2), 0.1)
}

/// **A small ball on the lateral face meets it in one null loop**, on
/// the face's own nappe, witnessed there, certified single. With no
/// event and its witness inside both faces it refuses R-loop — even on
/// a cone face that describes; with an event it clears W4; a witness
/// `Out` clears it W3. The mutant that reads the arc case as the whole
/// circle claims two essential curves, which W2 clears: red.
#[test]
fn a_ball_on_the_lateral_face_is_one_null_loop() {
    let cone = unit_cone();
    for side in [1.0, -1.0] {
        let ball = ball_on_the_face(side);
        let s = both(&cone, &ball);
        assert_eq!(shape(&s), shape_of(1, true, false, false, false), "{side}");
        let w = witness(&s);
        on_both(w, &cone, &ball);
        assert!(
            w.z * side > 0.0,
            "the loop's witness {w:?} is on the ball's nappe"
        );
        assert_eq!(certify(&s, false, |_| true, at(IN, IN)), Err(Refusal::Loop));
        assert_eq!(
            certify(&s, true, |_| true, at(IN, IN)),
            Ok(vec![Cleared::LoneEvented])
        );
        assert_eq!(
            certify(&s, false, |_| true, at(OUT, IN)),
            Ok(vec![Cleared::Out(Side::F)])
        );
    }
}

/// **The apex inside the ball: one essential curve per nappe**, each
/// witnessed on its own nappe, so W3 clears the mirror one on a face
/// that does not describe. The mutant witnessing both on one root puts
/// both on one nappe: red.
#[test]
fn a_ball_around_the_apex_meets_each_nappe_in_one_essential_curve() {
    let cone = unit_cone();
    let ball = sphere(p(0.3, 0.1, 0.2), 1.0);
    let s = both(&cone, &ball);
    assert_eq!(shape(&s), shape_of(2, false, true, false, false));
    let w = witnesses_on_both(&s, &cone, &ball);
    assert!(w[0].z > 0.0 && w[1].z < 0.0, "{w:?}");
}

/// **Every nappe class, and both nappes at once.** A ball on the axis
/// above the apex that every upper generator crosses: two essential
/// curves on that nappe (and below it, on the mirror one). A ball
/// beside the apex that both extreme lines cross, on opposite sides of
/// it: one null loop on each nappe, not single. A ball clear of the
/// cone: nothing. The mutants: the arc and whole cases exchanged, or
/// the nappes' side decision reversed (the witnesses leave the
/// carriers).
#[test]
fn sphere_and_cone_every_nappe_class() {
    let cone = unit_cone();
    for side in [1.0, -1.0] {
        let ball = sphere(p(0.0, 0.0, 3.0 * side), 2.5);
        let s = both(&cone, &ball);
        assert_eq!(shape(&s), shape_of(2, false, true, false, false));
        for w in witnesses_on_both(&s, &cone, &ball) {
            assert!(w.z * side > 0.0, "{w:?}");
        }
    }
    let beside = sphere(p(2.0, 0.0, 0.0), 1.6);
    let s = both(&cone, &beside);
    assert_eq!(shape(&s), shape_of(2, false, false, false, false));
    let w = witnesses_on_both(&s, &cone, &beside);
    assert!(w[0].z > 0.0 && w[1].z < 0.0, "{w:?}");
    let clear = both(&cone, &sphere(p(3.0, 0.0, 0.0), 0.5));
    assert_eq!(shape(&clear), shape_of(0, false, false, false, false));
}

/// **R-tan: the apex on the sphere, and a generator tangent to it.** The
/// ball of radius `√2` about `(2, 0, 0)` is `2 sin π/4 = √2` from both
/// extreme generator lines.
#[test]
fn a_ball_through_the_apex_or_on_a_generator_refuses_as_a_tangency() {
    let cone = unit_cone();
    assert_eq!(
        tangent(&both(&cone, &sphere(p(0.6, 0.0, 0.8), 1.0))),
        "section_cone_sphere_apex"
    );
    assert_eq!(
        tangent(&both(&cone, &sphere(p(2.0, 0.0, 0.0), 2.0f64.sqrt()))),
        "section_cone_sphere_generator"
    );
}

// -------------------------------------------------------------------
// Coaxial and parallel-axis partners
// -------------------------------------------------------------------

/// **Coaxial partners meet the cone in parallels, essential on both.**
/// A cylinder, always; a cone with its apex elsewhere on the axis,
/// always, and a common apex is R-tan; a torus when its tube reaches a
/// nappe's generator (`r − |R cos α ∓ z sin α|`), else nothing, and a
/// tube tangent to one R-tan. The torus below the apex reaches only the
/// mirror nappe. The mutants: the torus's two nappes required together,
/// or both read on one sign of `z`.
#[test]
fn coaxial_partners_meet_the_cone_in_parallels() {
    let cone = unit_cone();
    let pair = shape_of(2, false, true, true, false);
    let apart = shape_of(0, false, false, false, false);
    assert_eq!(
        shape(&both(
            &cone,
            &cylinder(p(0.0, 0.0, 5.0), Vec3::unit_z(), 0.5)
        )),
        pair
    );
    let other = |apex: Point3<f64>, axis: Vec3<f64>| Surface::Cone {
        apex,
        axis,
        half_angle: 0.3,
        u_ref: Vec3::unit_x(),
    };
    assert_eq!(
        shape(&both(&cone, &other(p(0.0, 0.0, 2.0), -Vec3::unit_z()))),
        pair
    );
    assert_eq!(
        tangent(&both(&cone, &other(p(0.0, 0.0, 0.0), Vec3::unit_z()))),
        "section_cone_coaxial_apexes"
    );
    let ring = |z: f64, big: f64| Surface::Torus {
        center: p(0.0, 0.0, z),
        axis: Vec3::unit_z(),
        major_radius: big,
        minor_radius: 0.2,
        u_ref: Vec3::unit_x(),
    };
    assert_eq!(shape(&both(&cone, &ring(1.0, 1.0))), pair);
    assert_eq!(shape(&both(&cone, &ring(-1.0, 1.0))), pair);
    assert_eq!(shape(&both(&cone, &ring(1.0, 3.0))), apart);
    assert_eq!(
        tangent(&both(&cone, &ring(1.0, 1.0 + 0.2 / FRAC_1_SQRT_2))),
        "section_torus_coaxial_cone_near"
    );
}

/// **A parallel-axis cylinder: two components, one per nappe**,
/// essential on the cylinder always and on the cone iff it encloses the
/// cone's axis; a ruling through the apex is R-tan. The witnesses sit
/// on the ruling nearest the axis, `(e − r_c, 0, ±|e − r_c|)`. The
/// mutant reading the enclosure the other way: red.
#[test]
fn a_parallel_axis_cylinder_every_class() {
    let cone = unit_cone();
    for (rc, encloses) in [(0.8, true), (0.3, false)] {
        let wall = cylinder(p(0.5, 0.0, 7.0), Vec3::unit_z(), rc);
        let s = both(&cone, &wall);
        assert_eq!(shape(&s), shape_of(2, false, encloses, true, false), "{rc}");
        let w = witnesses_on_both(&s, &cone, &wall);
        let x = 0.5 - rc;
        assert!((w[0] - p(x, 0.0, x.abs())).norm() < 1e-12, "{w:?}");
        assert!((w[1] - p(x, 0.0, -x.abs())).norm() < 1e-12, "{w:?}");
    }
    assert_eq!(
        tangent(&both(
            &cone,
            &cylinder(p(0.5, 0.0, 0.0), Vec3::unit_z(), 0.5)
        )),
        "section_cone_cylinder_apex"
    );
}

// -------------------------------------------------------------------
// The per-pair rule over real cone faces
// -------------------------------------------------------------------

/// The full rim of [`unit_cone`] at height `z`, as the intersection of
/// `cone` and the rim plane there.
fn cone_rim(body: &mut Body<f64>, cone: crate::geometry::SurfaceKey, z: f64) -> EdgeCurveSpec<f64> {
    let rim_plane = body.add_surface(plane(p(0.0, 0.0, z), Vec3::unit_z()));
    EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            s1: cone,
            s2: rim_plane,
            witness: p(-z, 0.0, z),
        },
        carrier: Curve3::Circle {
            center: p(0.0, 0.0, z),
            axis: Vec3::unit_z(),
            radius: z,
            u_ref: Vec3::unit_x(),
        },
        param_start: 0.0,
        param_end: TAU,
    }
}

/// **A seamless frustum band** of [`unit_cone`] over `z ∈ [z0, z1]`:
/// one face bounded by its two full rims and no generator edge, so it
/// does not describe. Built as the cylinder's seamless band is: each
/// rim splits off a cap, a strut joins them, and the strut's kill
/// leaves the band two loops.
fn seamless_cone_band(z0: f64, z1: f64) -> (Body<f64>, FaceKey) {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p(z0, 0.0, z0)).unwrap();
    let cone = body
        .set_face_surface(seed.face, FaceSurface::New(unit_cone()))
        .unwrap();
    let bottom = cone_rim(&mut body, cone, z0);
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
            p(z1, 0.0, z1),
            tol,
        )
        .unwrap();
    let top = cone_rim(&mut body, cone, z1);
    body.mef(
        MefSite::Chords {
            he1: strut.he_minus,
            he2: strut.he_minus,
        },
        top,
        FaceSurface::New(plane(p(0.0, 0.0, z1), Vec3::unit_z())),
        tol,
    )
    .unwrap();
    body.kemr(strut.he_plus, strut.he_minus).unwrap();
    (body, seed.face)
}

/// A slab `0.2` thick and `6` wide about `z = z0`, tilted by `tilt` about
/// the `y` line through `(0, 0, z0)`.
fn slab(z0: f64, tilt: f64) -> Body<f64> {
    let b = brick::<f64>(
        (-3.0, 3.0),
        (-3.0, 3.0),
        (z0 - 0.1, z0 + 0.1),
        Tol::witness(),
    );
    let turn = geom_core::Affine3::rotation_about_axis(p(0.0, 0.0, z0), Vec3::unit_y(), tilt);
    crate::transform::transform_rigid(&b, &turn, Tol::witness()).unwrap()
}

/// The verdict of every pair of `a`'s `face` against `b`'s faces, with
/// an event on every pair or on none.
fn scan_with(
    a: &Body<f64>,
    face: FaceKey,
    b: &Body<f64>,
    evented: bool,
) -> Vec<Result<Vec<Cleared>, Refusal>> {
    ops::section_pairs(
        a,
        b,
        band(),
        ops::SectionPath::Crossings,
        |_, _| false,
        |_, _| evented,
        false,
    )
    .unwrap()
    .into_iter()
    .filter(|pv| pv.a_face == face)
    .map(|pv| pv.verdict)
    .collect()
}

fn describes(body: &Body<f64>, face: FaceKey) -> bool {
    let surface = body
        .get_surface(body.get_face(face).unwrap().surface)
        .unwrap()
        .clone();
    ops::ChartCache::default().describes(crate::Operand::A, body, face, &surface, band())
}

/// **W2: the ellipse clears on a cone face that describes** — a
/// frustum sector with its generator edges, and apex-closed sectors of
/// widths `π/2` and `3π/2` through the apex closure. The tilted slab's
/// faces cut each in ellipses; every pair clears, the ellipses by W2.
#[test]
fn the_ellipse_clears_by_w2_on_a_describing_cone_face() {
    let slab = slab(1.0, 0.2);
    let sheets = [
        cone_sheet(
            cone_at(0.5, 0.0),
            &[
                Step::Arc(0.5, 0.0, 1.5 * PI),
                Step::Line(cone_at(2.0, 1.5 * PI)),
                Step::Arc(2.0, 1.5 * PI, 0.0),
            ],
        ),
        cone_sheet(
            p(0.0, 0.0, 0.0),
            &[Step::Line(cone_at(2.0, 0.0)), Step::Arc(2.0, 0.0, 0.5 * PI)],
        ),
        cone_sheet(
            p(0.0, 0.0, 0.0),
            &[Step::Line(cone_at(2.0, 0.0)), Step::Arc(2.0, 0.0, 1.5 * PI)],
        ),
    ];
    for (i, (body, face)) in sheets.iter().enumerate() {
        assert!(describes(body, *face), "sheet {i} describes");
        let verdicts = scan_with(body, *face, &slab, false);
        assert!(
            verdicts.contains(&Ok(vec![Cleared::Essential(Side::F)])),
            "sheet {i}: {verdicts:?}"
        );
        assert!(
            verdicts.iter().all(Result::is_ok),
            "sheet {i}: {verdicts:?}"
        );
    }
}

/// **On a seamless band the lone ellipse is W4 with an event, and
/// undecided without one.** The slab's faces cut the band in ellipses
/// inside both faces (`z ∈ [0.71, 1.38]` against the band's
/// `[0.5, 1.5]`). The band does not describe, so W2 is out; with an
/// event on the pair W4 clears the certified single ellipse. Without
/// one the witness decides, and the cone trim does not place a point on
/// a seamless band (it reads a full period of a face that is not the
/// chart rectangle), so the pair refuses R-undec. The mutant reading the
/// ellipse as a hyperbola clears it W1: red.
#[test]
fn the_ellipse_on_a_seamless_band_is_w4_with_an_event_and_undecided_without() {
    let (band_body, face) = seamless_cone_band(0.5, 1.5);
    assert!(
        !describes(&band_body, face),
        "the seamless band does not describe"
    );
    let slab = slab(1.0, 0.2);
    let quiet = scan_with(&band_body, face, &slab, false);
    assert_eq!(quiet.len(), 2, "the slab's two broad faces: {quiet:?}");
    assert!(
        quiet.iter().all(|v| *v == Err(Refusal::Undecided)),
        "{quiet:?}"
    );
    let loud = scan_with(&band_body, face, &slab, true);
    assert!(
        loud.iter().all(|v| *v == Ok(vec![Cleared::LoneEvented])),
        "{loud:?}"
    );
}

/// **W1 on a seamless band**: a slab standing along the axis cuts
/// hyperbolas, which clear unbounded whether or not the face describes.
#[test]
fn the_band_clears_a_hyperbola_unbounded() {
    let (band_body, face) = seamless_cone_band(0.5, 1.5);
    let upright = brick::<f64>((0.2, 0.4), (-3.0, 3.0), (-3.0, 3.0), Tol::witness());
    let steep = scan_with(&band_body, face, &upright, false);
    assert!(!steep.is_empty());
    assert!(
        steep
            .iter()
            .all(|v| *v == Ok(vec![Cleared::Unbounded, Cleared::Unbounded])),
        "{steep:?}"
    );
}

/// **R-undec: the parabola on a face that does not describe.** The
/// slab turned `π/4` lies at the cone's aperture: its broad faces' class
/// is essential with no witness, which only W2 can clear, and the
/// seamless band does not describe.
#[test]
fn a_parabola_on_a_seamless_band_refuses_undecided() {
    let (band_body, face) = seamless_cone_band(0.5, 1.5);
    let verdicts = scan_with(&band_body, face, &slab(1.0, FRAC_PI_4), false);
    assert!(!verdicts.is_empty());
    assert!(
        verdicts.iter().all(|v| *v == Err(Refusal::Undecided)),
        "{verdicts:?}"
    );
}
