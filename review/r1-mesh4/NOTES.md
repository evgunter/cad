# R1 probes — MESH-4 (PR #1517, frozen head 29d70b04b)

Blinded review lane `mesh/4r1-probes`. Everything here is ADDITIVE;
every mutation below was applied transiently to PR-owned files, run,
reverted, and the tree re-verified byte-identical to the frozen head
(`git diff 29d70b04b -- crates/` empty) before the next step.

## Instruments (this directory)

- `r1m4-runs.sh` / `r1m4-extract.sh` — R1's own two-build digest,
  shaped differently from the PR's: per band (default / 1e-6 / 1e-12)
  the mesh aggregated suite single-threaded with `--show-output`
  (sorted test rows + `test result:` counts + the three FNV
  instruments' hash lines), `step-import`'s poleguard suite, PLUS the
  `--features budget` mesh suite, which the PR's digest compiled out
  (the `pad` read lives behind that feature).
- `probe-band-edges.rs` — transient test appended to `sizing.rs`'s
  test module: bitwise parity of all four operations against the bare
  spellings over a sweep including the representable neighbours of the
  band edge, ±0.0, infinities and NaN.

## Runs and results

- **Two-build digest**: head raws under a fresh run of the lane at
  29d70b04b; base raws with `git checkout ba0a90a08 -- crates/mesh`
  applied to the same lane (the PR touches only crates/mesh), restored
  and re-verified byte-identical afterwards. Digest = sorted test rows
  + `test result:` counts + all 84 FNV hash lines per the three
  instruments, at three bands, plus poleguard and the budget-feature
  suite (which the PR's digest compiled out).
- **Determinism**: two independent runs of the head build produce
  bit-identical hash-line sets at every band (md5
  5e52d88c7d72df48a21fa9a46664b042 for all six band-runs).
- **Band-invariance observation**: the 28 FNV hash lines per band are
  identical ACROSS bands — the tour corpus's mesh bytes never come
  within any band of an ε decision, so the three-band leg of the FNV
  battery is trivially satisfied; the band-sensitive part of the gate
  is poleguard's per-band row behaviour (and the suite assertions),
  not the hashes.

- **VERDICT of the two-build gate**: `digest-ba0a90a08.txt` and
  `digest-29d70b04b.txt` (this directory) are md5-identical:
  `b32107d85598db884edd3d631140cf41`, 945 lines each. The PR's
  BINDING GATE reproduces under an independently-shaped instrument
  that also covers the budget-feature suite.

## Mutations (each applied, run, reverted; tree re-verified clean)

Raw logs in `mutants-summary.txt`. Outcomes:

- **M1** (raw accessor + `d <= eps.raw()` over `loop_polygon`'s
  `coincident`): inventory pin RED, walk.rs read column
  `[1,3,1,0] -> [1,2,1,0]`, carriers unmoved, sizing.rs stays 2 —
  reproduces the PR's red-first plant and CONFIRMS the corrected doc
  sentence (the carrier column does not see the accessor's body; the
  read column is the load-bearing half).
- **M2** (second raw `tol.eps()` in tessellate.rs): pin RED,
  tessellate.rs carriers 2 -> 3 — deliverable 3's enumeration pin has
  teeth.
- **M3a** (`coincident` flipped to strict `<`):
  `the_band_edges_are_where_the_operations_differ` AND
  `a_poisoned_length_is_neither_near_nor_far` RED.
- **M3b** (same mutant, poleguard at default band): GREEN — the
  corpus really cannot see edge inclusivity (the halfcap witness sits
  ~1.0002e-9 m from the pole, strictly outside the 1e-9 band), which
  is exactly the honesty note in the PR's gate section; the type rows
  are the sole defense, and M3a shows they hold.
- **M4** (`separates` respelled `!self.coincident(length)`):
  `a_poisoned_length_is_neither_near_nor_far` RED — the NaN
  independence argument is executable, as claimed.
- **M5** (`pad` widened DOWN): `pad_widens_upward_by_one_band` RED.
- **M6** (pole band genuinely widened x2): poleguard GREEN — NOT the
  expected red. Cause: at default band the witness test's branch
  accepts any S22 panic, and `closing_column`'s band (a different,
  unwidened read) still fires with the exact message the test greps
  for, masking the identification flip; at 1e-6 the witness is
  identified either way. So the corpus rows cannot distinguish
  "not-identified, panics S22" from "identified, panics S22
  downstream". **M6b**: the same mutant against the type rows —
  `the_band_edges_are_where_the_operations_differ` RED
  (`!coincident(2e-9)` fails). Decision-width is defended at the
  type, not by the corpus.
- **M7** (probe, additive): `r1_probe_ops_are_bitwise_the_bare_spellings`
  GREEN — all four operations are bit-for-bit the bare comparisons at
  the representable neighbours of the band edge, at ±0.0, at the
  infinities and on NaN; `pad` parity checked on bits.

After every step: `git checkout HEAD -- crates/` and
`git diff --quiet HEAD -- crates/` verified clean; final tree state
byte-identical to the frozen head outside `review/r1-mesh4/`.
