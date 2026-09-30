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
  hands its one merged member the chord between the endpoints the
  merge gives it (`EdgeCurveSpec::line_between`, `merged_chord_spec`),
  the scaffolding the surgery's struts and trims carry. That member is
  the upper meridian remnant, or the mate seam's rim-side piece. The
  final pass still states the slit as the band's seam, the meridian
  arc, through `attach_contact`. (The first push handed the arc under
  arc scaffolding; at the certified scalar that scaffold's residual
  enclosed wider than eps = 1e-12's band and the kill escalated, so the
  interval 1e-12 rows went red. The chord encloses at ulps.) Without
  this change these were 118 of the 129 sweep refusals S93 measured.
  At the head, `cargo test -p sweep --lib --test all` shows 1746
  passed and 7 ignored, at default eps and at 1e-12.
- `crates/sweep/src/revolve/full.rs` (CARVE's): the full-revolve
  zip's two kills take `kev_describing(he, &[], tol)`. Each one merges a
  copied vertex into its coincident original across a certified
  closing circle, and the merged fan keeps its carriers, which are
  re-certified under the run's band. The keys-only kill takes no band,
  so it would refuse these merges.
## 2026-09-24 — seam: TOPO re-worded one comment in `blend/surgery.rs`

TOPO's `half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`
(branch `topo/mint-rows-at-the-mint-site`) re-worded the comment above
the surgery's closing `mint_pcurves` in `crates/sweep/src/blend/surgery.rs`
("the input's caches are stale the moment the first strut lands"):
`mev`, `mef` and `mekr` now leave a face with complete rows complete
or rowless, never half-minted, so the pass is described as minting the
faces the surgery builds and re-deriving the rest. No code in the file
moved. The same unit's sweep run measured the fillet surgery's
"annulus mate trim mef" meeting states the closed-form lane cannot
mint mid-surgery; the operators leave those faces rowless for this
pass rather than refusing.
## 2026-09-27 — a note from ATREST: a P0 filed on CARVE's slate

Posted by the ATREST orchestrator so it is seen at CARVE's next sitting.
ATREST-4 (PR #3190, merged) widened tier 3's check 6 to planar loops
carrying arcs, and its review turned up a producer defect on CARVE's
ground: `work/carve/sweep-cap-plane-winds-against-a-convex-arc-region.md`.
A profile whose outer boundary carries a large CONVEX arc (the row's
C-shape, a 350° arc) passes `Profile::validate`, and `extrude`,
`loft_body` and a partial `revolve` all mint BOTH caps inside out —
`cap_points`' Newell sum over the vertices plus one apex per arc winds
against the region. Tier 3 certified these bodies until ATREST-4; it now
refuses exactly the two caps, pinned in `m5_s10_face_sense.rs` by rows
that go red when the verbs are fixed. The row carries the repro and the
shape of the fix (orient the cap by `profile`'s arc-exact winding).

Signed: (ATREST orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)

## Seam from TOPO, fix pass on PR 3161 (2026-09-29)

`Body::kev_merged_members(he)` is new and public: the merged members
of `kev(he)`, in the dying vertex's orbit order, each with the two
endpoints the merge gives it (`topo::MergedMember { edge, start, end }`,
`he_plus` forward order). It is read from the same plan and endpoint
reading the two kill doors certify against, so a caller no longer
re-derives what the merge will do. `kev`'s plan phase now also refuses
`OrbitBroken` where the dying vertex's orbit reaches a half-edge that
does not start there (two `next` tears could walk it through the killed
half, and the describing door then panicked); on a valid body nothing
changes.

**Your files, and what changed in them.**

- `crates/sweep/src/blend/surgery.rs`: `merged_chord_spec` now reads
  the member's merged endpoints from `kev_merged_members` instead of
  re-deriving them (its old rule, `start == dead`, agreed with the
  door only on valid bodies). It takes the refusal site name, so a
  read-door refusal is reported as `"rim closure kev"` or
  `"annulus closure kev"`. The chord it hands the kill is unchanged.
- `crates/sweep/src/blend/surgery.rs` `"rim kev"`,
  `crates/sweep/src/blend/open/planar.rs` `"corner kev"` and
  `crates/sweep/src/blend/open/ruled.rs` `"cap vertex kev"`: each is a
  spur kill, so the keys-only kill merges no fan. Each now says so and
  `debug_assert!`s it through `kev_merged_members`, where before the
  claim was only measured.
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `sweep/src/blend/surgery.rs`, `sweep/src/extrude.rs`, `sweep/src/lib.rs`, `sweep/src/loft.rs`, `sweep/src/revolve/full.rs`, `sweep/src/revolve/partial.rs`. The sweep walls state `wall_sense` in their spec (the post-mint `set_face_sense` calls are gone), caps state `true`, and the transient hole discs state `false` (their loop becomes the cap's ring). (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: In PR 3513 (branch `topo/every-escalation-names-its-decision`), `geom_brep::enters_material`, `enters_material_order2` and `classify_dihedral` return `LeverEscalation { rung: LeverRung, diag }` (the arm gate or the reading) instead of a bare `Indeterminate`, and a decided-zero arm carries its decided margin (`geom_core::k_stats::decide_positive_reported`) where it carried `INVALID`; `sweep::extrude` and `sweep::revolve::upgrade` read `.diag` and behave as before, a zero arm's payload now quoting its margin; the dropped rung is filed at `work/issues/section-arm-guards-escalate-untyped-and-certify-reads-the-dihedral-arm-as-transversality.md`. (TOPO implementer)
