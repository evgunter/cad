---
id: free-move-refuses-a-body-the-author-showed
kind: issue
title: free-move refuses a body the author showed, and prunes a frame whose space only a show draws
status: open
opened: 2026-11-01
priority: P2
cost: M
---



Found building `the-view-pane-toggles-spaces-and-bodies`.

The View pane's toggle draws a body the default picture leaves out
(`DisplayState::set_body_on` holds it in `toggled`;
`world::Picture::showing` draws it). The free-move doors still read the
default picture alone: `display::free_move_check` asks
`world::Picture::of(doc).is_drawn(node)` and refuses `NotDrawn` on a
shown operand, and `display::space_check` / `DisplayState::prune_with`
ask `world::shown_spaces(doc)`, so a frame on a space whose only drawn
body is a show is discarded as `NotShown`. `display::drawn_deriving_from`
reads the default picture the same way.

**What it should do.** Decide whether a shown body is a free-move
handle on its space. If it is, the three read
`Picture::showing(doc, &DisplayState::shown())`; that makes the frame's survival
depend on display state, which DI5 (`crates/editor-core/IDENTITY.md`)
keys on the document's shown spaces today, so it may want a line there.

Done when: a row shows an operand and either free-moves its space or
reads the typed refusal the pane then explains.
