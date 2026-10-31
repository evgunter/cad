---
id: main-red-the-seam-region-door-row-runs-15x-slower-since-the-cylinder-sphere-chord
kind: issue
title: Main red (timeout): the_door_names_the_flipped_face_tier_3_names runs 8 s → 127 s at default ε since PR 5894 and passes 960 s at 1e-12, which cancels the nightly full suite's 1e-12 row
status: closed
opened: 2026-10-29
priority: P1
cost: M
closed: 2026-10-30
branch: sect/seam-region-speed
---


Found by nightly triage 2026-10-29, run
[40823227622](https://github.com/evgunter/cad/actions/runs/40823227622)
(main `5648e577eb`), job `full suite (every eps row, slow set included)`.
The job was **cancelled**: at the 1e-12 ε row,
`sweep::all the_door_reads_seam_regions_as_tier_3_does::the_door_names_the_flipped_face_tier_3_names`
(`crates/sweep/tests/the_door_reads_seam_regions_as_tier_3_does.rs:228`)
was still running at 960 s when the job hit its time limit. In that
row 16,375 of 16,376 tests had finished, so this one was the only 1e-12
result lost. The steps after the ε loop (doc-tests, the slow-set names
check, the per-op scalpel, wasm32) never ran. The row passes slowly at
default ε and 1e-6, and at 1e-12 it was never seen to finish:

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
1e-12 row fits the job's limit. Until then, the nightly holds the row
out at 1e-12 (below).

Banded P3: the answers are right. The harm is a guard (the nightly's
1e-12 row) going blind. That makes it tooling against silent bugs.

Reproduce: `cargo nextest run -p sweep --ignore-default-filter -E
'test(=the_door_reads_seam_regions_as_tier_3_does::the_door_names_the_flipped_face_tier_3_names)'`
(time it), at both merges.

## Held out at 1e-12 (2026-10-29, GREEN)

`.github/workflows/nightly.yml`'s full-suite step runs its 1e-12 row
with `-E 'not (binary_id(sweep::all) & test(=…))'`, citing this file,
so that row finishes inside the job's limit again. The default-ε and
1e-6 rows still run the test (231 s and 47 s on nightly 76).

Whether it is slow or hung at 1e-12 is unknown: it was never observed
to finish there. Closing this row removes the exclusion, and un-pinning
owes a 1e-12 timing of the row first.

Time it at 1e-12: `CAD_TOLERANCE_EPS=1e-12` before the reproduce line
above.

## Re-banded P1 (SECT, 2026-10-29)

SECT's orchestrator re-bands it: the slowdown is SECT's own (PR 5894), and
the nightly's 1e-12 full-suite row is the guard that now runs blind on it.
PR 6011 (merged 2026-10-29) adds marching rows that take up to ~30 min each
at 1e-12, so the job's headroom is now in question too. Dispatched with
that headroom measurement.

## 2026-10-30, nightly triage

Nightly [41008276467](https://github.com/evgunter/cad/actions/runs/41008276467)
(main `39bbdfc777`): 136.0 s at default ε and 27.3 s at 1e-6, both
passing, down from 231 s and 47 s. The 1e-12 row still holds it out.

## Closed

2026-10-30, by `sect/seam-region-speed`. Timings at opt-level 2 on a
4-vCPU dev box (nightly 41008276467 ran main's row in 136 s at the
default ε where this box takes 117 s).

**Where the time went.** Not the measurement door and not the
re-validation of flipped faces: validating every valid body and every
flip is 0.4 s of the row. At `07320fac` the corpus's 24 booleans of a
ball with a cylinder (rod, turned rod, stub) all refused at the join's
catch-all (`CurvedBooleanUnsupported`) in 0.2 s each, and the row took
6.5 s at the default ε and 29 s at 1e-12. From `34051ed3` 20 of them
build their chords on the carried section, certify them, and refuse
only at the result gate (on main: 9 at a projected image's missing
volume lane, 5 at a conic trim on the sphere's chart, 4 at
`RingOnCurvedFace`, 2 at the spline edge join). They cost 1.4–8.5 s
each at the default ε and 9–52 s at 1e-12, 108 of the row's 117 s and
703 of its 758 s on main. Sampled with gdb, an operation's time sits in
the chords' certificates: the join's rung-3 door (limb 2's rounds, the
tube), the projected rows' certificates at the mint, and their
re-certification at the result gate, the zip, the description and the
graft. The march itself is a few percent.

**What it should not do.** The projected rows' door computed the twin
net's span hulls (`pcurve_cache::twin_span_hulls`) afresh on every
certification of the same image, where limb 2's rounds read the same
arithmetic through a memo; the result gate's re-certification and the
next operation on the same pair paid it again. It now reads it through
a memo keyed by its whole input (`pcurve_cache::span_memo`, bits and
decorations, no decision inside). Main's row goes 117 → 86 s at the
default ε and 758 → 600 s at 1e-12. No verdict moved: every corpus body
answers as before, and the same 20 operations refuse with the same
refusals.

**What it owes.** The rest is the certificate of a body the kernel now
builds: the 20 operations reach the volume gate, and certifying their
chords is the price of getting there. Each arc's rung-3 certificate is
still re-taken five times per operation; that residue is
`an-operation-retakes-each-carried-chords-rung-3-certificate-five-times`.
B ∖ A pays everything again in the other operand order
(`sectrung/a-surface-pairs-section-is-carried-in-operand-order`).

**Split.** The row is five tests, one per part of the corpus: the rest
(`the_door_names_the_flipped_face_tier_3_names`, 8.6 s, 2.1 s, 55 s at
the default ε, 1e-6 and 1e-12), both balls with the rod (20 s, 3.3 s,
141 s), the unit ball with the turned rod (22 s, 3.8 s, 144 s), the
tilted ball with it (22 s, 3.9 s, 146 s), and both balls with the stub
(19 s, 4.0 s, 133 s). The four cylinder parts build no valid body yet,
log `VACUOUS` and assert nothing until the volume lanes land
(`spiricarm/a-projected-boundary-on-an-analytic-chart-has-no-volume-lane`).
The nightly's 1e-12 hold-out is removed. The job's headroom is
`green/the-full-suite-jobs-three-eps-rows-outgrow-its-180-minutes`.
