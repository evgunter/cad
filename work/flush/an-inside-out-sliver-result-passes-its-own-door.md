---
id: an-inside-out-sliver-result-passes-its-own-door
kind: issue
title: An inside-out one-shell sliver result passes its own door and is caught only as the next op's InsideOutOperand, which blames the operand
status: closed
opened: 2026-10-12
priority: P2
cost: M
closed: 2026-10-14
---


Found by PR 4783's review (FLUSH, `flush/thinness-by-pairs`).

Check 7 passes a solid whose `V/A` is certified in band without
reading its sign (`validate.rs` `plus_v_at_target`, `PlusVVerdict::Sliver`),
so an inside-out sliver finishes at rest. Before PR 4783 the boolean
door refused every such one-shell result (`Escalated { ShellRole }`,
PR 4636's gate), whatever its orientation. With that gate retired
(Ev's pairs-only ruling, PR 4690), a one-shell sliver result reaches
the caller with its orientation unread at its own door.

The orientation is read only when the body is next used as an operand
(`boolean/reduce.rs` `gate_sliver_orientation`, over
`AtRestBody::slivers`), and an inverted one refuses there as
`InsideOutOperand` with `SliverInsideOut`. That blames the operand for
what was the previous op's result, which is a kernel defect, and the
caller gets no refusal from the op that made it.

No construction found so far yields an inverted sliver result: the
boolean builds orientation from its operands. This row asks whether the
door should read the result's slivers with the same probe
(`gate_sliver_orientation` over the result's `slivers()`) and refuse
`ResultInvalid { SliverInsideOut }` there, which is the kernel's own
defect class (`ops.rs` `finding_arm`).

Not measured here: whether any op inverts a sliver result. A probe would
run `gate_sliver_orientation` over every result whose `slivers()` is
non-empty across the batteries (R1's `review-sliver-probe` feature on
`flush/thinness-by-pairs-review` @ a79ba445 logs those results).

## Closed (FLUSH `flush/shell-role-reader`, 2026-10-14)

Check 7 reads a solid's orientation with the reader of a shell's role
(`validate.rs` `plus_v_errors`, `boolean/solid_contain.rs`
`side_at_infinity`): a ray aimed at one of its faces from beyond it,
the volume only where no face offers an aim that reads. A sliver's
volume no longer exempts it, so an inside-out sliver result refuses at
its own door as `ResultInvalid { NegativeVolume }`, the kernel's defect
class. `AtRestBody::slivers` and the operand gate that read it
(`gate_sliver_orientation`) are retired. Pinned on the at-rest gate:
`crates/topo/tests/shell_role_reader.rs`
`a_lone_slab_is_outer_and_inside_out_refuses_at_rest` and
`a_slab_in_the_plane_x_equals_y_reads_its_role`.

The reader found one such body main ships: a fin ∪ a pyramid across its
fold leaves a region winding −1
(`a-fin-crown-at-rest-holds-a-region-winding-minus-one-that-check-7-and-the-census-pass`).
