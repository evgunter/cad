---
id: the-mate-solve-reads-the-platform-atan2
kind: issue
title: The mate solve's two angle reads take the platform's atan2, not the kernel's libm door (D9)
status: open
opened: 2026-10-03
priority: P3
cost: E
---

Found by the MSOLVE-14 lane while making the solve generic.

## What

The coset fold solves two angles with `f64::atan2`, the platform's
libm: `candidate_rotation`'s two-axis reachability angle and
`clocking_about`'s free clocking (`crates/editor-core/src/mate/coset.rs`).
Every other transcendental in the kernel goes through `Real`, whose
`f64` `atan2` is the pure-Rust `libm` crate's, for D9's reason: a
system libm differs across platforms in the last ulp.

The two are not the same function. Measured on this box (glibc) over
twenty million random argument pairs in `[-1, 1]²` with a third of
each argument scaled by `1e-8`: 3,295,403 differ, by an ulp. So a solved
pose that reads either site is platform-dependent at `f64`.

## Why it was not changed in MSOLVE-14

The unit's fence is bit identity at `f64` with main. Moving to
`Real::atan2` would move a bit wherever the two disagree, so the unit
routes the angle through `SolveScalar::solve_atan2`. At `f64` that is
the platform's, as before. A dual's value channel reads the same one,
so the `Dual64` value channel stays the `f64` lane's bits, and its
tangent is the dual's own.

## What would close it

Make `f64`'s `solve_atan2` the `Real` one (libm). Then delete the
trait method, since every lane's would be `Real::atan2`. Re-baseline the
mate corpus fence
(`msolve14_run_scalar::a3_the_f64_solve_is_mains_bit_for_bit_on_the_mate_corpus`)
and any pose that moved, and say what moved.

