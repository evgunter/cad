# GUI first-light screenshots — 2026-08-27

First-ever captures of the `viewer` app (eframe + egui + wgpu, `app`
feature), taken headless under a **software** stack:

- **Adapter: lavapipe/llvmpipe** (Mesa 25.2.8, `DRIVER_ID_MESA_LLVMPIPE`,
  LLVM 20.1.2) via `mesa-vulkan-drivers` — **not real GPU hardware**.
  Nothing here speaks to issue #1097's real-hardware checklist items;
  this is only the software-adapter partial reading.
- **Display: Xvfb `:99`**, 1280x800x24, captured with ImageMagick
  `import -window root`.
- **Commit rendered: `66460de4debd57d36a92a067d84b5aafac3e3944`**
  (origin/main, merge of PR #1105).
- **Launch**: `cargo build -p viewer --features app` then
  `DISPLAY=:99 target/debug/viewer` (debug profile; no arguments — the
  binary takes none, so these show the built-in `plate_with_hole`
  startup document).

## Shots

| File | What it shows |
| --- | --- |
| `01-startup.png` | Startup scene as launched: toolbar, viewport with the plate-with-hole body, feature tree (Profile, Extrude), empty Properties pane. |
| `02-extrude-selected.png` | After clicking the Extrude tree row: row highlights, Properties shows `Distance 0.0080 Length` with the expression button. |
| `03-view-tab-orbit.png` | View tab telemetry: display δ 0.100 mm, faces 8, triangles 160, camera yaw −60.0° pitch 30.0°, distance 109.0 mm, history 1 state, "unsaved document". |
| `04-orbited.png` | After a middle-button orbit drag (yaw −142.5°, pitch −14.0°): the body seen from below, underside correctly darker. Left-drag is inert by design (reserved; orbit is middle, pan is secondary). |

## Caveat

Everything above was rendered by llvmpipe. Real-hardware first light
(actual GPU adapter, presentation path, vsync, DPI) remains untested;
treat these as evidence the pipeline draws correctly on the software
adapter only.

## GUI-2 addendum — selection, captured live (same recipe, later commit)

Added by the GUI-2 fix pass (PR #1106) with the recipe above unchanged:
Xvfb `:99` at 1280x800x24, lavapipe, `import -window root`, debug
profile, the built-in `plate_with_hole` startup document. Cursors were
driven with `xdotool mousemove` / `click 1`; the app window is 800x600
inside the root and the viewport pane is its left two thirds, so every
cursor is `x < 520`.

| File | What it shows |
| --- | --- |
| `05-startup.png` | The startup scene with the fix pass's changes in it — the baseline the four below move from. |
| `06-hover-highlight.png` | Hover over the hole's cylindrical wall: the wall tints BLUE, and Properties still reads "select a feature". Hover is transient and never touches the selection. |
| `07-face-selected.png` | Click, same cursor: the wall tints ORANGE, the **Extrude row highlights in the feature tree**, and Properties shows `face of feature 1`, its `Delete feature` affordance and the owning node's `Distance 0.0080 Length`. One selection value; the tree and the panel are views of it. |
| `08-second-face-selected.png` | Click the plate's top cap: the cap tints and **the hole wall reverts**. Single-select replaces; nothing accumulates. |
| `09-cleared-by-empty-click.png` | Click empty space inside the viewport: highlight gone, tree row unhighlighted, Properties back to "select a feature". A click that hits nothing clears. |

### What this is evidence of, and what it is not

**The GPU id pass executed for the first time.** Until these shots
`crates/viewer/src/gpu.rs` had never run: the pipeline creation, the
WGSL, the 1×1 `R32Uint` target, its clear, the `copy_texture_to_buffer`
and the blocking readback are all on the path a hovering cursor takes.
They ran here, on lavapipe.

**And the two picking paths agreed at every cursor.** The application
compares the id pass's answer against the ray path's by name and prints
`picking paths disagree at the cursor: …` in the status line when they
differ (issue #1097 §4's one-gesture check). No shot above carries that
message — including `09`, over empty space, where the id buffer's clear
value and the ray's miss must both read as "nothing".

**It is not the hardware reading.** lavapipe is a software rasterizer:
it says the code is correct, not that a real adapter agrees. #1097 §4's
three named failure modes stay open on hardware — a sign/scale error in
the 1×1 trick would show here too and does not, but `R32Uint` clear
semantics and the readback's frame-rate cost are both properties of a
real driver. Culling is still off in both pipelines (§2).
