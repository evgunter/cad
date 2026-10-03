//! **The snowman**: two balls of revolution on one axis, a big one
//! below and a smaller one above, under all three booleans — and the
//! union's waist rolled into a torus band.
//!
//! Each ball is the full revolve of a semicircle about the shared `y`
//! axis, both sketched in one plane. The two spheres meet in the
//! circle on their radical plane, and the crossing layer's
//! circle × sphere roots and the sphere pair's join carry every op to
//! it. The union's waist is a concave crease between two spheres on
//! distinct centres, which is the configuration
//! `BlendArm::SphereSphereTorus` names: the ball rolling in it stays
//! outside both, at `Rᵢ + r` from each centre, so its spine is a level
//! circle and the band is an exact torus.
//!
//! **What this pose is, and the poses beside it.** Coaxial is what a
//! snowman is, and it is the pose the scene draws; `tests` pins the
//! poses beside it, under all three ops. A head moved OFF the shared
//! axis 0.05 along `x`, in the plane both semicircles are sketched in,
//! tilts the radical plane against the spheres' polar axes. It BUILDS:
//! the join selects the section's arc by the run's side
//! (`work/reach/tilted-sphere-pair-section-refuses-at-the-polar-gate.md`),
//! the sphere faces it leaves are measured by Gauss–Bonnet, and every op
//! passes tier 3 at the two-ball closed form. Moved 0.05 along `z`
//! instead, the tilted section misses both seams and passes through the
//! faces as a ring, so it refuses `SectionArcSide { NoCertifiedRun }`
//! (`work/tang/a-tilted-sphere-sections-pierce-ring-has-no-run-side-arm.md`).
//! A head SPUN 0.9 rad about the shared axis, so the two revolves' seams
//! are no longer coplanar, puts the level section through the head's face
//! as a ring too; that ring builds, its chords' arcs chosen by the face's
//! window, and every op meets the coaxial closed form.
//!
//! **The waist is selected by description, and the description is
//! ambiguous.** `edge_adjacent_matches(Sphere, Sphere)` names the
//! waist's arcs AND every seam meridian of either ball, because a
//! meridian's two sides are two half-bands of one sphere. The kind
//! vocabulary has no atom for "two different surfaces" or for a
//! concave edge (SELECT-DESIGN GS-Q2 reserves `Convex`/`Reflex`
//! unbuilt), so the scene separates them through `query::rim_of`,
//! which refuses a co-surface seed by name. That residue is a library
//! finding (`work/tquery/adjacent-kinds-cannot-tell-a-crease-from-a-co-surface-seam.md`).
//!
//! **Every oracle here is closed-form, not bit-exact.** The radii are
//! the ones a person picks, so the volumes are irrational and are held
//! to a relative slack: the caps the radical plane cuts for the three
//! booleans, and for the fillet the union plus the band's ΔV by Pappus
//! on its meridian section.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::authoring::{p2, v3, validated};
use pncad::geom::Surface;
use pncad::geom_core::{Affine3, Point2, Tol, Vec2};
use pncad::prelude::{Open, Start, SurfaceKind, SurfaceKindSet, fillet_edges, query};
use pncad::profile::{ArcSweep, Center, ConstructedLoop, SketchPlane};
use pncad::sweep::{Revolution, RevolveAxis, revolve};
use pncad::topo::query::RimError;
use pncad::topo::{Body, BooleanBody, BooleanError, BooleanResult, EdgeKey};

use crate::{SceneBody, Stop, View};

/// The bottom ball's radius, centred at the origin.
const R1: f64 = 0.3;
/// The head's radius.
const R2: f64 = 0.2;
/// The head's centre height above the bottom ball's.
const D: f64 = 0.4;
/// The waist fillet's radius.
const ROLL: f64 = 0.05;
/// The spacing between the cell's bodies, along `x`.
const STEP: f64 = 0.8;
/// The scene's chord budget.
const DELTA: f64 = 2e-3;
/// The relative slack every volume is held to against its closed form.
/// Every face is a sphere or a torus, which the mass properties
/// integrate in closed form (`assert_volume` pins a zero pad), so the
/// slack is rounding's, and a dropped cap or a band counted twice
/// misses by orders of magnitude.
const SLACK: f64 = 1e-12;

/// A semicircle about `(0, y)` of radius `r`, closed along the axis.
fn semicircle(r: f64, y: f64, tol: Tol) -> ConstructedLoop<f64> {
    Open.at(Point2::new(0.0, y - r))
        .arc_to(
            Center {
                c: Point2::new(0.0, y),
                winding: ArcSweep::Ccw,
                p: Point2::new(0.0, y + r),
            },
            tol,
        )
        .expect("a semicircle about a centre on the axis")
        .line_to(Start, tol)
        .expect("the axis closes the semicircle")
        .into()
}

/// A ball: the semicircle fully revolved about `y`.
fn ball(r: f64, y: f64, tol: Tol) -> Body<f64> {
    let profile = validated(SketchPlane::xy(), vec![semicircle(r, y, tol)], tol)
        .expect("the semicircle validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("the semicircle fully revolves")
    .body
}

fn seamed(what: &str, out: Result<BooleanResult<f64>, BooleanError>) -> BooleanBody<f64> {
    match out.unwrap_or_else(|e| panic!("{what} refused: {e:?}")) {
        BooleanResult::Body(bb) => bb,
        BooleanResult::Empty => panic!("{what} came back empty"),
    }
}

fn ball_volume(r: f64) -> f64 {
    4.0 / 3.0 * PI * r.powi(3)
}

/// A spherical cap of height `h` on a sphere of radius `r`.
fn cap_volume(r: f64, h: f64) -> f64 {
    PI * h.powi(2) * (3.0 * r - h) / 3.0
}

/// The waist's height: the radical plane of the two spheres.
fn waist_height() -> f64 {
    (D * D + R1 * R1 - R2 * R2) / (2.0 * D)
}

/// The lens the balls share: each one's cap beyond the radical plane.
fn lens_volume() -> f64 {
    lens_volume_at(D)
}

/// The lens two balls of radii [`R1`] and [`R2`] share with their
/// centres `d` apart: each one's cap beyond the radical plane, which
/// sits `(d² + R1² − R2²) / 2d` from the bottom centre.
fn lens_volume_at(d: f64) -> f64 {
    let x = (d * d + R1 * R1 - R2 * R2) / (2.0 * d);
    cap_volume(R1, R1 - x) + cap_volume(R2, R2 - (d - x))
}

/// The rolling ball's centre in the meridian `(ρ, y)`: at `R1 + r`
/// from the bottom centre and `R2 + r` from the head's.
fn roll_centre() -> (f64, f64) {
    let y = ((R1 + ROLL).powi(2) - (R2 + ROLL).powi(2) + D * D) / (2.0 * D);
    (((R1 + ROLL).powi(2) - y * y).sqrt(), y)
}

/// `∫ ρ² dy` along the arc of the circle about `c` of radius `s` from
/// `p` to `q`, the short way round. With `ρ = a + s cos θ` and
/// `y = b + s sin θ` the integrand is a polynomial in `cos θ`, and its
/// antiderivative is closed-form.
fn arc_rho2_dy(c: (f64, f64), s: f64, p: (f64, f64), q: (f64, f64)) -> f64 {
    let a = c.0;
    let f = |t: f64| {
        a * a * s * t.sin()
            + a * s * s * (t + t.sin() * t.cos())
            + s.powi(3) * (t.sin() - t.sin().powi(3) / 3.0)
    };
    let t0 = (p.1 - c.1).atan2(p.0 - c.0);
    let t1 = (q.1 - c.1).atan2(q.0 - c.0);
    let mut dt = t1 - t0;
    if dt > PI {
        dt -= 2.0 * PI;
    } else if dt <= -PI {
        dt += 2.0 * PI;
    }
    f(t0 + dt) - f(t0)
}

/// The volume the fillet ADDS: the meridian section between the two
/// spheres' arcs from the waist to the tangent points and the rolling
/// ball's arc between those, revolved. By Green's theorem
/// `∬ ρ dA = ½ ∮ ρ² dy` counter-clockwise, so Pappus's `2π ∬ ρ dA` is
/// `π ∮ ρ² dy` over its three arcs.
fn band_delta_v() -> f64 {
    let x = waist_height();
    let waist = ((R1 * R1 - x * x).sqrt(), x);
    let c = roll_centre();
    let t1 = (c.0 * R1 / (R1 + ROLL), c.1 * R1 / (R1 + ROLL));
    let t2 = (c.0 * R2 / (R2 + ROLL), D + (c.1 - D) * R2 / (R2 + ROLL));
    PI * (arc_rho2_dy((0.0, 0.0), R1, waist, t1)
        + arc_rho2_dy(c, ROLL, t1, t2)
        + arc_rho2_dy((0.0, D), R2, t2, waist))
}

/// The body's volume against `expected`, read off closed-form faces
/// only; returns the relative residual.
fn assert_volume(what: &str, body: &Body<f64>, expected: f64, tol: Tol) -> f64 {
    let p = pncad::topo::mass_properties(body, tol)
        .unwrap_or_else(|e| panic!("{what}: mass properties, got {e:?}"));
    assert_eq!(p.volume_pad, 0.0, "{what}: closed-form faces only");
    let v = p.volume;
    let residual = (v - expected).abs() / expected;
    assert!(
        residual <= SLACK,
        "{what}: volume {v} against the closed form {expected}"
    );
    residual
}

/// The waist, said by description: the edges between two sphere
/// faces, then `rim_of` on each, which hands back the whole rim for an
/// arc on two surfaces and refuses a seam meridian `CoSurface`.
///
/// The census is pinned because the filed finding quotes it: the
/// description names the waist's two arcs and four co-surface
/// meridians, two per ball.
fn waist(body: &Body<f64>) -> Vec<EdgeKey> {
    let spheres = SurfaceKindSet::just(SurfaceKind::Sphere);
    let named: Vec<EdgeKey> = query::all_edges(body)
        .into_iter()
        .filter(|&e| query::edge_adjacent_matches(body, e, spheres, spheres))
        .collect();
    let mut rim: Option<Vec<EdgeKey>> = None;
    let mut meridians = 0;
    for &e in &named {
        match query::rim_of(body, e) {
            Ok(found) => match &rim {
                Some(held) => assert!(
                    held.contains(&e),
                    "the union has one waist rim; {e:?} names a second"
                ),
                None => rim = Some(found),
            },
            Err(RimError::CoSurface { .. }) => meridians += 1,
            Err(other) => panic!("{e:?} is neither a waist arc nor a seam meridian: {other:?}"),
        }
    }
    let rim = rim.expect("the union has a waist rim");
    assert_eq!(
        (named.len(), rim.len(), meridians),
        (6, 2, 4),
        "(Sphere, Sphere) names two waist arcs and four co-surface meridians"
    );
    rim
}

/// The cell's bodies sit side by side along `x`.
fn placed(body: &Body<f64>, slot: f64, tol: Tol) -> Body<f64> {
    pncad::topo::transform_rigid(body, &Affine3::translation(v3(slot * STEP, 0.0, 0.0)), tol)
        .expect("a translation is rigid")
}

/// A boolean result, placed in its slot and routed by
/// [`crate::declares_no_contacts`].
fn scene_body(name: &str, color: [f64; 3], bb: BooleanBody<f64>, slot: f64, tol: Tol) -> SceneBody {
    let body = placed(&bb.body, slot, tol);
    if crate::declares_no_contacts(&bb.contacts) {
        SceneBody::plain(name, color, body)
    } else {
        SceneBody::seamed(name, color, body, bb.contacts)
    }
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let (bottom, head) = (ball(R1, 0.0, tol), ball(R2, D, tol));
    let snowman = seamed("bottom ∪ head", pncad::topo::union(&bottom, &head, tol));
    let bitten = seamed("bottom ∖ head", pncad::topo::subtract(&bottom, &head, tol));
    let lens = seamed("bottom ∩ head", pncad::topo::intersect(&bottom, &head, tol));

    let (va, vb, vl) = (ball_volume(R1), ball_volume(R2), lens_volume());
    let mut worst = [
        assert_volume("bottom ∪ head", &snowman.body, va + vb - vl, tol),
        assert_volume("bottom ∖ head", &bitten.body, va - vl, tol),
        assert_volume("bottom ∩ head", &lens.body, vl, tol),
    ]
    .into_iter()
    .fold(0.0, f64::max);

    let rim = waist(&snowman.body);
    let rolled = fillet_edges(&snowman.body, &rim, ROLL, tol)
        .unwrap_or_else(|e| panic!("the snowman's waist fillets: {e:?}"));
    let dv = band_delta_v();
    worst = worst.max(assert_volume(
        "the filleted snowman",
        &rolled.body,
        va + vb - vl + dv,
        tol,
    ));

    let (rho, y) = roll_centre();
    assert!(!rolled.band_faces.is_empty(), "the fillet mints a band");
    for &f in &rolled.band_faces {
        let face = rolled.body.get_face(f).expect("a band face");
        match *rolled.body.get_surface(face.surface).expect("its surface") {
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                ..
            } => {
                assert!(
                    axis.x == 0.0 && axis.z == 0.0 && axis.y.abs() == 1.0,
                    "the band's axis is the snowman's, ±y; got {axis:?}"
                );
                assert!(
                    center.x == 0.0 && center.z == 0.0 && (center.y - y).abs() < 1e-12,
                    "the spine is centred on the axis at height {y}, got {center:?}"
                );
                assert!(
                    (major_radius - rho).abs() < 1e-12,
                    "the spine's radius is {rho}, got {major_radius}"
                );
                assert!(
                    (minor_radius - ROLL).abs() < 1e-15,
                    "the tube's radius is {ROLL}, got {minor_radius}"
                );
            }
            ref other => panic!("the waist band is a torus, got {other:?}"),
        }
    }
    println!(
        "   [snowman] the waist: {} arc(s) named by (Sphere, Sphere) and kept by rim_of, \
         4 seam meridians refused CoSurface; {} band face(s), spine at y = \
         {y:.6}, ρ = {rho:.6}; ΔV = {dv:.6e} m³; worst relative volume residual \
         {worst:.1e}",
        rim.len(),
        rolled.band_faces.len()
    );

    let snow = [0.86, 0.89, 0.94];
    vec![Stop {
        name: "snowman",
        caption: "snowman: ∪ rolled, ∪, ∖, ∩".to_string(),
        montage: true,
        story: "two coaxial balls of revolution under union, subtract and intersect, and \
                the union's waist rolled into an exact torus band",
        ops: "revolve(semicircle, +y, Full) twice; union / subtract / intersect; \
              fillet_edges(union, waist rim, r = 0.05) — the rim said as (Sphere, Sphere) \
              through rim_of",
        delta: DELTA,
        note: Some(format!(
            "R = 0.3 below, 0.2 above, centres 0.4 apart, so they meet in a circle on the \
             radical plane at y = {x:.6}; every volume meets the spherical-cap closed form, \
             and the rolled waist's meets it plus the band's Pappus ΔV = {dv:.4e} m³. The \
             band is a torus on BlendArm::SphereSphereTorus, its spine at {R1} + r and \
             {R2} + r from the two centres. A head moved off the axis in the seam plane \
             builds too, tilted section and all, and so does one spun about the axis; one \
             moved out of the seam plane still stops at its tilted section's ring",
            x = waist_height()
        )),
        view: View {
            elev: 18.0,
            azim: -90.0,
            up: 'y',
        },
        bodies: vec![
            SceneBody::plain("snowman_rolled", snow, placed(&rolled.body, 0.0, tol)),
            scene_body("snowman_union", snow, snowman, 1.0, tol),
            scene_body("snowman_bitten", [0.55, 0.75, 0.95], bitten, 2.0, tol),
            scene_body("snowman_lens", [0.85, 0.55, 0.25], lens, 3.0, tol),
        ],
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The scene's oracles, off the render walk: every volume against
    /// its closed form, the waist selection's census, and the band's
    /// torus.
    #[test]
    fn the_snowman_meets_its_closed_forms() {
        stops(Tol::witness());
    }

    /// **A head moved off the axis IN the balls' seam plane builds.**
    /// Moved 0.05 along `x`, the head tilts the radical plane against
    /// both balls' polar axes, and the section crosses the seams the two
    /// revolves share. The run-side arc rule and the Gauss–Bonnet sphere
    /// arm carry it (the tilted sphere pair). Every op passes tier 3 and
    /// meets the two-ball closed form at the moved centre distance.
    #[test]
    fn a_head_moved_off_the_axis_in_the_seam_plane_builds_to_its_closed_form() {
        let tol = Tol::witness();
        let (bottom, head) = (ball(R1, 0.0, tol), ball(R2, D, tol));
        let moved =
            pncad::topo::transform_rigid(&head, &Affine3::translation(v3(0.05, 0.0, 0.0)), tol)
                .expect("a rigid pose");
        let (va, vb, vl) = (
            ball_volume(R1),
            ball_volume(R2),
            lens_volume_at(D.hypot(0.05)),
        );
        for (op, out, expected) in [
            ("∪", pncad::topo::union(&bottom, &moved, tol), va + vb - vl),
            ("∖", pncad::topo::subtract(&bottom, &moved, tol), va - vl),
            ("∩", pncad::topo::intersect(&bottom, &moved, tol), vl),
        ] {
            let label = format!("head moved 0.05 along x, bottom {op} head");
            let bb = seamed(&label, out);
            pncad::topo::validate_geometric(&bb.body, tol)
                .unwrap_or_else(|e| panic!("{label}: tier 3, {e:?}"));
            assert_volume(&label, &bb.body, expected, tol);
        }
    }

    /// **A head spun about the shared axis builds.** Spun 0.9 rad, the
    /// head's seam leaves the bottom ball's plane, and the level section
    /// passes through the head's face as a ring; its chords take their
    /// arcs from the face's window. Every op passes tier 3 and meets the
    /// coaxial closed form, which the spin does not move.
    #[test]
    fn a_head_spun_about_the_axis_builds_to_its_closed_form() {
        let tol = Tol::witness();
        let (bottom, head) = (ball(R1, 0.0, tol), ball(R2, D, tol));
        let spin = Affine3::rotation_about_axis(
            pncad::geom_core::Point3::origin(),
            v3(0.0, 1.0, 0.0),
            0.9,
        );
        let moved = pncad::topo::transform_rigid(&head, &spin, tol).expect("a rigid pose");
        let (va, vb, vl) = (ball_volume(R1), ball_volume(R2), lens_volume_at(D));
        for (op, out, expected) in [
            ("∪", pncad::topo::union(&bottom, &moved, tol), va + vb - vl),
            ("∖", pncad::topo::subtract(&bottom, &moved, tol), va - vl),
            ("∩", pncad::topo::intersect(&bottom, &moved, tol), vl),
        ] {
            let label = format!("head spun 0.9 rad, bottom {op} head");
            let bb = seamed(&label, out);
            pncad::topo::validate_geometric(&bb.body, tol)
                .unwrap_or_else(|e| panic!("{label}: tier 3, {e:?}"));
            assert_volume(&label, &bb.body, expected, tol);
        }
    }

    /// **A head moved out of the seam plane stops at its tilted
    /// section's ring**, under every op. Moved 0.05 along `z`, the tilted
    /// section passes through the balls' faces without crossing either
    /// seam, so the join is cross-loop and closes no run for the run-side
    /// rule to read (`SectionArcSide { NoCertifiedRun }`,
    /// `work/tang/a-tilted-sphere-sections-pierce-ring-has-no-run-side-arm.md`).
    /// A pose that starts building, or refuses elsewhere, means the
    /// narration is stale.
    #[test]
    fn the_head_moved_out_of_the_seam_plane_refuses_at_the_tilted_ring() {
        use pncad::topo::{ArcSideCase, SplitJoinError};
        let tol = Tol::witness();
        let (bottom, head) = (ball(R1, 0.0, tol), ball(R2, D, tol));
        let moved =
            pncad::topo::transform_rigid(&head, &Affine3::translation(v3(0.0, 0.0, 0.05)), tol)
                .expect("a rigid pose");
        for (op, out) in [
            ("∪", pncad::topo::union(&bottom, &moved, tol)),
            ("∖", pncad::topo::subtract(&bottom, &moved, tol)),
            ("∩", pncad::topo::intersect(&bottom, &moved, tol)),
        ] {
            match out {
                Err(BooleanError::Join(SplitJoinError::SectionArcSide {
                    case: ArcSideCase::NoCertifiedRun,
                    ..
                })) => {}
                Err(e) => panic!("head moved along z, {op}: refused elsewhere, {e:?}"),
                Ok(_) => panic!("head moved along z, {op}: builds now — retell the narration"),
            }
        }
    }
}
