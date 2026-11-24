---
id: census-rows-whose-bracket-read-re-enters-the-scalar-are-filed-as-payload-or-selection
kind: issue
title: Bounds-census rows whose bracket read comes back as a T through from_computed are filed as Payload or Selection, and none says why a dual cannot reach them
status: open
opened: 2026-11-19
priority: P3
cost: M
refs: [dl5-has-no-arm-for-a-selection-whose-result-is-the-payload]
---


Filed by the SCALAR orchestrator, 2026-11-19, from two designers'
off-question findings on the DL5 fork
(`dl5-has-no-arm-for-a-selection-whose-result-is-the-payload`). Read
from the roster's own text, not executed: **verify before fixing.**

## What

A bracket read (`Bounds::lo`/`hi`) at a dual returns the value
channel's `f64` and drops the tangent. That costs nothing while the
`f64` leaves (a payload) or is only compared (a branch). It is the
defect of issue 874's class when the `f64` becomes a `T` again
(`Real::from_computed`) and the true quantity moves with the inputs.
`crates/geom-core/tests/bounds_census.rs` holds rows of that third
kind under variants that describe the first two:

- `geom_brep::curve_pair::polish`'s `pair`, the module doc's gap 5: it
  moves the iterate by the bracket midpoint of the step and re-enters
  through `from_computed`, disposed as "that freeze's `Selection`". If
  `Dual<Interval>` reaches it, a derivative enclosure through it drops
  `dt/dp`, and DL1 prunes on such enclosures.
- The `Payload` rows `carrier_span` and `knot_lines` (geom-brep), whose
  own text says their values are "lifted back through `from_computed`".
- `geom::projection_policy`'s `mid` and the two projection macro
  bodies: the counterexample the locally-constant clause exists for,
  filed under the variant whose doc says `mid` sits outside it. The
  doors that freeze a foot are bounded `CertifiedBounds`; the seed and
  box doors keep the sole `Bounds` bound.

Beside them, `Real::from_computed` has 121 call sites in 44 files and
no per-site disposition; only the `Dual` impl's doc states the hazard.

## What closing it takes

For each row above: who calls it at a dual, and whether what re-enters
can move with a seed. A reachable one is a defect row of its own. The
roster's labels follow whatever the DL5 fork decides; this row is the
soundness question under them, which no labelling answers.

Also read from the roster and not checked: `nonpositive_size_gate`
(`crates/sweep/src/blend/build.rs`) and `analytic_datum_verdicts`
(`crates/topo/src/validate.rs`) admit or refuse on a bracket outside
`decide`, on a sole bound `scripts/gates/bounds-allowlist.sh` cannot
see; the at-rest ledger calls the second "a bracket read that DECIDES
NOTHING".
