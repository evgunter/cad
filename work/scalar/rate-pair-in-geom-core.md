---
id: rate-pair-in-geom-core
kind: unit
title: SupSpeed and InfSpeed beside Margin: metered takes the inf, a sup door for overshoot metering, the three blurred sites typed
status: closed
opened: 2026-09-15
branch: scalar/rate-pair
pr: 2657
closed: 2026-09-15
---


## What

`D283`'s ruling, route A (PR 2457), first unit. Two types in
`crates/geom-core/src/predicate.rs` beside `Margin`: `SupSpeed` and
`InfSpeed`, meters per parameter unit, one operation each way
(`span · s` to meters, `m / s` to parameter units — bit-identical to the
bare arithmetic, D9), a surface being a pair. `Margin::metered` takes
`InfSpeed` (its doc's promise made a type) and gains the sup-side
sibling for overshoot and escape metering; the three sites that pass a
sup through `metered` today (`pcurve_cache::trim_containment`, the
`pcurve_iso_*` slack meters, `topo::pcurves::pcurve_loop_continuity`'s
v-channel — PROPS' row
`metered-margin-doc-promises-an-inf-bound-three-sites-pass-a-sup` and
TRIM's `loop-continuity-meters-u-through-levered-and-v-through-metered`)
move to it; `PatchRegularity`'s speeds and `plane_nurbs_ssi`'s local
speed become `SupSpeed`. The type is a dimension-and-direction tag, not
a positivity witness: the 0/∞ guards stay at their sites. Out of scope
by the ruling: second-order and param→param rates (mesh sizing,
chords), pointwise jet speeds (the march, Newton acceptance),
`levered_inv`'s curvature uses, `gap_is_noise`'s zero lever. Evidence:
`work/scalar/log.md`, "The rate census". PROPS' and TRIM's ground;
announce. Full v6 dual.

## Digest receipt: the D9 pin for "nothing's bits move"

The receipt is a DIFFERENTIAL between two trees, so the recipe has to
be executable by a second party on either of them. This one has no
free parameters: no cargo preamble, no absolute path, no line cut, no
rewrite rule.

```sh
# once per tree, from that tree's demos/tour
CARGO_TARGET_DIR=<target> cargo build --release
# then, from an empty directory
rm -rf tour-out && <target>/release/demo-tour tour-out > narration.log 2>&1
(cd tour-out && find . -type f | LC_ALL=C sort | xargs sha256sum) > listing.txt
sha256sum listing.txt narration.log
```

The binary is run DIRECTLY rather than through `cargo run`, so its
output is the kernel's words and nothing else; the outdir is the
literal relative name `tour-out`, so the path the narration prints is
the same string on both trees; and both streams are digested whole.

Taken at `origin/main` (`4f71edaea`) and at this branch's head, with
the same target directory rebuilt in place between them:

- **1766 emitted files** (`*.pncad`, `*.stl`, `*.step`, `uv/*.svg`,
  `uv.json`, `scenes.json`); digest of the sorted per-file digest
  listing:
  `87be4dd9df4cc3af9bd44593a6b981608c8e73721c746413a00322ff61e4a892`
- the tour's own **narration** (the census, genus, validation tiers
  and exact-vs-meshed mass properties it prints per stop), 729 lines:
  `e930abf542c371677b2c0d87b02c2bf84eb15fbaf62ded6389f48c899ef14d49`

Both values are the same at base and at head, and `diff` on both files
is empty. The listing digest is also the one the pre-change receipt
recorded on this branch (`afd28b980`), unchanged across the 128-file
main merge in between.

Neither of the two narration numbers that receipt carried is
reproducible and neither is kept: the first digested cargo's preamble
and an absolute outdir, and the correction that replaced it specified
neither the constant it rewrote the outdir to nor the trailing lines
it dropped. Two independent reviewers reproduced the listing digest
exactly at both trees and could reproduce neither narration number,
which is what sent this recipe back. The listing itself is a one-shot
comparison artefact and is not committed
(`memories/test-suite-cost.md`): the two aggregate hashes are what a
second run has to reproduce.

## Closed (2026-09-15) — PR 2657

`SupSpeed<T>`/`InfSpeed<T>` in `crates/geom-core/src/predicate.rs`
beside `Margin`: transparent `Copy` newtypes, no `PartialEq`/`PartialOrd`,
`new`/`get`, `to_meters` (both) and `to_param` (`SupSpeed` only — the
inf quotient over-states a reach and had no caller). `Margin::metered`
takes `InfSpeed`; `Margin::metered_sup` takes `SupSpeed`; the direction
rule has one home (the module doc). Producers typed at the mint
(`speed_lower_bound`, `param_rate`, `chart_stretch_sup` — now a `Result`
refusing the cone, with `chart_stretch_sup_v` for its exact `v` arm —
`nurbs_stretch_bounds`, `chart_arms_at`, `curve_rate_bound`, `v_meter`,
`split`/`classify`/`chord_join`, `PatchRegularity` speeds,
`plane_nurbs_ssi`'s local speed). Consumers: the three blurred sup
sites, `certify.rs` check 2's circle/ellipse arms, the loop-continuity
u channel on plane/spline charts through `ChartArm`, `chart_bound`'s
arm. Bit identity: `to_bits` differential rows, the `Interval` row, the
tour listing digest (1766 files, `87be4dd9…`) and a reproducible
narration digest identical at `origin/main` and head; k-lint counts
unmoved. Reviews: dual, both APPROVE WITH FIXES; R1's M1 (the cone arm's
false `SupSpeed` on a `pub` door — a tally candidate, demonstrated: the
frozen head answered `1` where the true sup at `v = 4` is `2`) and M2
(the pair's stated boundary false for plane/spline charts); twelve
fix-pass items taken. Rows: PROPS' and TRIM's closed; TRIM
`angular-arms-are-an-untagged-lever-beside-a-typed-rate`,
`placeholder-chart-sup-arms-are-not-a-bound` filed;
`certified-arms-are-an-untagged-inf-rate-beside-a-typed-pair` widened.
