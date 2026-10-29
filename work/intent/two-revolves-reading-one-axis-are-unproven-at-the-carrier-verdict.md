---
id: two-revolves-reading-one-axis-are-unproven-at-the-carrier-verdict
kind: issue
title: Two revolves whose axis slots read the same two variables glue their bores and walls unproven: the replay's carrier verdict finds no identity
status: closed
opened: 2026-10-29
priority: P2
cost: M
refs: [tangent-and-coaxial-scenes-are-in-the-corpus, a-coaxiality-is-one-axis-read-twice]
closed: 2026-10-29
---

Measured by INTENT stage 6 PR A on the corpus document
`revolves_on_one_axis` (`crates/editor-core/tests/corpus/revolves_on_one_axis.rs`),
pinned by `coincidence_door::two_revolves_reading_one_axis_glue_unproven`.

Two rings, each a rectangle revolved a full turn, both revolves'
`axis_through` slots reading the document variables `ax` and `ay` and
the same constant heading; the second stands on the first in its space
(the identity bundle) and the two are unioned. Every row is unproven
with recourse `OneConstruction` (`Undischarged::Otherwise`: the form is
over the symbols, read whole, and not zero, or not a Zero):

| row | relation, site | proof |
|---|---|---|
| the two bores, the two walls | `SameOriented`, `CarrierLadder` | unproven/`Numeric`/`OneConstruction` |
| the second's base on the first's top | `SameOpposite`, `PlaneLadder` | unproven/`Numeric`/`OneConstruction` |
| two rim vertices | `OnCarrier`, `VertexFusion` | unproven/`Numeric`/`OneConstruction` |

The two bores are revolutions of the same radius about one line both
revolves compute from the same symbols, so rung two's carrier verdict
on the symbolic carriers would be expected to be the zero polynomial.
Beside it, `axis_projection` (one revolve and a copy of it on its own
axis output) proves its two bores by `CarrierVerdict`. Not measured
here: which of the verdict's comparisons is not identically zero (the
radii are literal profile coordinates less `ax`, which may be the
cancellation the form does not see).

**Done when** the cause is measured and the bores' rows prove, or the
reason they cannot is stated and pinned; stage 6 PR B reads this beside
its own rung.

**Closed: the cause is the radii, and the rows are rightly unproven** (measured by INTENT stage 6 PR B). Each radius is a literal profile coordinate less `ax`. Every literal coordinate is its own free variable, so its own symbol in the replay, and within one wall the start and end coordinates are two symbols as well. The axis is not the cause: PR B's two-cylinder reading (`coincide/replay.rs` `one_cylinder`) finds the two revolves' axes one line in the symbols (two `axis` outputs, so no mate states it). It leaves each bore and wall row at its radius (`Axes { left: Radius }`), and the finding says the radii are equal only at the current values. Pinned in `coincidence_door::two_revolves_reading_one_axis_glue_unproven`. Reading one `Length` for both radii, at both ends of each wall, would prove the rows. A radius read from one end of a wall alone is the kernel's wall classification, `work/coincide/a-revolves-wall-classification-is-no-door-row.md`.
