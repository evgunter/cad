---
id: stackup-pairing-reads-the-shells-dual-refusal-as-a-violation
kind: issue
title: A sensitivity run over a shelled document fails whole on PairingViolation: pair_pass has no valve for the shell's DL3 refusal
status: open
opened: 2026-10-01
priority: P3
cost: M
---


## Finding

Read off the code, not run: found by BAND's `S90-impl` measurement,
which needed to know what a document-level `Dual64` evaluation does
when one of its nodes refuses on the DL3 seam.

- `topo::AtRestPolicy for Dual<T>` answers `shell_door()` with `None`
  (`crates/topo/src/props.rs`, the dual arm), and
  `eval::wire::wire_shell` turns that into
  `NodeErrorKind::ShellLaneUnsupported { scalar: "dual" }`, pinned by
  `lib_g17_shell_node::a_dual_evaluation_refuses_the_shell_typed`.
  `docs/DUAL-DESIGN.md` DL3 names this refusal as the designed one.
- `stackup::driver` evaluates the document at `Dual64` and pairs each
  pass with the `f64` anchor through `stackup::pair_pass`. That function
  turns exactly two `(Ok, Failed)` kinds into the per-entry valve
  (`SeedPinnedSection`, `ProfileLaneReplay` →
  `SensitivityOutcome::Unliftable`). Every other `(Ok, Failed)` pair,
  `ShellLaneUnsupported` among them, goes to `same_arm` and returns
  `PairingViolation::ResultArm`.
- So `stackup::sensitivities` over any document that contains a shell
  node fails as a whole, reporting that the build broke pairing. Per
  its own docs, `LiftRefusal` exists for "the lift's typed limits,
  never a finite wrong number", and the shell refusal is one of those
  limits.

Either the designed DL3 refusals (the shell's, plus any later door
value answered `None` at `Dual`) get a `LiftRefusal` arm, or the
whole-call failure is intended and should be written down at
`pair_pass`. BAND's `S90-impl` makes this matter more: one of the
options for tightening the blend doors to `CertifiedBounds` makes a
fillet or chamfer node refuse at `Dual` in exactly this way.
