# Torus×torus rim tangency (issue 968 item 3) — design conversation

**STATUS: DRAFT — for Evan.** The pre-implementation ruling MATE-7
needs (the Q4 ruling scheduled the torus declared-Rest lane last
and sent this disposition back separately, per the issue-966
record). MATE-7's other two needs — the torus operand gate under
covered declarations and the `carrier_eq` torus rung — are
implementation once this is ruled.

## The question

The cross-operand torus×torus tangency at a shared rim circle:
lily wall 1's stem glue meets it, the DEV-1 witness loci
(`tangent_locus`: plane×cylinder, parallel cylinders) do not cover
it, and the #966 thread left two candidate shapes:

- **(a)** CurveContact-granularity declared `Tangent` with a GROWN
  witness lane — `tangent_locus` gains a torus×torus arm at the
  shared rim circle (certified witness growth, NUMERIC class);
- **(b)** the wedge = π smooth-seam treatment — the rim is a smooth
  seam of the composed body, M9-3 PR-B's lily-tube-chain fixture
  pattern, no new witness lane.

## What changed since #966: the wedge table is live

MATE-3 (PR #1423) landed the #131 ruling's material-wedge verdict
table in tier 3, reading C3 `CurveContact` records as `Tangent`
claims: transverse and π legal; wedge 0/2π legal iff declared and
jet-determinate; in-band κ_rel escalates. So the vocabulary that
DECIDES what a rim contact is — smooth seam vs cusp pair — now
exists and is enforced, which the #966-era conversation did not
have.

## Proposal: the table decides; (a) and (b) are its two arms

Rule the disposition as a CASE SPLIT the landed table already
performs, rather than choosing one shape for the whole class:

- Where the composed material is SMOOTH across the shared rim
  (wedge π — the stem-corm weld's actual geometry, the tube-chain
  precedent), the join reconciles the rim as the **(b)** smooth
  seam: no declaration needed, no witness lane, the table's π row
  is the acceptance.
- Where material genuinely wedges to 0/2π at the rim (a kissing
  torus pair), the contact is the **(a)** declared-`Tangent` cusp
  family: the declaration is REQUIRED (never inferred — the C7
  rule), κ_rel jet-determinacy is the table's own condition, and
  the grown `tangent_locus` torus×torus arm supplies the certified
  witness the verification consumes.
- Osculating rims (in-band κ_rel) escalate — the table's row,
  already ratified.

Under this ruling MATE-7's slate becomes: item 1 (operand gate,
consulting the VERBS-GATE posture), item 2 (`carrier_eq` torus
rung), item 3a (the `tangent_locus` torus×torus arm — needed even
under the case split, since the cusp arm's verification and the
gate's admission both consume it), item 3b (the join's rim
reconciliation wiring to the table). One L/XL unit or two, per
pricing at spec time.

Honest counterarguments: ruling the case split leans on the wedge
table's reach at a JOIN seam, where MATE-3's item-4 handoff (the
M9-3 emission arm) is not yet built — the join must classify the
rim's wedge before zipping, which is new consumption of the table,
not enforcement after the fact; if that consumption is heavier than
it looks, shape (b)-only (defer kissing tori entirely, keep them
refused typed) is the cheaper first ruling and loses only the
kissing-pair class. And the demand signal (the lily's stem) is a π
seam — the (a) arm has no demo behind it yet.

## Questions for Evan

1. Rule the CASE SPLIT (the table decides; both shapes are arms),
   or the cheaper (b)-only first ruling with kissing tori staying
   typed refusals?
2. Either way, may `tangent_locus` grow the torus×torus rim arm in
   MATE-7 (it also serves the gate admission and the carrier rung's
   verification), or should witness growth wait for a consumer with
   a demo?
3. Pickup order: MATE-7 as one unit (gate + rung + disposition
   wiring) or split (gate+rung first, the rim wiring after the
   emission-arm handoff lands somewhere)?

Recommendation: the case split (1), witness growth in MATE-7 (2),
split into two units with gate+rung first (3) — the lily's stem
retires on the (b) arm, which the first unit's gate and rung
already unblock.
