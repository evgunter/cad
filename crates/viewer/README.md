# Running the viewer

```sh
cargo run -p viewer --features app -- [document.pncad]
```

The optional path is opened at startup through the same typed `Open`
operation the dialog feeds — and it is the only way to open a document
on a system with no file-chooser backend (below).

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
instance is not a viewport drag either: the free-move probe is the typed
x/y/z fields in the instance section of the Properties panel
(display-only, mm).

In the Properties panel, the document-parameters list ends with an
add-parameter row (name + dimension + value, one undoable edit); an
expression that names an undeclared parameter refuses typed, and the
refusal offers to create it — prefilled into that row, with the
dimension left as your pick.

## Troubleshooting

**Open…/Save As… need a file-chooser backend.** The dialogs go through
`rfd`, whose Linux backends are the XDG desktop portal
(`xdg-desktop-portal` plus a frontend for your desktop) or a `zenity`
binary. WSL distros ship neither. The app probes at startup: with
confidently none (no `zenity` on PATH, no D-Bus session bus) the
Open/Save As buttons are disabled with the reason as their tooltip.
`apt install zenity` (or a portal) fixes it, and the command-line
document argument needs no dialog at all. The probe's portal arm is only
a hint: a session bus *without* a working portal frontend still makes an
attempted dialog return the same silent nothing a cancel does (`rfd`
cannot tell the two apart), so a plausibly-present backend that never
shows a dialog reads as quiet cancels — install `zenity`.

**Dialog opens but every character is a box with tiny hex digits.**
Pango cannot shape the font fontconfig matched. Diagnose with
`fc-match sans` — if it names a `.pfb` (a PostScript Type 1 font, e.g.
Nimbus Sans L from the legacy URW set), Pango dropped Type 1 support and
renders hex boxes with no warning. On aged WSL images the usual root
cause is that the DejaVu files are MISSING ON DISK while dpkg still
claims `fonts-dejavu-core` is installed (plain `apt install` refuses with
"already installed"):

```sh
sudo apt install --reinstall fonts-dejavu-core && fc-cache -f
```

then verify `fc-match sans` answers DejaVu Sans. A stale
`~/.cache/fontconfig` can also pin an old match —
`rm -rf ~/.cache/fontconfig && fc-cache -f` is the no-sudo variant to try
first. The viewer itself is unaffected; egui bundles its own fonts, and
only the GTK/zenity dialog needs the system set.

**Window won't resize horizontally under WSLg.** WSLg presents Wayland
windows through an RDP RAIL shell whose client-side-decoration
negotiation breaks horizontal resizing. The viewer detects WSL
(`WSL_DISTRO_NAME`/`WSL_INTEROP`) and prefers the X11/XWayland backend
automatically; non-WSL environments are untouched. On a build without
that preference the manual equivalent is:

```sh
WAYLAND_DISPLAY= cargo run -p viewer --features app
```

## Driving it with no display (headless)

The whole application runs on a virtual X server with a software Vulkan
rasteriser — window, dialog, GPU picking and all. `tests/` is where the
interaction layer belongs and this is not a substitute for it; what only
this can do is **look at what the app actually draws** and **run two
builds of the app in one environment**.

```sh
apt-get install -y libxkbcommon-x11-0 mesa-vulkan-drivers libegl1 \
                   libgl1-mesa-dri zenity xdotool imagemagick xvfb

Xvfb :99 -screen 0 1400x900x24 &
export DISPLAY=:99
cargo run --release -p viewer --features app -- document.pncad
import -window root shot.png            # -crop WxH+X+Y +repage to trim
xdotool mousemove 95 11 click 1         # read coordinates off a screenshot
```

Each package earns its line. Without `libxkbcommon-x11-0` winit panics
before the window exists (`Library libxkbcommon-x11.so could not be
loaded`). `mesa-vulkan-drivers` supplies lavapipe, the software ICD wgpu
lands on — **`WGPU_BACKEND=gl` is a dead end**, refusing with
`CreateSurfaceError(Hal(FailedToCreateSurfaceForAnyBackend({})))`, while
Vulkan needs no environment variable once the ICD is installed. Without
`zenity` the chooser probe reports `Absent` and disables Open…/Save As…,
so everything but the dialog still works. The `libEGL warning: DRI3
error` lines on stderr are noise.

**Build `--release`.** In a debug build tessellation and BVH index build
take minutes, which is indistinguishable from a hang.

Interaction notes:

- **`pkill -f 'target/release/viewer'` kills the invoking shell**, whose
  own command line contains that string. Use `pkill -x viewer`.
- The file dialog is a separate window and is slow to appear; a fixed
  `sleep` before typing races it. Poll for the window instead.
- In the dialog, `ctrl+l` then a path NAVIGATES, and Return on a
  directory navigates rather than confirming — click the file row, then
  OK.
- While the dialog is up the app is genuinely frozen: `rfd`'s blocking
  call stalls the frame loop mid-`ui()`. Expected, not a bug.
- An apparent hang is worth a `gdb -p <pid> -batch -ex "thread apply all
  bt"` early. `top` reporting 0% CPU is an artifact of a single-sample
  read and is not evidence of blocking.

To TIME an interaction reproducibly, hash a crop of the region that will
change and poll it, jiggling the pointer to force a repaint:

```sh
BASE=$(import -window root -crop 300x40+845+45 png:- | md5sum)
```

**What it is not good for.** Rendering is on the CPU, so wall-clock
readings mean something for the CPU-bound work (evaluation,
tessellation, index build) and mislead for anything GPU-bound. And it is
not CI: the packages are an install, and the timings are noisy.

## Running it in a browser (spike — compile/link only)

```sh
cargo install wasm-bindgen-cli --version "$(awk '/^name = "wasm-bindgen"$/ { getline; gsub(/[",]/, ""); print $3; exit }' Cargo.lock)"
rustup target add wasm32-unknown-unknown
local-scripts/serve-wasm.sh          # prints the URL to open on the phone
```

The single-threaded browser lane — the *named fallback* to the
threaded web lane (GUI-5, banked). Nothing here needs a nightly
toolchain, `-Zbuild-std`, `wasm-bindgen-rayon`, or cross-origin
isolation; the pinned stable toolchain builds it as it stands.

**It has been built and linked, and nothing beyond that is verified** —
no first light on real hardware. It is also not CI-guarded: the wasm32
step excludes `viewer` (Toolkit and CI posture, below), so a dependency bump
can break this build with every check green.

Three things are known-absent by design:

- **No file I/O.** The browser build links no `rfd`, so Open…/Save As…
  are disabled with their reason showing — the same posture a Linux box
  with no portal and no zenity gets. It opens on the built-in startup
  document and stays there. Document I/O in a browser needs the
  download/upload or OPFS story GUI-5 owns.
- **No touch bindings.** `InputMap` binds orbit to a middle drag, pan to
  a secondary drag, and zoom to a wheel; a phone has none of the three.
  egui delivers the touch events and `Context::multi_touch` is there,
  but nothing consumes them, so navigation is unusable on a phone until
  a touch vocabulary lands. `InputMap::map` is a pure `ViewportEvent →
  CameraOp` function, so that work is headless-testable.
- **No phone layout.** `initial_layout` splits viewport-beside-panels
  horizontally at a 1280×800 design size. On a ~390 px viewport the
  four-pane dock is unusable; the panes scroll, which is not the same as
  fitting.

**Evaluation runs on the main thread.** The seam takes
`evalseam::InlineEvaluator` here instead of `ThreadEvaluator`, so a
rebuild blocks the frame that submitted it and the tab stops painting
until the kernel returns. The busy indicator cannot help — that needs an
in-op yield point, which v1 rules absent (GQ2, below).

**No WebGPU over plain http.** WebGPU requires a secure context and
`http://<lan-ip>` is not one, so `navigator.gpu` is absent whatever the
browser version and wgpu falls back to WebGL2. That works only because
the `egui-wgpu` edge takes default features, which include
`wgpu/webgl` — `default-features = false` on that edge would leave the
phone with no adapter. The page prints `secureContext` / `navigator.gpu`
/ `webgl2` in its error box so this diagnoses itself rather than
presenting as a blank screen.

### What serving it exposes

`serve-wasm.sh` runs an **unauthenticated** static server bound to
`0.0.0.0`, so anyone on the same network can fetch the build for as long
as it runs. The directory it serves holds only the wasm-bindgen output
files — but a `--release` build of this workspace keeps
`debug-assertions` on (the root `Cargo.toml` says why), so the binary
carries assertion strings and local source paths. Treat it as handing
the LAN a copy of an unreleased kernel: fine on a home network, not on
café or conference Wi-Fi. It is a foreground process; Ctrl-C ends the
exposure.

The WSL port-forward the script prints is the sharper edge, because both
halves **outlive the script**: a `netsh portproxy` entry survives
reboots, and a firewall rule with no `-Profile` re-opens the port on
every network the machine later joins. The printed commands scope the
rule to `Private` and are followed by the two lines that undo them; run
those when the demo is over.

# Architecture

This is the GUI/editor architecture (the GUI-DESIGN clauses G1–G5 and
the GQ answers) and the toolkit decision, stated as they stand. The
viewer is layer 3; `docs/DESIGN.md` D1–D9 bind everything below it and
are never overridden here.

## Where in the code

| Decision | Modules |
|---|---|
| G1 layer 2 (document as a value, `DocEdit` + pure `apply`, evaluation service, hit-testing) | `crates/editor-core` (`crates/editor-core/README.md`) |
| G1 layer 3 values and operations | `src/camera.rs` (`Camera`, `CameraOp`, `camera::apply`), `src/session.rs` (`DocSession`, `DocSession::perform`, the operation doors) and its vocabularies `session::{select, refuse, op, author, delete, probe}` (Module boundaries, below), `src/history.rs` (tree-shaped undo), `src/input.rs` (`ViewportEvent`), `src/tools.rs` and the per-tool modules |
| G3 free-move and hiding as display state | `src/display.rs` |
| G3 mate definition | `src/matetool.rs` |
| Feature tree, property panel, open/save, evaluation seam, scene | `src/tree.rs`, `src/props.rs`, `src/docio.rs`, `src/evalseam.rs`, `src/scene.rs` |
| Colour, themes, preferences | `src/theme.rs`, `src/prefs.rs`, `tests/theme.rs` |
| GQ7 picking | `src/pick.rs` (`EDGE_PICK_RADIUS_PX`, `PickKinds`), `crates/bvh` (`Bvh::ray`) |
| GQ6 toolkit, viewport, docking | `src/app.rs` (the frame loop and `ViewerApp`) with `src/pane/*` (the pane bodies) and `src/widgets.rs`, `src/gpu.rs`, `src/frame.rs` behind the `app` feature; `Cargo.toml`. The authoring vocabularies the panels offer are `src/forms.rs` and `src/drafts.rs`, which name no toolkit type |

## Module boundaries

Two files in this crate hold most of it — the session's state machine
and the toolkit adaptation — and both accreted one titled section per
unit until neither could be read whole. The boundary below is the rule
that keeps them readable; it is a rule rather than a map because a map
is out of date after the next unit and a rule is not.

**Every module in this crate is a VOCABULARY or a DRIVER, and its
`use` block says which.**

- A **vocabulary** module holds values, their wording, and pure
  functions over them. It names no driver type and no toolkit type: no
  `DocSession`, no `ViewerApp`, no `egui`. It can be read, and tested,
  without a session or a window existing.
- A **driver** owns mutable state and dispatches. There are exactly
  two: `session` (owns `DocSession`, dispatches `SessionOp`) and `app`
  (owns `ViewerApp`, drives the frame). A driver may name any
  vocabulary; no vocabulary may name a driver.

The rule is mechanically checkable — read the `use` block — which is
the property that makes it survive contact with the next unit. It is
also what was already true of the good modules here (`camera`,
`frame`, `input`, `display`) and false of the two that grew: both
files are one driver plus a pile of vocabulary that never left.

### The session's vocabularies

| Module | Holds |
|---|---|
| `session::select` | `Selection`, `FaceSelection`, `EdgeSelection`, `Hovered`, `Standing` — what is selected and whether it still denotes anything |
| `session::refuse` | `Refusal` with its `rank`/`preferred` ladder, its `Display`, and the recourse composers `affordance`/`exists_wording`/`offer_wording`; `NodeKindWanted` and `admits`, since they are a `Refusal` payload and its predicate |
| `session::op` | `SessionOp` and `OpOutcome` — already the crate's shared vocabulary, read by `tools`, `pick`, `frame`, `blend`, `combine`, `matetool`, `revolvetool` |
| `session::author` | `DatumSpec`, `PatternRuleSpec`, `datum_node`, the `ProfileShape` re-export — the authoring specs and their lowering to nodes, which hold no session state at all |
| `session::delete` | `DeleteAffordance` and `kind_census` — the cascade's wording |
| `session::probe` | `BoundsTarget`, `BoundsReading` and the range probe |

`session` itself keeps `DocSession`, its `Gesture`, `Landing`,
`AtRestBadge`, `perform` and the operation doors. Those are the driver
and cannot leave it: every door returns a `Refusal` and mutates the
session, and `perform`'s dispatch is the one place an operation becomes
state.

### The app's vocabularies

| Module | Holds |
|---|---|
| `forms` | What the panels offer for authoring, and how a typed field behaves. The vocabularies — `PathVerb`, `ArcMode`, `DatumKind`, `ShapeKind`, `PatternKindChoice`, `BOOLEAN_OPS`, `MATE_PRIMITIVES` — are hand-maintained mirrors of a kernel or sketch enum; the field-writing family — `FieldWriting`, `drag_tick` and the four drag speeds — mirrors nothing and is a product decision on its own (how much of a unit one pixel of drag is worth). Both are decisions the toolkit does not make, which is what puts them here rather than in `app` |
| `drafts` | `Drafts` and `CommitFault`: the in-flight form state, its defaults, and its lowering of typed field values to `Expr` and `LoopProgram` — the same layer as `session::author`, and today the larger half of it |

### The app driver, split for size

`app` is a driver, and a driver too large to read is still a driver.
`app.rs` keeps `ViewerApp`, `ViewerBehavior`, the frame loop,
`perform_batch`, `sync_scene`, `apply_status`, `Pane`,
`initial_layout` and the entry points; `pane::{viewport, features,
properties, create, view}` hold the `*_ui` functions that draw each
pane, one module per pane; and `widgets` holds the free helpers over
`egui::Ui` that those panes share.

**Splitting a driver across modules does not make the pieces
vocabularies.** The test is a module's ROLE, not its size or its file:
each of these names `egui`, and `widgets::delete_button` takes a
`&DocSession` because the wording it draws is the session's own
answer. That is the driver side of the rule behaving normally. Reading
the `use` block still decides it — the check says what a module IS,
not merely whether it is a vocabulary.

`app.rs`'s header claim — *toolkit adaptation, and nothing else* — is
true of the file rather than a claim it has outgrown.

Three items move out of `app` to modules that already own their
subject rather than to new ones: `datum_view` to `datums`, and
`tip_mark` with `heading` to `sketch` — all three are geometry over
values the receiving module already defines, and none names `egui`.

### `Refusal`'s delegation discipline

`Refusal` has two kinds of arm and the rule is where the failure's
*owner* is:

- **A delegating arm** (`Edit`, `Dimension`, `Parse`, `Io`, `Display`,
  `SlotUnit`, `Workspace`) exists where a module below layer 3 already
  owns the failure and its wording. Layer 3 adds nothing but the
  ranking, so it stores the payload and forwards the text.
- **A flat arm** exists where layer 3 is the only place the fact
  exists: there is no gesture in flight, this instance is itself, this
  name is already declared and CREATE is not REPLACE, the seat wanted a
  different node kind.

Each of those examples names a fact `apply` has been read for and does
not hold — `edit.rs` has no self-instance arm, `write_doc_param` has no
existence check because `DocEdit::SetDocParam` is create-or-replace,
and `DocEdit::InsertNode` checks a seat's input for EXISTENCE and not
for KIND. That reading is what puts an arm in this list; a fact that
merely feels like layer 3's is how the list acquires a member the door
already refuses.

`rank` stays a separate axis, and it is exhaustive over `Refusal`'s own
arms, so a new arm is compiler-caught. It is not exhaustive one level
down: `Display(_)` is a catch-all beneath its two named cases, so a new
`DisplayFault` variant is ranked by default rather than by decision.
Both costs — an arm ranked wrongly, and a delegated fault ranked by
default — are accepted, because the alternative of deriving a rank from
the arm's shape would make the ordering unstateable, and the ordering is
the part users see.

**A flat arm must not restate a refusal a door already gives.** That is
where the rule bites, and `delete_node` already states it in the code:
*an id the document does not hold takes the single-edit path so the
typed refusal comes from the door rather than from here.* Pre-checking
in layer 3 what `apply` refuses is two spellings of one rule, and the
delegating arm exists to carry the door's answer unchanged.

**A lookup is not a pre-check.** Opening a gesture on a parameter needs
its dimension, and the range probe needs its value and unit; both look
the parameter up whether or not an edit ever follows, so a flat arm is
the honest answer when the lookup fails. What separates the two cases
is whether an edit is about to be committed that would refuse on its
own.

### Gesture safety is data

The mid-gesture policy is one exhaustive value,
`SessionOp::permitted_during_value_gesture`, checked once in `perform`
before dispatch: 26 operations refuse while a value gesture is open and
13 are permitted. A fortieth operation cannot be added without
answering for it, and the whole policy is readable in one place rather
than inferred from every dispatch target.

It says nothing about the free-move gesture, which is a different value
with a different owner (`display::DisplayState`) and carries its own
in-flight refusal. The name carries that limit deliberately: both
fields are spelled `self.gesture`, and a predicate reading as a general
guarantee would be a table that looks complete and is not.

The table records behaviour rather than deciding it — `save` is
permitted mid-gesture and `open` is refused, which is what the code did
before the table existed. Whether that asymmetry is right is a separate
question with its own item.

### One open tool, not seven optional ones

`Tools` holds one `Option<OpenTool>`, an enum with one variant per tool
kind carrying that tool's state. Two tools open is not a state the door
avoids, it is a state with no spelling: the invariant is unrepresentable
rather than maintained. Each of the four per-tool rules is an arm of a
match the compiler completes: the pick routing and the survival step
match over the open value itself, the cursor narrowing
(`ToolKind::pick_kinds`) and the close-on-commit edit
(`ToolKind::commits`) over its kind. The read door
is not one of the four — each typed accessor matches its own variant
and answers `None` to every other, so a tool that never gets an
accessor compiles. The `Seated` trait and its `seated!` invocation,
which named five tool types by hand to erase them again, are gone with
the erasure they existed for.

`ToolKind::ALL` remains for the test suites that sweep the kinds, which
are now its only readers: `Tools::open_kind` asks the open value which
kind it is instead of scanning the list for the first field that is set,
and the chrome names each kind it offers literally rather than
iterating. A kind missing from `ALL` therefore narrows those sweeps
rather than making its tool permanently unreachable. `ALL` is still the
one list a compiler cannot force, and `ToolKind::ordinal` is still what
makes its completeness checkable by a row.

### What the boundary does not decide

The rule says where things live. It does not say the wording family
has one shape — recourse text is composed six ways across five modules,
and `AtRestBadge` stores a refusal it has already stringified. Naming
one shape for that family is a separate question, and the move above
neither answers nor forecloses it.

## The three layers (G1)

The split is three layers, not GUI-versus-library. The recipe is data
(D8) and so are changes to it.

1. **Kernel**: `build(params) → solid`.
2. **`editor-core`**, headless, no rendering dependency. The document
   is a value: the recipe DAG plus metadata. The edit vocabulary is a
   sum type `DocEdit` with one pure `apply : Doc × DocEdit → Result<Doc>`;
   undo is keeping the old value. Selections are values of the same
   stable-name type recipe nodes use to reference entities
   (`crates/editor-core/src/names/README.md`), so selection stability
   and reference stability are one problem. Hit-testing is an
   editor-core service, `ray → stable name`. Evaluation is memoized,
   incremental, epoch-stamped and cooperatively cancelable.
3. **Interaction**, this crate: tools fold input events into edits,
   `handle(event, ui_state) → (ui_state′, Vec<DocEdit>, overlay)`, and
   rendering is a function of the evaluated body, the selection and
   the overlays.

Boundary rules, each a type-level discipline:

- **The GUI never sees an arena key.** Only stable names cross the
  layer 2/3 boundary; the hit-test service inverts keys to names.
- **Transient gesture state never enters the document.** Rubber
  bands, in-flight drags and half-placed dimensions live in layer 3.
- **Preview versus commit is structural.** A gesture emits preview
  edits against scratch state and exactly one committed `DocEdit` on
  release: one undo step, one document transition.
- **Every operation the GUI performs is itself API.** Select, hide,
  free-move, a camera move: each is a typed operation on a state value
  (`CameraOp`, `SessionOp`), callable with no renderer present, and
  rendering is a pure view of what those operations produce. Nothing
  is expressible only as a widget interaction.
- **Layer 3 is headless-testable.** `tests/` replays event streams and
  asserts on the emitted edits; only pixel painting escapes. Pipeline
  CREATION is not pixel painting and does not escape: `src/gpu.rs`'s
  smoke row builds a device on a software adapter and constructs every
  render pipeline in the viewport, asserting nothing about a pixel.

  **What still escapes is more than pixels, and naming only the one
  exception would overstate the seat.** Buffer and texture allocation,
  render-pass encoding and the id pass's readback are all
  device-validated and none of them is pixel painting either; the
  smoke row reaches none of them, because each needs a frame rather
  than a constructor. So the honest line is that CONSTRUCTION is
  gated and everything downstream of a frame is not — which is what
  `work/chrome/viewer-first-light-on-real-hardware` still holds open.

The edit vocabulary is the one API surface shared by the GUI, the
Python bindings, macro recording and headless tests; each is a
consumer of `apply` and none knows about the others.

## Sketch editing (G2)

The sketcher is an editor-core instance one level down: its own
document (entities, constraints, solved state), its own edit
vocabulary and preview loop, and committing the sketch is one recipe
edit. A per-frame solve's payload is the entire solved assignment, and
which of constraints or assignment is authoritative is the witness
question, answered in `crates/editor-core/README.md` (W1–W9). The
sketcher is not implemented.

## What v1 is (G3)

The v1 GUI is click-to-select for editing (selection feeds the
existing edit doors), pan/rotate/zoom, free-moving completely
unconstrained instances of an assembly relative to each other
(fit-probing before a mate exists: a display transform, no solver),
hiding instances, and defining a mate between previously unmated
parts. Live dragging of partly constrained geometry is not on the
path; the witness contract stays ratified for whenever it arrives.
Hiding and free-move are display state, never persisted into the
recipe, and a free-moved placement is drawn distinguishably from a
mated one.

## Micro-decisions (G4)

- Dragging an expression-driven dimension refuses, with an affordance
  offering to edit the expression.
- Failures are typed values the GUI renders (the offending entity
  highlighted, the feature to act on marked in the tree); never
  exceptions or strings. Presentation is decided case by case. Which
  feature that is, is the payload's own answer: a row a failure merely
  reached draws POISONED and quiet, pointing at the row that carries
  the cause (`crate::tree`).
- Preview fidelity may degrade the chordal display tolerance, never ε,
  so preview cannot disagree with commit.

## Colour (G5)

A **theme** is a user preference: it supplies every semantic mark
(selection, hover, free-move probe, focus, unresolved), the default
body colour, the ambient term and the viewport **ground**
(`Theme::ground`, what fills the viewport where no geometry is drawn).
It is never written into a document, not persisted by `editor-core`,
and takes no part in any content key. A **document** overrides the
body colour: `Attr::Color` on a stable name is authored and travels
with the file, and the theme never overrides it back. Both are
`editor_core::appearance::Rgba8`, so the override is a substitution
within one colour space; linear light is entered once, at each
renderer's door. Colourblind legibility is a claim a theme makes, not
a constraint on every theme: a palette that claims its marks stay
distinguishable under dichromatic vision is held to it by simulation
in `tests/theme.rs`, measured on the composited colour, since marks
are mixed over the body colour. That bar puts `colorblind-safe` on a
light ground. Preferences live in hand-editable TOML at
`$XDG_CONFIG_HOME/pncad/viewer.toml` (`src/prefs.rs`); malformed TOML
refuses, an unknown key reports and the rest applies, an unknown
value reports and falls back, while a theme name typed on the command
line is refused rather than defaulted.

## The GUI questions

- **GQ1, the solver/replay boundary.** Solver output is demoted to a
  stored witness that selects the branch; the kernel certifies. The
  mechanism is W1–W9 in `crates/editor-core/README.md`.
- **GQ2, partial builds.** Evaluation returns a per-node result DAG; a
  failure poisons only its descendants and independent subgraphs
  complete (`editor_core::eval`, `NodeResult::{Ok, Failed, Poisoned}`).
  The tree's POISONED badge is wider than that DAG relation: a mate
  solve refuses across the placement graph, which the result DAG has
  no edges for, and those rows draw as downstream of the mate the
  fault names (`crate::tree`).
  Progress reporting and in-op yield points are absent; v1 shows a busy
  indicator over the shipped `CancelToken`.
- **GQ3, persistence.** Every `DocEdit` is persisted: the on-disk form
  is a snapshot plus an edit log, verified to replay through `apply`
  on save and replayed on load. The format carries no schema version
  before release; a file a build cannot read refuses typed with the
  regenerate recourse.
- **GQ4, document scope.** One document is one part's recipe, which
  may evaluate to several bodies; references are document-local; an
  assembly is a recipe DAG of the same formalism whose cross-document
  references are a wrapper over the local name with a content pin
  (`crates/editor-core/ASSEMBLY.md`).
- **GQ5, typed quantities.** The expression sublanguage carries typed
  quantities: `Dimension = Length | Angle | Count | Scalar`, every
  constructor dimension-checked, dimension-changing products refused,
  canonical values in metres and radians underneath, display units
  stored as presentation metadata and rendered by the panels.
- **GQ7, selection mechanics.** v1 is single-select; selection does
  not participate in document history. Pick priority is proximity in
  the picture, scoped to the body under the cursor: the ray picks a
  face first, and an edge of that face's own body within
  `EDGE_PICK_RADIUS_PX`, not hidden by the solid, beats it; elsewhere
  the face wins and off the body nothing wins. A tool narrows the
  kinds it accepts through `PickKinds`. Multi-select UX and the filter
  vocabulary wait on sketcher and tree design; filters, heterogeneous
  sets and vanishing-entity semantics are `docs/SELECT-DESIGN.md`'s.

## Toolkit and CI posture (GQ6)

**Toolkit: egui/eframe, with iced as the named fallback.** egui tracks
current wgpu, has the docking chrome a tree + viewport + property
panel needs, and has a production existence proof of this exact shape
(rerun). G1's architecture lives in `editor-core`, below any toolkit,
so the fallback costs only the interaction layer. Slint (GPL-only
OSI branch) and GPUI (unmaintained standalone) are out; bevy is
demoted. The conditions that would send v1 to iced, recorded so the
switch is a judgement and not a mood: the immediate-mode loop needing
ad-hoc frame-to-frame state to keep `Doc` authoritative; an egui MSRV
bump forcing a compiler move the bit-identity gate is not ready for;
chronic wgpu or paint-callback migration cost. None is met.

**Viewport, picking, docking.** The viewport is a thin custom wgpu
pass under eframe's wgpu renderer (`src/gpu.rs`). Picking is our own
deterministic `Bvh::ray` query, authoritative, with the GPU id-buffer
pass advisory. Docking is `egui_tiles`, a `Tree<Pane>` value the app
owns. All of it sits behind the non-default `app` feature; without it
the crate is renderer-free and headless-tested.

**Where the `app` feature gates.** The workspace nextest archive builds
this crate at DEFAULT features, so nothing behind the feature is in it.
The seat is a hosted row that runs
`cargo nextest run -p viewer --features app` in the `fmt` job, beside
the app-feature clippy row that already compiles the toolkit graph, on
the same seed-keyed `run_viewer_toolkit` axis and with a lavapipe
adapter installed for the pipeline smoke row. Archiving with
`--features app` was the alternative and was refused: the archive is
built once and downloaded by every leg of the `test` matrix, so the
toolkit graph's extra weight is paid per leg for rows that already
gate. The measured figures, and why they carry no guard, are stated at
that step in `.github/workflows/ci.yml` and only there. What the
default-feature lane is therefore NOT checking is printed there by
name, by the `app_lane_skipped_*` rows in `src/lib.rs`,
`tests/chrome_labels.rs` and `tests/error_display.rs`.

**wasm.** The whole kernel plus `editor-core` compiles to
`wasm32-unknown-unknown`, `--features interval` included, and CI
re-takes that reading on every code-tier pull request with one
`cargo check` step for the interval build only; the default-features
half rides on it because `scripts/check-interval-cfg-additive.py`
keeps the interval build a syntactic superset of the library sources.
The guard establishes that the crates compile, not that they link or
run. `pncad` and this crate additionally need `getrandom`'s wasm
backend named in both halves: the `wasm_js` feature (the stanza in
`Cargo.toml`) and `RUSTFLAGS='--cfg getrandom_backend="wasm_js"'`;
setting only the flag fails the build, which is why the feature is
declared here so the flag is all a builder has to remember
(`local-scripts/serve-wasm.sh`). The browser lane itself is deferred.

## Banked post-v1

GUI-5, the threaded web lane, and GUI-6, the history graph: a
branch-picker UI and a separable history sidecar over the tree-shaped
undo `src/history.rs` already keeps (an edit after undo mints a
sibling; nothing is destroyed). Both are in `docs/LONGTERM-IDEAS.md`'s
GUI section until dispatched.
