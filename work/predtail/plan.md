# PREDTAIL (plan)

Opened 2026-10-25 by PRED's priority-seam cut. PRED worked every
dispatchable P1 row it kept and was handed back `blocked` on
2026-10-26 with two parked P1 rows, and folded into this track the
same day (`docs/doc-ledger/pred-leaves-the-tracker.md`), so the class
is one track again. Each row PRED cut carries its 2026-10-25 re-census
section, measured against main `2d5677e1`. Start there, not from the
body above it.

## The slate

**32.5 budget points** of dispatchable work against a ceiling of 30.
The fold added none: PRED's two live rows are parked. The 2.5 over is
the six period-turn-rule follow-ups PRED filed on 2026-10-26, mostly E;
the next sitting splits along the P3/P4 seam if the slate grows.

| pri | item | cost | status |
|---|---|---|---|
| P1 | `cone-nappe-is-decided-in-five-places` | H, design | parked on PR 5500 (PCERT), which rewrites `pcurve_cache`'s nappe sites |
| P1 | `contact-verify-spells-the-orientation-and-second-order-margins-beside-their-homes` | M | parked on the D10 hold (`d10-one-way-to-say-intent-is-unbuilt`) |
| P3 | `band-derivation-has-a-scalar-twin` | M, design | open |
| P3 | `fixed-band-literals-are-an-unscoped-class` | H | open |
| P3 | `the-first-order-before-in-band-ordering-has-three-hand-written-homes` | M | open |
| P3 | `tol-and-band-travel-as-a-redundant-pair-past-the-offset-fit-doors` | H, design | open |
| P3 | `D46` | H | open |
| P3 | `the-aspect-cap-stands-in-for-the-sliver-criterion` | M, design | open |
| P3 | `f64-turn-rule-measures-headroom-against-f64-tau` | E | open |
| P3 | `harmonic-crest-in-span-has-spellings-beside-arc-window` | M | open |
| P3 | `parameter-span-levered-sites-outside-period-decide` | E | open |
| P3 | `pcurve-iso-arc-branch-decides-a-radian-gap-as-metres` | E | open |
| P3 | `projected-turn-reads-short-at-a-window-wide-azimuth-inf` | E | open |
| P3 | `spiric-turn-defect-inf-stays-global` | E | open |
| P4 | `D281` | M | open |

Three closed PRED rows are kept here because live rows elsewhere lean
on their content: `S65` (TESS's pinched-wall witness), `S66` (PROBE's
`S234`) and `period-headroom-margin-has-no-shared-home` (the
`solid_contain` period sites ENCL and CLEAVE hand to it).

## Order

1. The parked P1 rows first, the day their triggers fire.
   `cone-nappe-is-decided-in-five-places`: once PR 5500 merges,
   re-census the fourteen sites, then send the per-point disposition
   question (escalate vs fold) to designers.
   `contact-verify-spells-the-orientation-and-second-order-margins-beside-their-homes`:
   when the D10 hold fires, re-read `contact_verify` against the tree
   first; stage 4 may have rebuilt or deleted it.
2. The period turn rule's E follow-ups, as one short chain on one
   branch: `f64-turn-rule-…`, `parameter-span-levered-sites-…`,
   `pcurve-iso-arc-branch-…`, `projected-turn-…`, `spiric-turn-…`.
   Then `harmonic-crest-in-span-…`, which reads `ArcWindow` or keeps
   its own spelling with an agreement row, site by site.
3. `band-derivation-has-a-scalar-twin`, midpoint half. An infallible
   `Band::midpoint()` on `geom_core::predicate::Band` would collapse
   about 100 test sites mechanically, with an agreement row. Its pair
   half is a door question for the designers.
4. `D281`: a per-row read of 56 stale anchors under the S39 two-valued
   rule, converting each to a name anchor.
5. `fixed-band-literals-are-an-unscoped-class`: a per-site reading pass,
   cut into one unit per crate (geom-brep, topo and geom-core first).
6. `the-first-order-before-in-band-ordering-has-three-hand-written-homes`
   and `tol-and-band-travel-as-a-redundant-pair-past-the-offset-fit-doors`.
   Both are design questions with homes on D10-held ground. Take them
   after INTENT's stage 4.
7. `D46`, slice 2: the 188 unverdicted funnel names, cut by file in
   count order (its Slice 1 table). The `carrier_eq` and
   `contact_verify` names wait for INTENT's stage 4.
8. `the-aspect-cap-stands-in-for-the-sliver-criterion`: a designer
   question first (Ev, PR 5708).

## Review posture

PRED's, inherited: the repo-wide tiers
(`memories/orchestration-model.md`), named per unit at dispatch, in the
log. A unification that changes which margin decides a reachable answer
draws at least a full single review, because a wrong margin is
reachable and silent. Moving a predicate to a shared home is held to
the charter's agreement test.

No exit criteria are set: the program closes when its rows do.
