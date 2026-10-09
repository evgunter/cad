---
id: a-threshold-straddling-in-band-shell-is-typed-the-kernels
kind: issue
title: A result shell whose certified V/A straddles the band's edge, or whose walk decided its ends, is typed the kernel's (ResultInvalid) though it is in band or within ulps of it
status: open
opened: 2026-10-09
priority: P3
cost: M
design: true
refs: [a-near-tangent-intersections-sliver-lump-reads-its-role-in-band-and-refuses]
---


## What

Found by PR 4415's first review (NOTE-1, MINOR-2).

The door types a check-10 role refusal `Escalated` only when the role
read's certified enclosure of `V/A` lies wholly inside one sliver band
(`props.rs` `CertifiedSliver`, `ops.rs` `finding_arm`). Two arms
fall outside that and stay `ResultInvalid` (a kernel defect), though
D10 reserves `ResultInvalid` for definite findings.

- **An enclosure that touches or straddles the band's edge.** On
  `notch307 nt e0 a3` at ε = 1e-9 the reviewer bisected the tilt
  d* = 3.0407001874e-8, where the exact V/A is 1e-8 = Kε, and took 17
  tilts at d* ± k·2e-16:
  - those whose enclosure lies wholly below 1e-8 refuse `Escalated`;
  - the 3 whose enclosure straddles 1e-8 refuse
    `ResultInvalid { ShellRoleUndecided }`;
  - past them the intersection builds.

  The window is a few ulps wide.
- **Ends the f64 walk decided.** Suppose the walk reads both ends of a
  shell's bracket as zero or straddling, while the certified reading is
  wholly in band. Then the refusal keeps `ZeroVolume` or `Straddles`,
  with no sliver, so the door types it the kernel's. This takes a walk
  that is wrong by more than the band's width. The walk's world-origin
  error makes that possible
  (`work/tally/the-shell-role-refusal-quotes-the-walks-f64-margin-off-its-certificate.md`),
  but no pose is known to reach it.

## The shape to give

A design question:
- read a straddling enclosure again, tighter, before typing it;
- type "in band or within its rounding of the band" as in band;
- or leave the window to the kernel's arm and say so in D10's text.

Ends the walk decided would follow from making the certified reading,
not the walk's, the refusal wherever one was taken (the TALLY row).
