# CARVE — log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/carve/plan.md`. A/B band 5600–5699
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-17) — opened in BLEND's cut

Opened by BLEND's orchestrator as BLEND's one successor on BLEND's
ground, per `work/README.md`: *residue is re-homed before the sweep …
to a new program opened for it when the residue coheres into a track
of its own (a dozen items on one territory are a successor's opening
slate, and the closing program opens it)* (Ev, 2026-09-06), applied as
VIEW applied it to itself on 2026-09-17: the parent stays open, the
rows move by `git mv`, each track's charter is the sentence true of its
rows and false of the others'.

**BLEND is not closed by that act and is not closed by this one.** Its
unit order is landed to unit 12, unit 14 and unit 15 are at their gates
and unit 13 is walked as blocked; its thirty-two live residue rows were
review accretion on one crate and its neighbours, and twenty-one of
them cohere into this track. BLEND stays open with its three unit rows,
its exit walk is a separate `[ev]` step, and `docs/DOC-LEDGER.md`
records the sweep when it happens.

21 rows arrived from `work/blend/`, each by `git mv` with its body,
its id and its history unchanged — no row's prose was edited on the way
past, and the item schema carries no `program:` field, so a re-home is
the move and nothing else. The same cut placed eight rows on PATHS (the
profile fillet door), one on SYM, one on TOPO and one on META;
`work/blend/log.md`'s cut entry carries the table and the charters.

Nothing dispatches until the opening sitting writes the unit order.

## BLEND closed (2026-09-17)

BLEND's exit walk (PR #2826, "merged!" from Ev in chat) closed the
parent the day this program opened; the sweep (`docs/DOC-LEDGER.md`
sweep 17) deleted `work/blend/`. Four rows arrived with the walk —
`S90-impl` (BLEND's unit 13, blocked on PROPS' `H5`),
`contact-edge-arm-is-picked-from-the-carrier-kind-not-the-dihedral`,
`ruled-band-keys-a-d-hole-rim-on-the-caps-outer-cycle` and
`corner-config-recourse-and-policy-assert-a-default-for-any-tag` — so
the slate is twenty-five rows. Every path in `paths` is this program's
alone now; the "BLEND stays open" clause left `keep_out`.

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

**Your files, and what changed in them.**

- `crates/sweep/src/blend/surgery.rs` (BAND's and CARVE's): the two
  closure kills take `kev_describing`. `rim_phase`'s is
  `"rim closure kev"` and `rim_phase_annulus`'s crossing kill is
  `"annulus closure kev"`; the refusal site names are unchanged. Each
  hands its one merged member the band's meridian arc under arc
  scaffolding (`EdgeCurveSpec::arc_of_circle`), using the centre and
  radius the description pass already reads for the slit. That member
  is the upper meridian remnant, or the mate seam's rim-side piece.
  The final pass still states the slit as the band's seam through
  `attach_contact`. `attach_contact`'s arc construction moved into
  `short_arc`, which the new `merged_meridian_spec` shares. Without
  this change these were 118 of the 129 sweep refusals S93 measured.
  At the head, `cargo test -p sweep --lib --test all` shows 1501
  passed and 8 ignored, as on the merge base.
- `crates/sweep/src/revolve/full.rs` (CARVE's): the full-revolve
  zip's two kills take `kev_describing(he, &[], tol)`. Each one merges a
  copied vertex into its coincident original across a certified
  closing circle, and the merged fan keeps its carriers, which are
  re-certified under the run's band. The keys-only kill takes no band,
  so it would refuse these merges.
