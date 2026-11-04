---
id: main-red-the-arc-rim-pins-read-the-retired-refusal
kind: issue
title: Main red since PR 6484: dm1's arc-rim pins read the retired 'not an arc rim' refusal, and the lofted arc wall's Zero count went 10 to 18
status: open
opened: 2026-11-04
priority: P1
cost: S
---


Filed by the ONEDOOR main-red lane. Four slow-set rows are red on main
from PR 6484's merge (`bc927bc4b3`, PCERT E5) on: green at its first
parent `142bc8e616`, red at the merge and at the PR's own head
`e0b89be2ce` (so no concurrent merge is involved), and still red at
`aef1e58929`. All four are in `.config/nextest.toml`'s slow set, and
6484 edited none of their files, so the per-PR gate never ran them;
no nightly has run on a main that holds 6484 yet.

## What

**1. The arc-rim pick's refusal changed, and the dm1 pins read the old
words.** E5 retired the arc-rim arm's
"no boundary row of this chart … — not an arc rim of this chart" refusal:
`nurbs_iso_derive`'s `arc_rim` now answers a pick that shows no row, and
a row certificate that escalates or refuses at its residual, as
`PcurveCertifyError::NoGeneralLane { gap: LaneGap::Unwritten }`
(`crates/topo/src/pcurves.rs:1323`, `:1334`). `dm1-id-214.stp`'s
half-edge `HalfEdgeKey(15v1)` (a circle on the l-bracket wall `#382`)
now refuses as

    pcurve certification: the circle carrier is on the nurbs chart but
    is none of its own curves, so no closed form images it, and its
    general image is not written yet for this pair. There is no way
    through yet

and three rows still want "not an arc rim of this chart":

- `step-import::all r1_dm1_probe::dm1_no_longer_refuses_at_the_instancing_gate`
  (`crates/step-import/tests/r1_dm1_probe.rs:134`);
- `step-import::all r1_dm1_probe::the_l_bracket_alone_adopts_its_reversed_slit`
  (`crates/step-import/tests/r1_dm1_probe.rs:326`);
- `step-import::all tier_gate::every_corpus_import_passes_the_shared_gate`
  (`crates/step-import/tests/tier_gate.rs:342`–`:344`, `:462`
  `ARC_RIM_NOT_HELD`).

`crates/step-import/tests/wild.rs:91` names the same words; its dm1 row
is skipped there, so it is stale rather than red.

**2. The lofted arc wall's rim checks answer more Zeros.**
`sweep::all a_loft_computes_its_walls_in_t::a_lofted_arc_wall_is_a_function_of_the_sections_bulge`
collects 18 symbolic `pcurve_iso_boundary` Zeros where it pins 10
(`crates/sweep/tests/a_loft_computes_its_walls_in_t.rs:116`–`:118`).
All 18 are proven, so the property the row states holds; what moved is
the count. E5's `arc_rim` now certifies the picked row itself
(`crates/topo/src/pcurves.rs:1334`) before the mint's own
certification, which would add the extra iso-boundary checks.

## The question for PCERT

Whether each new reading is the right one. For dm1: is "not yet, no
general lane" the right ending for a circle the pick shows on no row,
or does it hide a rim the row does hold (the comment at
`crates/step-import/tests/wild.rs:85`–`:90` says the rows state the
circle with unnormalized weights)? If it is right, re-pin the three
rows and `wild.rs:91` to the new words. For the loft: is the second
certification of the same row intended, or a repeated read the mint
should reuse? If intended, the pin becomes 18 with the reason.

## Reproduce

    cargo nextest run -p step-import -p sweep --no-fail-fast -E \
      'test(r1_dm1_probe) | test(=tier_gate::every_corpus_import_passes_the_shared_gate) |
       test(a_lofted_arc_wall_is_a_function_of_the_sections_bulge)'
