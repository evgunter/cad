---
id: a-mate-reads-only-geometry-and-a-value-charts-on-the-target-space
kind: unit
title: D′2: WorldRead retires into values; a value names no mate and charts on the target space's axes; a point mate's spin chain; the placement helpers become one pure constructor; inline is one rule; the flatten edit
status: spec
opened: 2026-10-22
priority: P0
cost: H
parent: a-placement-pins-by-written-values-without-a-mate
---



## What

INTENT stage 3, D′2. Spec: `docs/INTENT-STAGE3-SPEC.md` §5′.2,
which governs where this body and it differ. It builds PR 5494's
`[placement.mates]` and `[placement.values]` (rows 190 and 191),
the helpers ruling (`work/intent/log.md`, 2026-10-20) and ASSEMBLY
A4's inline:

- No space has roles: `Target::World` and `WorldRead` retire, and
  each world mate's generator writes values on the world's axes.
- `Value { coord, by }` names no mate; `Coord` is
  `Slide(X|Y|Z) | Spin(X|Y|Z)`. The chart is the target space's
  axes, swung onto the one mated direction where a mate fixes one
  (`chart_body` retires).
- A lone point mate's rotation is a spin chain about the target
  space's axes through the mated point; spins are counted by axis.
- `mate::pin` keeps one pure constructor from the caller's mates and
  moves; `place_at`, `pin_at`, `frame_at`, `place_against`,
  `Anchors` and their heuristics retire; the place-part gesture
  writes the identity bundle.
- Inline mints a `new_space()`, copies the part verbatim, and places
  it under the instance's bundle; `refactor::flatten` is the edit
  that flattens an identity placement. The viewer's mate tool
  removes the values a new mate fixes.

Waits on D′1, because the chart reads the named target.

## Done when

The spec's §5′.2 rows, and tests 20, 21 and 24. Geometry moves only
where a mated target body stands off its space's identity or a
helper's rounded turn is replaced by the turn its caller meant, each
reported by document and node.
`p2_face::a_valued_mate_reading_a_mate_placed_copy_refuses_to_cross_split`
crosses split once a value charts on the target space: today it pins
split refusing the reader `Posed`, because the value is charted on the
mate's target body's frame.

**Absorbs:**
`a-value-charted-on-the-target-reads-frame-carries-the-datum-chain`,
`sym-solve-atan2-has-no-branch-rule-about-pi`,
`a-slide-value-doubles-its-translations-width`,
`an-inline-at-a-moved-landing-reading-a-variable-refuses`,
`inline-refuses-a-placed-part-off-the-identity`,
`the-viewer-cannot-mate-two-freshly-inserted-parts`.

## Handoff

2026-10-23, second implementer lane (the first stopped at `31b979f5`
with no PR and no CI run). State found at `31b979f5`:

- Builds and lints clean: `cargo clippy --workspace --all-targets
  --all-features -D warnings`, and the same in `demos/tour`,
  `demos/wild`, `tools/*` and `benches`. The "viewer and demos
  migration (in progress)" commit was finished by `8f83d72e`.
- The Done-when grep names no live symbol (its hits are topo's
  `pair_section_frame_at`, geom-brep's private `place_at` and
  comment prose).
- Tests in place: 24 and 21 and the flatten rows in
  `intent_dprime2_mates_read_geometry.rs`; 20 in
  `asm_r2a_mate_solve`; inline-then-flatten of a split in `p2_split`
  and `asm4_split_inline`; the `p2_face` valued-mate row renamed to
  `..._crosses_split`; the gate
  `scripts/gates/bundle-constructor-reads-no-evaluation.sh`.
- Absorbed items: five closed by the first lane; this lane closed
  `the-viewer-cannot-mate-two-freshly-inserted-parts` (the door's
  rebundle drops the values a mate fixes).
- Left: no CI has run; the absorbed items' `PR PRNUM` placeholders;
  the PR body (geometry moves by document and node, goldens, sweep
  hit lists); a `[render]` commit.
