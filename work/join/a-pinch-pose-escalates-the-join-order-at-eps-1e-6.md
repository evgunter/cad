---
id: a-pinch-pose-escalates-the-join-order-at-eps-1e-6
kind: issue
title: pinch_faces_tessellate's notch307 fib117 edge psi=1.9 cp S escalates bool_join_nearest at eps 1e-6, on main too
status: open
opened: 2026-10-06
priority: P3
cost: E
---


Found by CLEAVE's ray-walk PR 4083, whose diff seeds `topo` and so ran
`sweep`'s rows at ε 1e-6 and 1e-12 too. The pose
`notch307 fib117 edge psi=1.9 cp S` in
`crates/sweep/tests/pinch_faces_tessellate.rs`
(`a_face_through_two_vertices_on_one_point_tessellates`) refuses at
ε 1e-6:

```
Err(Escalated { decision: Coincidence(Join, Moot), diag: Indeterminate {
  margin: Value(-5.196042020649827e-6), band: (1e-6, 1e-5),
  predicate: Some("bool_join_nearest") } })
```

Measured on `main` at `cadf2ed188`, with nothing of PR 4083 applied: it
is the same refusal, so it is not that PR's. It passes at 1e-9 and
1e-12. Two chords from one germ to two candidate partners
(`join::nearer`) differ by 5.2 µm, which is inside the 1e-6 band, so the
join cannot order them. This is an honest escalation at that ε, unless
the order can be read some other way (the half-turn,
`bool_join_arc_travel`) on this pose.

PR 4083 pins it in the row (`REFUSES_AT_1E6`) so the row is not red at
the extra ε rows. To be decided: whether the pose should build at 1e-6,
and if so, remove the pin.
