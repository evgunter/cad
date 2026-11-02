//! **Review rows for PR 6029 (a curved body against its copy).** The
//! no-crossings certificates now read the sweep's touches as events
//! (W4), and a refusal on a pair settled one carrier is held for the
//! shells' `On` verdict. These rows put a section circle inside both
//! faces that only grazes a boundary at a touch, two operands sharing
//! carriers and a rim but not a patch, and copies partly shared, scaled
//! across the band or fed back as a third operand: each op builds the
//! closed form or refuses typed, and none builds wrong.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::revert_ops::subtract_both_orders_and_intersect;
use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, revolved_about_y};
use topo::{Body, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

fn hemisphere() -> Body<f64> {
    sweep::test_support::hemisphere_on_flat_base(1.0, tol())
}

fn rotated(body: &Body<f64>, axis: Vec3<f64>, angle: f64) -> Body<f64> {
    topo::transform_rigid(
        body,
        &geom_core::test_support::rigid(
            &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
            tol(),
        ),
        tol(),
    )
    .unwrap()
}

/// The box whose bottom face, the square of half-width ρ = 0.6 in the
/// plane at distance d = 0.8 from the centre, holds the sphere's section
/// circle inscribed: the circle lies inside the face and touches its
/// four edges at their midpoints. The cap beyond the plane is inside
/// the box, so `hemisphere ∩ box` is the cap, π h²(3 − h)/3 with h = 0.2.
#[test]
fn a_section_circle_inscribed_in_a_plane_face_is_cut_or_refused() {
    let (d, rho) = (0.8, 0.6);
    let hemi = hemisphere();
    let cap = PI * 0.04 * 2.8 / 3.0;
    let whole = 2.0 * PI / 3.0;
    for (axis, angle) in [
        (Vec3::new(0.0, 0.0, 1.0), PI / 4.0),
        (Vec3::new(0.0, 0.0, 1.0), -PI / 4.0),
        (Vec3::new(1.0, 0.0, 0.0), PI / 4.0),
        (Vec3::new(1.0, 0.0, 0.0), -PI / 4.0),
    ] {
        let block = rotated(
            &brick((-rho, rho), (d, d + 1.0), (-rho, rho), tol()),
            axis,
            angle,
        );
        builds_right_or_refuses(
            &format!("hemisphere, box turned {angle} about {axis:?}"),
            &hemi,
            &block,
            (whole, 1.2 * 1.2, cap),
        );
    }
}

/// The lens and the union of the unit balls about `(0, ±½, 0)`: each
/// lens face lies on one carrier with a union face, at one orientation,
/// and the two share only the rim circle at `y = 0` and its seam vertex.
/// Every witness the `On` ladder takes of either shell lies on that
/// circle, on the other's boundary.
#[test]
fn a_lens_against_the_union_of_its_balls() {
    let (q, s) = ((PI / 12.0).tan(), (PI / 6.0).tan());
    let w = 0.75f64.sqrt();
    let lens = revolved_about_y(
        vec![
            (Point2::new(0.0, -0.5), q),
            (Point2::new(w, 0.0), q),
            (Point2::new(0.0, 0.5), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    let union = revolved_about_y(
        vec![
            (Point2::new(0.0, -1.5), s),
            (Point2::new(w, 0.0), s),
            (Point2::new(0.0, 1.5), 0.0),
        ],
        sweep::Revolution::Full,
        tol(),
    );
    let l = PI * 0.25 * 2.5 / 3.0 * 2.0;
    let u = 8.0 * PI / 3.0 - l;
    builds_right_or_refuses("lens, union", &lens, &union, (l, u, l));
    builds_right_or_refuses("union, lens", &union, &lens, (u, l, l));
}

/// The hemisphere against a prism whose bottom face, in the plane at
/// distance 0.8 from the centre, holds the section circle (radius 0.6)
/// strictly inside it except where a thin notch's tip VERTEX touches the
/// circle from outside: one vertex-on-face touch, no crossing, and the
/// circle otherwise inside both faces. `hemisphere ∩ prism` is the cap.
#[test]
fn a_section_circle_touched_by_a_notch_tip_is_cut_or_refused() {
    let hemi = hemisphere();
    let cap = PI * 0.04 * 2.8 / 3.0;
    let whole = 2.0 * PI / 3.0;
    let notched = sweep::test_support::prism_at(
        sweep::test_support::corners(&[
            (-0.7, -0.7),
            (0.7, -0.7),
            (0.7, -0.05),
            (0.6, 0.0),
            (0.7, 0.05),
            (0.7, 0.7),
            (-0.7, 0.7),
        ]),
        0.8,
        1.0,
        tol(),
    );
    let prism_vol = 1.96 - 0.005;
    let x = Vec3::new(1.0, 0.0, 0.0);
    let z = Vec3::new(0.0, 0.0, 1.0);
    let up = rotated(&notched, x, -PI / 2.0);
    for (label, b) in [
        ("x -π/4", rotated(&notched, x, -PI / 4.0)),
        ("x -3π/4", rotated(&notched, x, -3.0 * PI / 4.0)),
        ("y then z +π/4", rotated(&up, z, PI / 4.0)),
        ("y then z -π/4", rotated(&up, z, -PI / 4.0)),
        (
            "y then z +π/4, tip turned",
            rotated(&rotated(&notched, z, PI / 2.0), x, -PI / 4.0),
        ),
    ] {
        builds_right_or_refuses(
            &format!("hemisphere, notched prism {label}"),
            &hemi,
            &b,
            (whole, prism_vol, cap),
        );
    }
}

/// As the notch probe, with the notch's tip on the hemisphere's POLE: a
/// vertex-on-vertex touch, the section circle through the pole and
/// otherwise inside both faces. Turned about the hemisphere's axis so
/// one turn keeps its seam meridian out of the cap.
#[test]
fn a_section_circle_through_the_pole_at_a_notch_tip_is_cut_or_refused() {
    let hemi = hemisphere();
    let cap = PI * 0.04 * 2.8 / 3.0;
    let whole = 2.0 * PI / 3.0;
    let notched = sweep::test_support::prism_at(
        sweep::test_support::corners(&[
            (-0.7, -0.7),
            (0.7, -0.7),
            (0.7, 0.7),
            (-0.7, 0.7),
            (-0.7, 0.05),
            (-0.6, 0.0),
            (-0.7, -0.05),
        ]),
        0.8,
        1.0,
        tol(),
    );
    let prism_vol = 1.96 - 0.005;
    let (x, y, z) = (
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    let tilted = rotated(&rotated(&notched, x, -PI / 2.0), z, -(0.6f64).atan2(0.8));
    for k in 0..8 {
        let psi = f64::from(k) * PI / 4.0;
        let b = rotated(&tilted, y, psi);
        builds_right_or_refuses(
            &format!("hemisphere, prism notched at the pole, turned {k}π/4"),
            &hemi,
            &b,
            (whole, prism_vol, cap),
        );
    }
}

/// Every op of `(a, b)` against the closed forms `va`, `vb` and the
/// intersection `vi`: an answer that builds is one sound body at its
/// volume, `Empty` only where the closed form is nothing, and any other
/// answer a typed refusal that is not a kernel invariant.
fn builds_right_or_refuses(
    label: &str,
    a: &Body<f64>,
    b: &Body<f64>,
    (va, vb, vi): (f64, f64, f64),
) {
    let fa = sweep::test_support::finished("A", a.clone(), tol());
    let fb = sweep::test_support::finished("B", b.clone(), tol());
    let mut all = vec![("A ∪ B", topo::union(&fa, &fb, tol()))];
    all.extend(subtract_both_orders_and_intersect(a, b));
    // A copy within the band glues, which moves a volume by up to its
    // area times the band: allow tens of bands over the unit bodies here.
    let slack = (1e-6 + 50.0 * tol().eps()) * va.max(vb);
    for (op, r) in all {
        let want = match op {
            "A ∪ B" => va + vb - vi,
            "A ∖ B" => va - vi,
            "B ∖ A" => vb - vi,
            _ => vi,
        };
        let label = format!("{label}, {op}");
        match r {
            Ok(BooleanResult::Body(bb)) => {
                let m = topo::test_support::pinned_volume(&label, &bb.body, tol());
                let t3 = topo::validate_geometric(&bb.body, tol()).is_ok();
                let t3p = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()).is_ok();
                println!(
                    "PROBE {label}: want {want:.9}; Body {:.9} shells {} t3 {t3} t3' {t3p}",
                    m.volume,
                    bb.body.shells().count()
                );
                assert!(t3 && t3p, "{label}: tier 3 and 3′");
                assert!(
                    (m.volume - want).abs() <= m.volume_pad + slack,
                    "{label}: volume {} against {want}",
                    m.volume
                );
            }
            Ok(BooleanResult::Empty) => {
                println!("PROBE {label}: want {want:.9}; Empty");
                assert!(want.abs() <= slack, "{label}: Empty against {want}");
            }
            Err(topo::BooleanError::ClassificationInvariant { what }) => {
                panic!("{label}: a kernel invariant: {what}")
            }
            Err(e) => println!("PROBE {label}: want {want:.9}; Err {e:?}"),
        }
    }
}

fn frustum(top_y: f64, scale: f64) -> Body<f64> {
    // The cone of radius 1 at y = 0 and apex at y = 2, cut at `top_y`.
    let r = 1.0 - top_y / 2.0;
    revolved_about_y(
        sweep::test_support::corners(&[
            (0.0, 0.0),
            (scale, 0.0),
            (scale * r, scale * top_y),
            (0.0, scale * top_y),
        ]),
        sweep::Revolution::Full,
        tol(),
    )
}

fn frustum_volume(top_y: f64, scale: f64) -> f64 {
    let r = 1.0 - top_y / 2.0;
    PI * top_y / 3.0 * (1.0 + r + r * r) * scale.powi(3)
}

/// **Claim 1's probes**: bodies sharing part of a carrier with the
/// other, copies scaled across the band, and a third operand.
#[test]
fn a_partly_shared_or_scaled_curved_operand_builds_right_or_refuses() {
    use sweep::test_support::{ball_poled_z, dome, lantern};
    let eps = tol().eps();
    let ball = |r: f64| ball_poled_z(r, Vec3::new(0.0, 0.0, 0.0), tol());
    let vball = |r: f64| 4.0 * PI * r.powi(3) / 3.0;
    let a45 = core::f64::consts::FRAC_1_SQRT_2;
    let vdome = PI * (0.75 * a45 - a45.powi(3) / 3.0);
    let (vh, vb1) = (2.0 * PI / 3.0, vball(1.0));
    builds_right_or_refuses("ball, hemisphere", &ball(1.0), &hemisphere(), (vb1, vh, vh));
    builds_right_or_refuses("hemisphere, ball", &hemisphere(), &ball(1.0), (vh, vb1, vh));
    builds_right_or_refuses(
        "dome, ball",
        &dome(1.0, tol()),
        &ball(1.0),
        (vdome, vb1, vdome),
    );
    builds_right_or_refuses(
        "dome, hemisphere",
        &dome(1.0, tol()),
        &hemisphere(),
        (vdome, vh, vdome),
    );
    let (s, t) = (frustum_volume(1.0, 1.0), frustum_volume(1.5, 1.0));
    builds_right_or_refuses(
        "frustum, taller frustum",
        &frustum(1.0, 1.0),
        &frustum(1.5, 1.0),
        (s, t, s),
    );
    builds_right_or_refuses(
        "taller frustum, frustum",
        &frustum(1.5, 1.0),
        &frustum(1.0, 1.0),
        (t, s, s),
    );
    for k in [0.5, 1.0, 10.0] {
        let g = 1.0 + k * eps;
        builds_right_or_refuses(
            &format!("ball, ball scaled 1 + {k}ε"),
            &ball(1.0),
            &ball(g),
            (vb1, vball(g), vb1),
        );
        builds_right_or_refuses(
            &format!("frustum, frustum scaled 1 + {k}ε"),
            &frustum(1.0, 1.0),
            &frustum(1.0, g),
            (s, frustum_volume(1.0, g), s),
        );
    }
    let x = Vec3::new(0.0, 1.0, 0.0);
    let vl = PI * (0.6 - 0.6f64.powi(3) / 3.0) + PI * 0.6 / 3.0 * (0.64 + 0.16 + 0.04);
    for angle in [0.3, 2.0] {
        builds_right_or_refuses(
            &format!("lantern, lantern turned {angle}"),
            &lantern(tol()),
            &rotated(&lantern(tol()), x, angle),
            (vl, vl, vl),
        );
    }
    // A third operand: the union of the ball with its copy, against the
    // ball again.
    let fa = sweep::test_support::finished("A", ball(1.0), tol());
    match topo::union(&fa, &fa, tol()) {
        Ok(BooleanResult::Body(u)) => {
            builds_right_or_refuses("(ball ∪ ball), ball", &u.body, &ball(1.0), (vb1, vb1, vb1));
            builds_right_or_refuses(
                "(ball ∪ ball), ball turned",
                &u.body,
                &rotated(&ball(1.0), Vec3::new(1.0, 1.0, 0.0).normalize(), 0.7),
                (vb1, vb1, vb1),
            );
        }
        r => println!("PROBE ball ∪ ball: {r:?}"),
    }
    let ff = sweep::test_support::finished("A", frustum(1.0, 1.0), tol());
    match topo::intersect(&ff, &ff, tol()) {
        Ok(BooleanResult::Body(u)) => {
            builds_right_or_refuses(
                "(frustum ∩ frustum), taller frustum",
                &u.body,
                &frustum(1.5, 1.0),
                (s, t, s),
            );
        }
        r => println!("PROBE frustum ∩ frustum: {r:?}"),
    }
}
