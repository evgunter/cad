---
id: mint-offset-ignores-cone-mirror-nappe
kind: issue
title: replace_face::mint_offset does not discharge ConeOffset's mirror-nappe consumer obligation
status: open
opened: 2026-08-29
github: 1199
refs: [1180]
---

## From GitHub issue 1199

Opened 2026-08-29; 0 comments.

`replace_face::mint_offset` does not discharge the cone's mirror-nappe obligation

**Not a defect in the mint.** `geom_brep::ConeOffset`'s header
(`crates/geom-brep/src/offset.rs:108-129`, ratified at ordinal 79) is
explicit that `n₊` does **not** flip across the apex, and says why:
following the per-point chart normal would split the double cone rather
than translate a parameter. It then states the obligation that puts on
consumers, verbatim — *"A mirror-nappe face's material therefore moves
`−d` along its OWN chart normal."*

**The consumer that does not discharge it.**
`crates/topo/src/replace_face.rs:1190`, inside `mint_offset`, hands the
caller's `d` straight to `geom_brep::offset_surface`. Its caller
`shell::inward()` derives that `d` from the face's SENSE — i.e. along
the FACE's outward direction — so on a face below its apex the two
conventions are opposite and the sign reaching the mint is turned the
wrong way.

`topo::offset_axial::nappe_signed` discharges the same obligation for
the axial door (#1180): the nappe is read from the face's own corners
and DECIDED, and a face straddling the apex is refused rather than
guessed.

**No wrong body ships from this today**, and that is measured rather
than assumed. Both review arms of #1180 attacked it independently:

- `sf2b_r1_probes::r1p3_the_cone_mint_is_nappe_blind_and_the_door_corrects_it`
  reproduces the sign at the MINT, off any door, and shows the axial
  door's correction against it.
- `sf2b_r2_probes::r2_per_chart_door_on_a_mirror_nappe_cone` asks the
  public single-chart verb to pull a cone chart inward on BOTH nappes
  and reports which way the volume actually went, with the tier-3
  verdict beside it.

On every fixture either arm could reach, the neighbouring CAPS refuse
first (`ReanchorOffCarrier` — the per-chart door's own oblique-junction
gate), so the turned sign never reaches a body that gets built. It is a
latent hazard standing behind a gate, not a live one.

**What closing this needs.** Either lift the nappe resolution to a
shared home both consumers read, or give `mint_offset` the face and let
it discharge the obligation as the axial door does. Whichever, the
SWEEP must include `ConeOffset::displacement` — it is the third face of
the same derivation and its `copysign` on the radial term is a separate
reading of the nappe question from `apex()`'s.

Filed out of #1180 (VERBS-SHELLFIX PR-2b), whose review adjudicated it
as the unswept sibling of a class that PR fixed at its own door.

## Home

`crates/topo/src/replace_face.rs` and `crates/geom-brep/src/offset*.rs` are both in VERBS' `paths:` territory, and the offset/shell arm is its Wave 3 ground.
