# Torus×torus rim tangency (issue 968 item 3) — design conversation

**STATUS: RATIFIED (Ev, in-chat, 2026-09-01 — all three
questions answered). Q1: the THIRD option — the ROUTING is ratified
(the material-wedge table decides which treatment a rim gets: π →
the smooth-seam zip; 0/2π → the declared-Tangent cusp family with a
certified witness; in-band → escalate), and only the π arm is built
now; the kissing arm is DEFINED-BUT-UNBUILT, its typed refusal
citing this ruling (the A11-rider pattern). Q2: `tangent_locus` may
grow the torus×torus witness arm WITH a demo landing alongside it —
under Q1's ruling that growth banks WITH the kissing arm it serves
(the permission and the demo rider are recorded here for whenever
it builds). Q3 (follows from Q1): MATE-7 splits — unit one is the
operand gate + the `carrier_eq` torus rung + the π-arm join wiring,
with the lily's stem as its demo (`docs/MATE-7A-SPEC.md`); the
kissing arm banks with its ruling of record.** The pre-implementation ruling MATE-7
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

## Questions for Ev

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

## The design considerations, in full (added 2026-09-01, answering Ev)

**The geometry.** Two tori sharing a rim circle are two physically
different situations wearing one description:

1. **The smooth continuation** — walking across the rim, the
   tangent plane is continuous and material fills a π wedge. This
   is the tube chain: consecutive segments of one bent pipe meeting
   at a circular seam. The lily's stem-corm weld is this. There is
   nothing to *declare* here — the rim is a seam of one composite
   wall, and the join's job is purely structural: zip the two walls
   into one body carrying a π edge (the treatment M9-3 already
   ships for the tube chain).
2. **The kiss** — tangent planes agree at the rim but the composed
   material pinches to wedge 0 (a knife-edge circle) or opens to 2π
   (a circular slit). This is the declared-cusp family the #131
   ruling covers for straight edges, at a circular rim. Here a
   DECLARATION is mandatory (the C7 rule: tangency is never
   inferred from values — the author must say the kiss is
   intended), and verification needs a certified WITNESS that the
   two surfaces genuinely touch along that circle. `tangent_locus`
   is the witness lane, and it has no torus×torus arm — growing one
   is real certified-numerics work.

**Why the old conversation was stuck.** At #966 time the kernel
could not TELL these situations apart, so the thread had to pick
one treatment for the whole class — hence the two candidate shapes,
each right for one situation and wrong for the other.

**What changed.** MATE-3 landed the material-wedge verdict table:
given the face senses and the tangency jet, the kernel now
CLASSIFIES a rim (π vs 0/2π vs in-band-escalate). So the class no
longer needs one answer — the table can route each rim to its
treatment.

**The honest catch.** The table runs at tier-3 validation, AFTER a
body exists. The case split needs the classification at JOIN time,
while zipping — new consumption of the table's machinery inside the
boolean lane, whose price is unmeasured, and the emission arm that
would mint declarations at joins (MATE-3's item-4 handoff) is not
built.

**What each ruling buys and costs:**
- **Case split now**: both situations get their defined path; the
  join learns to ask the wedge question (unpriced); the kissing arm
  has no demo demanding it yet.
- **(b)-only now**: the lily's situation is fully served with no
  new machinery beyond MATE-7's gate and rung; kissing tori stay
  typed refusals; the routing question returns whenever someone
  authors a kissing pair.
- **THIRD OPTION — ratify the routing, build only the π arm**:
  rule the PRINCIPLE (the wedge table decides which treatment a rim
  gets) into the design now, implement only treatment (1), and
  leave treatment (2) as a defined-but-unbuilt path whose typed
  refusal cites this ruling. This is the A11-rider pattern — ruled
  ahead of implementation — and separates "what is the design"
  from "what do we build". Under it, question 3's answer falls
  out: MATE-7 splits, gate + carrier rung + the π-arm join wiring
  (+ the ruled demo) first, the kissing arm banked with its ruling
  of record.

Recommendation, updated: the third option.
