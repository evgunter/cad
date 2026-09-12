# CHROME — viewer chrome and coverage (plan)

**STATUS: OPEN (2026-09-03).** Opened 2026-09-03 from `docs/WORK-TRACKS-2026-09.md` (CHROME section), which is this
program's charter until this plan supersedes it. Live state is
`work/chrome/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`chrome/`** — unit branches
`chrome/<unit>-<slug>`, orchestrator branch `chrome/orchestrator`.
Away-channel tag `(CHROME orchestrator)`. A/B ordinal band
**CHROME = 1600–1699**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in the opening commit, per that entry's rule.

## Charter

`crates/viewer`'s chrome defects and the two coverage rows that would
have caught a shipped startup panic. All E: the fix is written in each
item, and the only decision in the slate is a measurement (archive
with `--features app`, or a small separate job).

## Review posture

Batched style review, no A/B row, one PR per item. The slate lands
before VIEW's `viewer-session-god-module-split` conversation ratifies;
anything still open when it does rides the split.

## Unit order

1. `viewer-render-pipeline-creation-untested` — a lavapipe smoke row
   that creates a wgpu device and constructs every `viewer::gpu` pass
   (no surface, no pixels). Check first whether the `EdgePass`
   depth-bias fix on `claude/subagent-gui-integration-tests-i153yl`
   merged.
2. `viewer-chrome-not-in-nextest-archive` — measure the archive cost
   of `--features app` against a small `cargo test -p viewer
   --features app` job; make the silent feature-skip loud either way.
   S-TCOST's rule on CI build knobs applies: the measurement is in
   the PR.
3. `placed-union-has-no-session-op` — `SessionOp::AddPlacedUnion` and
   its combine row; retarget `story_authoring`'s dead-end chapter.
4. `probe-bounds-lacks-driven-slot-guard` — the `DrivenByExpression`
   refusal its sibling doors have; seed from the slot's current value.
5. `pickindex-per-part-window-twins` — one per-part window index
   generic in entity kind; before any vertex-pick unit.
6. `viewer-mate-tool-refuses-pattern-picks` — widen the `is_instance`
   gate to the A11 member vocabulary.
7. `refused-mate-badges-every-instance-row` — the refusing mate is the
   loudest badge; instance rows read "upstream mate refused".
8. `doc-params-carry-no-display-unit` — **re-cut on the item; this
   row was written from a stale premise.** The display unit beside
   `DocParam` had already landed in another program's PR, so there is
   no new persisted field and the GQ3 versioning announcement this row
   schedules is owed on nothing. What is left is the PANEL half, plus
   one `DocEdit` door in `editor-core` outside this program's `paths`.
   Read the item, not this row.
9. `viewer-first-light-on-real-hardware` — an Ev-run checklist
   (culling flip in both pipelines, `R32Uint` clear semantics,
   readback cost), not a lane unit; the hardening PR it names is
   checked for merge state first.

The viewer half of `error-types-with-no-display-class` lands from FIX,
whose item it is.

## Exit shape

The nine land; the walk convention applies. What VIEW's split leaves
of this program's files is VIEW's.
