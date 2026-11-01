---
id: sweep-planar-cap-finders-spelled-per-suite
kind: issue
title: sweep tests: finding a planar cap by its height is spelled per suite (plane_face_at, plane_chart_at_y)
status: open
opened: 2026-09-28
priority: P4
cost: M
---


## Finding

Found by the `dup/b9-b` lane at merge base `2adacbc0e` while it moved
`verbs_shell`'s operands into `crates/sweep/tests/common/shell_operands.rs`:
the reader every shell row pairs with those operands — "the planar
face(s) at height `y`" — is spelled privately per suite, in two
variants.

- **`plane_face_at(body, z) -> FaceKey`** (the one cap at `z`, normal
  along `±z`, tolerance `1e-9`): `verbs_shell.rs` (`plane_face_at`),
  `offd2_r1_probes.rs` (`plane_face_at`), `shellfix1_bitdump.rs`
  (`plane_face_at_z`), and `sf2a_r1.rs` (an inline closure
  `plane_face_at_z`) — the same predicate four times, differing only in
  the panic. `p1b_r1_probes.rs::plane_face_at` shares the name and reads
  `origin.y` with no normal check — a different reader.
- **`plane_chart_at_y(body, y) -> Vec<FaceKey>`** (every planar face
  whose origin sits at `y`, tolerance `1e-12`): `verbs_shell.rs`,
  `verbs_shell_r2_probes.rs`, `verbs_shell_r2b.rs`,
  `shellfix1_r1_probes.rs` and `crates/sweep/tests/common/cert_corpus.rs`
  (private, used by `f64_only_corpus`'s ring).

`shell8_common::cap(body, shell, axis, value)` is the shell-scoped
generalisation of the second and may be the home both collapse onto.

**Blind spot.** Name-shaped (`fn plane_face_at`, `fn plane_chart_at_y`)
plus a grep of the `origin.z - z).abs() < 1e-9 && normal.x` predicate;
a reader spelled with another predicate order is unmatched.
