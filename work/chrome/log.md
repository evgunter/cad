# CHROME log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/chrome/plan.md`. A/B band 1600–1699
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose CHROME section is the
charter this plan restates. Opens now. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `viewer-render-pipeline-creation-untested` from `work/issues/`
- `viewer-chrome-not-in-nextest-archive` from `work/issues/`
- `placed-union-has-no-session-op` from `work/issues/`
- `probe-bounds-lacks-driven-slot-guard` from `work/issues/`
- `pickindex-per-part-window-twins` from `work/issues/`
- `viewer-mate-tool-refuses-pattern-picks` from `work/issues/`
- `refused-mate-badges-every-instance-row` from `work/issues/`
- `doc-params-carry-no-display-unit` from `work/issues/`
- `viewer-first-light-on-real-hardware` from `work/issues/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## The slate opens (2026-09-04)

Orchestrator seated. Review posture is the plan's: batched style
review, no A/B row. Two units additionally carry a correctness lane —
`pickindex-per-part-window-twins` (a pure refactor whose failure mode
is #1098's silently-wrong name, so the lane's whole job is a
differential against the pre-refactor index) and
`placed-union-has-no-session-op` (a new op entering the replay/undo
vocabulary, so the lane's job is the round-trip). Nothing in the slate
is kernel ground and nothing takes an adversarial lane.

Three premises in `plan.md` re-checked against the tree before any
dispatch, per the dispatcher's-exposure rule:

- **Unit 1's precondition is met.** The `EdgePass` depth-bias fix is
  in-tree: the bias is applied in `vs_edge` as a relative clip-z
  shrink (`crates/viewer/src/gpu.rs:339`) and no pass requests a
  `DepthBiasState` on a `LineList`. The unit is the smoke row, not the
  fix.
- **Unit 8 is half-done, and its contended half is the half that
  landed.** `DocParam::Continuous` already carries `display_unit`
  beside `dim`, with `written_length`/`written_angle` as total
  authoring doors and the pairing checked by `persist::check`
  (`crates/editor-core/src/doc.rs`). So there is no new persisted
  field, and the GQ3-versioning announcement the plan schedules is
  owed on nothing. What remains is the PANEL half — which
  `crates/viewer/src/props.rs:34` states as the residue in its own
  module docs — plus one `DocEdit` door in `editor-core`, because
  `SetDocParam` is create-or-replace and a unit-only edit through it
  would silently delete a parameter's `Distribution`. That door is
  outside this program's `paths`; it is announced here rather than
  taken silently.
- **Unit 9's hardening PR merged.** `run()` builds an explicit
  `ViewportBuilder` (`crates/viewer/src/app.rs:5384`) rather than the
  bare `NativeOptions::default()` the 2026-08-28 comment names. The
  item's Ev-only residue is §2 (the culling flip, both pipelines) and
  §4's failure 2 and 3 (`R32Uint` clear semantics, readback cost on a
  real driver).
