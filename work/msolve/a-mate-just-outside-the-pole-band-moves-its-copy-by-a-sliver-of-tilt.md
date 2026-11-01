---
id: a-mate-just-outside-the-pole-band-moves-its-copy-by-a-sliver-of-tilt
kind: issue
title: A mate just outside the pole band moves its copy by its whole lever for a sliver of tilt, and no authoring lint says so
status: open
priority: P3
refs: [5742]
opened: 2026-10-26
---


Found by review 2 of PR 5742 (the value chart reads a written orientation), probe `r2_band_edges_and_the_copy_just_outside` on `review/intent-value-chart-R2`.

**What happens.**
- Set-up: a unit boss hangs under a plate, its orientation written backwards (`x: +x, z: −z`, its `z` along `−z`), at `Slide(X) = 10`.
- Inside the pole band (`mate_chart_pole`) the chart refuses `MateFault::Backwards` and names the orientation to write.
- Just outside the band, at a tilt of 1e-8, the copy places. Its side depends on which way the plate tilts: about `x` the boss lands at `x = +10`, about `y` at `x = −10`. A sliver of tilt moves it about 20 units.
- That is the least turn's `1/|n + d|` sensitivity near `n = −d`. It is continuous, it refuses only inside the band, and it is not a jump short of the band. But an author cannot tell from the document that a value now means a different place.

The unit (`work/msolve/the-value-chart-reads-a-written-orientation.md`) promises at most a lint here, and none exists.

**Owed.** An authoring lint: a mate whose orientation puts the copy's mated direction within some angle of the mated direction's opposite (`|n + d|` small, levered over the bundle's arm) is flagged when the author writes it, naming the orientation `toward` would pick (`MateFault::Backwards`'s `write`). It is a lint, never a refusal: the chart already refuses where it must.

**Done when** a row writes the backwards-oriented boss at a tilt just outside the band and sees the lint name the orientation to write, and a tilt well clear of the pole sees none.
