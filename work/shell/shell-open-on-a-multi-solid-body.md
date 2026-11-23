---
id: shell-open-on-a-multi-solid-body
kind: issue
title: shell_open on a multi-solid body refuses NotOneSolid: hollow, hollow, open is not three verbs
status: closed
opened: 2026-09-08
refs: [SHELL-8, SHELL-5]
closed: 2026-09-08
---


Found by SHELL-5's R1 end-to-end row (PR 2159,
`crates/sweep/tests/shell5_r1_probes.rs`,
`r1_e2e_hollow_twice_then_open_the_inner_wall`, step 3a): a user who
hollows a part, hollows it again (two thin solids now) and then wants
the inner wall opened writes `shell_open` on the twice-hollowed body,
and it refuses `ShellError::NotOneSolid { solids: 2 }`. The spelling
that works folds the opening into the second hollowing —
`shell_open(first, t, &[ceiling])` — so "hollow, hollow, open" is not
expressible as three verbs.

`ShellError::NotOneSolid`'s docs (`crates/topo/src/shell.rs`) say what
it would take: either a designation of WHICH solid to shell, or a
per-solid composition that inserts each solid's own moved clone into
that solid and partitions per solid. The opened arm on a multi-solid
body has the extra question of which solid a designated face's
counterpart lives in, which the graft map answers once the composition
is per solid.


**Closed by SHELL-8.** Shelling is per solid and applies to every solid
of the operand: each solid's own gates, offset door, clearance, moves,
evidence, insertion and partition, and a designation names a face on
any of them. `ShellError::NotOneSolid` is retired; only an operand with
NO solid refuses (`ShellError::NoSolid`), which is an empty operand
rather than an unsupported arity. The question the issue left open —
which solid a designated face's counterpart lives in — is answered on
the RESULT: the thin solids are partitioned before the rim surgery, so
both sides of the glue are found in the solid the designated face is in
there, and the lift's door is that solid's. "Hollow, hollow, open" is
three verbs (`crates/sweep/tests/shell5_r1_probes.rs`, step 3a).
