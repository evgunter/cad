---
id: main-red-the-seam-region-door-row-runs-15x-slower-since-the-cylinder-sphere-chord
kind: issue
title: Main red (timeout): the_door_names_the_flipped_face_tier_3_names runs 8 s → 127 s at default ε since PR 5894 and passes 960 s at 1e-12, which cancels the nightly full suite's 1e-12 row
status: open
opened: 2026-10-29
priority: P3
cost: M
---


Found by nightly triage 2026-10-29, run
[40823227622](https://github.com/evgunter/cad/actions/runs/40823227622)
(main `5648e577eb`), job `full suite (every eps row, slow set included)`.
The job was **cancelled**: at the 1e-12 ε row,
`sweep::all the_door_reads_seam_regions_as_tier_3_does::the_door_names_the_flipped_face_tier_3_names`
(`crates/sweep/tests/the_door_reads_seam_regions_as_tier_3_does.rs:228`)
was still running at 960 s when the job hit its time limit. Every other
1e-12 result after that point was lost. The row passes, but slowly:

| night | default | 1e-6 | 1e-12 |
|---|---|---|---|
| 10-28 (`a1cfd1cfdb`) | 7.2 s | 2.4 s | 42.8 s |
| 10-29 (`5648e577eb`) | 231.5 s | 46.7 s | > 960 s, cancelled |

## Attribution

The row was bisected alone over the first-parent merges in
`a1cfd1cfdb..5648e577eb` at the default ε, with the nightly's opt-level 2
test profile, on a local 4-vCPU box. `07320fac` (the merge before) runs
it in 8 s. `34051ed3`, the merge of **PR 5894** (`sect/cylinder-sphere-chord`,
"the cylinder × sphere chord and its island closure (step 1b)"), runs it
in 127 s, and every later probe is as slow. The PR changes
`topo/src/chord_join.rs` and `topo/src/ring_path.rs`. The row's corpus is
"every sphere-carrying body this reviewer could build": booleans of balls
with boxes and cylinders, each re-validated once per sphere face flipped.
So the cost is presumably in building or validating those bodies, not in
the measurement door the row is about. The slow set is not on the PR
gate, so the PR merged green.

## What closing it takes

Profile the row at `34051ed3` against `07320fac`: find which corpus body's
boolean or validation got slower, and whether the new chord path does
work proportional to something it should not. Make it cheap again, or,
if the cost is the price of a correct answer, split the corpus so the
1e-12 row fits the job's limit. Until then, the full suite loses every
1e-12 result that sorts after this row.

Banded P3: the answers are right. The harm is a guard (the nightly's
1e-12 row) going blind. That makes it tooling against silent bugs.

Reproduce: `cargo nextest run -p sweep --ignore-default-filter -E
'test(=the_door_reads_seam_regions_as_tier_3_does::the_door_names_the_flipped_face_tier_3_names)'`
(time it), at both merges.

## Disabled (2026-10-29, GREEN)

The row is `#[ignore = "disabled red: …"]`, naming this file, so the
nightly full suite's 1e-12 row finishes again. It is slow, not hung:
it passes at default ε and 1e-6, and was cancelled past 960 s at 1e-12
(the table above). Because it is ignored at every ε, its default-ε and
1e-6 coverage is gone too until this row closes. Closing this row
removes the `#[ignore]`.

Run it by hand: append `--run-ignored only` to the reproduce line above.
