---
id: shell-roles-refusal-ends-with-no-recourse
kind: issue
title: topo: ShellError::Roles forwards the shell-role refusal without the decision's ending
status: open
opened: 2026-09-28
---


(ENCL implementer, from the §5 sweep of the checks-window escalated
evidence row, `work/encl/checks-escalated-evidence-says-lower-the-tolerance.md`.)
The rule is D4 ¶1 (i) in `docs/DESIGN.md`.

## What

`crates/topo/src/shell.rs`, `ShellError::Roles`'s `Display` (~:574),
renders "the body's shells could not be sorted into one outer boundary
and its voids: {error}" and stops. On the `Escalated` and `ZeroVolume`
arms `error`'s own `Display` is payload only (an `Indeterminate`
renders data, not a recourse), so a shell op refused on its operand's
shell roles names no recourse at all.

## Repair shape

`topo::ShellClassifyError::ending` (`crates/topo/src/props.rs`) is now
the shell-role decision's one ending, and the checks window appends it.
Append it here too where it is `Some` (the `Props` arm already carries
its payload's recourse), and pin the rendered row in the shell verb's
refusal roster.
