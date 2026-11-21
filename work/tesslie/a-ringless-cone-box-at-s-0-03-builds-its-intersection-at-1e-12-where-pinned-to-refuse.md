---
id: a-ringless-cone-box-at-s-0-03-builds-its-intersection-at-1e-12-where-pinned-to-refuse
kind: issue
title: reach_probes: T1's box at s = 0.03, A ∩ B, builds at eps 1e-12 where the row pins CurvedPierceUnsupported; red on main
status: open
opened: 2026-11-21
---

`crates/mesh/tests/reach_probes.rs` `a_ringless_cone_face_near_its_apex_meshes_sound`
(about line 2002) pins T1's cone and its box at `s = 0.03` to refuse
`A ∩ B` with `CurvedPierceUnsupported` at ε 1e-12
(`refused_at(&[(1e-12, "CurvedPierceUnsupported")])`, line 2007). On
main `abbff25377` at ε 1e-12 the op builds instead, and the row fails
in `assert_ops` (line 1763: "T1's box at s = 0.03, A ∩ B: builds at ε
1e-12, where it refused CurvedPierceUnsupported"). It passes at 1e-9.

Found by the ONEHOME frames lane (PR 7131) running the 1e-12 suite on
its merge of main; the same row is red on main alone at the same ε, so
it is not that PR's. Unmeasured: which commit moved it, and whether the
built `A ∩ B` measures sound (the pin's sibling rows at `s = 0.03`
mesh), which decides whether the pin moves to a build or the refusal
comes back.
