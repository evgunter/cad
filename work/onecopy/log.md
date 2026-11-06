# ONECOPY — the log

- 2026-11-03 — Opened by cutting ONEHOME (`work/README.md` Track size) after ONEHOME's orchestrator ran out of usage: ONEHOME measured 70 budget points against 35. The 16 rows it had not started (no lane, no design fork, no in-flight PR riding them) moved here from `work/onehome/` by `git mv`, ids and bodies unchanged, each with a dated `## Slate triage` note; the two unbanded rows (`circle-frame-structs-re-spell-geom-circle`, `interval-safe-point-angle-read-two-ways`) were banded P1 and priced M. 30 budget points dispatchable. Nothing is dispatched. (cut by a session at the user's request)
- 2026-11-03 — Picked up (status `active`). Four lanes and one design fork, cut along the rows' couplings. Every brief adds the program's standing question: why the copy arose, where its home is, and what makes the next instance land there (a type or method over prose). (a) `onecopy/foot-and-angle`: `a-curves-closed-form-foot-is-re-spelled-outside-geom` and `interval-safe-point-angle-read-two-ways` (the circle foot's seed is the angle reading). Tier single FULL, because both move bits on every line and circle foot. (b) `onecopy/face-loops`: `a-faces-loops-are-walked-inline-beside-face-loops-linked` and `the-face-walk-units-follow-ups-from-its-review`. Tier single FULL: each site's miss rule is a decision. (c) `onecopy/curve-pair-reads`: `carrier-refusal-to-error-mapping-is-spelled-at-seven-sites`, `the-curve-pair-units-follow-ups-from-its-review` and `boundary-cuts-reads-line-against-conics-through-their-planes`. Tier single FULL: boundary_cuts changes which cuts a run takes. (d) `onecopy/ladder-climb`: `the-ladders-follow-ups-from-its-review` and `witness-and-door-searches-hand-roll-the-ladder`. Tier DUAL: about twenty searches each pick a ranking and move the k-stats record, and `whole_body_side` misattributes an escalation as Corrupt on main. Arm drawn: byte 81, 81 mod 3 = 0 → HOLDOUT, concurrent pair (protocol `7f7c233d8`). (e) Design fork "where a carrier's placement lives" for `analytic-surfaces-spell-their-frame-field-by-field`, with `circle-frame-structs-re-spell-geom-circle` and the P2 `local-point-direction-line-types-restate-geom-line` as evidence. It goes to designers first, not straight to a lane: the change rewrites ~4000 destructuring sites of `Surface`'s variants and is hard to reverse, and several shapes are viable (a per-variant `Frame` field, or a placement factored out of the intrinsic shape). Draw on `analysis/design-fork/onecopy-placement`: bytes 53, 64, three designers. Rim (`rim_wedge`) waits on ONEDOOR's PR 6443, which rewrites that file. PR 6371 (709 files) is in flight on ONEHOME, so the placement lane, once dispatched, merges main often. (ONECOPY orchestrator)
- 2026-11-03 — Lanes: (a) `onecopy/foot-and-angle` and (b) `onecopy/face-loops` are local subagents; (c) `onecopy/curve-pair-reads` is cloud session `session_013mPGJ2Ps1JGbGEhVVdykF9` and (d) `onecopy/ladder-climb` is cloud session `session_013mPGJ6JvwyvTbNJwVFVQDD`, which freezes its head at green for the dual pair. The three placement designers (A, B, C) run concurrently on one problem statement. (ONECOPY orchestrator)
- 2026-11-03 — Placement fork: the three designers agreed after two rounds, and all three say it lands without Ev (no ratified text changes; an elaboration of `[one-way]` and the PR 6325 conic precedent). So no `[ev]` PR, and no design-fork log row (that population is forks put to Ev). Positions moved this way: gates went from none / one field-shape tripwire / two gates to the single struct-like-variant gate (A's "a sign, not a ban" answered the field-shape nets). The spiric swapped between A and C in round 1 and settled in round 2 on `Spiric { torus, offset }`, with a reader check before landing. Ruled by me: `SurfaceDatum`'s spelling is the stage-3 implementer's, with the user-facing word kept. The decision and its staging are on `analytic-surfaces-spell-their-frame-field-by-field` ("Decided"). That row is parked on stage 1–2 and PRs 6371 and 6443; stages 1–2 are filed as `the-frame-carries-every-placement-reading` (P1, H). The circle-structs row and three P2 rows (local line types, foot on a plane, radial offset) ride it. The class finding for the program's standing question: a carrier kind with no type of its own makes every reader memoize its match in a private struct, and a home whose doors take loose fields gets taken apart. Ask both questions of every future home. Dispatched `onecopy/frame-readings` as a cloud lane. Tier single FULL; stage 3 will take the dual tier (a broad, hard-to-reverse sweep). (ONECOPY orchestrator)
- 2026-11-03 — PR 6449 (`onecopy/foot-and-angle`) is green at `5751445aa5` and under FULL review. The angle row resolved without a fork: pose's angle was `Azimuth::principal` in other words and is deleted. `angle_near` answers a different question (the caller's branch, anchored at its seed), so it stays. An anchored exit on `Azimuth` (`Azimuth::near`) would make one type own both readings, but it contradicts `Azimuth`'s own doc (anchored reads stop Sym folding); it is held for the review before filing. Root cause for the class: the foot copies were written at `T: Real` for a door at `T: Decide` that every caller already met; `param_near` predates `geom::Line`; pose's angle predates `Azimuth` moving into geom. Filed by the lane: `topo-reads-a-lines-parameter-inline-beside-line-param` (P1, E). (ONECOPY orchestrator)
- 2026-11-03 — PR 6449 FULL review: APPROVE-WITH-FIXES, no MAJOR; fix pass sent. Classes recorded: (1) a heuristic's tie-break decided at the caller's band, not `Band::least()`, leaves escalations no answer rests on, and `drive::log_read` can turn one into a refusal. The fix pass returns the pose to the least band and checks `angle_near`'s flip. (2) The fix re-minted its own defect: three hand-kept lists of the closed-form kinds (`Foot::is_nearest`, `chart_singularity::foot`, `nearest_on_edge`). The fix pass makes that one carrier-level reading on `Curve3`. Filed `an-angle-read-clear-of-its-cut-is-spelled-five-ways-and-angle-of-by-hand` (P1, M, design): the anchored-angle family and the hand-written `angle_of` sites the lane's sweep could not see. (ONECOPY orchestrator)
- 2026-11-03 — PR 6451 (`onecopy/curve-pair-reads`) is green at `fd0883bd8` and under FULL review. `CarrierRefusal` is now two arms, `Escalated` and `Unwalked`, so the four-arm sort cannot be written (structural, no gate). `boundary_cuts` reads the boundary pass's meetings; the lane's root cause is that the pass's panic contract looked unusable at the at-rest caller. Filed by the lane: `an-edges-end-vertices-are-spelled-inline-beside-query-edge-ends` (P2, M) and `a-spline-walk-that-could-not-run-reads-as-a-fault-at-two-section-sites` (P2, E). (ONECOPY orchestrator)
- 2026-11-03 — PR 6454 (`onecopy/ladder-climb`) is green and frozen at `146b2a501d`. The dual HOLDOUT pair is dispatched concurrently on that head with identical briefs: R1 is cloud session `session_013mPGJ2jcu2t74obpCMaShx`, R2 is `session_013mPGJ488mB4nRjH15Fe3eT`, and both run the same method. Scope is the follow-ups row plus tranche 1 of the hand-rolled-ladder row. The remainder is filed as `door-searches-left-on-the-hand-rolled-ladder` (P1, M). For the next instance the lane added a gate, `scripts/gates/escalation-discards.sh` (a ratchet on `Err(_) => continue|break|{}`), and `Rung` helpers that make `climb` usable at the shapes that had forced hand-rolled searches. Red main seen on the way: the 1e-12 fuzz row `ssi::certify::tests::a_torus_carriers_limb_two_holds_its_sampled_residual` is the known `work/ssiarith/a-torus-limb-twos-sampled-residual-overshoots-its-span-bound-by-rounding` (SSIARITH's row; the lane added its seed as evidence). (ONECOPY orchestrator)
- 2026-11-03 — PR 6451 FULL review: NOT-MERGEABLE-AS-IS, two MAJORs, fix pass sent. (1) The slow-set pin `d3_conic_trims` is red on head: the PR gate and the lanes' `--profile ci` runs skip the slow set, so the row guarding the change never ran. Class: a lane that changes a reading must run its touched crates' slow set (`cargo nextest run -p <crate>`) before opening the PR. Every brief from now on says so. (2) `boundary_cuts`'s schedule fallback answers wrong where the boundary pass declined (a silent false `StaleContactDeclaration` through the census), and `.ok()?` swallows kernel faults. The fallback predates the PR (`c57ddfceb8`); the PR widened it. Class: a fallback that answers where a method declined is `[loud-failure]`'s named defect. The fix is to carry the refusal's located window as cuts or refuse typed, and to check whether main's narrower fallback already answers wrong (P0 if so). Everyday Booleans are byte-identical on both heads. (ONECOPY orchestrator)
- 2026-11-03 — PR 6454 dual (HOLDOUT) is in: R1 and R2 each APPROVE-WITH-FIXES, no MAJOR, so there are no tally candidates. The correspondence pre-note is kept lane-private until the row is recorded. The union fix pass is sent. Adjudicated against R2's reading of claim 3: a fatal reading under `Any` must not supersede the escalations kept ahead of it (R1's red probe), so the ladder gets an abort outcome. Class, from both reviewers: "only the record moved" claims went unmeasured because CI skips the k-lint and interval jobs on a PR like this. A port that changes when readings happen owes a k-probe differential. The fix pass again re-minted copies (a result-to-rung adapter three times, two rankers in `stands`, `shared_rim` twice), the trap the style lane names. (ONECOPY orchestrator)
- 2026-11-03 — PR 6463 (`onecopy/frame-readings`, stages 1–2 together) is green at `34691c0862` and under FULL review (cloud `session_013mPGJ5DAtvGGgemUKHopYy`). The lane confirmed the decision's causes at every placement site. The line variants had a different cause: they predate `geom::Line`. It filed seven rows, including the full field-shape hit list (`placement-holders-the-frame-sweep-found`) and ~43 further radial-offset sites. A reverted bit-move it found: a one-ulp move of `spline_implicit`'s pivot flips a corner-touch classification (`pivot-on-a-corner-flips-a-corner-touch`). Two suspected nightly census reds (geom-core `bounds_census`, `certified_endpoint_census`) go to the reviewer to check on main. (ONECOPY orchestrator)
- 2026-11-03 — PR 6449 merged (FULL review APPROVE-WITH-FIXES, no MAJOR; delta APPROVE; last commit read by me). `a-curves-closed-form-foot-is-re-spelled-outside-geom` and `interval-safe-point-angle-read-two-ways` close. Homes: `Curve3::foot_is_nearest`/`nearest_foot` (one list of the closed-form kinds), `Curve3::is_periodic` (one reading of a carrier's period), and `Azimuth::principal_as` (the windowed angle under its caller's row). `angle_near`'s flip is unlogged (F31). Filed: `the-side-readers-curved-aim-foot-is-unguarded` (P3). (ONECOPY orchestrator)
- 2026-11-03 — PR 6451 fix pass green at `43391c2fab`, delta review dispatched. The lane took option (b): `Touch` carries no located window, so `boundary_cuts` now refuses typed (`FaceUnread::Boundary`/`Uncertified`) and the schedule fallback is deleted. It claims a P0 on main, pending the delta review's reproduction: a face with a spiric or spline edge, the `all` short-circuit, and non-line loci each answered a silent `Ok(false)`, a false `StaleContactDeclaration`. Filed: `region-landings-read-a-leg-against-an-edge-by-hand-rows-beside-meet` (P1, M). (ONECOPY orchestrator)
- 2026-11-03 — PR 6460 (`onecopy/face-loops`) is green at `1aaa203763` and under FULL review. ~60 inline loop walks now read one home: the order is on `Face::loop_keys`, and the resolution in `live.rs` was extended where sites did not fit. Two second homes are retired. For the next instance: making the fields private was weighed and rejected (~455 reads outside topo, writers in the Euler operators, an accessor still hands out raw keys), so a slow-set source pin was added in its place. The reviewer is asked whether a structural answer exists. Coupled: it edits `boundary_cuts`, as PR 6451 does, so whichever lands second resolves. Filed: `a-faces-loops-are-walked-inline-outside-topo` (P2). Main's nightly carries reds in editor-core, geom-core, geom-brep, sweep and test-utils (run 41583529337). The topo two are open rows on BOXES and FLUSH, not ONECOPY's. (ONECOPY orchestrator)
- 2026-11-03 — PR 6460 FULL review: APPROVE-WITH-FIXES, no input-reachable panic, outcomes byte-identical; fix pass sent. For the program's standing question the key finding is that the fix minted its own trap: newly public `loop_keys()` plus a silent resolve passes both the pin and the gate. The fix pass makes `loop_keys` `pub(crate)` with a defect-class pin. The structural answer, a resolved-face handle in place of the `Body` method grid, is filed as `a-face-is-read-through-a-resolved-handle` (P1, M), parked on this PR. Also in the fix pass: audit the home's lone-vertex skip once at the home, and give loops their role so callers stop re-deriving outer and ring from position. (ONECOPY orchestrator)
- 2026-11-03 — PR 6451 delta APPROVE. The last commit's gate fix was rejected: it held `engine-leaves`' census pin by rewriting the arms as `Self::` inside an `impl` of the engine's type, a second name the counter cannot see. Sent back: home the sort in `boolean/` per `[topo.homes]`, or raise the pin in plain sight with a row, and file the `Self` blind spot on the gate's owner. Class: a lane under a ratchet will satisfy the counter rather than the rule unless told that a visible raise is allowed. (ONECOPY orchestrator)
- 2026-11-03 — PR 6463 FULL review: NOT-MERGEABLE-AS-IS, one MAJOR, fix pass sent. Six editor-core slow-set document digests moved with the disclosed last-bit changes, unshown to be bits only. This is the slow-set class a third time, and the lane had been told the rule after dispatch. Again the fix minted its own instance: `Frame::new(<destructured fields>).axis_line()` in place of `carrier.frame()?.axis_line()`. The pivot row is re-scoped: a corner-touch verdict that moves with the plane's stored origin is a classifier defect on a decision boundary, not a formula reason. Filed `work/issues/main-red-slow-rows-uncovered-at-b2337e9e` (four main reds no row covers). (ONECOPY orchestrator)
- 2026-11-03 — PR 6451 merged (FULL review NOT-MERGEABLE-AS-IS with two MAJORs fixed; delta APPROVE; the gate fix redone by the lane and read by me). `carrier-refusal-to-error-mapping-is-spelled-at-seven-sites`, `the-curve-pair-units-follow-ups-from-its-review` and `boundary-cuts-reads-line-against-conics-through-their-planes` close. Homes: `CarrierRefusal::escalation()` over a two-arm type, and `boundary_cuts` as the boundary pass's meetings, refusing typed. The P0 on main is fixed: a face-order-dependent silent `Ok(false)`, a false `StaleContactDeclaration`, reproduced in review. The census pin was raised 506→510 in plain sight. Filed: `the-census-sort-of-a-boolean-refusal-has-no-legal-home-under-topo-homes` (P2, for Ev's channel when dispatched) and GUARD's `engine-leaves-does-not-count-self-arms-…`. (ONECOPY orchestrator)
- 2026-11-03 — PR 6451 merged (state-sync on its branch). `work/issues/` is retired on main, so the uncovered main-red row moved to GREEN. Dispatched the P1s this sitting filed: (f) `onecopy/small-reads` (cloud `session_013mPGJ3wzJoHHHmdmqhiVZo`), carrying `topo-reads-a-lines-parameter-inline-beside-line-param` and `a-foots-polish-tie-break-escalates-where-no-foot-rests-on-it`, tier single STYLE (FULL if a verdict moves); (g) `onecopy/meet-reads` (cloud `session_013mPGJ2ubR4JnjmKwSmSDkU`), carrying `a-boundary-touch-refusal-locates-no-window` and `region-landings-read-a-leg-against-an-edge-by-hand-rows-beside-meet`, tier single FULL. The anchored-angle row's design question goes to designers; the draw is on `analysis/design-fork/onecopy-anchored-angle` (bytes 60 and 203, three designers). Briefs now carry the slow-set rule, the self-minting warning and the ratchet rule from the start. (ONECOPY orchestrator)
- 2026-11-03 — Anchored-angle fork: the three designers agreed after one round, and all say it lands without Ev. It restores PR 5142's ratified `Azimuth::near`, which agent PR 5225 removed. In round 1 A and C moved to B's composition, the windowed read followed by `Period::near`, a mark-based whole-turn rename. It is unrotated, bit-stable in the anchor, and narrow at both antipodes, as the arithmetic showed. C withdrew "both flips are needed" and conceded the upper tie side. Ruled by me: the name `Period::near`. The chart's cut-side choice stays logged until measured: `the-charts-cut-side-choice-may-leave-the-log-once-measured` (P2). Not a design-fork log row, since it was not put to Ev. Dispatched `onecopy/angle-reads`, tier single FULL. The `atan2` gate is the class answer: the angle questions are now named doors, so a raw angle read has nothing left to be. (ONECOPY orchestrator)
- 2026-11-03 — `onecopy/angle-reads` is cloud session `session_013mPGJ38kwJtrUmvAdwbRGg`. (ONECOPY orchestrator)
- 2026-11-03 — PR 6454 fix pass frozen at `031b099648` and green; delta review sent to R1's session. The ladder gains `Rung::Aborts`, and one result-to-rung adapter replaces four. Filed by the lane: `value-discards-read-a-refusal-as-nothing-found` (P2, 24 sites). Main reds it reports are added to GREEN's `main-red-slow-rows-uncovered-at-b2337e9e`. (ONECOPY orchestrator)
- 2026-11-03 — PR 6475 (`onecopy/small-reads`) green at `dbb655f7a`; the lane raised its own tier to FULL (F32 unlogs an escalation that `drive::log_read` reads), and a FULL review is dispatched. For the next instance, the functions now take a `geom::Line`, so the method is the obvious read. Filed: `a-points-foot-on-a-segment-is-spelled-four-ways` (P2) and GREEN's `main-red-m10-p-fence-interval-corpus-digest-moved`. (ONECOPY orchestrator)
- 2026-11-03 — PR 6454 merged (dual HOLDOUT pair, both APPROVE-WITH-FIXES with no MAJOR; delta APPROVE; last commit read by me). `the-ladders-follow-ups-from-its-review` and `witness-and-door-searches-hand-roll-the-ladder` close; tranche 2 is `door-searches-left-on-the-hand-rolled-ladder`. Homes: `geom_brep::ladder`, with `Rung::Aborts`, `Rung::of_reading`/`Climb::into_reading` and one private non-finite reader (`banded`), and `shared_rim`/`rim_exists` over one private `rims`. For the next instance: the `escalation-discards` gate. (ONECOPY orchestrator)
- 2026-11-03 — PR 6475 FULL review: NOT-MERGEABLE-AS-IS, fix pass sent. F32 is falsified: `census::direction_at` rests on the polish tie's `.t()` without a certificate, and on a hairpin spline the tie reverses a tangent. F32 comes out, and the row is re-banded P3 for its noise half. The class goes to `a-foots-parameter-is-read-without-its-certificate` (P1, M, design): an unlogging argument of the form "no verdict rests on it" holds only if no reader can rest on it, and a bare `t()` lets one. (ONECOPY orchestrator)
- 2026-11-03 — PR 6460 merged (FULL review APPROVE-WITH-FIXES, no MAJOR; delta APPROVE; last commit read by me). `a-faces-loops-are-walked-inline-beside-face-loops-linked` and `the-face-walk-units-follow-ups-from-its-review` close. Homes: `Face::loop_keys`/`holds_loop` (`pub(crate)`, with a defect-class pin), `LoopRole` riding with each key, and the walks in `live.rs` (proven and linked entries; the lone-vertex skip audited once at the home). `a-face-is-read-through-a-resolved-handle` is unparked. GUARD has `engine-leaves-reads-an-associated-type-item-as-the-engines-item` for the pins this PR raised in plain sight. (ONECOPY orchestrator)
- 2026-11-03 — PR 6460 merged (state-sync on its branch). Dispatched `onecopy/face-view` (local lane): `a-face-is-read-through-a-resolved-handle`, with `props-face-start-reads-a-record-key-through-proven` riding. Tier single FULL; the lane checks ratified text first. The uncertified-foot row (`a-foots-parameter-is-read-without-its-certificate`) goes to designers. The draw is on `analysis/design-fork/onecopy-foot-certificate`: bytes 154 and 131, two designers. (ONECOPY orchestrator)
- 2026-11-03 — PR 6477 (`onecopy/meet-reads`) green at `5d30de284b`; FULL review dispatched as a cloud session. Row 1 (`a-boundary-touch-refusal-locates-no-window`) is fixed. The row's premise was corrected: a line locus never meets `RootRefusal::Touch`, so the window comes from the refused member's `Cover` along the line. `curve_face::located` carries each refusal's window, and both cut consumers read its three methods. Row 2 (`region-landings-…`) was tried and reverted. `meet`'s pose swing escalates on two great circles at a small angle, where the landings decide through the on-surface premise. That is a fork, so the row is back to open with `design: true` and waits on designers: `meet` takes a common surface, or an on-surface curve-pair reading becomes the rows' home. (ONECOPY orchestrator)
- 2026-11-03 — PR 6477's review is cloud session `session_013mPGJ2LBz3kX33xkSkVCKp`. The landings fork has two designers; the draw is on `analysis/design-fork/onecopy-landings` (bytes 121, 195). (ONECOPY orchestrator)
- 2026-11-03 — Foot-certificate fork: both designers agreed on their first reports, so no rounds were run. `Foot::t()` goes in favour of three named exits (`On { t }`, `nearest()`, `seed()`), and `direction_at` retires onto the certified `tangent_on_interior`. Only then does the polish tie leave the log. Lands without Ev. Filed the unit `a-foot-has-no-bare-parameter` (P1, M), parked on PR 6475. Class note for the program: an unlogging argument must be structural. "Nothing rests on it" holds only once nothing CAN rest on it. (ONECOPY orchestrator)
- 2026-11-03 — Landings fork: both designers agreed on their first reports; one reproduced both reds on `meet`. Lands without Ev. The decision is on `region-landings-…` ("Decided"): fix `conic_conic`'s screen order and its argument-order-dependent parametrization, then route the landings through `meet`. Class: a reading that loses to hand rows usually has a defect in its own decision order. That is not evidence that a second reading is needed. Dispatched `onecopy/meet-pose`, tier single FULL. (ONECOPY orchestrator)
- 2026-11-03 — PR 6475 merged (FULL review NOT-MERGEABLE-AS-IS; its MAJOR was resolved by withdrawing F32; fix pass read by me). `topo-reads-a-lines-parameter-inline-beside-line-param` closes. The foot-polish row stays open and rides `a-foot-has-no-bare-parameter`, which is now unparked: the tie leaves the log there, once nothing can rest on it. Filed by the lane: `three-line-offsets-wait-on-line-offset` (parked on the frame unit). (ONECOPY orchestrator)
- 2026-11-03 — Dispatched `onecopy/foot-certificate` (`a-foot-has-no-bare-parameter`) to the small-reads cloud session, which already has the hairpin context. Tier single FULL. (ONECOPY orchestrator)
- 2026-11-03 — PR 6477 FULL review: APPROVE-WITH-FIXES, no MAJOR. The two-face oracle over 625 pairs found 0 disagreements; 88 of main's refusals now answer `Ok(true)`, and no answer moved. Fix pass sent. Again the fix held its own trap only by convention: `Located`'s fields are `pub`, so a caller could take the meetings and ignore the windows. The fields go private. The fold also missed a sixth copy of the range idiom in the home's own file. (ONECOPY orchestrator)
- 2026-11-03 — PR 6463 fix pass green at `c121f56cdc`; delta review sent to the original reviewer's session. The six digests moved only through the `Debug` field name, as the lane verified with main's shape. `Round` and `ParallelPose` hold `geom::Circle`, and `FirstHarmonic` takes the carrier. The pivot row moved to SECTARMS as a P1 classifier defect, and the bounds-census stale lines went to TINT. (ONECOPY orchestrator)
- 2026-11-03 — PR 6482 (`onecopy/angle-reads`) green at `d9103cd83`; FULL review dispatched as a cloud session. `Period::near`, `Azimuth::near`, the births and `separation` landed. Four `Interval` workarounds that existed only because the floor was a turn wide are deleted, and several coincidence rows now certify at `Interval`. The `atan2` gate has 79 classed sites. The lane disclosed that its own fix minted copies (`Azimuth::of(w·v, w·u)` by hand ×6, three harmonic phases) and filed them. Class confirmed: a missing narrow home breeds per-site workarounds, and once the home exists the workarounds can be deleted rather than ported. (ONECOPY orchestrator)
- 2026-11-03 — PR 6482 FULL review: APPROVE-WITH-FIXES, one MAJOR, fix pass sent. The MAJOR is a causal story the code does not support: the `Sym` growth the PR attributed to itself is main's drift, so the merge base prints the identical ledger. Class: a moved number is the PR's only after comparing against its OWN merge base, not an older base. Also a loud-failure regression at Interval: a zero-length direction read the reference as if decided. Again the fix minted copies: the tube section 4×, `unwrap_near` once. (ONECOPY orchestrator)
- 2026-11-03 — PR 6477 fix pass green at `25247c0c7`; delta review sent to its reviewer. A main red caused by two PRs that each passed alone: 6460's `loop_keys` pin landed, then SPLICE's `925fa061a` added a bare `.loop_keys()` read in `spline_region_rows.rs`. Fixed by repointing the read to the home: PR 6489 (`onecopy/main-red-loop-keys`), tier orchestrator's read. (ONECOPY orchestrator)
- 2026-11-03 — PR 6489 merged (green; orchestrator's read): main's `loop_keys` pin is green again. (ONECOPY orchestrator)
- 2026-11-03 — PR 6483 (`onecopy/foot-certificate`) green at `b5c962aea`; FULL review dispatched as a cloud session. `Foot::t()` is gone, `direction_at` is deleted, and F32 now rests on structure. The lane built the f64 far-branch row (a point at t=0.35 polishes to t≈0.03 and the certificate escalates) and filed `a-splines-foot-search-misses-the-points-own-branch-at-f64` (26,760 far feet in 726,100 sampled). (ONECOPY orchestrator)
- 2026-11-03 — PR 6477 merged (FULL review APPROVE-WITH-FIXES, no MAJOR; delta APPROVE; last commit read by me). `a-boundary-touch-refusal-locates-no-window` closes. A refused boundary member carries its located window (`Located`, with private fields and three exits), and both cut consumers read it. On the two-face oracle, main's 88 refusals-where-true now answer. Filed: `a-conic-locus-refused-member-gets-no-located-window` (P2). (ONECOPY orchestrator)
- 2026-11-03 — PR 6490 (`onecopy/face-view`) green at `522fab4e1a`; FULL review dispatched as a cloud session. `FaceView` replaces the `Body` method grid, with one door per source and every shape a method on the view; ~300 sites across 85 files. Main minted `outer_cycle_linked`, a fresh grid cell, while the lane was live. That is the class's own evidence: a source × shape grid grows a cell per new pair. Filed: `loop-doors-resolve-a-records-loop-key-through-proven-or-typed` (P3). (ONECOPY orchestrator)
- 2026-11-04 — Meet-pose came back in two halves. The pose fix (screen order, and the less-oblique parametrization giving order symmetry) is clean and moved no consumer. Routing the landings through `meet` (PR 6491) regresses at Interval: `Unsettled::Meter` refusals (124/128 on the sphere-cap door), an end distance read at rounding, and 21 fast rows over 1 s. Sequencing, recommended option taken: land the pose fix alone on `onecopy/meet-pose-order`; 6491 goes to draft and waits on a new row, `meet-at-interval-refuses-on-its-basin-meter-and-reads-an-ends-distance-at-rounding` (P1). Rejected: accepting the cost and re-baselining. Class: a home that wins on f64 and loses at Interval is not yet the home for an interval caller. Measure both scalars before retiring a reading onto it. (ONECOPY orchestrator)
- 2026-11-04 — PR 6463 merged (FULL review NOT-MERGEABLE-AS-IS, its MAJOR shown to be the `Debug` field name; delta APPROVE-WITH-FIXES; last commit read by me). `the-frame-carries-every-placement-reading`, `local-point-direction-line-types-restate-geom-line` and `a-points-foot-on-a-plane-is-spelled-inline` close. `a-points-radial-offset-from-an-axis-is-spelled-inline` stays open for `offset_axial`'s hand-written foot. `circle-frame-structs-re-spell-geom-circle` is parked on stage 3 for `Rim`. Stage 3 (`analytic-surfaces-…`) is parked on PR 6371 alone, now that 6443 has merged. Unparked by this merge: `three-line-offsets-wait-on-line-offset` and `frame-azimuth-waits-on-the-frame-module`. (ONECOPY orchestrator)
- 2026-11-04 — PR 6463 merged (state-sync on its branch). Dispatched tranche 2 of the ladder, `door-searches-left-on-the-hand-rolled-ladder`, as `onecopy/ladder-climb-2` to the ladder lane's session, which has the context. Tier single FULL. (ONECOPY orchestrator)
- 2026-11-04 — PR 6492 (`onecopy/meet-pose-order`, the pose fix alone) green at `00566dbc79`; FULL review dispatched as a cloud session. PR 6491 (the landings) is a draft; it waits on `meet-at-interval-refuses-on-its-basin-meter-and-reads-an-ends-distance-at-rounding`, which lands with 6492. The landings row is back to open until that row's unit runs. (ONECOPY orchestrator)
- 2026-11-04 — PR 6495 (`onecopy/ladder-climb-2`, tranche 2) green and frozen at `f20661c4e7`; FULL review dispatched as a cloud session. Nine searches are ported with the tranche-1 helpers and no new helper. Two ranking decisions are pinned (`Sides::holds`, `slit::holding`). The lane claims "no reading moves into or out of an iterator"; tranche 1's identical claim was false, so the review measures it. (ONECOPY orchestrator)
- 2026-11-04 — PR 6483 FULL review: APPROVE-WITH-FIXES, no MAJOR, with every outcome in a 563-row diff unchanged; fix pass sent. F32 overclaimed: below the band's zero, `On`'s `t` certifies the point but not the branch (a hairpin with h=3e-10 gets the seed's branch at Interval, unlogged), so the ledger states what holds and a row pins it. Class again: `seed()` and `On{t}` return the same field, held apart only by prose. The fix pass adds a reader pin, or a `Seed` newtype if that is clean. Harness lesson from the reviewer: a shared `CARGO_TARGET_DIR` across worktrees silently runs the other tree's binary. (ONECOPY orchestrator)
- 2026-11-04 — PR 6492 FULL review: APPROVE-WITH-FIXES, one MAJOR, fix pass sent. The swing loop still reads the first conic first, so the order symmetry the PR claims fails on exactly the slim-lune shape: the fuzz finds 3 asymmetric pairs, against main's 26. Again the fix minted its own instance: the across pick is a heuristic choice through the logged door, with a dimensionless margin cast through the length door, which is the F31/F32 class the program filed hours earlier. (ONECOPY orchestrator)
- 2026-11-04 — PR 6490 FULL review: APPROVE-WITH-FIXES, no MAJOR; fix pass sent. No new input-reachable panic: all 84 removed `get_face` lines were paired with their replacements, and the public probe is byte-identical. Class: the PR's "no `Body` method reads a shape from a key" was overstated, because three survive. The new source pin was blind to a path-spelled call, a hole copied from its sibling `loop_keys` pin (PR 6460). Both pins get both spellings. (ONECOPY orchestrator)
- 2026-11-04 — PR 6492 fix pass green at `50a28be344`; delta review sent to its reviewer. The swing route is now order-free: the reviewer's fuzz, made a row, reads zero asymmetries over 18,000 draws, where the first head read 17. The across pick is unlogged under F33. The lane's class sweep left `curve_pair::compare` and `walk::step_taken` as the same F31/F32/F33 class; they go onto `a-heuristics-choice-has-no-door-of-its-own` once PR 6483 lands it. (ONECOPY orchestrator)
- 2026-11-04 — PR 6490 fix pass green at `c73e8010bd`; delta review sent to its reviewer. One half-edge list and one whole-faces proof. `Body::face_surface` stays public, because three outside tests landed meanwhile and the view is `pub(crate)`; a pin keeps it unread inside the crate. Both source pins now catch path-spelled calls. (ONECOPY orchestrator)
- 2026-11-04 — PR 6482 fix pass green at `09f151fd3`; delta review sent to its reviewer. The `Sym` causal story is retracted, re-checked against main and re-taken. `Azimuth::near` now reads the hull where the direction's length only reaches zero. The flipped pins check volume against f64 with a width ceiling. `Azimuth::tube` is the one tube section. More main reds go to GREEN. (ONECOPY orchestrator)
- 2026-11-04 — PR 6495 FULL review: APPROVE-WITH-FIXES, one MAJOR, fix pass sent. The `Sides::holds` ranking was argued from tier 3, which never calls it. Its consumer, step-import, reads Loop and Carrier as limits, so the new order inverted `Evidence`. Class: an ordering argument must name its actual consumer. "Exempt" is a consumer's question, not the ladder's. 544 e2e cases are byte-identical, and the reach instrument shows the changed record paths are unreached by every suite, so rows that make them reachable are owed. (ONECOPY orchestrator)
- 2026-11-04 — PR 6492 delta: APPROVE-WITH-FIXES, with order symmetry confirmed. New MAJOR, the class again: the fix kept two spellings of one screen and made their one-ulp disagreement a `Contradiction` fault, reachable on valid input. Sent back for one spelling. Owed by me once PR 6483 lands: add the class-sweep sites (`curve_pair::compare` and its four callers, `walk::step_taken`) to `a-heuristics-choice-has-no-door-of-its-own`. (ONECOPY orchestrator)
- 2026-11-04 — PR 6490 merged (FULL review APPROVE-WITH-FIXES; delta APPROVE; last commit read by me). `a-face-is-read-through-a-resolved-handle` and `props-face-start-reads-a-record-key-through-proven` close. A face is resolved once, by its source, into a `FaceView`, and every shape is a method on it. A row-dropper's proven list comes only from `half_edges_proven_whole`. Source pins catch both call spellings; `Body::face_surface` stays the public door, pinned unread inside the crate. (ONECOPY orchestrator)
- 2026-11-04 — PR 6482 merged (FULL review APPROVE-WITH-FIXES with one MAJOR, a causal story the code does not support, retracted; delta APPROVE-WITH-FIXES; last commit read by me). `an-angle-read-clear-of-its-cut-is-spelled-five-ways-and-angle-of-by-hand` closes. Homes: `Period::near` (whole-turn renames, mark-decided and unlogged) and `Azimuth::near` (windowed read then rename, unrotated). An exact point zero reads the reference; a direction whose length only reaches zero reads the hull. `Conic`/`Frame`/`Azimuth::tube` are the births, `separation` is decision-free, and the `atan2` gate covers 80 sites keyed (file, function, class). (ONECOPY orchestrator)
- 2026-11-04 — PR 6490 merged; face-view lane reclaimed. PR 6482 delta: APPROVE-WITH-FIXES; last commit sent. Two pins are stale from main's later movement. One gap: an underflowing point direction read the reference, which the exact-zero vs reaching-zero split exists to prevent. The circle's `Conic::azimuth` re-spelled `Frame::azimuth`. (ONECOPY orchestrator)
- 2026-11-04 — PR 6492 merged (FULL review APPROVE-WITH-FIXES, MAJOR fixed; delta APPROVE-WITH-FIXES, its MAJOR fixed; last commit read by me). `meet`'s conic pose is order-free (0 asymmetries over 18,000 fuzz draws and 1,575 ladder pairs), reads the parallel argument first through one margin, and picks the less oblique conic unlogged (F33). The landings row stays open, waiting on `meet-at-interval-…` (P1), which lands here. A duplicated PARALLEL/OFFSET read of one margin is noted on that row. (ONECOPY orchestrator)
- 2026-11-04 — PR 6492 merged (gate re-run after the state-sync merge carried main's 6490). Dispatched `onecopy/meet-interval` (local) for `meet-at-interval-…`, which unblocks draft PR 6491's landings. Tier single FULL; the lane stops if what the basin meter certifies at Interval turns out to be a fork. Lesson: a state-sync merge of my branch into a PR branch carries main with it, so it is not docs-only and needs a fresh gate. Merge the state branch before main moves, or gate again. (ONECOPY orchestrator)
- 2026-11-04 — PR 6483 fix pass green at `8e8381d0e`; delta review sent to its reviewer. The seed convention is held by a reader pin, because a `Seed<T>` newtype was weighed and every reader unwraps it for arithmetic. `edge_tangent_at` is the one tangent home. PR 6482's merge raced main twice: GitHub ignores the log's union driver, and a real `.config/nextest.toml` conflict needed a fresh gate. (ONECOPY orchestrator)
- 2026-11-04 — PR 6495 fix pass frozen at `09b97d95cf`; delta review sent. `Unread::kept` is the one ranking, exempting nothing, because exemption belongs to the consumer. The lane also changed `place_on_boundary`'s order, which moves a tier-3 report (a pole met by both readings is now exempt); the delta review judges it. (ONECOPY orchestrator)
- 2026-11-04 — PR 6482's re-gate went red on a semantic clash with PR 6490 (its `face_cycles` was retired onto `FaceView`); sent back to the lane to repoint. The ONEHOME lesson again: after a tree-wide change lands, every in-flight PR that calls into it needs a compile before merge. With PR 6490 merged, the in-flight PRs that touch topo need to merge main and compile: 6483 has been merged three times and is green; 6495's last merge is green. (ONECOPY orchestrator)
- 2026-11-04 — `onecopy/meet-interval` fixed regression (i) at its root. Stepping Gauss–Newton from a whole interval widened every iterate; it now steps from a point, in cross-product form. Ends are read once, and an endless span has no ends. The pinned sphere-cap row passes, and the torus row drops from 14.2 s to 1.1 s. The lane is opening a PR for this; tier single FULL. Ruled by me on its Question 2: the lune precision pin read a coincidental cancellation of two roundings, and the escalation is honest, so the pin is re-posed. No parameter-space end reading. Question 1 is what the polish's meters certify at an enclosing scalar (midpoint search, a metric of what the reading needs, or Krawczyk). It is a fork, with three designers drawn on `analysis/design-fork/onecopy-polish-meters` (bytes 68, 18). (ONECOPY orchestrator)
- 2026-11-04 — PR 6483 delta: APPROVE-WITH-FIXES, last commit sent. The class again, in pin form: a source pin listing FILES lets a new verdict read inside a listed file pass, and two call spellings (turbofish, UFCS) slipped it. Pins should count call sites per file and match every spelling. (ONECOPY orchestrator)
- 2026-11-04 — PR 6482's re-gate went red on a stray `mod zz_probe_dfb;` that main carried briefly; main's PR 6503 dropped it. Main then patched the same stale `face_cycles` call in `self_touch_curved.rs` its own way, through `unchecked_face_cycles`, which conflicted with the lane's fix. I merged main and kept the lane's `proven_face(f).cycles()`, the FaceView home, at `522ae629e6`; the re-gate is running.
- 2026-11-04 — PR 6495 delta review: APPROVE-WITH-FIXES, MAJOR-A sustained. `place_on_boundary`'s re-rank turns a tier-3 `ChartSingularityUndecided` into a silent exemption at the default band, and a coarser band reads it as a real `InsideEdge` defect. `Carrier` is not band-free, because the grazing ellipse settles `On` at 1e-8. Sent back to the lane:
  - restore the order;
  - make `Carrier` not Blocked;
  - write rows at tier 3 and at import;
  - put the limit-or-not classification on the reading type, so no consumer table can misfile a variant again;
  - list `rehome`'s arms rather than wildcard them, with corruption aborting.
  Class: a ranking change needs a row at each consumer it moves, and a label like "limit" belongs where the reading is minted.
- 2026-11-04 — Polish-meters fork: all three first reports are in (`~/.local/share/cad-work/oc-fork5/`).
  - Agreed: the step meter retires, the basin stays, Krawczyk is rejected, and no ratified text changes. The meters guard only completeness, the claim that an Off at the pair speaks for the region. A location along the carriers is conditioned by 1/sin φ, and that conditioning is the whole Interval refusal count.
  - Splits: an f64 lift (B, C) against recentring at T (A, which keeps the Dual tangent); a tangent-plane slack/residual meter (A, B) against Off decided on a clearance lower bound (C, which charges A/B with "two predicates for one fact"); what travel becomes; and band-or-exact for each meter. The round-1 brief is sent.
- 2026-11-04 — Polish-meters fork decided after round 1, without Ev; no ratified text changes. The decision is on the meet-interval row.
  - The search becomes f64 structure. A moved to the lift; no consumer reads a Dual meeting's tangent.
  - Off is one lower bound on the window's gap, the normal gap less the bend; the slack is a term in it, never a row of its own. A and B conceded C's K-premise charge.
  - Travel folds into the basin (A and C; B had taken C's ball, which C dropped).
  - Generator inequalities are read exact and logged, through one band-free `geom_core` door.
  - Each candidate carries `excess: T`, so a meter against the band cannot be typed.
  The lesson for the class: a meter read against the model's band is a unit error that the type system can rule out. Implementation waits for the meet-interval PR, then goes to the same lane.
- 2026-11-04 — PR 6482 merged at `522ae629e6` (`a6f0b5e99c`). I merged main into 6483 and resolved it: `self_touch_curved` takes main's read; the F31 audit takes 6482's text; the lever row's two seed notes are both kept. Compiling caught a clash git could not see: 6482 dropped `on_carrier`'s `check_unlogged` import, and 6483's F32 reads it. Clippy, the gates, geom (565) and topo census/link/touch (546) all pass. The state-sync rides last, and the gate is running at `a56784c9d6`. The disk filled mid-run; I reclaimed the finished lanes' targets and clones (meet-pose, rev-loops, rev-cpr2, foot-designer-a, tgt-6482).
- 2026-11-04 — PR 6495's second delta review: APPROVE-WITH-FIXES; MAJOR-A and MINOR-B are closed. The new rank types re-spell partitions that already exist. Sent the lane one last commit:
  - `piece_reading` reads `ContainError::refusal()`; slit and rehome disagreed on three arms;
  - `PointInSolidError::refusal` is derived from its own `in_band`/`confined`/`sideless`/`inconclusive`;
  - the doc splice is undone, and the dead `Sides::holds` channel goes;
  - `Standing` folds into `ladder::Refusal` with an `Unsettled` arm if that takes ≤150 lines; otherwise it is filed as a row.

  Class: a fix that names a single home ("so no reader spells a second table") has to grep for the second table before it says so.
- 2026-11-04 — PR 6483 merged. It had a FULL review with MAJORs, a delta review returning APPROVE-WITH-FIXES, and a last commit I read. Three rows close: `a-foot-has-no-bare-parameter`, `a-foots-parameter-is-read-without-its-certificate` and `a-foots-polish-tie-break-escalates-where-no-foot-rests-on-it`.
  - Homes: `Foot::{nearest, seed}` with `On { t }` certified by the gap. `census::curved::edge_tangent_at` is the one certified edge tangent, `census::edge_line` the one straightness reading, and `EdgeTangentUnread` is typed. The seed pin counts reads per file in every spelling. F32 is unlogged, and it says what the driver loses.
  - My merge of main: `self_touch_curved` takes main's FaceView read. The audit's F31 takes 6482's widened text. `on_carrier.rs` re-imports `check_unlogged`: 6482 dropped the import with `angle_near`, while 6483's F32 reads it. That is a semantic clash git could not see, caught by the compile.
- 2026-11-04 — PR 6483 merged at `a56784c9d6` (`d998a185b7`). Its three rows close. The class-sweep sites (`curve_pair::compare` and its four callers, and walk's `step_taken`) are added to `a-heuristics-choice-has-no-door-of-its-own`. That row now holds three doors to sort the `Band::least()` sites into: a choice (unlogged, with a ledger row), a proof's inequality (exact, logged, no band; decided by the polish-meters fork), and a reading (against the band). The two new doors are to be designed together.
- 2026-11-04 — PR 6495 (ladder tranche 2) merged. It had a FULL review with one MAJOR (an ordering argued from a non-consumer), a first delta with MAJOR-A (`place_on_boundary`'s re-rank hid an in-band placement), a second delta with copies the fix minted, and a last commit I read.
  - Homes: one `ladder::Refusal` with `Unsettled`, on `Evidence`'s third slot. Each reading type states its own `refusal()`, and `PointInSolidError`'s is derived from its partition. Slit and rehome read `ContainError::refusal`.
  - Disclosed behaviour changes: rehome and slit abort on body-level refusals and corruptions that were limits.
  - `door-searches-left-on-the-hand-rolled-ladder` goes back to open, because `shared_run`, `offset_axial`, `measured_ends` and `seam_germ_side` remain.
  - My merge of main was clean; clippy, gates and lint are clean, and topo, geom-brep and step-import ran before merging.
- 2026-11-04 — PR 6495 merged at `9f2b832db3` (`d15c34e863`). My merge of main was clean. The touched rows, slow set included, show 1210 of 1215; the 5 failures are 4 main reds plus one per-op-postcondition pinned red that `--run-ignored all` ran. The lane at session_013mPGJ6JvwyvTbNJwVFVQDD is released.
- 2026-11-04 — Two P0 main reds at ε 1e-12 were filed onto ONECOPY by PCERT, each bisected to one of our merges. Both are dispatched as cloud lanes, root cause first, with a fix at the one home and never by re-spelling at the call site:
  - section_end_poses' map residual, after PR 6482: `onecopy/p0-section-end`, session_013mPGJ3A3Kdic2ksHSq47MQ;
  - the needle's meet count, after PR 6492: `onecopy/p0-needle`, session_013mPGJ2oAi47f4EfDzt4XFu, told about the polish-meters decision and to check `onecopy/meet-interval`.

  Lesson: our lanes ran the touched crates' slow sets at the default ε only. Sweep at 1e-12 is the ci profile's run, which neither the PR gate nor the lanes reached. Briefs that touch readings now require all three ε on sweep.
- 2026-11-04 — The meet-interval lane opened PR 6516 at `b80329354c`, with the gate green. It carries:
  - the polish stepping from a point in the cross-product form;
  - a span's ends read once;
  - the height screen read once;
  - the lune pin re-posed 2e-11 off the vertex, as I ruled.

  The lane's full suites (geom-brep, topo, sweep) show the same reds at head and base, at the default ε. A FULL review is dispatched to session_013mPGJ1mbS3kUq2KmhnB6J7. It also runs sweep and geom-brep at 1e-12 against base and reports on the two P0s, and it judges the PR as an interim step before the decided f64-lift design. The local lane af87d40c9022afb11 has finished, and its pose target dir is reclaimed. The polish-meters implementation goes out after 6516 lands.
- 2026-11-04 — PR 6516's FULL review: APPROVE-WITH-FIXES with one MAJOR, all sustained, and a fix pass sent.
  - **M1.** Sweep's `band_ruled_cap_ring` row is pinned red at 1e-12 on the CPAIR row this PR fixes, so merging would turn nightly red. One finding had two homes (our row and CPAIR's), and a pin that cites a row is not found when that row is fixed.
  - **Minors.** Walk and census `run_ends` still read span ends by hand. The re-posed lune pin no longer guards its precision branch; mutation proved it.
  - **Notes.**
    - `place` is a third bracket-midpoint spelling, NaN on an unbounded enclosure, and it drops the tangent at `Dual<Interval>`, which has no bounds_census record.
    - `Span` has four doors over two facts, and its pub `end_point` bypasses `ended`.
  - **Class lesson.** A pin names its row; fixing the row must find the pin (a link check is owed). And "every site reads it" claims need a grep that matches every spelling.
- 2026-11-04 — The P0 section-end lane opened PR 6522.
  - **Root cause.** No angle-read defect. 6482's `azimuth(p).near(0)` puts the chord end one ulp from the old `angle_of`, and it is the closer one. With a carrier speed of about 1e5, that ulp is the whole end miss. The old decision and the new MapResidual escalation (`ulp(30000.3) = 3.64e-12`) measure the same rounding, and the last bit chooses which check meets it.
  - **The class predates 6482.** At `e9eaae65`, 17 pcurve refusals and 149 finish refusals were already untyped.
  - **Fix.** Test-only: the witness pose is re-posed (`0.3 → 0.5`, which escalates its end pin at both commits), with its argument written in the doc.
  - **Filed.** `charttail/an-unresolved-section-carrier-refuses-downstream-untyped-at-1e-12`, the structural answer.
  - **My read.** Accepted. Main had moved 80 code files, so I merged main via update-branch (`b2a57369be`). The gate is re-running, and I am running the row at all three ε locally.
- 2026-11-04 — The P0 needle lane opened PR 6525 at `85fcd7451`, gate green. Its root cause: two rim conics tangent at a shared vertex have across scores of exactly 0, so F33's bare difference at `Band::least()` was decided by rounding. It picked the circle, whose touch window cannot certify.
  - **Fix.** Each score carries a rounding charge (the parallel screen's noise), and a tie reads the first conic.
  - **Filed.** `meet-reads-two-conics-tangent-at-a-shared-end-in-one-order-only`, which the lane says predates 6492.
  - **FULL review dispatched** to a cloud session. It judges whether a charge on a choice falls under `[simple.conditioned]`, whether an order-dependent tie-break can land as an interim step, and whether the charge is derived or guessed.
  - **Coupling.** This touches `pose.rs` alongside PR 6516 and the planned polish redesign, so the merge order is 6522, then whichever of 6516 and 6525 is ready first, with the other merging main.
- 2026-11-04 — PR 6522 merged at `b2a57369be` (`92f8ec60a8`), after my merge of main, a green gate, and `section_end_poses` 6/6 at default, 1e-6 and 1e-12 locally. The section-end P0 closes. The lane at session_013mPGJ3A3Kdic2ksHSq47MQ is released.
- 2026-11-04 — PR 6525's FULL review (cloud `session_013mPGJ4XmZV2KjkLsnViPMU`) says NOT-MERGEABLE-AS-IS.
  - **What is confirmed.** The causal story holds to the bit, and no test outcome moves elsewhere.
  - **MAJOR-1.** The charge plus the first-conic tie-break makes `meet` order-dependent on conics tangent at a shared end: 123/400 random pairs on head against 12/400 on main, and 13/27 spun needle poses against 0/27. On main, the bare difference's rounding coin is antisymmetric, so it is order-free.
  - **MAJOR-2.** At tilts of about 1e-6 to 1e-4 rad, the score decisively picks the reading that cannot certify, so both orders refuse. The class is wider than exact tangency.
  - **Minors.** The charge is half its bound and is a second spelling of `running::Rounded`. F33's ledger reverses the two orders. Also `[simple.conditioned]`.
  - **What happens next.** The across choice at a tangent touch is a fork: what `meet` reads there, and where the rule lives. The draw on `analysis/design-fork/onecopy-across-key` is bytes 34 and 131, so three designers. The brief and the review are in `~/.local/share/cad-work/oc-fork6/`.
  - **The lane** (`session_013mPGJ2oAi47f4EfDzt4XFu`) holds, with its minors kept local.
  - **The P0 stays red** until the decision. I judged that a fix which breaks order-freeness across a class is worse than one red nightly row. (ONECOPY orchestrator)
- 2026-11-04 — Across-key fork: all three first reports are in (`~/.local/share/cad-work/oc-fork6/report-{a,b,c}.md`).
  - **Where they agree.**
    - The defect is the window's level `F`, not the across choice. `F`'s term-bound charge, about 9.7e-13 at the needle, is the size of the band's zero, so reading across the long ellipse's plane cannot certify the touch.
    - Keep main's bare antisymmetric `sb − sa`. 6525's charge and its first-conic tie come out.
    - No ratified text changes.
  - **Where they split: what `F` becomes.**
    - A: uncharged at f64, per `[simple.conditioned]`, re-posed where it loses digits. Landing: zero the charge, measured 400/400 with suites green.
    - B: posed about an anchor on the conic it measures from.
    - C: a local jet per piece, with per-order error terms.
  - **Round 1** asks four questions:
    - whether the error terms survive the clause's no-charge text;
    - whether A's uncharged global `F` decides correctly where it is ill-conditioned;
    - whether this is one final state;
    - which P0 landing is correct today. (ONECOPY orchestrator)
- 2026-11-04 — Across-key fork decided after round 1, without Ev; no ratified text changes. Round-1 answers are in `~/.local/share/cad-work/oc-fork6/round1-{a,b,c}.md`.
  - **What moved.**
    - A withdrew "zero the charge now": its own algebra gives δD ≈ 4.4e-11 > e at a far, thin pair.
    - B withdrew its anchored form, whose term bound is a charge with coefficients still of order (R_a/s_b)².
    - C dropped its per-order error terms, measured 3/800 global-uncharged certificates on noise, and called the remainder exact, not rounding.
  - **The decision.**
    - Main's bare across difference stays.
    - The window's level is read locally at each piece, uncharged at f64. `level_on` yields a `Level` with only local readings.
    - The level-to-distance conversion is local too, if it stays small.
    - Order rows for both orders, the spun needle and random tangent pairs.
  - **Rows to file:** `coplanar_ellipses`'s global level, the near-tangent tilt class, the `rounding_charge` gate gap, and F33 onto a `Tie` door.
  - **Class finding.** A chooser that is asked which reading can certify is a symptom: one of the readings is crippled. Fix that reading, and keep the chooser conditioning-only.
  - Sent to the 6525 lane. The review gets a delta pass. (ONECOPY orchestrator)
- 2026-11-04 — PR 6516's fix pass is at `ed74313f07`, with the gate green.
  - **Suites:** no outcome moved against base `354f9b3f`, at head and base, slow set included.
  - **Departures I accepted:**
    - n1 goes in the census's cannot-match list, not the roster, since the roster is exact against the sole doors.
    - Q7 is not taken, because it would be a second spelling of `Span::branch`.
    - `geom::bracket_mid` is the midpoint's one formula, read by `projection_policy::mid` and `place`.
  - **An extra removal:** `band_planar_cut_off_interval`'s `tier_3_prime` admission. That meter no longer fires.
  - **The lane found a second main red at 1e-12:** the atan2-cut fillet witness at shear 0.6, concave, escalates its pcurve envelope. I filed it as P0 `main-red-a-sheared-fillet-witness-escalates-its-pcurve-envelope-at-1e-12`.
  - **Caveat for PR 6491:** `meet`'s route reads the new hair pin at a rounding about 40× the hair.
  - A delta review goes to the original reviewer. (ONECOPY orchestrator)
- 2026-11-04 — Dispatched `onecopy/p0-sheared-fillet` (cloud `session_013mPGJ41MUUAhMA5NUsRfC4`) on the sheared-fillet main red at 1e-12. Tier single FULL. The lane stops if the escalation is honest or the fix is a fork. (ONECOPY orchestrator)
- 2026-11-04 — PR 6525 is at `3bfdc8f48`, with the gate green.
  - **What landed:** the decision, plus a local `H`. The premise that `H` was posed locally was false. Its global charge, about 7.5e-13 at coordinates near 180, also made main's slow-set `leaning_prisms…` red at 1e-12, and that is now fixed too.
  - **Tests:** no green→red. The needle, leaning prisms and the new order rows go red→green.
  - **Rows filed:** `coplanar_ellipses`'s global level, the near-tangent tilt class, and the `rounding_charge` gate gap.
  - **Not landed:** the far, thin oracle row. Its construction did not bite.
  - **Next:** a delta review goes to the original reviewer. (ONECOPY orchestrator)
- 2026-11-04 — PR 6516's delta review: APPROVE-WITH-FIXES.
  - **Fix 1: the hair pin pinned a coincidence.** The reviewer retracted its own number: 1e-11 + 3e-18 is inside the 1.22e-16 end rounding. Ruled by me, option (a): drop the pin and record at `to_end` that the branch is unguardable at an honest offset. This follows my Q2 ruling.
  - **Fix 2: the midpoint docs overclaim.** `bracket_mid` is the one spelling of its *form*. `krawczyk::midpoint` moves onto it with no bit change. A P3 row names the roughly 30 inline midpoints.
  - **A roster red that is not ours:** `bounds_census`'s roster is red on main, and TINT's open row `bounds-census-stale-roster-lines` covers it.
  - The fixes went to the local lane. (ONECOPY orchestrator)
- 2026-11-04 — PR 6525 delta review: MERGEABLE WITH FIXES.
  - **Confirmed:** the jets and conversion (ii) are sound, with 0 Interval contradictions. Order-free: 400/400 tangent pairs decide in both orders, against 302 on main.
  - **The fixes:**
    - Land the reviewer's far/thin row. It is red when the window reads the global `F` uncharged, which is the regression the design names.
    - Guard (ii) or call it unguardable.
    - Move the fuzz onto the vertex distribution.
    - Add `transverse`'s `Range` noise charge to the charge row.
    - Fix the stale closed row.
    - Spell the `1+32ε` raise once.
  - The fixes went to the lane. (ONECOPY orchestrator)
- 2026-11-04 — PR 6525 merged at `d6387c50de` (`b5d0d88052`). The gate is green, and the lane's state-sync closes the needle P0 and the tangent row. Both 1e-12 main reds in `band_turn_overrun*` are fixed: the needle and `leaning_prisms`. The window reads `H` and `F` locally, uncharged at f64. The lane at `session_013mPGJ2oAi47f4EfDzt4XFu` is released. (ONECOPY orchestrator)
- 2026-11-04 — PR 6533 (`onecopy/p0-sheared-fillet`) is at `9819a637ed`, with CI green.
  - **The mechanism:** a lossy read. `plane_plane_blend` stored `u_ref` as `(foot − center).normalize()`, about 1.8e-12 wide at Interval. The "control" was the witness's mirror image.
  - **The fix:** `SupportTrace::foot` is the trace's foot direction, and the control admission is removed. The headroom is thin: 0.89e-12 against z.
  - **Review:** a FULL review was dispatched as cloud `session_013mPGJ3HVYqYAoquA4iThSZ`.
  - **Filed:** GREEN P1 `main-red-the-four-thread-walk-misses-tilted-cut-uppers-serial-golden`, an uncovered slow-set main red that two lanes saw. (ONECOPY orchestrator)
- 2026-11-04 — PR 6516 merged at `ef82097926` (`b9cc0096cb`).
  - **Before the merge:** the gate was green, and the branch was already up to date with main, 6525 included.
  - **The row stays open:** the meet-interval row holds the polish-meters decision, which is not yet implemented.
  - **Dispatched:** `onecopy/polish-meters` to the same local lane, as planned. Tier single FULL. (ONECOPY orchestrator)
- 2026-11-04 — Polish-meters: the lane stopped before writing code. Item 4's "one exact logged door for four sites, nothing unlogged" put two heuristic choices onto a proof door: `curve_foot_polish` (F32, pinned unlogged) and walk's `step_taken`. Ruled by me: the proof door takes only `compare` and the basin; F32 and F33 stay where they are; `step_taken` waits for the choice door. Accepted the lane's exact bend, `m₋` from the quadratic, in which `e` enters the move. The amendment is on the row. The class lesson: a decision that names call sites should have them read before it is ratified, because a designer reading the class can miss a site's own ledger row. (ONECOPY orchestrator)
- 2026-11-04 — Polish-meters, second stop. The decision's composition could not prune carriers that meet past a span's end, the two-circles-on-a-sphere case main decides today. Ruled by me: the lane's (A). The candidate carries `reach = m₋`, and an Off-by-past prunes only when past by more than `reach`, decided against the band. A location prunes but never accepts. This is gated on an escalation census against main at all three ε. (B), a certified convergence premise at T, was rejected as the conditioning the fork removed. The class lesson: when retiring a premise, list every prune that rested on it. (ONECOPY orchestrator)
- 2026-11-04 — PR 6533 FULL review: FIX FIRST.
  - **MAJOR:** `attach_contact`'s TrimLine arm re-reads the band axis off the endpoint vertices (`surgery.rs:5317`), the same lossy read one step downstream. It is red at shear −0.8 at 1e-12. The stored axis brings the worst envelope down from 1.04e-12 to 0.50e-12.
  - **Smaller:** the siblings of `sided` (`corner_ball`, `build.rs`), the copy sweep's blind pattern, and stale doc lines.
  - **Sent to the lane.** Class note: a sweep for copies by regex finds only the spellings its author already imagined. The type that carries the stored datum (`ContactCarrier::TrimLine` with its line) is what makes the next one land. (ONECOPY orchestrator)

- 2026-11-04 — PR 6533 fix pass reported at head `a9ca21bd`, with hosted CI green. TrimLine now carries the stored trimline, and the worst envelope over shears ±0.1 to ±0.8 is 1.57e-13 against a 6e-13 bound. The new pins go red when the old reads are restored. `corner_ball` and build.rs route through `sided`. S3 is filed at `work/band/a-blend-arcs-seam-reference-is-read-off-its-vertex-less-its-centre.md`. The slow set is 94/94 at both ε, against base 91/92 at 1e-12. The delta review is sent to the reviewer.

- 2026-11-04 — PR 6533 delta review: MERGE. The reviewer restored each old read and saw all three pins go red. The `placed` closure leaves Transverse unchanged, and the sided routing matches the old values. Notes that do not gate the merge, both to go on the band row after the merge:
  - NOTE-1: the chamfer's support trimlines still re-read τ through `Chord` (`planar.rs:564`, `surgery.rs:5333-5336`), though the arm stores `Line{foot,tau}`. They are planes, so it probably costs nothing.
  - A slip: only shear −0.8 was red, not ±0.8.
  Main (PR 6530/6534, splice) merged into the branch cleanly as `c47276d9`; only the nextest slow-set lines were combined. The gate is running.

- 2026-11-04 — PR 6533 merged as `fe78065195`, gated on `c47276d9`. That closes the sheared-fillet P0. Both delta-review notes went onto `work/band/a-blend-arcs-seam-reference-is-read-off-its-vertex-less-its-centre.md`: the chamfer `Chord` re-read of τ, and the shear −0.8 correction (also made on the closed P0 row).

- 2026-11-04 — Dispatched cloud lane `session_013mPGJ4o1bDkCjbhg6Kqiat` on the P1 main red `main-red-the-m7-8-plain-names-probe-fails-after-the-faceview-fix-pass`, branch `onecopy/m7-8-plain-names`. The brief: reproduce it, bisect (suspect `e383ccfbcc`), then fix it as a regression at its home or re-pin it as a legitimate move.

- 2026-11-05 — The m7-8-plain-names lane opened PR 6535 at head `193e062b`. The move is legitimate: PR 6498 (the spline chart's incidence read) moved the wall's refusal to `CarrierOffChart`, and FaceView did not cause it. The bisect: fd1c1949 pass, f6e97bd6 fail. The probe now matches through the home classifier `IncidenceRead::of`. It landed red because the probe is in the slow set, which PR 6498's per-PR gate never ran. FULL review sent to `session_013mPGJ3HVYqYAoquA4iThSZ`.

- 2026-11-05 — PR 6535 FULL review: MERGE. The reviewer reproduced the bisect at f3bdc9ab and f6e97bd6. The new refusal is truthful: a 0.00208 margin against a true offset of about 0.0099. The match is no looser than before, and the "edge by edge and nothing else" pin stands. I sent a small fix pass:
  - MINOR-1, the export row's "pins" made "would pin";
  - a Resolution on the closed row naming PR 6498;
  - a two-sided bound on the margin;
  - a P3 row for the next by-name pin (`topo/tests/review_m3_pr4.rs:719-731`).

- 2026-11-05 — The 6535 fix pass is at head `f1b634bb`. The lane bounded the margin by `bow·2/9`, not the reviewer's `bow·(4/9)²`. The reviewer's figure is the smallest edge height, at the corners; a mid-edge sample may legitimately sit up to `bow·2/9` off, so the lane's bound is the sound one. The mutation check (bound 1e-3) goes red. A P3 row was filed for the cone pin. I updated the PR body myself; the gate is running.

- 2026-11-05 — PR 6535 merged as `dd446ad83e`, gated on `f1b634bb`. The m7_8 plain-names main red is closed.

- 2026-11-05 — Dispatched cloud lane `session_013mPGJ3PFKaTss2xjCqxX2Y` on P1 `door-searches-left-on-the-hand-rolled-ladder`, branch `onecopy/ladder-climb-3`. The lane ports the clean sites onto `climb`. For the ranking forks (`shared_run`, `measured_ends`) and `offset_axial`'s third outcome, it measures first and sends me the question rather than choosing.

- 2026-11-05 — ladder-climb-3 rulings, measured first:
  - Q1: census's four `InteriorsUnread` loops take Evidence's order via `InteriorsUnread::refusal`, with the limit before in band. That retires the two-orders copy. A row is filed for `of` folding `Unsettled` into Unbuilt.
  - Q2: `measured_ends` is ported as an identity under Any.
  - Q3: `offset_axial` gets two ladders. Ladder 1's in-band evidence refuses ahead of any tangent foot; no kept-answer outcome.
  - Q4: the agreement checks stay. `climb_agreeing` is filed as a design row.
  - Q5: `link::Cone::reads_given` is deferred to its own row. It is reached about 500 times per suite, and Unplaced's class needs ruling from what it means.
  The clean ports already made are `ssi::refine::fit_minimum` and `vtxfac::faces_holding_pierce`.

- 2026-11-05 — ladder-climb-3 opened PR 6536 at head `15fb5796`, with the CI gate ok.
  - Ported: census's 4 loops in Evidence order, `measured_ends`, and `offset_axial`'s two ladders, all with pins red on main's order. Also `fit_minimum` and `faces_holding_pierce`, and `Evidence::keep` added to ladder.
  - Four P2 rows filed. The row is narrowed to cost E: the pierce pass and the Any ports' unreached record rows.
  - The ci runs are green at both ε. FULL review sent to `session_013mPGJ3HVYqYAoquA4iThSZ`.

- 2026-11-05 — PR 6536 FULL review: MERGE. The three pins go red under main's rule, and every port keeps its outcome except where ruled. Fix pass sent:
  - MINOR-1, a minted copy: "the ranked refusal, or none" is spelled four times. It gets one home, `Evidence::refusal`, and all four route through it.
  - MINOR-2: the test lever moves to the fixture door, so `lever-sources` stays at 2.
  - NOTE-3: the remaining `solve_pairs` combinations are disclosed.
  - NOTE-4: the duplicate census row is folded into splitdoor's.
  Class lesson: a lane that retires one hand-spelled fold (`keep`) can leave its sibling fold (`refusal`) copied. A review should ask about every fold over the same enum.

- 2026-11-05 — PR 6536 delta review: MERGE. `Evidence::refusal` is the only spelling of the fold, the three pins are still red under main's rule, and the splitdoor verdict is untouched. Merged as `dbce1f3a9a`, gated on `251980e8`. The review's NOTE, that a refusal's standing now has two return types (`Refusal::rung` and `Kept`), is filed as P3 `a-refusals-standing-has-two-return-types`.

- 2026-11-05 — The polish-meters cost gate tripped: about 6 poses in three sweep tests (a keel near-tangent to a bore, a steep ellipse's end, and a moved copy) that main decides and (A) escalated on the reach rows. No test outcome moved, and geom-brep and topo were identical at all ε. Third amendment ruled: (C), the run argument posed on the basin's monotonicity. If `p` is decided in band, `d` is monotone to the end, and the end is decidedly Off, the pose prunes. The lane resumed for implementation and a sweep census re-run.
  Class lesson: a bound that is sound but loose (reach ≈ (σ+e)/sin φ) costs exactly where sin φ is small. Prefer an argument from a decided reading (the end's own Off) over a bound on travel.

- 2026-11-05 — The polish-meters lane opened PR 6537 at head `71936f1cc4`, and the cost gate passes after (C).
  - No test outcome moves at any ε.
  - One pose goes from decided to escalated at every ε: the steep ellipse's corner, with each end on the other carrier within 1.5e-8, so it is honest in band. The keel poses decide again.
  - Refused → decided: 27 at the default ε and 132 at 1e-12.
  - Departure 3: a point-valued polish step is taken at T, not as a literal, because editor-core's coincidence door reads the `Sym` symbols.
  FULL review sent to `session_013mPGJ3HVYqYAoquA4iThSZ`, with a soundness sample of the newly decided poses against an oracle.
  Filed two main reds the lane found: the K probe sweep doesn't compile under `probe` alone, and `m10_sym_profile`'s pinned rows are red on main.

- 2026-11-06 — PR 6537 FULL review: MERGE, with every prune's derivation checked.
  - Oracle: 0 contradictions over 51 moved poses and 0 mismatches over 72 run-arm samples at both ε. The escalating pose is an honest 1.5e near-miss.
  - Fix pass sent:
    - NOTE-4: derive the monotonicity premise the run arm rests on from the basin's proof, or narrow the arm.
    - NOTE-2: the basin premise is spelled twice (pose and walk); give it one home.
    - NOTE-3 and NOTE-5: the end-chord siblings and `past_by`'s unreachable premise.
    - The PR body reconciles 132 against 24 (the census sets differ).
  The m10_sym MINOR is already filed.

- 2026-11-06 — The 6537 fix pass is at head `d4cfba9fdd`.
  - NOTE-4: the run arm is derived, not assumed. Run pairs lie within `m₋`, the affine gap is convex, so a crossed end is within `e + 2·bend`. An end clears by its chord past `m₋` or its clearance past `e + 2·bend`; spline ends are not read.
  - NOTE-2: the basin has one home, `polish::bound`.
  - NOTE-3: `run_past` is folded into `past_by`.
  - NOTE-5: the unreachable is replaced by a Meter refusal.
  The `ci` census is unchanged. Delta review sent.

- 2026-11-06 — PR 6537 delta review: MERGE.
  - The derivation was checked step by step, including that the basin excludes the far root `m₊` and that b's span is covered symmetrically.
  - Oracle at head: 3 run-arm poses and 46 reach poses, 0 wrong. No outcome moved against 71936f1c.
  - The previous report's "run arm about 6,700" was the attribution order; the reach clears all but 4 calls.
  - Two small notes are filed as P3 `polish-bound-spells-unit-and-normalises-n-by-hand`. The gate is running on `d4cfba9fdd`.

- 2026-11-06 — PR 6537 merged as `0025d65e28`, gated on `d4cfba9fdd`. The polish-meters decision is landed: meet certifies at Interval from its window, basin, reach and run arms. Next is unblocking draft PR 6491 (landings).

- 2026-11-06 — Dispatched cloud lane `session_013mPGJ4BHWTqt3QU2NKLLBa` to take over draft PR 6491 (`onecopy/meet-pose`, the region landings through `meet`), now that `meet` reads at Interval. The brief:
  - merge main (about 700 commits), taking main's pose and across choice: F33 is the bare difference, so the branch's tie-reads-first must not return;
  - re-measure the Interval cost (the rounded bump at 2.4 s, the cone row at 126 s against 47 s) and the hair-pin caveat, and hide nothing in the slow set;
  - the lane stops on any design misfit.
  The polish-meters census worktrees and logs are reclaimed: disk is at 24G free.

- 2026-11-06 — The 6491 takeover stopped on its stop condition after merging main (`a5fab8a98`). Main's `meet` reads two 1e-12 rows differently from the rows: the far ray's leg is 1.2e-10 off the unit cylinder, which is a true 3D miss, and the skew pair's query is off the surface by about the band. So the Decided premise `A ∩ B = A ∩ P_B` holds only for legs on the surface. Cost: the rounded bump at Interval is 12.1 s against 1.03 s, and the cone row 172 s against 67 s.
  Design fork dispatched: two designers (draw 145, 34; A and B), recorded on `analysis/design-fork/onecopy-landings-off-surface`. The lane is holding.

- 2026-11-06 — The off-surface landings fork converged in one round: A conceded after measuring that a co-surface pair escalates at Interval from about 1e3 m.
  - Ruled: the region landings read a plane-section crossing through `curve_face`'s plane door, not `meet`. The component is read through the chart channels, and the door is entered once through `ChartPoint`. Cost is main's. The decision is in the landings row.
  - Filed P2 `co-surface-carriers-have-no-home-of-their-own-in-curve-pair`: the `OnSurface` type, a curve_pair co-surface arm, and the `[ev]` C5 sentence.
  - B cited a `loop-walks-…` row for `apply_cut_ins`, but no such row exists; the cut-in item lives on the new row.
  Class lesson: "the same set" is not "the same reading". A meeting of two co-surface curves has a 3D margin that is identically zero, so the pose that is well-conditioned where it decides is the plane section's.

- 2026-11-06 — The user asked what is in progress and said to ignore usage warnings from now on, to spend the week's remaining usage. That overrides the earlier wind-down instruction. No warning had arrived, so nothing had been wound down. Dispatched four more cloud lanes alongside 6491:
  - `onecopy/curve-pair-tidy` (`session_013mPGJ2GyGL3Dk6qnqcnSBQ`): the coplanar level, the polish unit repeat, and the near-tangent tilt;
  - `onecopy/main-reds-k-and-m10` (`session_013mPGJ3twWm8X2P9Cc7QnjR`): the K probe sweep and m10_sym;
  - `onecopy/feet-and-params` (`session_013mPGJ4KpUZurFkBCMfZ8Uh`): a segment foot, and the nearest parameter by kind;
  - `onecopy/refusal-standing` (`session_013mPGJ5BFZnAe1VYCrzZYNj`): the two return types, and the kept-refusal loops.

- 2026-11-06 — Ruled for refusal-standing row 1: option C. Abort is split out of `Refusal`; `rung` takes no closure; a type that can abort returns `Result<Refusal, Aborts>`; `Kept` is the margin verdict only. The return type is the type-level gate. The row's suggested rule broke on `chart_singularity::Unread`, which has an Unsettled kind and no abort. Conditions: no outcome moves; check `git log -S` for ratified text describing `Refusal`.

- 2026-11-06 — Rulings to lanes:
  - **feet-and-params:** `nearest` is a certificate over `param_near`; `Line::param_within` and `foot_within`; `plane_offset` filed for want of a plane-value type.
  - **refusal-standing row 2:** file the four ranking-moving sites (census Germ, `link::judge`, the census vertex probe, and the early-`?` class) as P2 rows. The early-`?` row names a gate.
  - **curve-pair-tidy, near-tangent tilt:** B. The window also certifies a root with no found meeting, with unanchored arcs Off past the escalation. The row's stated mechanism was wrong: the guard skipped the window entirely, and the spun row's pivot was 5.5e-5 off the shared end. The row narrows to the 4 basin residues.

- 2026-11-06 — The curve-pair-tidy lane reported a main red at `230be11d`: `curve_pair::span_rows::a_point_on_a_circles_axis_is_off_with_no_place_read` (span_rows.rs:364, the antipode pin is off by π), at both ε. Handed to that lane to bisect and fix, since it is likely ONECOPY's (6537 or the foot changes). My own local repro ran out of disk; reclaimed.

- 2026-11-06 — Two PRs are up:
  - **PR 6539** (refusal-standing, `1a607018`): C implemented, with no design text found to describe it and no outcome moved. Five P2 rows filed.
  - **PR 6540** (feet-and-params, `a8836515`): segment feet onto `Line::param_within`, `foot_within` and `foot_from`, where a ray is `foot_from` because Interval ∞ is NaI; `nearest` is a certificate over `param_near`. No outcome moved, but bits move slightly.
  FULL reviews: 6539 to `session_013mPGJ3HVYqYAoquA4iThSZ`, 6540 to a new reviewer `session_013mPGJ5U1sALaLAPBzTAy6S`.

- 2026-11-06 — Dispatched two more lanes:
  - `onecopy/topo-handles` (`session_013mPGJ41WtMZGxxhdgCBrqz`): a face's loops, an edge's ends, and a face's surface, all onto topo's queries;
  - `onecopy/spline-walks` (`session_013mPGJ2b3x82WoPQQ3Ro3gw`): the Interval piece budget, and the misfiled fault.
  The geom small-reads bundle (`three-line-offsets…` and the others) waits for 6540, which touches `Line`.

- 2026-11-06 — Ruled for topo-handles: B. Export `Body::face(k) -> Option<FaceView>`, so the type is the gate: 13 outside sites spelled `once(f.outer).chain(f.rings)` with four different torn-loop handlings. Conditions: check `git log -S` for a ratified decision that FaceView is crate-private; list each site whose behaviour changes when it becomes the D2 row-4 panic; expose the minimum.

- 2026-11-06 — curve-pair-tidy opened PR 6541 at `9616e08d`. The coplanar level is onto `Level` and the subdivision takes charged terms; normalisation is onto `Vec3::normalize`. It filed two rows, among them `the-window-reads-the-level-uncharged-where-the-subdivision-needs-its-bound`, which bears on the across-key fork's reading. My B ruling and the span_rows red crossed its report, so I sent both back for the same PR. Review waits until they land.

- 2026-11-06 — PR 6539 FULL review: MERGE, with no outcome or ranking moved. Fix pass sent:
  - MINOR-1: `Evidence::keep` restated `Refusal::rung`'s table, kept in step by a test. It now gets one home through `rung`.
  - NOTE-3: a comment on pieced_arc's in-band exception.
  - The overlap row's class list corrected.
  Class lesson again: a test that exists to keep two spellings in step is the sign of a copy.
- 2026-11-06 — Ruled for spline-walks row 2: (a). New `SectionUnwalked` arms carry `geom::PieceWalkRefusal`, with a ratified-roster check first.

- 2026-11-06 — The 6491 takeover built the ruling at `91aeff79`, with the PR undrafted.
  - The plane route goes through a new `span_plane_meet` beside the lane path. The component is read through `ChartChannels`, and `QueryFoot` is the entry.
  - The ci set is 8605/8605 at both ε, with both reds green. Cost is main's within noise (the sphere two-leg row is faster).
  - Four flagged decisions: miss-first, the skip of an Undecided end past the leg's end, `End::open`, and visibility.
  FULL review sent to a new reviewer, `session_013mPGJ4JScywfx3zssA8yRc`, with an independent oracle for each door's counts and sides.
  The lane found an unfiled main red at 1e-12 (`parallel_cylinder_join::seams_lying_on_the_walls…`). Filed as P1 `main-red-seams-lying-on-the-walls-read-the-axes-at-1e-12` and given to the same lane as a separate PR.

- 2026-11-06 — PR 6539 delta review: MERGE. The table is spelled once (`Refusal::kept`), and no ranking moved. The gate is running on `e0f5b8e3`.

- 2026-11-06 — PR 6539 merged as `1d005523ab`, gated on `e0f5b8e3`. Main had moved only in `work/` files (ROUND's), so the gate stood.

- 2026-11-06 — Gave the refusal-standing lane (`session_013mPGJ5BFZnAe1VYCrzZYNj`) ladder tranche 4 on `onecopy/ladder-tranche-4`: TouchUnread's standing, the census vertex probe, the overlap witness search, the early-`?` ports with their register gate, and pieced_arc's in-band exception.

- 2026-11-06 — Tranche-4 rulings:
  - **TouchUnread:** `Unreadable` and `OffInterior` abort, `Unanalysed` is a limit, `Exhausted` is Unsettled; each standing is written beside its variant.
  - **OverlapRefusal:** as proposed, plus a principle: a payload read by several doors (`MeetFault`, `BooleanError`, `NormalAtError`) carries its standing on its own type, and wrappers delegate. `MeetFault` gets one, which also bears on census's misfiling. `Unread{what}`'s string cases get a P3 row to type them.

- 2026-11-06 — PR 6491 FULL review: FIX FIRST, mostly records.
  - **The code checks out:** the oracle found 0 wrong over about 3.7M paths (ruled, sphere, torus) and 12,800 torus landings, and cost is main's.
  - **Records:** the row on the PR branch lacked my 2026-11-06 Decided section and the co-surface row, which were written only on the state branch.
  - **Real code findings:**
    - M3/M4: the torus reads the tube side in m² for a meridian across a parallel's plane, so a parallel within about 1e-4 of the top circle always grazes;
    - M5: the ruled door checks on-axis after the foot;
    - S1: the span read is spelled twice in `plane_crossing.rs`;
    - S2: miss-first is a knob giving two orders in one door;
    - S5: QueryFoot-only is prose.
  Fix pass sent; it outranks the seams red.
  Lesson: a decision written on the state branch must be carried onto the PR branch's row by the lane's brief, or the reviewer reads a PR whose own row contradicts it.

- 2026-11-06 — PR 6541 updated to `48518f3f`.
  - B is implemented, with oracle pins. Cost: 12 needle poses with gaps of 1.0–1.9e-11 refuse under B.
  - The tilt residues are narrowed to 2 basin plus 2 extreme.
  - The span_rows red: the pin was wrong (`Azimuth::near` reads a zero direction as its reference), and it is probe-gated, never run in CI. `ray_wall_margin_twins` is the same class and filed.
  FULL review sent to `session_013mPGJ3HVYqYAoquA4iThSZ`.

- 2026-11-06 — Gave the curve-pair-tidy lane its next unit: the harmonic phases' cut side, and whether the chart's cut-side choice can leave the log. It goes on a new branch, or on its own branch after 6541 merges.

- 2026-11-06 — spline-walks opened PR 6548 at `134fa11c`, with hosted CI green.
  - Row 1: `clear_of` stops at its first near piece, so the hairpin escalates after about 30 readings instead of exhausting after 5,852.
  - Row 2: a walk limit is typed (`Unwalked` / `SectionUnwalked`, carrying `PieceWalkRefusal`).
  FULL review sent to `session_013mPGJ4JScywfx3zssA8yRc`.

- 2026-11-06 — Rulings:
  - **spline-walks, the subdivision walks:** (a), a caller-named shareable `Budget` with `bisect_within`, on condition that D9's own text permits it; the lane stops if D9 fixes one budget.
  - **main-reds-k-and-m10:** not A. Moving 13.5 s of m10 rows per-PR breaks the CI policy in the `.config/nextest.toml` header (≥ 0.5 s goes to the slow set). The ruling: re-pin with the movers named, try a fast sentinel under 0.5 s, and file the class (a whole-kernel ledger pin seen only by a nightly that has been red for a week) as a CIW row.
  The K root cause was geom-brep lacking its self dev-dep with `test-support`; it is fixed, with a new gate.

- 2026-11-06 — The seams lane opened PR 6550 at `a8479010`.
  - It bisected the red to `6adc5fdd`, where `resolution_pad` replaced the shared pad. The failing measurement stopped on `Stop::Wall` short of the resolution, and `judge` never read the stop.
  - The fix adds `judged_at_resolution`, with a new `Verdict::Unmet` that holds only when the enclosure holds the closed form. It also fixes a second unfiled 1e-12 red, `tang_circle_cylinder::a_tilted_rod…`.
  FULL review sent to a new reviewer, `session_013mPGJ2b7vVaMCyapnz3dn7`. The lane did 6550 before 6491's fix pass, so I re-sent the fix pass to run first now.

- 2026-11-06 — Ruled the spline foot search: polish local minima plus the 4 smallest samples. Misses fall from 3.6% to 0.085% at 2.1× per foot; the lane measures whole-suite cost against the fast-set bars before landing.
- 2026-11-06 — Sent the 6491 lane the text of the review's taste items (S3, S4, S8, N3), preferring S8's one type.

- 2026-11-06 — PR 6541 FULL review: FIX FIRST, on MAJOR-1. B is sound and every run is green, but the coplanar-ellipses change put an f64 `Rounded` charge on `Level::jet_at`, against ratified `[simple.conditioned]` and Ev's PR 6329 ruling. Ruled:
  - revert the coplanar part out of 6541 and reopen its row;
  - rewrite the filed row as "re-pose the subdivision's touch arm, or else `[ev]`", not "charge the window";
  - pin and disclose the zero → e move for unbounded arcs beside a ring.
  Lesson: a lane that meets a refusal it can fix with a charge must check `[simple.conditioned]` before adding one; a test that needs a non-zero error marks a pose to re-pose.

- 2026-11-06 — The harmonic-cut unit is done locally, waiting on 6541.
  - The harmonic phases are onto `Azimuth::near`, and the `harmonic-cut` gate class is retired.
  - One 1e-12 pose is re-posed, citing 2a8502bb.
  - The chart's cut side stays logged: one row (`placement_rows::a_direction_on_the_windows_cut…`) moves under `Band::least()`. So for a window read the choice is not a pure renaming, and the ledger now says so.
  - It found three more editor-core slow reds on main (m10_bulge ×2, sym11), passed to the main-reds lane.

- 2026-11-06 — PR 6540 FULL review: MERGE, with each port checked bit by bit and no outcome moved. Fix pass sent:
  - MINOR-1: a 3D segment-distance copy left in step-import `adopt.rs`, while the row claimed done.
  - MINOR-2: `Line::param` and `Line::offset` re-derive `project_onto` with `dot`, which is looser at Interval. Route `Line` through `norm_squared` / `reject_from` and port `cover.rs`'s `off_line`.
  - MINOR-3: an unstated `t0 ≤ t1` precondition, to be stated and made to fail loudly.

- 2026-11-06 — topo-handles opened PR 6552 at `e8aa9bdb`.
  - `Body::face` replaces `passed_face` as the public resolving handle.
  - `edge_vertices` is now public, and `query::edge_ends` reads both half-edges' starts.
  - About 680 face→surface walks are ported, with a new `topology-reads-through-topo.sh` gate.
  - The ci set passes at both ε, with 3 torn-link pins re-aimed.
  FULL review sent to `session_013mPGJ3HVYqYAoquA4iThSZ`, with the edge-ends semantics as its first check. The three unfiled main reds it found (the needle cavity, seat7 digests, and the loop-boundary-discards selftest) went to the main-reds lane.

- 2026-11-06 — Design fork dispatched on `the-census-sort-of-a-boolean-refusal-has-no-legal-home-under-topo-homes`: where a crate-internal reading of an engine's type lives under ratified `[topo.homes]`. Draw 127, 42: two designers, Opus as A. Recorded on `analysis/design-fork/onecopy-census-sort-home`. The outcome goes to Ev as `[ev]` if it changes `[topo.homes]` or the gates. The topo-handles lane takes `value-discards` (all 24 arms) and the loop doors.

- 2026-11-06 — The census-sort fork converged on first reports, and both designers rejected the framing.
  - The census's sort is in its home; the tangent-line reading it reads is arm-2 code misplaced in `boolean/`, a unit-5 move.
  - `[topo.homes]` is unchanged, so no `[ev]` PR. The row becomes a move row (cost H): `contain`'s placement door, then `face_boundary`, then the reading, with `FaceUnread` reshaped by verdict kind.
  - Filed `engine-leaves-passes-a-raised-pin-and-its-message-points-the-wrong-way` and `the-census-has-no-fault-arm-and-sorts-a-kernel-fault-as-unsupported`.
  The stale local `main` (`9036ae9e5b`) misled both designers' clones; I fixed the ref.
- 2026-11-06 — Dispatched `onecopy/engine-leaves-pins` (`session_013mPGJ2hhJzfCkPLVNf3azD`): the raised-pin gate and its message, and the census fault vocabulary. The tangent-line move waits until tranche 4 and topo-handles leave containment.

- 2026-11-06 — PR 6548 FULL review: MERGE. `Stop` is sound and no answer moved. Fix pass sent:
  - M1: pin the section's Unwalked rendering.
  - M2: put recourse on `PieceWalkRefusal`, so the join and section renderings agree.
  - S2 and S3.
  - A P3 class row for three clearance walks, two `SiteRefusal` sorts and the census's `WALK_*`.

- 2026-11-06 — The 6491 fix pass is at `ec152e74`.
  - The torus now reads every circle edge across the meridian leg's own plane, and `bool_torus_region_family` is retired. That departs from my Decided item 4, because a parallel's plane is tangent to the tube at the rims.
  - One `span_place`; the lane path is miss-first too; `Round` and `Line` are built only from a foot.
  - The records are copied.
  Delta review sent to `session_013mPGJ4JScywfx3zssA8yRc`, with the torus oracle re-run and the cost re-timed.

- 2026-11-06 — PR 6550 FULL review: FIX FIRST. The bisect and mechanism are right and the runs are green, but `Unmet` has no width ceiling: at 1e-12 a pad of 1e-3 would pass, which is weaker than before the regression. Fix pass sent:
  - a ceiling at one home;
  - the tang ∪ arm's hand-restated rule (a copy) read from the one home;
  - false claims corrected (all six ops stop at the wall, and tang was red before `6adc5fdd` too);
  - a row for the 1e-12 work wall.
  Lesson: a verdict that admits a weaker reading needs its own ceiling, or it becomes a pass-through.

- 2026-11-06 — The spline-walks next unit is done locally.
  - D9 reads "Every traversal is bounded: never a hang", which permits named budgets, so `Budget` and `bisect_within` go ahead. Five walks are ported, walk 7 included as accepted, and the window's literal copy of bisect is retired.
  - 8 more walks are filed.
  - The foot search at N=4 moves the spline suites 41.7 s → 42.1 s; one row goes 0.34 → 0.57 s, under the 1 s bar.
  It waits on 6548's fix pass and merge.
- 2026-11-06 — The 6541 fix pass is at `a898acea`: the coplanar change is fully reverted to main, NOTE-2 is pinned, and the ci set is 8606/8606 at both ε. Delta review sent to `session_013mPGJ3HVYqYAoquA4iThSZ`, ahead of its 6552 review.
- 2026-11-06 — The topo-handles next unit is done locally: the loop doors take resolved loops through `Body::loop_cycle_linked` and `proven_loop`; all 24 value-discard arms are disposed, with one fix (`cusp`) and one new row (`ray_crossings` graze); and escalation-discards counts value discards. It waits on 6552.
- 2026-11-06 — Ladder tranche 4 opened PR 6553 at `d212dfe8`: standings on seven types as ruled, six ports each pinned red under main's rule, the new gate `escalation-question-marks.sh` (32-site register), and the ci set 5724/5724 at both ε. FULL review sent to `session_013mPGJ5U1sALaLAPBzTAy6S`.

- 2026-11-06 — The 6540 fix pass is at `2fc04c9b`, and I did the delta review myself, since the diff is small.
  - `Line::param` now uses `norm_squared`, and `offset`, `square_foot` and `off_line` read through `project_onto`.
  - `offset` stays subtractive rather than using `reject_from`, because the triple product widened the skew pair's enclosure past zero at 1e-12. I accept that departure.
  - `adopt.rs` is ported, and the containment arm is order-free.
  I merged main (editor-core and topo moves) as `12b3a643`; it compiles, and the gate is running.
- 2026-11-06 — engine-leaves-pins opened PR 6554 at `25d97600`: the gate reds a raised pin against the merge base unless it cites an open row (lint now fetches the base), the message is rewritten, and a census fault panics through one door, with three more misfiled sites fixed. FULL review sent to `session_013mPGJ2b7vVaMCyapnz3dn7`.

- 2026-11-06 — PR 6491 delta review: MERGE.
  - The torus departure is right: a parallel crosses the meridian's plane square, so the reading is well-conditioned everywhere.
  - The oracle found 0 wrong and 0 refused over 19,200 torus landings, and the lane's S2 callers did not move.
  - Cost is at or below main (the sphere row 1.6 s against 2.6 s).
  - Note: `Round::through` and `Line::from` still take a raw centre, normal and dir, so on-surface is by convention past the start. That goes on the co-surface row.
  I merged main as `25636f41`; it compiles and the gates pass. The CI gate is running.

- 2026-11-06 — The 6540 reviewer withdrew its base column: base and head shared one `CARGO_TARGET_DIR`, so base reused head's binaries, since cargo fingerprints by relative path and mtime. Its claim that the 9 slow reds are main's is unverified until the true base re-run, so the 6540 merge is held until then even if the gate passes.
  Lesson: a base-vs-head comparison needs separate target dirs, and the curve-pair-tidy lane hit the same trap with worktrees. Every brief and review prompt should say so.

- 2026-11-06 — PR 6554 FULL review: FIX FIRST, one MINOR. The workflow is correct and fails open only on an unscoped dispatch, which is disclosed.
  - The hole: a citation to `log.md` or `plan.md` (a file with no `status:`) passes as an "open row".
  - Copies: the citation regex is in Python and in awk, and the census fault has four spellings.
  Fix pass sent.
- 2026-11-06 — PR 6541 delta review: MERGE. No f64 `Rounded` charge is left, the revert matches main byte for byte, and the NOTE-2 pin goes red under the old rule. I merged main as `0570fa5f`; it compiles, and the gate is running.
- 2026-11-06 — The 6540 gate is ok at `12b3a643`. It is held for the reviewer's true-base re-run.
- 2026-11-06 — The 6548 fix pass is at `1da87412`. I did the delta review myself, on a small diff: `PieceWalkRefusal::recourse` is the one home, M1 is pinned, and two P3 rows are filed. I merged main as `998a3234`; it compiles, and the gate is running.

- 2026-11-06 — Merged:
  - **PR 6491** as `0cd3e0444e`, gated on `25636f41`. The region landings read a plane section; the P1 landings row is closed.
  - **PR 6541** as `c0b7fb3d69`, gated on `0570fa5f`: the B window, the polish unit and the span_rows re-pin. The coplanar row is reopened.
  Main had moved after each gate, in disjoint files (ROUND: editor-core, geom-core, sweep and others). For each, the local merge with main compiled the whole workspace's tests clean before I merged.
  Rule used: re-gate if files overlap; with disjoint files, a workspace `cargo check --tests` of the merge suffices.
- 2026-11-06 — PR 6554 delta: FIX FIRST. The hatch still admits `program.md` (active), parked and deferred rows, because the fixture's program had no status. Four bare census faults remain. Second fix pass sent. Lesson: a self-test fixture that is not shaped like the real tree proves only the fixture.
- 2026-11-06 — Gave the 6491 lane (`session_013mPGJ4BHWTqt3QU2NKLLBa`) the co-surface home: an `OnSurface` type and a curve_pair co-surface arm read by the region doors and `apply_cut_ins`, with the C5 sentence as an `[ev]` PR, possibly split from the refactor.

- 2026-11-06 — PR 6540 merged as `1613027896`. The true-base re-run confirmed the 9 slow reds are main's. Only an audit doc overlapped main's moves; the merge compiled and every gate passed. Dispatched the geom-Line small-reads bundle (7 rows, including the plane-offset home) to the feet-and-params lane.
- 2026-11-06 — Ruled the plane-offset fork: not a new `geom::Plane {origin, normal}`, since the placement decision reserves `Plane { frame }` for stage 3 and leaves the seamless-plane type to it. (b) instead: one home for the offset formula, `Frame::height` where a frame is held, and the loose holders listed for stage 3. Lesson: check a row's sibling decisions before ruling a new type name.
- 2026-11-06 — PR 6554 delta: MERGE, with every hatch probe red. One last small pass sent: `STALE_KEY` re-spells `live::NAMES_ONLY_LIVE` (a minted copy), and the local `GATE_BASE` must be the merge base. I review that diff myself and merge.
- 2026-11-06 — PR 6550 merged after a MERGE delta. The ceiling has one home in `Budget::verdict`, and the 1e-3 probe reads BAD. Hosted gate green at `23e1cf28`; no file overlaps with main, and the merge compiled. The quadtail row carries the 1e-12 work wall.
- 2026-11-06 — The 6548 gate was ok at `f049121f`, but `on_carrier.rs` overlaps 6540, now on main. I merged main again (`2f68e629`, no conflict, the workspace tests compile), and the gate is re-running. Rule applied: an overlapping file means re-gate.

- 2026-11-06 — The 6554 final pass is at `94517cab`, and I reviewed the diff myself.
  - `STALE_KEY` is gone; the stale-key sites pass `live::NAMES_ONLY_LIVE`, so there is one premise and one text.
  - The local-base recipe is now documented.
  - I merged main (`4a15d487`). The tracker rows conflicted because the lane had carried the state branch's older copies; I took main's. It compiles, and the gates pass with the merge base. The CI gate is running.
- 2026-11-06 — curve-pair-tidy opened PR 6559 (harmonic-cut) at `7ac43ccc`. It took main's `circle_roots` whole after 6491; the ci set is 8614/8614 at both ε. FULL review sent to `session_013mPGJ4JScywfx3zssA8yRc`.
- 2026-11-06 — PR 6548 merged as `69847789f0`, gated on `2f68e629`. The spline lane is opening its walks/foot PR.
- 2026-11-06 — The PR 6554 gate is ok at `4a15d487`, but the merge was refused for conflicts, since main moved with 6548. Resolving.
- 2026-11-06 — PR 6554 merged as `57927d44d0` after I resolved two row conflicts with main (taking main's). The code delta since the gate was main's 6548, in disjoint files; the workspace tests compile and the gates pass with the merge base. Lesson: confirm a merge's result before telling the lane or logging it.
- 2026-11-06 — PR 6559 FULL review: MERGE. The re-pose was verified both ways, and the 6491 conflict resolution is right. I corrected M1's overstated gate claim in the PR body, merged main (`b886c310`, overlaps in `props/curved.rs` and the audit), and the gate is running.
- 2026-11-06 — PR 6559 merged as `d811c097c0` (confirmed), gated on `b886c310`, with no overlap with main since. The curve-pair-tidy lane now has the coplanar re-pose under `[simple.conditioned]`; it stops for `[ev]` if none exists.
- 2026-11-06 — feet-and-params unit 2 opened PR 6562 at `758e2875`: 7 rows closed, about 45 radial offsets onto `Line::offset`, `plane_offset` per ruling (b) with 38 ports, `BoxFrame` retired, and 12,747 ci tests passing at both ε. FULL review sent to `session_013mPGJ2b7vVaMCyapnz3dn7`, tight versus loose at Interval first.
- 2026-11-06 — The coplanar re-pose measured: the touch arm's reach `√(2·depth/c)` of a value that is truly zero is the ill-posed quantity (rounding 1e-16 becomes 1e-8 rad); main passes only through the noise merge. Ruled A, read the touch as one in-band stretch (L·depth in metres), on condition that ratified C5 already says it (else `[ev]`), with the premise read at the site, an oracle check, and the harmonic doors' moves measured and pinned.
- 2026-11-06 — PR 6553 FULL review: MERGE, with 5 MINORs sent as a fix pass. pieced_arc is `First`, not `Any` (its answers differ); overlap's local `BooleanError` table keeps kernel faults; two `NormalAtError` variants misranked as limits; doc classes; a census pin that misses the census wiring; register misfiling; and a row for the callee-decides blind spot.
- 2026-11-06 — Ruled on 6553 MINOR-1: pieced_arc takes A (the ends are the walk's premise; an in-band end blocks a later On, and either end may answer End). `place_off` is decided by whether consumers read the edge key.
- 2026-11-06 — PR 6562 FULL review: MERGE; the bit-dumps are identical, including a tilted-axis probe. Fix pass sent: merge main (dirty), measure `offset_axial`'s canonical-origin change on a tilted axis, one `/|a|²` standard across the rows, two missed cross-form sites, and `cover.rs` `off_line` onto `Line::offset`. Also, spline-walks unit 2 opened PR 6563 at `cef7e264`; FULL review sent to `session_013mPGJ4JScywfx3zssA8yRc`.
- 2026-11-06 — Coplanar re-pose: C5 is confirmed ratified (57d65871ec, Ev on 5214). One row moves: equal ellipses crossing twice within about 1e-3 now read one meeting. Asked the lane to check (1) that C5 makes an everywhere-on arc a shared run, not a touch at θ*, and (2) whether the old pin was Ev's, which would make it `[ev]`.
- 2026-11-06 — The 6562 pass is at `063c4243`: main merged; `axial_frame` measured on three tilted vessels with 0 bit diffs, so it stays; one `/|a|²` standard stated; `cover.rs` Line arm onto `Line::offset`. The gate is running.
- 2026-11-06 — main-reds-k-and-m10 opened PR 6560 at `8638a09d`: the K self dev-edge with a gate, the ray-wall and m10 re-pins with movers named, a slab sentinel, bisects, and a CIW class row. Its only hosted red is viewer wasm32, an apt hang. I can't re-run jobs (403), and won't push an empty commit, so the next main merge into the branch re-triggers CI. FULL review sent to `session_013mPGJ5U1sALaLAPBzTAy6S`.
- 2026-11-06 — PR 6563 FULL review: MERGE. The walks are identical, and an independent seed gives 0 misses. Short pass sent: a misattached F32 doc; 4096 spelled three ways (`PAIR_EXIT_PIECES` retyped u32); optional `Budget::implied_by_*`; and a note on Interval start counts.
- 2026-11-06 — The 6553 fix pass is at `47f329f5`: contact per A; `place_off` restores main's first-escalation rule (`contain.rs` reads which edge); BooleanError faults abort; NormalAtError re-ranked; doc classes; census wiring pinned; register fixed; blind-spot row filed. Delta review queued with `session_013mPGJ5U1sALaLAPBzTAy6S`.
- 2026-11-06 — curve-pair-tidy opened PR 6565 at `6d57970a`, the coplanar touch re-pose. The touch is decided by the stretch in metres, with no square root and no charge. The one moved row was agent-written, so it is re-pinned to C5 with an oracle. The kind (touch rather than run) is argued from C5 as geometric. FULL review sent to `session_013mPGJ4JScywfx3zssA8yRc`, with the kind as an explicit check.
- 2026-11-06 — PR 6562 merged as `5ff0801009` (confirmed), gated on `063c4243`, with no overlap. The feet lane takes the height-offset pair reading and the cross-product row (measure Interval widths first).
- 2026-11-06 — The co-surface unit is split. PR 6566 (refactor: `curve_pair::on_surface`'s `SurfaceCurve`/`OnSurface` arm; region doors and `apply_cut_ins` read it; `conic_off_plane` retired; germ row closed) goes to FULL review with `session_013mPGJ2b7vVaMCyapnz3dn7`. PR 6567 `[ev]` (the C5 and `[one-way.readings]` sentence) waits for Ev; I'm subscribed to its activity.
- 2026-11-06 — PR 6565 FULL review: MERGE. The stretch bound is sound, and a circle–ellipse oracle read 2,640 times with 0 wrong. The reviewer reads C5's kind as extent-based, so 'touch as geometric' re-reads ratified text, and that goes to Ev as a separate doc-only `[ev]` PR; nothing observes it, so 6565 still merges. Fix pass sent: the oracle's seam bracket (MINOR), the stretch doc, and the unequal family in the body.
- 2026-11-06 — PR 6560 FULL review: MERGE. Both bisect endpoints reproduce exactly. MINOR-1: the new CIW row duplicated two rows and re-opened Ev's 2026-10-16 ruling (dependents' slow sets) without citing it. Ruled: close it as a duplicate, count the instances on Ev's row per its revisit trigger, and leave revisiting to Ev. Fix pass sent; merging main re-triggers past the wasm32 infra hang.
- 2026-11-06 — PR 6563 merged as `448af0f8cb` (confirmed), green at `c1b2aed0` and up to date. The spline lane takes the eight more walks and the three clearance walks with their two sorts.
- 2026-11-06 — The 6565 fix pass is at `d3d21c27`: the oracle seam is fixed and pinned (red on the old bracket), the stretch doc covers the clamp, and the body lists both families. `pose.rs` overlaps main's 6563, so I merged main as `691eeda4`; it compiles, and the gate is running. Then the lane opens the kind `[ev]` PR.
- 2026-11-06 — feet unit 3 opened PR 6568 at `fb6ffe2a`: `Line::distance` (the cross form, tight at Interval), `param_and_offset`/`param_and_distance`, 60 ports, and a `line-distance-reads-distance` gate. Bit-dumps show 0 diffs; two sites are kept for D9 twin bit-identity and an undecided axis. FULL review sent to `session_013mPGJ4JScywfx3zssA8yRc`.
- 2026-11-06 — PR 6553 delta: MERGE, with all five pins red under the reviewed rule. One minted copy (`unread_or_rest` restating two rows of `TouchUnread::refusal`) is sent back as a one-commit fix before the merge.
- 2026-11-06 — PR 6565 merged as `874b2fde76` (confirmed), gated on `691eeda4`. The coplanar touch re-pose is done: no f64 charge, C5's stretch rule. The lane opens the kind `[ev]` PR.
- 2026-11-06 — The 6553 last copy is fixed at `ba564ca0`. It conflicted with main (6554, 6563): the code auto-merged; I took the PR's closed rows, kept both sides of the splitdoor row, used main's cited census pin and lowered it to 503 as measured. It compiles and the gates pass with the merge base. Pushed `d36d821b`; the gate is running.
- 2026-11-06 — `[ev]` PR 6570 opened (what kind a touch's meeting is, C5), with reading A recommended; I'm subscribed. The lane notes doc-gate.sh is red on main's rustdoc links; it is nightly-only and not ONECOPY's, so no action.
- 2026-11-06 — The curve-pair-tidy lane can push only the branch that holds `[ev]` 6570, so it holds that PR alone. The normalise row (about 268 sites) goes to a new session, `session_013mPGJ4BYPtAKKhmyYfHDMk`, on `onecopy/normalize`.
- 2026-11-06 — PR 6560 merged as `27bcf1be41` (confirmed), green at `217d7db1`; only `.config/nextest.toml` overlapped, and it merges cleanly. The duplicate CIW row is closed onto Ev's row as a count. The main-reds lane takes the corpus-dump reds and the K revalue.
- 2026-11-06 — The 6553 hosted gate is red on `boxes::every_door_that_reads_a_box_is_inventoried`: BOXES' open P0 main red since 11-03, spelled into our PR. The lane fixes it in 6553 (the three doors, each with its S234 direction) and closes or notes the BOXES row.
- 2026-11-06 — PR 6568 FULL review: FIX FIRST. Sound, and no outcome moved, but 'tighter at Interval' holds only when the point carries the width; with a computed axis of width, h and ρ come out up to 2× wider, and the gate makes the reading mandatory. Fix pass sent: measure the regime at the 60 real sites, propose one home that is right in both (stopping for a ruling if it's a fork), port the three spellings left outside `Line`, and check the zero-axis hazard.
- 2026-11-06 — The 6553 box-inventory fix is at `724c3720`: three doors added to PINNED, each with its S234 direction (grant, prune, lever), and the BOXES P0 row closed. No overlap with main since; the gate is running.
- 2026-11-06 — Ruled 6568's fork: (A). `Line::height`/`distance`/`height_and_distance` read dir as unit (no division), the tightest or tied in every regime measured; `param`/`offset` keep the division for the parameterization. Conditions: the unit contract fails loud where checkable (debug_assert) and is stated per site; (B), a typed unit axis, is filed as the root-cause row with the 6 undecided-axis sites.
- 2026-11-06 — The spline lane opened PR 6571 at `ab79351b`: 6 more walks onto `bisect_within` (2 stay apart, with reasons), the clearance walks' questions stated, one SiteRefusal conversion, and the census's walk sentence onto `PieceWalkRefusal::sentence` (one pinned text move). FULL review sent to `session_013mPGJ5U1sALaLAPBzTAy6S`.
- 2026-11-06 — Corpus-dump unit: (1) the key split is replaced by a derivation (`lane_pass_nodes`). (2) Ruled A: `Rounded::divisor` reads through the logged door, named per caller (`vertex_section_sine`, `ruling_clearance_across`), with audit rows. Log moves are measured and re-pinned; an outcome move stops the lane. Plus a gate or recorder check so an unnamed `sign_within` can't recur.
- 2026-11-06 — PR 6553 merged as `2a75076e2e`. Its hosted gate was red only on a fuzz seed of `ssi::certify::a_torus_carriers_limb_two_holds_its_sampled_residual` at 1e-6, which I reproduced on main with the same seed, so it is main's red, not 6553's. Filed it as P1 `main-red-a-torus-carriers-limb-two-residual-exceeds-its-bound-by-rounding`. The BOXES P0 also closed with 6553.
- 2026-11-06 — The torus limb-two P1 is fixed in PR 6573 at `522679d0`. It was not a soundness defect: an exact residual read with Fraction/Decimal is under the bound. The test's slack missed the f64 sample's own rounding at the point's scale, so +8ε|p| is added, with the reason at the site. Latent since before 11-04; seed pinned; 80k-case sweep clean. I reviewed it myself (test-only, with an exact proof); the gate is running.
- 2026-11-06 — 6568 re-ruled to (A′). The unit-read contract fired: at-rest stored frames are unit only to about 4e-10 in |dir|², and `Curve3::Line` carriers are minted unnormalized, so a unit read is wrong by about 2e-10·|w|, roughly 200× the band. So `distance = |w×dir|/|dir|` and `height = w·dir/|dir|` are correct for any dir, with no contract. The typed unit axis row takes this as evidence. Lesson: a fail-loud contract on an assumed convention measured the convention; add it before relying on one.
- 2026-11-06 — PR 6571 FULL review: MERGE. The walks are exact. Fix pass sent: merge main, where 6553's `EdgeTangentUnread::refusal` conflicts, keeping both standings via a `PieceWalkRefusal` standing; make `of_sentence` exhaustive; pin the two moved sentences; state the reason on the pieces-read walls.
