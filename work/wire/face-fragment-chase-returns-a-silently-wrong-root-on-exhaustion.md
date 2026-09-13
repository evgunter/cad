---
id: face-fragment-chase-returns-a-silently-wrong-root-on-exhaustion
kind: issue
title: emit_topo's chase() falls out of its budget and returns a wrong face root with no refusal
status: open
opened: 2026-09-12
refs: [2474]
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
