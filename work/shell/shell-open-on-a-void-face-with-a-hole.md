---
id: shell-open-on-a-void-face-with-a-hole
kind: issue
title: shell_open on a void face carrying a hole is unmeasured
status: closed
opened: 2026-09-08
closed: 2026-09-08
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

## Closed

Closed in PR 2159's fix pass (2026-09-08): both reviewers built the
pillar-through-a-void fixture and opened its holed ceiling green with
the closed form. The adopted row is
`shell5_r1_probes::r1p6_open_a_void_ceiling_with_a_pillar_through_it`
(a `4³` box minus a `2×2×2` box carrying a `0.4×0.4` pillar, `t = 0.1`:
tier 3, volume `[4³ − 3.8³] + [(2.2³ − 0.2²·2.2) − (2²·2 − 0.4²·2)] −
(2.2² − 0.2²)·0.1 = 11.528`, one rim and one hole rim both facing the
gap, the designated face dead, the operand void fused away, and the
body tessellates); R2's `6×6×4` build of the same fixture measured the
same shape.
