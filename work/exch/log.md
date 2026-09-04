# EXCH log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/exch/plan.md`. A/B band 2100–2199
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose EXCH section is the
charter this plan restates. Opens now. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `step-import-degree-one-line-promotion` from `work/issues/`
- `step-import-curve-recognition-named-exclusions` from `work/issues/`
- `rational-patch-flux-quadrature-budget` from `work/cert/`
- `stl-header-refuses-plausible-names` from `work/lib/`
- `step-writer-hardcodes-user-header-fields` from `work/lib/`
- `epsilon-has-no-type-of-its-own` from `work/lib/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## EXCH-H1 cut and dispatched (2026-09-03)

First unit. `EXCH-H1` (`exch/h1-degree-one-line`) takes both halves
of `step-import-degree-one-line-promotion`: the certified line limb
in `recognize_curve` and the `ExtrudedPoint`/`PlacedSegment` rung in
`nurbs_iso_derive` — one unit, because a rung with no promoted line
has no witness and a promotion with no rung refuses strictly earlier
than today (the #327-measured result). Spec `docs/EXCH-H1-SPEC.md`
(M / NUMERIC, logged pre-draw). Recon corrections folded into the
spec rather than the plan: the zero-radius composite is prose-only
today (the limb builds it; `compose` needs no edit — radius 0 is
exact), dm1's tier rows are three cells since the 2026-08-13 audit,
and the "37 polyline carriers" is an uncommitted count Phase 1
censuses.

Seams, announced here since TRIM has no live orchestrator (opens at
CURVED's exit): `topo/src/pcurves.rs` edited at the one-arm rung seam
per both keep_outs (EXCH dispatched first; TRIM consumes the rung);
no edit under `geom-core/src/spline/` (S-CERT's glob — unit 2's
derivative channel will be filed as an S-CERT row when that unit is
cut).

Block EXCH-B1 drawn at dispatch; the draw record stays branch-side
(`exch/b1-block`) until the block concludes. Ordinal 2100 to be
claimed on main at review dispatch. Option-surface design (`[ev]`
items 4–6) is in discussion with Ev in-chat; no `[ev]` PR opened yet.

Cross-program note at dispatch: main's tour row was red from SHELL's
`Shelled` return with the fix (#1770) in flight — inherited, not
EXCH's; the lane merges main when it lands.

## EXCH-H1 re-scoped at Phase 1 (2026-09-04)

The lane executed Phase 1 as bound and stop clause 2 fired with the
right evidence: the spec mis-cited the blocker. Measured on the lane
(full battery differential against merge base): `nurbs_iso_derive`
needs no new arm at all — the missing limb is `run_iso_checks`'
seam-class `Curve3::Line` carrier limb in
`geom-brep/src/pcurve_cache.rs` (TRIM's file, Track Q rows riding),
without which the promotion regresses a first-class native arc-prism
round trip. Orchestrator's ruling, recorded as
`docs/EXCH-H1-SPEC.md` §"Re-scope at Phase 1": the announced TRIM
seam extends to exactly that one limb (the keep_outs'
whichever-dispatches-first rule in spirit; TRIM dormant until
CURVED's exit), Q's rows untouched. M / NUMERIC unchanged,
re-logged here. Phase-1 yield worth naming: dm1's degree-1 census is
now a measured table (37 carriers, √sup ∈ [1.9e-10, 6.2e-9] m
against eps_in 1e-5), and `#389`'s gap mechanism is a
control-order-reversed wall column — the adoption Line-column
candidate (the spec's disclosed contingency) is what hands it a
candidate.
