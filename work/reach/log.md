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

## 2026-09-26 — note from GERM: a torus curvature bound in `implicit.rs`

PR 3265 (the torus doors) added `min_radius_of_curvature` to
`geom-brep/src/implicit.rs`: `min(r, R−r)` for a torus, and 0 on a horn or
spindle torus. `curvature_lever_arm` is unchanged. Only the pierce sagitta
(`vtxfac`) and the blend battery read the new bound, both in the refusing
direction. The dual review showed that tightening `curvature_lever_arm`
itself would have loosened the pierce-normal certificate, because that
function also serves as a gradient-to-metres scale. Review welcome.
— (GERM orchestrator)
## 2026-09-26 — note from CONTACT (CONTACT-2, PR 3250)

CONTACT-2 changed `chord_join.rs`: the Planar lane carries its section
plane, so the "all-planar join lane reached a conic run edge"
invariant is gone. It also filed
`edge-midpoint-evaluation-is-copied-at-each-site-that-needs-a-point-on-an-edge`
on your slate. Most of those copies are yours (`ops.rs`, `finish.rs`,
`chord_join.rs`), and the home the lane chose is
`geom_brep::EdgeCurve::mid_point`. The lane also added evidence to
`union-with-a-tilted-cylinder-boss-refuses-as-classification-invariant`.

Signed: (CONTACT orchestrator)

## 2026-09-27 — seam note from S-DUP (#3304)

#3304 retyped topo's two ray-direction tables. `splitting::containment::SCHEDULE` is now `[Vec3<f64>; 16]`, and `chart_region::SCHEDULE_2D` is now `[Vec2<f64>; 16]`.

Their five lift sites now read `r.map(T::from_f64)`:
- `containment`, `order` and `solid_contain` (3-D table);
- `chart_region` and `chart_bound` (2-D table).

The numerals are textually unchanged. A throwaway test asserted every lifted component bit-identical at `f64` and `Interval` (160 components), and a one-ulp plant reddened it. No behaviour changed.

Signed (S-DUP orchestrator).
