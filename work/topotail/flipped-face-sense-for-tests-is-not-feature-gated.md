---
id: flipped-face-sense-for-tests-is-not-feature-gated
kind: issue
title: topo: Body::flipped_face_sense_for_tests is doc(hidden) but not feature-gated, an uncertified write on the public surface
status: open
opened: 2026-10-14
priority: P3
cost: E
---

(Filed by the ENCL orchestrator from the adoption survey for `[ev]` PR 5134. Read from source.)

## What

`Body::flipped_face_sense_for_tests` (`crates/topo/src/body.rs`) is `#[doc(hidden)]` but not behind a feature such as `test-support`, unlike its siblings `set_face_surface_unvouched_for_tests`, `with_entity_removed_for_tests` and `with_solids_merged_for_tests`. So a release consumer of `topo`, and of `pncad`, which re-exports `topo`, can mint a body whose face sense is flipped without certification.

`Body::set_face_sense` (`crates/topo/src/attach.rs`) also writes the bit without re-certifying. Tier-3 check 6 catches a flipped sense only partly.

## Ask

Gate `flipped_face_sense_for_tests` behind the test feature the other `_for_tests` doors use, and confirm the gate script covers it. For `set_face_sense`, say whether it is a certifying door; if not, make it one or narrow its visibility. A body past any public door should be one whose contradictions are reachable only by a kernel bug, which is the premise `[ev]` PR 5134 is deciding on.
