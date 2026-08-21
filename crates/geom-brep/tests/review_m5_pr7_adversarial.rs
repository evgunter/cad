//! **Promoted PR 7 review probe** (adversarial reviewer, adopted
//! verbatim as a committed row per the standing review policy).

//! REVIEW PROBE (PR 7): adversarial small-loop variants of shape (iv).
//!
//! The contract under attack: found or refused typed — never silent.
//! "Silent" here means `Ok` with fewer branches than the geometry has.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    // Adopted verbatim from the reviewer's probe: the indexing and
    // cast shapes are theirs, and rewriting them would weaken the
    // "independent second implementation" the row exists to be.
    clippy::needless_range_loop,
    clippy::unnecessary_cast
)]

use geom_brep::ssi::{self, BranchEnd, SSI_MAX_FIT_SAMPLES, SsiDomain, SsiError};
use geom_core::tolerance::DEFAULT_EPS;
use geom_core::{Band, Point3, Tolerance, Vec3};
use test_utils::vacuity;

/// The accounting floor this file's floor-clamped fixture plants, **in
/// metres** — above the tube radius of a 0.008 m cylinder, and the same
/// width at every ε of the battery.
const FLOOR_CLAMP_METRES: f64 = 0.1;

fn eps() -> f64 {
    Tolerance::get().eps
}

fn band() -> Band {
    Band::linear().unwrap()
}

fn sphere() -> Surface {
    geom::Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

type Surface = geom::Surface<f64>;

fn run(cyl: Surface, d: SsiDomain) -> Result<geom_brep::SsiOutcome, SsiError> {
    ssi::cylinder_sphere_ssi(&cyl, &sphere(), d, band())
}

/// A loop pair MUCH smaller than the seed floor (extent/64 = 0.031 m;
/// the cylinder is 0.02 m across): found-ness must not depend on the
/// seed grid being finer than the feature.
#[test]
fn a_loop_smaller_than_the_seed_floor_is_found_or_refused_typed() {
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.01, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.008,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let d = SsiDomain {
        center: Point3::new(0.0, 0.0, 0.0),
        half_extent: 1.5,
        extent: 2.0,
        floor_scale: 1.0,
    };
    match run(cyl, d) {
        Ok(out) => {
            assert_eq!(
                out.branches.len(),
                2,
                "SILENT MISS: the tiny-loop pair has two components, got {} \
                 (exhaustiveness {:?}, seeds {})",
                out.branches.len(),
                out.exhaustiveness,
                out.seeds
            );
            for b in &out.branches {
                assert_eq!(b.end, BranchEnd::Closed);
                assert!(b.certificate.hull_sup <= eps(), "{:?}", b.certificate);
            }
            let z: Vec<f64> = out.branches.iter().map(|b| b.witness.z).collect();
            assert!(z[0] * z[1] < 0.0, "one loop per pole: {z:?}");
        }
        Err(e) => {
            // Typed refusal is acceptable; silence is not.
            println!("typed refusal (acceptable): {e}");
        }
    }
}

/// The same pair with the domain center dithered so the loops straddle
/// subdivision-cell boundaries instead of sitting at symmetric centers.
#[test]
fn a_loop_straddling_cell_boundaries_is_found_or_refused_typed() {
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.03, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.08,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let d = SsiDomain {
        center: Point3::new(0.0137, -0.0261, 0.0407),
        half_extent: 1.47,
        extent: 1.9,
        floor_scale: 1.0,
    };
    match run(cyl, d) {
        Ok(out) => {
            assert_eq!(
                out.branches.len(),
                2,
                "SILENT MISS under a dithered domain: got {} (exhaustiveness {:?})",
                out.branches.len(),
                out.exhaustiveness
            );
        }
        Err(e) => println!("typed refusal (acceptable): {e}"),
    }
}

/// Reproduce the reported seed count on the one-loop slab (the
/// floor-refusal fixture's domain, healthy floor).
#[test]
fn one_loop_slab_seed_count_reproduces() {
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.03, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.08,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let d = SsiDomain {
        center: Point3::new(0.03, 0.0, 0.996),
        half_extent: 0.2,
        extent: 0.4,
        floor_scale: 1.0,
    };
    match run(cyl, d) {
        Ok(out) => println!(
            "one-loop slab: seeds = {}, branches = {}, exhaustiveness = {:?}",
            out.seeds,
            out.branches.len(),
            out.exhaustiveness
        ),
        Err(e) => println!("one-loop slab refused: {e}"),
    }
}

/// My own floor-refusal variant on the tiny pair.
///
/// **The clamp is stated in metres.** `SsiDomain::floor` is
/// `SSI_FLOOR · band.zero() · floor_scale`, so a literal scale names a
/// different width at every ε — the `1.0e8` this row carried is 0.1 m at
/// the compiled default and **1e-4 m at ε = 1e-12**, which is *below*
/// the 0.008 m cylinder's tube radius. The `Ok` arm's own message is
/// *"a floor above the tube radius must refuse"*: a premise about a
/// width, false at the fine end of the battery, on a row that would then
/// have panicked for the wrong reason or stood down in silence.
///
/// **And the stand-down is proved rather than assumed**, the way this
/// suite's sibling floor rows are: a refusal that is not the floor's is
/// only admissible when it is D9's fit budget, genuinely overrun, at an
/// ε finer than the compiled default. Anything else reds.
#[test]
fn the_tiny_pair_floor_variant_refuses_typed() {
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.01, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 0.008,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let d = SsiDomain {
        center: Point3::new(0.01, 0.0, 0.9999),
        half_extent: 0.05,
        extent: 0.1,
        floor_scale: SsiDomain::floor_scale_for(FLOOR_CLAMP_METRES, band()),
    };
    match run(cyl, d) {
        Err(SsiError::ExhaustivenessInconclusive { .. }) => {}
        Err(SsiError::FitSampleBudget { samples, budget }) => {
            assert_eq!(
                budget, SSI_MAX_FIT_SAMPLES,
                "the stand-down is D9's fit budget or it is not a stand-down"
            );
            assert!(samples > budget, "{samples} of {budget} is not an overrun");
            assert!(
                eps() < DEFAULT_EPS,
                "the fit budget fired at ε = {:e}, which is not finer than the compiled \
                 default {DEFAULT_EPS:e} — the floor claim is REACHABLE here and this row \
                 owes it, not a stand-down",
                eps()
            );
            vacuity::stood_down(
                &format!("the tiny-pair floor variant, eps = {:e}", eps()),
                &format!(
                    "the fit budget ({samples} of {budget} samples) refused before any \
                     branch was fitted, so THIS RUN ASSERTS NEITHER that the clamped \
                     {FLOOR_CLAMP_METRES} m floor refuses NOR what its refusal says"
                ),
            );
        }
        Err(e) => panic!(
            "a {FLOOR_CLAMP_METRES} m floor on a 0.008 m cylinder must refuse for the \
             floor's own reason, or stand down on D9's fit budget; got {e}"
        ),
        Ok(out) => panic!(
            "a floor above the tube radius must refuse, got Ok with {} branches",
            out.branches.len()
        ),
    }
}
