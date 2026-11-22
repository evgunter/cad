---
id: an-unfinished-leaf-names-the-copy-that-was-not-turned
kind: issue
title: A union's UnfinishedLeaf names the copy that was not turned: the member carried through the boxed turn is the one read in the first member's frame
status: closed
opened: 2026-11-14
priority: P2
cost: E
closed: 2026-11-16
pr: 7019
branch: onehome/unfinished-leaf-frame
---


Found by the review of BOXFINISH's PR 6959 (2026-11-14), on that PR's
head with the box-scalar switch on; the fixture is
`crates/editor-core/tests/review_chart_ends_turned_disc.rs` on
`boxfinish/review-chart-ends`, taken into the suite by that PR's fix
pass, where it pins this behaviour and cites this row.

A disc turned about an axis by 30° ± 1°, united with a second copy
standing clear, at `Sym<Interval>`: the union refuses
`UnfinishedLeaf { input: <the clear copy>, findings:
[SurfaceDatumUndecided { kind: Cylinder, datum: Axis, … }] }`. The
union is built in its first member's frame, so the clear copy is the
one carried through the boxed turn, and it is the one whose taken image
does not finish. The person is told that the copy they did not turn
"does not finish where this operation takes it".

Likely a wording or attribution defect in what `UnfinishedLeaf` names,
not a kernel one. The refusal itself is
`check-1-reads-a-turning-frames-length-off-its-bracket`.

## Closed (2026-11-16): the refusal names the frame beside the member

The attribution was true of the data: a union's fold computes in one
member's frame (`fold_members`, `crates/editor-core/src/eval/wire.rs`) and
takes every other member's image there (`MemberRead::taken_in`), so the
image that does not finish is the clear copy's. What the sentence lacked
was the frame. `NodeErrorKind::UnfinishedLeaf` now carries `into`, the
member whose frame the read moved `input` into (`None` where it was taken
in its own frame), and reads "<input>, read into <into>'s frame, does not
finish there: …". The subject is still the member that failed
([topology.tiers], "A door names only its subject"); the frame is context.
The fixture pins both nodes.

Computing the fold in a frame no boxed member gives is not this row's: it
belongs to the open `[ev]` PR 7006 (fork 274), where a move becomes a
placement and the fold reads each member in its own frame.


## Since placed members (fork 288, stage 1)

The fold no longer takes a member's image: it finishes each member in its own frame and places it, and the n-ary door moves it (`topo::nary_placed`). The clear copy's cylinder decides in its own frame, so its take no longer refuses. The union now refuses in the Boolean, `ShellWitnessExhausted` at `line_quadric_cylinder_disc`, on the copy's widened data. The fixture is renamed `crates/editor-core/tests/a_turned_discs_union_refuses_on_the_widened_copy.rs` and pins that site. That the refusal no longer names the member is `work/partframes/a-boolean-refusing-on-a-boxed-members-widened-data-does-not-name-the-member.md`.
