---
id: census-cyl-sheet-b-keeps-the-unrepaired-radial-read
kind: issue
title: cyl_sheet_b's descending rim still projects for its radial direction, the read its siblings were repaired away from
status: closed
opened: 2026-09-19
priority: P3
cost: E
closed: 2026-09-20
branch: dup/src-cyl-sheet
parent: topo-src-cyl-sheet-is-one-construction-twice-and-not-the-tests-one
pr: 2925
---


## Finding

`crates/topo/src/census.rs`'s `cyl_sheet_b` builds its descending rim's
carrier `u_ref` by **projecting a chart point back onto the plane
through the axis**:

```
let s = at(u1, v) - center - axis * ((at(u1, v) - center).dot(axis));
```

Every sibling spelling of that rim was repaired away from exactly this
read. The `tests/` family carried the repair with a comment saying why
— *"the projection-based read cancels catastrophically for
tilted/small frames and mints a non-structural chart image"* — and the
shared door `cyl_wall_sheet`
(`crates/topo/src/test_support_fixtures.rs`) now states it as
`CylFrame::radial`, reading the direction from the frame's own fields.
`cyl_sheet_b`'s frame is tilted in azimuth (`u_ref` at 0.7 rad) and
axis-reversed, which is the family of frames the repair was for.

## Status

**Not measured to fail.** `cyl_sheet_b` runs at radius 1 and a
half-radian window, where the cancellation is not fatal, and its rows
pass. What this row records is that one member of a repaired class
kept the pre-repair read — drift between copies, which is this
program's charter — and that nothing in the tree says which read is
authoritative.

## What a unit here owes

Re-take the read at the merge base, then either point `cyl_sheet_b` at
`CylFrame::radial` or state why the projection is right there. If the
repair matters, a row at a tilt where the two reads disagree is the
deliverable, not the edit.

## Why this row is not on `curved`'s slate

`crates/topo/src/census.rs` is `curved`'s territory. The finding is
drift between duplicate fixture builders, so it is filed with the rest
of that class on this program's slate; `curved` owns the file whenever
it wants the row.

## Closed

Reconciled here, and the routing question the row asked is answered by
the measurement rather than by judgement.

**The two reads agree bit for bit at this fixture's constants.**
Re-taken 2026-09-20 at merge base `cd9fdfd6b`: `cyl_sheet_b`'s body was
reproduced verbatim in a throwaway in-crate probe and dumped against
the same sheet built through the shared door with `CylFrame::opposed(0.7)`
and `CylFrame::radial(u1)` — the repaired read — at the window
`cross_description_pair(0.5, 1.3, 0.3, 0.7)` its only call site uses.
Every vertex, edge, half-edge, loop, face, shell, solid, point (by bit
pattern), curve and surface row: **empty diff**, 33 lines.

So folding it changes nothing a census row can measure, and the
question of whether to route it away does not arise. `cyl_sheet_b` is
now nine lines over `cyl_wall_sheet_keyed`, and the projection is gone.
The census rows that read it are green and their arena is unchanged.

**What this does NOT settle, and where it goes.** The empty diff is at
ONE frame and ONE window. It says the projection and the frame read
agree where this fixture stands; it does not say the repair was
unnecessary, and the repair's own justification — that the projection
cancels catastrophically for a tilted or small frame — is about frames
this fixture does not visit. The row asked for *"a row at a tilt where
the two reads disagree"* as the real deliverable; that row is a
coverage row over `CylFrame::radial`, not a duplication one, and it is
S-TINT's to open against the door that now states the read. This unit
did not open it, because it has no measurement of where the two
diverge and method item 8 says a disclosed blind spot is an instruction
to measure, not a licence to file a guess.

## Fix pass, 2026-09-20

**"ONE frame and ONE window" undercounted the windows.**
`cross_description_pair` has two call sites, `census.rs`'s
`a_cross_description_cylinder_patch_record_certifies` (window
`0.5..1.3 x 0.3..0.7`) and
`a_refuted_cross_description_cylinder_record_is_stale_typed`
(`0.5..1.3 x 2.0..2.5`). The first cut dumped only the first. The
reviewer checked the second; this lane then re-took both itself,
building `cyl_sheet_b` verbatim from merge base `cd9fdfd6b` against
the shipped `cyl_wall_sheet_keyed`: **33 dump lines each, both diffs
empty**. The conclusion stands. The count under it was wrong: a row
closing on "one window" having measured one of two, in the program
whose subject is counts that do not survive re-taking.

**Owning the question: a doc line, not a row.** Not guessing a
threshold was right. Leaving the question unowned was not: this row is
closed and goes with the program's directory when it closes, the projection read is now
gone from the tree, and what remained was `CylFrame::radial`'s rustdoc
asserting that the projection *"cancels catastrophically for a tilted
frame or a small radius"* with nothing under it. The fix pass first
filed that as an S-TINT row. Under the tracker's since-ratified rule
that a small thing fixable where you stand is fixed, not filed
(`work/README.md`, "The tracker is not comprehensive"), it is instead
**a doc edit in this PR**: the rustdoc now states only what its own
code holds — the frame read combines two orthogonal unit vectors,
where the projection subtracts an axial component and carries that
rounding — and makes no claim about when the projection fails. Nothing
unmeasured is asserted, so nothing is left to own.
