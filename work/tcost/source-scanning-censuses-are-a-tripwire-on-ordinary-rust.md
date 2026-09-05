---
id: source-scanning-censuses-are-a-tripwire-on-ordinary-rust
kind: issue
title: the source-scanning censuses hand-parse Rust and fail loud, so an ordinary-but-unusual signature reds another program's test with a byte offset for a message
status: open
opened: 2026-09-04
---


Found by FIX's `no-parametric-loop-constructor` lane (PR 1765) by
tripping it. Filed by the FIX orchestrator; S-TCOST is the natural
claimant (`crates/*/tests/*` is its territory).

## What happened

`crates/geom-core/tests/bounds_census.rs`'s `every_sole_bracket_bound_door_is_in_the_roster`
walks the tree's **source text** and hand-parses signatures. Its
`angle_end` helper closed a generic parameter list at the first `{` or
`;` with no bracket-nesting check, so an ordinary signature —

```rust
pub fn polygon_expr(points: impl IntoIterator<Item = [Expr; 2]>) -> Self
```

— read the `;` inside `[Expr; 2]` as the item's body and panicked:

```
a generic parameter list at byte 56361 does not close before its item's
body: <Item = [Expr; 2]>) -> Self {
```

The lane's repair was right: keep `[Expr; 2]` (it is exactly what
`ProgramStep::At` holds and what `pt_lit` returns — distorting a door's
type to suit a census parser is the wrong direction), and fix the
scanner to read the terminator at square/round-bracket depth zero only,
which is the nesting its own sibling `top_level_params` already
respects for commas.

## Why this is a class and not that one bug

Two censuses hand-parse Rust and both stop the walk **fail-loud by
design**: `bounds_census` and `flagged_census`. The lane checked and
`flagged_census::skip_turbofish` does **not** carry this particular bug
(it counts angle depth alone, with no `{`/`;` break), so that negative
result stands. The structural point survives it:

**A lane writing an ordinary-but-unusual signature can red a census in
another program's territory, with a panic that names a byte offset
rather than the rule it was enforcing.** The lane that trips it has
done nothing wrong, learns nothing from the message, and must repair a
test it does not own — under CI-red pressure, in a file it has never
read. That is the worst combination of circumstances in which to edit a
hand-written parser, and it is where a second bug hides.

Note the shape is not "the censuses are wrong". They enforce real
invariants and failing loud is correct. The defect is that their
failure mode is indistinguishable from a defect in the *code being
scanned*, and their coverage of Rust's grammar is whatever the
signatures in the tree happened to need so far.

## Dispositions worth weighing

1. **Make the panic name the rule.** The cheapest real improvement: a
   message that says "this census hand-parses signatures and could not
   read yours; the census is likely wrong, not your code" would have
   saved the whole diagnosis. It does not fix the parser and does not
   need to.
2. **Fence the scanners' grammar explicitly** — state at each scanner
   what it does and does not parse, so the next lane can tell in one
   read whether it is in scope.
3. **Stop hand-parsing.** The heavy option; only worth it if the
   censuses grow.

(1) is almost free and closes most of the cost. Not decided here.

## Home

`work/issues/` — the sites are `crates/geom-core/tests/` and
`crates/*/tests/`, S-TCOST's territory. Re-home by header edit.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/tcost/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.
