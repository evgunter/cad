---
id: check-10-is-quadratic-in-shells-per-solid
kind: issue
title: check 10 pays n(n-1) point probes and n sign walks per solid of n shells, with no bounding-box prefilter and per-shell work linear in the face arena
status: open
opened: 2026-09-27
priority: P3
cost: M
refs: [tier-3-does-not-check-shell-roles-per-solid]
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
