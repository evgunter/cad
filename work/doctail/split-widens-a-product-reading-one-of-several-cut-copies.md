---
id: split-widens-a-product-reading-one-of-several-cut-copies
kind: issue
title: on D′3, a product reading one of several cut world copies reads the part's whole ⊔ after split, and inline then flatten leaves that ⊔ as an extra node
status: open
opened: 2026-10-26
priority: P1
cost: M
design: true
refs: [5636, 5763]
---


## What

Found checking D′3's branch (PR 5636, head `d2de30104d`) for whether
inline then flatten leaves the part's product `⊔` as an extra node,
which would break A4's round trip by one node.

It does not when the host's product is the `⊔` of the world copies:
three pinned instances `c, a, b` with that product, cut `{a, b}`,
round-trips under `same_up_to_ids` (4 live nodes each side).

It does when the host's product reads ONE of the cut copies. The same
three, the product set to `a`'s copy (`DocEdit::SetProduct`), cut
`{a, b}`:

- split makes the part's product `⊔(a, b)` (`refactor.rs` `split`,
  "The part's product … the `⊔` of its world copies"), and the
  remainder's product, which read `a`, reads the instance's copy
  ("A product the cut took is the instance's copy"): the product now
  holds `b` too;
- inline splices the part's `⊔`, and `flatten` re-points the product to
  the copy's source, that `⊔`, so `flatten`'s "a `⊔` … read by nothing
  … goes with the placement" does not fire;
- `same_up_to_ids(d, flat)` fails `preimage: … has none`: `flat` holds
  one `Assemble` more than `d`, and its product is `⊔(a, b)` where
  `d`'s was `a`.

The probe, appended to `crates/editor-core/tests/p2_split.rs` on that
branch:

```rust
let (doc, _c) = pinned_at(doc, p.top, [30.0, 8.0, 0.0], &o);
let (doc, a) = pinned_at(doc, p.base, [2.0, 8.0, 4.5], &o);
let (doc, b) = pinned_at(doc, p.top, [16.0, 8.0, 0.0], &o);
let copy_a = doc.output(a, 0).expect("a copy");
let (doc, _) = step(doc, DocEdit::SetProduct { product: Some(Operand::Var(copy_a)) });
// split {a, b}, inline, flatten, same_up_to_ids → fails
```

## Owed

A choice (`design: true`): the remainder's product reading a pick of
the instance's copy (the piece `a` is), or split refusing a product
that reads part of the cut. Either way the round trip then holds, and
a row pins it.
