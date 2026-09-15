---
id: a-refused-chart-boundary-has-no-reachable-window
kind: issue
title: window_of's chart_boundary Err arm is unreachable from any fixture on this tree
status: open
opened: 2026-09-14
refs: [clearance-window-tightening-needs-chart-boundary]
---



## What

`window_of`'s description arm has an `Err(_)` branch: a boundary walk
that refuses leaves today's window standing, `bound = None`, and the
window counted loose in `ClearanceReport::windows`. That branch is the
identity claim TRIM-3 PR-2's E8 row was written for, and **no fixture
on this tree reaches it**.

Measured by both arms of PR-2's v6 dual: across every row of the
clearance suites the printed counts are `(n, 0)` — never a loose window
— and every `chart_boundary` call from `window_of` returns `Ok`. E8's
own fixture refuses one level earlier, at the SELECTION door, so its
`windows() == (0, 0)` is `ClearanceReport::refused`'s constant and not
the arm at all; E8 now says so in its name and its doc. A mutant that
turns the `Err` arm into a `ClearanceRefusal::Unsupported` survives the
whole suite.

## Why it is hard to reach

`chart_boundary` refuses on: a loop that wraps a whole period
(`LoopWraps` — no head constructor makes one, by design), a joint at a
sphere pole or cone apex (`window_of` never asks: `chart_arms` answers
`None` for those carriers, so the walk is not called), and a
`General`/`Fitted` carrier with no stored certificate
(`Certify { UnsupportedCarrier }` — and `EdgeCurve::certify` refuses a
NURBS carrier under a conventional description at build time, so no
door makes such a body). The reachable set is empty, not merely
untested.

## Fix shape

Either a `topo` door that mints a body with a `General` pcurve on a
plane or cylinder chart — which is what
`work/trim/general-pcurve-face-props-and-tess-refuse` is already about
and which would give this arm its first fixture — or a unit-level seam
in `clearance.rs` that lets a test hand `window_of` a refusing walk.
The first is real coverage and the second is a mock; prefer the first,
and until either lands read `windows()`'s second number as unexercised
rather than as zero-because-nothing-refuses.

## Home

TRIM — the arm is PR-2's, in the announced seam.
