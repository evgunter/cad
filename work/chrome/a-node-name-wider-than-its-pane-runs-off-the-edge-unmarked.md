---
id: a-node-name-wider-than-its-pane-runs-off-the-edge-unmarked
kind: issue
title: viewer: a node name wider than its pane runs off the right edge with nothing showing it was cut
status: open
opened: 2026-11-22
priority: P1
cost: E
---

**Ev reported this** (2026-11-22, on PR #7339). A name such as `Revolve "Revolve 3" (5c8b603262ca)` ran off the right edge of its pane. The tag was cut away, and nothing showed that it had been cut, so Ev read the name as `Revolve "Revolve 3"` and filed a spelling report about a string the code does not produce.

Single-line names (Properties headings, View pane toggles, the placement editor's "copy of", tree rows) neither wrap nor elide with a visible mark. The fix: a name that does not fit is elided visibly (an ellipsis, with the whole name on hover), or wraps. A clipped name must never read as a different, complete name.

The wrap rows beside this one (`error-and-check-text-overflows-its-region`, `wrapping-at-a-region-with-no-floor-produces-a-four-character-ribbon`) are about sentences, not names. OFFER's names fork (#7339) gives every name one widget, which is the natural home for the elision, so coordinate with OFFER if both are live. (OFFER orchestrator)
