//! **Two spheres crossing in a circle no edge reaches build.** A ball
//! whose seam lies inside another ball's face meets that face in a
//! whole circle, off every edge of either operand, so the crossing
//! layer finds nothing and the containment fallback's sphere extent
//! scan meets two faces that cross. The scan re-cuts each sphere so an
//! edge of each operand reaches the circle (`boolean::ops`, the sphere
//! arm of `sphere_extent_scan`):
//!
//! - a closed ball is re-charted about its own centre with its pole on
//!   the centre line, so its seam meridians cross the circle; the
//!   result keeps two faces per sphere and meshes;
//! - a trimmed face, or a closed ball crossed along two non-parallel
//!   axes, is cut along its own chart's meridian through each circle;
//!   the circle then lies tilted against the face's chart, and such a
//!   face meshes through the trimmed lane's sphere arm.
//!
//! Every ball here is the `y`-poled full revolve, its two seam
//! meridians in the plane `z = c_z`, so a centre line along `z` keeps
//! both seams off the circle at every depth. Every pose runs every op
//! in both member orders through `differential::outcome` (tiers 2 and
//! 3′, the certificate, the legal-operand check, the volume) against
//! the lens in its textbook closed form, which never forms the radical
//! plane, and through the mesher.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Band, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled_y, brick, finished};
use topo::{AtRestBody, BooleanError, BooleanResult};

use crate::common::differential::outcome;
use crate::common::oracles::{ball_lens as lens, ball_volume, cap_volume};

use Want::{Budget, Draws, Folds, Refuses};

/// A `y`-poled ball of radius `r` turned by `turn`, then moved to `c`.
fn placed(r: f64, turn: Affine3<f64>, c: Vec3<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let at = ball_poled_y(r, Vec3::new(0.0, 0.0, 0.0), tol);
    let at = topo::transform_rigid(&at, &geom_core::test_support::rigid(&turn, tol), tol).unwrap();
    let moved =
        topo::transform_rigid(&at, &geom_core::RigidMotion::translation(c).unwrap(), tol).unwrap();
    finished("the ball", moved, tol)
}

/// A `y`-poled ball of radius `r` at `c`, its seam meridians in `z = c_z`.
fn ball(r: f64, c: Vec3<f64>) -> AtRestBody<f64> {
    placed(r, Affine3::identity(), c)
}

/// A `y`-poled ball spun `spin` about `z`, then turned so its local `z`
/// points along `dir`, at `dist·dir`: its seam meridians lie in the
/// plane through its centre normal to `dir`, so they stay off any
/// circle about the centre line `dir`.
fn toward(r: f64, dist: f64, dir: Vec3<f64>, spin: f64) -> AtRestBody<f64> {
    let dir = dir / dir.norm();
    let (o, z) = (Point3::origin(), Vec3::new(0.0, 0.0, 1.0));
    let across = z.cross(dir);
    let tilt = if across.norm() < 1e-12 {
        Affine3::identity()
    } else {
        Affine3::rotation_about_axis(o, across / across.norm(), z.dot(dir).acos())
    };
    placed(
        r,
        tilt * Affine3::rotation_about_axis(o, z, spin),
        dir * dist,
    )
}

fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished("the box", brick(x, y, z, Tol::witness()), Tol::witness())
}

fn body(out: Result<BooleanResult<f64>, BooleanError>) -> AtRestBody<f64> {
    out.unwrap().body().unwrap().body.clone()
}

fn unit() -> AtRestBody<f64> {
    ball(1.0, Vec3::new(0.0, 0.0, 0.0))
}

/// What one op yields.
#[derive(Clone, Copy, Debug)]
enum Want {
    /// `SOUND`, and the body meshes and checks.
    Draws,
    /// `SOUND`, and the mesher refuses a triangle folded over the chord
    /// beside it near a chart pole
    /// (`work/tess/a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm.md`).
    Folds,
    /// `SOUND`, and the mesher refuses at once: the tilted circle passes
    /// so near a chart pole that its uniform chord schedule asks one edge
    /// for more chords than the sphere arm's budget
    /// (`SphereTrimShape { ChordBudget }`).
    Budget,
    /// A typed refusal the predicate accepts.
    Refuses(fn(&BooleanError) -> bool),
}

/// One op's `out` against `want`, its volume against `volume` to
/// `1e-9` relative, over a floor of `1e-15` of the operands' volumes
/// `scale` (a difference of nearly equal balls is a cancellation).
fn check(
    label: &str,
    out: Result<BooleanResult<f64>, BooleanError>,
    (volume, scale): (f64, f64),
    want: Want,
) {
    let tol = Tol::witness();
    if let Refuses(accepts) = want {
        match &out {
            Err(e) if accepts(e) => return,
            _ => panic!("{label}: wanted a typed refusal, got {out:?}"),
        }
    }
    let Ok(Some(bb)) = out.as_ref().map(BooleanResult::body) else {
        panic!("{label}: wanted a body, got {out:?}");
    };
    match want {
        Draws => {
            let m = mesh::tessellate(&bb.body, 5e-3, tol)
                .unwrap_or_else(|e| panic!("{label}: the mesh: {e:?}"));
            mesh::validate::check_mesh(&m).unwrap_or_else(|e| panic!("{label}: the mesh: {e:?}"));
            crate::common::differential::assert_meshed_geometry(label, &bb.body, &m, 5e-3);
        }
        Folds => {
            let drawn = mesh::tessellate(&bb.body, 5e-3, tol);
            assert!(
                matches!(&drawn, Err(mesh::TessellateError::FoldedTriangle { .. })),
                "{label}: the folded-triangle refusal, got {:?}",
                drawn.map(|m| m.positions.len())
            );
        }
        Budget => {
            let drawn = mesh::tessellate(&bb.body, 5e-3, tol);
            assert!(
                matches!(
                    &drawn,
                    Err(mesh::TessellateError::SphereTrimShape {
                        cause: mesh::types::SphereTrimCause::ChordBudget,
                        ..
                    })
                ),
                "{label}: the chord-budget refusal, got {:?}",
                drawn.map(|m| m.positions.len())
            );
        }
        Refuses(_) => unreachable!(),
    }
    let v = topo::test_support::pinned_volume("the measured body", &bb.body, tol).volume;
    assert!(
        (v - volume).abs() <= 1e-9 * volume.abs() + 1e-15 * scale,
        "{label}: volume {v} against the closed form {volume}"
    );
    let line = outcome(out, volume, tol);
    assert!(line.starts_with("OK SOUND"), "{label}: {line}");
}

/// `a ∪ b`, `b ∪ a`, `a ∖ b`, `b ∖ a`, `a ∩ b`, `b ∩ a` against `wants`,
/// each volume read from the operands' `va`, `vb` and the volume they
/// share.
fn assert_six(
    pose: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    (va, vb, shared): (f64, f64, f64),
    wants: [Want; 6],
) {
    let tol = Tol::witness();
    let ops = [
        ("a ∪ b", va + vb - shared, topo::union(a, b, tol)),
        ("b ∪ a", va + vb - shared, topo::union(b, a, tol)),
        ("a ∖ b", va - shared, topo::subtract(a, b, tol)),
        ("b ∖ a", vb - shared, topo::subtract(b, a, tol)),
        ("a ∩ b", shared, topo::intersect(a, b, tol)),
        ("b ∩ a", shared, topo::intersect(b, a, tol)),
    ];
    for ((op, volume, out), want) in ops.into_iter().zip(wants) {
        check(&format!("{pose}, {op}"), out, (volume, va + vb), want);
    }
}

/// The unit ball at the origin against `small`, of radius `r` at
/// centre distance `d`.
fn against_the_unit_ball(pose: &str, small: &AtRestBody<f64>, r: f64, d: f64, wants: [Want; 6]) {
    assert_six(
        pose,
        &unit(),
        small,
        (ball_volume(1.0), ball_volume(r), lens(1.0, r, d)),
        wants,
    );
}

/// The item's two witnesses. The plain pair re-charts both balls and
/// meshes. In the lens union the small ball crosses one trimmed sphere
/// face of the union (and clears its other sphere, so the two share the
/// one lens): the small ball is re-charted, the union's face is cut in.
#[test]
fn both_witnesses_build_under_every_boolean() {
    against_the_unit_ball(
        "plain",
        &ball(0.3, Vec3::new(0.0, 0.0, 0.95)),
        0.3,
        0.95,
        [Draws; 6],
    );
    let tol = Tol::witness();
    let lens_union = body(topo::union(
        &ball(1.0, Vec3::new(2.0, 2.0, 0.5)),
        &ball(1.0, Vec3::new(3.4, 2.0, 0.5)),
        tol,
    ));
    assert_six(
        "the lens union",
        &lens_union,
        &ball(0.3, Vec3::new(2.0, 2.0, 1.45)),
        (
            2.0 * ball_volume(1.0) - lens(1.0, 1.0, 1.4),
            ball_volume(0.3),
            lens(1.0, 0.3, 0.95),
        ),
        [Draws; 6],
    );
}

/// The plain witness at the `Interval` scalar refuses every op by name.
/// Re-charted along the centre line, each ball's seam meridians lie in
/// planes holding the other ball's centre, where the circle × sphere
/// roots' phase is the first-harmonic door's degenerate case: `f64`
/// decides it, and an enclosure of it is as wide as the circle.
#[test]
fn the_plain_witness_escalates_its_seam_crossing_at_the_interval_scalar() {
    use crate::common::interval::iv;
    let tol = Tol::witness();
    let ball_iv = |r: f64, z: f64| {
        finished(
            "the ball",
            ball_poled_y(iv(r), Vec3::new(iv(0.0), iv(0.0), iv(z)), tol),
            tol,
        )
    };
    let (a, b) = (ball_iv(1.0, 0.0), ball_iv(0.3, 0.95));
    for (op, out) in [
        ("a ∪ b", topo::union(&a, &b, tol)),
        ("a ∖ b", topo::subtract(&a, &b, tol)),
        ("b ∖ a", topo::subtract(&b, &a, tol)),
        ("a ∩ b", topo::intersect(&a, &b, tol)),
    ] {
        assert!(
            matches!(
                &out,
                Err(BooleanError::Escalated {
                    decision: topo::BooleanDecision::Crossing(_),
                    diag,
                }) if diag.predicate == Some("bool_wall_root_in_span")
            ),
            "Interval {op}: wanted the seam crossing's escalation, got {:?}",
            out.map(|_| "a body")
        );
    }
}

/// The small ball's radius, from a pebble to one larger than the unit
/// ball, against its centre's height across the whole crossing range
/// `(|1 − r|, 1 + r)`.
#[test]
fn a_ball_on_the_centre_line_builds_at_every_radius_and_depth() {
    for r in [0.05, 0.3, 0.7, 1.0, 1.5] {
        let (lo, hi) = ((1.0f64 - r).abs(), 1.0 + r);
        for t in [0.02, 0.25, 0.5, 0.75, 0.98] {
            let d = lo + t * (hi - lo);
            let small = ball(r, Vec3::new(0.0, 0.0, d));
            against_the_unit_ball(&format!("r {r}, d {d}"), &small, r, d, [Draws; 6]);
        }
    }
}

/// `10⁻⁴` and `10⁻⁶` in from either tangency every radius builds. The
/// depths are band-relative (at `ε = 10⁻⁶` they reach the escalation
/// gap, and at `10⁻¹²` the f64 placement frontier,
/// `work/apex/f64-cannot-place-a-shallow-crossing-within-the-finest-band.md`),
/// so the other ε rows stand down.
#[test]
fn a_ball_near_tangency_builds_at_the_default_band() {
    if Tol::witness().get().eps != 1e-9 {
        test_utils::vacuity::stood_down(
            "non-default eps",
            "the near-tangent depths are measured at the default band only",
        );
        return;
    }
    for r in [0.05, 0.3, 0.7, 1.5] {
        let (lo, hi) = ((1.0f64 - r).abs(), 1.0 + r);
        for d in [lo + 1e-4, lo + 1e-6, hi - 1e-4, hi - 1e-6] {
            let small = ball(r, Vec3::new(0.0, 0.0, d));
            against_the_unit_ball(&format!("r {r}, d {d}"), &small, r, d, [Draws; 6]);
        }
    }
}

/// Inside the band's escalation gap from either tangency the pair is
/// not re-cut: the nesting and gap questions escalate by name.
#[test]
fn a_depth_in_band_of_a_tangency_escalates() {
    use topo::{BooleanDecision, SphereQuestion};
    let band = Band::linear(Tol::witness()).unwrap();
    let e = (band.zero() + band.escalate()) / 2.0;
    for (d, question) in [
        (0.7 + e, SphereQuestion::Nested),
        (1.3 - e, SphereQuestion::Apart),
    ] {
        let out = topo::union(&unit(), &ball(0.3, Vec3::new(0.0, 0.0, d)), Tol::witness());
        assert!(
            matches!(
                out,
                Err(BooleanError::Escalated { decision: BooleanDecision::Sphere(q), .. })
                    if q == question
            ),
            "d {d}: wanted {question:?} escalated, got {:?}",
            out.map(|_| "a body")
        );
    }
}

/// A centre line off `z`, and a small ball whose chart is spun about
/// its centre line, put the two charts' poles out of parallel: each
/// ball is re-charted onto the centre line all the same.
#[test]
fn tilted_and_spun_charts_build() {
    let c = Vec3::new(0.02, 0.01, 0.9);
    against_the_unit_ball("tilted line", &ball(0.3, c), 0.3, c.norm(), [Draws; 6]);
    let z = Vec3::new(0.0, 0.0, 1.0);
    for spin in [0.5, 1.6] {
        let small = toward(0.3, 0.95, z, spin);
        against_the_unit_ball(&format!("spin {spin}"), &small, 0.3, 0.95, [Draws; 6]);
    }
    let small = toward(0.3, 0.95, Vec3::new(0.3, 0.2, 0.93), 0.7);
    against_the_unit_ball("spun on a tilted line", &small, 0.3, 0.95, [Draws; 6]);
}

/// A circle `10⁻⁵` and `10⁻⁷` rad from the unit ball's pole, and a
/// `10⁻³` ball centred on the unit sphere (its circle passes about
/// `5·10⁻⁷` from its own poles): the re-chart puts each pole on the
/// centre line, away from the circle. The distances are band-relative
/// (at `ε = 10⁻⁶` the `10⁻⁷` circle is inside the band, and at `10⁻¹²`
/// the `10⁻³` ball meets the f64 placement frontier,
/// `work/apex/f64-cannot-place-a-shallow-crossing-within-the-finest-band.md`),
/// so the other ε rows stand down.
#[test]
fn a_circle_beside_a_pole_builds() {
    if Tol::witness().get().eps != 1e-9 {
        test_utils::vacuity::stood_down(
            "non-default eps",
            "the circles beside a pole are measured at the default band only",
        );
        return;
    }
    let r = 0.01f64;
    let theta = (1.0 - r * r / 2.0).acos();
    for delta in [1e-5, 1e-7] {
        let a = theta + delta;
        let small = toward(r, 1.0, Vec3::new(0.0, a.cos(), a.sin()), 0.0);
        against_the_unit_ball(&format!("δ {delta:e}"), &small, r, 1.0, [Draws; 6]);
    }
    let tiny = toward(1e-3, 1.0, Vec3::new(0.0, 0.0, 1.0), 0.0);
    against_the_unit_ball("r 1e-3 on the sphere", &tiny, 1e-3, 1.0, [Draws; 6]);
}

/// Both spheres trimmed, each by a box across its own seam and clear of
/// the other ball: the cap the unit ball loses (`x > 0.9`) misses the
/// small ball, and the one the small ball loses (`x > 0.25`) lies
/// outside the unit ball, so the two still share one lens. Both faces
/// are cut in.
#[test]
fn two_trimmed_spheres_build() {
    let tol = Tol::witness();
    let big = body(topo::subtract(
        &unit(),
        &boxed((0.9, 2.0), (-2.0, 2.0), (-2.0, 2.0)),
        tol,
    ));
    let small = body(topo::subtract(
        &ball(0.3, Vec3::new(0.0, 0.0, 1.2)),
        &boxed((0.25, 2.0), (-2.0, 2.0), (-2.0, 3.0)),
        tol,
    ));
    assert_six(
        "two trimmed balls",
        &big,
        &small,
        (
            ball_volume(1.0) - cap_volume(1.0, 0.1),
            ball_volume(0.3) - cap_volume(0.3, 0.05),
            lens(1.0, 0.3, 1.2),
        ),
        [Draws; 6],
    );
}

/// The trimmed unit ball's face is cut along its meridian through a
/// circle passing `δ` from its pole, the meridian's boundary hit there
/// ordered against the circle's crossing by the arc between them. Every
/// `δ` builds, `10⁻⁷` included: the arc's pcurve beside the pole is the
/// projected image (C4), whose sector pieces refine toward the pole and
/// certify while the arc stays clear of it, and every op's volume holds.
/// At `10⁻³` the mesher draws every op. At `10⁻⁵` the uniform chord
/// schedule the circle asks near the pole draws the unit ball's
/// remnant in three ops and folds a triangle over the chord beside the
/// pole in the other three, which refuse typed ([`Want::Folds`]). At
/// `10⁻⁷` the schedule asks one edge for more chords than the sphere
/// arm's budget, and the mesher refuses at once ([`Want::Budget`]).
/// The distances are band-relative (at `ε = 10⁻⁶` the `10⁻⁵` circle's
/// crossings read the meridian's half in the escalation gap), so the
/// other ε rows stand down.
#[test]
fn a_trimmed_face_cut_beside_its_pole_builds() {
    if Tol::witness().get().eps != 1e-9 {
        test_utils::vacuity::stood_down(
            "non-default eps",
            "the cut beside a pole is measured at the default band only",
        );
        return;
    }
    let tol = Tol::witness();
    let trimmed = body(topo::subtract(
        &unit(),
        &boxed((0.9, 2.0), (-2.0, 2.0), (-2.0, 2.0)),
        tol,
    ));
    let r = 0.01f64;
    let theta = (1.0 - r * r / 2.0).acos();
    for delta in [1e-3, 1e-5, 1e-7] {
        let a = theta + delta;
        let small = toward(r, 1.0, Vec3::new(0.0, a.cos(), a.sin()), 0.0);
        assert_six(
            &format!("trimmed, δ {delta:e}"),
            &trimmed,
            &small,
            (
                ball_volume(1.0) - cap_volume(1.0, 0.1),
                ball_volume(r),
                lens(1.0, r, 1.0),
            ),
            if delta > 1e-4 {
                [Draws; 6]
            } else if delta > 1e-6 {
                [Folds, Folds, Folds, Draws, Draws, Draws]
            } else {
                [Budget; 6]
            },
        );
    }
}

/// The lens `ball(1, y = 0) ∩ ball(0.8, y = 1.4)` against a small ball
/// whose seam plane is normal to its centre line, at
/// `0.93·(0, cos 20°, sin 20°)`: its sphere crosses both of the lens's
/// sphere faces in whole circles along non-parallel axes, so no one
/// re-chart serves it and its own face is cut twice. The two caps it
/// keeps outside the lens are disjoint (it stays `0.25` from the circle
/// the lens's spheres meet in), so the shared volume is its two lenses
/// less itself once, and what it keeps outside the lens is those two
/// caps.
#[test]
fn a_ball_crossing_both_faces_of_a_lens_builds() {
    let tol = Tol::witness();
    let on_y = |r: f64, y: f64| ball(r, Vec3::new(0.0, y, 0.0));
    let lens_body = body(topo::intersect(&on_y(1.0, 0.0), &on_y(0.8, 1.4), tol));
    let (s, c) = 20f64.to_radians().sin_cos();
    let dir = Vec3::new(0.0, c, s);
    let to_top = (dir * 0.93 - Vec3::new(0.0, 1.4, 0.0)).norm();
    assert_six(
        "a ball across both lens faces",
        &lens_body,
        &toward(0.2, 0.93, dir, 0.0),
        (
            lens(1.0, 0.8, 1.4),
            ball_volume(0.2),
            lens(1.0, 0.2, 0.93) + lens(0.8, 0.2, to_top) - ball_volume(0.2),
        ),
        [Draws; 6],
    );
}

/// The unit ball crossed by two small balls of one operand. Along one
/// axis (`±z`) one re-chart serves both circles; along two (`z` and a
/// 40° tilt) the unit ball is cut in twice, and what lies inside it or
/// outside it is two lumps.
#[test]
fn a_ball_crossed_by_two_partners_builds() {
    let tol = Tol::witness();
    let z = Vec3::new(0.0, 0.0, 1.0);
    let (s, c) = 40f64.to_radians().sin_cos();
    let partners = |other: Vec3<f64>| {
        body(topo::union(
            &toward(0.3, 0.95, z, 0.0),
            &toward(0.3, 0.95, other, 0.0),
            tol,
        ))
    };
    let volumes = (
        ball_volume(1.0),
        2.0 * ball_volume(0.3),
        2.0 * lens(1.0, 0.3, 0.95),
    );
    assert_six(
        "partners at ±z",
        &unit(),
        &partners(-z),
        volumes,
        [Draws; 6],
    );
    assert_six(
        "partners at z and a tilt",
        &unit(),
        &partners(Vec3::new(-s, 0.0, -c)),
        volumes,
        [Draws; 6],
    );
}

/// A ball that pokes through a slab's face and crosses a sphere face
/// off every edge. Along one axis one re-chart serves both, and every
/// op builds; along two the plane's re-chart would graft the group
/// back under keys the sphere's cut no longer names, and every op
/// refuses typed.
#[test]
fn a_ball_escaping_through_a_plane_and_a_sphere() {
    let tol = Tol::witness();
    let tool = |partner: AtRestBody<f64>| {
        body(topo::union(
            &boxed((-3.0, 3.0), (-3.0, 3.0), (0.5, 3.0)),
            &partner,
            tol,
        ))
    };
    let volumes = (
        ball_volume(1.0),
        90.0 + ball_volume(0.3),
        cap_volume(1.0, 0.5) + lens(1.0, 0.3, 0.95),
    );
    assert_six(
        "one axis",
        &unit(),
        &tool(ball(0.3, Vec3::new(0.0, 0.0, -0.95))),
        volumes,
        [Draws; 6],
    );
    let renames = |e: &BooleanError| {
        matches!(
            e,
            BooleanError::FallbackExtentUnsupported { what, .. }
                if what.contains("rename the face the sphere's cut names")
        )
    };
    assert_six(
        "two axes",
        &unit(),
        &tool(toward(0.3, 0.95, Vec3::new(0.0, 0.6, -0.8), 0.0)),
        volumes,
        [Refuses(renames); 6],
    );
}

/// The unit ball against `ball(50)` `2·10⁻⁶` inside external tangency,
/// a band-relative pose read at the default band only. The union
/// builds and meshes at its closed form: the sphere arm reads the unit
/// ball's kept face by Gauss–Bonnet and asks no rim level, where the
/// iso arm read that level in band and refused it at the result gate.
/// The two lens-shaped differences build. The
/// intersection builds a sliver `2·10⁻⁶` thick, sound: its two caps
/// are small regions the sphere arm reads by the fan form, so its
/// volume lands within about `1e-9` of the lens in either order (it
/// read `−54%` and `−86%` before) and its at-infinity side decides, so
/// it is a legal operand (`work/cleave/a-built-sliver-is-not-a-legal-operand.md`).
#[test]
fn a_large_ball_near_external_tangency_is_pinned() {
    if Tol::witness().get().eps != 1e-9 {
        test_utils::vacuity::stood_down(
            "non-default eps",
            "the near-tangent sliver is measured at the default band only",
        );
        return;
    }
    let tol = Tol::witness();
    let d = 51.0 - 2e-6;
    let (a, b) = (unit(), ball(50.0, Vec3::new(0.0, 0.0, d)));
    let (va, vb, shared) = (ball_volume(1.0), ball_volume(50.0), lens(1.0, 50.0, d));
    for (op, out) in [
        ("a ∪ b", topo::union(&a, &b, tol)),
        ("b ∪ a", topo::union(&b, &a, tol)),
    ] {
        check(
            &format!("r 50, {op}"),
            out,
            (va + vb - shared, va + vb),
            Draws,
        );
    }
    check(
        "r 50, a ∖ b",
        topo::subtract(&a, &b, tol),
        (va - shared, va + vb),
        Draws,
    );
    check(
        "r 50, b ∖ a",
        topo::subtract(&b, &a, tol),
        (vb - shared, va + vb),
        Draws,
    );
    for (op, out) in [
        ("a ∩ b", topo::intersect(&a, &b, tol)),
        ("b ∩ a", topo::intersect(&b, &a, tol)),
    ] {
        let line = outcome(out, shared, tol);
        assert!(
            line.starts_with("OK SOUND t2=true t3p=true cert=true operand=true"),
            "r 50, {op}: the sliver, {line}"
        );
    }
    // `outcome`'s volume test is absolute (`1e-7`), which a lens of
    // `1.2e-11` passes whatever it reads, so the sliver's volume is held
    // here RELATIVE to the exact lens, per order and per gap. Measured at
    // most `4.0e-9` (`a ∩ b` at a `3e-6` gap, an error of 1.1e-19 m³),
    // with each face read about its shell's own vertex; `2.9e-9` when
    // every face was read about the world origin, the unit ball's
    // centre, where its cap's flux has no lever to cancel. The `b ∩ a` sliver at a `1e-5` gap reads `2.5e-3`
    // off: that is the flux's own `R·Area` against `c·A⃗` cancellation,
    // FLUXTAIL's `a-sphere-faces-flux-cancels-r-times-its-area-against-its-vector-area`
    // (`work/cleave/a-built-sliver-is-not-a-legal-operand.md` has the
    // table), held at its measured size so it cannot grow unseen.
    for gap in [2e-6, 3e-6, 5e-6, 1e-5] {
        let d = 51.0 - gap;
        let (a, b) = (unit(), ball(50.0, Vec3::new(0.0, 0.0, d)));
        let want = lens(1.0, 50.0, d);
        for (op, out) in [
            ("a ∩ b", topo::intersect(&a, &b, tol)),
            ("b ∩ a", topo::intersect(&b, &a, tol)),
        ] {
            let body = out
                .ok()
                .and_then(|r| r.body().map(|b| b.body.clone()))
                .unwrap_or_else(|| panic!("gap {gap:e}, {op}: the sliver builds"));
            let v = topo::test_support::pinned_volume("the measured body", &body, tol).volume;
            let rel = (v - want).abs() / want;
            let bound = if gap == 1e-5 && op == "b ∩ a" {
                3e-3
            } else {
                5e-9
            };
            assert!(
                rel <= bound,
                "gap {gap:e}, {op}: volume {v:e} against the lens {want:e}, {rel:e} relative"
            );
        }
    }
}
