---
id: path-scene-resolution-recourses-advise-a-coarser-tolerance-beside-geometry-levers
kind: issue
title: Two path recourses advise a coarser tolerance beside geometry levers, against D4 ¶1 (i)
status: closed
opened: 2026-10-21
priority: P3
cost: E
closed: 2026-10-21
branch: nurbs/scene-resolution-recourse
---


DESIGN D4 ¶1 (i) (`docs/DESIGN.md:509-518`): *"No refusal advises loosening
ε, except one that would otherwise name no recourse at all (a kernel
approximation limit), which names it as a last resort and says the refusal
may indicate a kernel bug."*

Two recourses in `crates/profile/src/validate.rs` name a coarser tolerance
beside geometry levers they already name:

- `ARC_SCENE_RESOLUTION_RECOURSE` (`validate.rs:197-199`): "move the profile
  nearer the origin, or author the arc with a smaller radius …; a coarser
  tolerance also reads it". The roster pins the wording at
  `crates/profile/tests/recourse_roster.rs:547`.
- `ARC_READ_BACK_UNDECIDED_RECOURSE` (`validate.rs:206-208`) *leads* with "a
  coarser tolerance reads it". It does go on to say it may be a kernel
  defect, so it may fall under the last-resort exception. Its other lever
  ("moving the profile nearer the origin") suggests not.

Written in paths fix pass 2 (9ff39614). Before changing either, check
whether an Ev ruling permits a coarser ε at a scene-resolution limit
(`git log -S'coarser tolerance' --all`, the PR threads of that pass). If one
does, the D4 sentence is what needs a ruling, as an `[ev]` PR, and not this
text.

Found while ruling on CENSUS PR 5513's `SPAN_PRECISION_RECOURSE`, which
copied this precedent and was corrected to its geometry levers (FULL review
M2). Scan the other programs' recourse constants for the same shape when
this is picked up: `grep -rn "coarser tolerance" crates --include=*.rs`.

**The `ContinuationTargetOffRay` instances (FULL review of PR 5536).** "or
widen the input tolerance" appears at `crates/profile/src/path.rs:1768` (the
OffRay `Display`), `:2086` (the offset escalation) and `:2101` (the
`path_leg_length` escalation). OffRay is a definite arm, so D4 ¶1 (i)'s
"never on a sign-certain arm" applies on top of "no refusal advises loosening
ε". The advice is prescribed by `docs/PATHS-DESIGN.md` §4's ε_input
paragraph ("or, if the miss is genuinely acceptable, to WIDEN the input
tolerance"), so two texts disagree. Provenance: the §4 sentence was written
in the BOOL-11 review-fix commit f4cd0eb3 (PR 1520), agent text, the same
commit as the data-gate paragraph PR 5498 retired. D4 ¶1 (i)'s rule came
later, in 7b494764 (2026-09-28, the round-2 answer to Ev's question on the
split). It says "loosening only at the import door (ε_in) and at an ε change",
which does not cover a path verb's ε_input refusal. So D4 is the later and
governing text: §4's sentence and the three messages change together, to
the geometry lever only. Before editing, check that 7b494764 merged under
Ev's ruling (its PR thread).

## Closed

**The ruling.** 7b494764 is on `[ev]` PR 3352 ("D4 two-tolerance: one
recourse per decision"), merged 2026-09-28. Ev asked whether to drop
the tolerance advice or keep it "only when there may be situations
where there is no other recourse", and ruled on the designers'
tighten-only answer: "this sounds good! i do think keeping 'loosen
eps' as a last resort when there would otherwise be no recourse at all
seems sensible, though it might make sense to also say that this may
indicate a bug". That is D4 ¶1 (i) as merged. PATHS-DESIGN §4's
"WIDEN the input tolerance" came from f4cd0eb3 (PR 1520), after the
document's ratification (#124), and PR 1520's thread has no word from
Ev on it. So D4 governs, and the §4 sentence changed with the
messages.

Five recourses now name their geometry levers only:
- `ARC_SCENE_RESOLUTION_RECOURSE`: nearer the origin, or a smaller
  radius.
- `ARC_READ_BACK_UNDECIDED_RECOURSE`: nearer the origin, and report it
  as a possible kernel defect where the scene resolves the size. It is
  not D4's last resort: that is for a refusal "that would otherwise
  name no recourse at all", and this one names a geometry lever. Its
  kernel-defect clause stays, true without the exception.
- The three `ContinuationTargetOffRay` sites (`path.rs`: the
  `Display`, the `path_continuation_target_offset` escalation and the
  `path_leg_length` escalation).
- A fifth the item did not list, in the same match: the seam-arrival
  escalation (`path_seam_arrival_turn`/`_side`) said "widen the input
  tolerance (K·ε); LOWERING the tolerance is the wrong direction". It
  passes only at Zero, so D4 gives it no ε arm either way.

`docs/PATHS-DESIGN.md` §4's ε_input paragraph says the same.
`recourse_roster::no_lever_ending_advises_a_coarser_tolerance` guards
every lever ending in `DECISION_ENDINGS`.

**Outside PATHS, not changed.** The scan
(`grep -rn "coarser tolerance\|loosen the tolerance\|widen" crates`)
finds every other loosening in the last-resort shape with the
kernel-bug clause, except two:
- `geom-brep/src/ssi.rs` `STEP_BUDGET_CURVATURE_RECOURSE`: "name a
  domain around just the feature traced, or loosen the tolerance,
  [last resort]". It loosens beside a geometry lever.
- `editor-core/src/names/emit.rs` `AMBIGUITY_FACTOR_LEVER`: "raise the
  tolerance's ambiguity factor K". It coarsens ε_input = K·ε by
  another dial.
