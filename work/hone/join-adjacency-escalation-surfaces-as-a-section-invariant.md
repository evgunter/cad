---
id: join-adjacency-escalation-surfaces-as-a-section-invariant
kind: issue
title: An in-band adjacency-skip verdict surfaces as SectionInvariant, an invariant-break refusal, instead of an escalation carrying its margin
status: open
opened: 2026-10-01
cost: E
priority: P3
---

(REACH, PR 3627: found pinning the planar-side line arm's in-band row.)

**Re-pointed (JOIN-1, PR 3790).** The boolean planar-side arms
(`bool_between_line_on_wall`, `bool_between_arc_window`) and the witness
row named below are deleted: the boolean lanes' skip reads the segment's
locus and never asks this function. What remains is the split lane's
`split_conic_inplane_mid` arm, whose in-band verdict still drops its
`Indeterminate` and surfaces through `skip_adjacent_chord` as
`SectionInvariant`; the fix below applies to it alone. A witness row
must now be built on the split lane (a conic between edge whose
midpoint sits in band off the section plane).

## What

`chord_join.rs` `between_edge_is_section` (~1694) returns `None` for an
in-band verdict on every arm (`bool_between_line_on_wall`,
`split_conic_inplane_mid`, `bool_between_arc_window`), and drops the
`Indeterminate` it was given (`Err(_) => Ok(None)`). Its one caller,
`skip_adjacent_chord` (~1606), maps that `None` to
`SplitJoinError::SectionInvariant { what: "section classification of the
join-adjacent edge escalated" }`, whose Display reads "curved-section
invariant at face …": an invariant-break refusal with no margin, no lever
and no tolerance offered.

An in-band reading is not an invariant break. The witness is
`chord_join::tests::bool_planar_lane_reads_a_line_on_the_wall_by_its_midpoint`'s
third row: a chord of half-angle `1e-4` rad across a unit wall, whose
sagitta `≈ 5e-9` m lies in the band at witness tolerance. Through an op
it would refuse as an invariant break rather than as the coincidence it is.

## The fix

Carry the `Indeterminate` out of `between_edge_is_section` and refuse
as an escalation with its margin (the boolean's `Escalated` funnel, a
decision naming the join's adjacency question), keeping
`SectionInvariant` for the structural breaks it already names. The
arc-window arm's decided `Zero` (a graze, ~1791) is a separate case: it
is band-decided, and should refuse with its decided margin, not as an
invariant.

