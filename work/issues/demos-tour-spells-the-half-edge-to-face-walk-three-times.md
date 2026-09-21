---
id: demos-tour-spells-the-half-edge-to-face-walk-three-times
kind: issue
title: demos/tour spells the half-edge to face walk three times, twice byte-identically
status: open
opened: 2026-09-19
priority: P4
cost: E
---


## Finding

- **Where**: `demos/tour/src/bodies.rs` (`surface_of`, ~:601),
  `demos/tour/tests/common/rim_select.rs` (`surface_of`, ~:62) — **byte-
  identical closure bodies under one name** — and
  `demos/tour/src/lily.rs` (`face_of`, ~:2853).
- **Importance**: medium — it is a finding about the PUBLIC API, from
  the seat that exists to report those.
- **Confidence**: sure. All three read; `demos/wild`, `benches`,
  `tools/*` and `interval-transcendentals` hold zero (`git grep
  parent_loop` over all of them).
- **Raised by**: PR #2857's fix pass, 2026-09-19. No program claims
  `demos/`, which is why this is in `issues/`.

PR #2857 added `pncad::topo::Body::face_of_half_edge(he) ->
Option<FaceKey>` and made it `pub`. The three sites above already reach
for that walk through the public API and write it out:

```rust
let surface_of = |he| {
    let l = body.get_half_edge(he)?.parent_loop;
    Some(body.get_face(body.get_loop(l)?.face)?.surface)
};
```

`bodies.rs` and `rim_select.rs` are that closure twice, byte for byte,
in a `src/` file and a `tests/common/` file of the same root.
`lily.rs`'s `face_of` is the same walk stopping at the face.

**This is not a conversion request against the demo rule.**
`work/dup/program.md`'s `keep_out` holds that demos are never converted
to reach past the public API — that rule protects the demo from reaching
IN. Here the fold goes the other way: `face_of_half_edge` is `pub`, so
the folded spelling is the natural public-API one and is what a user
would now write. That the demo wrote it out three times, twice
identically, IS the library finding (`memories/demo-purpose.md`): the
door did not exist when the demo was written, and the friction is the
evidence.

## What the instruments could not see — and why this bucket had no row

`demos/tour` was invisible to every census this class has been measured
with, for two independent reasons, and no census in S-DUP has ever had a
`demos` bucket:

- **`cargo check --workspace` does not compile it.** `Cargo.toml`
  `exclude`s it; `scripts/doc-gate.sh --print-roots` names SEVEN roots
  outside the workspace. The type-directed probe that produced this
  class's headline numbers ran `--workspace` only.
- **`bodies.rs` (~:601) is inside `#[cfg(feature = "probe")] fn
  bud_rim`**, so even a probe run in `demos/tour` itself, at default
  features, does not warn on it. Feature-gated code never type-checks,
  so it never warns.

The instrument that reached all three is a text one: a closure-name
census, `let (face_of|surface_of|...) = |` over every tracked file. It
over-fires on any closure named after a topology noun and says nothing
about a walk written inline, but it is text, so no cargo root and no
`cfg` can hide from it.
