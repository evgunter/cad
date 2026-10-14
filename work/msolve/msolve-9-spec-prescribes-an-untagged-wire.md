---
id: msolve-9-spec-prescribes-an-untagged-wire
kind: issue
title: MSOLVE-9's spec prescribes an untagged MateFrame wire, citing a LoggedEdit shape that no longer exists
status: closed
closed: 2026-09-24
priority: P1
cost: E
opened: 2026-09-23
rides_with: MSOLVE-9
---


`docs/MSOLVE-9-SPEC.md:70-74` prescribes the new `MateFrame` wire as
*"untagged over the two arms, each inner struct `deny_unknown_fields`
(the shape `LoggedEdit` established), so every existing document reads
as `Authored` unchanged"*. Two things under that sentence moved:

- **The precedent is gone.** PR 3123 (PORT) retired `LoggedEdit`'s
  two-shape untagged wire: an entry is now one struct,
  `{ "edit": …, "maintenance": […] }`, with no fallback, and
  the pre-rows bare shape refuses `Unreadable`. Ev's ruling there:
  backward compatibility with older logs is a red flag, and churn or a
  format change never outweighs a better final state.
- **The attribute is refused.** PR 2702 (PORT-DIMS-1) adds
  `scripts/gates/persist-no-backtracking.sh`, which fails CI on any
  `#[serde(untagged)]` under `crates/editor-core/src`. The load door's
  structured `DimensionError` rides a first-refusal-wins premise
  (`crates/editor-core/src/persist/refusal.rs`), and an untagged enum
  records a refusal on each failed arm and then discards it.

So the unit as specified would go red at the gate. The design it
wants — an existing file reading as `Authored` without a re-save — is
the compatibility PR 3123 just removed. An externally tagged
`MateFrame` (`{"Authored": {…}}` / `{"FromFace": {…}}`) with the
checked-in corpus regenerated is the shape that fits both, and C5's
row would move by the wrapping, as the LoggedEdit change moved
`golden.cad` and `die_composed_tour.pncad`. Whoever dispatches
MSOLVE-9 should amend the spec's wire paragraph before the lane reads
it; it is MSOLVE's spec, so this row does not edit it.

## Closed

Closed by MSOLVE-9 (PR 2934) under the spec's amendment (orchestrator,
2026-09-24, at the end of `docs/MSOLVE-9-SPEC.md`). `MateFrame`
(`crates/editor-core/src/mate.rs`) carries no serde attribute of its
own: the wire is serde's external tag, `{"Authored": {…}}` /
`{"FromFace": {…}}`, each inner struct `deny_unknown_fields`, and no
reader accepts the bare-vector frame. PR 2702's
`scripts/gates/persist-no-backtracking.sh`, run against this tree from
that branch, is green, and red on the untagged declaration it replaced.

Rows, in `crates/editor-core/tests/msolve9_from_face.rs`:
`both_arms_round_trip_and_a_stray_key_on_either_refuses` (a stray key
inside either arm refuses `Unreadable`, one beside the tag `Parse`),
`an_untagged_frame_refuses_whichever_arms_keys_it_carries` (either
arm's keys written with no tag refuse `Unreadable`),
`c5_every_tracked_document_loads_on_the_tagged_wire_and_re_saves_identically`
(every alignment the tracked corpus spells carries its tag; every file
loads and re-saves byte for byte), and
`a1_the_mate_follows_the_edited_face` (the saved face side is
`{"FromFace": {"face": …}}` with no vectors, the authored side
`{"Authored": {…}}`).

The corpus: no tracked document carries a mate (a `git grep` for
`"Mate"` and `"alignment"` over every tracked non-source file, the
`.gz` files decompressed, finds none), so nothing regenerated moved;
the two blessers (`PNCAD_BLESS=1` `lib_dietool_crossing`,
`M4_PR6_BLESS_GOLDEN=1` `m4_pr6_golden`) rewrote their files to the
bytes already committed.
