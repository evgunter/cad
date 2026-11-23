---
id: main-red-knot-split-test-folds-a-face-sense-by-hand
kind: issue
title: Main red (slow set): step-import knot_split.rs's test folds a piece's area under its face's sense bit by hand, so face_normal::no_source_file_folds_the_bit_by_hand fails
status: open
opened: 2026-11-18
priority: P1
cost: E
---


Found by ONEHOME's operand-coordinates lane (PR 7088) on a local run of the slow set, and reproduced on `origin/main` at `c9d55f754`.

**What fails.** topo's `face_normal::tests::no_source_file_folds_the_bit_by_hand` (`crates/topo/src/face_normal.rs:1552`) flags one unsanctioned hit, `("crates/step-import/src/knot_split.rs", "a")`. Every other entry matches the sanctioned list. The hit is in the C-shaped piece row's test: `crates/step-import/src/knot_split.rs:1770`, `.map(|(n, a)| (n, if sense { a } else { -a }))`, which folds each piece's area under the face's sense bit.

The row is in the slow set, so per-PR CI skips it unless a diff spells it: `gate ok` stays green while the nightly is red.

**Where it came from.** Commit 87801ff8f, the knot_split fix pass ("pin each refusal; pieces keep sense; …"), which reached main through `importgate/knot-place`.

**The fix it wants.** A test fold is still a fold. Read the signed area through `face_normal`'s one home, or compare the unsigned figure and pin the sense separately (the row already asserts `face.sense == sense` per piece at `knot_split.rs:1661`). The same shape was fixed in `work/tesslane/main-red-trimmed-rs-folds-the-face-sense-by-hand.md` and `work/clear/main-red-injective-rs-folds-the-face-sense-by-hand.md`.
