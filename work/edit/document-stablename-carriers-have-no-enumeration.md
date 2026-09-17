---
id: document-stablename-carriers-have-no-enumeration
kind: issue
title: Which carriers hold a StableName is answered in four places and enumerated in none
opened: 2026-09-16
closed: 2026-09-17
status: closed
branch: edit/stablename-carriers
pr: 2797
refs: [2784, stranded-appearance-keys-are-not-reported-by-dm7, load-door-appearance-key-id-check-is-pinned-by-no-row]
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

## Built (2026-09-17, branch `edit/stablename-carriers`)

`Carrier` and `NameCarrier` in `doc.rs`, with `Doc::name_carriers` and
the private `Doc::names_in` behind it, and the four sites read them.

- **The enumeration.** `Carrier` has one variant per `Doc` field that
  holds a `StableName` (`Payloads`, `Appearance`); `Carrier::ALL` is
  what `name_carriers` iterates and `names_in`' wildcard-free match is
  what places each. `NameCarrier::Payload { node, name }` and
  `NameCarrier::Store { name }` carry the difference the spec ruled;
  `NameCarrier::name()` is for the callers that ask one question of
  both. `Doc`'s struct doc says a field holding a name is placed in
  `Carrier`. Both types are `pub(crate)`: no consumer outside the
  crate asked for the answer, and a public one would be a façade and
  binding-census change with nothing behind it.
- **The order** is `Carrier::ALL`'s — payload names in document order,
  within one node in `Node::payload_names`' order, then the store's
  own `BTreeMap` order — and `Applied::maintenance`'s contract now
  points at it instead of restating it.
- **`edit.rs`**: `stranded_names` and `stranded_appearance_keys` are
  one function, `stranded_references`, which is `name_carriers()`
  filtered on the deleted node and mapped arm to arm.
- **`refactor.rs`**: all three name walks in the split and inline
  doors read the enumeration (the cut-side containment check, the
  remainder-side classification, `inline_part`'s classification).
- **`persist/check.rs`**: the payload walk inside the node loop and
  the store pass hundreds of lines below it are one pass over
  `name_carriers()`.
- **The row**: `doc::tests::name_carriers_reads_the_payloads_then_the_store`
  (both carriers, in the contracted order, over a fixture whose
  document order is the reverse of its id order and whose store keys
  sort before its payload names) and
  `doc::tests::the_carrier_roster_is_what_the_walk_iterates` (the
  `Carrier::ALL` half of the weld, `f6_variants!` + `set_difference`,
  the way `persist::check`'s `Walk` roster is).

**Corrections to the spec, measured.**

1. The spec said a mutant omitting a variant from `Carrier::ALL` is a
   compile error, so the row is the compile. It is only half true:
   dropping an entry alone is a type error (the array's length is its
   type), but dropping the length with it compiles, with a `dead_code`
   warning. And the likelier mistake — a variant ADDED to `Carrier`,
   placed in the match, and forgotten in `ALL` — compiles clean.
   `the_carrier_roster_is_what_the_walk_iterates` is the row that
   holds it, and it reds on both.
2. The spec said a `refactor.rs` site re-deriving from
   `payload_names` reds no row. It reds two, at each of the two
   classification sites (see the mutant table).
3. **`crates/editor-core/src/refactor.rs` is FIX's, not EDIT's**
   (`work/fix/program.md` `paths`); the spec's Territory section says
   EDIT. Crossed by announcement in the PR body.

**Not built here** (filed at the moment it was disclosed): the load
door's appearance-key id check is pinned by no row —
`work/edit/load-door-appearance-key-id-check-is-pinned-by-no-row`. It
was carried at the fix pass instead; see below.

## After the review (2026-09-17, verdict APPROVE-WITH-FIXES)

Every finding taken. What changed in the tree:

- **The named pin that did not exist.** `name_carriers`' doc said
  `edit::tests` and `dm7_delete_strands` held the order at the door.
  `edit::tests` holds one row and it is about mate-graph
  reconciliation. The doc now names the three rows that actually red
  when `Carrier::ALL` is reversed — measured, no others do:
  `doc::tests::name_carriers_reads_the_payloads_then_the_store`,
  `dm7_delete_strands::an_appearance_strand_follows_the_payload_strands_of_the_same_delete`
  and `..._precedes_the_cluster_acts_of_the_same_delete`. Every
  "pinned by" sentence in the diff was re-read against a mutant run
  before the push.
- **What the compiler forces, exactly.** A `Carrier` VARIANT is
  compile-forced, at two matches (E0004 at `Doc::names_in` and at the
  roster row's `f6_variants!`-generated match). A `Doc` FIELD is not:
  placing it as a variant is the struct doc's prose and the one step a
  person takes. The three sentences that blurred the two now say which
  is which.
- **The validator's order change is disclosed.** The name pass moved
  from id order over `doc.nodes` to document order over `Doc::order`,
  so a doubly corrupt file's first refusal can change. The review
  probe `rv_the_name_pass_refuses_in_document_order` is adopted as the
  row that pins which order it is now, with a doc saying that the
  order is NOT a contract — `validate_snapshot` promises a typed
  refusal, not which fault it names first — and why it is pinned
  anyway: an unpinned order that changes silently is how a diagnosis
  drifts one refactor at a time.
- **The store half of the load door's id check is held.** The review
  probe `rv_an_appearance_key_past_the_mint_counter_refuses_typed` is
  adopted. M6 (the validator skipping the `Store` arm) reds it, and
  only it. `work/edit/load-door-appearance-key-id-check-is-pinned-by-no-row`
  closes at this merge.
- **The roster row's residual is written down**: the census pins that
  every carrier is walked, not that an arm reads its own field, nor
  that a third carrier gets a `NameCarrier` shape that suits it. No
  new row — an arm that yields nothing reds thirteen rows already
  (M9), so a per-carrier non-empty row is subsumed.
- **`name_carriers` yields an iterator** (`impl Iterator<Item =
  NameCarrier<'_>>`, arms boxed behind the exhaustive match), so the
  four readers that refuse at the first bad name stop there. The order
  row and the roster row still red under their mutants (M2, M4, M5).
  The cost sentence now says the measured truth: the per-node `Vec`
  `Node::payload_names` returns is the node's own and unchanged; the
  walk adds no vector of its own.
- **`carrier_names` is `names_in`**, so the pair is no longer an
  anagram; `Applied::maintenance`'s pub doc states the order in words
  a consumer can read, since `Carrier::ALL` is `pub(crate)`;
  `NameCarrier::name()`'s doc cites `pncad`'s `Maintenance.name` as the
  same flattening one layer up; the `doc.rs` fixture says why no edit
  door can mint the order it pokes in.

## Closed (2026-09-17, EDIT orchestrator)

Built and merged as PR #2797 after one opus style review
(APPROVE-WITH-FIXES: 1 MAJOR, 3 MINOR, 4 NOTE, 9 style — every one
taken). `Carrier` / `NameCarrier` / `Doc::name_carriers` is the one
answer to which `Doc` fields hold a `StableName`, carrying the
payload-vs-store difference; `Carrier::ALL` is what the walk iterates
and the roster row welds it to the enum; the four hand-spelled sites
(DM7's walk, the split door's two checks, `inline_part`, the snapshot
validator) read it, with the DM7 order contract stated once and pinned
by three rows. The review's MAJOR was the shape #2784's review had
returned — a "pinned by" citation naming a test module that held no
such row — and the fix pass re-read every such sentence against a
mutant run; two undisclosed facts were disclosed (a `Doc` field is
compile-forced only at the variant; the validator's name pass now
runs in document order, pinned as a non-contract); the reviewer's
store-key probe closed `load-door-appearance-key-id-check-is-pinned-by-no-row`
on this branch. `refactor.rs` is FIX's, crossed by announcement. The
enumeration is crate-private until a consumer outside the crate wants
it (none does: measured).
