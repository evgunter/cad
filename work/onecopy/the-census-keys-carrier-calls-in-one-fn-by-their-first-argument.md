---
id: the-census-keys-carrier-calls-in-one-fn-by-their-first-argument
kind: issue
title: Two carrier calls in one fn whose first argument is the same non-name expression share one indirect key (NOT_FIRST), so the second's name hides
status: open
opened: 2026-11-10
priority: P3
cost: E
---


Found by the review of #6755, item 4. `topo-name-forwarders-hide-their-callers-names` left this note, and keying by `fn` narrowed it to one function.

A declared carrier's call that holds no literal and no `const` is an indirect site keyed by its first argument (`<file>: <fn>: <first arg>`, `NOT_FIRST`/`CARRIER_CALL`). Two such calls in one `fn` whose first arguments are spelled alike share a key. The second call's name, wherever it is passed, is then answered by the first's listing.

**Plant.** In a `fn` that already holds a listed `relation_step_as(e, …, name, band)` call (`crates/topo/src/props.rs`), add a second call with the same first argument `e`, whose name is a `let` binding of a literal:

```rust
let planted = "planted";
relation_step_as(e, RelOp::Le, bound, planted, band);
```

It keys as the listed `props.rs: <fn>: e`, so `"planted"` is read nowhere and the roster stays green.

**The fix**, either:
- key such a site by the argument that carries the name, where the carrier says which one: `NameCarrier::CallAt(token, n)`;
- key each such site by its ordinal within its `fn`.

**Done** when the plant reds its roster.
