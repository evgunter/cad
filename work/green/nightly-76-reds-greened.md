---
id: nightly-76-reds-greened
kind: unit
title: nightly 76's reds (rustdoc, the full suite's hung sweep row, the viewer app and gui-montage jobs, k-lint dev-probe) are greened, each with its fix row on its owner
status: open
opened: 2026-10-29
priority: P1
cost: M
---


## What

Nightly 76 (run 40823227622, main `5648e577eb`) ran with #5952, #5958
and #5966 in, and was red on five jobs:

- `rustdoc (gate, every root)`: two `topo` links,
  `splitting/neighborhood.rs:122` (`OutwardNormal` unresolved) and
  `sector_shape.rs:80` (a redundant explicit target).
- `full suite (every eps row, slow set included)`: cancelled at its time
  limit while `sweep::all
  the_door_reads_seam_regions_as_tier_3_does::the_door_names_the_flipped_face_tier_3_names`
  ran past 960 s.
- `viewer app feature (eframe + wgpu)` and `render lanes / viewer gui
  montage`: new reds.
- `k-lint (dev-probe)`: still red after #5966's ray-wall pin.

## Owed

GREEN's rule (`program.md`): fix a doc link at its site, re-bless an
explained move, otherwise pin or ignore with a fix row on the owner.
A hang is pinned as an `#[ignore = "disabled red: …"]` with its row,
since it cannot be pinned at an output. Done when nightly 77 is green
on these jobs, or each remaining red is named with the open PR or row
that greens it.

## 2026-10-29 (lane `green/nightly-76-*`)

Nightly triage #6018 landed first (11:10Z), and carries some of this.

| job | state | by |
|---|---|---|
| `rustdoc (gate, every root)` | greened on main | #6018 fixed the nine links (#5956, #5982, #5920, #5909, #5954, #6009, #5923). This lane's #6022 duplicated it and is closed. `scripts/doc-gate.sh` passed on `c173123e` plus those fixes. |
| `full suite` | PR #6025 | the seam-region row is slow, not hung: 7 s → 231 s default, > 960 s at 1e-12, since #5894 (bisected by #6018). `#[ignore = "disabled red: …"]` on SECT's row. |
| `viewer app feature` | greened on main by INTENT stage 6 A's re-pin (`3ab74f5ca`, a new corpus); #6030 now carries only the bisect | the refusal halving is #5920 and the 32 grazes are #5954. The evidence is on `work/pcert/review-pick-r2-tally-moved-at-5225-azimuth-placement.md`, and the question goes to TESS. |
| `render lanes / viewer gui montage` | re-blessed on this row's PR (#6031, render commit `758a47e60`) | #5954 retires `Pattern` + `Part`×2 into a placement over an index, so two cells rename to `…-place3-space`. #6018 removed the old cells. Against the last committed cells, the geometry and the at-rest badge are identical, and only the tree moves (7 → 5 nodes). |
| `k-lint (dev-probe)` | PR #6027, then **still red** at `large-K lint` | key-split re-pin (#5954); the lily's 1e-12 stem stop pinned (#5728, bisected); census floor 22 → 19 (#5613 and #5742 retired the tests). The large-K lint then fails on 51,730 margins: `work/instr/k-lint-dev-probe-large-k-gate-fails-on-51730-margins-once-the-sweep-runs.md`. |

**What the full suite's cancellation hid.** The tail of the log is the
1e-12 row, with 16375 of 16376 tests done, so every other 1e-12 result
is in it. Those reds are: the census (#6018), `face_normal` (rows on
file), and `review_pick_r2` (#6030). Every step after the ε loop never
ran: doc-tests, the slow-set names check, the per-op scalpel, wasm32.
The default and 1e-6 rows lie past the log API's 5000 lines, and the
run's log archive is not reachable from the box. #6018 read them, and
filed the 1e-6 fuzz ray red on FLUSH.

**After nightly 76.** `scripts/doc-gate.sh` on main `a454950e3` found
two more links, broken by #6007 and #5981. PR #6033 fixes them at their
sites.

**Remains for nightly 77:** `k-lint (dev-probe)` at `large-K lint`
(INSTR's row), and whatever the never-run full-suite steps show.

