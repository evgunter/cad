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

## First wave: four units on three PRs (2026-09-04)

`probe-bounds-lacks-driven-slot-guard` (PR 1746),
`viewer-mate-tool-refuses-pattern-picks` (PR 1748),
`viewer-render-pipeline-creation-untested` and
`viewer-chrome-not-in-nextest-archive` (PR 1755, one branch — they
interlock: one is a row, the other is where a row can run).

**The archive measurement, which `plan.md` names as the slate's only
decision.** Verdict: a row beside the app-feature clippy step under the
same `run_viewer_toolkit` axis, NOT `--features app` in the archive.
The archive is downloaded by every leg of the `test` matrix, so its
+179 MB is paid per leg and buys nothing for rows that already gate.
Numbers, both readings and their blind spots, are in PR 1755's body.

**Three corrections the lanes made to the orchestrator, all upheld.**
Recorded because the pattern is the point: every one of them was a
premise the orchestrator asserted and a lane checked.

- The dependency-graph delta was published as 71 → 265 crates. It is
  **65 → 211**. The orchestrator's `cargo tree | sort -u` did not
  strip `(*)` duplicate-subtree markers, so a crate appearing in two
  subtrees counted twice. Verdict unchanged; the number was wrong.
- The argument that the app-feature test row is nearly free in `fmt`
  ("that job already compiles this graph") rests on a false
  sub-premise: `cargo clippy` is CHECK semantics and leaves metadata,
  not rlibs. Consequence in
  `work/chrome/fmt-cache-carries-the-toolkit-codegen`, filed rather
  than decided — the deciding measurement does not exist.
- `viewer-chrome-not-in-nextest-archive` named ONE silent skip and
  there were three; and the cause the orchestrator suspected
  (`tests/all.rs`, `autotests = false`) was checked and cleared. The
  invisibility was purely the `#![cfg(feature = "app")]` inner
  attribute.

**What the style lane bought, stated plainly** because the posture was
chosen over the A/B protocol and should be judged. Twenty-two
findings. The three that justify the lane on their own were not
correctness findings at all: a PR body claiming a finding was
"recorded against that item" when the item was byte-identical to main
(the record existed on a branch that does not ride that PR); an item
whose title and cited cost were both false, so it scheduled finished
work while understating what was left; and `refactor.rs:1224`
restating the same vocabulary PR 1748 had just given one home, in the
crate that PR edited, self-declared in twenty lines of comment nobody
reads. None is reachable by asking whether a claim holds.

**The fence CHROME had to amend.** `keep_out` said *"editor-core
mate.rs and assembly.rs vocabulary is read and not edited"*, and PR
1748 edits `mate.rs`. The first disclosure named only "outside
`paths`" — the weaker boundary — which the style lane caught. The
clause is amended with its argument in `program.md`: the viewer's gate
was a RESTATEMENT of the rule `mate.rs` owns, so honouring the fence
would have preserved the defect it was written to keep out. This is
the session's one decision an orchestrator should not be able to take
alone, and it is recorded as such rather than buried in a diff.

**Items filed this wave**, all from lane or review findings that would
otherwise have existed only in a session's context:
`mate-member-vocabulary-restated-in-refactor` (issues/),
`session-gesture-guard-spelled-thirteen-times`,
`probe-rows-assert-in-one-direction-only`,
`fmt-cache-carries-the-toolkit-codegen`, plus four live residues
re-cut onto `doc-params-carry-no-display-unit`, whose scope shrank:
the persisted field the plan schedules an announcement for had already
landed in another program's PR.

**Inherited red, on PR 1748 only.** `pncad-py
tests::the_whole_tag_table_matches_its_committed_inventory` is red on
`main` itself (`work/lib/pncad-py-tag-inventory-misses-two-measure-tags`,
which asks for an instance rather than a repair — appended, not
repaired). PR 1746 is GREEN on the same base, so that item's "billed
to every code-tier PR" is too strong: it is billed to every PR whose
closure REACHES `pncad-py`. 1748 does only because it re-exports
through `crates/pncad`.
