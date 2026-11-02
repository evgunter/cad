---
id: main-red-trimmed-rs-folds-the-face-sense-by-hand
kind: issue
title: main red: crates/mesh/src/trimmed.rs negates three values by hand under a face's sense bit (topo face_normal fold census, slow set)
status: open
opened: 2026-10-29
priority: P1
cost: E
---

**What fails.** topo's slow-set row `face_normal::tests::no_source_file_folds_the_bit_by_hand` (`crates/topo/src/face_normal.rs:916`) walks every `crates/*/src` file for a value negated by hand under a face's `sense` bit. Run on origin/main (2026-10-29, `cargo nextest run -p topo --lib -E 'test(=face_normal::tests::no_source_file_folds_the_bit_by_hand)' --run-ignored all`), it fails with six hits beyond its `sanctioned` list, from two sources: this row covers `crates/mesh/src/trimmed.rs`; the other, `crates/topo/src/injective.rs` (`clear/main-red-injective-rs-folds-the-face-sense-by-hand`), is filed beside it. Found by SPLICE's structural-rim lane while validating PR 5959, and reproduced by the SPLICE orchestrator.

**The hits.**
- `crates/mesh/src/trimmed.rs:876` `Some(if self.sense { radial } else { -radial })`
- `crates/mesh/src/trimmed.rs:2152` `let want = if sense { 1.0 } else { -1.0 };`
- `crates/mesh/src/trimmed.rs:3107` `let area = if face.sense { area } else { -area };`

**Where it came from.** TESS's commit `8a799289c2` ("tess: bulk-load the trimmed CDT; …"), merged through PR 5920 (`tess/sphere-tilted-circle-lane`, merge `148c1a24fb`). The row is in the slow set (`.config/nextest.toml`), which the PR gate does not run, so the PR merged green.

**What closing it takes.** Read each value through the sense door `face_normal.rs` names (`geom_brep::OutwardNormal::from_chart`, or a `face_normal` door). Or, where the value is a genuinely signed scalar at its point of use, add it to the row's `sanctioned` list with that reason. The row goes green when both this row and its sibling close.

**2026-10-30, nightly triage** (nightly [41008276467](https://github.com/evgunter/cad/actions/runs/41008276467), main `39bbdfc777`): still red at every ε. The unsanctioned hits are now `crates/mesh/src/trimmed.rs`'s three (`1.0`, `area`, `radial`) and nothing else. The `injective.rs` sibling is sanctioned on main, so this row alone holds the test red.

## 2026-10-30 (GREEN, lane `green/nightly-77-pins`)

Pinned: `face_normal::tests::no_source_file_folds_the_bit_by_hand` now expects `sanctioned` plus a separate `pinned` list naming this row's three `trimmed.rs` hits (`1.0`, `area`, `radial`), so any other hit fails the test, and the failure message names both lists. The repair landing fails it too, until the repair strikes them from `pinned`. The census walks source and stops on nothing, so un-pinning owes no re-measure. The `the_planar_sense_flip_lives_in_one_place` sibling (`intent/main-red-trace-rs-reads-as-a-planar-sense-flip-re-fork`) is green on main since #6087.
