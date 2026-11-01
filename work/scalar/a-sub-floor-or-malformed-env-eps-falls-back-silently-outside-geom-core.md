---
id: a-sub-floor-or-malformed-env-eps-falls-back-silently-outside-geom-core
kind: issue
title: A malformed or sub-floor CAD_TOLERANCE_EPS falls back to the default silently in every test binary but geom-core's, so an eps row can run at 1e-9 and report green
status: open
opened: 2026-10-13
---

(Filed by the ENCL lane on PR 4925, the ε floor, from that PR's review.)

## What

`crates/geom-core/src/tolerance.rs`, `resolve_from_env`: a `CAD_TOLERANCE_EPS` that does not parse, is not finite and positive, or (since PR 4925) lies below `EPS_MIN` ≈ 2.2e-13 m is recorded in `Tolerance::env_init_errors` and printed by `ToleranceReport`. The run then decides at the compiled default, 1e-9. The module's "Initialization protocol" docs say loudness is restored structurally: a regular `#[test]` in geom-core asserts the recorded slice is empty.

That test exists only in geom-core's binaries. Any other test binary run with a sub-floor or typo'd ε runs its whole suite at 1e-9 and reports green. Examples:
- `CAD_TOLERANCE_EPS=1e-14 cargo nextest run -p sweep`. That is the run `sweep/tests/offd_r1_probes.rs` used to document; PR 4925 retired the documentation.
- A nightly row whose value is mistyped.

`Tol::witness()` is infallible by design (D9), so the guard has to be per binary or per harness. Every eps-sensitive suite's binary could carry the empty-slice assertion. A nextest setup script or the `ci.yml`/`nightly.yml` eps steps could fail on a reported fallback.

## Where

- `crates/geom-core/src/tolerance.rs`: `resolve_from_env`, `Tolerance::env_init_errors`, and the "Initialization protocol" docs that make the claim.
- `.github/workflows/ci.yml` (the "test (eps 1e-6, 1e-12 …)" step) and `.github/workflows/nightly.yml` (the eps matrix): both set the variable and read only nextest's verdict.
