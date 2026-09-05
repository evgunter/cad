---
id: document-news-has-no-home
kind: issue
title: What a tool has to say is one door, but the panes reach it three different ways
status: open
opened: 2026-09-05
---


## What this is

`frame::tool_news` is the door for what a tool has to say —
`Subject::Document`, twelve call sites. It is the one door in that
family a type does not pin (its sites render through
`tools::ToolKind::says`, `tools::ToolNotice` and the typed forms
vocabulary, and arrive as text), and the residue is that the sites
reach it three different ways:

- `crates/viewer/src/pane/create.rs` — ten sites, each
  `frame::tool_news(ToolKind::X.says(&error))` or similar.
- `crates/viewer/src/app.rs` — two sites, inside `.map()` closures over
  `tools::ToolNotice` and the seat vocabulary's declined picks.

A door that takes a `String` cannot tell a tool's words from anyone
else's, so nothing stops a future site handing it a camera refusal.

## The shape of an answer

`tools::ToolNotice` and `ToolKind::says`'s output become one typed
value with a `Display` — the shape `frame::Withdrawal` took in the same
unit and the shape `prefs::Notice` already has — and `tool_news` takes
that type instead of a `String`. Then the door is type-pinned like its
six siblings and the twelve sites hand over a value rather than text.

`ToolKind::says` returning `String` (`crates/viewer/src/tools.rs:118`)
is the thing in the way: it is a wording function over `impl Display`,
so its output has no type of its own.
