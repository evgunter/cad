---
id: the-three-station-arc-loft-fixture-has-eight-spellings
kind: issue
title: The three-station arc loft is spelled eight times across crates/sweep/tests; three were byte-identical
status: open
opened: 2026-09-21
priority: P4
cost: E
---


## Finding

- **Where**: `crates/sweep/tests/` — see the hit list.
- **Importance**: low-medium. Nothing has drifted yet in the
  byte-identical group; the scaled group already has four spellings of
  one shape with four independent reasons attached.
- **Confidence**: sure for the hit list, which was read site by site;
  the blind spots below are stated rather than assumed away.
- **Raised by**: ATREST-2's fix pass, 2026-09-21, which was about to
  mint a fourth copy and was sent to sweep for the rest instead.

`crates/sweep/tests/common/mod.rs`'s `arc_section` already carries this
program's rule at its own level (*"One copy for the crate. It was four
…"*). The BODY built from it did not: the loft call around it was
written out per suite.

## What this unit did

Hoisted `arc_prism` and `square_prism` into
`crates/sweep/tests/common/mod.rs` beside `quintic_prism`,
`tilted_cut_upper` and `bulged_extrusion` (bodies already live there,
so this is the module's existing routing and not a new home).
`reporting_door_bit_digest.rs` and `m5_s10_face_sense.rs` now use them.
`the_reporting_doors_bits_are_unchanged` is green over the move, which
is the evidence the bodies are the same bodies.

## The hit list

Pattern: every file under `crates/sweep/tests/*.rs` containing both
`loft_body` and `arc_section` (34 files call `loft_body`; 12 call both).

**Byte-identical to `arc_prism`** — `[arc_section(1.0); 3]`,
`stacked(&[0.0, 1.0, 2.0], 1.0)`, v-degree 2:

| site | disposition |
|---|---|
| `reporting_door_bit_digest.rs::arc_prism` | hoisted to `common` |
| `m5_s10_face_sense.rs::atrest2_arc_loft` | deleted, uses `common::arc_prism` |
| `m8_3_rational_volume.rs`, inline in `tier3_admits_the_rational_wall_body_and_its_volume_brackets_the_extrusion` | **not this unit**: the row's neighbour builds the same profile through `extrude` as its oracle, so the two constructions are the row's subject and want reading together before either moves |

**The same shape scaled by `s`** — `[arc_section(s); 3]`,
`stacked(&[0.0, 1.0, 2.0], s)`, v-degree 2. These four are
byte-identical to each other under a rename:

| site | disposition |
|---|---|
| `continuation_is_thread_count_invariant.rs::arc_loft(s)` | **not this unit**: wants one scaled `arc_prism_at(s)` beside `arc_prism`, which is a four-suite change |
| `mass_props_are_thread_count_invariant.rs::arc_loft(s)` | same |
| `sign_certified_plus_v.rs::arc_loft(s)` | same |
| `shell_census_is_thread_count_invariant.rs::arc_loft()` | same, with `s` fixed at `1e9 * eps` inside |

**Deliberately different, with the reason stated at the site** — left
alone, and cited here so a later sweep does not fold them by mistake:

| site | why it differs |
|---|---|
| `tcost_k3_certificate.rs::arc_prism(s)` | 2 stations, v-degree 1, *"the extra station doubles the quadrature's cost for a property no row here reads"* |
| `reporting_door_bit_digest.rs::arc_taper` | sections of DIFFERING scale — a different quadrature lane |
| `transform_nurbs_walls.rs` (~:59) | `[1.0, 1.4, 1.0]`, a taper |
| `cert5_offgrid_knot_rational.rs` (~:80) | `n` sections, `n` is the row's variable |
| `mass_props…` / `shell_census…` (~:356, ~:366) | 2 sections at `1e9*eps` |

`square_prism`'s shape is narrower: two sites held it
(`reporting_door_bit_digest.rs`, `m5_s10_face_sense.rs`, both folded).
The bare `quad([(-1,-1),(1,-1),(1,1),(-1,1)])` SECTION appears in three
more files (`bool6_r2_probes.rs`, `vrev_reversed_chart_hazard.rs`,
`vrev_acceptance_set.rs`) but none of them lofts it into this body.

## What the pattern could not match

- **A hand-rolled bulged section.** The pattern keys on the name
  `arc_section`. `crates/sweep/tests/pcurve_p1b_r2_probes.rs` builds a
  bulged `ProfileVertex` chain inline (~:96) and was checked by hand:
  it calls `loft_body` nowhere, so it is not a member — but a file
  that did both would be invisible to this grep. The same blind spot
  covers any copy that inlines the `0.4142135623730951` bulge.
- **Other crates.** Scoped to `crates/sweep/tests/` deliberately.
  `arc_section`'s own doc records that `step-import` holds four copies
  of the SECTION and routes cross-crate constant de-duplication to
  LIB-U6; whether any of them is lofted was not measured here.
- **A body reached through a wrapper.** A suite that lofts an arc
  section inside a helper in `sweep::test_support` and calls only the
  helper matches neither term at the call site.
- **Accuracy is as of this branch's merge base.** A copy landed since
  is not in the list.
