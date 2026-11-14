---
id: check-10-is-quadratic-in-shells-per-solid
kind: issue
title: check 10 pays n(n-1) point probes and n sign walks per solid of n shells, with no bounding-box prefilter and per-shell work linear in the face arena
status: open
opened: 2026-09-27
priority: P3
cost: M
refs: [3301]
---



Filed at ATREST-7's review (PR #3301, NOTE-1), measured by the reviewer
in a debug build: one solid holding an outer wall and **9 / 17 / 33 / 65
voids** validates in **58 / 193 / 708 / 2802 ms**, about ×4 per doubling.

## Where the cost is (`crates/topo/src/validate.rs`, `shell_winding_errors`)

- **n (n − 1) point probes** per solid of n shells, one per ordered
  pair (witness shell, other shell), with no prefilter. A shell whose
  bounding box definitely excludes the witness contributes `0` whatever
  its role (outside an `Outer`, and outside a `Void`'s cavity), so a
  box test decides most pairs of a porous part without a ray; only
  boxes that contain the witness need the walk.
- **Arena-linear per-shell factors**: `SolidFaces::of_shell` and
  `shell_vertices` each filter the whole face arena by back-pointer,
  so selecting n shells costs n × |faces|; the shared-key guard in
  `SolidFaces::guarded` walks the arena again per selection.
- **n sign walks** for the roles, each recomputing every face's flux
  from round 0.

## Reusing check 7's fluxes for the roles — not cheap, and why

Check 7's `plus_v_by_sign` stops each SOLID's walk at the round where
the solid's enclosure excludes zero, holding each face's run in the
returned `SignCertificate`. A shell's role could be read off the subset
of those runs belonging to the shell's faces — but (a) the solid's stop
round need not decide the shell's sign (a thin cavity inside a fat wall
is decided later than the solid is), so the per-shell read needs to
RESUME refinement from the held runs, which `sign_walk` has no entry for
(it always starts at round 0 over the faces handed in); and (b) the
runs are private to `props` behind `SignCertificate`, whose contract is
the whole-solid enclosure. A resumable `sign_walk` over a face subset of
a held certificate is the change, in `props` — worth it only once the
per-shell walks show up in a profile, which on closed-form (planar)
shells they do not: there the walk is one round.

## What a fix is

A bounding-box prefilter on the pairs (the box of each shell once per
solid; `Bounds` already brackets every face), a single arena pass that
buckets faces by shell for the selections and the witnesses, and — if
curved porous parts measure it — the resumable per-shell sign walk above.

- 2026-11-02 — moved from RESTSHELL to RESTFRONT by `git mv` when RESTSHELL merged into RESTFRONT, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and RESTSHELL had fallen far under it).

- 2026-11-09 — **a new, larger per-solid cost since check 10 reads every
  witness** (`onecopy/shell-winding`, PR 6670). Check 10 used to read each
  shell's winding at its first decisive witness; under the ratified
  every-vertex semantics it reads every vertex (`stands::witness_insides_everywhere`
  / `ladder_agreeing`), each a `point_in_solid_faces` ray cast. PR 6670
  already reads one witness rung at a time (vertices first, edges/faces
  only where no vertex places), which roughly halved it, but a valid
  multi-shell body still casts one ray per vertex per other shell rather
  than one.

  Profile (callgrind, `sweep` `tangent_charts_shell_onto_their_closed_forms`):
  `point_in_faces` 81% of the solid's validate, within it `cast_ray` 52%
  and `selection_reach` + `loop_extent_from` ≈40% — all per-(witness,
  shell) and q-dependent, so not cacheable across a shell's witnesses.
  Three `sweep` tests crossed the 0.5 s line and moved to the slow set in
  that PR: `tangent_charts_shell_onto_their_closed_forms` (0.43 → 1.17 s),
  `revolve_axis_gaps::the_start_vertex_and_the_authored_winding_change_nothing`
  (1.09 → 2.04 s), `shell7_r1_diff::shell7_r1_diff_corpus` (1.05 → 1.39 s).
  The sweep + editor-core `ci` total did not move (211 s either way).

  **The box prefilter this row proposes does not help these bodies.** Their
  faces are tangent charts / closed forms whose `boolean::boxes::face_box`
  cover does not build, so the box is poison and prunes nothing (measured:
  the per-witness side-at-infinity shortcut off the box never fired). The
  prefilter still helps porous bodies whose covers build; these need a
  different lever — a resumable per-witness cast that shares the probed
  shell's geometry across a shell's witnesses, or a census-style crossing
  detector run ahead of check 10 so a non-crossing shell is read at one
  witness and only a crossing one reads every vertex.
