---
id: a-boolean-refusing-on-a-boxed-members-widened-data-does-not-name-the-member
kind: issue
title: A Boolean refusing on a boxed member's widened data does not name the member
status: open
opened: 2026-11-19
priority: P2
cost: M
refs: [a-body-records-its-parts-in-their-frames, an-unfinished-leaf-names-the-copy-that-was-not-turned]
---



Since stage 1 of fork 288 (`onehome/placed-boolean-doors`), a union's fold finishes each member in its own frame and hands it to the n-ary door placed (`topo::nary_placed`, `crates/topo/src/boolean/nary.rs`). The door moves each member not in the computing frame, carrying its verdicts: every finding family but the session box is a fact of the shape, which a rigid motion keeps (`FindingFamily::under_isometry`, `crates/topo/src/finish.rs`).

Under a boxed motion the image's data is wider than the source's. A decision the source makes can then fail on the image inside the Boolean. The person is told of a shell-witness exhaustion, and not which member could not be read.

`crates/editor-core/tests/a_turned_discs_union_refuses_on_the_widened_copy.rs` is the case. A disc is turned 30° ± 1° about `y`, and an unturned copy stands clear of it. At `Interval` the union refuses `ShellWitnessExhausted` at `line_quadric_cylinder_disc`, on the clear copy carried through the turn.
- Before placed members, the clear copy's image was finished in the turned frame. Its cylinder's datum, read through the turn, refused at the member's take as `UnfinishedLeaf { input: clear, into: Some(turned) }`, which named both members.
- The subtract already took its operands this way on main (`wire_subtract` finishes `from` in its own frame, then moves it), so its refusals behave the same.

**Where it goes:** stage 2 (`work/partframes/a-body-records-its-parts-in-their-frames.md`). There the Boolean reads each member through the relative motion at the reading, not on a moved copy, so the widening that causes this refusal goes away. Until then, a refusal raised on a moved member's data could say which member it was carried through, and which turn.

**What is named and what is not.** Where the placed door's move itself refuses (an image past the session box), the refusal names the operand.
- `topo::PlacedBooleanError::Placed { operand }` and `verbs::VerbError::Placed { operand }` carry the side, and `topo::nary_placed`'s refusal carries the member's place.
- The fold and the subtract raise it as `NodeErrorKind::UnfinishedLeaf { input, into }`: the operand moved, and the one whose frame it was moved into.
- A refusal the Boolean raises afterwards, inside its own decisions on the moved copy's widened data, is the one this row is about. It names neither.

## Moved to PARTFRAMES (2026-11-21)

Moved from `work/onehome/` by `git mv`, id and body unchanged (Ev, PR 7282: stage 2 split off): its body names stage 2 as where it goes.
