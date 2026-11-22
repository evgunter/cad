# ONEROUND — the plan

An orchestrator holds the track on `oneround/orchestrator`. Live state is `work/oneround/log.md`'s tail and the item files beside this plan.

Branch prefix **`oneround/`**; away-channel tag `(ONEROUND orchestrator)`.

## Charter

Every f64 rounding charge, lever arm and consumed extent reads its one ratified home (`crates/geom-core/README.md`, "Float rounding"), and the gates that ratchet them shrink to their oracles.

## Posture

State on 2026-11-21, 04:55Z. Each lane is its own cloud session; the session ids, heads and what each is owed are in the log's entries from 02:45Z on.

- **Landed:** `oneround/rounded-room` (PR 7145), `oneround/section-area` (PR 7146), `oneround/lever-doors` (PR 7143), and three fixes to main reds they caused or uncovered (PRs 7223, 7238, 7246). The other reds on main are routed to their programs in the log.
- **In review:**
  - `oneround/rounded-pairs` (PR 7221; M, single FULL): reviewed, in its fix pass, then a delta read.
  - `oneround/lever-sites-brep` (PR 7219; M), `oneround/lever-sites-topo` (PR 7228; H) and `oneround/unsourced-bits` (PR 7227; M): each in a single FULL review.
  - `oneround/box-region` (PR 7225; H, a concurrent pair): one report in, one out. Its PR carries the report back Ev is owed (PR 7140) before it merges: every face has a cover, the cost, the moved verdicts, and a limit it found on valid spline-faced bodies past about 707 m.
  - `oneround/anchored-conic` (PR 7203; H, a concurrent pair), as its step A: the two births private to `conic.rs`, the phase stored, the plane-cone section anchored. Stacked on `onehome/carrier-frames` (PR 7131), which trails main in 12 files, so it has no hosted CI and cannot merge until that base is level.
- **Building:**
  - `oneround/box-certify-ask` (M, single FULL), stacked on `box-region`.
  - `oneround/turn-lever` (H, a concurrent pair), stacked on `lever-sites-topo`: the turn-lever doors read both bounds and each of about 41 callers takes the one its action needs.
  - `oneround/unsourced-stand-ins` (H, single FULL) and `oneround/unsourced-unions` (M, single FULL), stacked on `unsourced-bits`: families 2 and 4, and the rest of family 1 with `profile`'s six.
- **A design fork, reconciled and being measured before it goes to Ev:** what a conic stores. PR 7194 (one stored form, anchored, the centre derived; Ev approved it) was built and measured, and does not hold as written: the centre's readers and the `Interval` lane lose by it. Three designers now agree on one stored point with its placement on the conic, the centre allowed only where no semi-axis exceeds the session box. One choice is open (inside that limit, the mint's own point or always the centre), and one measurement by the conic lane decides it. The `[ev]` PR follows that measurement.
- **Decided, still to be cut:**
  - `sizes-are-read-back-out-of-extent-levers` (P1, M): after PRs 7219 and 7228 merge.
  - The records unit (Ev's condition, PR 7141): family 3, every arm held in a record, the pinned row's points-only sites and wrappers, about 34 lever fields; then the bare arms of `Margin::over_lever` and the deletion of the `unsourced` doors.
  - The general anchor (about 60 sites; the cylinder's shallow cuts) and the `Reach` rule for an anchored conic: after the conic fork is answered.
- **The far-centred conic's other decided margins** (P3, from the phase-1 probe): read each against the stored-point form when it lands; several close with it.
- **Tail:** `a-conics-points-are-restated-in-centre-form-outside-its-evaluator` (P2), `site-rounding-charges-owed-a-re-posing` (P2), `the-interval-revalidation-escalation-row-is-reached-only-past-the-box-at-1e-9` (P2), `extent-reads-made-outside-the-extent-home` (P2), `consumed-extent-wraps-extent-ball` (P3), `a-quadric-surface-is-stored-by-a-datum-that-may-lie-far-from-its-face` (P3, `design`; a probe first, after the conic fork).
