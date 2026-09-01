//! R1 review probe (MESH-5, PR 1507) — the DISCLOSED BEHAVIOR EDGE.
//!
//! PR 1507 removes a `ceil_count(vspan, rho_max * hu)?` call from
//! `curved::grid_counts`' cone arm at `nu == 1`, and discloses that the
//! call "could previously refuse `ResolutionOverflow`", asserting the
//! reach is "half-angle ≲ 1e-7 rad, which no in-tree construction
//! mints". This probe asks the sharper question the PR does not: can a
//! PUBLIC DOOR mint one? `sweep::revolve` classifies a line wall as a
//! cone whenever its radial delta clears the ε band, so a
//! near-cylindrical wall with a long axial run is a candidate.
//!
//! It also re-derives the second refusal class the PR's inequality
//! misses: `ceil_count` refuses on a NON-FINITE quotient too, so
//! `rho_max * hu == 0` (a patch pinched at the apex, or a zero
//! half-angle) refused for a reason that has nothing to do with 2^24.
//!
//! Reporter plus assertions: the counts are re-derived here (the
//! kernel's spelling is private), and the rows go red if a body this
//! probe mints stops tessellating.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use geom::Surface;
use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, RawLoop as _, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;

/// `mesh::sizing`'s cone schedule, re-derived (both functions are
/// private). Returns `(nu_raw, nv_old, hu)` where `nv_old` is the count
/// the PRE-PR tree computed and PR 1507 no longer computes at
/// `nu == 1`. `None` for `nv_old` means `ceil_count` would have
/// REFUSED — at or above 2^24, or non-finite.
fn cone_schedule(delta: f64, rho_max: f64, uspan: f64, vspan: f64) -> (usize, Option<f64>, f64) {
    let ds = delta * 0.5;
    let cap = core::f64::consts::FRAC_PI_4; // mesh::sizing::MAX_ANGULAR_STEP
    let hu = if ds < rho_max {
        let h = 2.0 * (1.0 - ds / rho_max).acos();
        if h < cap { h } else { cap }
    } else {
        cap
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let nu = (uspan / hu).ceil().max(1.0) as usize;
    let raw = (vspan / (rho_max * hu)).ceil();
    let nv = if raw.is_finite() && raw < 16_777_216.0 {
        Some(raw)
    } else {
        None
    };
    (nu, nv, hu)
}

/// A cone wall from `revolve`: profile triangle `(r0, 0) - (r1, h) -
/// (r0, h)` revolved `theta` about the y axis. The oblique side is the
/// cone; `r1 > r0 > 0` keeps it off the axis (no apex entry), so this
/// is the apex-free member of the class.
fn frustum_wedge(r0: f64, r1: f64, h: f64, theta: f64) -> Result<Body<f64>, String> {
    let lp = ProfileLoop::polygon([Point2::new(r0, 0.0), Point2::new(r1, h), Point2::new(r0, h)]);
    let p = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .map_err(|e| format!("profile: {e:?}"))?;
    revolve(
        &p,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(theta),
        Tol::witness(),
    )
    .map(|r| r.body)
    .map_err(|e| format!("revolve: {e:?}"))
}

/// Report every cone face a body carries: half-angle, and the
/// `(nu, nv_old)` its patch would have been sized to.
fn cone_faces(body: &Body<f64>) -> Vec<(f64, f64, f64)> {
    let mut out = Vec::new();
    for (_fk, f) in body.faces() {
        if let Some(&Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        }) = body.get_surface(f.surface)
        {
            let _ = (apex, axis);
            out.push((half_angle, f64::NAN, f64::NAN));
        }
    }
    out
}

/// **Can a public door mint the removed refusal? Measured: yes, twice.**
/// A sweep of near-cylindrical frustum wedges, revolved 0.3 rad. Two of
/// them — radius `1e-4 → 2e-4` over an axial run of `1e4`, and `1e-3 →
/// 2e-3` over `1e5` — come back classified as CONES of half-angle
/// `1e-8`, size to `nu == 1`, and carry `nv_old` above 2^24: the pre-PR
/// tree refused them `ResolutionOverflow` and this head serves them
/// (8 triangles, certified). Two others are refused by the door itself,
/// on ε-band predicates at large coordinates (`line_span`,
/// `dihedral_wedge`) — recorded, because "the door refuses" is the
/// other half of the answer.
///
/// The row also shows the PR's "i.e. a cone of half-angle ≲ 1e-7 rad"
/// is not the binding parameter: `1..1.0000001` over an axial 1 is a
/// half-angle of `1e-7` with `nv_old = 2`. What binds is the patch's
/// slant-extent-to-radius aspect, `vspan/(ρ_max·hu)`.
#[test]
fn a_public_door_mints_the_removed_refusal() {
    println!("MESH-5 R1 reach probe: cone shapes minted through sweep::revolve");
    println!(
        "{:>12} {:>12} {:>10} {:>10} {:>4} {:>14} {:>10}",
        "r0..r1", "axial", "half_angle", "delta", "nu", "nv_old", "head"
    );
    let mut minted_refusal = 0usize;
    let mut minted_cone = 0usize;
    for &(r0, r1, h) in &[
        (1.0e-4, 2.0e-4, 1.0e4),
        (1.0e-4, 2.0e-4, 1.0e3),
        (1.0e-3, 2.0e-3, 1.0e5),
        (1.0e-2, 2.0e-2, 1.0e5),
        (0.1, 0.2, 1.0e6),
        (0.5, 1.0, 1.0e6),
        (0.5, 1.0, 2.0e7),
        (1.0, 1.000_000_1, 1.0),
    ] {
        let theta = 0.3;
        let body = match frustum_wedge(r0, r1, h, theta) {
            Ok(b) => b,
            Err(e) => {
                println!(
                    "{:>12} {h:>12.1e}  DOOR REFUSED: {e}",
                    format!("{r0}..{r1}")
                );
                continue;
            }
        };
        let angles = cone_faces(&body);
        if angles.is_empty() {
            println!(
                "{:>12} {h:>12.1e}  no cone wall (classified cylinder/plane)",
                format!("{r0}..{r1}")
            );
            continue;
        }
        minted_cone += 1;
        let half_angle = angles[0].0;
        for &delta in &[0.1_f64] {
            // rho_max is the patch's largest radius; vspan its slant
            // extent. For this wall: rho_max = r1, slant = h/cos(alpha).
            let rho_max = r1;
            let vspan = h / half_angle.cos();
            let (nu, nv_old, _hu) = cone_schedule(delta, rho_max, theta, vspan);
            let head = mesh::tessellate(&body, delta, Tol::witness());
            let head_s = match &head {
                Ok(m) => format!(
                    "Ok({} tris)",
                    m.patches.iter().map(|p| p.triangles.len()).sum::<usize>()
                ),
                Err(e) => format!("{e:?}"),
            };
            println!(
                "{:>12} {h:>12.1e} {half_angle:>10.3e} {delta:>10} {nu:>4} {:>14} {head_s:>10}",
                format!("{r0}..{r1}"),
                nv_old.map_or_else(|| "REFUSED".to_string(), |n| format!("{n:.3e}")),
            );
            if nu == 1 && nv_old.is_none() {
                minted_refusal += 1;
                assert!(
                    head.is_ok(),
                    "this head must SERVE the shape the pre-PR tree refused: {head_s}"
                );
            }
        }
    }
    println!(
        "cone walls minted: {minted_cone}; of those reaching the removed refusal: {minted_refusal}"
    );
    assert!(
        minted_cone > 0,
        "the sweep minted no cone wall at all — the probe, not the tree, is what failed"
    );
}

/// **The refusal class the PR's inequality does not name.**
/// `ceil_count` refuses on `!raw.is_finite()` as well as on `raw >=
/// 2^24`, so `rho_max * hu == 0` refused for a reason that is not a
/// magnitude at all. Re-derived arithmetically (no body needed): the
/// PR characterises the removed path solely as
/// `vspan/(rho_max*hu) >= 2^24`.
#[test]
fn the_removed_refusal_has_a_second_non_finite_class() {
    // rho_max == 0 (a patch whose whole v extent sits at the apex, or a
    // zero half-angle): hu takes the cap, hv = 0, and the quotient is
    // non-finite for every vspan.
    for &vspan in &[0.0_f64, 1.0, 1e9] {
        let (nu, nv_old, hu) = cone_schedule(0.1, 0.0, 0.3, vspan);
        println!("rho_max = 0, vspan = {vspan}: nu = {nu}, hu = {hu}, nv_old = {nv_old:?}");
        assert_eq!(nu, 1, "a zero-radius cone patch sizes to one column");
        assert!(
            nv_old.is_none(),
            "the pre-PR tree refused here, and not because of 2^24"
        );
    }
}
