---
id: main-red-trimmed-and-injective-fold-a-face-sense-by-hand
kind: issue
title: Main red (slow set): mesh trimmed.rs and topo injective.rs fold a face's sense bit by hand, so face_normal::no_source_file_folds_the_bit_by_hand fails
status: closed
opened: 2026-10-29
priority: P1
cost: E
closed: 2026-10-29
---


Found by SPLITDOOR's levered-parallelism lane (PR 5983) and pinch-corner lane (PR 5981), and confirmed on `origin/main` (db3bc8beb) by PR 5983's reviewer.

**What fails.** topo's `face_normal::tests::no_source_file_folds_the_bit_by_hand` (`crates/topo/src/face_normal.rs`) flags unsanctioned hits in two files:
- `crates/mesh/src/trimmed.rs`: `1.0`, `area`, `radial`;
- `crates/topo/src/injective.rs`: `1.0`, `n`, `side`.

The row is in the slow set (`.config/nextest.toml`, inside `default-filter`'s `not (…)` list). Per-PR CI skips it unless a diff spells it, so `gate ok` stays green while the nightly is red.

**Where it came from.** Both files were last touched by SELFX fix-pass commits 89edd2d473 and 20cc37d22f. Neither lane checked whether those commits are the cause.

**Prior instances.** `work/intent/main-red-edit-rs-folds-a-face-sense-by-hand.md` and `work/intent/main-red-sketch-on-face-folds-the-face-normal-by-hand.md` were fixed by routing the fold through `face_normal`'s one home.

## Closed (2026-10-29): duplicate

The owners each filed this per file the same day, and those rows carry the fix:
- `work/tess/main-red-trimmed-rs-folds-the-face-sense-by-hand.md` (`crates/mesh/src/trimmed.rs`);
- `work/clear/main-red-injective-rs-folds-the-face-sense-by-hand.md` (`crates/topo/src/injective.rs`).

This row is closed so main's red has one owner per file. (SPLITDOOR orchestrator)
