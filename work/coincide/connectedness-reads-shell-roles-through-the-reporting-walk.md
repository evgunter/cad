---
id: connectedness-reads-shell-roles-through-the-reporting-walk
kind: issue
title: the connectedness check counts roles through classify_shells, so a quadrature budget refusal ends a count whose roles decide
status: open
opened: 2026-10-21
priority: P2
cost: E
---


## What

Found by the sweep of
`work/quad/check-7-refuses-the-reporting-budget-on-a-definite-sign.md`
(the class: a reporting-target question answered on behalf of a caller
that does not ask it). Unmeasured on a witness; read from the code.

`editor_core::checks::connectedness` (`crates/editor-core/src/checks.rs`)
counts each output's `Outer` shells through `topo::classify_shells`.
That door is a REPORTING door: `classify_shells_through`
(`crates/topo/src/props.rs`) first runs `measured_at_target` over each
shell's faces, and a face whose quadrature cannot meet `1024·ε` refuses
the whole call `ShellClassifyError::Props { QuadratureBudget }`, which
the check files as `CheckEvidence::Unsupported`. The check reads only
`role`, and the role is decided without the number: `shell_role` reads
a solid's only shell as `Outer` by arity, and any other shell off aimed
rays, or off a SIGN-level volume walk (`shell_volume_role`).

So a document whose output is a valid body with a fitted rational wall
(the lily's lofted lens blades at the default ε, before the parent
row's fix; any such wall at a tight enough ε after it) reports its
component count as unsupported though every role decides.

## The fix's shape

Count roles through the role reader (`topo::props::shell_role`, or a
public sign-level door over it) rather than through the door that also
measures each shell's volume to the reporting target.
