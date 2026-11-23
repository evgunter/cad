---
id: a-hop-whose-chord-is-proved-zero-is-refused-by-the-junction-gate-not-by-name
kind: issue
title: A hop whose chord is proved zero is refused by the junction gate on a quotient that is no direction, not by name: the mint has the proof in hand and no refusal to give
status: open
opened: 2026-11-23
---

Found by the review of BOXFINISH's expression-question unit (PR 7488).

The profile's direction mint (`crates/profile/src/seg.rs` `axis_or`)
asks each component of a chord whether its expression is zero
(`geom_core::Expression::proves_zero`). Where BOTH are proved, or one
is proved and the other reads Zero, the chord has no length; the mint
gives it no axis and hands back the quotient, which is no direction
there. What refuses the document is whatever reads that quotient next:

| lane | the answer |
|---|---|
| `f64`, `Interval` at a point, `Sym<f64>` | `PathError::JunctionTurnWithinTolerance` from the verb's junction gate |
| `Sym<Interval>` over a box, both proved or the other the point 0 | `Escalated { gate: PathJunctionTurn }` |
| `Sym<Interval>`, the other component straddling 0 | `Escalated { gate: SegmentDirectionAxis }` |

Pinned by `a_chord_with_no_length_is_refused_in_every_lane`
(`crates/profile/tests/recorded_directions.rs`). None of these names
the fact: a hop from a point to itself. Over a box it is an escalation
(a cue to bisect) where the answer is definite at every valuation: the
chord is zero as an expression.

**Why it is not fixed where it was found.** The mint returns a
direction or `AxisUnsigned`; a refusal by name needs a `PathError`
variant for a zero hop and a decision about which gate owns "this hop
has no length" (the verbs read length at `vertex_separation` in
validation, after the junction gate has read the direction). That is
the profile's gate order, not the mint's.

**What would close it.** `line_to` and its siblings refuse a hop whose
chord is proved zero in both components by name, before the direction
is minted or read; over a box that is a definite refusal, not an
escalation.
