# SECT — the plan

The section-first join: one certified section per germ face pair, read
by both operands and the matcher, into which the per-pair section
frames and per-kind-pair join lanes retire.

Opened 2026-10-10 by JOIN's close (`docs/doc-ledger/join-leaves-the-tracker.md`;
Ev approved the plan in chat on 2026-10-10). Every row here moved from
`work/join/` by `git mv` with its id and body unchanged. Nothing is
dispatched.

## The design

Ratified in `[ev]` PR 4313 (`docs/DESIGN-FORK-LOG.md` row 100). The
shape is "The shape to give" in
`work/sect/cylinder-sphere-germ-pair-has-no-join-lane.md`, and it is not
re-litigated here:

- a chord is a sub-arc of its germ face pair's section at the pair's C5
  rung, computed once; foot parameters replace frames, and pcurves are
  derived (C4's projected image where no closed form exists);
- every island closes by the chord's own curve, so `RingClosure`'s
  section plane and the radical-plane code retire;
- `GermLane` carries a section datum per side, and kind dispatch lives
  only in C5, so torus and cone pairs refuse at C5, not in the join;
- coincident and tangent sections (coaxial, an offset or a tangency
  decided Zero, in band) are decided at the section door. They refuse
  there until INTENT stage 4's PR E (`booleans-glue-on-zero`), and from
  E on they glue and record there.

The props and mesher fitted-boundary arms, and PCERT's spline-carrier
mint (fork row 89), are prerequisites for these poses' rows going
`SOUND`, not for the build.

**Ownership.** SECT owns the curved section-frame and lane rows
outright. No per-pair stopgap lane is built on another track: a stopgap
rebuilds what this design retires. Ev left the choice open on
2026-10-10, and JOIN's orchestrator chose ownership.

**The gate has fired.** The build was ordered after INTENT stage 4's
PR A (`the-join-builds-what-the-rest-zip-builds`, PR 4364), which
rewrote the join's ring re-homing on a wall chart and deleted the REST
zip. PR 4364 merged 2026-10-09T21:06Z.

## The slate

**31 budget points** of dispatchable work against a ceiling of 31
(`work/sect/program.md` states why it is not 30).

| pri | item | cost | title |
|---|---|---|---|
| P1 | `cylinder-sphere-germ-pair-has-no-join-lane` | H +design | cylinder × sphere: a space-quartic section with a frame and no join lane |
| P1 | `skew-cylinder-germ-pair-has-no-section-frame` | M | two cylinders with skew axes: no section frame |
| P1 | `torus-germ-pairs-have-no-section-frame` | H +design | a torus against a sphere, a cylinder or a torus: no section frame |
| P1 | `closed-in-face-section-loop-has-one-site` | H | a closed operand conic lying in the partner's face: a one-site section loop |
| P1 | `cylinder-sphere-tangency-is-decided-twice-and-its-offset-computed-three-times` | M | cylinder × sphere tangency decided at two sites, the axis offset computed three times |
| P1 | `the-walk-order-is-spelled-twice` | M | the order along a section conic, spelled by the partner ranking and by the split's `conic_pairs` |
| P2 | `a-bar-through-a-ball-refuses-at-a-door-that-moves-with-scale` | M | the nearest-facing rank reads chord length in metres, so the refusing door moves with scale |
| P3 | `along-edge-ring-on-a-curved-face-has-no-join-arm` | M | a segment along an edge of both solids closing a ring on a curved face: no join arm |
| P3 | `in-band-axis-offset-is-noarm-at-one-arm-and-escalates-at-another` | E +design | an in-band axis offset: `NoArm` at the cylinder × sphere frame, escalated at the parallel-cylinder arm |
| P3 | `the-travel-slack-could-be-read-away-near-the-site` | M | the travel margin's `2ε cot ψ` slack |

`the-walk-order-is-spelled-twice` and `a-bar-through-a-ball-…` were
unpriced on JOIN and are priced here (P1 M, two spellings of one order;
P2 M, a margin read in absolute metres).

## Order

The build order is the one `[ev]` PR 4313's body set out and Ev
approved with it, three H units:

1. the section object, with cylinder × sphere chords and closure;
2. sphere × sphere, plane × curved and parallel cylinders moved onto
   it, retiring the radical-plane code;
3. matching by component parameter, retiring `pair_section_frame`.

So:

1. **Dispatch step 1 as the cylinder × sphere lane.**
   `cylinder-sphere-germ-pair-has-no-join-lane` is the first instance
   of the section object. `cylinder-sphere-tangency-is-decided-twice-…`
   and `in-band-axis-offset-is-noarm-…` ride with it: the section door
   is the one home for tangency and an in-band offset, and the frame
   that spells them today retires. The brief is written against row
   100 and the 4313 body; where writing it finds a choice they leave
   open (the section object's type and home, or the seam between the
   door's coincidence decision and INTENT E's record), that choice goes
   to one Opus and one Fable designer (`docs/prompts/designer.md`,
   `memories/orchestration-model.md`) before the lane builds.
2. **Step 2**, once step 1 lands. The planar lanes become closed-form
   instances of the same object, and the ring-lane islands close by
   the chord's curve.
3. **Step 3, and the rows it settles.** Matching by foot parameter
   retires the frames. `skew-cylinder-germ-pair-has-no-section-frame`
   owes no frame proof and joins once C5 has its rung-3 arm;
   `torus-germ-pairs-have-no-section-frame` refuses typed at C5 (C2
   limb 2) rather than in the join; `along-edge-ring-on-a-curved-face-…`
   falls out, because its chord copies an edge whose curve is known.
   `the-walk-order-is-spelled-twice`, `a-bar-through-a-ball-…` and
   `the-travel-slack-…` are read again then: each may close by
   retirement, and the ones that do not are built.
4. **`closed-in-face-section-loop-has-one-site`** is not D10-gated (its
   fixture is undeclared, and the missing self-loop arm is the join's
   topology; released from the hold on 2026-10-08). Its transverse
   cases landed in PR 4345. It is built on the section object, in
   whichever step first reaches a conic lying in the partner's face.

## Review posture

Per unit, at dispatch, by the review tiers of
`memories/orchestration-model.md` and `docs/DUAL-REVIEW-PROTOCOL.md`
(an H unit draws a concurrent pair, an M unit a urandom draw); the log
names each tier and its reason. The A/B experiment is suspended, so the
track carries no band.

A lane measures its row's repro on current main before it builds.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`,
so the program closes without an exit walk.
