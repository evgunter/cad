---
id: step-scaffold-strut-offset-is-absolute-in-a-unit-free-format
kind: issue
title: step-import mints its scaffold strut at a fixed 1.0 offset, in a format whose coordinates carry no unit contract
status: open
opened: 2026-09-15
priority: P3
cost: D
---


## Finding

`crates/step-import/src/assemble.rs`'s `insert_selfloop_tied` resolves
the one ambiguous splice — a self-loop whose two uses share their only
built anchor — by minting a temporary strut beside the self-loop's
vertex, splicing against it, and killing it. The strut's far endpoint
is the vertex offset by `STRUT_OFFSET`, which is **`1.0`, absolute**,
along `+x`.

A STEP file's coordinates carry no unit contract at this site. The
length unit is the file's (`SI_UNIT`, and the reader's `eps_in` comes
from `UNCERTAINTY_MEASURE_WITH_UNIT`), and a model may be stated in
metres, millimetres or microns. So the one offset is:

- **degenerate at the top of the range** — where `|p.x|`'s own `f64`
  spacing exceeds 1.0 the sum is `p.x` again and the strut has no
  length. That half is closed: `strut_endpoint` withholds the
  endpoint and the site refuses `StepImportError::Topology`, which is
  the standing `plant` already gave the same state (S415, PORT).
- **enormous at the bottom** — a 1.0 strut beside a part measured in
  microns is a scaffold a million times the part. It is killed by
  `kev` before the assembly finishes and no scaffold survives to rest,
  so nothing is wrong with the OUTPUT; what is unexamined is whether
  a strut that far from the body can be spliced and killed without
  the intervening `mef_chord` meeting a tolerance it now exceeds, and
  what the operators' own gates do with a chord many orders of
  magnitude outside the model's extent.

## Not answered here

S415 closed the degeneracy and deliberately did not choose a
magnitude. Picking one is a design call on this program's ground: the
candidates (a fraction of the solid's extent, a multiple of `tol`, the
spacing of `p.x` itself) trade against each other, and the choice
interacts with what `mev_line`'s certification and `mef_chord`'s
tolerance ask of a chord. `tol` is already in scope at the site.

## Where to look

- `crates/step-import/src/assemble.rs` — `STRUT_OFFSET`,
  `strut_endpoint`, `insert_selfloop_tied`.
- `crates/topo/src/euler.rs` — `mev_line`, `mef_chord` (what a chord
  is certified against).
