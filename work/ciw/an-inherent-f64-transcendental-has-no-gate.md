---
id: an-inherent-f64-transcendental-has-no-gate
kind: issue
title: D9's pure-Rust libm is enforced for atan2 only; disallow the other inherent f64 transcendentals through clippy.toml, not a new gate
status: open
opened: 2026-10-16
priority: P3
cost: M
refs: [sweep-places-reads-the-platform-libm]
---

## Finding

D9 (`docs/DESIGN.md`; `geom_core::Real for f64`'s doc in
`crates/geom-core/src/real.rs`) routes every transcendental through the
pure-Rust `libm` crate, because the platform libm differs across
platforms in the last ulp. An inherent call on a bare `f64`
(`x.sin()`, `t.sin_cos()`) compiles silently and goes to the platform
libm. `sweep::sweep_places` was one
(`work/inlane/sweep-places-reads-the-platform-libm.md`, fixed).

`.atan2(` in production code is already held by
`scripts/gates/atan2-sites.sh` (added 2026-11-10), but only for the
angle-door discipline. Nothing holds `sin`, `cos`, `sin_cos`, `tan`,
`asin`, `acos`, `atan`, `exp`, `ln`, `log*`, `powf`, `cbrt` or `hypot`.
A grep cannot tell an `f64` receiver from a generic `T: Real` one: about
2300 such calls sit in `crates/*/src`, most of them on `T`.

## Shape

Add a `disallowed-methods` list for the inherent `f64` methods
(`f64::sin`, `f64::cos`, …) to the root `clippy.toml`. Clippy resolves
the receiver's type, so a `T: Real` call is not flagged. The per-PR
clippy (`ci.yml:316`/`:320`, `-D warnings`) already runs, so this adds
no step and no measurable time. Test modules and the `test-support`
fixture builders that need the platform functions (for example
`crates/sweep/src/test_support.rs`, about forty calls) carry
`#[allow(clippy::disallowed_methods)]` with a reason. Do not add a
script under `scripts/gates/`, because the lint job's gates step is
already the slowest step on every PR.

What would close it: `clippy.toml` disallows the inherent f64
transcendentals, today's production sites are routed through `Real`,
and each test or fixture allow states its reason.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): Option 1, a grep gate in `scripts/gates/`, adds to the lint job's gates step on every PR. The only incident is `sweep_places` (filed 2026-10-16, fixed), and CI runs on one platform, so the platform-libm drift D9 rules out can never turn CI red. Option 2, `clippy.toml` `disallowed-methods`, resolves receiver types and rides the existing `-D warnings` clippy (`ci.yml:316`/`:320`), so it costs nothing per PR. `.atan2(` is already held by `scripts/gates/atan2-sites.sh` (added 2026-11-10). A grep would face about 2300 `.sin()/.cos()/.exp()/...` calls in `crates/*/src`, mostly on `T: Real`.
