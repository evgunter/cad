---
id: face-fragment-chase-returns-a-silently-wrong-root-on-exhaustion
kind: issue
title: emit_topo's chase() falls out of its budget and returns a wrong face root with no refusal
status: closed
opened: 2026-09-12
refs: [2474]
pr: 2518
closed: 2026-09-13
---


## Finding

Found by PR 2474's R1 reviewer under Q4 (an invariant a bugfix
establishes sweeps its siblings). Accurate at `245e1445d`, which is the
head where the sibling was fixed.

PR 2474 made `emit_topo`'s two split-lineage chases refuse when their
budget is spent — `chase_edge_to_table` (via `Body::split_root`) and
`chase_b` — because spending a budget bounded by the arena means the
walk revisited a key, and a revisit is a corrupt record. `chase()` at
`crates/editor-core/src/names/emit_topo.rs` is the third walk of that
shape and was **not** swept:

```rust
fn chase(rows: &BTreeMap<FaceKey, FaceKey>, mut f: FaceKey) -> FaceKey {
    for _ in 0..=rows.len() {
        match rows.get(&f) {
            Some(&p) => f = p,
            None => return f,
        }
    }
    f
}
```

On exhaustion it falls out of the loop and returns the key it happened
to be holding. That key is then used as a **group key** and handed to
`upstream_name`, so a cycling fragment map does not refuse — it names
faces after the wrong root, which for a naming kernel is the worst of
the three outcomes (a wrong name beats a refusal only in the sense that
nobody notices).

Four call sites, all already inside functions that return
`Result<_, NamingError>`: `name_split`'s chord grouping, `descend_face`
(twice), and `group_fragments`' `divided` set and its per-face root —
the last of which is a `.map(..).collect()` that becomes a
`collect::<Result<BTreeSet<_>, _>>()?`.

## Why PR 2474 did not sweep it, said plainly

**This is a labelled half-fix, not an oversight.** The refusal that PR
built is `NamingError::SplitLineage(topo::SplitLineageCycle)`, and
`SplitLineageCycle` carries an `EdgeKey`. `chase` walks FACE fragment
rows, so the refusal it needs carries a `FaceKey` — a second
locator-carrying variant, which means a second `naming_error_tag` word,
which `crates/pncad-py/src/tests.rs`' own banner calls new **public
Python surface**. Minting a public tag word for a defect surfaced in a
fix pass is a vocabulary decision of its own and does not belong in a
fix pass; raising `Emission { what: "a face fragment chain cycles" }`
instead would land a deliberately locator-less refusal in the PR whose
whole subject is locator-less refusals.

## What a taker owes

A refusal that names the cycling FACE. Two shapes worth weighing before
picking one:

- a sibling variant carrying a `FaceKey`, mirroring `SplitLineage`, at
  the cost of a second Python word; or
- generalising the two into one variant over `names::table::EntityKey`,
  which already spells Face/Edge/Vertex/Body — one word for the class,
  at the cost of re-cutting a variant that landed in 2474 and that two
  reviewers have already read.

Whichever, the same Q6 note `chase_edge_to_table` now carries belongs at
the new raise: cycles in this family are real (a graft copies
`SplitEdge` records with their source keys), and no case constructs one
this particular walk can reach.


## Closed 2026-09-13 (PR 2518)

`chase` returns `Result<FaceKey, NamingError>` and refuses
`FragmentLineage { face }` on a spent budget — the face it was **asked
about**, mirroring `Body::split_root`, not the cursor it happened to be
holding. Seven call sites, not the four this row counted
(`descend_face` has four arms); every one was already inside a
`Result<_, NamingError>` function, so no signature moved.

### The unguardable note was false, and the guard exhibits the defect

The unit shipped *"no door reachable from this crate builds one"* at two
claim sites. **`topo::BooleanNaming` is a public struct with public
fields that the emitter takes as data, and this file's own rows had been
minting synthetic `face_fragments_a` for two milestones.** The claim was
about a call graph, made without reading the tests in the file being
edited. Caught by the review, which wrote the guard in ~35 lines.

The guard does better than pass. With the refusal removed,
`name_boolean` returns `Ok` with a **total 27-row table** in which the
two cap faces carry **each other's names**: `top = FaceKey(1v1)` takes
`FromA([Cap(Start)])` and `bottom = FaceKey(2v1)` takes
`FromA([Cap(End)])`. Nothing missing, nothing refusing, and the document
wrong about which face is which. Re-measured by the lane rather than
quoted from the review.

### And the class reached PR 2474, which had already merged

**`chase_b`'s identical note was wrong too, and it is now guarded** — one
synthetic `graft_edges` row closes a loop that provenance records alone
cannot, because that walk advances in two steps and only the first is the
caller's data.

**`chase_edge_to_table` genuinely cannot be reached, and the reason is
now checkable rather than a survey**: it advances only on
`Body::edge_provenance`, which is `pub(crate)` to `topo`; its one writer
is `Body::split_edge`, which records the parent on a child it has just
minted — so every record points at a key that already existed and a
chain is **strictly decreasing in age**. `grep Unguardable` over
`editor-core/src` now returns exactly one hit, at that site.

**The dividing line the family turned out to have is writer access**, and
that is the transferable part: a bounded walk is guardable exactly when
something outside the crate can write a step of it.

### The sibling the sweep found, priced and handed over

`topo/boolean`'s `KeyView::live_vertex`/`live_face` answer `None` on a
spent budget — **also** their answer for a chain that simply ends — so a
cycle reads as "genuinely consumed" and a declared contact is dropped.
Filed on BOOL with the reviewer's repro attached and its measured
numbers: cycle → 0 records, dead end → 0 records, indistinguishable. BOOL
inherits a measurement rather than an argument.
