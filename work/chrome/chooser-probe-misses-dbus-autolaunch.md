---
id: chooser-probe-misses-dbus-autolaunch
kind: issue
title: The chooser probe reads 'no session bus' where rfd reaches a portal through D-Bus autolaunch
status: open
opened: 2026-09-18
---


## Finding

`platform::chooser_backend_of` (`crates/viewer/src/platform.rs`) reads
"is there a session bus" off `DBUS_SESSION_BUS_ADDRESS` alone. `rfd`'s
portal backend does not stop there: with the variable unset it still
reaches a bus through D-Bus X11 autolaunch (`dbus-launch --autolaunch`)
and, where `xdg-desktop-portal-gtk` runs, the PORTAL answers — not
zenity.

Observed on 2026-09-18 while verifying the save-as-directory fix
(`work/vseam/save-as-opens-at-the-filesystem-root.md`), on Ev's WSL box
(`DBUS_SESSION_BUS_ADDRESS` unset, `zenity` on PATH, a long-running
`dbus-launch --autolaunch` plus `xdg-desktop-portal-gtk` 1.6.0 already
up): under an Xvfb display with the variable unset, the first dialog
went through zenity, which itself autolaunched a bus; every later
dialog on that display was the portal's `xdg-desktop-portal-gtk`
window.

What it costs:

- The `(Zenity::NotOnPath, SessionBus::NotAdvertised) => Absent` arm
  disables Open…/Save As… on a box where a portal may well answer
  through autolaunch. The arm is the one the chrome refuses over, so
  it is the one that has to be right.
- Any code that believes "no bus advertised" means "zenity answers"
  is wrong on a box like Ev's. The save-as fix first spelled the
  directory into zenity's `--filename` on that belief and the portal
  then showed the full directory path in its Name field; that arm was
  removed before the PR (see `file_dialog` in `crates/viewer/src/app.rs`).

## What a fix has to decide

Whether the probe should also count "an X11 display and `dbus-launch`
on PATH" as a possible bus (a hint, like `PortalPossible`), or whether
the `Absent` verdict should be narrowed to what autolaunch cannot reach.
