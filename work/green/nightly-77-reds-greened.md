---
id: nightly-77-reds-greened
kind: unit
title: nightly 77's reds left by triage #6082 are greened: pinned or re-blessed, each on the triage's row
status: closed
opened: 2026-10-30
priority: P1
cost: M
closed: 2026-10-30
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
| `sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured` | all | re-blessed (G, PR 5954: rule H and the DECIDE-3 trade) | `green/g-moves-four-main-reds-messages` |
| `sym_9_the_drive_writes_the_ladders_receipt`; `m10_sym_profile_interval` plate digest; `sweep` thread-count `chk` | all; all; 1e-12 | pinned at main's value, not explained | `green/g-moves-four-main-reds-messages` |
| `perf12_census_goldens` | all | pinned: the three goldens re-cut at main's reading | `restread/main-red-perf12-…-at-5817` |
| `revalue_corpus::revalue_heat_sink_fins` | all | pinned (`pinned_red::row`) | `inlane/under-the-apart-move-heat-sink-fins-…` |

## Owed

Done when nightly 78 is green on these rows, or each remaining red is named with its row.

## Closed (2026-10-30)

Every red this row names is greened on main: re-blessed with its cause,
fixed at its site, or pinned (`test_utils::pinned_red`, exact-value or
census pins, or a nightly-filter exclusion) with a dated line on the
owning program's open row. The last of them landed in #6093. The causes
live on those rows, not here. A pin that goes red later is a new
nightly red, handled from that night's triage.
