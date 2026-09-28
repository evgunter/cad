---
id: step-export-reclassifies-shell-roles-with-its-own-planar-flux
kind: issue
title: step-export decides a multi-shell solid's outer/void roles with its own planar flux and a raw f64 sign, refusing every curved shell that topo::classify_shells_of already classifies
status: open
opened: 2026-09-28
priority: P1
cost: M
refs: [a-shell-role-is-decided-by-two-spellings]
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
`docs/KERNEL-VERBS.md`'s STEP rows. A body the shell verb builds on
everyday geometry cannot leave the tree, though the kernel's own door
answers the question the writer refuses on.

**Fix.** Read the roles with `topo::classify_shells_of(body,
shells, tol)` over the solid's shells, then retire
`shell_signed_volume`, and with it `CurvedShellClassification` if
nothing else raises it. Map `ShellClassifyError` onto the writer's
refusals. The self-retiring rows above then flip, and they say so.

**The behaviour this changes**, which the owner weighs:
- the sign becomes band-decided rather than a raw `f64` comparison, so
  a planar shell whose volume is within the band now refuses typed
  (`ZeroVolume`/`Escalated`) where it used to pass on headroom;
- curved multi-shell solids stop refusing;
- the error surface changes.
