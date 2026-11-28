---
id: the-gathers-finish-re-derives-certificates-its-moved-members-carry
kind: unit
title: The gather's finish re-derives from values the certificates its finished, moved members already carry: a product turned by a boxed joint refuses at plain Interval, and a member's volume is re-read at Sym
status: review
opened: 2026-11-24
priority: P1
cost: H
branch: boxfinish/gather-carries
pr: 7723
---


Cut from
`a-chains-link-refuses-at-an-edges-stored-curve-with-the-box-arm-off`
(M11, M12) by two designers who weighed it independently and agreed at
first report (2026-11-24; their reports are on `boxfinish/briefs`,
`probe/briefs/fork-plain-interval/`), with the premise they corrected
confirmed on M12's trace.

**What happens.** The tour's chain is a product: a bar and three pins,
each extruded from constants, each placed by a joint. Every member's
own finish runs over a zero-width box and passes, at `f64` and at
`Interval` alike. `FinishedBody::moved` carries a finished body through
a rigid motion and asks only the session box. The gather (`⊔`:
`wire_assemble`, `topo::boolean::graft_placed`) then takes the moved
images as construction-state data, grafts them into one body, and ends
in `T::leaf`, which with the switch's box arm off is the whole of
tier 3 on the aggregate. Check 2 certifies every edge again from
values: the moved carrier's start against the moved vertex, two
enclosures of one point, each as wide as the joint's box.

- At plain `Interval` nothing can decide that: 24 of 30 edges refuse
  at `carrier_endpoint_start` (the bar's 12 and the two turned pins'
  12; the unturned pin's 6 pass), with 48 planar incidence escalations
  beside them. The tour ends there.
- At `Sym<Interval>` the tier proves again what the motion had
  guaranteed, and the finish stops later re-reading ONE MEMBER's
  volume (`SolidKey(1v1)`, six aimed faces: the bar,
  `[-1.08e-7, 2.53e-7]`).
- At `f64` it is redundant work (the cost row's "check 2 repeats").

**The agreed final state.** DESIGN `[one-way.places]` already says it:
a certificate is a fact about its support, read wherever that support
is kept and derived wherever it is not; an edge's or pcurve row's
certificate is carried by every door that keeps its support; a
finished body moved by an isometry stays finished. The code falls
short of it at the gather.
1. The gather is handed its members as finished bodies under their
   motions, so that "the same support up to this isometry" is known by
   construction, not by comparing values.
2. Its finish reads every certificate a member carries (the edge
   certificate's endpoint checks and the planar pcurve incidence
   checks included, and a member's volume sign) and derives only what
   the merge adds: the questions between members, the aggregate's own
   shell roles, the session box.
3. Re-deriving a carried certificate is a kernel-bug check
   (`[failure.taxonomy]` row 5): a `debug_assert!` at the point
   scalars, nothing at the box scalars, where it cannot succeed
   (`[simple.checks]`).

No ratified text changes. Plain `Interval` gets no leaf of its own and
no way to answer an identity: where a box changes a SHAPE (a boxed
dimension in a profile), a plain-`Interval` finish refuses, which E12
names as the reason the symbolic tier exists. Whether the tier-off
dial (`SymbolicDials::off`, `LeafLane::Numeric`: built only by tests
and the tour's chain table) should remain is not decided here.

**Measure first.** That the graft's mapped data is bit-identical to
`member.moved(placement)`'s; what tier 3 asks of the aggregate that no
member's finish answered (the set the gather must still derive); what
check 2's existing "locus certificate read where its support is
unchanged" compares, and why a boxed motion's image never matches.

**Done when.** With the switch's box arm off: the tour's one-link chain
certifies at plain `Interval` with tier 3 run, and the command runs to
its end; at `Sym<Interval>` the aggregate's finish re-reads no
member's volume; a row that a gather of finished members under boxed
motions finishes at `Interval`; a row that a member NOT finished
(construction-state data) is still derived in full; the corpus's
finish count re-measured and re-pinned.

Letter H, dual review (a concurrent pair): it changes what the at-rest
gate derives for every product and pattern, and a certificate carried
where its support was not kept is a false certificate.
(BOXFINISH orchestrator)
