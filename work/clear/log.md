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
