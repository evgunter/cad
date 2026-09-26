# ENCL — the plan

certified enclosures: what a certificate claims, and what it is worth

## The slate

| pri | item | cost | title |
|---|---|---|---|
| P0 | `tangent-parallel-certifier-passes-a-transverse-arc` | H | certify: TangentParallel admits a 90-degree crossing described as a tangent intersection |
| P0 | `offset-fit-at-tight-eps-refuses-every-curved-nurbs-chart` | H | at eps 1e-12 the offset fit refuses every curved NURBS chart, and the mint's cost at the default eps is unmeasured on a body |
| P0 | `encl-refusal-prose-outgrows-the-viewer` | D | the offset meter and offset fit refusals are too long for the viewer |
| P1 | `approx-surface-tolerance-is-now-always-the-runs-eps` | D | SurfaceSpec.tolerance is always the run's eps; map_approx reads it as the surface's own |
| P1 | `patch-bound-offset-fit-recentring-origins` | D | patch_bound and offset_fit recentre the same nets against different origins |
| P1 | `a-third-spelling-of-cut-every-span-into-splits-pieces` | D | a third spelling of "cut every span into splits pieces"; the concept's home is below both crates |
| P2 | `one-pass-refinement-would-cut-the-rational-bounds-widening-tail` | H | one-pass (Oslo) refinement would cut the rational bound's widening tail |
| P3 | `H11` | E | point the re-derived NaN-end postcondition sites at CertifiedEnclosure's home |
| P3 | `budget-refusal-drops-the-enclosure-the-caller-needs` | E | the teapot probe is the one consumer left without a bracket |
| P3 | `offset-fit-stall-face-has-no-fixture` | E | RefinementStalled has no fixture at any door |
| P4 | `offset-fit-door-bound-is-not-monotone-in-the-cell-bound` | E | the door bound is not monotone in the cell bound; the note belongs at `measure` |
| P4 | `offset-fit-reuses-derivedknots-for-a-degree-elevation-failure` | E | a failed degree elevation reports as DerivedKnots |
| P4 | `refine-chain-hands-back-a-pair-its-only-caller-re-borrows` | E | rides with the third-spelling row (both edit `patch_bound`'s refinement helpers) |

## Order

**Wave 1, concurrently** — the three P0 rows, each on its own lane:

- `tangent-parallel-certifier-passes-a-transverse-arc`. The row's
  explanation does not survive a reading of the tree: the arm meters
  `Margin::levered_inv(sin θ, |κ_rel|)` = `sin θ · r` against ε, the
  D4 ¶1 threshold, which refuses `sin θ = 1` at any `r` above ε. So the
  unit opens by reproducing the mutant and measuring what the arm
  actually reads at the arc's samples; the fix follows the cause. If the
  cause is the arm computing something other than D4 ¶1 says, the fix
  lands here; if D4 ¶1 itself admits the crossing, the margin's
  definition is a ratified decision and goes to Ev as an `[ev]` PR.
- `offset-fit-at-tight-eps-refuses-every-curved-nurbs-chart`. The
  identical `3.27e-10` at targets 1e-12 and 1e-15 reads as a FLOOR, not
  a budget: measure where the certified bound's floor comes from before
  touching any budget number, and take part 2's body-level cost
  measurement.
- `encl-refusal-prose-outgrows-the-viewer`, with
  `offset-fit-reuses-derivedknots-for-a-degree-elevation-failure`
  riding (both reword `OffsetFitError`'s arms).

**Wave 2**: the small rows as one lane (`H11`,
`offset-fit-stall-face-has-no-fixture`,
`offset-fit-door-bound-is-not-monotone-in-the-cell-bound`,
`budget-refusal-drops-the-enclosure-the-caller-needs`); then
`a-third-spelling-of-cut-every-span-into-splits-pieces` with its
rider, and `patch-bound-offset-fit-recentring-origins` (a measurement).

**On Ev**: `approx-surface-tolerance-is-now-always-the-runs-eps` —
the field is named in the ratified O2 clause
(`crates/geom-brep/README.md`), so whether it retires is a design
choice; it goes to Ev as an `[ev]` PR with a recommendation.

**Last**: `one-pass-refinement-would-cut-the-rational-bounds-widening-tail`
(P2, nothing unsound; it moves every `f64` refined net's last bits).

## Review posture

The review tiers of `memories/orchestration-model.md`, all lanes Opus.
The model A/B this plan used to inherit (protocol v7) is suspended, so
there is no triage question left to answer. The tier is named per unit
in `log.md` at dispatch.
