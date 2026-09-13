---
id: levered-clash-margins-hide-their-arm
kind: issue
title: Three levered mate-fold clash margins reach the refusal with their arm invisible
status: open
opened: 2026-09-04
---


Residue disclosed by `mate-contradiction-names-one-mate-twice`, which
gave `MateFault::Contradictory` a `lever: Option<(f64, f64)>` and filled
it at the one raising site that has the lever in hand
(`crates/editor-core/src/mate/solve.rs:544`, the clocking rider). The
socket is in place; three sibling margins still arrive with `None` and
so still print a levered product as a bare metre figure the reader
cannot re-derive:

- `crates/editor-core/src/mate/coset.rs:582` — `mate_member_axis_fixed`,
  margin `(x.linear * axis - axis).norm() * arm`.
- `crates/editor-core/src/mate/coset.rs:643` — `rotation_residual`,
  feeding `mate_member_rotation_identity`, margin `‖Q − I‖_F · arm`.
- `crates/editor-core/src/mate/coset.rs:743` —
  `mate_rotation_two_axis_reachable`, margin `reach * arm`.

All three reach `MateFault::Contradictory.clash` through
`FoldStop::Clash { predicate, margin }`
(`crates/editor-core/src/mate/solve.rs:690`). Filling the lever there
means widening `member_of`'s `Err((name, margin))` and the `checks`
vec to carry the disagreement and the arm beside the product, plus the
same for `candidate_rotation` — solve-internal plumbing in S-MATE's
live territory (`crates/editor-core/src/mate/*`), not the
refusal-display prose S-MATE's `keep_out` cedes. It was left out of the
display unit on that fence and needs S-MATE's assent or a re-home.

The five remaining `member_of` predicates
(`mate_member_translation_zero` / `_along` / `_in_plane`,
`mate_member_point_on_axis`, `mate_member_point_fixed`) measure lengths
outright and correctly carry no lever; two rows in
`crates/editor-core/tests/asm_r2a_mate_solve.rs` pin that.

The tree's own precedent for the shape is
`crates/profile/src/path.rs:1201`, `PathError::JunctionCusp`, which
renders "turn margin {margin} m on a {arm} m arm".

## The socket is typed RADIANS — do not fill it blindly

`lever` is documented and rendered as `(radians, arm)`: the `Display`
writes "a roll of {radians} rad". That is honest for the one site
filling it today, whose disagreement IS an authored angle. **It is not
honest for any of the three above** — a sine (`mate_axes_parallel`), a
Frobenius departure from the identity (`rotation_residual`) and a
reach are pure numbers, not radians, and dropping one into this field
would re-mint the very defect the display unit closed, one field
inward.

An earlier draft carried a `unit: &'static str` beside the value to
cover both cases. It was removed as a free string on a public surface
that could be set to `"m"` and print a length as a disagreement — a
knob no production site varied. Whoever takes these three owes the
choice explicitly: a second arm in the sentence for dimensionless
residuals, a typed unit (`quantity::AngleUnit` and friends exist), or
a small-angle argument for calling a sine radians. It is a decision,
not a fill.

## Re-homed from FIX to MSOLVE, 2026-09-12 (FIX orchestrator)

**The "needs S-MATE's assent or a re-home" line above is stale, and
the way it is stale matters.** S-MATE left the tracker on 2026-09-04
(`docs/DOC-LEDGER.md` sweep 6), which reads at first like the blocker
evaporating. It is the opposite: the territory did not go unowned, it
was **inherited by two open programs**. `work/msolve/program.md` and
`work/docm/program.md` both carry `crates/editor-core/src/mate/*` in
`paths`. So the assent was still owed — to a live owner rather than to
a program that no longer exists — and a FIX lane reading that line as
"the owner is gone, take it" would have walked into two live fences.

**MSOLVE rather than DOCM**, on the charters as written: this row is a
refusal that reaches the user with its arm invisible — assembly
SEMANTICS and refusal quality in the solve, which is MSOLVE's charter
("one refusal that reports a false cause"), not document custody.
`mate-clocking-has-no-gui-path` went to DOCM by the same reading,
since its live half is an `AddMate` door and the `DocEdit` set is
DOCM's.

**And MSOLVE is working this exact subject right now.** Open PR
**#2116, "MSOLVE-6: the mate's lever is the mated parts' own
extent."** This row is about three levered mate-fold clash margins
whose arm the refusal does not carry. Same file family, same quantity.
FIX dispatching it would have raced a live PR on the lever it is
about; **sequence it against #2116 rather than beside it.**

**The decision in the row's last paragraph is unchanged and is still
the substance**: `lever` is typed and rendered as `(radians, arm)`,
and a sine, a Frobenius departure from the identity and a reach are
pure numbers, not radians. Filling the socket blindly re-mints the
defect the display unit closed, one field inward. A second arm in the
sentence for dimensionless residuals, a typed unit, or a small-angle
argument — it is a decision, not a fill.

FIX took nothing here and changed no code; this is a routing move
only.
