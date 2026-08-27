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
