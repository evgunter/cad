# GUI-1 — the v1 GUI (plan, DRAFT)

**STATUS: DRAFT — a proposal for Evan's pushback, ratifying
nothing.** Every *decision* this plan leans on is already ratified
elsewhere and is cited, not re-litigated: the three-layer split and
boundary rules (`docs/GUI-DESIGN.md` G1), the v1 minimum (G3), the
toolkit (GQ6: **egui**, iced the named fallback), and the
viewport/picking recommendations (`docs/GQ6-RESURVEY.md` §§2–3).
What this plan adds is a unit ladder, a platform posture (OQ1 — the
one genuinely new call), and the open questions the ratifying
conversation must close. Sequencing stance: DESIGN.md places GUI
after usable-as-library; starting this program is Evan's call per
LQ5, and LIB's dispatchable residue (the Python assembly series)
does not block any unit here.

## Scope — G3, verbatim, plus its implied substrate

The v1 minimum is G3's four capabilities and nothing more:

1. **click-to-select** parts/features, feeding the *existing* edit
   doors — no in-viewport manipulation;
2. **pan / rotate / zoom**;
3. **free-move of completely-unconstrained assembly parts** — a
   display transform for fit-probing before a mate exists, no
   solver involvement, visually distinct from mated placement (the
   G3 honesty requirement);
4. **hiding parts** in an assembly.

Implied substrate, without which the four are not demonstrable: a
viewport rendering the evaluated bodies of an open document; a
feature tree showing the GQ2 per-node result DAG (failed/poisoned
badges — typed values rendered minimally, per the ratified
case-by-case error-presentation stance); a property panel that
applies `DocEdit`s (set parameter / expression, suppress, Rebind)
through pure `apply` with incremental re-evaluation; open/save on
the shipped snapshot+log persistence.

**Exclusions, stated so nobody reads more:** live dragging/editing
of constrained geometry (G3 excludes it by ratification); the
sketcher (G2's nested editor is a later milestone); the solver and
everything witness-shaped (unbuilt as sequenced, M10); units
display beyond canonical values unless LIBRARY-DESIGN U8 lands
first (OQ4); error-UX breadth; the undo history TREE (the banked
concept stays banked — v1 gets at most a linear stack, OQ3).

## Platform posture (OQ1 — proposed, needs ratification)

**Proposed: dual-target from day one via eframe — native is the
development and acceptance lane for every unit; the browser is a
build lane added as its own unit (GUI-5), not the v1 acceptance
platform.**

The web option is real and measured (GQ6-RESURVEY §4: kernel +
`editor-core` check clean on `wasm32-unknown-unknown`, interval
lane included, CI-guarded since #807) — and egui runs on wasm, so
targeting the browser re-ranks nothing in the toolkit ruling. But
"check clean" is the whole of what is established, and the browser
lane carries four costs the native lane does not, none retired:

- **Threads.** The evaluation service's rayon idioms have no wasm
  equivalent without cross-origin isolation plus
  `wasm-bindgen-rayon`; a single-threaded lane is unmeasured, and
  `editor-core` ships no progress reporting or in-op yield points —
  so a long evaluation on the browser main thread freezes the UI
  with no cooperative out. Natively, evaluation moves to a worker
  thread and the problem does not exist.
- **`cargo check` is not codegen, linking, or running.** The guard
  establishes compilation; first-light on WebGPU is unproven, and
  Firefox-on-Linux WebGPU is still in progress.
- **The `pncad` row is unguarded.** The `getrandom` `wasm_js` cfg
  reading is one-time (RESURVEY §4 row 3); a dependency bump can
  falsify it with every check green.
- **Persistence.** Snapshot+log wants a filesystem; the browser
  needs a download/upload or OPFS story — UI work with no native
  analog.

Constraint carried through every unit so the web lane stays cheap:
the interaction layer never assumes threads (evaluation sits behind
a seam that can run on a worker thread natively, in a Web Worker or
single-threaded on wasm), and no unit takes a dependency that fails
the existing wasm guard.

## The units

Ordered; 1 is independent of 0 and may run concurrently.

0. **GUI-0 — the scaffold spike** (RESURVEY §5's named increment):
   the `viewer` crate (the name DESIGN.md's crate table reserves) —
   eframe app, docked chrome (`egui_tiles` or `egui_dock`; pick at
   implementation), thin custom wgpu viewport drawing one
   tessellated body at display-δ, orbit/pan/zoom camera. Delivers
   G3 item 2 and the one measurement no survey could take: the
   friction of holding an authoritative `Doc` under an
   immediate-mode loop — the egui→iced fallback conditions get
   their first data here.
1. **GUI-1 — `Bvh::ray` + the hit-test service** (headless, no
   rendering dependency): a ray-slab test and traversal on the
   existing deterministic BVH (RESURVEY §3: extend our own, not
   parry3d), then the editor-core service `ray → stable ref` on the
   mesh back-references and the shipped arena-key→stable-name
   inversion. CI-tested entirely without pixels.
2. **GUI-2 — selection in the viewport**: GPU ID-buffer pass for
   hover/click exactness (RESURVEY §3) consuming GUI-1's service
   for the ray path; click-to-select bound to a selection set of
   stable refs (the G1 rule: no arena key crosses into layer 3);
   selection highlight rendering. Minimal GQ7 cut only (OQ2);
   tools survive a selected ref vanishing (the ratified
   resolution-failure semantics). Delivers G3 item 1's selection
   half.
3. **GUI-3 — the document panels**: feature tree over the GQ2
   result DAG (Ok/Failed/Poisoned badges; failure messages from
   the typed payloads); property panel applying `DocEdit`s through
   `apply` with memoized incremental re-evaluation and cancelation;
   the expression-driven-dimension refusal affordance (ratified
   micro-decision); linear undo (`Doc` values — keep the old one);
   open/save through persistence. Completes G3 item 1.
4. **GUI-4 — assembly display state**: per-instance hide and
   free-move transforms for completely-unconstrained instances,
   layer-3 state only (never persisted, per G3's kernel-relevant
   consequences), rendered visually distinct from mated placement.
   Delivers G3 items 3–4.
5. **GUI-5 — the web lane** (stretch, separable): wasm build of
   `viewer`, the evaluation seam's browser answer (worker or
   single-threaded), the `getrandom` cfg made a guarded lane,
   open/save via download/upload. Rides entirely on the constraint
   carried above; skipping it costs v1 nothing.

Layer-3 testability (G1) binds every unit: tools are
`handle(event, ui_state) → (ui_state′, edits, overlay)` and CI
replays synthetic event streams asserting on emitted edits; only
pixel-painting escapes.

## Sizing

Six PR-sized units, of which two (GUI-1, and GUI-3's panel wiring)
are ordinary headless Rust over shipped substrate, and one (GUI-5)
is optional. DESIGN.md's "a GUI is a second project of comparable
size to the kernel" scoped the FULL GUI — sketcher, live editing,
DOF diagnosis, error-UX breadth; G3 excludes exactly those, and
`editor-core` already ships the document/edit/eval/naming layer
that would otherwise dominate. **Estimate: one mid-sized milestone
(M9-scale), not a program.** The risks that could grow it, named:
the immediate-mode seam (measured at GUI-0, with a ratified
fallback), wgpu plumbing depth in GUI-0/2 (the one genuinely new
craft in the codebase), and egui's release churn landing on the
toolchain pin (RESURVEY §5 watches it).

## Open questions for the ratifying conversation

- **OQ1 — platform posture**: ratify native-first with the web
  lane as GUI-5, or promote the browser to the v1 acceptance
  platform (accepting its four costs on the critical path).
- **OQ2 — GQ7 minimal cut**: single-select only for v1, or
  multi-select with modifiers (the edit doors that motivate
  selection mostly take one ref; suppress/hide want sets).
- **OQ3 — undo scope**: linear stack (proposed — nearly free) vs
  none; the history tree stays a banked concept either way.
- **OQ4 — units display**: canonical meters/radians in the
  property panel (honest, ugly) vs pulling U8's units/display
  layer forward as a dependency.
- **OQ5 — long evaluations**: is busy-indicator + cancel (shipped
  `CancelToken`) acceptable for v1, or does progress reporting
  (absent, with in-op yield points) get built first?
- **OQ6 — acceptance shape**: what document does the exit demo
  open? (The tour corpus is the standing oracle; the lily and the
  ASM-DEMO assembly are the natural candidates — the assembly
  exercises all four G3 items.)
