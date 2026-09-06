---
id: overrun-attribution-picks-the-first-candidate
kind: issue
title: The anchor-fit refusal reports the FIRST corner-side candidate, not the one it is metered against
status: open
opened: 2026-09-05
---


Found by the FILLET-ATTR dual review (PR 1895) and measured by one arm.
FILLET-ATTR fixed the attribution defect one level UP — which CORNER of a
carrier pair a refusal is about — and this is the same defect one level DOWN,
between the candidate circles at ONE corner. It is out of that unit's fence
(the envelope's entries are corners, not candidates) and is filed here rather
than left in a PR body.

## The mechanic

`crates/profile/src/sugar.rs:612` — inside `arc_fillet_trims`, the loop over
the corner's candidate centres keeps the FIRST candidate whose trim overruns a
leg:

```rust
} else if corner_side && overrun.is_none() {
```

and `crates/profile/src/sugar.rs:~639` reports it when no candidate survives:

```rust
return Err(overrun.unwrap_or(ArcTrimRefusal::NoCorner { ... }));
```

That refusal becomes `CornerReason::AnchorOutsideTrimmedExtent { side, carrier,
setback, available }` — the numbers an author reads, and the numbers the
"reduce the radius or move the anchor" recourse is metered against.

## The measurement (reviewer, on FILLET-ATTR's grid A)

Instrumenting the discard arm over grid A (arc x arc, 18 144 authorings):

- **232 discarded candidates**, i.e. 232 corners where a SECOND corner-side
  candidate also overran and was dropped;
- **all 232** had the kept candidate's refusal surface as the reported one;
- **all 232** carried payload numbers differing from the kept candidate's —
  e.g. a kept setback of 0.259 m against a discarded 0.609 m at the same
  corner.

So on those corners the sentence's setback, and the recourse the author is
asked to follow, are metered against a candidate chosen by enumeration order.

## Why the site's own comment does not cover it

The comment above the arm argues that attribution is sound because
`corner_side` is a real test, so the candidate rounding the OTHER intersection
of the carriers never reaches the arm. That is true and is about the CORNER;
it says nothing about which of several corner-side candidates at the SAME
corner is reported, which is what the measurement moved.

## What would close it

The same shape of answer FILLET-ATTR's ruling gave one level up, or a stated
rule for the pick. Not decided here.

## Home

`work/fillet/` — `crates/profile/src/sugar.rs`'s candidate machinery is the
FILLET program's ground, and the adjacent corner-level rule is this program's
own (issue 1281, ruled on PR 1734).
