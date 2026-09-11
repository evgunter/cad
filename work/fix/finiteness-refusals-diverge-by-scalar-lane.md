---
id: finiteness-refusals-diverge-by-scalar-lane
kind: issue
title: one document answers UnderflowedDirection at f64 and DegenerateDirection at Interval — the point-scalar posture's first visible cost
status: open
opened: 2026-09-11
---


(FIX orchestrator) Disclosed by the `direction-underflow-reports-zero-length`
lane (PR 2359), which pinned the divergence in two rows rather than
leaving it for someone to discover.

## The fact

The same document, the same node, the same authored direction
`[1e-180, 0, 0]`:

- at `f64` → `NodeErrorKind::UnderflowedDirection`
- at `Interval` → `NodeErrorKind::DegenerateDirection`

Both refuse, so nothing silent is minted and no geometry is wrong. But
a user who switches lanes to get a certified answer gets a **different
refusal for the same input**, and the two name different causes.

## Why it happens, and why it is not obviously wrong

`1e-180` squares to `[0, 1e-323]`, so the norm encloses `[0, 3.1e-162]`
— an enclosure that straddles zero but sits wholly inside the band, so
`decide` answers `Zero` **definitely** and never reaches the underflow
question. The gate no-ops there by construction:
`is_underflowed_length` asks `witness / len`, which for an enclosure
containing zero is an unbounded enclosure rather than poison.

That is the **ratified point-scalar posture** working as written. PR
2356's correctness review established the same thing at the overflow
end — every finiteness gate is a point-scalar gate, and
`Interval::is_poison` being `is_nai() || is_empty()` means an
enclosure that has lost all information still answers "finite".
`crates/geom-core/src/real.rs` states this honestly.

## What this row is for

**Not to reverse the posture** — that is a design question and not this
program's to take. It is to record that the posture now has a
**user-visible consequence** rather than only a documented scope, and
to put the two instances in one place:

1. the overflow end, where the five doors' gates are no-ops at
   `Interval` (PR 2356, n2 of its correctness review);
2. this end, where the refusal's *identity* differs by lane.

The second is the sharper one. A no-op gate means the enclosure lane
decides against the band as it always did — defensible, and invisible.
A divergent refusal identity is something a user can see and cannot
explain.

**What is owed first is a decision, not a diff**: is a refusal's
identity allowed to be lane-dependent? If yes, it should be written
down where a reader of either arm will find it, because two arms
currently describe one input. If no, the repair is at the enclosure
lane and is `geom-core`'s, not a door's.

PR 2359's two rows assert the current behaviour, so whichever way the
question is answered, they say exactly where the answer lands.
