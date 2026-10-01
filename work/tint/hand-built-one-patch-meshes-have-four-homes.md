---
id: hand-built-one-patch-meshes-have-four-homes
kind: issue
title: Four sites hand-assemble a one-patch Mesh, two on a defaulted FaceKey that check_mesh now reads
status: open
priority: P3
cost: E
opened: 2026-09-22
---



Found by TESS-4 (PR 3094), which touched one of the four and had to
retire a premise all four rest on.

**Where** — each assembles a `mesh::Mesh` from raw positions and one
patch of triangles, so a suite can hand it to a validator or a writer:

| Site | Shape | Face key from |
|---|---|---|
| `crates/mesh/tests/review_m2_pr6_checkmesh_audit.rs` `hand_mesh` | helper | the first face of `common::ball()` |
| `crates/mesh/tests/r1_probes_issue303.rs` `hand_mesh` | helper, same signature | the first face of a `prism_on` unit square |
| `crates/stl/tests/export.rs` `degenerate_mesh_is_refused_typed` | inline | `Default::default()` |
| `crates/viewer/src/scene.rs` `one_triangle` | inline | `FaceKey::default()` |

The two `hand_mesh`es are the near-twin pair: identical signature and
body, differing only in which real body they borrow a face key from,
and neither borrows it for any reason the suite states. (Not this class:
`crates/editor-core/tests/gui1_pick.rs` names a `&FacePatch` as a
return type — it reads patches, it does not build one.)

**The premise all four rest on, and it is gone.** Until TESS-4
`check_mesh` never read a patch's `face`, and the audit suite's helper
said so in its header. `MeshError::EmptyPatch { face }` reads it, to
name the face that emitted nothing. Nothing breaks today — none of the
four builds an empty patch — but two of them would report a defaulted
key as the culprit if one ever did, and the two that borrow a real key
borrow it for a reason that no longer needs stating the way it was.

**Why it is worth a home.** A shared builder over
`(positions, Vec<Vec<[u32; 3]>>)` would give one place to decide the
face-key question, and the suites that need a *multi*-patch mesh
currently cannot get one without writing their own — TESS-4 added
`hand_mesh_patches` beside the audit helper for exactly that, which is
a fifth spelling waiting to happen. Whoever takes it also decides
whether a defaulted `FaceKey` in a fixture is acceptable now that a
validator reports it.

Cross-crate reach is the usual obstacle: `mesh`, `stl` and `viewer`
cannot all name one test home today (the same link problem
`work/tint/tests-common-body-fixtures-triplicated.md` records for body
fixtures). This row is the `Mesh`-value twin of that one and is filed
separately because its subject is a hand-assembled VALUE, not a body
fixture, and no instrument that finds duplicated fixtures would find it.
