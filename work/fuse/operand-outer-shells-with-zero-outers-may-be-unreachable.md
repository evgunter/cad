---
id: operand-outer-shells-with-zero-outers-may-be-unreachable
kind: issue
title: ShellError::OperandOuterShells with zero outers may be unreachable now that shell takes an AtRestBody
status: closed
opened: 2026-10-06
priority: P3
cost: E
refs: [one-home-for-where-a-shell-stands]
closed: 2026-10-08
pr: 4315
---


## The finding

Since main's `shell8_r2_probes` change (shell takes an `AtRestBody`, so
a no-outer solid is refused at the gate), the 4108 lane believes the
zero-outers arm of `ShellError::OperandOuterShells` is unreachable:
- an at-rest multi-shell solid has every shell's role decided (check
  10);
- a solid of voids only fails check 7.

It did not change the variant. Confirm with a mutant (`unreachable!`
in that arm, full ci suite). If the arm is unreachable, retire it or
make it an invariant; otherwise pin the reaching input.

## Closed (PR 4315, 2026-10-08)

Confirmed and retired on SHELL's
`shell-operand-shape-arms-behind-the-at-rest-gate`: the zero-outers arm
is check 10's `ShellWinding` on a finished operand, and the role count
is now an `unreachable!` invariant.
