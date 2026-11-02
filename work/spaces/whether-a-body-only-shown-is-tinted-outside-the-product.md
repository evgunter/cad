---
id: whether-a-body-only-shown-is-tinted-outside-the-product
kind: issue
title: whether a body only the author's show draws carries the outside-the-product tint
status: open
opened: 2026-11-01
priority: P2
cost: E
design: true
---



Found in the STYLE review of PR 6270 (`the-view-pane-toggles-spaces-and-bodies`).

**This is a design question** for Ev, and wants an `[ev]` PR before
code.

A drawn body the product slot does not reach is tinted outside the
product (`crates/viewer/README.md`, *The picture draws bodies*;
`world::Picture::is_outside`, `PickIndex::is_outside`). A body only the
author's show draws, such as an operand a boolean took over shown
again (`world::Picture::showing`), is never reached by the product slot,
so it is tinted too. That reads correctly ("this is not the
product"). But the tint was written for bodies the default draws beside
the product, and a shown operand sits inside the very boolean that
is the product, where the tint may read as "this is a separate part".

**The fork.** Tint a shown-only body as outside (as now), draw it with
a tint of its own (a "shown for inspection" look), or draw it untinted.

Done when: Ev has ruled and a row holds the ruling on a shown operand.
