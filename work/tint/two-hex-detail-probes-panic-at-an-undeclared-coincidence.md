---
id: two-hex-detail-probes-panic-at-an-undeclared-coincidence
kind: issue
title: join1_r1_hex_detail and its join3 copy panic on main: the flush hex-and-box union is refused UndeclaredCoincidence
status: open
opened: 2026-10-05
---


Found by the `topo/kill-release-band-twin` lane while it ran every
`#[ignore]`d join battery in `crates/sweep/tests/` on its branch and on
main (6e57858c), release.

`join1_r1_probes::join1_r1_hex_detail`
(`crates/sweep/tests/join1_r1_probes.rs`, the `expect("builds")` on
`topo::union(&hex, &b, tol())`, ~:415) and its copy
`join3_r2_r1copy::j3r2_r1_hex_detail`
(`crates/sweep/tests/join3_r2_r1copy.rs`, ~:550) both panic on main,
identically on the branch:

```
builds: UndeclaredCoincidence { diag: Indeterminate { margin: MarginDiag(Value(0.0, None)),
  terminal_sliver: false, band: Band { zero: 1e-9, escalate: 1e-8 },
  predicate: Some("bool_plane_offset") }, pair: [(A, FaceKey(7v1)), (B, FaceKey(3v1))],
  relation: SameOriented }
```

The hex prism and the box share a flush face pair the probe does not
declare, and the boolean now refuses an undeclared coincidence, as it
should. So the probe is stale, not the kernel: its premise ("builds")
predates the declaration requirement. Shapes: declare the flush pair
(the sibling batteries' `*_declared_*` spelling), or delete the two
detail probes if their batteries already carry the pose. Run:
`cargo test -p sweep --test all --release -- --ignored --nocapture --exact join1_r1_probes::join1_r1_hex_detail`.
