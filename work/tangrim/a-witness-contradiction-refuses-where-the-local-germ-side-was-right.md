---
id: a-witness-contradiction-refuses-where-the-local-germ-side-was-right
kind: issue
title: The witness's contradiction rung refuses a near-tangent arc that comes back within its arm, where the germ's local side was right
status: open
opened: 2026-11-03
priority: P2
cost: M
---

Found by PR 6385's re-review (`a-leaves-verdict-carries-no-witness-point`).

`topo/src/bound_side.rs` `Tie::witnessed` (≈1070, the `Some(lead) if
lead != side` arm) refuses `ReadFault::Witness` where the parting is
decided toward one side and the edge's own point at the arm is decided
on the other. The re-review sampled about 12 000 near-tangent poses (a
circle against a sphere or a cylinder, bending near the face's normal
section). 1 004 of them refuse this way because the arc comes back
across the face within the arm, while main's local germ side, the
parting's sign, was right near the vertex. No test in `-p sweep -p topo`
reaches it (PR 6385's outcome diff).

The arm is the shorter of the edge's extent and the face's radius, which
is long enough for the third-order remainder to flip the point at the
arm. The likely remedy is an arm short enough for the leading term to
hold (a remainder bound on the arc and the carrier), read at a length
where the witness and the parting must agree, not a refusal. Pin: the
review's probe `rerev6385-probes/probe_head.rs` (branch
`tangrim/rerev6385-probes`, commit 1bb6d094).
