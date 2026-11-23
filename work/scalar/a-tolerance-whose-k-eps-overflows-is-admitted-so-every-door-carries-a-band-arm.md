---
id: a-tolerance-whose-k-eps-overflows-is-admitted-so-every-door-carries-a-band-arm
kind: issue
title: Tolerance admits an ε whose K·ε overflows, so Band::linear is partial and a dozen error enums carry a Band arm for it
status: open
opened: 2026-11-23
design: true
priority: P3
cost: M
---


## What

`Tolerance`'s admission rule (`geom_core::tolerance::admissible`,
`crates/geom-core/src/tolerance.rs`) is: ε finite, K finite and K > 1,
and ε ≥ `eps_min(K)`. It sets no ceiling on ε·K. So a run can commit an ε
within a factor K of `f64::MAX`, and then `Band::linear(tol)`
(`crates/geom-core/src/predicate.rs`) refuses with
`BandError::InvalidValue` on `escalate`. `Band::linear`'s own `# Errors`
names this as the one arm a run's tolerance reaches.

Evidence: review probe for PR 7477 (`origin/review/7477-probes`,
`crates/topo/tests/probe_7477_band.rs`). Run under
`CAD_TOLERANCE_EPS=1e308 CAD_AMBIGUITY_K=10`, `Body::mev_line` on a
lone vertex refuses `EulerOpError::Band(InvalidValue { field: Escalate,
value: inf })`. The probe needs the process-global tolerance committed at
that ε, so it cannot run as an ordinary row in a shared test binary.

Because the band is partial, roughly twenty operation error enums carry
a `Band(BandError)` arm for it, among them `EulerOpError` (PR 7477),
`JoinError`, `BlendError`, `SplitReduceError`, `ExtrudeError`,
`LoftError`, `ProfileError` and `PcurveMintError` (`git grep -n
'Band(BandError),'`). Every door also opens with a `Band::linear(tol)?`.

## The question (design)

Should `Tolerance` refuse, at construction, an ε whose K·ε is not
finite? A new `ToleranceError` arm would do it, beside `BelowFloor`.

- If so, `Band::linear` becomes total. Its signature could drop the
  `Result`, or keep it with an `expect` that names the admission rule.
  The `Band` arms that exist only for the run's band would then go,
  which is a public-shape change across those enums.
- `Band::linear_at` and `Band::angular_at` keep their caller-argument
  arms whatever is decided: their inputs are not the run's.
- The cost is one more admission rule. The physical session box
  (D4 ¶4, ε ≈ 1e-9 at 1 km) is nowhere near the ceiling, so no
  realistic run is refused.
