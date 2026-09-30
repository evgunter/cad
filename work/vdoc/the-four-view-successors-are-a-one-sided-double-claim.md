---
id: the-four-view-successors-are-a-one-sided-double-claim
kind: issue
title: CHROME's keep_out names VIEW and not the four successor programs that now hold VIEW's rows
status: open
opened: 2026-09-17
priority: P4
cost: E
---


Filed by VIEW's re-scope of 2026-09-17, onto CHROME's slate rather than
`work/issues/`, per `work/README.md`: *a finding now goes straight onto
the slate of the program whose ground it lands on*. One-file-one-item
means VIEW could not write this clause into `work/chrome/program.md`
itself.

## The finding

VIEW's slate was re-cut on 2026-09-17 into four successor programs, each
claiming a subset of `crates/viewer`:

| program | paths it claims | shares with CHROME |
|---|---|---|
| `vnews` | `frame.rs`, `display.rs`, `pane.rs`, `pane/create.rs`, `pane/features.rs`, `pane/properties.rs`, `pane/viewport.rs`, `seats.rs`, `tools.rs`, `session/refuse.rs` | 10 tracked paths |
| `vgeom` | `camera.rs`, `datums.rs`, `marks.rs`, `scene.rs`, `bounds.rs`, `props.rs`, `widgets.rs`, `readout.rs`, `input.rs`, `sketch.rs`, `gpu.rs`, `idpass.rs`, `pickindex.rs`, `pane/view.rs`, `pane/viewport.rs`, `pane/properties.rs` | 16 tracked paths |
| `vseam` | `app.rs`, `session.rs`, `session/*`, `evalseam.rs`, `pickcache.rs`, `pickindex.rs`, `generation.rs`, `history.rs`, `g1.rs`, `docio.rs`, `forms.rs`, `vocab.rs`, `combine.rs`, `tools.rs`, `pane/create.rs`, `pane/viewport.rs` | 21 tracked paths |
| `vdoc` | `crates/viewer/README.md`, `crates/viewer/tests/*`, `crates/viewer/src/lib.rs` | 63 tracked paths |

CHROME's `paths` are `crates/viewer/src/*`, `crates/viewer/tests/*` and
`crates/viewer/README.md`, so it shares ground with all four. **Each of
the four `keep_out`s names `chrome` and carries CHROME's 2026-09-15
carve-out in the direction that binds it; CHROME's `keep_out` names
`view` and cannot name them**, because VIEW may not edit CHROME's
`program.md`. `work.py lint` therefore prints four new one-sided
warnings, and the overlap is invisible from CHROME's side — which is
exactly the condition `work/README.md` says is the live one:

> An overlap written on both sides is a handoff a lane can announce; an
> overlap written on one side or neither is a live conflict, and the
> program that was there first is the one that cannot see it.

## What the row asks for

One clause in `work/chrome/program.md`'s `keep_out` naming `vnews`,
`vgeom`, `vseam` and `vdoc` and restating the carve-out against them.
**The carve-out itself does not change** — the file division of
2026-09-15 stands verbatim; what changes is which program is on the
other side of each line:

- CHROME works `datums.rs` and `bounds.rs`. `datums.rs` is now in
  **`vgeom`**'s paths (three VGEOM rows sit on it) and `bounds.rs` is
  too. This is the one place the carve-out has to be re-argued rather
  than re-addressed, and CHROME is the program that gets to argue it:
  `chrome/datums-substitution-sweep` is dispatched on that file and
  VGEOM's own item 1 is the same fail-loud class at four other sites.
- CHROME cedes `scene.rs`, `gpu.rs`, `marks.rs`, `blend.rs`,
  `props.rs`, `sketch.rs` → **`vgeom`**; `theme.rs` and
  `pane/features.rs` → **`vnews`** (the tone row went there);
  `app.rs`, `session.rs`, `session/*` → **`vseam`**; `pane/*`,
  `frame.rs`, `display.rs` → split between **`vnews`** and **`vseam`**
  as those two `keep_out`s record; `pickindex.rs` → **`vgeom`** and
  **`vseam`** both, by subject.
- CHROME works `tests/valid_range.rs`, `tests/combine_ops.rs` and
  `tests/tree_badges.rs`; the rest of `crates/viewer/tests/*` is
  **`vdoc`**'s, and is still S-TCOST's and S-TINT's as well.

`the-gui-shows-no-measure-value-and-no-clearance` arrived on this slate
in the same commit, re-homed from VIEW: it is a chrome-coverage row on
`tree.rs`, which the carve-out never ceded.

The `*/tests/*` half of this is the standing case already filed as
`work/meta/double-claim-lint-rule-waits-on-the-tests-seam.md` and is not
re-filed here.
