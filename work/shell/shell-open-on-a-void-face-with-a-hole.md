---
id: shell-open-on-a-void-face-with-a-hole
kind: issue
title: shell_open on a void face carrying a hole is unmeasured
status: open
opened: 2026-09-08
---


`shell_open` on a hollow operand accepts a designation on a VOID face
and runs the sealed construction's rim surgery with the glue's roles
swapped (`crates/topo/src/shell.rs`, the `host`/`guest` assignment in
`shell_open`'s rim loop: the lifted counterpart encloses the designated
face, so the counterpart survives as the rim). The hole path of that
surgery — `pair_rings(host, guest)`, `mfkrh` on the guest's ring with
the host's surface and sense, `ring_move` of the host's ring onto the
promoted face — is written in the same role terms and is argued to
nest the right way (a void's dilated twin has the SMALLER hole, so the
host's ring sits inside the guest's exactly as on the outer shell), but
no fixture reaches it: every hollow-operand row designates a ring-free
void face (`crates/sweep/tests/verbs_shell.rs`,
`opening_the_hollow_boxs_void_ceiling_cups_the_inner_wall_only`), and
the workspace has no void with a pillar through it. Until a row builds
one — a holed box subtracted from a bigger box, then the void's holed
ceiling designated — the path's correctness rests on tier 3's windings
at the verb's closing `validate_geometric`, not on a closed form.
