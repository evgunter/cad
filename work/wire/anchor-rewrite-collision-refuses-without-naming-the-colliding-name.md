---
id: anchor-rewrite-collision-refuses-without-naming-the-colliding-name
kind: issue
title: anchored() turns remap_table's collision into a static string, so a broken bijection names no name
status: open
opened: 2026-09-20
---



## Finding

Found by FIX's `remap-name-misses-lose-the-id-they-caught-at-six-refactor-sites`
lane in the sweep that row asked for — "any sibling remapper that
returns a located error into a call site that raises a name-shaped
one". Outside FIX's fence: `crates/editor-core/src/eval/anchor.rs` is
WIRE's by `paths`, and `crates/editor-core/src/eval/wire.rs` with it.

`anchor::remap_table` (`crates/editor-core/src/eval/anchor.rs`,
`:385` at the merge base) rewrites an emitted name table canonical →
program. Its one failure is an insert that collided, and it reports it
as `None` — the colliding NAME, which the `insert`/`insert_tied` call
has in hand at that moment, is dropped on the floor.

Its one caller, `wire::anchored` (`crates/editor-core/src/eval/wire.rs`,
`:1769`), raises

```rust
NodeErrorKind::Naming(names::NamingError::Emission {
    what: "program-anchor rewrite collided (bijection invariant broken)",
})
```

— a STATIC string with no name in it. Both the site's own comment and
the function doc say a collision is an internal bug, which is exactly
the case where the name that collided is the whole of the diagnosis: a
reader who hits this learns that some name in some table collided and
has nothing to grep for.

Same shape as the FIX row that found it: the inner walk knows which
name, the outer refusal does not carry it. The difference is only that
the located fact is a collision rather than a map miss, and that the
inner error is spelled `Option` rather than `Result` — so the fix
starts by making `remap_table` return `Result<NameTable, StableName>`
(or the pair, if both sides are wanted) rather than by un-dropping an
existing payload.

## What a taker owes

`remap_table` returning the colliding name, and `anchored` putting it
in the refusal — or a stated argument that a caller of an internal-bug
refusal needs nothing beyond the sentence, which has to say what a
reader does with "some name collided" when the tables are large.

Whether `NamingError::Emission`'s `what: &'static str` can carry it at
all is part of the question: it may want a sibling arm rather than a
formatted string, and that is a shape decision on WIRE's ground.
