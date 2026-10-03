---
id: chooser-probe-misses-dbus-autolaunch
kind: issue
title: The chooser probe's 'no session bus' does not mean rfd cannot reach a portal (D-Bus autolaunch)
status: open
opened: 2026-09-18
priority: P3
cost: E
---


## Finding

`platform::chooser_backend_of` (`crates/viewer/src/platform.rs`) reads
"is there a session bus" off `DBUS_SESSION_BUS_ADDRESS` alone. `rfd`'s
portal backend does not stop there: with the variable unset it can
still reach a bus through D-Bus X11 autolaunch (`dbus-launch
--autolaunch`), and where `xdg-desktop-portal-gtk` runs, the portal
answers.

**What was observed** (2026-09-18, verifying PR #2858 on Ev's WSL box
under an Xvfb display, `DBUS_SESSION_BUS_ADDRESS` unset, `zenity` on
PATH, so the probe said `ZenityPresent`): the first dialog on the
fresh display went through zenity, and zenity (GTK) autolaunched a
session bus for that display. Every later dialog there was the
portal's `xdg-desktop-portal-gtk` window. Ev's own `:0` display
already had a long-running autolaunched bus and a portal-gtk 1.6.0.

So in the `ZenityPresent` arm, "no bus advertised" does not tell you
which backend answers. That arm stays usable either way, so the
verdict it gives the chrome is right. What went wrong was a reading
built on it: the first cut of #2858 assumed zenity would answer and
spelled the directory into the file name, and the portal then showed
the whole path in its Name field. #2858 removed that reading before
it merged.

**Not observed, and the reason to keep this row:** the `Absent` arm
(`zenity` not on PATH, variable unset) disables Open…/Save As…. The
same autolaunch could reach a portal there when `dbus-launch` and a
portal are installed, which would make the disabling wrong. Nobody has
reproduced that. It needs a box with no zenity, a portal, and no bus
variable.

## What a fix has to decide

Reproduce the `Absent` case first. If a portal answers there, decide
whether the probe should count "an X11 display and `dbus-launch` on
PATH" as a possible bus (a hint, like `PortalPossible`), or narrow the
`Absent` verdict to what autolaunch cannot reach.
