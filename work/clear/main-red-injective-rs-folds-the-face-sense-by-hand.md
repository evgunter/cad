---
id: main-red-injective-rs-folds-the-face-sense-by-hand
kind: issue
title: main red: crates/topo/src/injective.rs negates three values by hand under a face's sense bit (topo face_normal fold census, slow set)
status: closed
opened: 2026-10-29
priority: P1
cost: E
branch: clear/sensefold
pr: 6028
closed: 2026-10-29
---

**What fails.** topo's slow-set row `face_normal::tests::no_source_file_folds_the_bit_by_hand` (`crates/topo/src/face_normal.rs:916`) walks every `crates/*/src` file for a value negated by hand under a face's `sense` bit. Run on origin/main (2026-10-29, `cargo nextest run -p topo --lib -E 'test(=face_normal::tests::no_source_file_folds_the_bit_by_hand)' --run-ignored all`), it fails with six hits beyond its `sanctioned` list, from two sources: this row covers `crates/topo/src/injective.rs`; the other, `crates/mesh/src/trimmed.rs` (`tess/main-red-trimmed-rs-folds-the-face-sense-by-hand`), is filed beside it. Found by SPLICE's structural-rim lane while validating PR 5959, and reproduced by the SPLICE orchestrator.

**The hits.**
- `crates/topo/src/injective.rs:871` `Some(if sense { n } else { -n })`
- `crates/topo/src/injective.rs:898` `Some(if sense { side } else { -side })`
- `crates/topo/src/injective.rs:1609` `let s = if b.sense { 1.0 } else { -1.0 };`

**Where it came from.** CLEAR's SELFX fix passes (`20cc37d22f`, `89edd2d473`), merged through PR 5955 (`clear/selfx`, merge `5e934f49f9`). The row is in the slow set (`.config/nextest.toml`), which the PR gate does not run, so the PR merged green.

**What closing it takes.** Read each value through the sense door `face_normal.rs` names (`geom_brep::OutwardNormal::from_chart`, or a `face_normal` door). Or, where the value is a genuinely signed scalar at its point of use, add it to the row's `sanctioned` list with that reason. The row goes green when both this row and its sibling close.

## Closed

SENSEFOLD (#6028), with this commit last. `Presented::shared_normal`
reads its normal through `OutwardNormal::from_chart(n, sense)`, which
is bit-identical. `face_side`'s `side` and `present_plane`'s
quarter-turn `1.0` are sanctioned in `face_normal.rs`'s list, each with
its reason: both are signs in chart coordinates, which no door yields.
Every self-intersection receipt is unchanged.

`face_normal::tests::no_source_file_folds_the_bit_by_hand` now reports
only `crates/mesh/src/trimmed.rs`'s hits. It goes green when TESS closes
`tess/main-red-trimmed-rs-folds-the-face-sense-by-hand`.
