---
id: replace-face-reads-an-approx-iso-as-u-fixed
kind: issue
title: replace_face's Approx iso arm assumes every IsoLine description fixes u, but EdgeDescriptionSpec::chart_image mints u-moving ones
status: open
opened: 2026-10-06
priority: P3
cost: E
refs: [no-approx-faced-body-is-both-movable-and-valid]
---


`replace_face.rs`'s `Surface::Approx` arm in the offset
re-description path reads a chart `IsoLine` description as `u` fixed
(`u = p0.x`) and moves the image along `v` only, on a comment that
`EdgeDescriptionSpec::iso` is the only door minting one. That comment
is false: `EdgeDescriptionSpec::chart_image` (geom-brep
`description.rs`) also mints `IsoLine` descriptions, and the fixture
`box_with_approx_cap` uses it for two u-moving images. A u-moving
description reaching the arm would degenerate — probably loudly
downstream, but unmeasured.

Fix: decide the description's moving axis rather than assume it (or
refuse typed on a u-moving one), correct the comment, and pin a row on
a u-moving image. Found by TESS's survey (2026-09-22, recorded on the
now-closed `no-approx-faced-body-is-both-movable-and-valid`), filed
on its own by SHELL's 2026-10-06 triage. Signed (SHELL orchestrator).
