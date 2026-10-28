---
id: main-red-edit-rs-folds-a-face-sense-by-hand
kind: issue
title: Main red: editor-core edit.rs folds a face's sense bit by hand, so topo face_normal::no_source_file_folds_the_bit_by_hand fails
status: closed
opened: 2026-10-27
priority: P1
cost: E
closed: 2026-10-28
---


Found by ATREST's crossing-touch lane while validating #5823, and confirmed by the ATREST orchestrator on origin/main.

**What fails.** topo's fast-set row `face_normal::tests::no_source_file_folds_the_bit_by_hand` (`crates/topo/src/face_normal.rs:826`) walks every `crates/*/src` file for a value negated under a face's `sense` bit by hand. It now flags `crates/editor-core/src/edit.rs:6831`:

```
let outward = if pose.sense { pose.axis } else { -pose.axis };
```

**Where it came from.** MSOLVE's commit `6840af678f` ("R2 fix pass — the façade picks sketch-on-face's nearest orientation…") reached main through INTENT's #5742 (`intent/value-chart-orientation`, merge `19eff38c58`, 2026-10-27 04:21Z). The PR gate's change filter does not select that topo row for a diff that touches only editor-core, so the PR stayed green.

**What closing it takes.** Read the outward direction through the sense door `face_normal.rs` names (its module doc: "a consumer that wants an outward normal comes here … or hands the bit to a `geom_brep` door"). If `pose.axis` is a genuinely signed scalar used at its point of use, add it to the row's `sanctioned` list with that reason instead. The gate's blind spot (a topo source lint that walks every crate is not selected by a non-topo diff) is CIW's to weigh.

## Closed (INTENT, 2026-10-28)

Duplicate of `main-red-sketch-on-face-folds-the-face-normal-by-hand`, fixed by PR 5849 (merged 2026-10-27): `edit.rs` now reads `geom_brep::OutwardNormal::from_chart(pose.axis, pose.sense).vec()`, and no hand fold of `pose.axis` remains in `crates/` on main.
