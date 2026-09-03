---
id: probe-cutaway-comment-claims-shipped-box
kind: issue
title: probe.rs says the cutaway probe splits the SAME box the projectbox stop ships — the shipped halves now carry a +5 m offset the probe's don't
status: open
opened: 2026-09-01
github: 1518
refs: [1506]
---

## From GitHub issue 1518

Opened 2026-09-01; 0 comments.

`demos/tour/src/probe.rs`'s `projectbox_cutaway` group says it samples
the same bodies the tour ships:

> The cutaway splits the SAME box the projectbox stop ships

Since the montage-v3 tranche-2 cell merge that is no longer quite true.
`projectbox`'s stop now places the two section halves beside the whole
box (`cutaway::SECTION_GAP`, +5 m along x) so both fit one montage cell,
while the probe builds its own halves from the un-offset box. The
shapes are identical; the coordinates differ by 5 m.

## Why it is worth a line rather than a shrug

Nothing is wrong today. The K-margin telemetry the probe records is
about predicate conditioning, and a rigid translation of 5 m on a part
whose features are O(0.1–3 m) is unlikely to move any margin that
matters. The tour's own assertions are unaffected — they run on the
shipped bodies.

What is worth recording is that a stated premise quietly stopped
holding. "The probe measures the same body the stop ships" is exactly
the kind of sentence that is load-bearing when someone later reads a
probe row as evidence about a shipped body, and it is the kind that no
gate checks.

## Options

1. Have the probe take the placed halves, so the sentence is true again.
2. Keep the probe on the un-offset box and reword the comment to say
   the probe measures the section's SHAPE, not the shipped placement.

(2) is probably right — the offset is presentation, and telemetry taken
at the origin is the cleaner reading — but either way the comment and
the code should agree.

Found by a review of #1506.

## Home

`work/code-quality/` — a comment whose stated premise stopped holding is prose debt, the register's ground; `demos/tour` is in no open program's territory.
