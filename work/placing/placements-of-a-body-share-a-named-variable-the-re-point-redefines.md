---
id: placements-of-a-body-share-a-named-variable-the-re-point-redefines
kind: issue
title: Placements of a body read a variable under the usual naming rule, so several placements share a named one (bracket = B) and the re-point redefines it (bracket = F)
status: open
opened: 2026-11-09
priority: P1
cost: M
---


## Ev's direction (2026-11-09, `[ev]` PR 6581)

> i think the workflow i want looks more like "placements of bodies take a variable with the same rules as usual, ie that an anonymous variable can only be used in one place. so if you have multiple placements of B you already have defined `bracket = B` or similar, and must update this to `bracket = F` (perhaps with the gui giving you a shortcut for this)"

## Today (after PR 6581)

- **Placements read the body directly.** A placement's source reads the body's own output. Outputs are exempt from the naming rule (VR2/VR7: the node names them), so two placements of `B` both read `B` directly, with no named variable between them.
- **The re-point rewrites each placement.** The re-point offer (`world::repoint_targets`, `SessionOp::RepointPlacements`) rewrites each placement's source from `B` to `F` in one action, through `take_over`.
- **A new placement still reads `B`.** After a re-point, a new Place of `B` still reads the unfilleted body.

## Wanted

- **The usual naming rule applies to placement sources.** A placement's source is a variable read like any other, so an anonymous variable feeds one placement only. Two or more placements of one body read a named variable (`bracket = B`).
- **The re-point is one definition edit.** "Re-point" means editing that definition (`bracket = F`). The GUI shortcut is the offer that PR 6581 built, now writing that one edit instead of per-placement sources.
- **Open, for the build or designers:**
  - **How the name comes into being.** When a second placement of `B` is made, does Place ask for a name, mint one, or refuse until one is given? What does the first placement read?
  - **Where the exemption stops.** Does the output exemption stop at placement sources only, or does the rule change more widely?
  - **What reading the name means.** What does a later Place of `bracket` read?
  - **How delete interacts.** How do delete-by-continuation and `take_over` interact with a named source?

**Done when** a row:
- places `B` twice and reads one named variable as both placements' source;
- fillets `B` and accepts the offer, which writes one `DefineVar` (`bracket = F`), so both copies show `F`;
- places `bracket` again and shows `F`;
- undoes the offer in one step.
