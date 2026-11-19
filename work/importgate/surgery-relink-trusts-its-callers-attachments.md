---
id: surgery-relink-trusts-its-callers-attachments
kind: issue
title: surgery::relink trusts each caller's hand-derived cut attachments; and the shared module lives under one consumer's namespace
status: open
opened: 2026-11-19
priority: P3
cost: E
---

Found by the single review of PR 7134.

- **The attachments are unchecked.** `relink` (`crates/step-import/src/normalize/surgery.rs` ~260–330) never checks that a cut's `ends[0]`/`ends[1]` name the loop use leaving the cut's start or end vertex. Each of its three callers derives these positions by hand: `at_of` (`knot_split.rs` ~1065), `leaving` (`normalize.rs` ~449) and `leaving` (`normalize/singularity.rs` ~1103). The old singularity walk matched by vertex, so it could not be fed an inconsistent position. One check per attachment, `use_start(all[index(at)])` against the cut's end vertex, would make the invariant fail loudly.
- **The module's home.** `surgery` sits under `normalize` (`normalize.rs:96`), yet the crate-level `knot_split` reaches into it, which makes it a core hosted inside one of its consumers. It was placed there to stay off `lib.rs` while the snap lane owned that file. Once the snap lane merges, move it to the crate level.
- **Message wording.** `Unlinked::Branch` maps to "a pole vertex with other than two edges" (`singularity.rs` ~1139), though the branch vertex may be a spoke's far end. `knot_split.rs` ~1121–1124's guard message, "a piece traced from no crossing", describes only indirectly what it detects.
