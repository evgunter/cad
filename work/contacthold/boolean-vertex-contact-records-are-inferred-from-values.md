---
id: boolean-vertex-contact-records-are-inferred-from-values
kind: issue
title: The boolean records vertex-level contacts (VvContact/VfContact) from Zero verdicts with no declaration, and tier 3′ (ii) calls them declared — intent inferred from values, which tier 3′ (i) forbids
status: parked
opened: 2026-10-03
priority: P1
cost: M
design: true
blocked_on: [contact-records-cite-their-decision]
---


Filed by CLEAVE (2026-10-03), from the split-band-on designer pair's round 2.

## Read, not yet run

Ev's ruling: D1 tier 3′ (i)'s "near-coincidence NEVER silently becomes
contact" covers margins at or below ε, and intent cannot be inferred from a
value coincidence. Recorded in
`work/cleave/split-band-on-at-a-concave-edge-may-mint-a-pinch-from-near-coincidence.md`.

`crates/topo/src/boolean/reduce.rs` mints `VvContact` and `VfContact` from
a Zero verdict with no declaration: `push_vv`, and `vertex_on_face` into
`contacts.vf`. The result carries the survivors into `BooleanBody`, and the
pseudomanifold door then passes them as "declared-contact records". Tier
3′ (ii)'s text says exactly that: "the ON-set survivors".

Both designers named the same witness, read from the test and not run:
`m3_pr6_tier3prime::corner_kiss_promoted`. Two bricks kiss at one corner
with no declaration (`flush_declarations` declares flush faces only), and
the union carries one v-v record and passes at rest.

At face granularity the boolean already refuses undeclared coincidence
(`UndeclaredCoincidence`, `refuse_undeclared_continuations`).

The line the split pair drew applies here too:
- a Zero event that is consumed into structure (fused, or a crossing)
  only places topology, and is fine;
- one that survives into the result as a record is touching, and needs
  backing.

C4's declaration vocabulary is face-pair only (`FacePairDeclaration`), so
a corner resting on a face has no place to be declared. The boolean's
vertex copies (`copies_of`) are the same case.

## Owed

1. Measure. Run `corner_kiss_promoted` and a box whose corner rests on
   another box's face with `BooleanDeclarations::none()`, and list every
   test that relies on undeclared vertex records.
2. Then a designer pair, on these questions:
   - Is there a vertex- or edge-level declaration place? `CarriedVv` and
     `CarriedVf` are the existing shapes.
   - What does a face-pair declaration cover?
   - How do tier 3′ (ii)'s "ON-set survivors" words change? They are
     ratified text, so that question goes to Ev.
3. Audit the other doors that turn a Zero into distinct cells on one
   point. A revolve whose profile meets its axis twice is a candidate.

## Parked on the D10 hold (2026-10-08)

D10 answers its design question: a Zero coincidence is recorded at the one door and linted. Likely closes when that unit lands. (CONTACT close-out triage; CONTACT's log (`docs/doc-ledger/contact-leaves-the-tracker.md` names the SHA it is read at).)

## Re-parked on B2 (2026-10-10)

Its trigger, stage 4 B (`coincidences-are-recorded-at-one-door`, PR
4354), fired, but B left `ContactRecords` as it was (the S4-B ruling,
option (b)); the record this row is about is rewritten by B2
(`contact-records-cite-their-decision`, live on
`intent/s4-b2-records-cite`), which makes every row cite its
`Coincidence` across ~99 files. Building on the record's shape while
B2 rewrites it would collide and be built twice. Re-read it against
B2's merged record. (CONTACTHOLD orchestrator)
