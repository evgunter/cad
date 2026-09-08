---
id: k-report-baseline-fold-cert1-roster
kind: issue
title: k-report: fold the CERT-1 roster changes into the next baseline re-derivation (props_meridian_pole; sphere props_rim_level margins)
status: closed
branch: meter/k-report-cert1-fold
opened: 2026-08-29
closed: 2026-09-08
github: 1251
refs: [1220]
track: K
---

## From GitHub issue 1251

Opened 2026-08-29; 0 comments.

**Scheduling register for PR 1220's K-telemetry consequences** (S-CERT CERT-1), so the roster change has a place that executes instead of an "if the census flags it" hedge.

What moved, for the next K-REPORT runbook pass:

- **New recorded name `props_meridian_pole`** (`props/curved.rs`, `sphere_meridian_span_levels`): two samples per sphere meridian arc; margin = signed chord from the pole's span-relative direction to the nearer span end, × R. Its indeterminate outcome **folds rather than refusing** (the decide still records; PR 1220's body carries the continuity argument), so its in-band population is expected and benign — the baseline should not read in-band samples on this name as a landing.
- **Sphere `props_rim_level` / `props_rim_level_group` margins re-shaped**: the axial `|Δ sin v|·R` became the direction chord `2·sin(Δv/2)·R` (larger wherever rims are distinct), and rims sitting at their own extreme now record a rounding-scale second-component residual instead of bitwise 0 — a new near-zero cluster in those populations.
- `rim_dim_scale_twins.rs`'s sphere twin now pins the chord and the two-population shape (nothing in the ambiguity band).

Per the K-REPORT runbook this is the re-derive-the-baseline case, not a geometry change; the sampled k-lint axis had not drawn a fresh row between PR 1220's merge base and its head, so the first draw lands whenever the schedule next picks it up.

Refs: PR 1220, `docs/K-REPORT.md`, `docs/predicate-dimension-audit.md` (the `props_meridian_pole` row and retired note N7).

## Home

`work/cert/` — the roster that moved is `props/curved.rs`'s, inside S-CERT's `crates/geom-brep/src/props/*` territory, and the change is CERT-1's own consequence.

## Re-homed (2026-09-06)

Moved from `work/cert/` to `work/code-quality/` on S-CERT's exit walk PR
(#1924, its handoffs ledger; merged by Ev 2026-09-06 = ratified), before
`work/cert/` was deleted at sweep 7 of `docs/DOC-LEDGER.md`. Id, body
and header are unchanged; the directory is the claim (`work/README.md`).
The `## Home` section above naming `work/cert/` is superseded by this
line and is kept as the record of why the file was filed there.

## Claimed by METER (2026-09-06)

Moved from `work/code-quality/` to `work/meter/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, `track:` letter and body unchanged. Track K; `docs/K-REPORT.md`'s runbook is METER's territory; the `props/curved.rs` names are read only.

## Closed (2026-09-08)

**Re-derived, and it was feasible in-lane.** The `k-lint (gate)` job
carries no `env:` block, so the configuration those baselines were
produced under is cargo's plain dev/test default — the one lane
deliberately excluded from the archive jobs' opt-level bump — and
`rust-toolchain.toml` pins the compiler for both.
`scripts/k_probe_sweep.sh` reproduces it with no substitution, in 618 s
wall at `ada6bba9`. Nothing here is a local stand-in taken under a different
configuration.

**The measurement, in `docs/K-REPORT.md`'s M11 addendum
(2026-09-08).** Three ε rows, 2 068 116 / 140 / 164 samples, **0
flags** at every rule.

- `props_meridian_pole`: 7 940 samples per row, identical split at all
  three — 7 552 `zero` (largest |m| 8.16278e-17 m, 862 bitwise 0) and
  388 `negative` (smallest |m| 5.47723e-2 m, 1 370× the baseline
  floor). **Zero `indeterminate`.** The in-band population the item
  warned would be benign is EMPTY in the gated corpus — measured, not
  read off a green.
- The reading the item exists for is recorded anyway, because a future
  landing is what it is for: the fold arm means an in-band sample on
  this name is **not** a landing, and the M7 addendum's dimension
  caveat now has a sibling — check the deciding site's DISPOSITION too.
  Rule 1 keeps gating the name; a name-shaped exemption would be a
  threshold adjusted to restore a number.
- Rim margins: `props_rim_level` 790 (all `zero`; 492 bitwise 0, 298
  rounding residuals, largest 1.24127e-15 m), `props_rim_level_group`
  306 (262 `positive`, smallest 1.90693e-2 m; 44 `zero`, all bitwise
  0). The near-zero cluster is the re-shaping's expected consequence
  and lands only on the sphere- and torus-bearing shapes.

**No new era cut, and that is the measured answer rather than a
default.** Both calibration witnesses are pointwise identical to M7's
at all three rows (`volume_backstop` 4.79652e-5;
`props_quad_converged` 164.674·ε at 1e-9), and the zero side's ceiling
is identical too (`pm_census_ee_span`, 5.32907e-15).
`docs/k-report-data/README.md`'s rule 1 cuts a new file when the
DISTRIBUTION moves; a roster growth is explicitly not that. The README
now says so with the numbers, and the addendum names what would cut
one.

**Residue, with its own file** (`work/README.md`): rule 1's prose in
`tools/k-lint/src/lib.rs` asserts that a recorded `indeterminate` means
the kernel refused typed, which the folding site falsifies —
`work/meter/k-lint-rule-1-prose-assumes-every-in-band-site-refuses.md`.
Not fixed here: `tools/k-lint/*` was held by unit 3's fix pass.
