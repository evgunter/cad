---
id: check-10-reads-a-cavitys-role-from-its-volume-sign
kind: issue
title: Check 10 reads a cavity's role from its V/A, so a definite thin cavity (brick minus slab) refuses ShellRole though its faces are a definite pair
status: open
opened: 2026-10-11
priority: P2
cost: M
design: true
---


Found by FLUSH's thin-hollow design fork (fork log row 113). Measured
on main at ε = 1e-9: `brick (−1,2)³ ∖ slab(1.5e-8)` refuses
`Escalated { ShellRole }` on the slab-shaped cavity (V/A −7.5e-9).
The cavity's faces are a definite pair 1.5e-8 apart, and the same slab
alone builds.

Both designers keep check 10's refusal for now, but one of them leans
towards reading a shell's role off a crossing ray rather than V/A. With
that, the "shell" half of D10 would reduce to pairs. That is a
follow-up question after the row-113 ruling, not part of it. It
depends on the containment fix in
`work/cleave/a-missed-ray-reads-the-solids-volume-sign-for-its-side`.
