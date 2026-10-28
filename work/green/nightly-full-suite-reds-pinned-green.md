---
id: nightly-full-suite-reds-pinned-green
kind: unit
title: every red row in the nightly full suite outside the slow-set row is re-blessed or pinned green, each with its fix row on its owning program
status: open
opened: 2026-10-28
priority: P1
cost: H
---


## What

Nightly 74 (run 40637814390, main `a1cfd1cfdb`) failed 141 test rows
across the three ε in `full suite (every eps row, slow set included)`.
`main-reds-editor-core-slow-set-and-the-k-probe-sweep` covers some of
them; most sit on rows that kernel programs filed (`grep -rl
'main-red' work/`) and have not greened.

## Owed

Per GREEN's charter (`program.md`, "A red is greened, not fixed,
here"): for each red row in tonight's run that is neither in the
slow-set row nor on an open PR that already greens it (e.g. #5935),
re-bless an explained deliberate move, or pin what main produces (or
`#[ignore = "pinned red: work/<program>/<row-id>"]`) and make sure a
fix row exists on the owning program, naming the bisected merge, the
old and new values and the reproduce command. Done when the nightly's
full-suite job is green, or every row left red is named with the open
PR that greens it.

## 2026-10-28 — the full-suite pins (`green/full-suite-pins`)

Nightly 74's 141 failing rows are 60 tests. Taken out:

- the slow-set row's 28 (`main-reds-editor-core-slow-set-and-the-k-probe-sweep`:
  docm9 ×5, m10_3 planted flip, m10_3_r2, m10_6, m10_9 pad and registry,
  m10_sym ×2, revalue ×7, sym11, sym_9 ×2, intent_s2_c_world,
  m4_pr3_names_ci, perf12_census_goldens);
- PR 5935's eight (since merged): m10_3 certification width, m10_4_r1,
  m10_9 no-registrant, perf1_torus_sizing, band_turn_overrun leaning
  prisms, operand_gate_support_plates, reach_split_gate azimuth and pose;
- two that main had greened by `91c83c38`: `an_annular_tube_through_a_plate::thin_nested_rings_build`
  (PR 5911) and `germ_torus_doors::subtract_and_intersect_refuse_where_union_does` (PR 5925).

The other 22 are greened on this branch: 17 pinned at what main
produces, 2 `ignore`d, 3 re-blessed (the census arrivals). Each cites
its fix row; the PR body has the table. Filed:
`offset/main-red-two-review-4781-rows-refuse-at-limb-2s-kernel-limit-since-5728`
and `spliceseam/main-red-the-lofted-cylinders-far-union-builds-the-wrong-volume-at-1e-6`.

Remaining: the slow-set row's 28 (its own lane), and whatever tonight's
nightly shows that this branch's base did not carry.
