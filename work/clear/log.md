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
- 2026-10-30 — **SHELL-4 parked again; CAVITYDIAG dispatched** (`docs/CAVITYDIAG-SPEC.md`, branch `clear/cavitydiag`, diagnosis first). SHELL-4's lane finished its rows (`7d97070458`), but the gate over-refuses three valid bodies: the corpus pot at Interval (sphere/cylinder vertex), and the cavities of the klein elbow (Departure) and the cone frustum (budget). Filed as `shell-cavity-gate-over-refuses-three-valid-bodies` (P1). Over-refusing legitimate shells is ruled out, so the gate cannot ship until these are fixed or the design question is put.
- 2026-10-31 — **CAVITYDIAG merges (#6105)** with this commit last.
  - **The pot is fixed at the source.** `pcurve_cache`'s cylinder arm decides a rim's axial terms `Zero` at `pcurve_cylinder_chart_axial`, the cone arm's twin.
  - **The review.** A single FULL review at `685ab423a5` returned APPROVE-WITH-FIXES. It probed the source fix and found it reproduces the lane's topo concession bit for bit, so the orchestrator chose the source fix. CLEAR crosses into PCERT's file `pcurve_cache.rs` for this (announced). PCERT's row is closed.
  - **Disclosed:** a route split on tilted cylinders. f64 reads a curve where Interval reads the line, and verdicts match.
  - **The orchestrator's read** also fixed a doc comment that the insertion had moved onto the new function.
  - **Filed (P1):** the frustum (`cone-window-reaches-the-apex-past-a-frustums-face`, decided shape) and the elbow (`injectivity-reads-no-spiric-edge-on-a-torus`, a new arm). SHELL-4 is parked on both.
  - **Class, recorded:** a fix at the reader that copies a producer's decision should first be tried at the producer. This is the second time in two units that the review found the source fix both smaller and concession-free.
- 2026-10-31 — **The FRUSTUM review is in; EPSROWS and SPIRIC are dispatched.**
  - **FRUSTUM (#6114).** A single FULL review at `16e62919fb` returned APPROVE-WITH-FIXES, with no MAJOR. The window holds as a superset and nothing else moved. A fix pass went to the lane:
    - pin the two unsound mutants that survive (*noslant*, *skipmissing*);
    - tighten the wall row's near end;
    - reword `misses_apex` to say "composed total";
    - file the sibling `splitting/rules.rs` `cone_axial_window`, which decides "holds the apex" by a ¾-turn rule.
  - **Class, recorded:** two guards that cover the same case each survive their own mutant (*noreset*, *nonstrict*). A per-step table should mutate overlapping guards together, or name the guard that a step's row actually pins.
  - **Class, recorded:** one fact has four homes. A cone face's axial window and its apex test are computed in `clearance`, `splitting/rules`, `replace_face` and `census/depth_gap`, by two different apex rules.
  - **EPSROWS dispatched.** Branch `clear/epsrows`, single FULL review: the helix half touches the iso-line mint producer. It covers the nightly's ε 1e-12 red on the tee and the long-helix rows, which the FRUSTUM review diagnosed. It is not SPIRIC's code, so it runs as its own unit.
  - **SPIRIC dispatched.** Branch `clear/spiric`, H, a DUAL concurrent pair. It runs before FRUSTUM merges, because its spec already has it merge main as FRUSTUM lands, and SHELL-4 waits on both.
- 2026-10-31 — **FRUSTUM dispatched** (`docs/FRUSTUM-SPEC.md`, branch `clear/frustum`, cloud lane): end a cone face's window at the face when the face does not reach the apex. Single FULL review. SPIRIC follows once FRUSTUM merges, since both touch clearance.rs.
- 2026-10-31 — **FRUSTUM merges (#6114)** with this commit last. A cone face's window ends at its boundary's slant range unless the boundary winds about the axis (`winds_nowhere`). The cavity and the 3/64 operand now hold at both scalars, and nothing else moved.
  - **The fix pass (`6889fd9432`)** removed the slant guard instead of pinning it: it can change no window, because where `0` is in the slant, widening to `0` is a no-op. The argument now sits at `window_of`. It pinned a cap missing a joint (*skipmissing*) and tightened the wall row's ends. The orchestrator read it and accepted it.
  - **Filed:** splitdoor's `cone-axial-window-widens-a-seamed-frustum-to-its-apex` (P2), the second apex rule.
  - **SHELL-4** is still parked, now on SPIRIC alone.
- 2026-10-31 — **Seam (CARVE):** CARVE's embedding-certificate measurement filed four rows on your slate: `clearance-refines-reads-only-the-lower-halves` (E), `injectivity-monotone-arm-reads-a-sliver-knot-span-it-cannot-bound` (M), `injectivity-departure-refuses-at-depth-on-loft-cap-corners-and-twisted-seams` (H) and `injectivity-price-on-tour-lofts-exceeds-the-pair-budget` (H). All four are now P0: CARVE's P0 `self-overlapping-spines-build-and-validate` (the loft and sweep door's embedding certificate) is parked on them. Today the injectivity sweep certifies 5 of the tour's 16 loft and sweep bodies, at 0.1–9 s against builds of 1–20 ms. The measurement scratch and logs are on `carve/embedding-certificate` (`crates/sweep/tests/carve_embed_scratch.rs`, `demos/tour/tests/carve_embed_scratch.rs`), and the env-gated `refines` experiment there is the first row's fix as measured. (CARVE orchestrator)
- 2026-10-31 — **EPSROWS merges (#6160)** with this commit last. **Tier moved from FULL to the orchestrator's read:** the net diff has no source change. The helix's cause (Interval lane frames against f64-skin walls) was already removed on main by #6138's single f64 carry. The lane's producer fix moved no verdict once reverted alone, so it came out. What is left is one test pin: the tee's Interval union now pins "refuses typed, no defect flag" at every ε, instead of one ε's arm.
  - All 21 self-intersection rows pass at default, 1e-6 and 1e-12.
  - **The class grep:** 1590 rows at 1e-12. 5 red, all already filed; 18 ran past 300 s and were appended to the sectrung fine-band row.
  - **Filed:** a fresh 1e-6 main red on QUAD (`lofted-concave-prism-pad-pin-moved-at-1e-6`, P1). It is not CLEAR's code.
- 2026-10-31 — **SPIRIC merges (#6159)** with its dual row last. Departure now certifies a plane cap against a torus across a spiric edge. The torus face is presented by the edge's own wall pcurve, read as a graph over the minor angle (σ = ±1 exactly). The cap's plane is the separator. Torus corners are read through **parallels only**. The klein elbow's cavity and an off-axis torus cut hold at both scalars. A grafted fold reads `Violated` off its spiric edges. Every existing self-intersection receipt is byte-identical.
  - **The dual (H, concurrent, frozen at `b65b2fe19d`).** Both reviews returned APPROVE-WITH-FIXES with no MAJOR, so the blind coding (byte 92) tallies 0. A single fix pass (`080781ffab`) covered the 11-item union. Each item has a mutant that reds its own row. The orchestrator read it and accepted it.
  - **Class, recorded:** the mutant table mutated each step only at the + signs and at canonical representatives (σ = κ = +1, a parallel, a ψ nearly constant along the curve). That is why `rising`, κ, the `u`-rate and a ψ read at one point all survived. A per-step table should mutate each step at every sign and representative its guard admits.
  - **Class, recorded:** `reading` admitted a shape (meridian corners) beyond the one argued, and only reviewers asking what each step NEEDS found it. This is the third arm in a row (CURVEDEP, CORNER, SPIRIC) where an admission outran its argument.
  - **Filed:** `one-face-torus-cut-refuses-across-its-meridian` (P3).
  - **SHELL-4 is unparked** (`blocked_on: []`).
- 2026-10-31 — **SHELL-4 parked again; POTDOC dispatched.** On resume, all of SHELL-4's gate rows are green except one. Editor-core's corpus pot (`vessel::document()`) shells into `CavityUncertified` at Interval: Departure across the mouth rim, between the mouth plane and the sphere belly band. The same pot built in sweep (`corpus_pot`) shells Ok. The refusal therefore follows how the document builds the pot, not its surfaces. Filed as `document-pot-cavity-refuses-departure-at-its-mouth-rim-at-interval` (P1). It is dispatched as POTDOC (`clear/potdoc`, M, single FULL review, diagnosis first, fix at the producer). Over-refusing a valid shell is ruled out, so the gate waits. Editor-core's `m10_3_driver_interval` and `m10_9_pins_interval` reds are GREEN's, not SHELL-4's; they are already filed.
- 2026-11-01 — **POTDOC merges (#6172)** with this commit last. The document pot and sweep's pot are the same B-rep. The document's belly sphere is exact, so at the corner where the seam meets the mouth rim, two enclosures of one vertex, computed by different routes, differ by 2.3e-15. `few_ulps` is 1.8e-15, so `present_chart`'s cover test refused at every depth. The fix is in the reader: `run_on` carries a clip side past a corner it shares with the other axis's line. It sits behind `sides_are_the_faces`, together with that line's coverage and `convex`. It is exact, adds no ε and changes no receipt. The review agreed that a producer fix would have to couple two edges' mints and would not be smaller, so this time the reader was the right home.
  - **The review.** A single FULL review at `1ee245d704` returned APPROVE-WITH-FIXES. Two guards were unpinned: the other-axis filter, and the low arm of the whole-enclosure check. `convex` was unpinned through `present_chart`. The fix pass (`21aeef58`) added all 8 cut corners on two sheets and a T-junction row for `convex`, and removed a check that could never fire. The orchestrator read it and accepted it.
  - **Class, recorded:** for the third unit running (SPIRIC, POTDOC, and FRUSTUM's near end), the negative rows covered only one orientation, all high-side. Mutate both arms and both axes at spec time; the briefs already ask for this.
  - **Filed:** `vacuous-reads-a-cell-at-a-corner-local-at-every-depth` (P3, over-refusal, predates this unit).
  - **SHELL-4 is unparked** and resumed.
- 2026-11-01 — **SHELL-4 merges (#6183)** with this commit last, closing `shell-curved-wall-clearance-window`. At `Interval` and `Sym<Interval>`, `shell` and `shell_open` now check the moved cavity clone for injectivity before inserting it.
  - A verified crossing refuses `CavityCrosses`, carrying a witness: two face keys and their chart stations.
  - An engine refusal comes back as `CavityUncertified` with the refusal verbatim.
  - The planar gates stay at every scalar and run first (item 2).
  - At `f64`, `Probe` and `Dual` nothing changes, and the window stays open there.
  - Every valid body shells Ok at both certifying scalars: the tube, the vessel, the corpus pot (sweep and document) and the frustum. So does the klein elbow. Getting there took five upstream units: RIMSCALAR, CAVITYDIAG, FRUSTUM, SPIRIC and POTDOC.
  - **The review.** A single FULL review at `efa24d2906` returned APPROVE-WITH-FIXES and found no path to Ok on an uncertified cavity. The fix pass (`b8e52882ef`) added a `CavityUncertified` row from a real body (the one-face torus cut), and the refusal→Ok mutant reds it. It also named the planar gate's over-refusal of a valid wedge at Interval. The orchestrator read it and accepted it.
  - **Filed:**
    - `the-planar-wall-clearance-gate-refuses-valid-wedges-at-interval-that-the-cavity-gate-certifies` (P2);
    - CLEARTAIL's `one-home-for-the-clearance-witness-verifier` (P3).
  - **The spec's curved twin and box rows** are pinned to their typed upstream refusals: OFFSET's arc-necked extrude row, and `shell_moved_walls_overlap`.
  - **Not a handback.** CARVE filed four CLEAR rows at P0 on 2026-10-31 (`6d2622ba97`), parking its embedding certificate on them. CLEAR still has dispatchable P0 work, so the sitting goes on.
- 2026-11-01 — **SEAMDIAG merges (#6224)** with this commit last. It is a diagnosis unit; the orchestrator read it and accepted it. The parent row is parked on the two causes it found, each filed P0 with a fix shape:
  - **Twisted seams.** `graph_of` subtracts heading perpendiculars normalised before the along-edge part is dropped. So for two cells offset along a seam, the farther face weighs less, and at a dihedral cosine above `|hy⊥|/|hx⊥|` both faces read one side. The fix is the bisector, two lines, measured by SEAMDIAG: all four seams hold in about 0.2 s. HEADING is dispatched.
  - **Cap corners.** No reading presents a plane cap at a corner of two arcs. The carrier arm would certify it, but it reads the wall's zero side through the centred form over a rational spline net. That is second order in the cell, so it decides only at depth 51–73. ZEROSIDE is dispatched to read the side off its own control points.
  - **`m8_14`'s half-turn helix** no longer reproduces on main; it is pinned as holding.
  - **Rows pinned** at both scalars (the tour rows at f64). HEADING and ZEROSIDE flip them.
- 2026-11-01 — **CARVE's four P0 rows taken up; REFINES, SLIVER and SEAMDIAG dispatched.** CARVE filed four CLEAR rows on 2026-10-31 (`6d2622ba97`) and parked its `self-overlapping-spines-build-and-validate` on them, so CLEAR's sitting continues past SHELL-4.
  - **REFINES** (`clear/refines`, E, single FULL). The orchestrator decided the shape: an axis refines when EITHER half narrows it. This is CARVE's experiment, and it is sound because the door is admission only.
  - **SLIVER** (`clear/sliver`, M, single FULL, diagnosis first). The fix shape (drop or bound the sliver span in `rect_jet`, or snap the split to the knot) is the lane's to weigh.
  - **SEAMDIAG** (`clear/seamdiag`, a diagnosis unit like CAVITYDIAG). It prints the cap-corner and twisted-seam loci and returns a fix shape for each.
  - **The price row** (`injectivity-price-on-tour-lofts-exceeds-the-pair-budget`) is a design fork: a coarser arm or a per-body budget. It waits on SLIVER's and SEAMDIAG's data before the two designers weigh it.
- 2026-11-01 — **REFINES merges (#6206)** with this commit last. `refines` admits an axis when either half narrows the window's box. A box with any non-finite bound narrows nothing (`narrows`).
  - **Now decided rather than turned away at the door:** the arc spine past a full turn, CONTACTHOLD's closed spine and CARVETAIL's loft read `Violated`, all real crossings checked by hand. The long helix holds again. Receipts for faces already admitted are unchanged.
  - **The review.** A single FULL review at `ae2561ad94` returned APPROVE-WITH-FIXES. M5b (min-side only) and M6 (a poison half narrows) survived. The fix pass (`4dfb0439`) added max-side ribbons and the `narrows` finiteness rule with its row, built the real x = 1 bow-tie wall, and made `arc_spine_places` one shared fixture. The orchestrator read it and accepted it.
  - **Main red during the unit:** #6200's blend rewording, filed under CHROME as P0 (#6222) and fixed by #6227.
- 2026-11-01 — **SLIVER merges (#6229)** with this commit last.
  - **The cause.** `restricted_jet` divided by a cell's few-ulp overlap with the next knot span, so rounding became slope at every depth.
  - **The fix.** `SpanRow::restrict_derivative` (geom-core) takes the derivative across the span's own knots, then restricts. `rect_jet` and `span_deriv` read through it. It decides nothing and adds no ε.
  - **Rows.** The curl walls hold, and REFINES's short-of-a-turn row now holds at the default budget.
  - **The review.** A single FULL review at `80e5fcc1eb` returned **APPROVE**. It ran 1.4M exact-oracle checks with 0 misses, and every mutant was killed. A cleanup pass (`905cfeb4`) widened the fuzzer to cells and knots a few ulps off, which now catches lax shape (a); un-skipped wall 4v1; and filed the follow-ups. The orchestrator read it and accepted it.
  - **Filed:**
    - `restrict-derivative-widens-on-a-few-ulp-knot-span` (P3);
    - CLEARTAIL's `derivative-bernstein-row-over-a-sub-interval-has-three-homes` (P3);
    - **`rect-jet-reads-no-point-meet-at-a-cell-end-on-a-knot` (P2).** It was found by the widened fuzzer and is pre-existing. At a knot of multiplicity ≥ the degree, a cell ending on the knot drops the one-point meet, so its jet misses the derivative `ders` reads from the next span. It awaits a ruling on `a <= b`, which would widen jets at every p-fold knot.
- 2026-11-01 — **The price row weighed; JETFRAME dispatched.** The fork drew three designers (byte 42): one Fable and two Opus, labelled A, B and C. The bytes and the mapping are on `analysis/design-fork/injectivity-price`.
  - **All three, independently, rejected the premise.** Tour lofts are expensive mostly because `rect_jet` builds a rational net's derivative in world coordinates, so its width carries the body's distance from the origin and never shrinks with the cell. It is not the geometry, and not the pricing.
  - **They converged on:** fix the enclosure, add an oriented distance test, and keep one fixed pair budget per question (Ev's PR 6102 shape for measurement). They rejected a per-body budget and a coarser arm.
  - **Not an `[ev]` PR,** since there is no split. The one open question is whether loft and sweep doors run the check on every rebuild. That is CARVE's door; the designers advise putting it to Ev with numbers after the fixes land.
  - **Filed:**
    - `rect-jet-forms-the-rational-derivative-in-the-absolute-frame` (P0, dispatched as JETFRAME, `clear/jetframe`, single FULL review with an oracle);
    - `distance-test-separates-boxes-not-directions` (P0, since it blocks the P0 price row);
    - `budget-refusal-names-no-pair` (P2);
    - `monotone-halves-retest-the-hull-that-just-failed` (P3).
  - The price row is parked on the first two.
  - **Class, recorded:** CLEAR has four filed rows whose enclosure width grows with translation. One home for a rational derivative over a cell is the fix; JETFRAME sweeps for it.
- 2026-11-01 — **TRV merges (#6233)** with this commit last. The clearance engine refuses what does not certify.
  - **R1, at the lift.** The lift is now a type, `lift::Lift`, minted only by `certified_lift`, and every reader takes it.
  - **`poisoned(sep)`** gates the root, pair, witness and overflow reads. `overflow` now poisons as `pair` does, instead of flooring at 0.
  - **`present_chart`** asks each clip line where it is placed. `present_plane`'s `[e]` arm asks `y`.
  - **Census:** clearance 25/7, injective 70/27.
  - **The review.** A single FULL review at `5caee7bf7e` returned APPROVE-WITH-FIXES. Six of its 19 mutants survived, all sub-asks or unpinned gates. It also found that the touch read came before the merge's ask. The fix pass (`838c6136de`) pinned every one; 18 of 18 are now killed. On the way it found that the census lane's reading of the `[e]` arm was right, and withdrew the "row is wrong" claim. The orchestrator read it and accepted it.
  - **Class, recorded:** for the fourth unit running, the mutant table pinned one coordinate, one end or one site per class. This time it was the sub-asks inside a site.
  - **Filed:** `injective-reads-past-the-trv-census-decide-on-clamped-enclosures` (P2). SPLICE's census drift was filed separately (#6236).
