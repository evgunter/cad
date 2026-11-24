---
id: at-rest-components-read-a-re-charted-face-by-its-moved-key
kind: issue
title: The at-rest quieting rule reads a re-charted sphere face's descent by its key, which names an input face at another place or none
status: closed
opened: 2026-10-27
closed: 2026-11-23
---


Filed by WIRE's `wire-rechart` lane (branch `wire/re-chart-naming-record`).

## What

`crates/editor-core/src/checks/at_rest.rs` `components` reads each
result face's descent (`topo::FaceDescent::of(&result.naming)`,
`result_face`) and maps the root to the aggregate copy's face by key
(`product.copies[copy].keys.face(root)`).

When the pair boolean re-charted a closed sphere shell
(`topo::boolean::ops::apply_recuts`), that root is a key of the re-cut
operand, not of the input:
- the first turned shell keeps its input keys at turned places, so the
  root names an input lune the piece does not lie in;
- a grafted shell's keys are new, so `keys.face(root)` answers `None`
  and the whole pair's components read as `None`.

`BooleanNaming::recharted` now holds the map from each re-charted cell
to its input shell's faces. A piece of a turned sphere descends from
that whole set of faces, not from the one face its key names.

The same file's quieting step reads `topo::FaceDescent::of(&kept.naming)`
the same way (`result.body.solid_of_face(root)`), over the kept part of a
second intersect, which can re-chart too.

Unmeasured: whether any at-rest document pairs a re-charted ball with
an assertion. The fix reads the root through `naming.recharted` and
takes every face of the input shell as the piece's sources.

## 2026-11-23 — the input shell, read and then declined

HELDRECORDS' `nary-lineage` PR (7516) replaced `FaceDescent::result_face`
with `FaceDescent::source`. It returns a `topo::FaceSource`: the operand,
the root in the keys the op ran on, and `inputs`, every input face a
turned face stands for. The old door is gone.

- `components` first entered every input face as a source. Its review
  found that this widened two checks: `covered` would count an undecided
  verdict on a face the component holds no area of, and the quieting rule
  could match an assertion to a component it does not bound.
- So `components` now names a piece only through a single input face. A
  turned face of a shell of several faces leaves the pair's components
  `None` (`Unlocalized::Unnamed`), the fail-loud reading. This is stricter
  than before: a turned face that kept its key no longer names one lune
  it may not lie in. The row's fix is therefore still owed: which faces
  of the shell the turned piece holds (`topo::overlap`, as the holdings
  read it).
- The quieting step's `crossing` names a component by any input face's
  solid. Every face of a shell lies in one solid, so this is the old
  reading made right, not a widening.

Still unmeasured on an at-rest document; no fixture turns a shell.

## Closed

Closed 2026-11-23 by ORBIT's PR 7602 (`orbit/one-sphere-escape-cut`), which retires the boolean's rotation re-chart: every sphere escape is cut in along a meridian of the ball's own chart, so no shell is turned or grafted and no key lands at a turned place. `BooleanNaming::recharted` and `FaceSource::inputs` are gone with it. A cut piece reads back to the face it divides through the division rows (`FaceSource::root`). `components` and the quieting step in `crates/editor-core/src/checks/at_rest.rs` now read each piece through that one root face, which is a face of its input. The `Unnamed` reading for a turned face had no other case and is removed.
