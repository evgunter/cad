---
id: a-mate-on-a-taken-over-copys-face-goes-stale
kind: issue
title: A mate another placement seats on a face of a body's copy goes stale when a fillet or mirror takes that copy over: the mated part is no longer placed
status: open
opened: 2026-10-25
priority: P0
cost: M
---


Found by the third review of PR 5683 (the mirror tool; sure, probed). A small block is mated onto a base's top face, and then the base is mirrored. The small block's `Place` row fails: "constraint #… side … no longer resolves … the recorded reference disagrees with the recipe … (Place <base copy>'s payload differs). Recourse: rebind it." The small block is no longer placed.

A fillet of the base fails identically (the reviewer's comparator), so this is the takeover's own rule, not the mirror's. `repoint` changes the copy's payload, and every recorded reference that reads the copy's faces goes stale. The refusal is typed and carries a recourse, but the everyday case is exactly this: a symmetric or filleted base with parts seated on it. `session::select`'s `Renamed::through` does not cover it; it serves the transform's one-placement-deeper case.

**To decide when dispatched.** Should the takeover re-seat recorded references whose face survives in the result through the result's names (a fillet and a mirror both keep the source's names; the image carries them verbatim), or does the reference record the wrong thing? Check whether the recorded payload should name the copy's body rather than its source's operation. The tests should mate onto the body that is then taken over, the case the mirror suite's mated row excludes.
