---
id: viewport-reads-one-of-the-scroll-deltas-two-axes
kind: issue
title: The viewport reads the scroll delta's y axis and drops its x, with no statement either way
status: open
opened: 2026-09-12
---



Filed by the DOOR unit that closed
`work/door/viewport-pointer-buttons-mirror-a-toolkit-enum-by-hand`.
It is the same shape as that row's defect, one axis over, and it is
`crates/viewer/src/pane/viewport.rs`, which is VIEW's ground.

## The finding

`viewport_ui` reads the toolkit's scroll delta as
`ui.input(|i| i.smooth_scroll_delta.y)` and builds
`input::ViewportEvent::Scroll { units }` from it. `smooth_scroll_delta`
is an `egui::Vec2`: `x` is the horizontal axis a trackpad's two-finger
sideways swipe and a tilt wheel produce, and it reaches nothing. No
`ViewportEvent` names it, so no binding could read it even if the
adapter passed it on.

That may well be the right product answer — the viewer's only scroll
binding is zoom, and horizontal scroll has no CAD convention behind it
the way middle-drag orbit does (`crates/viewer/src/input.rs`, module
docs). What is missing is that **nothing says so**. An adapter that
takes one of an upstream value's two components, with no sentence at
the site, cannot be told apart by a reader from one that has fallen
behind its upstream — which is exactly what the DOOR row above turned
out to be.

## The shape an answer has

The same one that row took: state the decision where the narrowing
happens. Either `x` gets a binding (and then a vocabulary that can
carry it), or the site says which axis the viewer reads and why the
other one is not a gesture here. The button half of the same adapter
now does this in `viewer_button`'s doc, and this is the residue that
fix did not reach.

Cheap either way; the point is the statement, not the code.
