---
id: the-census-keys-no-closure-forwarder-by-its-binding
kind: issue
title: The predicate census keys a site inside a bound closure by its enclosing fn, so a second closure forwarder in one fn folds into the first
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Found by the review of #6755 (the forwarder-by-fn key), item 1.

`test_utils::source::predicate_census` keys an indirect site by its innermost enclosing `fn` item (`<file>: <fn>: <head>`). A closure bound to a name inside that `fn` is not an item, so its sites take the `fn`'s key.

**The review's plant.** Add a second forwarding closure to editor-core's `crates/editor-core/src/checks/at_rest.rs::constant_pairing`, beside the declared `zero_as`, and call it with a literal:

```rust
let pos_planted = |name, margin| decide(name, margin, band);
pos_planted("planted", m);
```

Its site folds into `checks/at_rest.rs: constant_pairing: name`, which is already listed. `"planted"` is decided and read by no census, and the roster stays green.

**Today's holding.** The rosters list each `fn` that holds a forwarding closure under `CLOSURE_FORWARDS` (32 lines across topo, geom, geom-core, geom-brep and editor-core). The `FORWARDS` cross-check does not apply to them, because their `fn` is not the forwarder.

**The fix is in `source_scope`, the tree's scope home.** It reports each `let NAME = <closure>` binding with its span, and the census keys a site inside one as `<fn>::<NAME>: <head>`. Then `CLOSURE_FORWARDS` lines become `FORWARDS` lines, which the cross-check holds to the closure's declared carrier. A second scope reader in `test_utils::source` is what `crates/test-utils/tests/reader_census.rs` forbids, and a site key that resolves through a binding is resolving a name. A hand-rolled `closure_bindings` was tried in #6755 and reverted for that reason.

**Done** when the census keys bound closures through `source_scope`, the `pos_planted` plant reds editor-core's roster, and no `CLOSURE_FORWARDS` line remains.
