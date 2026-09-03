//! **Review probes for MESH-12 (R1): is the import route to the
//! rim-continuation condition really dead?** (issue 1588, PR 1617.)
//!
//! The PR's measurement 2 says the import door refuses every
//! `R·Δv ≥ ε` rim row at its pcurve re-mint, before any props decide
//! runs, so `CoherenceCondition::RimContinuation` has no importable
//! witness at any ε row. That measurement was taken on a scratch
//! generator's STEP files which are not committed, so nothing in the
//! tree re-takes it. These rows take it again on a shape the tree DOES
//! carry: `topo/tests/mesh12_rim_row_reach.rs`'s two-level rim cap,
//! written out through `step_export` and read back through
//! `import_step`, plus the re-mint run directly on the Euler-door body
//! so the gate is named rather than inferred from a message.
//!
//! Offsets are derived from the run's own ε; no literal.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use topo::{Body, FaceSurface, MefSite, MevSite};

/// The sphere under every row: R = 10 mm about +Z at the origin.
const RS: f64 = 0.010;
/// The lower rim's latitude.
const V1: f64 = 0.5;

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

/// The unit's own fixture, rebuilt here so this crate can reach it:
/// a sphere whose one rim row is two on-sphere arcs at latitudes `V1`
/// and `V1 + dv`, junctions at the mean latitude.
fn two_level_rim_cap(dv: f64) -> Result<Body<f64>, String> {
    let tol = Tol::witness();
    let vm = V1 + 0.5 * dv;
    let a = p3(RS * vm.cos(), 0.0, RS * vm.sin());
    let b = p3(-RS * vm.cos(), 0.0, RS * vm.sin());
    let rim = |v: f64| Curve3::Circle {
        center: p3(0.0, 0.0, RS * v.sin()),
        axis: v3(0.0, 0.0, 1.0),
        radius: RS * v.cos(),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Sphere {
            center: p3(0.0, 0.0, 0.0),
            radius: RS,
            axis: v3(0.0, 0.0, 1.0),
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let e1 = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim(V1), 0.0, core::f64::consts::PI).unwrap(),
            tol,
        )
        .map_err(|e| format!("{e:?}"))?;
    body.mef(
        MefSite::Chords {
            he1: e1.he_minus,
            he2: e1.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(rim(V1 + dv), core::f64::consts::PI, core::f64::consts::TAU)
            .unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .map_err(|e| format!("{e:?}"))?;
    Ok(body)
}

/// **The gate is the pcurve re-mint, named.** `topo::mint_pcurves` is
/// the pass `import_step` runs on every adopted body; run it directly
/// on the certifying doors' own two-level rim cap and it refuses at
/// `R·Δv = 1.0ε` and `1.5ε` — the same gap the coherence condition
/// reports — while `0.5ε` mints. That is the PR's measurement 2
/// without a STEP file in the way, so the claim rests on the gate
/// rather than on a message's wording.
#[test]
fn r1_the_pcurve_re_mint_refuses_the_rim_row_the_condition_reports() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for (f, want_ok) in [(0.5, true), (1.0, false), (1.5, false)] {
        let mut body = two_level_rim_cap(f * eps / RS).unwrap();
        let r = topo::mint_pcurves(&mut body, tol);
        println!("R1-REMINT f={f} -> {r:?}");
        assert_eq!(r.is_ok(), want_ok, "f = {f}: {r:?}");
        if !want_ok {
            let e = format!("{r:?}");
            assert!(
                e.contains("pcurve_loop_continuity") || e.contains("LoopDiscontinuity"),
                "f = {f}: refused by something else: {e}"
            );
        }
    }
}

/// **And the same shape through the import door itself.** The cap is
/// written out by `step_export` and read back by `import_step`: the
/// `f = 0.5` shape makes the round trip, the `1.0` and `1.5` shapes do
/// not. A row that cannot export at all says so and stops — an export
/// refusal is a fact about the writer's subset, not about the import
/// gate, and must not be read as one.
#[test]
fn r1_the_import_door_does_not_carry_the_two_level_rim_row() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let opts = step_export::StepOptions::default();
    for f in [0.5, 1.0, 1.5] {
        let body = two_level_rim_cap(f * eps / RS).unwrap();
        match step_export::step_string(&body, &opts, tol) {
            Err(e) => println!("R1-IMPORT f={f} export refused: {e:?}"),
            Ok(text) => {
                let r =
                    step_import::import_step(&text, &step_import::ImportOptions::default(), tol);
                println!(
                    "R1-IMPORT f={f} -> {}",
                    r.as_ref()
                        .map(|_| "imported".to_string())
                        .unwrap_or_else(|e| format!("{e:?}"))
                );
                if f > 0.9 {
                    assert!(r.is_err(), "f = {f}: the import door carried the gap");
                }
            }
        }
    }
}

/// **The quiet half.** Below the band the same construction is a body
/// the examination has nothing to say about — the control the
/// measured-dead record needs, since "no witness" and "no examination"
/// are different statements.
#[test]
fn r1_the_sub_band_rim_row_is_quiet_under_the_examination() {
    let tol = Tol::witness();
    let body = two_level_rim_cap(0.5 * tol.eps() / RS).unwrap();
    let report = topo::examine_chart_coherence(&body, tol);
    println!(
        "R1-QUIET findings={} unexamined={}",
        report.findings.len(),
        report.unexamined.len()
    );
    assert!(report.findings.is_empty(), "{:?}", report.findings);
}
