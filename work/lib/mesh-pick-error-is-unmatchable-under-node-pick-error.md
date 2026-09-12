---
id: mesh-pick-error-is-unmatchable-under-node-pick-error
kind: issue
title: MeshPickError is unmatchable under NodePickError's carrier
status: closed
opened: 2026-09-03
closed: 2026-09-08
refs: [1661]
---

Banked at LIB-B-PICKING. **Not a request to relitigate CUR3** — the
decision it observes is honored throughout that unit; what is recorded
here is a downstream consequence CUR3's own argument does not address.

## The finding

`NodePickError` (`crates/editor-core/src/resolve/pick.rs:220`) has five
arms and four of them are matchable through the façade: `Standing`
carries a curated `HitTestError`, `NotABody` and `NoSuchBody` carry a
`RecipeNodeId` and a `u32`, `Tessellate` carries a prelude-curated
`TessellateError`. The fifth does not:

```rust
    /// The tessellated mesh failed indexing (corrupt back-references).
    Index(MeshPickError),
```

`MeshPickError` (`crates/editor-core/src/resolve/pick.rs:88`) is in
`NOT_CARRIED` — `crates/pncad/tests/all.rs:3126-3127`, with the stanza
at `:3041` — so a façade consumer cannot name the type, cannot match
its arms, and cannot read its `patch` / `triangle` / `index` payload
except out of the `Display` prose.

## Why the census's own rule notices this and CUR3's argument does not

CUR3's stanza argues from CONSTRUCTION: `MeshPick` stays interior, so a
raw `PickTarget` has no constructor, so `NodePick` is the only door.
That argument is sound and this unit depends on it — it is why the
Python `pick_face` takes `NodePick`s and the confidently-wrong-name
lane (#1098) has no spelling in Python at all.

But `MeshPickError` is not reached by CONSTRUCTING anything. It arrives
as a payload of a curated refusal, and the rule the CUR3/CUR4 pair
settled is stated at
`crates/pncad-py/tests/test_binding_census.py`'s `BlendError` entry:
**a payload's category follows what its CARRIER does at the crossing.**
`ReadbackError` projects its arms as tags, so CUR3 curated
`DanglingRef` and its two lanes became two tags. `NodePickError` also
projects its arms as tags — `crates/pncad-py/src/tags.rs`'s
`node_pick_error_tag` — so by that rule its payloads should be
matchable too, and this one is the single exception on the enum.

## What it costs today

`crates/pncad-py/src/py/pick.rs` gives the arm one tag, `mesh_index`,
and forwards the kernel's prose. So:

- a Python caller cannot branch on WHICH indexing invariant broke (a
  single arm today, so the tag is currently lossless in practice — the
  loss is that a second arm added kernel-side would silently join the
  first under one tag, with no compile-time alarm anywhere, because the
  match this crate can write is on the carrier, not the payload);
- the actionable numbers (`patch`, `triangle`, `index`) are readable
  only by parsing prose, which is not a stable interface;
- `crates/pncad-py/src/tests.rs::picking_refusal_tags_are_stable`
  cannot construct the arm, so the pin has no line for it.

The same is true of `HitTestError::Unnamed`, for a different and
non-negotiable reason: its payload is an `EntityRef`, an arena key, and
G1 forbids the key crossing. That one is correctly unprojected — the
Python side projects the entity's KIND and body index instead, which is
the whole of what a bug report can act on. `MeshPickError` carries no
key at all; it is deliberately arena-key-free, by its own docstring.

## The shape of a fix, if one is wanted

Curate `MeshPickError` alone (not `MeshPick`) into
`crates/pncad/src/select.rs`, the way `DanglingRef` rides beside
`ReadbackError`, and give `crates/pncad-py/src/tags.rs` a
`mesh_pick_error_tag` with a `position_out_of_range` arm. That keeps
every word of CUR3's construction argument intact — `MeshPick` stays
interior, a raw `PickTarget` stays unconstructible — and closes the
payload half. It is a curation micro-unit, the queue `DanglingRef`
joined at LIB-B-READBACK.

Whether it is worth doing is a judgement for the curation queue, not
for a binding unit: the arm reports a corrupt mesh, which is a kernel
defect rather than anything a user provokes.

## Closed

LIB-CUR5, under LB17's rule. `MeshPickError` alone is carried into
`crates/pncad/src/select.rs` beside `NodePickError`; `MeshPick` stays
interior, so every word of CUR3's construction argument holds — a raw
`PickTarget` still has no constructor through the façade, and the
stanza in `select.rs` says so where the carriage is. `tags.rs` gains
`mesh_pick_error_tag` (`position_out_of_range`, the enum's one arm,
matched exhaustively so a second indexing invariant stops the build),
and `NodePickError`'s `mesh_index` arm projects it at `index_variant`
rather than in place of `variant`: `mesh_index` says which door's
invariant broke and a caller branching on the standing ladder needs
it to stay put. Census row on the `DanglingRef` precedent
(`BOUND_AS`, at the new spelling); `TAG_INVENTORY` row; the arm is
constructible from `pncad-py`, so `picking_refusal_tags_are_stable`
pins both words and its "two arms have no constructor" paragraph is
down to one (`HitTestError::Unnamed`, whose payload is an arena key).

**What this does NOT cover.** The three numbers the arm carries —
patch, triangle and the out-of-range index — still reach Python only
through the kernel's `Display`. Filed as
`work/lib/mesh-index-numbers-cross-as-prose-under-a-projecting-door.md`.
No Python test provokes the arm and none can: it reports a corrupt
mesh, so the tag is pinned in Rust and `test_picking.py`'s class
docstring says why there is no row for it there.
