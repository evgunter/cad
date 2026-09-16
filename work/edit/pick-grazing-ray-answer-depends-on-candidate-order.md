---
id: pick-grazing-ray-answer-depends-on-candidate-order
kind: unit
title: a grazing ray's pick answer is decided by the candidate order, not the geometry
status: review
opened: 2026-09-12
branch: edit/pick-grazing
pr: 2721
---



## The finding

`pick_face`'s exact test (`ray_triangle`, `crates/editor-core/src/resolve/pick.rs`)
can answer a `t` OUTSIDE the triangle's own box when the ray lies in
the triangle's plane: Möller–Trumbore's determinant is then ~1e-19
and `u`, `v`, `t` are rounding noise that can land inside the closed
acceptance. Seen on the gallery ring at PERF-9's tie-break row (a ray
along +y through a chord point of the tube): the triangle
`patch 0 tri 1` (det 2.1e-19) answers `t = 1.476` for a box whose
conservative entry is `t_enter = 1.480 − 4 ulp`; the true graze is at
the corner, `t = 1.480`.

What decides the pick today is the early-out, by ulp luck: a
neighbouring triangle's hit rounds to `1.480 − 6 ulp`, which is
strictly below the grazed box's `t_enter`, so the loop breaks before
the garbage `t` is tested and the answer is the corner. A reference
over the same tree with no early-out answers the garbage `t` (a point
0.004 below the corner, on a face the ray does not cross there). So
for a grazing ray the answer is a function of the candidate ORDER and
of which neighbour rounds which way, not of the geometry.

PERF-9's two-level index reproduces the single-tree candidate
sequence exactly (`MeshPick::candidates`), so it answers the same as
today, garbage-luck included, and `viewer`'s `index_memo` reference
loop keeps the early-out for that reason. The defect is upstream of
the index: the exact test needs a guard consistent with the pruning —
refuse a hit whose `t` is below its box's `t_enter` (a true hit never
is: the entry is a certified lower bound), or refuse near-parallel
determinants — so the answer becomes a function of per-triangle tests
alone. Either changes today's answers on grazing rays; the pin to
re-baseline is `index_memo`'s reference loop, whose early-out then
becomes redundant rather than load-bearing.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/edit/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): EDIT is DOCM's successor on the document-model ground (persist, the edit vocabulary, the node and resolver doors). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Built (2026-09-16)

`ray_triangle` (`crates/editor-core/src/resolve/pick.rs`) is now the
exact test AND the box-entry guard, one public predicate over
`(ray, corners, t_enter)`: a `t` strictly below the candidate's own
box entry is refused on the bits, no tolerance. `pick_face` calls it
with each candidate's `t_enter`; its early-out is kept for cost and
documented as unable to change the answer (every accepted hit is at
or above its own entry, candidates ascend in entry). `viewer`'s
`index_memo` reference loop drops its early-out and its private copy
of the test, calls the shared predicate, and asserts on every ray of
every landing that the pruned walk and the reversed walk answer the
reference's triangle at the reference's `t` bits (acceptance 2). The
ring's grazing ray is a probe that was red on the no-early-out
reference (`t = 1.476` on patch 0) and answers the corner at
`t = 1.480`. `crates/bvh/tests/ray.rs` gained the entry-bound row
against exact dyadic true hits (acceptance 5).

Measured: 83 answers moved over the corpus landings — 74 were noise
hits (determinant < 1e-15) replaced by the true hit; 9 were genuine
near-tangent grazes whose rounded `t` fell 1–15 ULP below their box's
entry, each answered by a sibling triangle at the same point within
ULPs. No aimed graze was lost by the guard; the 273 aimed rays that
miss their point on `main` are unchanged and filed as
`pick-closed-acceptance-loses-a-graze-to-rounding`. The residual
order dependence the spec asked to measure (a noise `t` above its
entry, inside the box) did not occur over the corpus, so the
determinant bound was not added.
