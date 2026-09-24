---
id: viewer-prose-calls-the-frame-entry-point-update
kind: issue
title: The viewer's prose names the frame entry point `update`, and no such function has ever existed
status: open
opened: 2026-09-16
priority: P4
cost: E
---



## Finding

`ViewerApp`'s frame entry point is `<ViewerApp as eframe::App>::ui`
(`crates/viewer/src/app.rs`, `impl eframe::App for ViewerApp`). Two
places in the tree call it `update`:

- `crates/viewer/src/frame.rs`, `projection_badge`'s doc: *"The status
  line is painted in the toolbar, EARLIER in the same `update` than the
  pane that writes this"*.
- `work/view/the-guard-that-decides-whether-a-preference-is-kept-has-no-test.md`:
  *"the palette picker in `ViewerApp::update`"*.

**It is not a rename, which is the part worth recording.**
`git log -S'fn update(&mut self, ctx: &egui::Context' --
crates/viewer/src/app.rs` returns **nothing**: no commit in this
repository's history ever added such a function to `app.rs`. The name
comes from `eframe::App`'s OTHER method — the one this crate does not
implement — so a reader following it finds a plausible symbol in the
toolkit and never learns that the sentence is about `ui`. That is the
third outcome the register's `git log -S` rule names (never existed),
and it reads from a grep of the current tree exactly like the second
(moved).

## Disclosed rather than repointed

Found by `view/datum-refusals-named`, which wrote three intra-doc LINKS
to `ViewerApp::update` on the strength of the same prose and had them
rejected by `doc-gate.sh` — a link must resolve where a code span need
not, so the same mistake is caught in one direction and not in the
other. The lane's own three were corrected before landing. These two
were wrong at that branch's merge base, so under this program's
in-fence rule they are left as written and named here instead of being
repointed on the lane's authority.

## Fence

`crates/viewer/src/frame.rs` and one row in `work/view/` — both VIEW's.
The repair is two words and its value is that the next lane writing a
link off that sentence does not spend a CI round trip finding out.
