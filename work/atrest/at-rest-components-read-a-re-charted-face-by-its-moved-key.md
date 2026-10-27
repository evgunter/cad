---
id: at-rest-components-read-a-re-charted-face-by-its-moved-key
kind: issue
title: The at-rest quieting rule reads a re-charted sphere face's descent by its key, which names an input face at another place or none
status: open
opened: 2026-10-27
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
