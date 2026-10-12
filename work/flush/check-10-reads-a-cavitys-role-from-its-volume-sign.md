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

## Measured (CLEAVE `cleave/missed-ray-side`, 2026-10-11)

The containment fix this row depends on is on that branch: a missed
ray's side is read off a ray aimed at a planar face wherever the
volume does not decide it. With it, `brick (−1,2)³ ∖ slab(t)` still
refuses `Escalated { ShellRole }` on the cavity for every
`t ∈ {10, 12, 15, 20}ε` at ε 1e-9. That is this row's reader.

The same reader, a shell's role off its `V/A`, also serves the public
`topo::classify_shells`. On a lone at-rest slab `t` thick it refuses
`Escalated { chk_shell_volume_sign }` with margin `t/2`, for every `t`
above at ε 1e-9 and 1e-6. A one-shell solid's shell is its outer shell,
so this is not a question about the slab. `editor-core`'s expected-
components check (`checks.rs`, `classify_shells`) therefore refuses a
thin body it could count. The readers that share it are
`props::shell_role_read` (`stands::ShellRead`), the result sort
(`pieces.rs`), `shell.rs`'s void read, and the census's `gate_role`.
