---
id: nightly-77-reds-greened
kind: unit
title: nightly 77's reds left by triage #6082 are greened: pinned or re-blessed, each on the triage's row
status: open
opened: 2026-10-30
priority: P1
cost: M
---


## What

Nightly 77 (run 41008276467, main `39bbdfc777`) was triaged by #6082, which fixed the census re-syncs, the reader-census line and the large-K step and filed or annotated a row for every other red, pinning none. This unit greens the rest on main `2e1c1bff`, citing the triage's rows and filing none in parallel:

| red | ε | disposition | row |
|---|---|---|---|
| `pncad-py` `prose_census` undecided roster (`clearance.rs`, `certify.rs`) | all | pinned: `UNDECIDED` lines citing the rows | `clear/main-red-prose-census-cannot-decide-the-selection-refusal-face-arm`, `splice/main-red-prose-census-cannot-decide-certify-errors-check` |
| same row, `merge_faces.rs` `GlueRecordBroken` (PR 6079, after nightly 77) | all | pinned, row filed | `contacthold/main-red-prose-census-cannot-decide-the-glue-record-broken-surfaces` |
| `topo` `the_planar_sense_flip_lives_in_one_place` | all | green on main since #6087 | `intent/main-red-trace-rs-reads-as-a-planar-sense-flip-re-fork` (closed) |
| `topo` `no_source_file_folds_the_bit_by_hand` | all | pinned: a `pinned` list beside `sanctioned` | `tess/main-red-trimmed-rs-folds-the-face-sense-by-hand` |
| `topo::all` `a_prism_tilted_within_the_zero_builds_as_upright` | all | pinned (`pinned_red::row`) | `splitdoor/main-red-a-prism-tilted-…-since-arm-vs-reach` |
| five torus-section rows | 1e-6 | pinned at 1e-6 | `sect/main-red-five-torus-section-rows-refuse-at-1e-6-since-components-by-foot` |
| `sym_9_retry_interval` ×2 | all | re-blessed (G, PR 5954) | `green/g-moves-four-main-reds-messages` |
| `m10_sym_profile_interval` plate digest; `sweep` thread-count `chk` | all; 1e-12 | pinned at main's value | `green/g-moves-four-main-reds-messages` |
| `perf12_census_goldens` | all | pinned: the three goldens re-cut at main's reading | `restread/main-red-perf12-…-at-5817` |
| `revalue_corpus::revalue_heat_sink_fins` | all | pinned (`pinned_red::row`) | `inlane/under-the-apart-move-heat-sink-fins-…` |

## Owed

Done when nightly 78 is green on these rows, or each remaining red is named with its row.
