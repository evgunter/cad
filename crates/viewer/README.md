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

In the Properties panel, the document-parameters list ends with an
add-parameter row (name + dimension + value, one undoable edit); an
expression that names an undeclared parameter refuses typed, and the
refusal offers to create it — prefilled into that row, with the
dimension left as your pick.

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

**Dialog opens but every character is a box with tiny hex digits:**
Pango cannot shape the font fontconfig matched. Diagnose with
`fc-match sans` — if it names a `.pfb` (a PostScript Type 1 font,
e.g. Nimbus Sans L from the legacy URW set), Pango dropped Type 1
support years ago and renders hex boxes with no warning. On aged WSL
images the usual root cause is that the DejaVu files are MISSING ON
DISK while dpkg still claims `fonts-dejavu-core` is installed (plain
`apt install` refuses with "already installed"). Fix:

```sh
sudo apt install --reinstall fonts-dejavu-core && fc-cache -f
```

then verify `fc-match sans` now answers DejaVu Sans. (A stale
`~/.cache/fontconfig` can also pin an old match —
`rm -rf ~/.cache/fontconfig && fc-cache -f` is the no-sudo variant to
try first.) The viewer itself is unaffected — egui bundles its own
fonts; only the GTK/zenity dialog needs the system set.

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

## Running it in a browser (spike — compile/link first light only)

```sh
cargo install wasm-bindgen-cli --version "$(awk '/^name = "wasm-bindgen"$/ { getline; gsub(/[",]/, ""); print $3; exit }' Cargo.lock)"
rustup target add wasm32-unknown-unknown
local-scripts/serve-wasm.sh          # prints the URL to open on the phone
```

**What this is.** The single-threaded browser lane — the *named
fallback* in `docs/GUI-PLAN.md`'s platform section, not the threaded
GUI-5 lane Evan deferred on 2026-08-28. Nothing here needs a nightly
toolchain, `-Zbuild-std`, `wasm-bindgen-rayon`, or cross-origin
isolation; the pinned stable toolchain builds it as it stands.

**What it is NOT, so nobody reads more into it.** It has been built
and linked, and nothing beyond that has been verified — no first light
on real hardware, phone or desktop. It is also not CI-guarded: the
wasm32 step excludes `viewer` (`docs/GQ6-RESURVEY.md` §4), so a
dependency bump can break this build with every check green.

Three things are known-absent by design rather than by oversight:

- **No file I/O.** The browser build links no `rfd`, so Open…/Save As…
  are disabled with their reason showing — the same #1125 posture a
  Linux box with no portal and no zenity gets. It opens on the
  built-in startup document and stays there. Document I/O in a browser
  needs the download/upload or OPFS story GUI-5 owns.
- **No touch bindings.** `InputMap` binds orbit to a middle drag, pan
  to a secondary drag, and zoom to a wheel (see the table above); a
  phone has none of the three. egui delivers the touch events and
  `Context::multi_touch` is right there, but nothing consumes them
  yet, so navigation is expected to be unusable on a phone until a
  touch vocabulary lands. `InputMap::map` is a pure
  `ViewportEvent → CameraOp` function, so that work is headless-testable.
- **No phone layout.** `initial_layout` splits viewport-beside-panels
  horizontally at a 1280×800 design size. On a ~390 px viewport the
  four-pane dock is unusable; the panes scroll (#1125) but that is not
  the same as fitting.

**Evaluation runs on the main thread.** The seam takes
`evalseam::InlineEvaluator` here instead of `ThreadEvaluator`, so a
rebuild blocks the frame that submitted it and the tab stops painting
until the kernel returns. The busy indicator cannot help — that needs
an in-op yield point, which GUI-PLAN rules absent for v1.

**No WebGPU over plain http.** WebGPU requires a secure context, and
`http://<lan-ip>` is not one, so `navigator.gpu` is absent on the
phone whatever the browser version and wgpu falls back to WebGL2. That
works only because the `egui-wgpu` edge takes default features, which
include `wgpu/webgl` — putting `default-features = false` on that edge
would leave the phone with no adapter. The page prints
`secureContext` / `navigator.gpu` / `webgl2` in its error box so this
diagnoses itself rather than presenting as a blank screen.
