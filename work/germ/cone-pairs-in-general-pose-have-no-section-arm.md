---
id: cone-pairs-in-general-pose-have-no-section-arm
kind: issue
title: Cone × cylinder and cone × cone in general pose refuse on reach at the section certificate; the ruling reduction makes them tractable
status: open
opened: 2026-09-28
priority: P2
cost: M
refs: [torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere]
---

## What

The section certificate's cone arms (`section_cert::cone_pair`, and
the coaxial cone in `torus_pair`) cover cone × plane, cone × sphere,
and coaxial and parallel-axis partners. Beyond them, cone ×
cylinder and cone × cone in GENERAL pose refuse on reach. The ruling
reduction makes them tractable: each cylinder ruling meets the cone in
a quadratic whose discriminant is a degree-2 trigonometric polynomial
with at most four roots, and the in-tree Ferrari quartic
(`line_torus_roots`) can isolate them (the spec's §2.7 and Q5).
