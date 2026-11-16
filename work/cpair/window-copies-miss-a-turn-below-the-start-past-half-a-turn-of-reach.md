---
id: window-copies-miss-a-turn-below-the-start-past-half-a-turn-of-reach
kind: issue
title: holds_only's copies starts at the copy nearest the span's start, so a ring reaching past half a turn misses the copy below it
status: open
opened: 2026-11-09
---


Found by `onecopy/half-open-angles`, repointing `holds_only`'s `copies`
onto `Period::near`.

## The finding

`geom-brep/src/curve_pair/pose/window.rs` `holds_only`'s `copies(t,
reach)` starts at `Period::near(t, t0)`, the copy within half a turn of
the span's start `t0`, and walks UP to the span's end. A copy below that
one, at `t0 − π − δ` or lower, meets `[t0, t1]` when its reach is past
half a turn, and is never collected. The window's half-width is held
under a quarter turn (the `curve_pair_window_width` check at the top),
so only a ring can reach that far. `Window::ring`'s radius is
`(e − gap) / (2·speed_hi)`, bounded only by `e / (2·speed_hi)`: past π
on a conic whose greatest speed is under `e / 2π`, a carrier smaller
than the band's escalation.

Not shown reachable: such a conic is far below resolution, and may be
refused upstream. The old `reduce_periodic_centred` start had the same
gap, plus the lower copy at an exact tie.

## Fix

Either show that no conic slower than `e / 2π` reaches the window, and
state it at `ring`; or walk `copies` down from the nearest copy while
`at + reach` is past `t0` before walking up.
