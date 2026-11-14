---
id: main-red-trimmed-rs-folds-the-face-sense-by-hand
kind: issue
title: main red: crates/mesh/src/trimmed.rs negates three values by hand under a face's sense bit (topo face_normal fold census, slow set)
status: closed
opened: 2026-10-29
priority: P1
cost: E
branch: tesslane/sense-door
closed: 2026-11-14
pr: 6930
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

- 2026-11-14 — moved from TESS to TESSLANE by `git mv` when TESS and TESSSIZE merged into their successor TESSLANE (`docs/doc-ledger/tess-and-tesssize-leave-the-tracker.md`), id and body unchanged.

## Unit (2026-11-14, TESSLANE)

**Letter S. Tier: single review, STYLE.** The change is two sites and reads as right or wrong on sight; what is at risk is the shape of the fix (a third spelling of the fold, or a sanction that should have been a door), which is the style lane's ground.

On main today the census is green on a `pinned` list of two hits, `radial` (`TrimmedFace::outward`) and `area` (a test's signed chart area); the `1.0` hit this row first listed is gone.

- `TrimmedFace::outward` reads the face's outward direction through the sense door (`crates/topo/src/face_normal.rs` names the by-value doors), not by negating a per-arm radial vector under the bit. Whether the door's value is unit or raw matters only as far as `outward`'s callers read it; the unit says which they need.
- The test's `area`: a door, or the `sanctioned` list with its reason, whichever is true of what it negates.
- Both entries leave `pinned`.

Branch `tesslane/sense-door`.

## Closed (2026-11-14, PR 6930)

`TrimmedFace::outward` reads `geom_brep::implicit_outward_normal`; the test's `area` is sanctioned as a chart-frame area; the census's `pinned` list is gone. The census was also red on main on three hits beyond this row (`walk.rs` `TAU`, two more `injective.rs` `1.0`), which the unit sanctioned; the `TAU` one was TESSWALK's row, claimed here and closed with this one. What the review found beyond the unit is filed: `a-faces-chart-winding-rule-has-no-home-in-mesh` (here, P1), `onedoor/the-fold-census-keys-on-file-and-text-not-on-a-site`, `clear/injective-folds-a-faces-in-chart-normal-through-a-sign-four-ways`, `onecopy/the-tour-folds-face-normals-and-flux-signs-by-hand`.
