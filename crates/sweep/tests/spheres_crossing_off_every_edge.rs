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
//!   face does not mesh yet
//!   (`work/tess/sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane.md`).
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
use topo::{AtRestBody, BooleanDeclarations, BooleanError, BooleanResult};

use crate::common::differential::outcome;
use crate::common::oracles::{ball_lens as lens, ball_volume, cap_volume};

use Want::{Draws, Lumps, Refuses, Tilted};

/// A `y`-poled ball of radius `r` turned by `turn`, then moved to `c`.
fn placed(r: f64, turn: Affine3<f64>, c: Vec3<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let at = ball_poled_y(r, Vec3::new(0.0, 0.0, 0.0), tol);
    let at = topo::transform_rigid(&at, &turn, tol).unwrap();
    let moved = topo::transform_rigid(&at, &Affine3::translation(c), tol).unwrap();
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
    /// `SOUND`, and the mesher refuses a sphere face bounded by a
    /// circle tilted against its chart (TESS's tilted-circle row).
    Tilted,
    /// Two lumps whose tier 3′ the census cannot decide (curved faces
    /// of the two lumps within reach of each other,
    /// `work/contact/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`),
    /// otherwise sound; the mesher refuses as for [`Want::Tilted`].
    Lumps,
    /// A typed refusal the predicate accepts.
    Refuses(fn(&BooleanError) -> bool),
}

/// The mesher's refusal of a sphere face its tilted circle bounds.
fn tilted_refusal(body: &topo::Body<f64>) -> bool {
    matches!(
        mesh::tessellate(body, 5e-3, Tol::witness()),
        Err(mesh::TessellateError::UnsupportedCurvedShape {
            source: geom_brep::props::PropsError::NotIsoRectangle { .. },
            ..
        })
    )
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
        }
        Tilted | Lumps => assert!(
            tilted_refusal(&bb.body),
            "{label}: the tilted-circle mesh refusal"
        ),
        Refuses(_) => unreachable!(),
    }
    if let Lumps = want {
        let census = topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol);
        assert!(
            bb.body.shells().count() == 2
                && census.as_ref().is_err_and(|errors| {
                    errors
                        .iter()
                        .all(|e| matches!(e, topo::ValidationError::CensusUndecidable { .. }))
                }),
            "{label}: two lumps the census cannot decide, got {census:?}"
        );
    }
    let v = topo::mass_properties(&bb.body, tol).unwrap().volume;
    assert!(
        (v - volume).abs() <= 1e-9 * volume.abs() + 1e-15 * scale,
        "{label}: volume {v} against the closed form {volume}"
    );
    let line = outcome(out, volume, tol);
    let sound = match want {
        Lumps => "OK BAD t2=true t3p=false cert=true operand=true",
        _ => "OK SOUND",
    };
    assert!(line.starts_with(sound), "{label}: {line}");
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
    let (tol, none) = (Tol::witness(), BooleanDeclarations::none());
    let ops = [
        (
            "a ∪ b",
            va + vb - shared,
            topo::union_with(a, b, &none, tol),
        ),
        (
            "b ∪ a",
            va + vb - shared,
            topo::union_with(b, a, &none, tol),
        ),
        ("a ∖ b", va - shared, topo::subtract_with(a, b, &none, tol)),
        ("b ∖ a", vb - shared, topo::subtract_with(b, a, &none, tol)),
        ("a ∩ b", shared, topo::intersect_with(a, b, &none, tol)),
        ("b ∩ a", shared, topo::intersect_with(b, a, &none, tol)),
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
        [Tilted; 6],
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
        [Tilted; 6],
    );
}

/// The trimmed unit ball's face is cut along its meridian through a
/// circle passing `δ` from its pole, the meridian's boundary hit there
/// ordered against the circle's crossing by the arc between them. At
/// `δ = 10⁻⁷` the arc's pcurve beside the pole refuses certification by
/// name. The distances are band-relative (at `ε = 10⁻⁶` the `10⁻⁵`
/// circle's crossings read the meridian's half in the escalation gap),
/// so the other ε rows stand down.
#[test]
fn a_trimmed_face_cut_beside_its_pole_builds_or_refuses_by_name() {
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
    let near_pole = |e: &BooleanError| {
        let certify = |s: &topo::PcurveMintError| {
            matches!(
                s,
                topo::PcurveMintError::Certify {
                    error: geom_brep::PcurveCertifyError::ArcNearPole,
                    ..
                }
            )
        };
        match e {
            BooleanError::Pcurves { source } => certify(source),
            BooleanError::Merge(topo::MergeCoplanarError::Pcurve { source }) => certify(source),
            _ => false,
        }
    };
    for (delta, want) in [(1e-3, Tilted), (1e-5, Tilted), (1e-7, Refuses(near_pole))] {
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
            [want; 6],
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
        [Tilted, Tilted, Tilted, Lumps, Tilted, Tilted],
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
        [Tilted, Tilted, Tilted, Lumps, Lumps, Lumps],
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
/// refuses at the result gate: the rim of the unit ball's kept face
/// reads its level in band. The two lens-shaped differences build. The
/// intersection builds a sliver `2·10⁻⁶` thick whose at-infinity side
/// the next op cannot decide (`operand=false`), filed on the result
/// gate as `a-built-sliver-is-not-a-legal-operand`.
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
    let none = BooleanDeclarations::none();
    let d = 51.0 - 2e-6;
    let (a, b) = (unit(), ball(50.0, Vec3::new(0.0, 0.0, d)));
    let (va, vb, shared) = (ball_volume(1.0), ball_volume(50.0), lens(1.0, 50.0, d));
    let rim_in_band = |e: &BooleanError| {
        matches!(
            e,
            BooleanError::ResultInvalid { errors }
                if matches!(
                    errors.as_slice(),
                    [topo::ValidationError::VolumeUncomputable { .. }]
                )
        )
    };
    for (op, out) in [
        ("a ∪ b", topo::union_with(&a, &b, &none, tol)),
        ("b ∪ a", topo::union_with(&b, &a, &none, tol)),
    ] {
        check(
            &format!("r 50, {op}"),
            out,
            (va + vb - shared, va + vb),
            Refuses(rim_in_band),
        );
    }
    check(
        "r 50, a ∖ b",
        topo::subtract_with(&a, &b, &none, tol),
        (va - shared, va + vb),
        Draws,
    );
    check(
        "r 50, b ∖ a",
        topo::subtract_with(&b, &a, &none, tol),
        (vb - shared, va + vb),
        Draws,
    );
    for (op, out) in [
        ("a ∩ b", topo::intersect_with(&a, &b, &none, tol)),
        ("b ∩ a", topo::intersect_with(&b, &a, &none, tol)),
    ] {
        let line = outcome(out, shared, tol);
        assert!(
            line.starts_with("OK BAD t2=true t3p=true cert=true operand=false"),
            "r 50, {op}: the sliver, {line}"
        );
    }
}
