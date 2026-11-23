---
id: a-face-limb-two-reading-clears-a-carrier-on-the-other-nappe
kind: issue
title: The edge certificate's limb 2 on a cone face can clear a carrier lying on the other nappe: a part whose nappe lever refuses reads √|f|, the double cone's distance
status: open
opened: 2026-11-21
priority: P2
cost: M
---


Found by SECTCONE's `the-search-certificate-reads-limb-two-through-the-edge-reader` lane (PR 7350), 2026-11-22. It predates that PR.

`pcurve_cache/projected.rs` `Meters` (the apex part, `Meters::apex_part`) reads a cone part whose nappe lever is not decided positive by `√|f|`. Here `f` is the canonical composite `cos²α·ρ² − sin²α·z²`, which vanishes on BOTH nappes, so `√|f|` bounds the distance to the double cone, not to the face's nappe. A Face reading (`Limb2Locus::Face`, the nappe taken from the window's middle) of a carrier lying on the other nappe can therefore clear. The lane's first version of the double-cone row did, at 1.2e-8.

**What would close it:** a Face reading's apex part bounds the distance to its own nappe, or refuses. Pin a carrier on the opposite nappe that the Face reading must refuse.
