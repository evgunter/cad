//! R1 review probes for MESH-8 (issue 868, PR #1585): the relocated
//! coherence examination against bodies the unit's own rows do not
//! reach.
//!
//! - a WIDER quiet corpus (sweep's test-support bodies, partial
//!   revolutions across the wedge rule, the byte instrument's band);
//! - the issue-1571 body at δ = 0.5, its outcome PRINTED and the one
//!   thing asserted being that no panic there is the deleted
//!   assertion's — run with `walk.rs` at the merge base this row goes
//!   red, which is the BEFORE half of the relocation's evidence. Since
//!   MESH-11 that body no longer reaches the walk at all: the branch
//!   door (`props::require_one_chart_branch`) refuses it typed at
//!   every δ, so what this row now prints is that refusal, and the
//!   assertion — no panic — holds for a second reason;
//! - a sub-ε closure wobble built THROUGH THE EULER DOORS, which the
//!   unit's test header says cannot be constructed: a meridian stated
//!   as a circle whose plane is tilted a hair out of the meridian
//!   plane keeps its arc long (certifiable) while its mid-point
//!   azimuth sits an arbitrarily small angle from its endpoints'.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::*;
use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use profile::{ProfileLoop, ProfileVertex, RawLoop};
use sweep::Revolution;
use topo::{Body, CoherenceCondition, FaceSurface, MefSite, MevSite};

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// The byte instrument's own `band(rho)` body, re-derived here because
/// `r2_bytes.rs` keeps it private.
fn band(rho: f64) -> Body<f64> {
    let (hh, rc) = (0.5f64.sin(), 0.5f64.cos());
    let yt = (1.0 - rho * rho).sqrt();
    let bulge = ((yt.atan2(rho) - hh.atan2(rc)) / 4.0).tan();
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(rc, hh), bulge),
        ProfileVertex::new(p2(rho, yt), 0.0),
        ProfileVertex::new(p2(0.3, 1.3), 0.0),
        ProfileVertex::new(p2(1.1, 0.9), 0.0),
    ]);
    sweep::revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn wider_corpus() -> Vec<(String, Body<f64>)> {
    let tol = Tol::witness();
    let mut out: Vec<(String, Body<f64>)> = vec![
        ("band_0.1".into(), band(0.1)),
        ("band_0.6".into(), band(0.6)),
        ("cube".into(), sweep::test_support::cube(1.0, tol)),
        ("dome".into(), sweep::test_support::dome(1.0, tol)),
        ("lantern".into(), sweep::test_support::lantern(tol)),
        ("swept_elbow".into(), sweep::test_support::swept_elbow(tol)),
        (
            "sphere_zone_full".into(),
            sweep::test_support::sphere_zone(0.5, Revolution::Full, tol),
        ),
        (
            "revolved_dome_profile".into(),
            sweep::test_support::revolved_about_y(
                sweep::test_support::dome_profile(1.0),
                Revolution::Full,
                tol,
            ),
        ),
    ];
    for theta in [0.3, 1.5, core::f64::consts::PI, 4.0, 5.5, 6.2] {
        out.push((format!("sphere_wedge_{theta}"), sphere_wedge(theta)));
        out.push((format!("cone_wedge_{theta}"), cone_wedge(2.0, theta)));
        out.push((
            format!("sphere_zone_partial_{theta}"),
            sweep::test_support::sphere_zone(0.5, Revolution::Partial(theta), tol),
        ));
    }
    out
}

/// **Quiet on a wider corpus than the unit's 14 bodies.** Every body
/// here meshes today (they are other suites' subjects); a finding on
/// one of them is a FALSE POSITIVE of the widened per-edge closure
/// condition, and the assertion prints it in full.
#[test]
fn the_examination_is_quiet_on_a_wider_corpus() {
    let tol = Tol::witness();
    let mut noisy = Vec::new();
    for (name, body) in wider_corpus() {
        let report = topo::examine_chart_coherence(&body, tol);
        println!(
            "PROBE corpus {name}: findings={} unexamined={}",
            report.findings.len(),
            report.unexamined.len()
        );
        if !report.findings.is_empty() || !report.unexamined.is_empty() {
            noisy.push(format!(
                "{name}: {:?} / unexamined {:?}",
                report.findings, report.unexamined
            ));
        }
    }
    assert!(noisy.is_empty(), "eps {}:\n{}", tol.eps(), noisy.join("\n"));
}

/// Issue 1571's body — the half-cap with ONE great-circle arc over the
/// north pole.
///
/// ADOPTION NOTE (not the reviewer's): four copies of this builder now
/// stand — here, `mesh8r2_probes.rs`, `mesh7r1_probes.rs`, and
/// `topo/tests/mesh8_coherence.rs`, where the one-home question is
/// weighed and answered. A `tests/` binary is a separate crate, and
/// the only cross-crate door is `topo`'s `test_support`, gated off for
/// every crate but `topo` on purpose.
fn pole_crossing_half_cap() -> Body<f64> {
    let tol = Tol::witness();
    let z = 0.5_f64;
    let r = (1.0 - z * z).sqrt();
    let a = p3(r, 0.0, z);
    let b = p3(-r, 0.0, z);
    let rim = Curve3::Circle {
        center: p3(0.0, 0.0, z),
        axis: v3(0.0, 0.0, 1.0),
        radius: r,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let great = |axis: Vec3<f64>| Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis,
        radius: 1.0,
        u_ref: v3(-r, 0.0, z),
    };
    let mut g = great(v3(0.0, 1.0, 0.0));
    if g.eval(core::f64::consts::FRAC_PI_2).z < z {
        g = great(v3(0.0, -1.0, 0.0));
    }
    let t_end = g.param_near(a, 0.0).unwrap();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Sphere {
            center: p3(0.0, 0.0, 0.0),
            radius: 1.0,
            axis: v3(0.0, 0.0, 1.0),
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let e_rim = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim, 0.0, core::f64::consts::PI).unwrap(),
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e_rim.he_minus,
            he2: e_rim.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(g, 0.0, t_end).unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

/// Tessellates under `catch_unwind` and returns what happened as text:
/// the panic's first line, or the `Ok`/`Err` shape with `check_mesh`.
fn tessellate_outcome(body: &Body<f64>, delta: f64) -> String {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        mesh::tessellate(body, delta, Tol::witness())
    }));
    std::panic::set_hook(hook);
    match out {
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_string()))
                .unwrap_or_default();
            format!("PANIC {}", msg.lines().next().unwrap_or(""))
        }
        Ok(Ok(m)) => format!(
            "Ok positions={} check_mesh={:?}",
            m.positions.len(),
            mesh::validate::check_mesh(&m)
        ),
        Ok(Err(e)) => format!("Err({e:?})"),
    }
}

/// **The BEFORE / AFTER half of the relocation, at δ = 0.5 on issue
/// 1571's body.** Printed, and one thing asserted: whatever says
/// something here, it is not the deleted `closing_column` assertion.
/// With `crates/mesh/src/walk.rs` checked out at the merge base this
/// row goes RED with that assertion's text, which is the "before".
#[test]
fn the_pole_crossing_body_at_delta_half_is_not_announced_by_the_deleted_assertion() {
    let body = pole_crossing_half_cap();
    let report = topo::examine_chart_coherence(&body, Tol::witness());
    println!(
        "PROBE 1571 debug_assertions={} report findings={} {:?}",
        cfg!(debug_assertions),
        report.findings.len(),
        report
            .findings
            .iter()
            .map(|f| (f.condition, f.gap, f.lever, f.metres))
            .collect::<Vec<_>>()
    );
    // The tiers, for the record: `topo::coherence`'s header says both
    // π-rad witnesses "certify green through tier 3"; issue 1571's own
    // text says its Euler-door bodies were tier-3 refused at rest for
    // construction artefacts. Printed, whichever it is.
    println!(
        "PROBE 1571 tiers: validate={:?} validate_closed={:?} validate_geometric={:?}",
        topo::validate(&body).map_err(|e| e.len()),
        topo::validate_closed(&body).map_err(|e| e.len()),
        topo::validate_geometric(&body, Tol::witness()).map_err(|e| format!("{e:?}"))
    );
    for delta in [0.5, 0.1] {
        let got = tessellate_outcome(&body, delta);
        println!("PROBE 1571 delta={delta}: {got}");
        assert!(
            !got.contains("closing meridian's carrier-midpoint azimuth"),
            "the deleted assertion is what announced this body: {got}"
        );
    }
    assert_eq!(
        report
            .findings
            .iter()
            .filter(|f| matches!(f.condition, CoherenceCondition::MeridianClosure { .. }))
            .count(),
        2
    );
}

/// A cylinder face — rim arc `v0 → v1` at z = 0, meridian line
/// `v1 → v2`, rim arc back `v2 → v3` at z = 1 — closed by a MERIDIAN
/// stated as a CIRCLE through `v3` and `v0` whose plane is tilted by
/// `phi` out of the meridian plane. Both endpoints lie exactly on the
/// carrier and on the cylinder; the carrier's mid-point sits at
/// azimuth ≈ `0.105 · phi`, which is the closure gap, at lever arm 1.
///
/// The arc is long (~53° of a radius-1.118 circle), so its span
/// certifies at any band, and the gap is set by `phi` alone.
fn tilted_meridian_sliver(phi: f64) -> Body<f64> {
    let tol = Tol::witness();
    let on = |theta: f64, z: f64| p3(theta.cos(), theta.sin(), z);
    let rim = |z: f64| Curve3::Circle {
        center: p3(0.0, 0.0, z),
        axis: v3(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let rim_back = |z: f64| Curve3::Circle {
        center: p3(0.0, 0.0, z),
        axis: v3(0.0, 0.0, -1.0),
        radius: 1.0,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let th = 0.8_f64;
    let (v0, v1, v2, v3_) = (on(0.0, 0.0), on(th, 0.0), on(th, 1.0), on(0.0, 1.0));
    // The tilted circle: centre one unit inward from the segment's
    // midpoint along `w`, a horizontal direction `phi` off the
    // outward radial; radius sqrt(1 + 0.25).
    let w = v3(phi.cos(), phi.sin(), 0.0);
    let m = p3(1.0, 0.0, 0.5);
    let c = m - w * 1.0;
    let rr = 1.25_f64.sqrt();
    let u_ref = (v3_ - c) * (1.0 / rr);
    let make = |axis: Vec3<f64>| Curve3::Circle {
        center: c,
        axis,
        radius: rr,
        u_ref,
    };
    let n = v3(0.0, 0.0, 1.0).cross(w);
    let mut circ = make(n);
    let mut t0 = circ.param_near(v0, 0.0).unwrap();
    if t0 < 0.0 {
        circ = make(n * -1.0);
        t0 = circ.param_near(v0, 0.0).unwrap();
    }
    assert!(t0 > 0.0 && t0 < 1.5, "the short arc, got {t0}");
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(v0).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Cylinder {
            origin: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let e01 = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            v1,
            EdgeCurveSpec::arc_of_circle(rim(0.0), 0.0, th).unwrap(),
            tol,
        )
        .unwrap();
    let strut = |body: &mut Body<f64>, at, to, spec| {
        body.mev(MevSite::Fan { he1: at, he2: at }, to, spec, tol)
            .unwrap()
    };
    let e12 = strut(
        &mut body,
        e01.he_minus,
        v2,
        EdgeCurveSpec::line_between(v1, v2),
    );
    let e23 = strut(
        &mut body,
        e12.he_minus,
        v3_,
        EdgeCurveSpec::arc_of_circle(rim_back(1.0), -th, 0.0).unwrap(),
    );
    body.mef(
        MefSite::Chords {
            he1: e23.he_minus,
            he2: e01.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(circ, 0.0, t0).unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

/// **A sub-ε closure wobble IS constructible through the Euler doors.**
/// `topo/tests/mesh8_coherence.rs`'s header says no row there can be
/// one because "an arc's span must certify, which puts a floor of
/// order ε on the arc length and therefore on the gap a chord can
/// open" — true of a CHORD, whose gap is half its span, and false of
/// a circle tilted out of the meridian plane, whose gap is set by the
/// tilt while the span stays long. At 0.25 ε the report is empty (the
/// band's quiet side, at the body level, which the unit pins only at
/// the predicate); at 4 ε the same body reports four closure findings.
#[test]
fn a_tilted_meridian_circle_opens_a_closure_gap_of_any_size() {
    let tol = Tol::witness();
    let eps = tol.eps();
    // gap ≈ 0.105 · phi at lever 1, so phi ≈ metres / 0.105.
    let phi_for = |metres: f64| metres / 0.105;
    let quiet = topo::examine_chart_coherence(&tilted_meridian_sliver(phi_for(0.25 * eps)), tol);
    println!(
        "PROBE tilted 0.25eps: findings={} {:?}",
        quiet.findings.len(),
        quiet
            .findings
            .iter()
            .map(|f| (f.gap, f.lever, f.metres))
            .collect::<Vec<_>>()
    );
    assert!(quiet.unexamined.is_empty(), "{:?}", quiet.unexamined);
    assert!(
        quiet.findings.is_empty(),
        "a quarter-band wobble must be noise: {:?}",
        quiet.findings
    );

    let loud_body = tilted_meridian_sliver(phi_for(4.0 * eps));
    let loud = topo::examine_chart_coherence(&loud_body, tol);
    println!(
        "PROBE tilted 4eps: findings={} {:?}",
        loud.findings.len(),
        loud.findings
            .iter()
            .map(|f| (f.condition, f.gap, f.lever, f.metres))
            .collect::<Vec<_>>()
    );
    let closures: Vec<_> = loud
        .findings
        .iter()
        .filter(|f| matches!(f.condition, CoherenceCondition::MeridianClosure { .. }))
        .collect();
    assert_eq!(
        closures.len(),
        4,
        "two endpoints, two faces: {:?}",
        loud.findings
    );
    for f in &closures {
        assert!(
            (0.5 * 4.0 * eps..2.0 * 4.0 * eps).contains(&f.metres),
            "aimed at 4 eps = {}, got {} m",
            4.0 * eps,
            f.metres
        );
    }
    // What the mesh side says about the loud body — printed, not
    // asserted: it is either refused typed by the shape door or meshed
    // with nothing in `mesh` saying a word, and which one it is tells
    // the reader whether anything is "silent" here.
    for delta in [0.5, 0.05] {
        println!(
            "PROBE tilted 4eps tessellate delta={delta}: {}",
            tessellate_outcome(&loud_body, delta)
        );
    }
}
