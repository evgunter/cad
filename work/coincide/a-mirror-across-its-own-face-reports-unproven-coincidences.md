---
id: a-mirror-across-its-own-face-reports-unproven-coincidences
kind: issue
title: mirroring a body across its own face reports its self-image walls as unproven coincidences
status: open
opened: 2026-10-31
priority: P1
design: true
---

## Finding

Found driving the viewer (2026-10-31; Ev: P1, "the design will need to be careful here"). Mirroring the startup plate across its own front face with the mirror tool (one face pick: the body mirrors across it, fused with its image) gives two `unproven-coincidence` findings on the Union, one per side wall that the mirror plane cuts square:

> check unproven-coincidence: Union "Union 1" (…): the side wall over loop 0 step 2 of Extrude a59299af4a92 continues on one carrier with the side wall over loop 0 step 2 of Extrude a59299af4a92, decided from values (a pair of planes its margins read as one, margin 0e0), is not proven structural: the two cells are one construction placed or reflected apart; with every variable a symbol, neither the carriers' verdict nor the deciding margin is shown identically zero within the door's budget — Recourse: assert the coincidence at its site; …

A wall perpendicular to the mirror plane is its own image under the reflection, so its coincidence with its image is structural. It holds for every value of every variable, but the door's symbolic read outgrows its budget. Each finding also names both of its cells with the same words, since one is the other's image.

The design question: whether the door should recognise a cell and its image under the reflection it was mirrored by, and on what evidence. A wall that is not square to the plane is not its own image, and recognising those by values would be the unproven case again. Related: `work/coincide/a-check-findings-text-restates-every-path.md` (the length), `work/chrome/an-unproven-coincidence-is-hard-to-resolve-by-a-variable.md`.

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt`.

```text
window 1280 1600
click "Combine bodies"
click "Mirror tool…"
vclick 0.3 0.62
settle
click "Commit mirror"
settle
click "checks: 2 finding(s)"
settle
dump
```
