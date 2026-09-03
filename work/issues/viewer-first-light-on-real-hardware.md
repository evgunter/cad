---
id: viewer-first-light-on-real-hardware
kind: issue
title: viewer first light - run the app on real hardware, and settle what only a GPU can settle
status: open
opened: 2026-08-27
github: 1097
refs: [1094, 1106, 1110]
---

## From GitHub issue 1097

opened 2026-08-27, 3 comments.

Opened by the GUI-0 fix pass (PR #1094) to schedule the deviations that unit
**disclosed but could not close**, per the v5 rule that a disclosed pickup
gets an issue or a named unit rather than a sentence in a PR body.

GUI-0 shipped the `viewer` crate with its renderer-free half fully tested in
headless CI (45 rows across five suites, including two promoted review
suites). Its eframe/wgpu half is **compiled and clippy-clean but has never
been executed** — the implementer lane and both reviewer lanes had no GPU and
no display, and the spec forbade standing up a software rasterizer for it.
Everything below is a question that only a real run answers.

**Extended by GUI-2 (viewport selection)** with §4: the GPU id-buffer pass is
the same shape of debt — written, linted, never executed.

## 1. First light — does the window open, and is the plate on screen

```
cargo run -p viewer --features app
```

What has never run: `ViewerApp::new`, `eframe::App::ui`, every `pane_ui`,
`initial_layout`, and all of `crates/viewer/src/gpu.rs`. The wgpu pipeline has
never been created on a device and the WGSL has never been compiled by naga at
runtime.

Expected on screen: a docked `egui_tiles` layout — viewport pane at three
quarters width, a tabbed side panel (Recipe / View) — showing a flat-shaded
60 × 40 × 8 mm plate with a ⌀24 mm through hole, framed to fill the pane.

Worth checking specifically, because each is a thing CI cannot see:

- **The depth buffer is actually attached.** `NativeOptions::depth_buffer = 32`
  reaches `egui_wgpu::RendererOptions::depth_stencil_format`; if that wiring is
  wrong the symptom is the hole's far wall drawing over the near face rather
  than an error.
- **The first-frame fit.** Startup frames at a provisional square aspect and
  the viewport re-frames at the pane's real aspect on its first layout
  (`pending_fit`). A visible one-frame jump, or a plate that stays badly
  framed, means that handoff is wrong.
- **Navigation feel**: middle-drag orbits, shift-middle or right-drag pans,
  wheel zooms. The rates are `InputMap::default()` and are a value, not a
  constant — if they feel wrong, that is a number to change, not a design.
- **The left mouse button selects** (changed by GUI-2; it was reserved and
  inert before). Clicking a face highlights it and selects the owning feature
  in the tree; clicking empty space clears the selection.

## 2. Back-face culling: `cull_mode: None` → `Some(Back)`

`crates/viewer/src/gpu.rs` draws both sides. The triangles ARE outward-wound
(`mesh::FacePatch`'s contract, asserted in CI as a positive enclosed volume by
the divergence theorem) and the shading uses that winding. What could not be
settled without hardware is the *second* question: which screen-space winding
wgpu calls "front", given NDC is y-up and framebuffer coordinates are y-down.
Getting it backwards makes a closed solid vanish entirely, which is why the
lane chose the option that cannot fail that way.

The change is one line plus a check:

1. set `cull_mode: Some(wgpu::Face::Back)` with `front_face: wgpu::FrontFace::Ccw`;
2. run — if the plate disappears, it is `FrontFace::Cw` instead;
3. record which one it was **in the module comment**, replacing the paragraph
   that currently explains why culling is off. That paragraph is the thread to
   pull; it names this issue.

**There are now TWO pipelines to change together** (GUI-2): the shaded pass and
the id pass, both `cull_mode: None` for the same reason. A run that turns
culling on must turn it on in both, or the id buffer and the picture will
disagree about which faces exist.

Not urgent: with a depth buffer and an opaque closed body the two are visually
identical. It is worth doing because the *reason* it is off is ignorance, and
ignorance recorded is a debt.

## 3. Anything the run reveals

The lane's honest expectation is that something here is wrong in a way no
amount of reading catches. That is what first light is for. File follow-ups
against this issue rather than silently patching.

## 4. The GPU id-buffer pass — GUI-2's addition

`crates/viewer/src/gpu.rs`'s `IdPass` renders per-patch ids into a **1×1**
`R32Uint` target and reads back four bytes at the cursor. Nothing in it has
executed. Everything about it that a machine without a GPU can check IS checked
— the id↔(node, body, patch) mapping is a pure function pair with round-trip,
collision-freedom and stability rows in `crates/viewer/tests/select_pick.rs`,
and the transform that decides which pixel it samples
(`viewer::cursor_projection`) has a row composing it with the camera's
projection. What is left is the pass itself.

**The verification, and it is one gesture.** The application already runs both
picking paths on every hovered cursor and compares them: the ray path
(`pick::PickIndex`, the shipped answer, headlessly tested) and the id pass. A
disagreement prints in the status line as

```
picking paths disagree at the cursor: id buffer , ray 
```

So the check is: **open the app, sweep the cursor slowly over the plate — the
top face, a side wall, the cylindrical hole wall, and off the model — and watch
the status line.** Silence is agreement. Specifically confirm:

- **Hover highlight tracks the cursor**, and the highlighted patch is the one
  under it (a blue-ish tint; the selected face is orange-ish).
- **No disagreement message anywhere on the sweep.** One appearing over a
  particular kind of face is the finding — record which face and which two ids.
- **Off the model both answer nothing**: id `0`, ray `0`, so no message and no
  highlight.
- **Click a known face and read the tree.** The owning feature highlights; the
  property panel shows its slots. That is the same selection value.

Three ways this can be wrong that only hardware shows, listed so a failure is
diagnosable rather than mysterious:

1. **The 1×1 trick's sign or scale is wrong** — `cursor_projection` blows one
   source pixel up to fill the whole target. A y-flip error makes the id buffer
   answer the face mirrored about the pane's horizontal centre line, which
   reads as "disagrees everywhere except near the middle".
2. **`R32Uint` clear semantics** — the pass clears with
   `LoadOp::Clear(Color::TRANSPARENT)` expecting zero (`IdMap::NOTHING`). If a
   backend clears a uint target differently, the miss case answers garbage and
   the status line will say so over empty space.
3. **The blocking readback** (`queue.submit` + `poll(Wait)` + `map_async`
   inside `prepare`) — if it deadlocks or stalls, the symptom is a frozen or
   crawling frame rate while the cursor is inside the viewport, not a wrong
   answer. If the cost is unacceptable, the fix is to gate the query on the
   cursor having MOVED rather than to remove the comparison.

If the two paths agree across the sweep, say so on this issue: that is the
hardware half of GQ6-RESURVEY §3's two-lane picking strategy, and it is the
only place it can be taken.

## Not in scope

- The `app` feature staying non-default — **settled OFF** by Ev's
  viewer-CI-posture ruling (`docs/GUI-LOG.md`, 2026-08-27).
- `epaint_default_fonts`'s OFL-1.1 / Ubuntu-font-1.0 row — a judgment stated
  in PR #1094's body, with a named fallback if Ev overrules it. No issue
  unless he does.

## Comments

**2026-08-27** — orchestrator:

(GUI orchestrator) Partial first-light reading taken on a SOFTWARE adapter — every item below is llvmpipe/lavapipe under Xvfb (Mesa 25.2.8), not real hardware; the hardware check this issue asks for stays open.

- The full egui+wgpu pipeline creates a device, surface, and draws every frame with clean stderr (`cargo run -p viewer --features app` on main @ `66460de4`, zero code changes).
- The plate renders **solid, not inside-out**: top lit, sides darker, hole interior shaded correctly, underside correctly darker from below. With `cull_mode: None`, nothing suggests winding trouble on this adapter — the culling flip's visual precondition looks satisfiable, but the flip itself should still be verified on hardware per this issue's checklist.
- Chrome, tree selection, Properties, View telemetry, and middle-drag orbit all render and respond.
- Not exercised: real GPU adapter, presentation/vsync, DPI scaling, the rfd Open dialog (no portal service headless).

Screenshots + setup README: `docs/gui-shots/2026-08-27/` (PR #1110). One automation note for whoever does the hardware pass: left-drag in the viewport is inert BY DESIGN (primary is reserved for GUI-2 selection; orbit=middle, pan=secondary) — it is not a dead camera.

---
_Generated by [Claude Code](https://claude.ai/code)_

**2026-08-27** — orchestrator:

## §4 partial reading — the id pass has now EXECUTED, on lavapipe

From the GUI-2 fix pass (PR #1106). Shots and the exact recipe are at
`docs/gui-shots/2026-08-27/README.md` (`05`–`09`), same Xvfb + lavapipe
stack #1110 established. **Software rasterizer, so this closes nothing
here** — it moves three questions from "never run" to "run once, on a
driver that is not a driver".

**What ran.** Everything in `crates/viewer/src/gpu.rs`: pipeline
creation, both WGSL entry points compiled by naga at runtime, the 1×1
`R32Uint` target and its clear, `copy_texture_to_buffer`, `map_async`
and the blocking readback. A hovering cursor takes that whole path, and
these shots hovered.

**What agreed.** The app compares the id pass's answer against the ray
path's and prints `picking paths disagree at the cursor: …` when they
differ — the one-gesture check §4 describes. None of the five shots
carries that message, over the plate's cap, over its cylindrical hole
wall, and over empty space where both sides must read as nothing. So
failure mode 1 (a sign or scale error in the 1×1 trick) is not present:
that one is geometry and would misbehave identically on any rasterizer.
Its headless half is also now pinned by three rows that go red under
both `* -sx` and `* (sy * 100.0)` — the two mutations that used to
survive.

**What is still open on real hardware**, unchanged:

- **`R32Uint` clear semantics** (§4 failure 2) — lavapipe clearing a
  uint target to zero says nothing about another driver doing so.
- **The blocking readback's cost** (§4 failure 3) — a software adapter's
  frame rate is not the signal. Note the readback is now **gated on
  cursor movement** (`frame::IdQueryLog`), which §4 named as the remedy
  if the cost bites; it was documented but absent before, and is real
  now.
- **Culling** (§2) — still `None` in BOTH pipelines, still one change
  with one check.
- Depth attachment, first-frame fit, navigation feel (§1) — #1110's shots
  cover the software half of those already.

One §4 correction while here: the sentence "Off the model both answer
nothing: id `0`, ray `0`, so no message" was true of the intent and not
of the code. Leaving the pane used to leave the last answer matched
against a cleared hover, so the message fired permanently over empty
space — exactly the symptom §4 tells the operator to read as a clear-value
fault. Fixed in #1106 (the outstanding query is voided when the pointer
leaves) and covered by a replay row; an operator following §4 today will
not meet that false positive.


---
_Generated by [Claude Code](https://claude.ai/code)_

**2026-08-28** — orchestrator:

(GUI orchestrator) **First light on real hardware happened — Ev ran the app.** The good news first: the pipeline draws on a real GPU, orbit/selection/panels work, and the checklist's geometric questions look answered in practice. Two findings from the run, both real-hardware-only (invisible under Xvfb/lavapipe):

1. **Window resize broken horizontally** — the window resizes vertically (from the top bar only) but not horizontally, so content sticks off the right edge unreachably. `run()` passes bare `NativeOptions::default()` (no ViewportBuilder at all), so resizability/decorations are whatever the winit backend negotiated; hypothesis is a Wayland CSD negotiation issue. Awaiting `XDG_SESSION_TYPE` + desktop + whether forcing XWayland changes it.
2. **Open/Save As silently do nothing** — no dialog appears. `rfd`'s blocking portal backend returns `None` both on user-cancel AND on backend failure (no `xdg-desktop-portal` reachable, no `zenity` fallback binary) — the silent-failure lane the GUI-3 review flagged as an unmentioned runtime dependency. Awaiting portal-service status + zenity presence. Workaround meanwhile: the CLI document argument (`viewer `).

Hardening PR incoming regardless of root cause: explicit ViewportBuilder (resizable + sane default/min inner size) and a status-line message when a dialog returns nothing. Root-cause fixes follow the diagnostics.

---
_Generated by [Claude Code](https://claude.ai/code)_

## Home

Viewer/GUI ground: the GUI v1 program is closed and may hold only closed items, so this open first-light checklist lands under `work/issues/`.
