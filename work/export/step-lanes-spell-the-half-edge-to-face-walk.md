---
id: step-lanes-spell-the-half-edge-to-face-walk
kind: issue
title: step-import/src and step-export's example spell the half-edge to face walk
status: closed
opened: 2026-09-19
priority: P1
cost: E
branch: export/walks-and-zeros
pr: 7067
closed: 2026-11-17
---


## Finding

- **Where**: `crates/step-import/src/adopt.rs` (~:84, ~:201, ~:395,
  ~:800), `crates/step-import/src/assemble.rs` (`parent_loop`, ~:444 —
  a private door of its own), and one site in
  `crates/step-export/examples`.
- **Importance**: low
- **Confidence**: the file and line citations were read; the
  `step-export/examples` site is a probe bucket and has not been read.
- **Raised by**: PR #2857's fix pass, 2026-09-19, announced by seam from
  `work/dup/half-edge-to-face-walk-is-spelled-once-per-suite.md`.

PR #2857 exported `pncad::topo::Body::face_of_half_edge(he) ->
Option<FaceKey>`. The step lanes spell the walk out.

`assemble.rs`'s `parent_loop` (~:444) is the interesting one: it is
already a private door, but it stops at the LOOP and raises
`StepImportError` with the edge id as context — a refusal the `topo`
door cannot make, so it is the entity-naming posture and keeps its own
walk. The four `adopt.rs` sites should be read against it: where the
refusal does not distinguish the hops, they fold.

`crates/step-import/tests` and `crates/step-export/tests` hold two more
and are `work/fixture/the-half-edge-to-face-walk-is-spelled-per-test-file.md`.

## What the instrument could not see

`git grep parent_loop` over the two crates, plus a type-directed probe
bucket for the example. The probe counts field writes and 2-hop reads
alongside 3-hop read walks, so the bucket is an upper bound.

## Closed (2026-11-17)

PR 7067 (`export/walks-and-zeros`). It had a single style review and a fix pass. The signed-zero fold has one home, `geom_core::fold_signed_zero`, and the importer's half-edge walks go through `topo::Body::face_of_half_edge` or the local `body_loop`/`loop_face` helpers. The PR body lists each site and what was done.
