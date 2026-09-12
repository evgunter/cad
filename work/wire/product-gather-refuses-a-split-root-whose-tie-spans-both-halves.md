---
id: product-gather-refuses-a-split-root-whose-tie-spans-both-halves
kind: issue
title: The product gather refuses DuplicateName for a split root whose tied name has one candidate in each half
status: dispatched
opened: 2026-09-06
branch: wire/product-gather-tie-across-halves
---



Found by MSOLVE-5's correctness arm (PR 2090, probe P10), outside the
unit's fence; reproduces on main (`product.rs` untouched by the unit).

Document: a 4×4×4 block minus a U-shaped cutter, leaving two cap
fragments under ONE name (a genuine N2 tie, the shape
`m4_pr3_names_bool::symmetric_u_cutter_fragments_tie_and_naming_stays_total`
pins), then split by the plane `y = 2`, which separates the two
fragments without cutting either. The split's own table stays `Tied`
with one candidate per half, as `names/table.rs:186-189` documents.
The split is the only root, so the gather carries each half as its
own source — and `carry_names` (`crates/editor-core/src/product.rs:786-816`)
inserts the name once per source body: the second half hits
`DuplicateName` and the gather refuses `ProductError::Naming`
("root 8's face name (minted by node 6) collides in the product's
name table"). A document that evaluates and names totally does not
gather.

What is owed: a tie spanning the output bodies of one root needs
`insert_tied` (or a merge) on the aggregate table, or the row dropped
under a stated rule — either way a decision about the product's
table, not the split's. The probe is saved with the correctness arm's
file (`/home/user/msolve-1-scratch/correct5/msolve5_probes.rs`,
`probe10_split_root_separating_a_tie_gathers`) and is easy to rebuild
from the description above.

## Re-homed to WIRE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

WIRE is the evaluation seat's successor. `docs/DOC-LEDGER.md` sweep 9
parked three of these rows in `work/issues/` as "the successor's opening
slate" when EVAL closed on 2026-09-08, naming two more already there;
this program is that successor, and it takes the rest of the seat's
ground with them.

Its class at the cut was **M** — needs a stated rule for a tie spanning
one root's bodies; probe exists, fix is local. The class is a dispatch
estimate made by reading the row against the tree on 2026-09-11, not a
verdict on the finding, and a lane that finds it wrong says so in its
PR. The id, the `track:` letter where the row carries one, and the body
above are unchanged by the move.

## The mechanism already exists, and `carry_names` is bypassing it (2026-09-12)

Ev asked whether `ix` is already sufficient to determine the right
behaviour with no algorithmic change. **It is, and the reason is
stronger than `ix`**: the accumulate-then-narrow mechanism this row
needs is already built, in `crates/editor-core/src/names/defer.rs`, and
`carry_names` is a third hand-written copy of the rule it implements.

### `carry_names` is a third narrowing door

`crates/editor-core/src/product.rs:883-887`:

```rust
match moved.len() {
    0 => {}
    1 => into.insert(name.clone(), moved[0])...,
    _ => into.insert_tied(name.clone(), moved)...,
}
```

`crates/editor-core/src/names/defer.rs:148-157`:

```rust
pub(super) fn narrow_into(t, name, ents) -> Result<(), DuplicateName> {
    match ents.as_slice() {
        [one] => t.insert_ref(name, *one),
        _ => t.insert_tied_ref(name, ents),
    }
}
```

The same three-way rule, twice. And `narrow_into`'s own doc calls itself
**"The one narrowing rule for a tie's survivors"** and says `flush` and
`NameTable::project` both write through it *"so the two doors cannot
narrow differently."* `carry_names` is a **third** door that narrows,
outside that guarantee — the Q1 shape, with the reconciling sentence
present at one site and the copy invisible to it.

### `TieRows` is the accumulator, and its doc describes this exact case

`defer.rs:108-126`: `TieRows(BTreeMap<NameRef, Vec<EntityRef>>)` with
`push` (defer one row) and `flush` (drain every deferred name through
`narrow_into`). Its doc:

> Upstream candidates that were equally admissible stay equally
> admissible downstream, so their same-named descendants MERGE into one
> entry at flush: `Tied` when ≥ 2 survive, narrowed back to `Unique`
> when exactly one does.

That is this row's fix, written down, for the emitters.

### The fix, and what it costs

`carry_names` pushes a **tie-descended** row into a `TieRows` instead of
inserting it, and the gather flushes once after the last source is
grafted. `Entry::Unique` rows keep going through `insert` directly —
which is `defer::put`'s `from_tie` parameter exactly.

Three things the earlier reading of this row got wrong, all retracted:

- **No restructuring of the pass-3 loop.** `push` takes the
  already-mapped `EntityRef`, so the per-body `GraftKeys` are consumed
  inline exactly as they are now. The "you must hold every body's keys
  to insert once" problem was invented.
- **No guard is retired.** `TieRows`'s doc is explicit: *"Rows that do
  NOT descend from a tie keep going through `NameTable::insert`
  directly, so a genuine aliasing bug is still a typed `Duplicate` — and
  so is a tie-descended name colliding with a strict one, since the
  flush inserts into the same table."* The aliasing guard stays where it
  is.
- **The skipped-empty-body case handles itself.** A source whose body
  has no solids is skipped at `product.rs:697`; only the surviving half
  is ever pushed, and `flush` narrows one candidate to `Unique`. No
  special case.

**The one real cost**: `TieRows`, `narrow_into` and `put` are
`pub(super)` within `crate::names`, so `product.rs` cannot see them
today. Either widen them to `pub(crate)`, or give `names` a door that
carries one source's rows into an accumulator and let `product.rs` call
that. The second is probably right — the narrowing rule belongs beside
the table, which is the whole point of `narrow_into` having a home — but
it is the unit's call, not this row's.

### What is still a decision and not a mechanism

The remaining judgement is unchanged and small: the product genuinely
holds two faces under one name, so the aggregate table says `Tied` and
downstream owns the ambiguity (`select_where` refuses `TiedDisagrees` on
a mixed match, GS-Q4). That is a **behaviour move, not a pure repair** —
the document goes from refusing at gather to gathering and possibly
refusing later at selection — and the unit's PR should say so rather
than present it as a bugfix with no consequences.
