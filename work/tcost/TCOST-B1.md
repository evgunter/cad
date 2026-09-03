---
id: TCOST-B1
kind: unit
title: declare editor-core's shared test helpers once, not once per suite
status: dispatched
pr: 1616
branch: tcost/b1-dedup-suite-helpers
opened: 2026-09-03
---

First build-side unit, cut from the build profile: editor-core's shared test
helpers are declared once instead of once per suite file, cutting test-code
volume and its generic instantiations in the test-target compile. Build-side
levers are in scope under the same review split (Ev, 2026-09-02); no CI
build-knob change rides it.

Not yet in `work/tcost/log.md` (the log holds only the opening entry at
migration); PR 1616 is the record.
