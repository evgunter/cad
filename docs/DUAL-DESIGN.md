# The Dual contract — what a `Dual` actually has to do (M10-D)

**Status: DRAFT — design conversation, awaiting Evan's sign-off.**
This is the design pass the DESIGN.md M10 roadmap entry reserved:
it answers the collected question (*what does a `Dual` actually
have to do*), cleans up the `Bounds` / `CertifiedEnclosure` split
on that answer, and unblocks M10-4. Decisions DL1–DL6; each with
the proposal, the evidence, and what it forecloses. Grounding: the
D1 ruling (Evan, 2026-08-19: *a `Dual` may not certify — at least
for now — but it may have `Bounds`*), ERROR-DESIGN E4/E5/E9, the
`real.rs` ratification ledger, issues 687 and 701, and the
substrate survey of 2026-08-29 (facts cited inline). No code rides
this PR; the implementing unit dispatches after ratification.

## DL1 — The charter answer: a Dual is tangent transport; it never
## certifies, and the hedge closes

**Proposal**: a `Dual<T>` is the tangent bundle of the `T` build —
its value channel IS the plain-`T` run bit-identically (the
ratified dual contract), and its tangent channel transports
derivative data alongside, consumed by exactly three uses: E4/E5
sensitivities (`Dual<f64>`), derivative *enclosures* for E7
monotonicity pruning and E5 contribution bounds (`Dual<Interval>`),
and nothing else. **A Dual never certifies — permanently.** D1's
*"at least for now"* closes: ERROR-DESIGN never needs a dual to
certify (E4's certified tier consumes `Dual<Interval>` enclosures
for pruning and advisory bounds, never for refusal — E9), and a
future need would be a new design conversation on its own
evidence, not a door left ajar. `CertifiedEnclosure` keeps no
`Dual` impl; the compile-fail pins stay.

- Why close it rather than keep the hedge: a hedge with no expiry
  is the state the smell scan's closing rule refuses, and every
  audit since D1 (the fillet seam, the lane splits) has had to
  re-derive "but could a dual certify someday?" as live
  uncertainty. Naming the answer NO converts four standing
  hesitations into one revisitable decision.
- Forecloses: any future `impl CertifiedEnclosure for Dual`
  without a ratified reversal of this decision.

## DL2 — `ContentBits for Dual<T>`: feed BOTH channels; the seed
## rides the tangent bits (closes issue 687)

**Proposal**: `impl<T: ContentBits> ContentBits for Dual<T>` feeds
the value channel's exact representation, then the tangent
channel's, through the base scalar's own `feed` (domain-separated
by position exactly as every other multi-field feed). No explicit
seed identifier enters the key, because none is needed: the memo
principle is *"same key ⇒ same input bits ⇒ (D9) same output —
the key IS the correctness proof"* (`eval/memo.rs:1-7`), and
feeding both channels extends it verbatim. Under seed pᵢ vs. pⱼ:

- a node not downstream of either seed carries identical
  value+tangent bits in both passes → same key → reuse, and reuse
  is SOUND (bit-equal inputs, deterministic dual ops);
- a node downstream of the seeded parameter differs in tangent
  bits → distinct keys → no cross-pass contamination.

So the failure #687 names (a memo serving one parameter's pass
from another's) is unrepresentable, and E4's n-pass sweep gets
cross-pass reuse of the parameter-independent subgraph for free —
thread pass k−1's `Evaluation<Dual64>` as `prior` (the door already
exists: `evaluate(doc, prior, …)`, `eval/mod.rs:1002`).

- **Rejected — value-channel-only feed**: two seeds collide on
  every node; the memo becomes unsound the day a prior is passed.
  **Rejected — an explicit seed id in the key**: a second
  mechanism restating what the tangent bits already say, and wrong
  under multi-seeded vectors later (E11.4's door).
- Recorded interaction, not a blocker: content keys also hash
  f64 recipe payload directly (placement frames, profile steps —
  `eval/mod.rs:1583-1592, 1721`). Placements are authored
  document data no parameter drives; profile steps are the C6
  pin, whose lift is M10-P's design and carries its own key
  story. Neither changes this impl.
- The `e4_dual_door` suite and `memo.rs`'s compile-fail row flip
  to their successor laws in the implementing unit, as both texts
  anticipate.

## DL3 — A Dual evaluation does not run certified gates: validation
## is the value lane's job, already done

**The measured problem**: if `evaluate::<Dual64>` compiled today, a
corpus document with an ellipse-trimmed or spline face would FAIL
the product gather's tier-3 (+V through `PropsQuadLane`, whose
`Dual` arm refuses) with `VolumeUncomputable`; `Approx` faces
report `ApproxLaneUnsupported`; curved coincident pairs
`CensusUnsupported`. Those refusals are the system correctly
saying *a dual may not certify* — asking a dual evaluation to
validate IS asking it to certify.

**Proposal**: the E4 sensitivity pass evaluates WITHOUT the
certified gates: the evaluation service gains a scalar-policy seam
— certified validation (the product gather's `validate_geometric`,
`recertify_approx`, the census door) runs at scalars with
certification rights (f64's decide-with-escalation lane and
`Interval`), and is structurally absent at `Dual`. This is sound
because the sensitivity is OF THE AS-BUILT BODY: the dual's value
channel is bit-identical to the f64 build (D9), which was already
validated in the f64 evaluation E4 rides beside; re-validating the
same bits through refusing arms adds no information and subtracts
availability. The E4 driver asserts (cheaply, by content key
equality of the value channel where it needs a hook) that it is
differentiating the build the f64 run validated.

- Not a weakening: nothing a dual pass produces is consumed as a
  certificate anywhere (DL1); its outputs are E4-marked
  sensitivities, chamber-certified or `local_only` by M10-3's
  leaves, which are Interval work.
- Foreclosed by this shape: "tolerate refusals per-face and limp"
  — availability by policy, not by swallowing typed errors.

## DL4 — `Enclosure` joins the allowlist gate (closes issue 701)

**Proposal**: `bounds-allowlist.sh` greps `Enclosure` (and
`CertifiedEnclosure` as today's skip-list handles) exactly as it
greps `Bounds`, same file allowlist. The blanket
`impl<T: Bounds> Enclosure for T` makes every `Dual` an
`Enclosure` since D1; no `Enclosure`-bounded signature exists in
`crates/*/src` today (`real.rs:727-740`), so the gate lands green
and the hole (a future `T: Enclosure` bound on certifying code,
with no CI row saying so) closes while it is still hypothetical.
The known alias gap (#279's class) is unchanged in scope — this
adds a name to the existing instrument, not a new instrument.

## DL5 — The fillet seam's lapsed justification: discharge by
## ratifying the delegation rule, not by building an empty lane

**The obligation** (`real.rs:431-489`): `sweep::fillet::{battery,
build,surgery}` is the one allowlisted `Decide + Bounds` seam with
no refusing lane, ratified when `Bounds` had no `Dual` impl; that
guard lapsed at D1, and what is owed is "a lane, or a written
reason it needs none," on the public surface.

**Proposal — the written reason, as a ratified general rule**: a
`Bounds` read is lane-exempt when it (a) feeds an error payload or
report, or (b) selects among constructions whose classification is
value-channel-decided — both sound at any scalar by value-part
delegation (the value channel's bracket is the base scalar's; its
branch is the base scalar's branch). A read that MINTS a
certificate object or feeds a `CertifiedEnclosure` consumer is
never exempt. The fillet seam's fourteen reads were enumerated
twice under exactly this test (ten payloads, four selections, the
two consequential ones delegation-sound; PR 682's body); the rule
generalizes the audit into the standing criterion, recorded on the
`Bounds` ledger, and the seam's obligation line retires. Building
a `FilletLane` whose refusing side would be empty is the dead-code
pattern the M5 reviews punished.

## DL6 — Poison never masquerades as ill-posedness in certified
## lanes (the class PCURVE's datum names)

**The datum** (PCURVE, on the M10 plan PR, A/B-controlled): a
chart residual provably exact at f64 (`0e0`) reaches the caller as
`margin: Invalid` — the poison outcome, documented as "the
question was never validly posed" — at `Interval` with a
conversion present. A second member with the same signature (the
`f64::INFINITY` sentinel embedding as NaI and absorbing through
`min`) is recorded in the same fixture's header.

**Proposal (the contract, not the instance)**: in a certified
lane, `Invalid`/NaI is a legal OUTCOME only when the inputs
themselves pose no real question; an operation pipeline that turns
a well-posed exact quantity into a non-real has a DEFECT at the
site that minted the non-real, and the refusal text must never
claim ill-posedness for it. Concretely: (i) enclosure-lane code
takes the widening path, never an absorbing one, wherever both
exist (`min`/`max`/hulls over possibly-NaI operands are the known
absorbers); (ii) a certified lane's refusal distinguishes
"too wide at this ε" (escalate; recourse = refine/bisect) from
"non-real entered" (a bug-shaped refusal naming the minting site).
E9 already settled the tangent side (tangent poison forfeits its
uses, never refuses); this is its value-lane counterpart, scoped
to certified lanes. The instance's root cause stays with PCURVE
P-1b; the class issue they file collects members; audit of
existing absorbers is that issue's schedule, not this PR's.

## The implementing unit (dispatches after ratification)

One PR-sized unit: DL2's impl + the door-suite successor flips;
DL3's scalar-policy seam in the evaluation service; DL4's gate
line; DL5's ledger text + obligation retirement. DL6 lands as
contract text here and in the class issue. `Dual<Interval>` gets
`ContentBits` by the same impl (both channels via `repr_bits`).
