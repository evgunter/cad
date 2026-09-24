---
id: a-slot-edit-through-a-zero-fit-renumbers-a-loops-live-names
kind: issue
title: A slot edit through a Zero fit renumbers a loop's live names and reports nothing
status: open
needs_ev: true
priority: P0
cost: H
opened: 2026-09-20
refs: [a-committed-profile-program-has-no-whole-program-edit, stranded-names-are-retired-to-an-undrawable-coordinate]
---

## The finding

`crates/editor-core/src/eval/anchor.rs`'s module doc used to say "a
parameter edit CANNOT renumber, by construction". It is false for one
class of program. A loop's segment count is a function of its
arguments as well as its structure: a corner fillet whose runs reach a
`Zero` fit emits nothing (`crates/profile/src/path.rs`,
`emit_fillet_in`: "A `Zero` fit emits nothing and springs the arc off
the last vertex"). So a plain `DocEdit::SetParam` on the fillet's
radius changes how many segments the step draws, and every LIVE name
on a later segment of that loop denotes a different wall afterwards —
with no refusal, no `Maintenance` row, and no `Vanished`, because the
old index is still drawn.

Measured by reviewer R1 on the retired coordinate (the first spelling
of the retirement, one past the loop's end, went live under such an
edit — answered by `RETIRED_FLOOR`, see
`stranded-names-are-retired-to-an-undrawable-coordinate`), and
extended here by one row on a LIVE name, pinned as measured:
`edit_set_program::a_slot_edit_through_a_zero_fit_renumbers_a_live_name_and_reports_nothing`
(`crates/editor-core/tests/edit_set_program.rs`). The chain `At,
Toward(+x), Fillet(r), Toward(+y), FarEndTo(2,2), LineTo(0,2),
LineTo(Start)` draws THREE segments at `r = 2` (both runs fit `Zero`)
and FIVE at `r = 0.3`; a derived frame on wall 2 — the LEFT edge
`(0, 2) → (0, 0)` at `r = 2` — keeps evaluating after `SetParam
r = 0.3` on the RIGHT edge `(2, 0.3) → (2, 2)`, `Applied.maintenance`
empty. The frame did not refuse and did not move: the name it holds
now denotes the opposite wall.

## Where it sits in the design

`anchor.rs`'s sentence is corrected to name the exception (this row's
id); DI1's record line in `crates/editor-core/IDENTITY.md` names it as
the same aliasing class one level in, on a held locator rather than a
node id. `SlotId::Profile`'s stability claim (V2: "step indices are
stable under every slot edit") is still true — STEP indices are; it is
the SEGMENT indices the published names carry that move, and a step
that draws a variable number of segments is exactly where the two
numberings come apart.

## What would close it

Not this unit's to fix. The door that knows is `SetParam`'s
`check_profile_after_slot_edit`, which replays the program and could
compare the per-step spans before and after through the same
`SegmentMap` `SetProgram` uses (DM8's checked records on both sides,
the identity provenance): a step whose span length moved under a slot
edit would then report `Rebound` for every name after it, or strand
it, exactly as a reshaping does. That is a widening of what a slot
edit may report — today it reports nothing but cluster maintenance —
and is a design question for EDIT, filed here rather than built.

## Survey (2026-09-24, EDIT orchestrator, read-only lane on main at `a0be7d132`)

**Doors that change a profile's replay without changing its steps.**
`SetParam` and `SetExpression` on a profile slot
(`check_profile_after_slot_edit`, `edit.rs`), `SetDocParam` and
`SetDocParamValue` through `write_doc_param`, which re-checks slots
but replays no profile, so a parameter change that reshapes a program
is not even refused by VQ9 today. A `Count` parameter reaches a profile
argument through `CountToScalar`. `SetTolerance` can change whether
fused-arc carriers are identical (`carriers_are_identical`, the ε
band), which changes the segment count; it has no in-process door.
`UpdateReference` moves a part's pin, and a host's names embed the
part's names through `InPart`. Names held outside the document (the
viewer's selection, Python text) never see a `Maintenance` row.

**Analysis.** Profile structure is chosen at the document's nominal
values. The guided lane pass refuses a sample whose fit or span
changes (`structure.rs`, `path/program.rs`), and Monte Carlo counts
it unmeasured. No sample silently re-denotes a name today.

**Steps whose segment count depends on values.** The line-line corner
fillet (`resolve_fillet`), `FarEndTo` (`end_side_at`), the arc-carrier
fillets through `emit_fillet_in` (the fused `FilletArc`, `ArcFillet`,
`ArcFilletArc`), and fused `FromTip` carrier identity (the ε band).
`CircleSplit`, the ray merge and the degenerate arms are not
value-dependent. The fit gates use exact order, so `Zero` is a knife
edge, reached mainly by authored coincidences such as a full-round
slot whose radius is the half-width.

**Naming by step needs roles, not ordinals.** Spans are positional: in
this row's chain the `FarEndTo` step draws [line, arc, line] at
r = 0.3 and [arc] at r = 2, so an ordinal piece index flips from a
line to the arc. The arc's role is already recorded
(`RadiusEmission`, keyed by the authoring step).

**Size of naming by step.** About ten production sites: the sweep
emitters and `role.rs` builders (unchanged if the anchor rewrite does
the translation), the anchor rewrite in `eval/anchor.rs` (the choke
point), DM8's door in `program.rs`, `SegmentMap`, the `RoleSeg`
rewrite walk, content-key hashing, the derived `Ord`. The resolver and
pick path match names by equality and read no coordinate. The Python
text form is the serde form. The wire form has no schema version;
renaming the field makes old files refuse typed. Files carrying
`"segment": N`: the bool13 goldens, `golden/golden.cad`, the die tour
corpus, `plate_param.pncad`, `test_document.py`, the tess budget CSV.

**Stale sentences either fix must sweep.** `crates/profile/README.md`
V3 ("a continuous edit cannot renumber"), `names/role.rs` ("a
parameter edit cannot renumber a frozen selection"), `names/README.md`
N1 (this PR), `pncad-py/src/py/select.rs` ("canonical chain").
