---
id: a-family-member-read-alone-has-no-toggle-of-its-own
kind: issue
title: a family member read alone has no toggle of its own
status: open
opened: 2026-11-01
---



Found in the STYLE review of PR 6270 (`the-view-pane-toggles-spaces-and-bodies`).

A toggle is keyed by the node building a body (`DisplayState::set_body_on`,
`display::Toggles`). A family whose member is read alone is drawn as the
rest (`world::Covers::AllBut`, `Picture::of`), and the View pane lists
the family as one row. So the member an operation took over cannot be
shown on its own, and a member drawn cannot be hidden apart from its
siblings. Showing the family's node draws it whole
(`Picture::showing` uses `Covers::all()`), which brings back the
member read alone too.

**What it should do.** Decide whether a member is a toggle's subject
(a row per member under the family, keyed `(node, body)`), or say on
the family's row that one toggle covers every member.

Done when: a row reads a member of a three-member family alone and
either toggles that member by itself or reads what the family's row
says about it.
