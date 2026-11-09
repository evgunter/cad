---
id: a-combine-of-a-world-copied-familys-member-leaves-the-copy-drawing-it
kind: issue
title: A combine of a family member picked through the family's world copy leaves the copy drawing that member, and lands the result outside the world
status: open
opened: 2026-11-09
priority: P2
cost: M
design: true
---



Found by the REFS unit `viewer-boolean-tool-cannot-pick-a-read`. The
boolean tool's seats now read a viewport pick on a family's drawn member
as that member (`Port::at_seat`, `crates/viewer/src/seats.rs`;
`world::seat_member`), and a pick through a copy reads past it to the
source, the seat rule (`world::seat_read`). So for a body placed into
the world and then patterned (the copy re-pointed to the family, as
`combine_ops::a_pattern_takes_its_body_over_and_a_member_read_places_nothing`
sets up), a pick on member 1 in the world authors `pattern[1] ∪ other`.

The combine takes nothing over for a member read
(`DocSession::combine_in_world`, `crates/viewer/src/session.rs`): the
takeover vocabulary re-points a reader of a VARIABLE (`Takeover::of_var`),
and the world copy reads the family whole, so re-pointing it to the
result would drop every other member from the world. What lands, read
off `world::Picture::of` after the commit (pinned in part by
`combine_ops::a_member_picked_through_a_world_copy_reads_the_family_and_takes_nothing_over`):
the world copy still covers every member (`Covers::AllBut({})`), so
member 1 is still drawn in the world, and the union is drawn in the
family's own space (`in_one_space` homes it on the first operand), not
in the world; the world's product volume is still the two members.

The same pick on a family nothing copies is right
(`combine_ops::a_boolean_reads_the_pattern_member_picked_in_the_viewport`):
the family is drawn in its own space, the read takes member 1 out of it,
and the result stands there.

**Decide when dispatched:** what a combine of one member does to a copy
of its family — re-point the copy to a `⊔` of the other members and the
result; or read the copy's own member (`copy[i]`, in the world) instead
of the source's, which breaks the seat rule that a pick on a copy is a
pick of the body it places; or something else. It is a takeover ruling
(`work/doors/`), which is why it is not decided in the REFS unit.
