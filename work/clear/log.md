# CLEAR — the log

## 2026-09-20 — opened

Cut out of SHELL, which was carrying 65.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed SHELL's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

9 rows arrived by `git mv` with their ids, bodies and history
unchanged. SHELL keeps its band 2300-2399; band 8000-8099 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-10-02 — Seam note from AUTHOR's exit: `clearance-refusal-names-one-face-twice-across-bodies` has a viewer consumer parked on it, DOORS's `the-gui-has-no-clearance-consumer` (P1 H, moved from `work/author/`). When it closes, set that row `open` and DOORS `ready`. (AUTHOR orchestrator)
- 2026-10-06 — Two rows arrive from SHELL's triage, by `git mv`, ids unchanged: `shell-curved-wall-clearance-window` (parked on SHELL-4, which says it closes it — 0 points) and `violated-witness-can-sit-off-the-trimmed-face` (re-priced P3/E: `verify_witness` now filters through `certified_off_the_face` where `chart_arms` answers, so what remains is a regression row pinning the two r2 fixtures; the cone/sphere/torus residue is CHART's `clearance-window-cone-sphere-torus`). (SHELL orchestrator)

## 2026-10-06 — two rows filed from CARVE's designers

CARVE's designers on the loft's section placement both found that the
clearance engine cannot yet certify a loft's embedding, and that
neither gap was on a slate:
`self-intersection-drops-every-vertex-sharing-face-pair-globally` (P2,
H, design open) and `window-of-refuses-an-untrimmed-iso-bounded-nurbs-patch`
(P2, M). With SHELL-3, they are what a loft-door embedding certificate
would need. CARVE will say here if its design lands on them.

Signed: (CARVE orchestrator)

- 2026-10-24 — Note from CHART: `min-separation-tightening-crosses-the-drive` moved onto this slate by `git mv` at CHART's priority-seam cut. Its fix shape is all on `clearance.rs` and `measure.rs`, and its ruling (may a lane split be invisible to the census) is Ev's. See the row's `## Re-homed` section. (CHART orchestrator)

- 2026-10-25 — **Band rule (work/README.md, Priority):** `SHELL-3` re-banded P1→P0, and `window-of-refuses-an-untrimmed-iso-bounded-nurbs-patch` and `self-intersection-drops-every-vertex-sharing-face-pair-globally` P2→P0. They are the three prerequisites the P0 `self-overlapping-spines-build-and-validate` (CARVE) is parked on, and each one's fix as described is what that loft certificate needs: the engine in `topo`, a window for an iso-bounded loft wall, and the vertex-sharing pairs examined (without that, a triangle-section loft has no examined pair). (band-rule triage)

## 2026-10-27 — picked up; the CLEARTAIL cut; sequenced behind INTENT's E

- **Picked up, `active`.** The slate stood at 42.5 points against 30.
- **Cut along the priority seam.** Six P2–P4 rows moved to the new
  CLEARTAIL program by `git mv`, ids unchanged. One of them,
  `clearance-edge-curve-ladder-can-read-through-body-stored-curve`,
  was unpriced and is priced P4/E.
- **Closed** `shell-curved-clearance-consumer`. Ev ruled B on
  `[ev]` #1737, and `SHELL-3` and `SHELL-4` build that answer.
- **`SHELL-4` parked** on `SHELL-3`. It was `open` with a blocker.
- **Legacy `D` re-priced.** The window-selection row is now M, with no
  open design: the shape to copy is `resolve_face` and `entity_of`.
  `symbolic-tier-and-clearance-engine` is now H with design open.
- **Sequencing.** INTENT's E (#5811) is the one open PR that edits
  `clearance.rs`, at the sites `SHELL-3` moves. CLEAR lands no code in
  that file until E merges. The alternative was to build `SHELL-3` on
  E's branch, but E is still under dual review, so its head is not
  stable. Designers run in the meantime (`plan.md`).

- 2026-10-28 — **SHELL-3 merges (#5909)** with this commit last. The orchestrator read and accepted the fix pass at `8cb7116401`. Unparked: `SHELL-4`, `symbolic-tier-and-clearance-engine` and `min-separation-tightening-crosses-the-drive`. Pruned the fired `SHELL-3` entry on CARVE's `self-overlapping-spines-build-and-validate`.
## 2026-10-27 — designers on the symbolic tier and on self-intersection

- **The symbolic tier: decided, with no `[ev]`.** Each designer was
  shown the other's report for one round, and they converged. The
  decision is on the row (`## Decided`) and in fork-log row 202. It
  merged in #5864.
  - The door goes generic over `&Body<T>`, and the arithmetic stays
    `Interval` through a lift on read.
  - A door value on `AtRestPolicy` replaces `MinClearanceLane`.
  - It is built in two steps: SHELL-3 moves the engine with the generic
    signature, then the symbolic row turns `Sym` on.
  - Filed: `clearance-replays-a-leaf-off-the-verdicts-lane`.
- **Self-intersection: three rounds, then converged.** Rounds 1 and 2
  each crossed on one or more points, and round 3 asked only about the
  metric doors' pairing rule. The converged design is on the row:
  embedding, departure and monotone certificates, second order at
  cusps, and the pairing rule dropped. Fork-log row 205.
  - What waits on Ev is the rewording of ERROR-DESIGN E7's scope
    clause: `[ev]` #5866, with `needs_ev` set on the row.
  - Class finding, both designers: the normal-cone rules, right-angle
    and half-space alike, are unsound (the 370° ribbon). Recorded on
    the row.
- **SHELL-3's spec is drafted and waits for INTENT's E (#5811) to merge.**
  `min-separation-tightening-crosses-the-drive`'s designers also wait
  for E, as that row asks.
- **Blinding.** The byte for both forks was drawn late, after the first
  reports and before any `[ev]` PR. Both rows say so.

## 2026-10-28 — Ev ruled on E7; E merged; SHELL-3 and the minsep designers dispatched

- **#5866 merged.** Ev approved the rewording of E7's
  *Self-intersection scope* clause, and it now leads with injectivity.
  Ev's words are on the row.
  - Ev asked whether big curved faces are a problem. They are not; the
    cost is near-tangency. A large-curved-body budget row is now pinned
    on the self-intersection unit.
- **INTENT's E (#5811) merged** as `6f1c805c25`. Its final
  `clearance.rs` frames through `Framed::computing_among` and `read_in`
  in `clearance_with`, and repeats that at `f64` in `verify_witness`.
  Both stay in `editor-core` under SHELL-3's spec. INTENT's next units
  (the carry, G) announce `wire_place`, `product.rs` and
  `eval/parts.rs`, not `clearance.rs`.
- **SHELL-3 dispatched** on `clear/shell-3-engine-into-topo`, spec
  `docs/SHELL-3-SPEC.md`.
  - Review: single FULL review on the frozen head. It is a move with no
    behaviour change, and the goldens are the differential.
- **`min-separation-tightening-crosses-the-drive`'s designers
  dispatched.** The row asked for them to wait until E landed. The
  blinding byte was drawn at dispatch this time (analysis branch).

## 2026-10-28 — SHELL-3's review

- **The PR is #5909.** CI was green at the frozen head `4a4f7f211`.
- **The full review read APPROVE-WITH-FIXES.** It found no behaviour change: the moved bodies differ from the base only in the lift and the renames, and every clearance golden and certifying key held.
  - It found three MINORs: the topo README, a doc deferral in `drive.rs` with no schedule behind it, and an M5 PR 11 ledger sentence that is now false.
  - Its style lane asked for the midpoint to have one home, for the two band inputs to become one, for topo's prose about editor-core's census to move to editor-core, for the verifier contract to be stated, and for a `same_body` row that can go red.
  - All of that is in the implementer's fix pass.
- **Class findings filed:**
  - `the-bound-walk-and-min-separation-duplicate-one-frontier-loop` (P1);
  - `the-clearance-budget-pair-is-spelled-three-times` (P1);
  - `the-clearance-sliver-rule-lives-in-one-of-its-two-consumers` (P4, on CLEARTAIL).
- **Scope added to the symbolic row:** narrowing `MeasureClearanceRefused`'s carrier, and a row that runs the generic door at a non-`Interval` `T`.

## 2026-10-28 — the second wave

- **SHELL-3 merged (#5909)**, `a1b4b82930`.
- **NURBSWIN dispatched** (P0, `window-of-refuses-an-untrimmed-iso-bounded-nurbs-patch`) as a local lane on `clear/nurbswin`, spec `docs/NURBSWIN-SPEC.md`. Review: single FULL.
- **SYMCLEAR dispatched** (`symbolic-tier-and-clearance-engine`, carrying the lane-replay row and the `NeedsEnclosure` door-path row) as a cloud session, `session_018u3gVF5GZTM5S9659fmMRF`, on `clear/symclear`, spec `docs/SYMCLEAR-SPEC.md`. Review: single FULL. It ran in the cloud because the container's disk holds one lane's build at a time.
- **Sequenced after SYMCLEAR:** CLEARTEXT, the four refusal rows (spec drafted). SYMCLEAR narrows `MeasureClearanceRefused`'s carrier, which those rows reshape.
- **After NURBSWIN:**
  - the self-intersection build (dual review);
  - the loop-duplication and budget-triple refactor, which goes ahead of it because the self-intersection build changes the same loop;
  - `min-separation-tightening-crosses-the-drive`;
  - then `SHELL-4`.
- 2026-10-28 — **NURBSWIN merges (#5927)** with this commit last. The orchestrator read and accepted the fix pass at `2cef54df49`. Pruned the fired blocker on CARVE's `self-overlapping-spines-build-and-validate`.
- 2026-10-28 — **SELFX dispatched** (P0, `self-intersection-drops-every-vertex-sharing-face-pair-globally`) as a local lane on `clear/selfx`, spec `docs/SELFX-SPEC.md`. Review: DUAL, concurrent pair (H), on a frozen head (`docs/DUAL-REVIEW-PROTOCOL.md`). Its class letter is H.
- 2026-10-28 — **SELFX in review.** PR #5955 is green at `91109e84ac`. The DUAL concurrent pair runs as two cloud sessions (R1 `session_018u3gVGWkWwvU2JyhiPsRje`, R2 `session_018u3gVJQa1Ftewruzok4tLu`), with identical briefs and the frozen head pinned in each. The lane deferred the second-order (cusp) arm, as the spec allows: a cusp seam refuses typed, and the arm is filed. Departure is band-level: a crossing confined within ε of a shared edge is not seen. That claim is put to both reviewers.
- 2026-10-28 — **SELFX dual review, adjudicated as a union.**
  - **R1: APPROVE-WITH-FIXES.** MAJOR: the rows assert `certified >= 1`, so with cross-face pairs dropped and the self pairs kept, the loft and helix rows stay green (executed).
  - **R2: NOT-MERGEABLE-AS-IS.** MAJOR: `present_chart` clips in edge order with a re-centred wrap. On a periodic face crossing its seam, one order discards most of the face, and departure certified a pierced pair at depth 0 (unit-level evidence, executed).
  - Both found the band-sliver claim stated beyond the code, the `FaceKey::default()` fallback, and stale headers. Each found minors the other did not; the tally lives in the row file.
  - **Fix pass** sent to the SELFX lane.
  - **Filed:** `departure-presents-no-curved-shared-edge` (P1/H). It is a capability gap that fails closed, and it gates SHELL-4 on curved cavities. Also filed `an-analytic-face-may-cover-its-carrier-more-than-once` (P2).
  - After the fix pass, a focused single review of the presentation fix runs before merge, given that a soundness MAJOR was fixed.
- 2026-10-28 — **SYMCLEAR merges (#5928)** with this commit last. The orchestrator read and accepted the fix pass at `0fa633d5`; the `cell_region` hoist is sound, because the station's bounds are the same at both lanes. Friction, recorded: tier-on per-leaf queries moved fifteen rows into the slow set. The cost is the symbolic tier's per-decision `decide_sign` form expansion (~0.9 s on the e2e leaf), which is SYM's ground. Unblocked: CLEARTEXT.
- 2026-10-29 — **SELFX merges (#5955)** with this commit last. Fix pass 2 (`89edd2d473`) was read and accepted. The dual-review row records tally 1. CARVE's loft row is unparked.
- 2026-10-29 — **LOOPS merges (#5976)** with this commit last. The single FULL review (`bb8d6a4dcc`) returned APPROVE-WITH-FIXES and re-proved there was no behaviour change by sweeping budgets on both doors. Fixed in the closing commit:
  - the MINOR: the `CellReceipt` type doc contradicted its field doc on `abandoned`;
  - Q5: the module header's "same split rule";
  - Q7: `Walk::split` now takes `N` from `cut`'s array length, not a separate count.

  Filed to CLEARTAIL (P4): the root phase's second walk and the second spelling of `abandoned`. Accepted unfixed, with reasons:
  - Q3: nothing guards the shared walk once the differential is reverted. Its equivalence was proved at merge, and from here it is ordinary code.
  - `CellBudget` as a fourth spelling of the budget pair: it is the refusal type, which belongs to CLEARTEXT.
  - Q2: `let c = bound.c();` is shaped by the from-f64 gate's text matching.

  Main's cost-curve red, which the lane added to GREEN's main-reds row, was fixed by #5979. Unblocked: the minsep tightening.
- 2026-10-29 — **TIGHTEN dispatched** (`docs/TIGHTEN-SPEC.md`, branch `clear/tighten`, cloud lane). It carries out the 2026-10-28 decision on `min-separation-tightening-crosses-the-drive`: a scoped unlogged `k_stats` door, `min_separation` deciding its cells, and the retirement of `LowerBoundOnly`. Review: DUAL.
- 2026-10-29 — **CLEARTEXT merges (#5977)** with this commit last. The single STYLE review (`25f81b1897`) returned APPROVE-WITH-FIXES:
  - two refusal sentences were false in reachable cases: `Unsupported` at the injectivity raise, and the table-local body index of `Tied`/`OtherBody`;
  - the item-3 hit list was incomplete;
  - an unscheduled VIOLATED line;
  - style: a broken invariant read as a caller fault, side-less one-face arms, two renderings of one refusal, and duplicated wording.

  The orchestrator read and accepted the fix pass at `7bc8900b`. Every item was taken. The new `_ =>`-over-`Result` pass found one real case: the meeting-pair margin's `Err`, possible only when the bound lies in the band, had counted as a violation and then been refused as `WitnessUnverified`; it now refuses `Sliver`. Filed to CLEARTAIL: `clearance-violated-verdict-names-its-pair-by-bare-keys`. Filed to TALLY: the `eval/measure.rs` corruption arms. `injective.rs` gained four one-line side arguments, so CURVEDEP (#5991, in DUAL review) will merge main over them.
- 2026-10-29 — **Cut to CLEARTAIL**, at load 47/30. These rows move by `git mv`, with ids and bodies unchanged:
  - P2: `a-recorded-contact-refuses-the-injectivity-question-until-contact-design-hands-it-off`, `an-analytic-face-may-cover-its-carrier-more-than-once`, `the-second-order-arm-certifies-a-declared-wedge-0-edge`;
  - P3: `comb-rows-ask-a-scope-against-itself-since-the-metric-doors-dropped-pairing`;
  - unpriced: `boundary-of-evaluates-a-spline-edge-over-its-whole-span`, `iso-bounded-domain-has-two-refusals-no-valid-body-reaches`, `restricted-net-hull-has-four-spellings`.

  `an-assertion-over-scopes-that-meet-refuses-a-window-superset-for-an-exact-zero` stays, because TIGHTEN (#5998) closes it. Rows that open units in flight file into CLEARTAIL directly, unless they are P1. Queued: `main-red-injective-rs-folds-the-face-sense-by-hand` (P1/E), after CURVEDEP merges, since both are in `injective.rs`.

## 2026-10-29 — the third wave

- **SELFX merged (#5955)**, `5e934f49`. CARVE's loft-certificate row is open.
- **Dispatched:**
  - **LOOPS:** the duplicated frontier loop and the triple budget pair. It runs as a local lane; its review is a single FULL review.
  - **CLEARTEXT:** the four refusal-text rows. It runs as a cloud session, `session_018u3gVFPqmdk3LCeY6Ud5ae`; its review is a single STYLE review.
  - **CURVEDEP:** `departure-presents-no-curved-shared-edge`. It runs as a cloud session, `session_018u3gVF7V1QL1mSkAd58VeW`; its review is a DUAL concurrent pair.
- **Waiting:** the minsep tightening on LOOPS, and SHELL-4 on CURVEDEP.
- 2026-10-29 — **TIGHTEN merges (#5998)** with this commit last. The arm is SEQUENTIAL (byte 64, mod 3 = 1).
  - **The review.** A single review at `e27260a2` returned APPROVE-WITH-FIXES with no MAJOR, so there was no second review. It probed 15 configurations, and every bracket contained the truth. Mutants A and B were killed by shipped rows. Mutant C (the scope removed) was caught only by wall time.
  - **Its findings.** MINORs:
    - `OnEdge` in `hi` while the docs claimed exact containment;
    - a redundant station read and a dominated arm;
    - stale docs.

    NOTEs:
    - `lo` could be `+∞`;
    - the latent k-lint hazard.
  - **The fix pass at `535a6988`**, read and accepted:
    - `hi` is now stated "up to the band" at every claim site and in F28;
    - one station read;
    - an `Option` incumbent;
    - a live kernel-defect assert on `lo`;
    - a fast scope-pin row, red under mutant C;
    - the r2 gap row pinned tight;
    - `flagged_census` now checks every `unlogged` row.
  - **Cost**, re-measured: about 2.4× on single-drive rows, about 1.25× on the suites.
  - **Amended in the row's Decided section:** no bisection on an absent `hi`; `hi` up to the band.
  - **Also closed:** `an-assertion-over-scopes-that-meet-…`.
- 2026-10-29 — **CURVEDEP merges (#5991)** with this commit last. Departure now certifies across a closed curved shared edge (pipe, mitre, drum). The arm is concurrent (letter H).
  - **The DUAL pair at `b7394bd758`.** R1 returned APPROVE-WITH-FIXES and R2 NOT-MERGEABLE-AS-IS. Coded blind with byte 162 (A = R1): tally 0. R2's MAJOR, that lax soundness guards go red on no row, is bilateral with R1's MINOR. Sensitivity: R2's partial-side mutants F/G, split out, would have scored 1. R1 alone found that kernel-built oblique cuts still refuse.
  - **Fix pass 1 (`716862ab`):**
    - a red row per mutant;
    - a lamina (curved wedge-0) row;
    - the oblique cut pinned refusing;
    - `Run::graph` split per step;
    - `Zero::Curve`/`Zero::Past` in the types.
  - **The focused review**, `session_018u3gVJ4e`: APPROVE-WITH-FIXES. Six of its own mutants survived, plus a dangling citation and an `f64::min` that ignored NaN.
  - **Fix pass 2 (`3470ea3b`), read and accepted:**
    - red rows for the Past guard's `analytic` and `zero_on`, and the whole-turn `rb`;
    - arguments at the guard for step 1's per-piece monotone check, "passes both ends" and `apart`'s ±1 copies. The orchestrator spot-checked two: a sinusoid over less than a turn crosses a level at most twice; a run ending inside the columns ends at a vertex, which `alone` refuses;
    - a fail-closed `rb`;
    - an order-independent lamina pin.
  - **Moved to CLEARTAIL:** its two P2 filings, `curved-presentation-reads-plane-and-cylinder-images-only` and `a-coordinate-line-on-a-face-of-unknown-extent-is-unplaced`.
  - **Stays P1 in CLEAR:** `departure-reads-no-curved-corner`, which SHELL-4 waits on.
- 2026-10-29 — **SENSEFOLD dispatched** (`docs/SENSEFOLD-SPEC.md`, branch `clear/sensefold`, cloud lane) for `main-red-injective-rs-folds-the-face-sense-by-hand`, after CURVEDEP. Review: the orchestrator's read.
- 2026-10-29 — **SENSEFOLD merges (#6028)** with this commit last. The orchestrator read the diff and accepted it: one hit goes through the sense door, and two chart-space signs are sanctioned with their reasons. Receipts are unchanged. The slow-set row stays red on TESS's `trimmed.rs` sibling only.
- 2026-10-29 — **CORNER dispatched** (`docs/CORNER-SPEC.md`, branch `clear/corner`, cloud lane) for `departure-reads-no-curved-corner` (P1/H). Review: DUAL, a concurrent pair. **SHELL-4 is parked on it.** Its vessel fixture's cavity meets disc and cylinder along open semicircles, so shipping the gate now would refuse a valid vessel. The tube has none, and the curved twin isn't built. The curved-corner row's SHELL-4 paragraph is corrected to match.
- 2026-10-29 — **A main red from SENSEFOLD (#6028), fixed by #6055.** SENSEFOLD's `from_chart` call in `injective.rs` turned the slow-set row `face_normal::tests::the_planar_sense_flip_lives_in_one_place` red. A CORNER reviewer noticed. #6055 adds the by-value door `face_normal::chart_outward_normal`, which is bit-identical. **Class, recorded:** the orchestrator's read of a change made to satisfy one slow-set guard row did not run that row's sibling guard rows in the same test module, and the PR gate selects neither. When a unit edits code to satisfy a guard row, its read runs every guard row in that module, slow set included.
- 2026-10-29 — **CORNER (#6045): DUAL concurrent pair at `45cf1e4507`.** R1 and R2 both returned NOT-MERGEABLE-AS-IS. Coded blind with byte 137 (A = R2): tally 0, because the one MAJOR is bilateral. Neither found a code defect.
  - **The MAJOR:** step (3)'s "stays on the face side inside R" is pinned by no row in either reading. Executed mutants: the curve read at V's tangent only; the line read at V's piece only.
  - **Unilateral:** the whole-turn extent check fires on non-periodic axes (a completeness loss, executed); the N5 `sign τ_a` drop; an undisclosed ε at the vertex.
  - **Class, recorded:** a second soundness arm in a row (after CURVEDEP) shipped guards that its own `mutant → row` table missed. The lane's table covered the guards it thought of, and both reviewers found the "stays" guard by asking what the proof's steps need, not what the code checks. Future soundness-arm specs ask for the table per PROOF STEP, not per guard.
  - The fix pass went to the lane.
- 2026-10-30 — **CORNER merges (#6045)** with this commit last. Departure now certifies across a convex curved corner.
  - **Rounds:**
    - Fix pass 1 (`5356815d`): "stays" pinned in both readings, both line guards load-bearing, the whole-turn check limited to periodic axes, two unit folds, `on_vertex` as the 6th ε, the torus out of `reading`.
    - A focused review (`session_018u3gVHR1`) returned APPROVE-WITH-FIXES: the audit row was missing, and `on_vertex`'s rows were blind to the `v` axis and to an in-band gap.
    - Fix pass 2 (`c140ecc3`), read and accepted: the audit row; `v` and in-band rows; the hang-prone rows bounded by a nextest timeout; N3 as an explicit refusal; `graph_read`'s whole-turn check gated by `periodic`, with every `whole_turn` caller swept.
  - **Moved to CLEARTAIL:** its P2 and P3 filings.
  - **SHELL-4 is unparked.** Its parking note was wrong: the vessel already held, via the carrier arm. SHELL-4 is CLEAR's last P1.
- 2026-10-30 — **SHELL-4 dispatched** (`docs/SHELL4-SPEC.md`, branch `clear/shell4`, cloud lane). This is CLEAR's last P1. Review: a single FULL review, not DUAL. The design was ruled by Ev on #1737, and the unit wires the reviewed injectivity certificate into one verb, so the risk is placement, typing and cost, which one reviewer with claims to falsify covers.
- 2026-10-30 — **SHELL-4's lane stopped; RIMSCALAR dispatched** (`docs/RIMSCALAR-SPEC.md`, branch `clear/rimscalar`). The vessel holds at `Body<f64>` but refuses `Departure` at `Body<Interval>`. **Class, recorded:** every self-intersection row runs `Body<f64>` only, so the certifying scalar is untested. CORNER's reviews and SHELL-4's unpark note both read `f64`. New P1 `injectivity-refuses-at-interval-what-it-certifies-at-f64`; SHELL-4 re-parked on it, with its WIP kept at `623daf636b`. Filed on OFFSET (P2): `offset-corner-on-surface-escalates-at-interval-on-an-arc-necked-extrude`. The curved-residue row is updated with the box-with-void's current behaviour. Review for RIMSCALAR: a single FULL review with mutation, since the fix aligns the `Interval` reading with the reviewed `f64` one.
- 2026-10-30 — **RIMSCALAR merges (#6090)** with this commit last.
  - **The cause:** `harmonic_of` read an angle rate as `w·τ/τ`, a few ulps wide at `Interval`, so the corner presentation's exact ±1 test failed.
  - **The fix:** read it off the integer winding. Every self-intersection and pipe row now runs at both scalars, with matching verdict classes.
  - **The review:** a single FULL review at `534e7b8f39` returned NOT-MERGEABLE-AS-IS. Its MAJOR was that the lane's first fix added "the strip", an `iso_of` slack that is a new ε loosening with no funnel or audit row. Reverting the strip alone left every verdict unchanged, so it bought a receipt count, not a verdict. It also widened the touch concession about 4× past its bound.
  - **The fix pass (`c1481be9`)** removed the strip and pinned the vessel by verdict at both scalars. The orchestrator read it and accepted it.
  - **Class, recorded:** a fix that adds a concession must show a verdict that needs it. The reviewer's test, reverting the concession alone and diffing verdicts, goes into future briefs for any unit that adds an ε.
  - **Filed:** the drum divergence (CLEARTAIL, P2); the home of `angle_rate` (PCERT, P3).
  - **SHELL-4 is unparked.**
