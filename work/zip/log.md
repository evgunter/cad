# ZIP — the log

## 2026-09-20 — opened

Cut out of REACH, which was carrying 151 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed REACH's PRIORITY seam per `work/README.md` "Track
size", and was made into several tracks at once so they can run in
PARALLEL — Ev, in chat: *"for these high priority tracks it's ideal to
have several components that can be worked on in parallel."*

7 rows arrived by `git mv` with their ids, bodies and history
unchanged. REACH keeps its band 6000-6099; band 7200-7299 claimed for
this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

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

**Your files, and what changed: `crates/topo/src/boolean/zip.rs` (`record_kev`) and `crates/topo/src/boolean/rest.rs` (the slit fuse's kill in `zip_folded`), the latter shared with TANG.** Each of these kills merges
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

Also `crates/topo/src/merge_faces.rs`, which TOPO and ZIP both claim.
The three new variants go into `OpPlacement::of`'s enum-verdict arm.
The `kev` row of its site table now says the fan-merge refusals cannot
fire at `strut_tip`'s site, whose far vertex has valence one.
