---
id: plate-ceiling-is-now-the-scaffold-pushforward
kind: issue
title: every measured document is bounded by carrier_matches_mapped_source — the carrier against the scaffold pushforward, door open and shut alike, and four identity residuals stand between the plate and a macroscopic real margin
status: open
opened: 2026-09-06
---


**Measured by M10-9, re-cut in its fix pass** (the registered-identity
door; branch `m10/m10-9-registered-identity`), on five documents: the
tour's two-hole plate at its real study (±0.05 mm spacing, σ = 0.01 mm
radii; `demos/tour` stop 1), R2's filleted bracket, R1's eccentric
annulus, R2's rounded-corner pad and R2's link.

## One predicate bounds all five, and it did so before the door

Read as the OVER-BAND SET at the refusing end of a 16-step bisection —
not as the single name a drive reports when it stops — exactly one
predicate is over the band on every one of the five documents, at
ε = 1e-6, 1e-9 and 1e-12, **door open and door shut**:

```
carrier_matches_mapped_source   [0, ~1.0001 · ε]
```

That is the whole finding, and it is sharper than the version it
replaces. The door discharges 40–60 decisions per document
(`carrier_endpoint_start` and `_end` outright), and the identities it
discharges **were never what bounded one**. The earlier reading — that
the plate's bound "walks one further with each registrant",
`carrier_endpoint_start` → `carrier_endpoint_end` →
`carrier_matches_mapped_source` — is an artefact of reading a drive's
first refusal at TWICE the ceiling, where several predicates are over
the band at once and evaluation order (validation before certification)
picks which name comes back. At ceiling + δ the plate is bounded by
`carrier_matches_mapped_source` with the door SHUT too.
(`m10_9_evidence_interval::m10_9_ceilings_with_and_without_the_door`
prints both scales side by side; the pad's version of the same table is
`m10_9_r1_probes_interval::r1_the_pads_over_band_set_at_multiples_of_its_ceiling`.)

## Why the arc carrier's builder cannot register this one

`carrier_matches_mapped_source` is the fenced SCAFFOLDING residual
(`crates/geom-brep/src/certify.rs`, the `Resolved::Scaffold` arm):

```
Margin::of( spec.carrier.eval(t_i).distance( mc.eval(s_i) ) )
```

with `t_i = sample_param(param_start, param_end, i)`,
`s_i = i / (CERT_SAMPLES − 1)`, `CERT_SAMPLES = 9`, and `mc` the
`MappedCurve::PlacedSegment` pushforward of the sketch segment through
the placement.

It IS a theorem of the same construction — the arc carrier is that
pushforward, re-expressed in the circle's own frame — but it is **not a
same-object identity**, and that is exactly the line ERROR-DESIGN E12's
reserve draws ("exact and simple but only for same-OBJECT identities;
the cosurface case is an expression identity"). Three separate reasons,
each sufficient:

1. **Two independently built objects.** The `Curve3::Circle` and the
   `MappedCurve`. The door aliases NODES; a registration must name the
   node the consumer asks about.
2. **There is no node until the certifier has chosen a sample.** To
   state it, a constructor would have to enumerate `CERT_SAMPLES` and
   reproduce `sample_param` and `i/(N−1)` — a registrant per SAMPLE
   rather than per identity. Visibility is not the obstacle
   (`certify::sample_param` is `pub`, exported precisely so two
   samplers over one schedule cannot drift); carrying the funnel's
   sampling schedule into a construction site is.
3. **The two spellings share almost no atoms.** `SketchSegment::eval`
   (`crates/geom-brep/src/mapped.rs:118-145`) is anchored on `a`, not on
   the centre — `a + (R − I)·v`, with `cos − 1` spelled `−2·sin²(sθ/2)`
   — a deliberate trade that keeps the enclosure tight and, as a side
   effect, spells the arc through `atan(bulge)` and `sin`/`cos` atoms.
   The carrier spells it through `azimuth::frame` and the sagitta
   closed form. The two normal forms meet only where the trig collapses
   — at `i = 0`, and that is exactly what the door reaches: **8 of 72
   decisions on the plate, one per curve**, and the same shape on the
   annulus (8 of 72), the bracket (8 of 99) and the pad (12 of 144).

M10-9 therefore stops here, as amendment A1 directs: the first bound
that is not a same-object identity of the arc carrier is named with its
predicate and its enclosure, and not widened into.

## How far behind it the next bound is: the staged walk

The residual can be discharged HYPOTHETICALLY — the staged-ceiling dial
(`k_stats::identity_pass_*`, the test-only `identity-pass-testing`
feature) passes a named predicate's indeterminate answer as `Zero`, so
"what would bound this document next" is a measurement. Passing the
plate's identity residuals one at a time:

| passed, cumulative | whole-certifying ceiling | × previous | over the band next |
| --- | --- | --- | --- |
| — (the shipped tier) | `7.812e2 · ε` | — | `carrier_matches_mapped_source` `[0, 1.0003·ε]` |
| + `carrier_matches_mapped_source` | `1.042e3 · ε` | 1.33× | `carrier_on_surface_2` `[−1.0003·ε, 1.0003·ε]` |
| + `carrier_on_surface_2` | `1.250e3 · ε` | 1.20× | `pcurve_map_residual` `[0, 1.0000·ε]` |
| + `pcurve_map_residual` | `1.562e3 · ε` | 1.25× | `witness_on_surface_2` `[−1.0002·ε, 1.0002·ε]` |
| + `witness_on_surface_2` | **`2.630e8 · ε` = 0.263 of the real study** | **1.68e5×** | `assert_bound` `[7.29e-9, 2.0e-4]` — a REAL margin |
| + five more identity residuals (`carrier_on_surface_1`, `witness_on_surface_1`, `carrier_endpoint_start`, `_end`, `witness_at_mid_parameter`) | `2.630e8 · ε` | 1.00× | unchanged |

(`m10_9_r2_probes_interval::r2_evidence_plate_ceiling_with_identities_passed`,
at the default ε; R2's row, re-run first-hand in the fix pass and
agreeing with R2's numbers to the digit.)


**The shape of that table is the finding, and it is not the obvious
one.** The first three residuals are worth 1.33×, 1.20× and 1.25× — 2×
between them. The FOURTH is worth **1.68 · 10⁵×**, and it only pays
once the other three are gone. Two things follow, and the second
decides the next unit's shape.

- **A per-identity mechanism cannot finish this.** The payoff is not
  spread across the family; it is entirely in the last member of it,
  and reaching three of four buys a factor of two. A door that
  discharges one identity per registrant — which is what M10-9 is —
  can only ever be on the 2× side of that cliff. The family has to go
  at once.
- **Behind the cliff there is a REAL margin, and it is macroscopic.**
  With the four passed, what stops the plate is the document's own
  `assert_bound`, enclosure `[7.29e-9, 2.0e-4]` — a study-scale
  assertion two hundred microns wide, not a widened identity. Passing
  five FURTHER identity residuals moves it by nothing (the table's last
  row), which is the check that the cliff is real and not the next
  residual in a queue. That is the honest end of this road: the tier's
  job is done when the identity residuals stop bounding the document,
  and 0.263 of the real study is where that happens on this plate.

## What is owed

- **The next unit's starting point is the FORM-LEVEL mechanism**, not
  another registrant. M10-9's spec forbids a form-level axiom store and
  was right to: this unit had to establish, by measurement, that node
  aliasing reaches what it reaches and no further. It does, and the
  measurement above says a per-identity door cannot finish the job.
  Whatever comes next has to state a relation between two EXPRESSIONS
  — the carrier and its mapped source — once, and have the tier carry
  it across every sample.
- **Or retire the scaffolding residual for arc carriers**, since the
  carrier and the mapped source are the same construction: D3's fence
  exists because a transient scaffolding edge has no surfaces yet, and
  the residual is what stands in for a description. That is a
  PCURVE/D3 question, not an E12 one, and it would remove the bound
  rather than discharge it.
- Either way the five documents re-measured, and this row re-cut
  against it. `work/m10/plate-ceiling-is-now-the-arc-span-identity`
  closed with A1 and stays closed.

## Two notes carried from review

- The ring table (§4 of the PR) closes
  `work/m10/plate-rim-residual-needs-the-wide-coefficient-ring` **on
  cost** — a wider coefficient ring buys no ceiling at 41× and 56× the
  leaf — and not on the question that row asked, which was whether the
  rim residual is reachable at all. The door answered that one instead,
  and by a different route.
- `line_span` and `contact_at_shared_vertex`, which the first cut named
  as the bracket's and the pad's bound, are themselves IDENTITY-shaped
  residuals of the fillet construction, not real margins; they are
  recorded as such in `work/m10/symbolic-tier-census`.
