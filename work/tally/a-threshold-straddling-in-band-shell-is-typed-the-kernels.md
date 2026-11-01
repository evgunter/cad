---
id: a-threshold-straddling-in-band-shell-is-typed-the-kernels
kind: issue
title: A result shell whose certified V/A straddles the band's edge is typed the kernel's (ResultInvalid) though it lies within ulps of the band
status: open
opened: 2026-10-09
priority: P3
cost: M
design: true
refs: [4415]
---


## What

Found by PR 4415's first review (NOTE-1).

The door types a check-10 role refusal `Escalated` only when the role
read's certified enclosure of `V/A` lies wholly inside one sliver band
(`props.rs` `CertifiedSliver`, `ops.rs` `finding_arm`). An enclosure
that touches or straddles the band's edge does not certify a sliver.
Such a shell stays `ResultInvalid` (a kernel defect), though D10
reserves `ResultInvalid` for definite findings.

On `notch307 nt e0 a3` at ε = 1e-9 the reviewer bisected the tilt
d* = 3.0407001874e-8, where the exact V/A is 1e-8 = Kε, and took 17
tilts at d* ± k·2e-16:
- those whose enclosure lies wholly below 1e-8 refuse `Escalated`;
- the 3 whose enclosure straddles 1e-8 refuse
  `ResultInvalid { ShellRoleUndecided }`;
- past them the intersection builds.

The window is a few ulps wide.

This row first named a second arm too: a walk that decides the ends
while the certificate is in band. PR 4415's second review witnessed it
(`vee300 nt e0 a1 d6e-9`), and that PR fixed it. The door now reads the
certified sliver beside check 10's refusal whatever the walk read
(`ValidationError::ShellRoleUndecided::sliver`).

## The shape to give

A design question:
- read a straddling enclosure again, tighter, before typing it;
- type "in band or within its rounding of the band" as in band;
- or leave the window to the kernel's arm and say so in D10's text.

## A single-shell result is never role-read at the gate (CLEAVE, 2026-10-10)

From CLEAVE's `a-built-sliver-is-not-a-legal-operand`, which is parked on
FLUXTAIL's
`a-sphere-faces-flux-cancels-r-times-its-area-against-its-vector-area`.

**What happens now.** Check 10 skips a solid with one shell
(`validate.rs` `shell_winding_errors`). Such a result's sign is decided
only by check 7, which reads the f64 closed form with `volume_pad = 0`.
A sphere-pair sliver `2e-6` thick then ships, though its certified
`V/A` enclosure, about ±9e-6, straddles zero. The next boolean refuses
it at the at-infinity read, which is the same certified walk.

**Option (B), not built.** The gate could run the certified role read
on single-shell results too. That would honour "no door ships a body it
cannot certify", but two things are open:
- **Its cost.** It adds one sign walk per result. That is unmeasured,
  as is how many results a later op can still read that it would refuse.
- **Its typing.** This row's question: an enclosure undecided by the
  kernel's own arithmetic is neither in band (`Escalated`) nor a
  definite finding (`ResultInvalid`). The CLEAVE case is the
  arithmetic's: the true `V/A` is 50 Kε, so `Escalated` would break PR
  4335's honesty condition.

## The same straddle on check 7's arm (FLUSH, 2026-10-11)

Found by a FLUSH designer lane (fork log row 113), read from the code on
main after PR 4636. When check 7's certified total for a solid has an
enclosure that straddles the sliver band's edge, `plus_v_round` records
`OpenRound::AboutTheBody(None)` and `plus_v_at_target` returns
`PlusVVerdict::Pass` (`validate.rs`). So such a result passes the
boolean door silently. Check 10's straddling shell, by contrast, refuses
`ResultInvalid`. The two subjects (a solid's total and a shell's role)
should take the same arm. This row's fix should cover check 7's case
too, or name it as a sibling.
