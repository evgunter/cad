---
id: split-widens-a-product-reading-one-of-several-cut-copies
kind: issue
title: on D′3, a product reading one of several cut world copies reads the part's whole ⊔ after split, and inline then flatten leaves that ⊔ as an extra node
status: closed
opened: 2026-10-26
priority: P1
cost: M
refs: [5636, 5763]
closed: 2026-11-17
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

## Decided (DOCTAIL orchestrator, 2026-11-17, from two concurring designers)
The product slot is a reader like any other slot (DESIGN `[operations]` "being the product is a use"; VR2; ASSEMBLY.md A4 *Split* "re-pointed to a pick of the instance's copy (the member of the part's `⊔` that copy is, or the copy whole)" and A10 "Split and inline … re-point the slot where they take what it reads"). The code departs from that text: `Doc::reader_counts` does not count the product, `Doc::unused` patches it in by hand, `product_read` refuses a pick, and split, inline and flatten each have a product branch that knows only "the copy whole". So split widens the product (probe: volume 9 before, 12 after), inline widens a pick-product, and flatten strands one. The fix is one enumeration of read sites that includes the product, re-pointed by the shared rule. No refusal. When the cut already holds the body that is the part's product (the one world copy, or a `⊔` of exactly the cut's world copies), that body is the part's product, and split mints no second `⊔`.
Split refuses a product whose read the part's world carries no copy of, A4's severed-edge rule, as it does a node seat (added at review).

## Closed

The product slot is a read site. `Doc::read_sites` (`crates/editor-core/src/doc.rs`, over `Readers`, welded by `f6_variants!` as `Carrier` is) enumerates a node's slots and operands, a variable's definition and the product slot; `reader_counts`, and through it `unused`, `unread_anonymous_vars`, `shared_unnamed_vars` and the load door's `first_unnamed_reader_fault`, walk it. `SetProduct` admits a pick and a source written in place, minting the anonymous variable the slot alone reads; the gather reads a pick's piece (`product::gather_pick`). Split re-points the product through `heir_of` with node seats (`ReadAt`), and takes a cut `⊔` of exactly the cut's world copies that something reads as the part's product; inline re-points it through `host_body`, flatten through `in_source`, continuation through `handed`. Rows in `crates/editor-core/tests/p2_split.rs`.

At review: `SetProduct` lowers its read through the node seat's own `lower_read`, and the gather follows a source to its body. Split, inline, flatten and `continuation::splice` walk `Doc::read_sites` for their seats, sources and product. A named source over a cut copy that crosses whole is defined afresh over the instance's copy. Inline refuses a part whose product is no operation's output as `UnplaceablePart` rather than panicking. The round-trip comparator (`tests/fixture/round_trip.rs`) now compares the product. Filed: `inline-of-a-part-whose-product-is-a-pick-reaches-an-unreachable` (P3), `a-named-source-over-a-cut-copy-whose-heir-is-a-piece-is-stranded` (P2, design), and SPACES' `the-viewers-product-readers-know-only-a-direct-read-or-a-union`.

At the delta review: a seat or the product slot reading through a source is now re-pointed by what the source reads, a pick under it included. A named source or pick the remainder keeps that cannot be carried is refused (`SplitError::NamedReadUncarried`). `continuation::splice` hands sources over in the read-site walk's `Definition` arm. The "picks no piece" sentence has one home (`edit::PicksNoPiece`). `ProductReadFault::Member` gives the true reason: the gather reads no single member. Filed: `inline-and-flatten-redefine-a-placements-source-where-split-rewrites-it-with-the-seat` (P2).

At the second delta review, these changes landed:
- One rule (`seat_through` / `reread`) re-points a seat through the sources it owns, in split's seats and product slot and in inline's and flatten's product slot.
- A pick of a pick crosses by its composed path.
- The `NamedReadUncarried` guard admits a source that rides one split carries.
- A named source or pick every reader of which is cut goes with the cut.

Filed: `a-named-pick-only-the-cut-reads-refuses-as-a-part-edit` (P3).

At the third delta review, these changes landed:
- "The path a pick reaches" has one home (`Doc::pick_path`), used by the edit door, the seat's evaluation, the gather and split.
- The owned-source helpers are one walk (`owned_chain`).
- "Every reader is cut" follows anonymous definitions.
- The part's carry writes a pick afresh.
- `NamedReadUncarried` says why (`NamedUncarried`).

`a-named-pick-only-the-cut-reads-refuses-as-a-part-edit` is closed.
