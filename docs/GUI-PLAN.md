# GUI v1 — the plan

**STATUS: RATIFIED (Evan's in-conversation sign-off on this
draft, 2026-08-27, with all rulings below folded) and DELIVERED —
units GUI-0…GUI-4 merged as PRs #1094, #1093, #1101, #1106,
#1113, the program CLOSED 2026-08-28 on the ratified exit walk
`docs/GUI-EXIT-WALK.md` (#1121).** Every
*decision* this plan leans on is ratified
elsewhere and is cited, not re-litigated: the three-layer split and
boundary rules (`docs/GUI-DESIGN.md` G1, including the
operations-are-API rule recorded 2026-08-27), the v1 minimum (G3),
the toolkit (GQ6: **egui**, iced the named fallback), and the
viewport/picking recommendations (`docs/GQ6-RESURVEY.md` §§2–3).
Sequencing stance: DESIGN.md places GUI after usable-as-library;
this program ran on Evan's call per LQ5, with LIB's dispatchable
residue (the Python assembly series) blocking nothing here.

## Rulings (Evan, 2026-08-27, in-conversation)

- **Platform: native before browser.** The web lane is GUI-5,
  separable, never on the v1 acceptance path.
- **The web lane ships threaded, not single-threaded** (ruled in
  the round-2 conversation): a pinned-nightly wasm build with
  `wasm-bindgen-rayon`, per the platform section below; the
  single-Worker lane is the named fallback, not the plan.
- **Selection: single-select for v1.** Mate authoring holds its
  two sequential picks in tool state (GUI-4) — ruled, closing the
  round-2 OQ-a.
- **Undo: linear for v1.** The history graph + sidecar is banked
  as post-v1 unit GUI-6, Evan-sized at one-to-two units; the
  visualization sketch is recorded in GUI-DESIGN's undo-tree
  section.
- **Units: canonical meters/radians in the panels.** U8's
  units/display layer is not a dependency. *(SUPERSEDED post-close,
  2026-08-29, Evan-requested. BOTH clauses: the panels now show and
  author each value in the display unit its literal remembers, and
  `crates/viewer/src/props.rs` imports `pncad::quantity` — the
  units layer IS a panel dependency now. A `SetSlotUnit` op changes
  the notation without touching the value. What the ruling still
  buys, and what is unchanged: everything crossing the
  panel/session boundary is canonical metres and radians, so the
  units layer is a dependency of the RENDERING and of nothing
  below it.)*
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

**The browser threading posture — RULED: the web lane ships
threaded.** Concretely: the wasm artifact is built on a *pinned
nightly* with `-Zbuild-std` and
`+atomics,+bulk-memory,+mutable-globals`, runs its rayon pool as
Web Workers via `wasm-bindgen-rayon`, and is served with the
cross-origin-isolation headers (`COOP: same-origin` +
`COEP: require-corp`) that unlock `SharedArrayBuffer`. Costs,
named: a second toolchain pin (pinned like the stable one; the
*source* stays stable-compatible — build-std is a build flag,
never a nightly feature in code), the browser's
never-block-the-main-thread discipline on how joins are driven,
and header control wherever the app is served (we control our
serving; static hosts have a service-worker workaround). What
this does **not** put at risk is D9 determinism: the value paths
are libm-only pure Rust and wasm's software `mul_add` is
correctly rounded, so numeric results are independent of the
compiler lane — the nightly costs churn, not bit-identity. The
**single-Worker single-threaded lane is the named fallback** if
the nightly lane proves brittle (build-std breakage,
`wasm-bindgen-rayon` maintenance): same evaluation seam, no
source change.

Constraint carried through every unit so the web lane stays cheap:
the interaction layer never assumes threads — evaluation sits
behind a seam that runs on a background thread natively and in a
Worker (or inline) on wasm — and no unit takes a dependency that
fails the existing wasm guard.

## The units

Ordered; 1 is independent of 0 and may run concurrently. Units
GUI-0…GUI-4 are merged and the entries below are the delivered
scope; GUI-5 is DEFERRED post-v1 beside GUI-6 (Evan's ruling,
2026-08-28, at the close of `docs/GUI-LOG.md`).

0. **GUI-0 — the scaffold spike** (RESURVEY §5's named increment):
   the `viewer` crate (DESIGN.md's crate table row) — eframe app,
   docked chrome (`egui_tiles`, decided here — see below), thin
   custom wgpu viewport drawing one tessellated body at display-δ,
   orbit/pan/zoom camera as typed layer-3 operations (the G1
   operations-are-API rule). Delivers G3 pan/rotate/zoom and the
   one measurement no survey could take: the friction of holding
   an authoritative `Doc` under an immediate-mode loop — the
   egui→iced fallback conditions took their first data here, and
   none of them were met.
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
   two sequential picks held in tool state (RULED;
   `select::face_frame` already derives a frame from a face
   pick), a class/alignment choice from the ASM vocabulary, one
   committed `DocEdit` adding the mate node; the instance's
   free-move transform is superseded by the solved placement when
   the mate lands.
5. **GUI-5 — the web lane** (DEFERRED post-v1, separable): wasm
   build of `viewer` on the threaded posture above (the pinned-nightly
   build lane, guarded in CI once it exists), the `getrandom` cfg
   made a guarded lane, the cross-origin-isolation serving story,
   open/save via download/upload. Skipping it costs v1 nothing.

Banked past v1 (not a v1 unit): **GUI-6 — the history graph +
sidecar** — the undo tree's branch-graph UI (sketch recorded in
GUI-DESIGN's undo-tree section) and the separable history sidecar
file per the state/history separation note. Evan-sized at
one-to-two units; v1's tree-shaped undo state (the undo note
above) is what makes it additive.

The G1 boundary rules bind every unit: operations are API, tools
are `handle(event, ui_state) → (ui_state′, edits, overlay)`, CI
replays synthetic event streams asserting on emitted edits; only
pixel-painting escapes.

## Acceptance — RULED shape: open the app, pick a demo piece

The exit demo opens the application and loads any of the existing
demo *documents* through a file-open dialog.
`pncad::document::save` exists and the tour's assembly demo
already writes `assembly/*.pncad`; the document-authored tour
scenes today are **assembly, checks, ring, diefillet,
heatsink** (corrected 2026-08-27 at GUI-3: `scalar` is the trait
module, not a scene; `ring` is the fifth) — the assembly
exercises every G3 item plus the mate
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
that would otherwise dominate. **Scope: one mid-sized milestone
(M9-scale), not a program.** The three risks that could have grown
it — the immediate-mode seam, wgpu plumbing depth in GUI-0/2 (the
one genuinely new craft in the codebase), and egui's release churn
landing on the toolchain pin — are walked closed in
`docs/GUI-EXIT-WALK.md`; the egui-churn watch (RESURVEY §5)
stands.

## The docking crate, settled in GUI-0

- **OQ-b — docking crate: `egui_tiles`**, over `egui_dock` — the
  two live ecosystem crates for movable/tabbed panel chrome
  (feature tree / viewport / property panel). Licensing decided
  it first: `egui_tiles` is MIT OR Apache-2.0 and keeps both
  branches of this project's dual license alive, where `egui_dock`
  is MIT-only. Shape decided it second: the layout is a
  `Tree<Pane>` VALUE the app owns and a `Behavior` impl renders —
  the same state-is-a-value discipline G1 puts on everything else
  here. The pin and the full argument live in
  `crates/viewer/Cargo.toml`.
