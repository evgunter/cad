---
id: path-scene-resolution-recourses-advise-a-coarser-tolerance-beside-geometry-levers
kind: issue
title: Two path recourses advise a coarser tolerance beside geometry levers, against D4 ¶1 (i)
status: open
opened: 2026-10-21
priority: P3
cost: E
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
