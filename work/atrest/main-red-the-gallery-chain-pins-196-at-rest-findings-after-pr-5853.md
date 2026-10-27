---
id: main-red-the-gallery-chain-pins-196-at-rest-findings-after-pr-5853
kind: issue
title: Main red (suspected from PR 5853): the gallery's chain document reads 10 at-rest findings, overlaps among them, where gallery.rs pins 196 could-not-look; PR 5853's CI skipped the demos job
status: open
opened: 2026-10-27
priority: P0
cost: E
---


Filed by the SPLICE lane on PR 5868, whose demos job went red on a test
its diff does not reach (it touches the spline seam reading only; the
chain is extrudes and cylinder pins).

## What

`demos/tour` `gallery::tests::each_gallery_document_denotes_its_scene_or_says_why_not`,
CI run 40577287351 (PR 5868 at `9156ff67` merged with main at or after
`3bb4dc553`):

```
panicked at src/gallery.rs:379:13:
assertion `left == right` failed: chain: at-rest findings (... ONE HUNDRED
NINETY-SIX at-rest findings, all could-not-look and none interference ...)
— checks: 19 finding(s)
  check unproven-coincidence: ... (9 rows)
  check at-rest: Assemble ...: Place ... and Place ... overlap, bounded by
  the end cap of Extrude ..., the start cap ..., the side wall over loop 0
  step 4 ..., the side wall over piece 0 of loop 0 step 0 of Extrude ...
  (9 rows, and one the kernel could not bound)
  left: 10
 right: 196
```

The same test passed on PR 5868 at `06e486b9` (CI run 40563245558),
merged with main before PR 5853. PR 5853 ("atrest: curved copies filling
one volume are interference", merged as `3bb4dc553`) changes exactly this
reading, and its own CI skipped the demos job (run 40566399922), so no
run saw the pin against it. Main's push CI does not run demos either.

## What would settle it

Run the test at `3bb4dc553` and at its first parent. If it reds only at
the merge, decide whether the chain's pins now overlap their bar ends
(re-pin the count and the document's words) or the reading over-reports.
