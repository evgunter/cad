---
id: a-certificate-is-read-by-its-support
kind: unit
title: A certificate is read by its support: a chord's locus certificate is its section component's cut to its span, doors that keep the support carry it, the gate compares (PR 7182's ruling)
status: closed
opened: 2026-11-20
priority: P1
cost: H
refs: [a-ball-and-cylinder-boolean-costs-seconds-per-op-in-an-optimized-build, an-operation-retakes-each-carried-chords-rung-3-certificate-five-times]
branch: sect/certificate-by-support
closed: 2026-11-20
---


Filed by SECT's orchestrator, 2026-11-20. Parked on `[ev]` PR 7182, which
carries both designers' recommendation and the measurements (ball ∪ rod
on PR 7173's head: rung 3's locus certificate derived up to five times
per op on bit-identical support, the projected pcurve row three times;
≈2.4 s → ≈1.4 s estimated).

## The build, if ruled yes

1. A `LocusCertificate` (C2's limbs 2–3) split from the edge certificate,
   constructed only by the certifier, recording its support (carrier by
   `Arc` then bits, interval, unordered surface pair, band); restriction
   to a sub-interval is free.
2. A chord's locus certificate is its carried section component's
   `SsiCertificate` cut to the chord's span (check first that it covers
   the same limbs at the operation's band, and whether the chord keeps the
   component's carrier — the designers lean yes, exact restriction).
3. Doors that keep the support carry it (`restate_edge`, the partner copy,
   `kev_describing`, zip, graft, `certify_rebased_run`), re-running only
   placement checks; pcurve rows likewise (site → `mint_pcurves` → gate,
   `carry_rows`, split rows).
4. The gate compares support, then reads or re-derives. A test-profile
   pin re-derives every carried certificate and asserts bit-equality.
5. At `Sym`, equality means the same `Arc`; anything else re-derives.

Closes with it: `an-operation-retakes-each-carried-chords-rung-3-certificate-five-times`
and `a-fitted-chords-projected-pcurve-certificate-is-taken-at-three-doors-per-operation`
(on PR 7173's branch). Raise beside ONEHOME's carry rows.

## Closed (2026-11-20)

Built on `sect/certificate-by-support` except item 2: a carried
component's `SsiCertificate` is taken on the loop's base-opening net,
not on the net a chord's door reads, so the chord's locus certificate
is not a restriction of one already held. That question is
`a-chords-locus-certificate-is-not-its-section-components-cut-to-its-span`
(design). Each chord now takes rung 3 once, at its join door; doors
that keep its support read it. Split rows and split edges, whose spans
differ from their parent's, derive: that is the same restriction
question, filed there.
