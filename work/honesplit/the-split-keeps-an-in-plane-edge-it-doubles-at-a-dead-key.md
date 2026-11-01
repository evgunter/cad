---
id: the-split-keeps-an-in-plane-edge-it-doubles-at-a-dead-key
kind: issue
title: The split's lineage states an in-plane edge it turns into two coincident edges as kept under its own key, which neither side holds
status: open
opened: 2026-10-19
priority: P3
cost: M
design: true
---


Found by `cleave/keel-slit-ring`, which builds the split's zero-area
ring (`work/cleave/a-keel-in-the-plane-wrapped-round-at-both-ends-refuses-a-degenerate-section.md`).

## What

A split along an edge of its operand lying in the plane, where the
other side's material wraps round one end of it (a slit, PR 4622) or
every end (a zero-area ring), kills the edge on the side that held it
and gives the other side two coincident edges over it, each holding the
whole line. `SplitResult::lineage` (`crates/topo/src/splitting/finish.rs`)
states no fate for the edge, so `Lineage::fate` reads it as kept under
its own key, which neither side holds. Measured at the default ε on the
box less the V-trench of
`crates/topo/tests/a_keel_wrapped_at_both_ends_splits_as_a_ring.rs`,
both the ring (`x ∈ [1.5, 2.5]`) and the one-end slit (`x ∈ [1.5, 4.5]`):
the keel's fate is `Kept { at: the keel, copies: [] }`, live on neither
side, and `SplitResult::carry_contacts` of a record citing the keel
refuses `CarryRefusal::Unmapped` on it. The refusal is loud; no wrong
landing was seen.

The naming layer is not affected: each of the two edges is a section
chord of the wall it bounds, and takes its own unique name
(`crates/editor-core/tests/a_keel_ring_names_its_two_edges.rs`).

## Owed

A fate for the doubled edge. Neither `Divided` (the two edges do not
partition it) nor `Substituted` (one cell) says it; `Kept { at, copies }`
with the two edges, as a vertex the split copies is kept, comes closest,
and the strut side states nothing. The join knows the pairing where it
reads the trace (`Sweep::section_kind`, `Sweep::strip_spurs`).

## Slate triage (2026-10-26, CLEAVE)

- Moved to HONESPLIT (`work/honesplit/`) by `git mv`, id and body unchanged: whose charter is split and carve bookkeeping, unproven walks and the reads nothing reaches. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
