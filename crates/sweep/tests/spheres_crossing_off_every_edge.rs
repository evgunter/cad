//! **Two spheres crossing in a circle no edge reaches build.** A ball
//! whose seam lies inside another ball's face meets that face in a
//! whole circle, off every edge of either operand, so the crossing
//! layer finds nothing and the containment fallback's sphere extent
//! scan meets two faces that cross. Each face holding the circle is
//! cut along its own chart's meridian through it
//! (`boolean::ops::apply_cut_ins`), so an edge of each operand crosses
//! the other's face and the re-entered join lands the circle as chords
//! on both: neither face keeps it as a ring.
//!
//! Every ball here is the `y`-poled full revolve, its seam meridian in
//! the plane `z = c_z`, so a centre line along `z` keeps both seams off
//! the circle at every depth. Every pose runs every op in both member
//! orders, held to tier 3, tier 3′ and its volume against spherical
//! caps.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Band, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled_y, brick, finished};
use topo::{AtRestBody, BooleanDeclarations, BooleanError, BooleanResult};

use crate::common::oracles::{ball_volume, cap_volume, lens_volume};

/// A `y`-poled ball of radius `r` at `c`, its seam meridian on `+x`.
fn ball(r: f64, c: Vec3<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let at = ball_poled_y(r, Vec3::new(0.0, 0.0, 0.0), tol);
    let moved = topo::transform_rigid(&at, &Affine3::translation(c), tol).unwrap();
    finished("the ball", moved, tol)
}

fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished("the box", brick(x, y, z, Tol::witness()), Tol::witness())
}

fn body(out: Result<BooleanResult<f64>, BooleanError>) -> AtRestBody<f64> {
    out.unwrap().body().unwrap().body.clone()
}

/// `a ∪ b`, `b ∪ a`, `a ∖ b`, `b ∖ a`, `a ∩ b`, `b ∩ a`, each a body at
/// tiers 3 and 3′ whose volume is read from the operands' volumes `va`,
/// `vb` and the volume they share.
fn assert_six(pose: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, volumes: (f64, f64, f64)) {
    assert_six_lumps(pose, a, b, volumes, &[]);
}

/// [`assert_six`], where the ops `lumps` names yield two lumps whose
/// tier 3′ the census cannot decide: curved faces of the two lumps are
/// within reach of each other
/// (`work/contact/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
fn assert_six_lumps(
    pose: &str,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    (va, vb, shared): (f64, f64, f64),
    lumps: &[&str],
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
    for (op, volume, out) in ops {
        let label = format!("{pose}, {op}");
        let bb = match out {
            Ok(BooleanResult::Body(bb)) => bb,
            out => panic!(
                "{label}: wanted a body, got {:?}",
                out.map(|r| r.body().map(|b| b.body.faces().count()))
            ),
        };
        topo::validate_geometric(&bb.body, tol)
            .unwrap_or_else(|e| panic!("{label}: tier 3: {e:?}"));
        match topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol) {
            Ok(()) if !lumps.contains(&op) => {}
            Err(errors)
                if lumps.contains(&op)
                    && bb.body.shells().count() == 2
                    && errors
                        .iter()
                        .all(|e| matches!(e, topo::ValidationError::CensusUndecidable { .. })) => {}
            got => panic!(
                "{label}: tier 3′, two lumps {}: {got:?}",
                lumps.contains(&op)
            ),
        }
        let v = topo::mass_properties(&bb.body, tol).unwrap().volume;
        assert!(
            (v - volume).abs() <= 1e-9 * volume.max(1.0),
            "{label}: volume {v} against the caps' {volume}"
        );
    }
}

/// The unit ball at the origin against the ball `(r, c)`.
fn against_the_unit_ball(pose: &str, r: f64, c: Vec3<f64>) {
    assert_six(
        pose,
        &ball(1.0, Vec3::new(0.0, 0.0, 0.0)),
        &ball(r, c),
        (
            ball_volume(1.0),
            ball_volume(r),
            lens_volume(1.0, r, c.norm()),
        ),
    );
}

/// The item's two witnesses: a plain pair, and a lens union whose
/// trimmed sphere face the small ball crosses (the small ball clears
/// the union's other sphere, so they share the one lens).
#[test]
fn both_witnesses_build_under_every_boolean() {
    against_the_unit_ball("plain", 0.3, Vec3::new(0.0, 0.0, 0.95));
    let tol = Tol::witness();
    let lens = body(topo::union(
        &ball(1.0, Vec3::new(2.0, 2.0, 0.5)),
        &ball(1.0, Vec3::new(3.4, 2.0, 0.5)),
        tol,
    ));
    assert_six(
        "the lens union",
        &lens,
        &ball(0.3, Vec3::new(2.0, 2.0, 1.45)),
        (
            2.0 * ball_volume(1.0) - lens_volume(1.0, 1.0, 1.4),
            ball_volume(0.3),
            lens_volume(1.0, 0.3, 0.95),
        ),
    );
}

/// The plain witness at the `Interval` scalar: the circle and both
/// cuts run on enclosures, and every body certifies with a volume
/// bracket around the caps, down to ε 1e-6.
#[test]
fn the_plain_witness_builds_at_the_interval_scalar() {
    use crate::common::interval::iv;
    use geom_core::{Bounds, Interval};
    let tol = Tol::witness();
    let ball_iv = |r: f64, z: f64| -> AtRestBody<Interval> {
        let at = ball_poled_y(iv(r), Vec3::new(iv(0.0), iv(0.0), iv(z)), tol);
        finished("the ball", at, tol)
    };
    let (a, b) = (ball_iv(1.0, 0.0), ball_iv(0.3, 0.95));
    let shared = lens_volume(1.0, 0.3, 0.95);
    let (va, vb) = (ball_volume(1.0), ball_volume(0.3));
    for (op, want, out) in [
        ("a ∪ b", va + vb - shared, topo::union(&a, &b, tol)),
        ("a ∖ b", va - shared, topo::subtract(&a, &b, tol)),
        ("b ∖ a", vb - shared, topo::subtract(&b, &a, tol)),
        ("a ∩ b", shared, topo::intersect(&a, &b, tol)),
    ] {
        // At ε 1e-12 the enclosures outgrow the band. The merge's mint
        // certifies the arcs' map residual with enclosures wider than
        // the band, as a tilted sphere pair's does
        // (`work/pcert/fitted-general-circle-rows-escalate-loop-continuity-at-the-interval-scalar.md`).
        // Small ∖ unit meets first the crossing layer reading the
        // unit ball's cut on the small ball's (the two cuts share a
        // plane, `boolean::ops::sphere_pair_cut`). Each escalates by name.
        if tol.eps() < 1e-10 {
            let predicate = match &out {
                Err(BooleanError::Merge(topo::MergeCoplanarError::Pcurve {
                    source:
                        topo::PcurveMintError::Certify {
                            error: geom_brep::PcurveCertifyError::Escalated { cause, .. },
                            ..
                        },
                })) => cause.predicate,
                Err(BooleanError::Escalated {
                    decision: topo::BooleanDecision::Crossing(_),
                    diag,
                }) => diag.predicate,
                _ => panic!("Interval {op} at eps 1e-12: wanted an escalation, got {out:?}"),
            };
            let want = if op == "b ∖ a" {
                "bool_wall_root_in_span"
            } else {
                "pcurve_map_residual"
            };
            assert_eq!(predicate, Some(want), "Interval {op} at eps 1e-12");
            continue;
        }
        let out = out.unwrap_or_else(|e| panic!("Interval {op}: refused {e:?}"));
        let body = &out
            .body()
            .unwrap_or_else(|| panic!("Interval {op}: empty"))
            .body;
        topo::validate_geometric(body, tol)
            .unwrap_or_else(|e| panic!("Interval {op}: tier 3: {e:?}"));
        let v = topo::mass_properties(body, tol)
            .unwrap_or_else(|e| panic!("Interval {op}: mass properties, got {e:?}"))
            .volume;
        let slack = 1e-9 * want.max(1.0);
        assert!(
            v.lo() - slack <= want && want <= v.hi() + slack,
            "Interval {op}: volume [{}, {}] against the caps' {want}",
            v.lo(),
            v.hi()
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
            against_the_unit_ball(&format!("r {r}, d {d}"), r, Vec3::new(0.0, 0.0, d));
        }
    }
}

/// `10⁻⁴` and `10⁻⁶` in from either tangency every radius builds. The
/// depths are band-relative (at `ε = 10⁻⁶` they reach the escalation
/// gap, and at `10⁻¹²` the f64 placement frontier,
/// `work/reach/f64-cannot-place-a-shallow-crossing-within-the-finest-band.md`),
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
            against_the_unit_ball(&format!("r {r}, d {d}"), r, Vec3::new(0.0, 0.0, d));
        }
    }
}

/// Inside the band's escalation gap from either tangency the pair is
/// not cut in: the nesting and gap questions escalate by name.
#[test]
fn a_depth_in_band_of_a_tangency_escalates() {
    use topo::{BooleanDecision, SphereQuestion};
    let band = Band::linear(Tol::witness()).unwrap();
    let e = (band.zero() + band.escalate()) / 2.0;
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    for (d, question) in [
        (0.7 + e, SphereQuestion::Nested),
        (1.3 - e, SphereQuestion::Apart),
    ] {
        let out = topo::union(&unit, &ball(0.3, Vec3::new(0.0, 0.0, d)), Tol::witness());
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

/// A centre line off `z` tilts the circle against both charts while
/// both seams stay off it.
#[test]
fn a_tilted_centre_line_builds() {
    for c in [
        Vec3::new(0.02, 0.01, 0.9),
        Vec3::new(-0.03, 0.02, 1.05),
        Vec3::new(0.0, 0.05, 0.8),
    ] {
        against_the_unit_ball(&format!("{c:?}"), 0.3, c);
    }
}

/// Both spheres trimmed, each by a box across its own seam and clear of
/// the other ball: the cap the unit ball loses (`x > 0.9`) misses the
/// small ball, and the one the small ball loses (`x > 0.25`) lies
/// outside the unit ball, so the two still share one lens.
#[test]
fn two_trimmed_spheres_build() {
    let tol = Tol::witness();
    let o = Vec3::new(0.0, 0.0, 0.0);
    let big = body(topo::subtract(
        &ball(1.0, o),
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
            lens_volume(1.0, 0.3, 1.2),
        ),
    );
}

/// The lens `ball(1, y = 0) ∩ ball(0.8, y = 1.4)` against a small ball
/// turned so its seam plane is normal to its centre line, at
/// `0.93·(0, cos 20°, sin 20°)`: its sphere crosses both of the lens's
/// sphere faces in whole circles, so its own face is cut twice. The
/// two caps it keeps outside the lens are disjoint (it stays `0.25`
/// from the circle the lens's spheres meet in), so the shared volume
/// is its two lenses less itself once, and what it keeps outside the
/// lens is those two caps.
#[test]
fn a_ball_crossing_both_faces_of_a_lens_builds() {
    let tol = Tol::witness();
    let on_y = |r: f64, y: f64| ball(r, Vec3::new(0.0, y, 0.0));
    let lens = body(topo::intersect(&on_y(1.0, 0.0), &on_y(0.8, 1.4), tol));
    let (s, c) = 20f64.to_radians().sin_cos();
    let at = Vec3::new(0.0, c, s) * 0.93;
    let turn = Affine3::rotation_about_axis(
        Point3::origin(),
        Vec3::new(1.0, 0.0, 0.0),
        -70f64.to_radians(),
    );
    let small = ball_poled_y(0.2, Vec3::new(0.0, 0.0, 0.0), tol);
    let small = topo::transform_rigid(&small, &turn, tol).unwrap();
    let small = finished(
        "the small ball",
        topo::transform_rigid(&small, &Affine3::translation(at), tol).unwrap(),
        tol,
    );
    let to_top = (at - Vec3::new(0.0, 1.4, 0.0)).norm();
    assert_six_lumps(
        "a ball across both lens faces",
        &lens,
        &small,
        (
            lens_volume(1.0, 0.8, 1.4),
            ball_volume(0.2),
            lens_volume(1.0, 0.2, 0.93) + lens_volume(0.8, 0.2, to_top) - ball_volume(0.2),
        ),
        &["b ∖ a"],
    );
}

/// A ball that pokes through a plane face asks for a re-chart, and the
/// re-chart's graft renames the faces a sphere cut-in would name: a
/// ball both poking a slab's face and crossing a sphere face off every
/// edge refuses typed, in every op and order.
#[test]
fn a_ball_poking_a_plane_and_crossing_a_sphere_refuses_typed() {
    let tol = Tol::witness();
    let tool = body(topo::union(
        &boxed((-3.0, 3.0), (-3.0, 3.0), (0.5, 3.0)),
        &ball(0.3, Vec3::new(0.0, 0.0, -0.95)),
        tol,
    ));
    let unit = ball(1.0, Vec3::new(0.0, 0.0, 0.0));
    let none = BooleanDeclarations::none();
    for (op, out) in [
        ("a ∪ b", topo::union_with(&unit, &tool, &none, tol)),
        ("b ∪ a", topo::union_with(&tool, &unit, &none, tol)),
        ("a ∖ b", topo::subtract_with(&unit, &tool, &none, tol)),
        ("b ∖ a", topo::subtract_with(&tool, &unit, &none, tol)),
        ("a ∩ b", topo::intersect_with(&unit, &tool, &none, tol)),
        ("b ∩ a", topo::intersect_with(&tool, &unit, &none, tol)),
    ] {
        match out {
            Err(BooleanError::FallbackExtentUnsupported { what, .. }) => assert!(
                what.contains("rename the face the sphere's cut names"),
                "{op}: {what}"
            ),
            out => panic!(
                "{op}: wanted the typed refusal, got {:?}",
                out.map(|_| "a body")
            ),
        }
    }
}
