# Running the viewer

```sh
cargo run -p viewer --features app -- [document.pncad]
```

The optional path is opened at startup through the same typed `Open`
operation the dialog feeds — and it is the only way to open a document
on a system where no file-chooser backend exists (below).

## Mouse bindings

| Gesture | Action |
| --- | --- |
| LEFT click | select (face pick; also feeds the mate tool's picks) |
| MIDDLE drag | orbit (shift+middle: pan) |
| ALT + LEFT drag | orbit — the trackpad binding (plain left never moves the camera) |
| RIGHT drag | pan |
| scroll | zoom |

Left-drag in the viewport is inert **by design** — primary is reserved
for selection; hold ALT to orbit with it on a trackpad. Moving an
instance is not a viewport drag either: the free-move probe is the
typed x/y/z fields in the instance section of the Properties panel
(display-only, mm).

## Troubleshooting (first light was WSLg — issue #1097)

**Open…/Save As… need a file-chooser backend.** The dialogs go through
`rfd`, whose Linux backends are the XDG desktop portal
(`xdg-desktop-portal` plus a frontend for your desktop) or, failing
that, a `zenity` binary — and WSL distros ship **neither**, so out of
the box the dialogs fail. The app probes for a backend at startup:
with confidently none (no `zenity` on PATH, no D-Bus session bus) the
Open/Save As buttons are disabled with the reason as their tooltip —
`apt install zenity` (or a portal) fixes it, and the command-line
document argument above needs no dialog at all. The probe's portal arm
is only a hint: a session bus *without* a working portal frontend
still makes an attempted dialog return the same silent nothing a
cancel does (`rfd` cannot tell the two apart), which is why a
plausibly-present backend that still never shows a dialog reads as
quiet cancels — if that is what you see, install `zenity`.

**Zenity renders every character as boxes (tofu).** A bare WSL distro
has no fonts for GTK to draw with (the viewer itself is unaffected —
egui bundles its fonts). Remedy:

```sh
sudo apt install fontconfig fonts-dejavu-core   # then `fc-cache -f` if needed
```

**Window won't resize horizontally under WSLg — CONFIRMED root cause.**
WSLg presents Wayland windows through an RDP RAIL shell whose
client-side-decoration negotiation breaks horizontal resizing; forcing
the X11/XWayland path fixes it entirely (verified on the first-light
box). The viewer now detects WSL (`WSL_DISTRO_NAME`/`WSL_INTEROP`) and
prefers the X11 backend automatically; non-WSL environments are
untouched and need nothing set or unset. On a build without that
preference, the manual equivalent is:

```sh
WAYLAND_DISPLAY= cargo run -p viewer --features app
```

Findings record: issue
[#1097](https://github.com/evgunter/cad/issues/1097).
