---
id: main-red-the-impl-census-reds-on-geoms-debug-for-line
kind: issue
title: main red: hand_written_impl_census reds on geom's Debug for Line, which reads its fields by name
status: open
opened: 2026-11-24
priority: P1
cost: E
---


Found by `from-f64-is-split-by-where-the-number-comes-from` (PR 7721), running the slow-set censuses that read what it changes. `cargo test -p test-utils --test all -- hand_written_impl_census::` on its head (main 9dc5eeaf merged) fails `every_hand_written_walk_is_held_to_its_declaration`:

```text
crates/geom/src/curves.rs:721 impl Debug for Line<T> — reads self.origin with no exhaustive pattern
```

`impl<T: Real> core::fmt::Debug for Line<T>` writes `.field("origin", &self.origin).field("dir", &self.dir.get())` without destructuring `Self`. PR 7721 does not touch `crates/geom/src/curves.rs`; the file's last change on main is cdfbf073 (the conic readers, 2026-11-23). The census is in the slow set, so a PR that does not edit its file never runs it. `the-impl-census-reds-on-two-arrivals-from-pr-7052` (closed) was the same census on two other impls.
