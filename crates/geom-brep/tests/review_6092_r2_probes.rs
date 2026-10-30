//! Review probes for PR 6092 (per-span round-zero schedules), lane r2.
//!
//! Every row drives the public patch door and holds what it returns
//! against the shared independent oracle (`shared::patch::oracle_patch`,
//! plain-f64 Cox–de Boor and composite Gauss–Legendre per knot span). A
//! bracket that excludes the oracle is the failure, whether the lane
//! accepted it or stopped at its wall; a typed refusal is allowed and
//! printed.
//!
//! Slow: about 40 min in release on head (the multi-span rows run to
//! the per-span-cell wall). A review probe, not a gate row.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/props/",
    "crates/geom-core/src/spline/",
    "crates/geom-brep/tests/shared/",
];

use crate::shared::patch::{oracle_patch, patch_net, to_target};
use crate::shared::ring::p3 as p;
use crate::shared::tol::band;
use geom_brep::props::quad::{
    FaceCutBounds, LaneRun, LaneStop, mean_boundary_displacement, nurbs_patch_face,
};
use geom_core::Bounds;
use geom_core::Interval;
use geom_core::Tol;
use geom_core::spline::KnotVector;

/// A deterministic weight in `[lo, hi]` on a log scale, keyed on the
/// node: no RNG crate, so the row is the same bits everywhere.
fn log_weight(i: usize, j: usize, lo: f64, hi: f64) -> f64 {
    let h = ((i * 7919 + j * 104_729 + 13) % 1000) as f64 / 999.0;
    (lo.ln() + (hi.ln() - lo.ln()) * h).exp()
}

/// A curved net over `ku × kv`, translated by `off`.
fn net(
    ku: &KnotVector,
    kv: &KnotVector,
    off: f64,
    wf: &dyn Fn(usize, usize) -> f64,
) -> (Vec<[Interval; 3]>, Vec<f64>) {
    let (nu, nv) = (ku.control_count(), kv.control_count());
    let mut cp = Vec::new();
    let mut ws = Vec::new();
    for i in 0..nu {
        for j in 0..nv {
            let x = i as f64 / (nu - 1) as f64;
            let y = j as f64 / (nv - 1) as f64;
            let z = 0.35 * (x - 0.5) * (x - 0.5) + 0.25 * y * y + 0.30 * x * y;
            cp.push(p(off + 2.0 * x, off + 1.5 * y, off + 1.0 + z));
            ws.push(wf(i, j));
        }
    }
    (cp, ws)
}

fn kv(deg: usize, interior: &[f64]) -> KnotVector {
    let mut k = vec![0.0; deg + 1];
    k.extend_from_slice(interior);
    k.extend(vec![1.0; deg + 1]);
    KnotVector::clamped(k, deg).unwrap()
}

enum Mode {
    /// The reporting target at the run's ε.
    Target,
    /// Never accept: run to the wall or the floor.
    Never,
}

/// Drive the door; hold whatever bracket comes back against the oracle.
fn hold(
    name: &str,
    ku: &KnotVector,
    kv: &KnotVector,
    cp: &[[Interval; 3]],
    ws: &[f64],
    acc: Mode,
) -> Option<LaneRun> {
    let (ua, ub) = ku.domain();
    let (va, vb) = kv.domain();
    let eps = Tol::witness().get().eps;
    let mut target = to_target(eps);
    let mut never = |_: &FaceCutBounds, _: LaneStop| Ok(false);
    let t0 = std::time::Instant::now();
    let out = nurbs_patch_face::<f64>(
        &patch_net(ku, kv, cp, ws),
        (ua, ub, va, vb),
        0.0,
        0.0,
        band(),
        match acc {
            Mode::Target => &mut target as &mut geom_brep::props::quad::Accept<'_>,
            Mode::Never => &mut never,
        },
    );
    let dt = t0.elapsed();
    let run = match out {
        Ok(run) => run,
        Err(e) => {
            println!("PROBE {name}: REFUSED {e} ({dt:?})");
            return None;
        }
    };
    let o = oracle_patch(ku, kv, cp, ws);
    // An adaptive ladder: the coarsest pair a factor of two apart that
    // agrees to 1e-8 relative is believed; none up to 256 cells a span,
    // and the row prints the bracket without asserting containment.
    let mut prev = o.dense(16);
    let mut settled = None;
    for cells in [32, 64, 128, 256] {
        let next = o.dense(cells);
        let ok = |a: f64, b: f64| (a - b).abs() <= 1e-8 * (1.0 + b.abs());
        if ok(prev.0, next.0) && ok(prev.1, next.1) {
            settled = Some((prev, next, cells));
            break;
        }
        prev = next;
    }
    let (fl, ar) = (run.bounds.flux, run.bounds.area);
    let Some(((f1, a1), (f2, a2), cells)) = settled else {
        println!(
            "PROBE {name}: ORACLE UNSETTLED at 256 cells | {:?} flux [{:.6e}, {:.6e}] oracle~{:.9e} area [{:.6e}, {:.6e}] oracle~{:.9e}",
            run.end,
            fl.lo(),
            fl.hi(),
            prev.0,
            ar.lo(),
            ar.hi(),
            prev.1
        );
        return Some(run);
    };
    println!(
        "PROBE {name}: {:?} round {} of {} | flux [{:.6e}, {:.6e}] w {:.3e} oracle {f2:.12e} | area [{:.9e}, {:.9e}] w {:.3e} oracle {a2:.12e} | disp {:.3e} | oracle@{cells} | {dt:?}",
        run.end,
        run.round,
        run.rounds,
        fl.lo(),
        fl.hi(),
        fl.hi() - fl.lo(),
        ar.lo(),
        ar.hi(),
        ar.hi() - ar.lo(),
        mean_boundary_displacement(fl, ar).unwrap_or(f64::NAN),
    );
    // The oracle's own error, read off its 16/32 ladder, is the slack a
    // containment miss must clear.
    let fs = 10.0 * (f1 - f2).abs() + 1e-12 * (1.0 + f2.abs());
    let as_ = 10.0 * (a1 - a2).abs() + 1e-12 * (1.0 + a2.abs());
    assert!(
        f2 >= fl.lo() - fs && f2 <= fl.hi() + fs,
        "UNSOUND {name}: flux {f2:.17e} outside [{:.17e}, {:.17e}]",
        fl.lo(),
        fl.hi()
    );
    assert!(
        a2 >= ar.lo() - as_ && a2 <= ar.hi() + as_,
        "UNSOUND {name}: area {a2:.17e} outside [{:.17e}, {:.17e}]",
        ar.lo(),
        ar.hi()
    );
    Some(run)
}

/// **Claim 2, rational lane**: extreme weights (1e-3..1e3 on a log
/// scale), non-dyadic spans of very unequal length (1e-4 beside 0.5),
/// a double (C⁰) knot, and translations up to 1e6 m: every bracket
/// returned, accepted or stopped, holds the oracle.
#[test]
fn rational_extreme_weights_unequal_spans_and_far_translations_hold_the_oracle() {
    let ku = kv(2, &[0.1234, 0.1235, 0.61, 0.61]);
    let kvv = kv(2, &[0.0007, 0.3333]);
    for &(lo, hi) in &[(1.0, 1.0 + 1e-9), (0.5, 2.0), (1e-3, 1e3)] {
        for &off in &[0.0, 1e3, 1e6] {
            let (cp, ws) = net(&ku, &kvv, off, &|i, j| log_weight(i, j, lo, hi));
            hold(
                &format!("rational w[{lo:e},{hi:e}] off {off:e}"),
                &ku,
                &kvv,
                &cp,
                &ws,
                Mode::Target,
            );
        }
    }
}

/// The same, one span a direction, run to the wall: the one-span face
/// is where the per-span constants halved (4 hull blocks, 32 area
/// cells), so its at-the-wall bracket is the one to hold.
#[test]
fn rational_one_span_extreme_weights_hold_the_oracle_at_the_wall() {
    let ku = kv(2, &[]);
    let kvv = kv(2, &[]);
    for &(lo, hi) in &[(0.5, 2.0), (1e-3, 1e3)] {
        for &off in &[0.0, 1e6] {
            let (cp, ws) = net(&ku, &kvv, off, &|i, j| log_weight(i, j, lo, hi));
            hold(
                &format!("one-span rational w[{lo:e},{hi:e}] off {off:e} NEVER"),
                &ku,
                &kvv,
                &cp,
                &ws,
                Mode::Never,
            );
        }
    }
}

/// **Claim 2, polynomial composite arm** (degree > 4 in u takes it):
/// hulls now per hull block. Unequal non-dyadic spans, a C⁰ knot, far
/// translations, run to the wall.
#[test]
fn polynomial_composite_arm_holds_the_oracle_at_the_wall() {
    let ku = kv(5, &[0.0003, 0.4142, 0.4142, 0.4142, 0.4142, 0.4142]);
    let kvv = kv(2, &[0.271]);
    for &off in &[0.0, 1e6] {
        let (cp, ws) = net(&ku, &kvv, off, &|_, _| 1.0);
        hold(
            &format!("poly (5,2) off {off:e} target"),
            &ku,
            &kvv,
            &cp,
            &ws,
            Mode::Target,
        );
    }
}

/// **Near-coincident knots**: the per-span refinement now splits a
/// span `g` wide at its midpoint, however narrow, where the old
/// domain grid skipped a point within its clearance of a knot. Prints
/// the round-0..wall widths across `g` so a widening near `g → 0` shows.
#[test]
fn near_coincident_knots_hold_the_oracle_and_print_their_width() {
    for &g in &[1e-2, 1e-5, 1e-8, 1e-11, 1e-14] {
        let ku = kv(2, &[0.37, 0.37 + g]);
        let kvv = kv(2, &[0.5]);
        let (cp, ws) = net(&ku, &kvv, 0.0, &|i, j| log_weight(i, j, 0.5, 2.0));
        hold(
            &format!("near-coincident u knots g {g:e} rational"),
            &ku,
            &kvv,
            &cp,
            &ws,
            Mode::Target,
        );
        let (cp, ws) = net(&ku, &kvv, 0.0, &|i, _| 1.0 + 0.1 * i as f64);
        hold(
            &format!("near-coincident u knots g {g:e} w(u) exact-v"),
            &ku,
            &kvv,
            &cp,
            &ws,
            Mode::Target,
        );
    }
}
