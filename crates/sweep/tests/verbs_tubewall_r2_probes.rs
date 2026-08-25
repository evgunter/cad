//! **VERBS-TUBEWALL review probes (r2, independent lane)** — a
//! consumer suite authored against the PUBLIC door only, to falsify
//! the unit's own claims rather than to re-state them.
//!
//! What each row is for is written on the row. The rows that are
//! deliberately *stronger* than the shipped suite's, and why:
//!
//! - the shipped intent-recoverability row uses `0.5 - 0.125`, which
//!   is EXACT in binary, so it cannot distinguish "one IEEE
//!   subtraction" from any reconstruction that happens to round back
//!   to `0.375`. These rows use radii whose subtraction actually
//!   rounds (`0.3 - 0.1`, `0.7 - 0.3`, …);
//! - the shipped refusal row pins NaN escalation but not an IN-BAND
//!   wall, which is the merit the unit claims a bracket read cannot
//!   give it. These rows walk a ladder and require the escalating
//!   zone to be NON-EMPTY, which is what goes red if the wall ever
//!   stops being metered;
//! - the shipped suite never exercises a window whose `t0` is 0, nor
//!   the `u_ref` a window actually stores, so the "full period only"
//!   caveat is untested in both directions.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Surface;
use geom_core::{Point3, Tol, Vec3};
use sweep::{Revolved, TubeError, TubeWindow, tube_along_arc, tube_along_arc_hollow};
use topo::Body;

fn c() -> Point3<f64> {
    Point3::new(0.0, 0.0, 0.0)
}

fn build(
    major: f64,
    window: TubeWindow<f64>,
    minor: f64,
    wall: f64,
) -> Result<Revolved<f64>, TubeError> {
    tube_along_arc_hollow::<f64>(
        c(),
        Vec3::unit_y(),
        Vec3::unit_x(),
        major,
        window,
        minor,
        wall,
        Tol::witness(),
    )
}

fn tiers(body: &Body<f64>, what: &str) {
    assert_eq!(topo::validate(body), Ok(()), "{what} tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "{what} tier 2");
    assert_eq!(
        topo::validate_geometric(body, Tol::witness()),
        Ok(()),
        "{what} tier 3"
    );
}

fn minors(body: &Body<f64>) -> Vec<u64> {
    let mut out: Vec<u64> = Vec::new();
    for (_, face) in body.faces() {
        if let Some(Surface::Torus { minor_radius, .. }) = body.get_surface(face.surface) {
            out.push(minor_radius.to_bits());
        }
    }
    out.sort_unstable();
    out
}

fn u_refs(body: &Body<f64>) -> Vec<Vec3<f64>> {
    let mut out = Vec::new();
    for (_, face) in body.faces() {
        if let Some(Surface::Torus { u_ref, .. }) = body.get_surface(face.surface) {
            out.push(*u_ref);
        }
    }
    out
}

fn rel(got: f64, want: f64) -> f64 {
    ((got - want) / want).abs()
}

// ---------------------------------------------------------------
// C2 — intent recoverability where the subtraction actually ROUNDS.
// ---------------------------------------------------------------

/// The stored inner minor radius is `minor_radius - wall` computed in
/// the SAME single IEEE subtraction the caller writes, at radii where
/// that subtraction is inexact. `0.3 - 0.1` is
/// `0.19999999999999998`, one ulp under `0.2`: a door that stored the
/// "obvious" `0.2`, or that reconstructed the radius through bulge
/// arithmetic, goes red here and cannot go red on the shipped
/// suite's exactly-representable `0.5 - 0.125`.
#[test]
fn r2_inner_radius_is_the_callers_own_rounding() {
    for (major, minor, wall) in [
        (2.0_f64, 0.3_f64, 0.1_f64),
        (2.0, 0.7, 0.3),
        (5.0, 1.1, 0.9),
        (10.0, 0.2, 0.01),
    ] {
        let inner = minor - wall;
        assert_ne!(
            inner.to_bits(),
            (minor - wall + 0.0_f64).to_bits().wrapping_add(1),
            "sanity: the probe's own arithmetic"
        );
        for window in [TubeWindow::Full, TubeWindow::Arc { t0: 0.25, t1: 1.75 }] {
            let t = build(major, window, minor, wall).expect("builds");
            let got = minors(&t.body);
            let mut want = vec![
                inner.to_bits(),
                inner.to_bits(),
                minor.to_bits(),
                minor.to_bits(),
            ];
            want.sort_unstable();
            assert_eq!(
                got, want,
                "major {major}, minor {minor}, wall {wall}: stored minor radii are not \
                 {{minor, minor, minor-wall, minor-wall}} bit for bit"
            );
        }
    }
}

/// The rounding is REAL at the probe's radii: `0.3 - 0.1 != 0.2`.
/// Without this the row above could pass vacuously on a door that
/// rounded, so it is pinned rather than assumed.
#[test]
fn r2_the_probe_radii_actually_round() {
    assert_ne!((0.3_f64 - 0.1).to_bits(), 0.2_f64.to_bits());
    assert_ne!((0.7_f64 - 0.3).to_bits(), 0.4_f64.to_bits());
    assert_ne!((1.1_f64 - 0.9).to_bits(), 0.2_f64.to_bits());
    // and the shipped suite's constants do NOT round, which is why
    // its row is weaker than this one.
    assert_eq!((0.5_f64 - 0.125).to_bits(), 0.375_f64.to_bits());
}

/// The `u_ref` caveat, exercised in BOTH directions. The unit's
/// claim is "verbatim for the full period"; the module doc adds "and
/// `t0 = 0`". Both hold, and a window with `t0 != 0` stores the
/// ROTATED direction — so the caveat is honest and not a hedge.
#[test]
fn r2_u_ref_caveat_is_exact_in_both_directions() {
    let full = build(2.0, TubeWindow::Full, 0.3, 0.1).expect("full builds");
    for u in u_refs(&full.body) {
        assert_eq!(
            (u.x.to_bits(), u.y.to_bits(), u.z.to_bits()),
            (1.0_f64.to_bits(), 0.0_f64.to_bits(), 0.0_f64.to_bits())
        );
    }
    // A window that starts at t0 = 0 also stores u_ref verbatim: the
    // rotation is by an exact zero angle. The unit's PR body says
    // "full period" only; the module doc says the truth.
    let at_zero = build(2.0, TubeWindow::Arc { t0: 0.0, t1: 1.5 }, 0.3, 0.1).expect("builds");
    for u in u_refs(&at_zero.body) {
        assert_eq!(
            (u.x.to_bits(), u.y.to_bits(), u.z.to_bits()),
            (1.0_f64.to_bits(), 0.0_f64.to_bits(), 0.0_f64.to_bits()),
            "an Arc starting at t0 = 0 stores u_ref verbatim too"
        );
    }
    // A window with t0 != 0 stores the rotated start direction, and
    // it is NOT the caller's u_ref — the one derived quantity.
    let rotated = build(2.0, TubeWindow::Arc { t0: 0.25, t1: 1.75 }, 0.3, 0.1).expect("builds");
    let us = u_refs(&rotated.body);
    assert!(!us.is_empty());
    for u in us {
        assert!(
            (u.x - 0.25_f64.cos()).abs() < 1e-15 && (u.z + 0.25_f64.sin()).abs() < 1e-15
                || (u.x - 0.25_f64.cos()).abs() < 1e-15 && (u.z - 0.25_f64.sin()).abs() < 1e-15,
            "window start direction {:?} is not u_ref rotated by t0",
            (u.x, u.y, u.z)
        );
        assert!(
            u.x.to_bits() != 1.0_f64.to_bits(),
            "a t0 != 0 window must NOT store the caller's u_ref verbatim"
        );
    }
}

// ---------------------------------------------------------------
// C3 — the metering claim's own merit, made RED-able.
// ---------------------------------------------------------------

/// The unit's central argument for metering over a bracket read is
/// that an in-band wall ESCALATES instead of silently building a
/// sliver. A bracket read has no such zone at all, so this row goes
/// red the day the wall check becomes `wall.lo() > 0.0`.
///
/// Written eps-agnostically: walk a geometric ladder of walls and
/// require the three zones to appear in order — refuse, escalate,
/// build — with the ESCALATING zone non-empty.
#[test]
fn r2_the_wall_has_a_nonempty_escalating_zone() {
    let mut zones: Vec<(f64, &'static str)> = Vec::new();
    let mut w = 1e-300_f64;
    while w < 1e-2 {
        let z = match build(2.0, TubeWindow::Full, 0.5, w) {
            Err(TubeError::NonpositiveWall { .. }) => "refuse",
            Err(TubeError::Escalated { .. }) => "escalate",
            Ok(_) => "build",
            other => panic!("unexpected verdict for wall {w}: {other:?}"),
        };
        zones.push((w, z));
        w *= 1.2;
    }
    let seen: Vec<&str> = {
        let mut v: Vec<&str> = Vec::new();
        for (_, z) in &zones {
            if v.last() != Some(z) {
                v.push(z);
            }
        }
        v
    };
    assert_eq!(
        seen,
        vec!["refuse", "escalate", "build"],
        "the wall ladder must pass through exactly refuse -> escalate -> build \
         (a bracket read would show refuse -> build with no escalating zone); saw {seen:?}"
    );
}

/// The same for the BORE arm: a wall that leaves an in-band inner
/// radius escalates rather than minting a circle that rounds onto
/// the outer one.
#[test]
fn r2_the_bore_has_a_nonempty_escalating_zone() {
    let minor = 0.5_f64;
    let mut zones: Vec<&'static str> = Vec::new();
    // gap = minor - wall, walked down through the band.
    let mut gap = 1e-2_f64;
    while gap > 1e-300 {
        let z = match build(2.0, TubeWindow::Full, minor, minor - gap) {
            Err(TubeError::WallExceedsRadius { .. }) => "refuse",
            Err(TubeError::Escalated { .. }) => "escalate",
            Ok(_) => "build",
            other => panic!("unexpected verdict for gap {gap}: {other:?}"),
        };
        if zones.last() != Some(&z) {
            zones.push(z);
        }
        gap /= 1.2;
    }
    assert!(
        zones.contains(&"escalate"),
        "the bore arm must have a non-empty escalating zone; saw {zones:?}"
    );
    assert_eq!(zones.first(), Some(&"build"));
    assert_eq!(zones.last(), Some(&"refuse"));
}

/// **AMENDED at the fix pass** (disclosed): this row was written to
/// record a mismatch as a fact — `Escalated` from a wall predicate
/// opened with the SOLID door's name even though only the hollow door
/// can reach `tube_wall`/`tube_wall_bore`/`tube_wall_gap`. The finding
/// was accepted and fixed, so the row is amended to pin the CORRECTED
/// naming rather than deleted: an escalation carrying a hollow-only
/// predicate name says `tube_along_arc_hollow`, and the arms both
/// doors share say `tube door` rather than picking one.
#[test]
fn r2_escalation_from_a_wall_predicate_reports_the_hollow_doors_name() {
    let err = build(2.0, TubeWindow::Full, 0.5, f64::NAN).expect_err("a poisoned wall refuses");
    let msg = err.to_string();
    assert!(matches!(err, TubeError::Escalated { .. }), "{msg}");
    let TubeError::Escalated { source } = &err else {
        unreachable!()
    };
    assert!(
        matches!(source.predicate, Some("tube_wall")),
        "the poisoned wall escalates at tube_wall, got {:?}",
        source.predicate
    );
    assert!(
        msg.starts_with("tube_along_arc_hollow escalated:"),
        "a hollow-only predicate must name the hollow door: {msg}"
    );
    // And the shared arms do NOT claim either door: a non-unit axis
    // is raised identically by both.
    let shared = build(2.0, TubeWindow::Full, 0.5, 0.125);
    let _ = shared;
    let frame = tube_along_arc_hollow::<f64>(
        c(),
        Vec3::unit_y() * 1.5,
        Vec3::unit_x(),
        2.0,
        TubeWindow::Full,
        0.5,
        0.125,
        Tol::witness(),
    )
    .expect_err("a non-unit axis refuses");
    assert!(
        frame.to_string().starts_with("tube door: "),
        "a shared arm names neither door: {frame}"
    );
}

// ---------------------------------------------------------------
// C5 — the closed forms, re-derived independently, over a spread of
// radii / walls / windows the shipped suite does not visit.
// ---------------------------------------------------------------

/// Pappus on the annulus, and the torus-shell forms, re-derived here
/// from scratch:
///
/// - section area `A = π(ro² − ri²)`, centroid on the spine at
///   distance `R` from the axis, so an elbow of span θ has
///   `V = θ·R·A` and a full period `V = 2π·R·A = 2π²R(ro²−ri²)`;
/// - each wall is a surface of revolution of a circle of radius `r`
///   whose centroid is also at `R`, so its area is `θ·R·2πr`; an
///   elbow adds two flat annuli (`2A`), a full period adds nothing:
///   `A = 4π²R(ro+ri)`.
#[test]
fn r2_closed_forms_over_varied_radii_walls_and_windows() {
    /// Major radius, outer minor radius, wall, and the window as
    /// `(t0, t1)` or `None` for a full period.
    type Case = (f64, f64, f64, Option<(f64, f64)>);
    let cases: [Case; 10] = [
        (2.0, 0.5, 0.125, None),
        (2.0, 0.5, 0.125, Some((0.25, 1.75))),
        (2.0, 0.3, 0.1, Some((0.0, 0.5))),
        (2.0, 0.3, 0.1, Some((-1.25, 0.75))),
        (5.0, 1.25, 0.7, None),
        (5.0, 1.25, 0.7, Some((3.0, 6.0))),
        (10.0, 0.2, 0.01, None),
        (10.0, 0.2, 0.01, Some((0.1, 6.2))),
        (0.5, 0.4, 0.35, None),
        (1.0, 0.9, 0.45, Some((0.0, 3.0))),
    ];
    for (major, minor, wall, arc) in cases {
        let inner = minor - wall;
        let window = match arc {
            None => TubeWindow::Full,
            Some((t0, t1)) => TubeWindow::Arc { t0, t1 },
        };
        let t = build(major, window, minor, wall)
            .unwrap_or_else(|e| panic!("R {major} ro {minor} wall {wall} arc {arc:?}: {e}"));
        let what = format!("R {major} ro {minor} wall {wall} arc {arc:?}");
        tiers(&t.body, &what);

        let area_sec = PI * (minor * minor - inner * inner);
        let (theta, caps) = match arc {
            None => (2.0 * PI, 0.0),
            Some((t0, t1)) => (t1 - t0, 2.0 * area_sec),
        };
        let v = theta * major * area_sec;
        let a = theta * major * 2.0 * PI * (minor + inner) + caps;
        let p = topo::props::mass_properties(&t.body, Tol::witness()).expect("mass properties");
        assert!(
            rel(p.volume, v) < 1e-11,
            "{what}: volume {} vs closed form {v} (relative {})",
            p.volume,
            rel(p.volume, v)
        );
        assert!(
            rel(p.surface_area, a) < 1e-11,
            "{what}: area {} vs closed form {a} (relative {})",
            p.surface_area,
            rel(p.surface_area, a)
        );

        // Census: the full period is two shells (outer + cavity),
        // a window is one open shell with no cavity.
        match arc {
            None => {
                assert_eq!(t.body.shells().count(), 2, "{what}");
                assert_eq!(t.cavities.len(), 1, "{what}");
                assert_eq!(
                    t.body.faces().count(),
                    4,
                    "{what}: two walls, two faces each"
                );
            }
            Some(_) => {
                assert_eq!(t.body.shells().count(), 1, "{what}");
                assert!(t.cavities.is_empty(), "{what}");
                assert_eq!(
                    t.body.faces().count(),
                    6,
                    "{what}: four torus half-walls plus two annular caps"
                );
            }
        }
    }
}

/// The hollow body plus its own bore IS the solid body, at every
/// window: `V_hollow + V_bore == V_solid`. Independent of the Pappus
/// forms above (it is a kernel-vs-kernel identity), so it stays
/// informative if the closed forms are wrong.
#[test]
fn r2_hollow_plus_bore_is_the_solid_tube() {
    for (major, minor, wall, arc) in [
        (2.0_f64, 0.5_f64, 0.125_f64, None::<(f64, f64)>),
        (2.0, 0.3, 0.1, Some((0.25, 1.75))),
        (5.0, 1.25, 0.7, Some((0.0, 2.0))),
        (5.0, 1.25, 0.7, None),
    ] {
        let window = match arc {
            None => TubeWindow::Full,
            Some((t0, t1)) => TubeWindow::Arc { t0, t1 },
        };
        let hollow = build(major, window, minor, wall).expect("hollow builds");
        let solid = tube_along_arc::<f64>(
            c(),
            Vec3::unit_y(),
            Vec3::unit_x(),
            major,
            window,
            minor,
            Tol::witness(),
        )
        .expect("solid builds");
        let bore = tube_along_arc::<f64>(
            c(),
            Vec3::unit_y(),
            Vec3::unit_x(),
            major,
            window,
            minor - wall,
            Tol::witness(),
        )
        .expect("bore builds");
        let vh = topo::props::mass_properties(&hollow.body, Tol::witness())
            .expect("props")
            .volume;
        let vs = topo::props::mass_properties(&solid.body, Tol::witness())
            .expect("props")
            .volume;
        let vb = topo::props::mass_properties(&bore.body, Tol::witness())
            .expect("props")
            .volume;
        assert!(
            rel(vh + vb, vs) < 1e-11,
            "R {major} ro {minor} wall {wall} arc {arc:?}: hollow {vh} + bore {vb} != solid {vs}"
        );
    }
}

// ---------------------------------------------------------------
// C1 — the solid door is untouched THROUGH the shared body.
// ---------------------------------------------------------------

/// The solid door's own output, dumped bit-faithfully. Rust's `{:?}`
/// for `f64` is round-trip exact, so this string IS the body's bits.
/// The row here only pins that the dump is stable within a run; the
/// cross-revision comparison against the merge base is done by the
/// review harness, which runs this same dump with `tube.rs` reverted.
#[test]
fn r2_solid_door_dump_is_deterministic() {
    let a = tube_along_arc::<f64>(
        c(),
        Vec3::unit_y(),
        Vec3::unit_x(),
        2.0,
        TubeWindow::Arc { t0: 0.25, t1: 1.75 },
        0.5,
        Tol::witness(),
    )
    .expect("solid elbow");
    let b = tube_along_arc::<f64>(
        c(),
        Vec3::unit_y(),
        Vec3::unit_x(),
        2.0,
        TubeWindow::Arc { t0: 0.25, t1: 1.75 },
        0.5,
        Tol::witness(),
    )
    .expect("solid elbow");
    assert_eq!(format!("{:?}", a.body), format!("{:?}", b.body));
}

// ---------------------------------------------------------------
// Fragility: what the door builds at the edges of its own band.
// ---------------------------------------------------------------

/// A wall just above the definiteness threshold and a bore just above
/// it both BUILD. This row is not an assertion that they should — it
/// records what the door does at the thinnest wall it accepts, so a
/// change of mind about the sliver policy is visible.
#[test]
fn r2_the_thinnest_accepted_wall_still_validates() {
    // walk up until the door stops escalating, then take the first
    // build.
    let minor = 0.5_f64;
    let mut w = 1e-300_f64;
    let built = loop {
        match build(2.0, TubeWindow::Full, minor, w) {
            Ok(t) => break (w, t),
            Err(TubeError::NonpositiveWall { .. } | TubeError::Escalated { .. }) => w *= 1.05,
            Err(e) => panic!("unexpected {e}"),
        }
        assert!(w < 1.0, "no wall was ever accepted");
    };
    let (w, t) = built;
    tiers(&t.body, &format!("thinnest accepted wall {w}"));
    assert_eq!(t.body.shells().count(), 2);
    assert_eq!(t.cavities.len(), 1);
    // and its stored inner radius is still the caller's subtraction.
    let got = minors(&t.body);
    let inner = minor - w;
    let mut want = vec![
        inner.to_bits(),
        inner.to_bits(),
        minor.to_bits(),
        minor.to_bits(),
    ];
    want.sort_unstable();
    assert_eq!(got, want, "thinnest wall {w}");
}

/// **The gap the two decides do not measure.** The unit's cavity
/// evidence is `Carried { Positive }` — strict containment of the
/// inner circle in the outer — justified by two verdicts: `wall`
/// definitely positive, and `minor_radius - wall` definitely
/// positive. Neither of those is the RADIAL GAP between the two
/// circles, which is `minor_radius - fl(minor_radius - wall)`, and
/// at a large `minor_radius` that gap is ZERO while both verdicts
/// still read `Positive`: any `wall` under half an ulp of
/// `minor_radius` rounds the subtraction away entirely.
///
/// The row scans for such a case and reports what the door does with
/// it. It is written to PASS when the door refuses or escalates and
/// to FAIL when the door builds a body whose two circles coincide.
#[test]
fn r2_a_wall_below_half_an_ulp_of_the_radius_collapses_the_bore() {
    let eps = Tol::witness().eps();
    // A radius large enough that half an ulp exceeds the definiteness
    // threshold K*eps, so a `wall` can be definitely positive AND
    // round away against it.
    let mut minor = 1.0_f64;
    while f64::from_bits(minor.to_bits() + 1) - minor < 400.0 * eps {
        minor *= 2.0;
    }
    let ulp = f64::from_bits(minor.to_bits() + 1) - minor;
    let wall = ulp / 4.0; // definitely positive (> K*eps) and < ulp/2
    assert!(
        wall > 10.0 * eps,
        "probe setup: wall {wall} must be definitely positive at eps {eps}"
    );
    assert_eq!(
        (minor - wall).to_bits(),
        minor.to_bits(),
        "probe setup: the subtraction must round away (ulp {ulp})"
    );
    let major = minor * 2.0;
    let verdict = build(major, TubeWindow::Full, minor, wall);
    match verdict {
        Err(e) => {
            // The honest outcomes: a typed refusal or an escalation.
            println!("R2GAP: refused as expected: {e}");
        }
        Ok(t) => {
            let m = minors(&t.body);
            let distinct: std::collections::BTreeSet<u64> = m.iter().copied().collect();
            panic!(
                "the door BUILT a hollow torus at R {major}, minor {minor}, wall {wall} \
                 whose inner and outer circles are the SAME circle: stored minor radii \
                 {distinct:?} ({} distinct), shells {}, cavities {} — `Carried {{ Positive }}` \
                 strict-containment evidence for two coincident circles",
                distinct.len(),
                t.body.shells().count(),
                t.cavities.len()
            );
        }
    }
}

/// The same hole from the other side and without any ulp arithmetic:
/// a hollow tube whose two stored minor radii are NOT distinct must
/// never be built, at any radius the door accepts. Scans a ladder of
/// radii so it stays informative if the arithmetic above is ever
/// wrong about where the collapse begins.
#[test]
fn r2_stored_inner_and_outer_radii_are_always_distinct() {
    let eps = Tol::witness().eps();
    let mut minor = 1.0_f64;
    let mut collapsed: Vec<(f64, f64)> = Vec::new();
    while minor < 1e18 {
        let ulp = f64::from_bits(minor.to_bits() + 1) - minor;
        for wall in [ulp / 4.0, ulp / 3.0, 100.0 * eps, 1e-3] {
            if wall <= 20.0 * eps || wall >= minor {
                continue;
            }
            if let Ok(t) = build(minor * 2.0, TubeWindow::Full, minor, wall) {
                let distinct: std::collections::BTreeSet<u64> =
                    minors(&t.body).into_iter().collect();
                if distinct.len() < 2 {
                    collapsed.push((minor, wall));
                }
            }
        }
        minor *= 16.0;
    }
    assert!(
        collapsed.is_empty(),
        "the door built hollow tori whose inner circle IS the outer circle at \
         (minor, wall) = {collapsed:?}"
    );
}

/// **Where the collapse is actually caught, and what it costs.** The
/// row above shows the door does not build the coincident-circle
/// body — but it is not the wall funnel that stops it: the two wall
/// decides pass, the frame is built, both loops are classified, and
/// the refusal arrives from the pcurve/attachment certification gate
/// as an opaque `TubeError::Revolve`. This row records that, densely,
/// across BOTH windows: for every collapsed configuration it reports
/// which door refused, and fails if any of them BUILDS.
///
/// **AMENDED at the fix pass** (disclosed): the FACT half below was
/// written to record that no wall door named this class — the
/// refusals arrived from downstream geometry gates by luck. The
/// finding was accepted and fixed by a third decide,
/// `tube_wall_gap` on the REALIZED gap `minor_radius - inner`, so
/// the assertion is INVERTED rather than deleted: every collapsed
/// configuration must now be named by a wall door, and specifically
/// by `WallGapCollapsed`. It goes red if that decide is removed.
#[test]
fn r2_collapsed_bore_is_refused_by_a_wall_door() {
    let eps = Tol::witness().eps();
    let mut rows: Vec<String> = Vec::new();
    let mut built: Vec<String> = Vec::new();
    let mut minor = 1.0_f64;
    while minor < 1e17 {
        let ulp = f64::from_bits(minor.to_bits() + 1) - minor;
        for frac in [2.5_f64, 3.0, 4.0, 8.0, 64.0] {
            let wall = ulp / frac;
            if wall <= 20.0 * eps || (minor - wall).to_bits() != minor.to_bits() {
                continue;
            }
            for (tag, window) in [
                ("full", TubeWindow::Full),
                ("arc", TubeWindow::Arc { t0: 0.25, t1: 1.75 }),
            ] {
                let what = format!("minor {minor:e} wall {wall:e} ulp/{frac} {tag}");
                match build(minor * 2.0, window, minor, wall) {
                    Ok(t) => built.push(format!(
                        "{what}: BUILT, shells {}, cavities {}, distinct radii {}",
                        t.body.shells().count(),
                        t.cavities.len(),
                        minors(&t.body)
                            .into_iter()
                            .collect::<std::collections::BTreeSet<_>>()
                            .len()
                    )),
                    Err(TubeError::NonpositiveWall { .. }) => {
                        rows.push(format!("{what}: NonpositiveWall"))
                    }
                    Err(TubeError::WallExceedsRadius { .. }) => {
                        rows.push(format!("{what}: WallExceedsRadius"));
                    }
                    Err(TubeError::WallGapCollapsed { .. }) => {
                        rows.push(format!("{what}: WallGapCollapsed"));
                    }
                    Err(TubeError::Escalated { .. }) => rows.push(format!("{what}: Escalated")),
                    Err(e) => rows.push(format!("{what}: {}", short(&e))),
                }
            }
        }
        minor *= 2.0;
    }
    for r in &rows {
        println!("R2COLLAPSE {r}");
    }
    // THE SOUNDNESS HALF, and the one that must stay green: no
    // collapsed configuration may reach a built body. Red the day one
    // does.
    assert!(built.is_empty(), "collapsed bores that BUILT: {built:#?}");
    assert!(
        !rows.is_empty(),
        "the probe found no collapsed configuration at eps {eps} — its arithmetic is stale"
    );
    // THE ATTRIBUTION HALF, inverted at the fix pass: every collapsed
    // configuration is now named by the wall funnel, before the
    // frame, both classifications and the mint run — so the "decided
    // FIRST, before anything is minted" posture covers this input
    // class too, and the `Carried { Positive }` containment evidence
    // is never supplied for a pair of coincident circles. Red the day
    // the `tube_wall_gap` decide is removed.
    assert!(
        rows.iter().all(|r| r.contains("WallGapCollapsed")),
        "every collapsed bore must be named by the realized-gap wall door. Rows: {rows:#?}"
    );
}

fn short(e: &TubeError) -> String {
    let s = e.to_string();
    s.chars().take(90).collect()
}

/// The refusal ORDER: the wall arms are decided before the frame, so
/// a call that is wrong in both ways reports the WALL. Recorded
/// because the shipped suite never puts a bad wall and a bad frame in
/// the same call, and because it is the one place the hollow door's
/// verdict sequence differs in kind from the solid door's.
#[test]
fn r2_wall_verdicts_preempt_the_frame_verdicts() {
    let e = tube_along_arc_hollow::<f64>(
        c(),
        Vec3::unit_y() * 1.5, // not unit: the solid door's NonUnitAxis
        Vec3::unit_x(),
        2.0,
        TubeWindow::Full,
        0.5,
        0.0, // and a zero wall
        Tol::witness(),
    )
    .expect_err("refuses");
    assert!(
        matches!(e, TubeError::NonpositiveWall { .. }),
        "the wall is decided first: {e}"
    );
}

// ---------------------------------------------------------------
// The certified scalar.
// ---------------------------------------------------------------

#[cfg(feature = "interval")]
mod certified {
    use geom_core::Real;
    use geom_core::interval::Interval;

    use super::*;

    fn iv(x: f64) -> Interval {
        <Interval as Real>::from_f64(x)
    }

    fn hollow_iv(
        major: f64,
        window: TubeWindow<Interval>,
        minor: f64,
        wall: f64,
    ) -> Result<Revolved<Interval>, TubeError> {
        tube_along_arc_hollow::<Interval>(
            Point3::new(iv(0.0), iv(0.0), iv(0.0)),
            Vec3::new(iv(0.0), iv(1.0), iv(0.0)),
            Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
            iv(major),
            window,
            iv(minor),
            iv(wall),
            Tol::witness(),
        )
    }

    fn encloses(v: Interval, pad: f64, exact: f64, what: &str) {
        let lo = geom_core::Bounds::lo(v) - pad;
        let hi = geom_core::Bounds::hi(v) + pad;
        assert!(
            lo <= exact && exact <= hi,
            "{what}: {exact} not in [{lo}, {hi}]"
        );
    }

    /// The interval rows at radii the shipped interval rows do not
    /// visit, and with the AREA pinned on the elbow too (the shipped
    /// elbow row pins volume only).
    #[test]
    fn r2_interval_rows_over_varied_radii() {
        for (major, minor, wall, arc) in [
            (2.0_f64, 0.3_f64, 0.1_f64, None::<(f64, f64)>),
            (5.0, 1.25, 0.7, None),
            (2.0, 0.3, 0.1, Some((0.0, 1.5))),
            (10.0, 0.2, 0.01, Some((0.25, 1.75))),
        ] {
            let inner = minor - wall;
            let window = match arc {
                None => TubeWindow::Full,
                Some((t0, t1)) => TubeWindow::Arc {
                    t0: iv(t0),
                    t1: iv(t1),
                },
            };
            let t = hollow_iv(major, window, minor, wall)
                .unwrap_or_else(|e| panic!("R {major} ro {minor} wall {wall} arc {arc:?}: {e}"));
            let what = format!("interval R {major} ro {minor} wall {wall} arc {arc:?}");
            assert_eq!(
                topo::validate_geometric(&t.body, Tol::witness()),
                Ok(()),
                "{what} tier 3"
            );
            let area_sec = PI * (minor * minor - inner * inner);
            let (theta, caps, shells) = match arc {
                None => (2.0 * PI, 0.0, 2),
                Some((t0, t1)) => (t1 - t0, 2.0 * area_sec, 1),
            };
            assert_eq!(t.body.shells().count(), shells, "{what}");
            let m = topo::props::mass_properties(&t.body, Tol::witness()).expect("props");
            encloses(m.volume, m.volume_pad, theta * major * area_sec, &what);
            encloses(
                m.surface_area,
                m.area_pad,
                theta * major * 2.0 * PI * (minor + inner) + caps,
                &what,
            );
        }
    }

    /// The wall funnel decides at the certified scalar too: a wall
    /// whose enclosure straddles the band escalates rather than
    /// building.
    #[test]
    fn r2_interval_wall_still_refuses_and_escalates() {
        assert!(matches!(
            hollow_iv(2.0, TubeWindow::Full, 0.5, 0.0),
            Err(TubeError::NonpositiveWall { .. })
        ));
        assert!(matches!(
            hollow_iv(2.0, TubeWindow::Full, 0.5, 0.6),
            Err(TubeError::WallExceedsRadius { .. })
        ));
        let mut saw_escalate = false;
        let mut w = 1e-300_f64;
        while w < 1e-2 {
            if matches!(
                hollow_iv(2.0, TubeWindow::Full, 0.5, w),
                Err(TubeError::Escalated { .. })
            ) {
                saw_escalate = true;
                break;
            }
            w *= 1.2;
        }
        assert!(saw_escalate, "no in-band wall escalated at Interval");
    }
}
