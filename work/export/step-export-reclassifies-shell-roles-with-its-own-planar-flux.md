---
id: step-export-reclassifies-shell-roles-with-its-own-planar-flux
kind: issue
title: step-export decides a multi-shell solid's outer/void roles with its own planar flux and a raw f64 sign, refusing every curved shell that topo::classify_shells_of already classifies
status: open
opened: 2026-09-28
priority: P1
cost: M
refs: [3393, step-export-refuses-every-hollow-body]
---


Found by S-DUP's sweep for `a-shell-role-is-decided-by-two-spellings`
(the row this one refs). It sits on EXPORT's slate because the fix
edits `crates/step-export/src/*`.

A shell's outer/void role has one kernel reading:
`topo::props::ShellRole::decided_at`, which check 7, check 10,
`classify_shells`/`classify_shells_of` and point containment's
at-infinity side all route through. `step-export` has its own copy,
and the copy includes the flux:

- `crates/step-export/src/volume.rs`, `shell_signed_volume`, is a
  second divergence-theorem walk over the planar/line subset. It
  refuses any other face as `StepExportError::CurvedShellClassification`.
- `crates/step-export/src/writer.rs`, in the multi-shell branch that
  calls `shell_signed_volume`, reads the role as a raw `f64` sign.
  `volume == 0.0` or non-finite gives `ShellVolumeIndeterminate`.
  `volume < 0.0` gives `VoidShellUnsupported`. No band is involved.

The module docs justify both parts: "no curved counterpart in closed
form", and the sign is "safe by headroom". The first claim no longer
holds. `topo::classify_shells_of` classifies curved shells, in
closed form where a face has one and with a certified bracket where it
needs quadrature. A shelled vessel (a revolved cylinder shelled at
0.1) classifies through it as one `Outer` and one `Void`, both in
closed form (`volume_pad == 0.0`), measured 2026-09-28. Yet `crates/sweep/tests/verbs_shell.rs`,
`a_curved_two_shell_shell_refuses_step_export`, pins the export of a
shelled tube as a standing `CurvedShellClassification` refusal. So
does `demos/README.md` ("no closed form yet"), along with
`docs/KERNEL-VERBS.md`'s STEP rows. The writer refuses on a question
the kernel's own door answers.

**Fix.** Read the roles with `topo::classify_shells_of(body,
shells, tol)` over the solid's shells, then retire
`shell_signed_volume`, and with it `CurvedShellClassification` if
nothing else raises it. Map `ShellClassifyError` onto the writer's
refusals. The self-retiring rows above then flip, and they say so.

**The behaviour this changes**, which the owner weighs:
- the sign becomes band-decided rather than a raw `f64` comparison, so
  a planar shell whose volume is within the band now refuses typed
  (`ZeroVolume`/`Escalated`) where it used to pass on headroom;
- a curved multi-shell solid is classified instead of refusing
  `CurvedShellClassification`. A shelled tube still does not export:
  its void shell now refuses `VoidShellUnsupported`, the writer's
  standing refusal of every hollow body until it writes
  `BREP_WITH_VOIDS` (`step-export-refuses-every-hollow-body`). The
  pinned row flips to that refusal, not to success;
- the error surface changes.

**A consumer the fix makes export, not flip** (SHOW,
projectbox-section-cuts-through-bores, 2026-10-02). The tour's
`projectbox` cell splits the bored enclosure by a tilted plane through
two of its round, bored bosses (`demos/tour/src/cutaway.rs`,
`sectioned_beside`). The plane frees both boss tops, so the above half
is one solid of three shells (the walls' piece and two caps), and its
arcs make it refuse `CurvedShellClassification` (`kind: "circle
curve"`). The scene pins it with `SceneBody::step_at_frontier`, so its
manifest `step` is null and the FreeCAD lane draws no above half.
`topo::classify_shells_of` on that half returns `[Outer, Outer, Outer]`,
measured: the tour asserts it
(`projectbox::tests::the_bored_box_and_its_halves_pass_their_tiers_and_the_above_half_is_three_outer_shells`),
and the review measured the same for square bosses and for cuts at
`z = 0.45` and `0.6`. With the roles read through `classify_shells_of`
this body would EXPORT, so its pin fails as a success and says to drop it,
unlike the hollow-body pins (`hollowring`, `hollowtorus`,
`torusvessel`, `fivewall`), which flip to `VoidShellUnsupported`.

**2026-10-03 — the `projectbox` cut exports another way (FUSE, PR
3891).** Under Ev's ruling that a solid is one piece of material (PR
3901), `split` sorts each side into pieces, so the above half is now
three solids of one shell each and the writer classifies no shell at
all: it exports. Its `step_at_frontier` pin is dropped
(`demos/tour/src/cutaway.rs`). The writer's own classification gap this
item names is untouched; this body just no longer reaches it.

## Evidence from REACH (`reach/check7-interval`)

`classify_shells_of` reads a shell's role off its interval
re-derivation (`topo::props::rederive`). The `f64` sum is not that
re-derivation. A shell whose volume is below its own rounding therefore
refuses typed there instead of reading a sign. The witness is `topo`'s
`tier3_tests::far_anchored_slab`, where the `f64` sum reads −1.2e-12 m³
for a +1e-13 m³ slab. `shell_signed_volume`'s raw `volume < 0.0` has no
such guard. Routing the writer through `classify_shells_of`, as the fix
above says, closes that too.
