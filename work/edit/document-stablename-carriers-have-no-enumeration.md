---
id: document-stablename-carriers-have-no-enumeration
kind: issue
title: Which carriers hold a StableName is answered in four places and enumerated in none
opened: 2026-09-16
status: spec
refs: [2784, stranded-appearance-keys-are-not-reported-by-dm7]
---

(Found by the style review of PR 2784, which added the document's
SECOND `StableName` carrier — the appearance store — to DM7's report.)

## The finding

`Node::payload_names` is the one exhaustive answer to "which PAYLOADS
carry a name", and a new payload kind that forgets it fails a census.
There is no such answer to the wider question — **which of the
document's own fields hold a `StableName` at all** — and four sites
now spell that list by hand, each in its own order, each a place a
third carrier can be forgotten silently:

- `edit.rs`, `stranded_names` and `stranded_appearance_keys` — DM7's
  report, the payload walk and the store walk, adjacent and
  independent. A third carrier is a third function nobody is told to
  write.
- `refactor.rs`, `part_names_are_self_contained` — the split door's
  containment check: the `payload_names` loop and the
  `doc.appearance().keys()` loop, one after the other.
- `refactor.rs`, `inline_part`'s foreign-name classification — the
  same two loops again, for the other refactoring door.
- `persist/check.rs`, the snapshot validator — `payload_names` inside
  the per-node walk, `doc.appearance.keys()` in its own pass much
  further down, far enough apart that neither reads as half of one
  list.

Four hand-written spellings of one list. Nothing fails if a fifth site
spells three of four, and nothing fails when a new `Doc` field holds a
`StableName` and no site is updated at all.

## The shape

One named answer to "which carriers hold a `StableName`", exhaustive
the way the census's `Walk` became on PR #2780: a `Doc` field added
without being placed does not compile. Each of the four sites above
then reads that answer rather than re-deriving it, and the third
carrier arrives at one edit instead of four.

Note the two carriers are NOT the same shape — a payload name has a
carrying node and a store key has none, which is exactly why DM7's
report has two arms — so the enumeration has to carry that difference
rather than flatten it. That is the design question inside the
mechanical one, and it is why this is a row rather than a sweep.

## Why it was not built at PR 2784

That unit adds the second carrier and is the change that makes the
list worth naming; building the enumeration in it would be a fourth
site's worth of refactoring riding a clause implementation. Filed at
the moment it was disclosed.

## Spec (2026-09-16, EDIT orchestrator) — middle tier, branch `edit/stablename-carriers`

**Premises, verified against the tree.** The four sites: `edit.rs`
`stranded_names` (1670) / `stranded_appearance_keys` (1708);
`refactor.rs` 1281+1317/1321 and 1711/1715; `persist/check.rs` 994 and
1077. `Node::payload_names` is exhaustive over payloads. `Doc`'s fields
holding a `StableName`: payloads (via nodes) and `appearance` (a
`BTreeMap<StableName, AppearanceRecord>`); `placements`/`witnesses` are
keyed by `RecipeNodeId`, `params` by `ParamName`, `metadata` by
`String` (PR #2784's sweep).

**The design question, ruled.** The enumeration carries the
difference: `NameCarrier::Payload { node: RecipeNodeId, name }` (a
carrying node) and `NameCarrier::Store { name }` (the appearance
store; no node) — DM7's two arms map onto it one to one, so
`stranded_names` + `stranded_appearance_keys` become ONE walk over
`Doc::name_carriers()` (or the name that says it) filtered on
`name.node == deleted`, emitting `Strand`/`StrandedAppearance` by arm.
Exhaustiveness the way the census's `Walk` is: `name_carriers` is
built by an exhaustive match over a `Carrier` enum whose variants are
the `Doc` fields that hold a name (`Payloads`, `Appearance`), with
`Carrier::ALL` iterated and the match placing each — a new variant
fails to compile until placed; a doc comment on `Doc`'s struct says
that a field holding a `StableName` is added there. Do NOT flatten to
`Vec<&StableName>` for the sites that need the node.

**What lands.** The enumeration; the four sites read it (`edit.rs`'s
two walks, `refactor.rs`'s two loops, `persist/check.rs`'s two passes),
each site's behaviour unchanged — prove it: every DM7 row, every
`refactor` row, every `persist/check` row stays green, and the DM7
order contract (payload strands before store strands) is kept by the
enumeration's own order, stated on it and pinned by the existing
rows. One new row: a mutant that omits a variant from `Carrier::ALL`
is a compile error, so the row is the compile — say so; a row that
asserts `name_carriers` over a document with both carriers yields
both, in the contracted order.

**Mutants:** drop the `Store` arm from the walk (every appearance-strand
row reds); swap the order (the order row reds); a `refactor.rs` site
reading `payload_names` again instead of the enumeration (no row reds —
say so; the sweep is what finds it, grep for `payload_names()` outside
`node.rs` and the enumeration at the end and list every hit).

**Not this unit:** a THIRD carrier; `AppearanceRecord`'s own `MetaValue`
tree (opaque, PR #2784).

**Territory:** `crates/editor-core/src/{doc.rs, edit.rs, refactor.rs,
persist/check.rs}` (EDIT); `crates/editor-core/tests/*` (also
TCOST/TINT). Middle tier.
