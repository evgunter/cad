# FIT — the log

## 2026-09-20 — opened

Cut out of VGEOM, which was carrying 43 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed VGEOM's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

6 rows arrived by `git mv` with their ids, bodies and history
unchanged. VGEOM keeps its band 5300-5399; band 8500-8599 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## A note from CHROME (2026-09-22) — 2 rows re-homed here

CHROME measured 88 budget points against its 30-point ceiling and was
cut along its priority seam (`work/chrome/log.md`, same date), opening
FORMS and OFFER. Per `work/README.md`'s re-homing rule, rows whose
charter fit a live program went to it instead of into a new directory.
Moved here by `git mv`, ids, bodies and history unchanged:

- `a-flat-rung-row-uses-an-absolute-epsilon-on-a-scaled-value`
- `mispaired-ids-exempts-the-empty-window`

The first is a display-fit rung row, beside `a-flat-rung-pair-is-read-as-flat-below-it`. The second is the id/patch pairing guard on the pick seam: VGEOM declined it on its charter test (`work/vgeom/log.md`, 2026-09-21 — *what fails to arrive is an identity, not a figure*), which is this program's pick-index ground and sits beside `id-readback-failure-reads-as-nothing-under-the-cursor`.

If a row does not belong here, say so on this log and CHROME's
successor will take it back.

Signed (CHROME orchestrator).

## 2026-09-27 — seam note from S-DUP (#3304)

#3304 rewrote the rustdoc of `crates/viewer/src/camera.rs`'s `CameraError::UnusableBounds` arm. It dropped a sentence that justified keeping the arm unsplit by "a promoted review suite that pins this arm", which is the reading Ev withdrew.

- **The arm doc.** It is now a per-door list of what a caller can infer from the arm. It covers `projection_matrix`, `ray_through`/`datum_view`, `fitted`/`apply(Frame)` and `framing`, and says which input each one checks first.
- **The first line and `Display`.** They no longer say "positive finite": a non-finite input is `NotFinite` first.
- **Unchanged.** The API is the same, and the arm stays unsplit. Whether to split it is still this ground's call.

Signed (S-DUP orchestrator).
