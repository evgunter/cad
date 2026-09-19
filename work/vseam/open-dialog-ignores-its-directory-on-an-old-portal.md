---
id: open-dialog-ignores-its-directory-on-an-old-portal
kind: issue
title: Open… ignores its starting directory under xdg-desktop-portal 1.6 (Ev's box)
status: open
opened: 2026-09-18
---


## Finding

Open… and Save As… now take their starting directory from one rule
(`frame::dialog_dir`, through `ViewerApp::file_dialog` in
`crates/viewer/src/app.rs`), and `rfd` hands it to the portal as
`current_folder`. Save As… honours it on the portal Ev's box runs
(`xdg-desktop-portal` / `-gtk` 1.6.0, reached through D-Bus
autolaunch). **Open… does not**: the same portal ignores
`current_folder` on its OpenFile call and opens on its own "Recent"
view. Observed 2026-09-18 under Xvfb on that box, with a remembered
directory in the preferences.

`current_folder` on OpenFile arrived in a later revision of the portal's
FileChooser interface than the one 1.6 implements, so there is nothing
more to hand `rfd` for this backend. When zenity answers instead, Open…
opens at zenity's default (the process's working directory, which is the
launch directory), again without the document's or remembered
directory, because `rfd` forwards no directory to zenity.

## What a fix has to decide

Whether this is worth anything beyond the platform upgrade: e.g. a
text path field on Open…, or a "recent documents" list in the viewer
itself. Low priority next to the Save As… fix, which is the half Ev
reported.
