# TANG log

## Opened at CURVED's cut (2026-09-20)

Opened by CURVED's orchestrator on Ev's in-chat direction. Eight items
moved from `work/curved/`: the declared-tangency lane, the germ and
pierce lane and the pinch design, none dispatched by CURVED (each was
waiting on a design conversation). Band 6100–6199 recorded in the
ledger's banding entry in the same commit. The first sitting opens
with `m9-3-semantic-residues` items 4–5.

## 2026-09-21 — a note from ATREST: #1076 is load-bearing for two ATREST rows

Posted by the ATREST orchestrator. TANG's slate has no way to see this
and it may change what `arc-aware-point-in-loop` is worth.

**`work/tang/arc-aware-point-in-loop` (#1076) is the keystone of two
of ATREST's P0/P1 rows**, not only of its own consumers:

- `work/atrest/check-9-nesting-is-line-bounded-only` — check 9's
  nesting half is silent on every arc-bearing outer loop, which is the
  annular rim of every shelled vessel of revolution. Two of its three
  thirds (`ArcParity`, `NoWalk`) wait on #1076 outright; only the
  `Disc` class can be closed without it.
- `work/atrest/validate-tier3-curved-boundary-containment` — the last
  unmarked deferral in tier 3's not-yet-checked list. Its
  region-bounding half wants the same thing: a walk that can express a
  loop's region when the loop is not a polygon.

Read from the tree rather than assumed: `splitting::containment::point_in_loop`
takes a `normal` and walks the polygon through the loop's VERTICES, and
`boolean::solid_contain` is the 3-D promotion of that same walk — same
`SCHEDULE` const, and it calls `point_in_loop` per face. So the
polygon assumption is not local to one arm; it is the floor the
containment layer stands on, and #1076 raises that floor for every
consumer above it, ATREST's tier-3 arms included.

**Nothing is asked of TANG here and nothing is blocked today.** ATREST
is not waiting on a date: its check-9 unit will take the `Disc` third,
which needs no arc-aware walk, and will state its `ArcParity`/`NoWalk`
residue as waiting on #1076 by name. This note exists so that when
TANG prices or sequences #1076 it knows the row has consumers outside
its own program — and so that if TANG ever considers narrowing or
deferring it, ATREST hears about it rather than discovering it.

If TANG would like ATREST to take any part of #1076 on its own ground,
say so on `work/atrest/log.md` or ping the tag; `crates/topo/src/splitting/`
is not ATREST's territory and this note is not a claim on it.

Signed: (ATREST orchestrator)

## Announced seam from TOPO (2026-09-24)

TOPO's `kevs-fan-merge-needs-a-re-describing-kill-door` (branch
`topo/kev-describing-door`; PR title "TOPO: kev refuses a merge that
would strand a carrier; kev_describing takes the re-descriptions")
lands Ev's ruling (c) on PR 2527. `Body::kev` stays keys-only and now
refuses, before mutating, a fan merge that would re-base a certified
edge onto the surviving vertex (`EulerOpError::MergeRebasesCarriers`,
naming every such member) or move one end of a null edge
(`RebasedNullEdge`). It carries a merge that moves nothing: an empty
fan, or a killed null edge. `Body::kev_describing(he, &[(EdgeKey,
EdgeCurveSpec<T>)], tol)` is the kill that takes a band and the merged
members' re-descriptions. A listed member is certified at the merged
endpoints. An unlisted one passes the re-basing gate `mev`'s fan site
passes. `kev_describing(he, &[], tol)` is the kill with a band and
nothing re-described.

**Your file, and what changed: `crates/topo/src/boolean/rest.rs`, shared with ZIP: the slit fuse's kill in `zip_folded`.** Each of these kills merges
two vertices that the section or the fuse put a band apart, across a
certified circle, so the merged fan's carriers still end where they
land. Each kill now takes `kev_describing(he, &[], tol)`, which
re-certifies every member under the run's band. The keys-only kill
takes no band, so it would refuse these merges. Measured by
instrumenting `kev` across `cargo test -p topo` and
`cargo test -p sweep` on the merge base: every member at these sites
re-certifies within band, including the zip's 58 ulp-distinct merges.
The strut-undo kill in `boolean/rest.rs` kills a null edge, whose two
vertices hold one point, so it stays keys-only.
