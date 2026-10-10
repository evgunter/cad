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
