---
id: census-face-pair-confirm-is-a-trial-ladder
kind: issue
title: confirm_face_pair tries four readers in turn instead of dispatching the face pair to one
status: open
opened: 2026-10-14
---

Found by the pair-table audit (`fuse/every-pair-question-has-one-total-table`), which tests every geometric pair question against C5's rule: one total kind-pair table, one method per pair, no runtime fallback (`crates/geom-brep/README.md`, C5).

## What

`census::confirm_face_pair` (`crates/topo/src/census.rs:6629`) confirms a face-face record by trying four readers in order:
1. two planes → `confirm_patch`;
2. else `carrier_pair_relation` (`OutsideInventory` or `Distinct` fall through);
3. then `point::face_face`;
4. then `boolean::tangent_pair_at_rest`.

The order is a runtime trial ladder: which reader answers depends on which earlier one declined, not on the kind pair.

## Owed

Dispatch on the face pair's kinds to the one reader that owns that pair, so a pair no reader owns refuses typed at the dispatch. `restread/the-two-carrier-face-pair-confirm-escalates-where-its-second-face-reads-the-point-out` is a symptom on the same door.
