---
id: save-as-opens-at-the-filesystem-root
kind: issue
title: Save As opens at the filesystem root for an unsaved document (Ev-requested, high priority)
status: review
opened: 2026-09-17
branch: vseam/save-as-dir
pr: 2858
---

## Ev's note (verbatim)

> the 'save as' button opens to the filesystem root. it should instead open to the path the viewer was launched from or the last path that was opened previously or something

## Priority

**High priority — requested directly by Ev** (in chat, 2026-09-17, from
Ev's own list of UI nits). This row goes ahead of the rest of the
program's order; see the plan's *Ev's requests* section.

## Where it lives

`app.rs`, `pick_save`. It calls `set_directory` only when the document
already has a path (the parent of the current file). For a document
that has never been saved, no directory is set, and the native dialog
falls back to its own default, which is the filesystem root. `pick_open`
never sets a directory at all.

## What a fix has to decide

The starting directory, in some order of preference:
1. the current document's directory
2. the directory last opened from or saved to in this session (or
   across sessions, through `prefs`)
3. the directory the viewer was launched from (the process's working
   directory at startup)

Ev names options 3 and 2. Open should probably follow the same rule, so
the two dialogs agree.
