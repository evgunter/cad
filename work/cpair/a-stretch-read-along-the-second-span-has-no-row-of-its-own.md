---
id: a-stretch-read-along-the-second-span-has-no-row-of-its-own
kind: issue
title: The curve-pair reader's stretch reading along the second span has no row that only it decides (a hairpin)
status: open
opened: 2026-10-16
priority: P3
cost: M
refs: [5227]
---


Found by PR 5227's review (R1, NIT5), on `fuse/curve-pair-reader-c3`.

A stretch is one span's (C5). `curve_pair::Found::finish` reads the stretches along each span in turn (`curve_pair::stretches`, run on `a` and then on `b` with the meetings' sides swapped). So two meetings that one span separates by a definitely-off point, but the other does not, escalate.

Only a hairpin tells the two readings apart, and no row builds one. Two meetings `P` and `Q` on a line `b` would need a span `a` that leaves `b` between them along its own parameter (a loop), while another part of `a` hugs `b` within the band between `P` and `Q`. Closed-form carriers cannot draw that pose: a circle or an ellipse whose span excludes the hugging arc reads the halfway point past its span, so off. It needs a spline span, which the walk reads.

**Done when:** a row builds such a spline against a segment and is red when `stretches` reads `a` alone.
