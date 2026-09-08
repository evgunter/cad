---
id: first-refusal-at-twice-the-ceiling-is-an-order-artefact
kind: issue
title: every "what bounds this document" statement from M10-7 on was read at twice the ceiling, where evaluation order picks the name
status: open
opened: 2026-09-06
---

**Found by M10-9's fix pass**, by re-measuring what two reviews
disputed. It is a finding about the INSTRUMENT, and it reaches back
past M10-9.

## The instrument

`crates/editor-core/tests/m10_8_arc_family_interval.rs`, `ceiling`:

```rust
let (lo, hi, _) = crate::m10_8_harness::ceiling(doc_at, dials.rules, tol, 1.0e-14, 1.0e3, 40);
…
// What refuses first beyond it, from a whole-box replay at twice
// the ceiling.
let doc = doc_at(lo * 2.0);
```

The BRACKET is bisected to 40 steps, and then the "first refusal beyond
it" is read at **twice** the widest certifying scale — a scale the
bisection has already shown to be far past the boundary. M10-9's own
first cut copied the shape.

## Why that answers the wrong question

A drive stops at its FIRST refusal. At a scale well past the ceiling
several predicates are over the band at once, and which name comes back
is EVALUATION ORDER — profile validation runs before certification, and
within certification the sample schedule is walked in order. So the
name is a fact about the drive, not about the document.

Measured on five documents (the tour's two-hole plate, R2's filleted
bracket, R1's eccentric annulus, R2's rounded pad, R2's link) at
ε = 1e-6, 1e-9 and 1e-12, door open and door shut — the whole
over-band SET, with enclosures, at the refusing end of a 16-step
bisection
(`m10_9_evidence_interval::m10_9_ceilings_with_and_without_the_door`):

| read at | plate | bracket |
| --- | --- | --- |
| **ceiling + δ** | `{carrier_matches_mapped_source}` `[0, 1.0001·ε]` | `{carrier_matches_mapped_source}` `[0, 1.0000·ε]` |
| 2× the ceiling | `carrier_endpoint_start` (door shut) / `carrier_matches_mapped_source` (open) | `line_span` `[-1.0969·ε, 1.0964·ε]`, in VALIDATION |

The bracket's 2× reading is the sharpest case: at twice its ceiling the
replayed profile fails VALIDATION on `line_span` and the certification
predicates are never asked at all. That is a different refusal, not a
later one.

## What it reaches back to

- **M10-9's first cut** reported that the plate's bound "walks one
  further with each registrant" and that R2's bracket and pad were
  bounded by `line_span`, a real margin. Neither is true at ceiling + δ:
  all five documents are bounded by `carrier_matches_mapped_source`,
  door open and door SHUT, and `line_span` is itself an identity
  residual (`work/m10/symbolic-tier-census`). Corrected in this pass.
- **M10-8's own conclusion**, which is what OPENED M10-9: "the two-hole
  plate's real study is still bounded at `7.81e2 · ε` by
  `carrier_endpoint_start`" (`work/m10/M10-9.md`). Read at ceiling + δ
  under M10-8's tier exactly (`SymRules::shipped_without_the_door`),
  the plate is bounded by `carrier_matches_mapped_source` there too.
  The unit was opened on an order artefact.

  That does not make M10-9 wrong work: the rim and span identities are
  real, they were genuinely stopping drives just past the ceiling, the
  door discharges them, and the door is the mechanism E12 reserved. It
  does mean the unit's RESULT is sharper than "no ceiling moved" — the
  identities it discharges were never what bounded a document — and
  that a family of documented "bounded by X" statements from M10-7
  onward has not been re-read.

## What is owed

1. **The harness reads the SET at `hi`.** `m10_8_arc_family_interval::ceiling`
   should replay at the refusing end of its own bracket and report every
   predicate over the band with its enclosure, not one name at `2·lo`.
   M10-9 built that instrument
   (`m10_9_evidence_interval::over_band_set`) and did not move M10-8's
   harness under it, because doing so re-cuts M10-8's published tables
   in a fix pass for another unit. A doc-comment warning is on the
   M10-8 helper naming this row.
2. **Re-read M10-7's and M10-8's "bounded by" statements** against the
   set at ceiling + δ, and re-cut `docs/K-REPORT.md`'s M10-7/M10-8
   sections and `work/m10/symbolic-tier-census` where they differ. The
   NUMBERS (the ceilings) are unaffected — those are bisected, not
   read off a drive — so this is a re-cut of predicate names and of the
   conclusions drawn from them.
3. Whoever takes it should also decide whether "bounded by" is worth a
   named helper in the harness, so the next unit cannot spell it the
   cheap way.
