# REACH log

## Opened at CURVED's cut (2026-09-20)

Opened by CURVED's orchestrator on Ev's in-chat direction (break the
two programs into smaller tracks; leave in CURVED and TRIM only a chunk
sized to finish this session; no `[ev]` PR needed, it is mostly moving
issues around). Forty-nine items moved from `work/curved/` — S-BOOL's
residue re-homed onto CURVED at S-BOOL's 2026-09-16 exit, plus
CURVED's own operand-reach, merge-door and census lanes. Band 6000–6099
recorded in the ledger's banding entry in the same commit. No unit
dispatched; the first sitting picks from `work/reach/plan.md` §Lanes.
Two items carry orchestrator context worth knowing: `rest-zip-seam-chord-on-cylinder-wall`
was filed by the merge-door unit's STOP-2 re-scope (#2105 — the
chord pre-exists on the merge base; both reviewers executed the
honesty crux), and `the-chord-dip-charge-has-two-homes` was half-fixed
by the torus arm (#2535; the `geom-brep` spellings collapsed, the
`boxes.rs` home is the remaining half).

## Rows arriving from FIX, 2026-09-20

Ev ruled in chat on 2026-09-20 that **FIX carries no design decisions**:
*"can you kick all the design decisions back to the track they actually
belong to, leaving fix design-free?"* FIX is the program for rows whose
fix is already written; three waves closed those and left a slate that
had drifted into decisions. Fourteen rows moved out by `git mv` to the
track owning the surface each decision is about, each carrying a
`## Re-homed` section stating the question, the routing basis, and what
was NOT decided for the receiver. Routing was taken from
`python3 scripts/work.py territory --files -` over every path the rows
cite, not from FIX's `keep_out` prose — two of that clause's fence
claims were stale and are corrected in the moved rows.

**One row: `graft-recertifies-through-the-narrow-lane`.**
`boolean::combine::graft_solids_with` re-certifies every grafted carrier
through the plain `geom_brep::EdgeCurve::certify`
(`crates/topo/src/boolean/combine.rs`, in the curve pass), so a grafted
edge of the M7-8 class — an `Intersection` between a plane and a DESCRIBED
NURBS wall — would refuse `CertifyError::Unimplemented`, surfacing as
`BooleanError::GraftRecertify`. The split is stated at neither door.

**What was explicitly NOT established, by the lane that filed it:**
whether a body of that class ever reaches the graft. The boolean pipeline
has a NURBS re-gate ahead of it, so this may be a latent split rather than
a live refusal — which is the first thing to settle and may close the row.

**The bound cannot simply be raised.** `graft_solids_with` is
`T: geom_core::Decide` under `boolean_op_with`, which `verbs::Verb`'s
blanket impl runs and the dual corpus instantiates at `Dual64`; no `Dual`
implements `CertifiedEnclosure`, so tightening the chain does not compile.
The remedy PR 2418 took at the sibling door was `certify_via` with the lane
as an argument. That sibling,
`plain-transform-rigid-still-refuses-the-m7-8-class`, went to SHELL in
this sweep under the same constraint and the same ratified discriminator
(`crates/geom-core/src/real.rs`, Ev 2026-08-29: *"the discriminator is
that nothing generic calls this door"*). **Whichever of you rules first,
the other should read that ruling rather than re-derive it.**

Also arriving nearby: CENSUS took
`a-new-kind-pair-arrives-unguarded-by-default`, whose second half asks
whether your `boolean/mod.rs:2878` guard earns its share of ~230 unfired
lines. That half wants your assent, not an announcement.

Signed (FIX orchestrator).

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

**Your file, and what changed: `crates/topo/src/splitting/reassembly.rs`: the reassembly oracle's two zip kills.** Each of these kills merges
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
