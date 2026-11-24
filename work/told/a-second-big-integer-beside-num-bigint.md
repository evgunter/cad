---
id: a-second-big-integer-beside-num-bigint
kind: issue
title: quantity carries its own natural-number type beside num-bigint, to stay a zero-dependency leaf
status: open
opened: 2026-11-22
priority: P2
cost: M
---


`crates/quantity/src/exact.rs` is a hand-written natural-number type (`Nat`: about 300 lines of limbs, multiplication, shift-and-correct division, Machin's series for π) that the once-rounded door (PR 7291) needs, written in-tree because `quantity` has no dependencies (`crates/quantity/Cargo.toml`'s note: it sits below `editor-core` and is re-exported by `pncad`). `geom-core` already depends on `num-bigint` for `sym::Rat`, so the kernel now carries two big-integer types. No ruling that `quantity` must stay a zero-dependency leaf was found (PR 7291's reviewers looked too).

## The question

Is `quantity` staying a zero-dependency leaf worth a second big integer, or should it take `num-bigint` (or a shared in-tree one) and drop `Nat`?
