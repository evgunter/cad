---
id: the-session-box-reads-no-curved-face-so-an-interior-extreme-pokes-out
kind: issue
title: The session box reads a body's reach off its vertices and edges only, so a curved face whose extreme is interior is admitted past 1 km
status: open
opened: 2026-11-09
---


Found by the delta review of PR 6608 (`onecopy/main-reds-stations`, head 0e255fe7), finding F5. The gap predates that PR.

`topo::session_box` reads a body's reach from its vertices and its edges' own points, whose doc says they "bound its faces". That is false for a curved face whose linear extreme lies in its interior:
- A revolved sphere with r = 50 centred at (0, 0, 975) truly reaches z = 1025. `validate_geometric` admits it.
- The same sphere centred at (975, 0, 0) is refused at −25, because its seam meridian carries the extreme there.

Both the old ball reading and the new cube reading admit the z case.

This is the gap behind the module doc's "every check takes 'in the box' as its premise". Reading from below is the ratified direction (session_box.rs's module doc, geom-core README :336, 8a97c7fa), so a body may poke out by up to the gap between what is read and the true reach. What is not read at all is unbounded.

**Fix shape:** read each face's coordinate range from its carrier over its region. One home for a face's axis-aligned extent may already exist in `Cover`/`world_box` or in extent.rs. Check before minting one.

**Pin:** the z-offset sphere above is refused.
