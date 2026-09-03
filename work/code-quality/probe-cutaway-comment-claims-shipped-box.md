---
id: probe-cutaway-comment-claims-shipped-box
kind: issue
title: probe.rs says the cutaway probe splits the SAME box the projectbox stop ships — the shipped halves now carry a +5 m offset the probe's don't
status: closed
opened: 2026-09-01
github: 1518
refs: [1506]
closed: 2026-09-03
branch: smell/x-prose
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

## Closed

Landed on `smell/x-prose` (SMELL Track X), comment-only. **Option 2**,
as the issue recommends: the probe stays on the un-offset box and the
comment now says what it measures.

The first sentence was true and stays — the cutaway does split the same
box (`projectbox::build`'s `acc.body`, which is also what
`cutaway::sectioned_beside` takes at `demos/tour/src/projectbox.rs:105`).
What had quietly stopped holding was the reading of it, so the comment
now states the seam out loud: the group meters the section's SHAPE, the
stop places the halves `SECTION_GAP` = 5 m along +x
(`demos/tour/src/cutaway.rs:73,96-100`) so the pair and the whole box
share one montage cell, and the probe takes `cutaway::build`'s output
where it stands. Same ops, same halves, coordinates 5 m apart.

Option 1 was not taken and the issue's ground for that holds on
re-reading: the offset is presentation applied by `sectioned_beside`
after `build`, and the probe consumes `build` directly, so taking the
placed halves would mean metering a body at a rendering offset. The
K-margin population is unchanged, so no `k-lint` baseline moves.
