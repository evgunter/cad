---
id: a-corner-supports-replaced-region-is-not-isolated-by-a-row
kind: issue
title: No row isolates a mixed corner's replaced support region against another chain's reach
status: open
opened: 2026-11-17
priority: P3
cost: E
refs: [7078]
---

Found by PR 7078 (lane `band-reach`). A corner reach's `replaces` (`reach::Stretch::replaces`) clears the cells of a corner support that another chain's reach meters, where the patch replaces that support. PR 7078 changed what the mixed corner's `replaces` holds and where a link's replaced strip starts at a corner (past the patch, not from the vertex). Its exactness is argued in the PR but pinned by no row that isolates it.

What a row needs: a body where a chain that does not meet the corner has a reach crossing one of the corner's supports, inside and just outside the region the patch replaces. For example, a short requested edge of a feature standing on `E` beside the corner, whose band's reach reaches `E`'s replaced annulus. On the inside it must clear; just outside, it must refuse. Not attempted in the fix pass. The likely hard part is a fixture where the other chain's reach meets a corner support before the face is skipped, by that chain's own supports or by the corner's skips.
