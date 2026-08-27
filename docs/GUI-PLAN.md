# GUI-1 — the v1 GUI (plan, DRAFT)

**STATUS: DRAFT, second round — the 2026-08-27 in-conversation
rulings (Evan) are folded and marked RULED; the plan as a whole is
not yet ratified.** Every *decision* this plan leans on is ratified
elsewhere and is cited, not re-litigated: the three-layer split and
boundary rules (`docs/GUI-DESIGN.md` G1, including the
operations-are-API rule recorded 2026-08-27), the v1 minimum (G3),
the toolkit (GQ6: **egui**, iced the named fallback), and the
viewport/picking recommendations (`docs/GQ6-RESURVEY.md` §§2–3).
Sequencing stance: DESIGN.md places GUI after usable-as-library;
starting this program is Evan's call per LQ5, and LIB's
dispatchable residue (the Python assembly series) blocks nothing
here.

## Rulings (Evan, 2026-08-27, in-conversation)

- **Platform: native before browser.** The web lane is GUI-5,
  separable, never on the v1 acceptance path.
- **Selection: single-select for v1.** Mate authoring gets its two
  picks through tool state, not through a general multi-select
  (GUI-4; the pick mechanics are an implementation-time choice).
- **Undo: linear for v1.** The history tree stays future; see the
  undo note below for what v1 holds open for it.
- **Units: canonical meters/radians in the panels.** U8's
  units/display layer is not a dependency.
- **Long evaluations: busy indicator + the shipped `CancelToken`.**
  Progress reporting and in-op yield points stay absent.
- **Exit demo: an open dialog over the existing demo documents**
  (see acceptance below).
- **Mate authoring is v1 scope** — defining a new mate between
  previously-unmated parts, an extension beyond G3's four items,
  ruled in because free-move fit-probing exists precisely to
  precede it.

## Scope — G3 verbatim, the ruled addition, and the implied substrate

The G3 four: **click-to-select** feeding the existing edit doors
(no in-viewport manipulation); **pan / rotate / zoom**;
**free-move of completely-unconstrained assembly parts** (display
transform, no solver, visually distinct from mated placement);
**hiding parts**. Plus the ruled addition: **defining a mate from
two picks** (below). Implied substrate: a viewport rendering the
evaluated bodies of an open document; a feature tree over the GQ2
per-node result DAG (Failed/Poisoned badges, messages from the
typed payloads); a property panel applying `DocEdit`s through pure
`apply` with incremental re-evaluation; open/save on the shipped
snapshot+log persistence.

**Exclusions, stated so nobody reads more:** live dragging/editing
of constrained geometry (G3 excludes it by ratification); the
sketcher (G2's nested editor is a later milestone); the solver and
everything witness-shaped (M10); units display beyond canonical
values; error-UX breadth; the undo history tree UI.

**Undo note.** The tree is structurally cheap here — parent
pointers over `Doc` values the linear stack already retains — so
v1's linear undo is held as the degenerate walk of a tree-shaped
state: an edit after undo mints a sibling rather than truncating,
and v1 chrome exposes only undo/redo along the current branch.
Nothing is destroyed; the branch picker and the history sidecar
(GUI-DESIGN's state/history separation note) are the future work.
Save writes the current path's linear log — persistence is
untouched.

## Platform — RULED: native first, browser as a build lane

The web option stays real and measured (GQ6-RESURVEY §4: kernel +
`editor-core` check clean on `wasm32-unknown-unknown`, interval
lane included, CI-guarded since #807; egui runs on wasm). What
keeps it off the v1 acceptance path: `cargo check` is not codegen,
linking, or WebGPU first-light; Firefox-on-Linux WebGPU is still
in progress; the `pncad` `getrandom` `wasm_js` row is an unguarded
one-time reading; and persistence needs a download/upload or OPFS
story the native lane doesn't.

**The browser threading answer, named so it is not overread as a
blocker:** what v1 needs is only that evaluation never freeze the
UI. The intended wasm shape is **one dedicated Web Worker running
the evaluation seam single-threaded**, message-passing results —
no SharedArrayBuffer, no cross-origin-isolation headers, no
toolchain change. Rayon-on-wasm (`wasm-bindgen-rayon`) is needed
only if *parallel* evaluation in the browser is ever wanted; its
real cost is that wasm atomics still require a nightly
`-Zbuild-std`, colliding with the pinned stable toolchain — so it
stays unbuilt until single-worker evaluation measures too slow.

Constraint carried through every unit so the web lane stays cheap:
the interaction layer never assumes threads — evaluation sits
behind a seam that runs on a background thread natively and in a
Worker (or inline) on wasm — and no unit takes a dependency that
fails the existing wasm guard.

## The units

Ordered; 1 is independent of 0 and may run concurrently.

0. **GUI-0 — the scaffold spike** (RESURVEY §5's named increment):
   the `viewer` crate (the name DESIGN.md's crate table reserves) —
   eframe app, docked chrome (`egui_tiles` or `egui_dock`; pick at
   implementation), thin custom wgpu viewport drawing one
   tessellated body at display-δ, orbit/pan/zoom camera as typed
   layer-3 operations (the G1 operations-are-API rule). Delivers
   G3 pan/rotate/zoom and the one measurement no survey could
   take: the friction of holding an authoritative `Doc` under an
   immediate-mode loop — the egui→iced fallback conditions get
   their first data here.
1. **GUI-1 — `Bvh::ray` + the hit-test service** (headless, no
   rendering dependency): a ray-slab test and traversal on the
   existing deterministic BVH (RESURVEY §3: extend our own, not
   parry3d), then the editor-core service `ray → stable ref` on
   the mesh back-references and the shipped
   arena-key→stable-name inversion. CI-tested without pixels.
2. **GUI-2 — selection in the viewport**: GPU ID-buffer pass for
   hover/click exactness (RESURVEY §3), consuming GUI-1's service
   for the ray path; click-to-select bound to a selection value of
   stable refs (single-select per the ruling; the G1 rule — no
   arena key crosses into layer 3); selection highlight. Tools
   survive a selected ref vanishing (the ratified
   resolution-failure semantics).
3. **GUI-3 — the document panels**: feature tree over the GQ2
   result DAG; property panel applying `DocEdit`s through `apply`
   with memoized incremental re-evaluation and cancelation
   (busy + cancel per the ruling); the expression-driven-dimension
   refusal affordance (ratified micro-decision); linear undo per
   the undo note; **open/save with an open dialog over demo
   documents** (acceptance below).
4. **GUI-4 — assembly interaction**: per-instance hide and
   free-move transforms for completely-unconstrained instances,
   layer-3 state only (never persisted, per G3), rendered
   visually distinct from mated placement; **the mate tool** —
   two picks held in tool state (`select::face_frame` already
   derives a frame from a face pick), a class/alignment choice
   from the ASM vocabulary, one committed `DocEdit` adding the
   mate node; the instance's free-move transform is superseded by
   the solved placement when the mate lands.
5. **GUI-5 — the web lane** (stretch, separable): wasm build of
   `viewer`, the evaluation Worker, the `getrandom` cfg made a
   guarded lane, open/save via download/upload. Skipping it costs
   v1 nothing.

The G1 boundary rules bind every unit: operations are API, tools
are `handle(event, ui_state) → (ui_state′, edits, overlay)`, CI
replays synthetic event streams asserting on emitted edits; only
pixel-painting escapes.

## Acceptance — RULED shape: open the app, pick a demo piece

The exit demo opens the application and loads any of the existing
demo *documents* through a file-open dialog.
`pncad::document::save` exists and the tour's assembly demo
already writes `assembly/*.pncad`; the document-authored tour
scenes today are **assembly, checks, diefillet, heatsink,
scalar** — the assembly exercises every G3 item plus the mate
tool. A small exporter mode in the tour (each document-authored
scene saves its `.pncad`) supplies the gallery directory. The
remaining tour scenes drive the kernel API directly and are
openable only once re-authored as documents — per-scene
LIB-flavored work (the die_tool re-authoring is already banked
there), incremental and independent of every GUI unit; the
gallery grows as they land.

## Sizing

Six PR-sized units (GUI-4 grew the mate tool — call it six and a
half), of which two (GUI-1, GUI-3's wiring) are ordinary headless
Rust over shipped substrate, and one (GUI-5) is optional.
DESIGN.md's "a GUI is a second project of comparable size to the
kernel" scoped the FULL GUI — sketcher, live editing, DOF
diagnosis, error-UX breadth; G3 excludes exactly those, and
`editor-core` already ships the document/edit/eval/naming layer
that would otherwise dominate. **Estimate: one mid-sized
milestone (M9-scale), not a program.** The risks that could grow
it, named: the immediate-mode seam (measured at GUI-0, with a
ratified fallback), wgpu plumbing depth in GUI-0/2 (the one
genuinely new craft in the codebase), and egui's release churn
landing on the toolchain pin (RESURVEY §5 watches it).

## Remaining open questions

- **OQ-a — mate pick mechanics**: tool-held sequential picks
  (proposed — the selection model stays single-select) vs an
  ordered two-slot selection. Both emit the same mate edit;
  implementation-time choice, recorded here so it is chosen
  consciously.
- **OQ-b — docking crate**: `egui_tiles` vs `egui_dock`, chosen
  inside GUI-0.
