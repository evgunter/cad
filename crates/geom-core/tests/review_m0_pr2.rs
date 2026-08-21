//! Adversarial e2e review artifact for M0 PR 2 (`Real` trait +
//! `Tolerance`, 2026-07-15/16), salvaged from the M0 orchestrator session
//! scratchpad and promoted into CI 2026-07-16 (archived at
//! `references/review-artifacts-m0/pr2-review-demos/`). These are
//! **independent derivations** — do not "simplify" them to match shipped
//! fixtures; the independence is the regression value.
//!
//! Salvage adaptations: the review predates the D4 revision that removed
//! the global angular tolerance, so every `eps_angular` /
//! `CAD_TOLERANCE_EPS_ANGULAR` probe is dropped; `env_init_error()` became
//! `env_init_errors()` (a slice). The review's cross-process
//! `tol_driver`/`tol_probe` harness is reproduced here by re-executing
//! this test binary with `--exact` (each probe needs a fresh process —
//! the global tolerance commits exactly once per process).
//!
//! # The forbidden surface (compile-fail probes, documentary)
//!
//! The review's `forbidden/` crate proved these do **not** compile against
//! `T: Real` — the no-comparison / no-extraction discipline is structural.
//! They are kept in `compile_fail` doctest form to match the repo idiom
//! (see `crates/topo/src/entity.rs`), but note rustdoc does not run
//! integration-test doctests, so these are documentation of the review's
//! findings, not executed gates.
//!
//! `Real` exposes no ordering — `a < b` fails to typecheck:
//!
//! ```compile_fail,E0369
//! use geom_core::Real;
//! pub fn cmp<T: Real>(a: T, b: T) -> bool { a < b }
//! ```
//!
//! ...no equality:
//!
//! ```compile_fail,E0369
//! use geom_core::Real;
//! pub fn eq<T: Real>(a: T, b: T) -> bool { a == b }
//! ```
//!
//! ...no `Ord`-family sorting (`sort` needs `Ord`, `sort_by` needs
//! `partial_cmp` from `PartialOrd`):
//!
//! ```compile_fail,E0277
//! use geom_core::Real;
//! pub fn sort_them<T: Real>(v: &mut Vec<T>) { v.sort(); }
//! ```
//!
//! ```compile_fail,E0599
//! use geom_core::Real;
//! pub fn sort_them<T: Real>(v: &mut Vec<T>) {
//!     v.sort_by(|a, b| a.partial_cmp(b).unwrap());
//! }
//! ```
//!
//! ...no `Sum` (accumulation is written as an explicit fold):
//!
//! ```compile_fail,E0277
//! use geom_core::Real;
//! pub fn total<T: Real>(v: &[T]) -> T { v.iter().copied().sum() }
//! ```
//!
//! ...no value-level sign extraction and no scalar collapse:
//!
//! ```compile_fail,E0599
//! use geom_core::Real;
//! pub fn sig<T: Real>(a: T) -> T { a.signum() }
//! ```
//!
//! ```compile_fail,E0599
//! use geom_core::Real;
//! pub fn extract<T: Real>(a: T) -> f64 { a.to_f64() }
//! ```
//!
//! ```compile_fail,E0605
//! use geom_core::Real;
//! pub fn extract<T: Real>(a: T) -> f64 { a as f64 }
//! ```
//!
//! Two review findings that DO compile are pinned as runtime tests below
//! instead: the `extra_bound` hole (`T: Real + PartialOrd` compiles and
//! only fails at scalars lacking `PartialOrd`, e.g. the interval scalar)
//! and the Debug/`Any` extraction holes
//! ([`documented_holes_in_the_real_surface`]).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::tolerance::{DEFAULT_EPS, ENV_EPS};
use geom_core::{Real, Tolerance, ToleranceEnvErrorKind, ToleranceError};
use geom_core::Tol;

// ---------------------------------------------------------------------
// Realistic M0–M2-style consumer code written against `geom_core::Real`
// alone, to test whether the trait surface suffices (the review's lib).
// ---------------------------------------------------------------------

/// A 2-D point, generic over the scalar — the shape PR 6 hands us.
#[derive(Debug, Clone, Copy)]
struct P2<T> {
    x: T,
    y: T,
}

/// Point on a circle: center + r*(cos θ, sin θ). The bread-and-butter M2
/// evaluator (arc edges, revolve profiles).
fn circle_point<T: Real>(cx: T, cy: T, r: T, theta: T) -> P2<T> {
    let (s, c) = theta.sin_cos();
    P2 {
        x: cx + r * c,
        y: cy + r * s,
    }
}

/// Quadratic discriminant b^2 - 4ac — line/circle intersection root count
/// feeds this to a sign predicate (PR 3). Note from_f64 for the constant.
fn quadratic_discriminant<T: Real>(a: T, b: T, c: T) -> T {
    b * b - T::from_f64(4.0) * a * c
}

/// Perpendicular distance from a point to a line through (p0, d), via
/// cross/norm, all inside Real.
fn point_line_distance<T: Real>(px: T, py: T, x0: T, y0: T, dx: T, dy: T) -> T {
    let cross = (px - x0) * dy - (py - y0) * dx;
    let len = (dx * dx + dy * dy).sqrt();
    cross.abs() / len
}

/// Transversality-margin-flavored quantity: angle between two direction
/// vectors via atan2 of (|cross|, dot) — robust near 0 and π. This is
/// exactly the D2 transversality margin shape.
fn angle_between<T: Real>(ux: T, uy: T, vx: T, vy: T) -> T {
    let cross = ux * vy - uy * vx;
    let dot = ux * vx + uy * vy;
    cross.abs().atan2(dot)
}

/// Axis-aligned bbox of a circular arc's endpoints — exercises min/max as
/// lattice ops the way M2 tessellation/bbox code will. (Deliberately no
/// containment test: that would need a *predicate*, which cannot be
/// written against Real — PR 3 owns it.)
fn arc_endpoint_bbox<T: Real>(cx: T, cy: T, r: T, t0: T, t1: T) -> (P2<T>, P2<T>) {
    let p0 = circle_point(cx, cy, r, t0);
    let p1 = circle_point(cx, cy, r, t1);
    let lo = P2 {
        x: p0.x.min(p1.x),
        y: p0.y.min(p1.y),
    };
    let hi = P2 {
        x: p0.x.max(p1.x),
        y: p0.y.max(p1.y),
    };
    (lo, hi)
}

/// Bernstein basis evaluation with powi — NURBS basis functions will look
/// like this (integer powers, from_f64 coefficients): (1 - t)^3.
fn cubic_bernstein0<T: Real>(t: T) -> T {
    (T::one() - t).powi(3)
}

/// Circle-circle intersection midpoint parameter x = (d² + r0² − r1²)/(2d):
/// exercises division and from_f64 constant density.
fn circle_circle_lens_x<T: Real>(d: T, r0: T, r1: T) -> T {
    (d * d + r0 * r0 - r1 * r1) / (T::from_f64(2.0) * d)
}

/// Drive the generic evaluators at T = f64 with known closed-form answers
/// (the review's `geom_eval` binary).
#[test]
fn generic_real_consumers_evaluate_closed_forms_at_f64() {
    // Circle point at 30 degrees on unit circle centered at (2, 3).
    let p = circle_point(2.0f64, 3.0, 1.0, std::f64::consts::FRAC_PI_6);
    assert!((p.x - (2.0 + 0.75f64.sqrt())).abs() < 1e-15);
    assert!((p.y - 3.5).abs() < 1e-15);

    // Discriminant of x^2 - 3x + 2 = 0 -> 9 - 8 = 1.
    assert_eq!(quadratic_discriminant(1.0f64, -3.0, 2.0), 1.0);

    // Distance from (0,1) to the x-axis (through origin, dir (1,0)) = 1.
    assert_eq!(point_line_distance(0.0f64, 1.0, 0.0, 0.0, 1.0, 0.0), 1.0);

    // Angle between x-axis and y-axis = pi/2; near-parallel margin small.
    let a = angle_between(1.0f64, 0.0, 0.0, 1.0);
    assert!((a - std::f64::consts::FRAC_PI_2).abs() < 1e-15);
    let a2 = angle_between(1.0f64, 0.0, 1.0, 1e-12);
    assert!(a2 > 0.0 && a2 < 1e-11);

    // Anti-parallel: atan2(|cross|, dot) must give ~pi, not ~0.
    let a3 = angle_between(1.0f64, 0.0, -1.0, 1e-12);
    assert!((a3 - std::f64::consts::PI).abs() < 1e-11);

    // Bbox of quarter arc endpoints.
    let (lo, hi) = arc_endpoint_bbox(0.0f64, 0.0, 2.0, 0.0, std::f64::consts::FRAC_PI_2);
    assert_eq!((lo.x, lo.y), (1.2246467991473532e-16, 0.0));
    assert_eq!((hi.x, hi.y), (2.0, 2.0));

    // Bernstein B0(0.25) = 0.75^3 = 0.421875 exactly.
    assert_eq!(cubic_bernstein0(0.25f64), 0.421875);

    // Circle-circle lens: d=4, r0=3, r1=3 -> x = 16/8 = 2 (symmetric).
    assert_eq!(circle_circle_lens_x(4.0f64, 3.0, 3.0), 2.0);

    // NaN propagation through a realistic pipeline: poisoned radius must
    // survive bbox min/max (Real::min propagates; f64::min would drop it).
    let (lo, _hi) = arc_endpoint_bbox(0.0f64, 0.0, f64::NAN, 0.0, 1.0);
    assert!(lo.x.is_nan(), "NaN must survive the bbox lattice ops");
}

// ---------------------------------------------------------------------
// The review's `forbidden/holes.rs`: gadgets that DO compile against the
// provided surface. They are documented consequences of Real's supertraits
// (Debug, 'static), pinned here so a change in either direction is loud.
// ---------------------------------------------------------------------

/// HOLE 0: the cheap comparison hole — just add the bound. Compiles today;
/// only fails when instantiated at a scalar lacking PartialOrd (PR 4+).
fn branch_with_extra_bound<T: Real + PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

/// HOLE 1: Debug-format-then-parse extracts an f64 from generic T.
fn extract_via_debug<T: Real>(a: T) -> f64 {
    format!("{a:?}").parse().unwrap_or(f64::NAN)
}

/// HOLE 2: branchable comparison gadget from Debug + min: min returns one
/// of its arguments, and Debug strings of equal bit patterns are equal.
fn le_via_min_debug<T: Real>(a: T, b: T) -> bool {
    format!("{:?}", a.min(b)) == format!("{a:?}")
}

/// HOLE 3: value-level sign via atan2 — atan2(0, x) is 0 for x > 0 and pi
/// for x < 0: sign information in a value channel outside any predicate.
fn sign_via_atan2<T: Real>(x: T) -> T {
    T::zero().atan2(x)
}

/// HOLE 4: T: 'static (implied by the Send + Sync + 'static supertrait)
/// makes every T: Real also T: Any, so safe downcasting extracts the
/// concrete f64 with zero unsafe.
fn extract_via_any<T: Real>(a: T) -> Option<f64> {
    (&a as &dyn std::any::Any).downcast_ref::<f64>().copied()
}

/// HOLE 5: branch on type, not value — TypeId lets generic code specialize
/// behavior per scalar type.
fn is_f64<T: Real>() -> bool {
    std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>()
}

#[test]
fn documented_holes_in_the_real_surface() {
    assert_eq!(branch_with_extra_bound(1.0f64, 2.0), 1.0);
    assert_eq!(extract_via_debug(2.5f64), 2.5);
    assert!(le_via_min_debug(1.0f64, 2.0));
    assert!(!le_via_min_debug(3.0f64, 2.0));
    assert_eq!(sign_via_atan2(-7.0f64), std::f64::consts::PI);
    assert_eq!(sign_via_atan2(7.0f64), 0.0);
    assert_eq!(extract_via_any(2.5f64), Some(2.5));
    assert!(is_f64::<f64>());
}

// ---------------------------------------------------------------------
// powi edge cases: the documented "n < 0 via 1/x^|n|" reciprocal rule and
// its accuracy consequences, from outside the crate.
// ---------------------------------------------------------------------

#[test]
fn powi_edge_cases_document_the_reciprocal_rule() {
    // i32::MIN does not overflow (unsigned_abs) and the reciprocal rule
    // applies: 2^|MIN| overflows to inf, so the reciprocal is 0.
    assert_eq!(Real::powi(2.0f64, i32::MIN), 0.0);
    assert_eq!(Real::powi(0.5f64, i32::MIN), f64::INFINITY);
    assert_eq!(Real::powi(1.0f64, i32::MIN), 1.0);
    assert_eq!(Real::powi(-1.0f64, i32::MIN), 1.0); // even exponent -> 1
    assert_eq!(Real::powi(-1.0f64, i32::MAX), -1.0); // odd -> -1
    assert_eq!(Real::powi(2.0f64, 1075), f64::INFINITY); // overflow
    assert_eq!(Real::powi(0.0f64, -2), f64::INFINITY); // 1/0^2
    assert_eq!(Real::powi(-0.0f64, -1), f64::NEG_INFINITY); // sign!
    // Poison guard at n = 0: NaN^0 stays NaN (the generic shortcut is
    // guarded at f64); a healthy non-finite base still collapses to one.
    assert!(Real::powi(f64::NAN, 0).is_nan());
    assert_eq!(Real::powi(f64::INFINITY, 0), 1.0);
    // Subnormal base survives a single multiply, then underflows.
    assert_eq!(Real::powi(5e-324f64, 1), 5e-324);
    assert_eq!(Real::powi(5e-324f64, 2), 0.0);

    // Reciprocal-form accuracy loss (documented consequence): 2^-1074 is
    // the min subnormal (5e-324), but powi computes 1/2^1074 = 1/inf = 0.
    assert_eq!(Real::powi(2.0f64, -1074), 0.0);
    // Near the boundary the reciprocal lands subnormal instead of the
    // correctly-rounded normal: still finite and positive, just lossy.
    let near = Real::powi(10.0f64, -308);
    assert!(near > 0.0 && near < f64::MIN_POSITIVE);
}

// ---------------------------------------------------------------------
// The Tolerance env/init state machine, cross-process (the review's
// tol_driver + tol_probe + tol_threads). The global commits exactly once
// per process, so each probe re-executes THIS test binary with `--exact`
// and a fresh, fully-controlled environment; the child takes the probe
// branch below and asserts, the parent checks the exit status.
// ---------------------------------------------------------------------

const CHILD_MODE: &str = "REVIEW_M0_PR2_TOL_CHILD_MODE";
const CHILD_RAW: &str = "REVIEW_M0_PR2_TOL_CHILD_RAW";
const CHILD_EXPECT_EPS: &str = "REVIEW_M0_PR2_TOL_CHILD_EXPECT_EPS";
const CHILD_EXPECT_ERR: &str = "REVIEW_M0_PR2_TOL_CHILD_EXPECT_ERR";
const TEST_NAME: &str = "tolerance_state_machine_cross_process";

fn expected_eps() -> f64 {
    std::env::var(CHILD_EXPECT_EPS)
        .expect("parent sets the expected eps")
        .parse()
        .expect("parent sends a parseable expected eps")
}

/// The child side: probes the global Tolerance under whatever env the
/// parent set. Panics (failing the child process) on any violation.
fn tolerance_child(mode: &str) {
    match mode {
        // Plain first-touch via get(): committed value + recorded errors.
        "get" => {
            let t = Tol::witness().get();
            assert_eq!(t.eps, expected_eps(), "committed eps");
            let errors = Tolerance::env_init_errors();
            match std::env::var(CHILD_EXPECT_ERR).as_deref() {
                Ok("none") => assert!(errors.is_empty(), "unexpected env errors: {errors:?}"),
                Ok(kind) => {
                    assert_eq!(errors.len(), 1, "exactly one recorded env error");
                    assert_eq!(errors[0].var, ENV_EPS);
                    assert_eq!(
                        errors[0].raw,
                        std::env::var(CHILD_RAW).expect("parent sets the raw value")
                    );
                    match (&errors[0].kind, kind) {
                        (ToleranceEnvErrorKind::Unparsable, "unparsable") => {}
                        (ToleranceEnvErrorKind::Invalid { value }, "invalid") => {
                            // The rejected value parsed; it round-trips
                            // through the same parse the library did.
                            let reparsed: f64 = errors[0].raw.parse().unwrap();
                            assert_eq!(value.to_bits(), reparsed.to_bits());
                        }
                        (got, want) => panic!("env error kind {got:?}, expected {want}"),
                    }
                }
                other => panic!("parent must set {CHILD_EXPECT_ERR}: {other:?}"),
            }
        }
        // Explicit init first, then get; env must be ignored entirely.
        "init-then-get" => {
            let mine = Tolerance::with_eps(7e-7);
            assert_eq!(Tolerance::init(mine), Ok(()));
            assert_eq!(Tol::witness().get(), mine, "env (1e-6) must be ignored");
            assert!(
                Tolerance::env_init_errors().is_empty(),
                "init never consults the environment, records nothing"
            );
            // Second init: typed AlreadyInitialized with both values.
            let attempted = Tolerance::with_eps(1e-3);
            assert_eq!(
                Tolerance::init(attempted),
                Err(ToleranceError::AlreadyInitialized {
                    current: mine,
                    attempted,
                })
            );
        }
        // get() first (commits from env), then a late init must fail typed
        // and leave the committed value untouched.
        "get-then-init" => {
            let committed = Tol::witness().get();
            assert_eq!(committed.eps, 3e-8, "env value wins on first get");
            let attempted = Tolerance::with_eps(5e-5);
            assert_eq!(
                Tolerance::init(attempted),
                Err(ToleranceError::AlreadyInitialized {
                    current: committed,
                    attempted,
                })
            );
            assert_eq!(Tol::witness().get().eps, 3e-8, "unchanged after late init");
        }
        // env_init_errors() before any get(): must force init and answer.
        "error-first" => {
            let errors = Tolerance::env_init_errors();
            assert_eq!(errors.len(), 1, "malformed env recorded before any get");
            assert_eq!(errors[0].kind, ToleranceEnvErrorKind::Unparsable);
            assert_eq!(Tol::witness().get().eps, DEFAULT_EPS);
        }
        // Invalid init value: typed error, global left uncommitted, so a
        // following get() still resolves from env.
        "bad-init-then-get" => {
            assert_eq!(
                Tolerance::init(Tolerance::with_eps(-1.0)),
                Err(ToleranceError::InvalidValue { value: -1.0 })
            );
            assert_eq!(
                Tol::witness().get().eps,
                9e-9,
                "rejected init must leave the global uncommitted for env"
            );
        }
        // 8 threads race the first touch; all observed values must be
        // identical (no tearing, deterministic commit).
        "threads" => {
            let barrier = std::sync::Barrier::new(8);
            let results: Vec<(u64, bool)> = std::thread::scope(|s| {
                let handles: Vec<_> = (0..8)
                    .map(|_| {
                        s.spawn(|| {
                            barrier.wait();
                            let t = Tol::witness().get();
                            let e = !Tolerance::env_init_errors().is_empty();
                            (t.eps.to_bits(), e)
                        })
                    })
                    .collect();
                handles.into_iter().map(|h| h.join().unwrap()).collect()
            });
            assert!(
                results.windows(2).all(|w| w[0] == w[1]),
                "TEARING: threads observed different tolerances: {results:?}"
            );
            assert_eq!(results[0].0, 4e-8f64.to_bits());
            assert!(!results[0].1);
        }
        other => panic!("unknown child mode {other}"),
    }
}

/// The parent side: spawn one fresh child process per scenario.
fn probe(mode: &str, env: &[(&str, &str)]) {
    let exe = std::env::current_exe().expect("test binary path");
    let mut cmd = std::process::Command::new(exe);
    // Self re-exec: name the probe by MODULE PATH, not by bare fn name.
    // `tests/all.rs` aggregates every suite into one binary, so libtest sees
    // this probe as `<this_module>::tolerance_state_machine_cross_process`. Stripping the leading crate name off
    // `module_path!()` yields the right filter in the aggregated layout AND in
    // a standalone one (where `module_path!()` has no `::` at all).
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::{TEST_NAME}"),
        None => TEST_NAME.to_string(),
    };
    cmd.args([probe.as_str(), "--exact"])
        .env_remove(ENV_EPS) // full control, even under the CI eps matrix
        .env(CHILD_MODE, mode);
    for (k, v) in env {
        cmd.env(k, v);
    }
    let out = cmd.output().expect("spawn tolerance probe child");
    assert!(
        out.status.success(),
        "probe mode={mode} env={env:?} failed:\n{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
}

/// A `get`-mode probe: env raw value -> expected committed eps + error kind.
fn probe_get(raw: Option<&str>, expect_eps: &str, expect_err: &str) {
    let mut env = vec![
        (CHILD_EXPECT_EPS, expect_eps),
        (CHILD_EXPECT_ERR, expect_err),
    ];
    if let Some(raw) = raw {
        env.push((ENV_EPS, raw));
        env.push((CHILD_RAW, raw));
    }
    probe("get", &env);
}

#[test]
fn tolerance_state_machine_cross_process() {
    if let Ok(mode) = std::env::var(CHILD_MODE) {
        tolerance_child(&mode);
        return;
    }

    const DEFAULT: &str = "1e-9"; // parses to exactly DEFAULT_EPS

    // no env: compiled defaults.
    probe_get(None, DEFAULT, "none");
    // well-formed env value wins.
    probe_get(Some("1e-6"), "1e-6", "none");
    // malformed values: default + recorded Unparsable.
    probe_get(Some("bogus"), DEFAULT, "unparsable");
    probe_get(Some(""), DEFAULT, "unparsable");
    // no trimming, by design: a value needing cleanup is a config anomaly.
    probe_get(Some("1e-9 "), DEFAULT, "unparsable");
    // hex floats and comma decimals do not parse as Rust f64 literals.
    probe_get(Some("0x1p-30"), DEFAULT, "unparsable");
    probe_get(Some("1,5e-9"), DEFAULT, "unparsable");
    // parseable but invalid values: default + recorded Invalid.
    probe_get(Some("-1e-9"), DEFAULT, "invalid");
    probe_get(Some("inf"), DEFAULT, "invalid");
    probe_get(Some("1e-999"), DEFAULT, "invalid"); // underflows to 0
    probe_get(Some("1e999"), DEFAULT, "invalid"); // overflows to inf

    // Ordering semantics.
    probe("init-then-get", &[(ENV_EPS, "1e-6")]);
    probe("get-then-init", &[(ENV_EPS, "3e-8")]);
    probe("error-first", &[(ENV_EPS, "oops")]);
    probe("bad-init-then-get", &[(ENV_EPS, "9e-9")]);

    // First-touch race.
    probe("threads", &[(ENV_EPS, "4e-8")]);
}
