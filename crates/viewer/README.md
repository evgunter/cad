# Running the viewer

```sh
cargo run -p viewer --features app -- [document.pncad]
```

The optional path is opened at startup through the same typed `Open`
operation the dialog feeds — and it is the only way to open a document
on a system with no file-chooser backend (below).

**Two documents share this page.** Everything down to *Architecture* is
about RUNNING the viewer — mouse bindings, the file-chooser and font
prerequisites a Linux or WSL desktop needs, headless operation, the
browser spike — and it is the tenth of the page a user wants.
*Architecture* and everything after it is the crate's implementation
record: what the code does and why it is arranged this way, for changing
it rather than using it. The ratified plan that record answers to is
[`GUI-DESIGN.md`](GUI-DESIGN.md) beside it.

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
add-parameter row (name + dimension + value, written in the unit the
form's picker names, one undoable edit); an
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

**Where the dialogs open.** Open… and Save As… start at the first of
three directories that still exists: the current document's, the one
the last dialog returned a path in (kept in the preferences file as
`[files] last_dir`), and the directory the viewer was launched from
(`frame::dialog_dir`). The portal is handed that directory; `rfd`'s
zenity fallback is not, and opens at the launch directory, its own
default. `xdg-desktop-portal` 1.6 honours the directory for Save As…
but shows its "Recent" view for Open….

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
| G1 layer 3 values and operations | `src/camera.rs` (`Camera`, `CameraOp`, `camera::apply`), `src/g1.rs` (`Slot`, the preview/commit rules both gestures obey), `src/session.rs` (`DocSession`, `DocSession::perform`, the operation doors) and its vocabularies `session::{select, refuse, op, author, delete, probe}` (Module boundaries, below), `src/history.rs` (tree-shaped undo), `src/input.rs` (`ViewportEvent`), `src/tools.rs` and the per-tool modules |
| G3 free-move and hiding as display state | `src/display.rs` |
| G3 mate definition | `src/matetool.rs` |
| Feature tree, property panel, open/save, evaluation seam, scene | `src/tree.rs`, `src/props.rs`, `src/docio.rs`, `src/evalseam.rs` (all three seams and all three workers) with `src/generation.rs` (`Generation`, the counter every seam keys its answers by), `src/scene.rs` |
| Colour, themes, preferences | `src/theme.rs`, `src/prefs.rs`, `tests/theme.rs` |
| GQ7 picking | `src/pickindex.rs` (the index and every query over it, up to what a pick MEANS — `PickIndex`, `IdMap`, `EDGE_PICK_RADIUS_PX`, `PickKinds`, `op_for`, `hovered_for`), `src/marks.rs` (what a frame marks over a built index — `highlight`, `edge_overlay`, `focus`), `src/pickcache.rs` (the index's lifecycle — `IndexInputs`, `PickCache`, `NotIndexed`), `crates/bvh` (`Bvh::ray`) and `camera::cursor_projection` (the id pass's 1×1 target transform, which is projection algebra rather than a mark) |
| GQ6 toolkit, viewport, docking | `src/app.rs` (the frame loop and `ViewerApp`) with `src/pane/*` (the pane bodies), `src/widgets.rs` and `src/gpu.rs`, all behind the `app` feature; `Cargo.toml`. `src/frame.rs`, `src/platform.rs` and `src/idpass.rs` are vocabularies and are built unconditionally. The authoring vocabularies the panels offer are `src/forms.rs` and `src/drafts.rs`, which name no toolkit type and are behind the feature only because the panels are |

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

**`scripts/gates/viewer-module-kinds.sh` is the machine that reads
it**, on both halves of CI, in the job that carries no tier condition —
its inputs are this file and `Cargo.toml`, and a change set of only
those is TIER=docs, on which every build-gated job is skipped. Each
module declares its own kind, once, in its own doc header —

```rust
//! Module kind: **vocabulary** — …
//! Module kind: **driver** …
```

— because the subject of the rule is a module's `use` block and the
declaration belongs beside it: an author changing a module's role
meets the contradiction in the file they are editing, and a new module
cannot land without answering the question.

**The gate's rosters and its needles are read out of the documents
they enforce, not restated.** The driver roster is the table below;
the vocabulary roster is the two vocabulary tables; and what a
vocabulary may not name is `Cargo.toml`'s `app` feature — *every*
`dep:` in it. That last one is the right population rather than a
curated list of toolkit crates, and the manifest already says why:
every entry there is optional and reached only through `app`, so a
vocabulary — which is compiled in a default-feature build — naming one
is naming something that is not there. A hand-kept version of that
list got it wrong in both directions on its first day.

The gate refuses a module that declares neither kind or both, refuses
a `driver` declaration on a module the driver table does not list, and
reads every vocabulary's code — not only its `use` lines, since a
fully-qualified name evades an import check, and not only single
lines, since `use crate::{app::x, camera::Camera};` hides the driver
inside a brace group. What it does not decide is a module's ROLE: it
reads what a module NAMES, so a module that owns state and dispatches
while importing nothing forbidden still passes as a vocabulary. The
gate's own header states the rest of its blind spots.

### The drivers

**Two drivers, eleven modules — and the table counts MODULES.** The
rule above ratifies two drivers, `session` and `app`; the app driver is
split for size across ten of the rows below (*The app driver, split for
size*), so the roster is eleven rows long and always was. A reader who
takes the two as a count over this table meets a contradiction, and the
count that answers to it is **eleven**.

**This table is the roster**, not a summary of one:
`viewer-module-kinds.sh` reads it, requires every module in it to
declare `driver` in its own header, and refuses a `driver` declaration
on a module the table does not list. The rule that re-derives the number is the other
side of that pairing — the modules that declare the kind:

    rg --files-with-matches '^//! Module kind: \*\*driver\*\*' crates/viewer/src | wc -l

which prints **11**, one per row below; drop the `| wc -l` and it names
them. That pairing is exactly what the gate holds the two sides to. So a new module OF the app driver is a row
here plus its own header; a third DRIVER is an amendment to the rule
above as well, and neither is a header edit alone.

| Module | Is |
|---|---|
| `session` | the session driver: owns `DocSession`, dispatches `SessionOp` |
| `app` | the app driver: owns `ViewerApp`, drives the frame |
| `pane` | the pane bodies' parent |
| `pane::create` | the create pane |
| `pane::features` | the feature-tree pane |
| `pane::profile` | the profile editor both profile doors draw (create form, Properties pane) |
| `pane::properties` | the property pane |
| `pane::view` | the view pane |
| `pane::viewport` | the viewport pane |
| `widgets` | the free helpers over `egui::Ui` the chrome shares — mostly the panes; `widgets::message` is also `app.rs`'s (Where a MESSAGE wraps, below, and the row that holds that roster) |
| `gpu` | the wgpu viewport renderer |

`session` is a driver **and** the parent of six vocabularies, so
`crate::session::SessionOp` in a vocabulary is the rule working, not a
violation. The gate reads that off the vocabulary tables below rather
than carving it out by hand: a driver that hosts a tabulated
vocabulary is not a forbidden import path.

### The session's vocabularies

| Module | Holds |
|---|---|
| `session::select` | `Selection`, `FaceSelection`, `EdgeSelection`, `Hovered`, `Standing` — what is selected and whether it still denotes anything |
| `session::refuse` | **every refusal vocabulary a session door raises**, each with its `Display`, the payloads and predicates that decide it, and the recourse text it spends — the rule, so that the next one sorts itself. Today that is two: `Refusal`, with its `rank`/`preferred` ladder and the composers `affordance`/`exists_wording`/`offer_wording`, plus the `NodeKindWanted` payload and its predicate `admits`; and `FaceFrameFault`, with `face_frame_seat`, the free function that answers it, and the recourse constant `NO_FACE_PICKED`, which `forms` spends |
| `session::op` | `SessionOp` and `OpOutcome`, with the gesture names `SessionOp` is keyed by — `ValueGestureName`, `FreeMoveName`, `GestureName` — and `CancelDoor`; already the crate's shared vocabulary, named in **27** of this crate's files, which `rg -l -e SessionOp -e OpOutcome crates/viewer/src` lists — drivers, panes and tool modules alike |
| `session::author` | `DatumSpec`, `PatternRuleSpec`, `datum_node`, the `ProfileShape` re-export — the authoring specs and their lowering to nodes, which hold no session state at all |
| `session::delete` | `DeleteAffordance` and `kind_census` — the cascade's wording |
| `session::probe` | `BoundsTarget`, `BoundsReading` and the range probe |

`session` itself keeps `DocSession`, its `Gesture`, `perform` and the
operation doors, plus the three values the session states about itself
— `Landing`, `AtRestBadge` and `Outstanding`. None of them can leave:
every door returns a `Refusal` and mutates the session, and `perform`'s
dispatch is the one place an operation becomes state.

**This page is the one home for the argument below.** The types
themselves carry their invariant and a pointer here; the reasoning is
written once.

`Outstanding` is the third of those values and the one with a rule
attached. The session answers two questions about work — is the picture
older than the document (`busy`), and does the seam have work
(`running`) — and each is useful alone, so both stay. **Read together
they are one three-state fact, and a consumer is handed that fact and
never the pair.** Two adjacent `bool`s that mean different things
transpose silently: the swap type-checks, the chrome it produces is
plausible, and a row that covers the consumer by repeating the same
positional convention agrees with a transposed call site rather than
contradicting it. So `DocSession::outstanding` reads the two by name —
there is no argument list for them to be positions in — and
`frame::progress` takes the folded value beside the index seam's
`bool`, two arguments of different types that no call site can
transpose. The fold itself is covered by driving a session into each of
the three states (`tests/eval_seam.rs`), because a row that names the
states says nothing about which session state produces which.

`platform`'s `Zenity` and `SessionBus` are the same rule one module
over: two independent environment readings that `ChooserBackend`
ranks, named so the pair cannot be transposed either. That is the whole
population of adjacent same-typed `bool` parameters in this crate;
`work/view/adjacent-same-typed-arguments-are-the-same-swap.md` carries
the wider class, where the types are not `bool`.

**Every "is work outstanding" answer comes from whichever of the two
records can be wrong about it, and there are only two.** There are
three seams — evaluation, the pick index and the display fit — and
each has a consumer that reports whether work is owed. The seam knows
that it is busy; a consumer that keeps its own record knows WHAT it is
busy with, and where they can disagree the consumer's record is the
one that decides.

- `DocSession::running` is `EvalService::busy` directly. The session's
  own record (`DocSession::busy`, its two generations) answers a
  different question — is the picture older than the document — and
  the two differ after a cancel, which is the whole of
  `Outstanding::Canceled`.
- `PickCache::indexing` is the cache's own record alone. The seam
  cannot tell an orphan from a live build: `PickCache::forget` drops
  the attempt when the picture stops existing, and a build already
  destined to be discarded (`IndexLanding::Stale`) must not go on
  lighting the indicator while the seam finishes it. In the other
  direction they cannot disagree — an attempt is recorded in the step
  that submits it and the seam holds the request until the answer
  `pump` takes straight to `land`.
- The fit's two reads in `app` are `FitService::busy` directly; there
  is no second record to consult.

**A seam that has stopped answering is no longer one of the states any
of this covers.** It used to be: reporting the cache's record alone
left the toolbar spinning on `indexing…` for the life of the window,
repainting every frame to collect a result nobody would send, and
refusing every click with *the picture is still being indexed*, of a
picture nobody was indexing. What closed that is the panic below and
not a second read — a seam whose worker has gone ends the process
where it finds out, so no consumer has to describe one.

**A worker that CRASHED is no longer one of the states this rule
covers, and the change is deliberate** (Ev, in-chat, 2026-09-17:
*"isn't a worker dying an infra thing that should show up as a
panic?"*, then *"panic on crash is good"*). A seam's two endings used
to be answered identically — a failed `send` and a `Disconnected`
receive both just cleared the flags — so a crashed seam and an idle one
became indistinguishable everywhere above the boundary, permanently and
in silence. They are told apart now by the one thing that separates
them: `Coalescing::close` takes the request channel and is called from
`Drop` alone, so a detection that still holds the channel is a crash
and nothing else. Shutdown forgets its work quietly; a crash panics on
the UI thread, at the point of detection, naming the seam.

That panic is not a hole in D9. The workspace's no-panic family is
scoped to INPUT, and nothing a document contains and nothing a reader
does can stop a worker; this is the bug-the-code-observes class that
clause already hands to `unreachable!`, taking `panic!` instead
because it is genuinely reachable. What makes the announcement worth
anything is that it terminates: `egui`, `eframe`, `egui-winit` and
`egui-wgpu` 0.36.1 contain no `catch_unwind`, and `winit` 0.30.13 has
none on the linux backends, catching only on macOS and Windows where
both paths re-raise. `tests/eval_seam.rs`'s
`a_panic_inside_an_egui_frame_is_not_swallowed` executes the layer
nearest the panic against eframe's own per-frame door, so a toolkit
upgrade that added a catch would red rather than quietly make the
loudest thing this crate does a no-op.

### What the session knows because of the document is one value

`DocSession` holds a `Derived`: what is selected, what is hovered,
what a drag previews over the document, what the last run said about
it (`LandedRun`), and what a range probe found in it. `Open` and
`NewDocument` install a different document under a live session, and
all five of those are statements about the document that was there
before — left in place they answer about the previous model until the
first run lands. Both doors therefore reset the whole block by
construction (`Derived::none`) rather than field by field, and the
constructor writes that same value, which makes it the one spelling of
"nothing is known yet". A field added to `Derived` does not become
correct on its own: `Derived::none` stops compiling until someone
writes its cleared value, at one site, by hand. That is the whole
mechanism — one site to update instead of three, and a compiler error
instead of a silent omission.

`LandedRun` is the same rule one level down. The seven things a
landing produces — the evaluation, the document it answers, its
generation, the gather's refusal, the A5 badge, the advisory report
and the gathered body — are statements about one (document,
evaluation) pair, taken from that pair's single gather in `land`. As
one value they cannot come from different runs, which is the property
`landed_pair` needs: it returns two of the seven, and the two it
returns are the pair a single run answered.

The body is the one of the seven that is not always there, and the one
with a cost on the other side of the ledger. It is kept so that the
display fit does not gather the same product a second time — 87 ms
against an `Arc` clone, on a 165-root, 990-face document — and the
price is that the session retains one gathered aggregate for the life
of a landing, beside the `Doc` and `Evaluation` it already holds. One
at a time: the next landing replaces it, and `Open` drops it with the
rest of `Derived`. It is absent when the gather refused, and when the
A5 gate refused and so consumed the product it was judging; the
consumer that needs one there gathers it itself, at a door that says
so (`scene::product_of_evaluation`). `bounds` sits in
`Derived` but is discarded on a stricter rule than the block's: every
submit drops it (`request_eval`), because a range is a statement about
one document and a commit or an undo already invalidates it. Both
doors consequently drop it twice, which is redundant on purpose — the
walk names every field it invalidates rather than leaving one to a
route its reader cannot see.

Three neighbours stay outside `Derived`, and the reasons are the
interesting part. `DisplayState` is `clear`ed rather than rebuilt,
since its revision counter is the chrome's rebuild key and must not go
backwards; its own `clear` closes the same hazard inside it. `gesture`
is cleared by nothing and must not be, and the refusal runs the other
way round from the sentence one reaches for: while a value drag is in
flight the DOOR is refused (`Open` and `NewDocument` are two of the
rows `permitted_during_value_gesture` says no to, checked once in
`perform`) and the drag is left untouched, because a gesture
dissolved under the pointer is the half-acted state that refusal
exists to prevent. So the precondition is established before either
door writes anything, and the reset re-checks nothing — a check there
could only fire with the session already half-replaced. `path` and
`resolver` are facts about the backing file rather than about the
document, and are the part of the two doors that genuinely differs:
`Open` sets both, `NewDocument` clears both.

**That table governs value gestures only, and there is a second.** The
free-move drag `DisplayState` owns is a different value with a
different owner, so it has its own row list
(`SessionOp::permitted_during_free_move`) rather than a widened one:
the two drags refuse different sets, and one table could serve both
only by refusing the union. A value gesture refuses every operation
that moves the document, because it previews against a snapshot of it;
a free move does not, because a commit landing under a probe is pruned
against the new document and REPORTED, which is a better answer than a
refusal.

**What the two tables agree on is exactly these two doors.** `Open` and
`NewDocument` REPLACE the document rather than moving it, and
`clear_for_new_document` then drops the whole display state — so a
prune has nothing to report against and the drag would go under the
pointer holding it. Both are refused while either drag is open, and
neither is ever dissolved in silence
(`no_operation_dissolves_an_in_flight_free_move_in_silence`). The
free-move refusal is `DisplayFault::FreeMoveInFlight` — *"finish the
free-move first"*, which names a door the user has — and not
`Refusal::GestureInFlight`, which names the other drag.

**The dump is held to the same declaration.** This paragraph is the one
home for the rule; the four walks that follow it state their own `_`
arms and point here rather than restating it.

`Debug` for `DocSession`, for `Derived` and for `LandedRun`
destructures its own value exhaustively, so a new field stops the
rendering compiling. The error is E0027, pattern-does-not-mention-field
— a *different* error from `Derived::none`'s, which is a struct literal
and so raises E0063, missing-field-in-initializer. The property is the
same at both sites and the two errors are not, which is worth saying
because a reader looking for one and finding the other concludes the
mechanism is not there. `DocSession` renders `Derived` as one field
rather than reaching through it, so those members travel with their
declaration instead of being listed a second time.

**A field the walk will not carry is bound to `_` rather than left out
of the pattern**, which is what makes the omission a decision a reader
can see, and those `_` arms are precisely what a `finish_non_exhaustive`
here stands for — one reason each, never a blanket one:

| walk | `_` arms | why |
|---|---|---|
| `LandedRun` | `evaluation`, `doc` | the result DAG and the recipe DAG it answers |
| `DocSession` | `tol` | `Tol(())`, a ZST with no content |
| | `eval` | a `dyn` service implementing no `Debug` |
| | `requested_doc` | a whole recipe DAG |
| | `display` | not derived from the document, as large as its hidden and moved sets, and reachable through `DocSession::display` |
| `PickCache` | `seam` | a `dyn` service implementing no `Debug` |
| `Derived` | — | none, so it `finish`es |

**A carried field may be summarised, and several are.** `states` is the
history's length and `checks` is its two counts (`ChecksReport` is a
`Vec` per finding with no bound, and a dump that inlined it would be
the thing these walks exist to keep readable); `scratch`, `body`,
`gesture` and `resolver` are their presence, rendered as the elisions
`Some(<Doc>)`, `Some(<Body>)`, `Some(<Gesture>)` and
`Some(<DirResolver>)`; and `index` is the generation it describes
inside one, `Some(<PickIndex for Generation(4)>)`. Summarising is what
a `#[derive(Debug)]` cannot do at all, which is the reason these are
written out rather than derived; the recipe and result DAGs are what
makes that reason bite.

**A summarised field renders as a summary**, so the marker only ever
has to answer the question it can answer. A count, a pair of counts and
an elision are each something no value of the field's own type renders
as, which is the property being bought: `scratch: false` was a `bool` a
reader who knows `std` and not this page could take for the whole of a
`Doc`, and `index: Some(Generation(4))` was an `Option<Generation>`
this cache does not have. `finish`/`finish_non_exhaustive` says whether
every FIELD is shown; whether the value shown is the whole field is
answered at the field, which is the only place a two-valued marker
could not have said it. **The sweep behind that list**, and it is a
reading of four `fmt` bodies rather than a grep: in each of the four
walks — `Derived`, `LandedRun` and `DocSession` in `session`,
`PickCache` in `pickcache` — read every `.field(…)` call, **21 of
them**, and take the ones whose value argument is not the destructured
binding itself. **Nine calls, seven fields**: `checks` and `index` each spend two arms, and the absent
arm renders `None`, which is the whole field. `viewer`'s
`tests/debug_dumps.rs` holds the seven to their spellings, and is the
only reader of these dumps in the tree. What holds the NEXT summarised
field to the rule is not a check: the destructuring makes the compiler
send whoever adds a field to the walk, the rule is stated here and in
each impl's own doc comment for them to read when they arrive, and no
PRODUCTION path computes on a dump — the only reader is that suite, and
what it reads a dump for is these spellings — so a lapse costs a reader
a misreading and can never cost an answer.

`PickCache::forget` takes the same destructuring for the same reason
one seam further, and is a row of the table below: it clears the three
fields that describe a picture — `index`, `attempt`, `error` — and must
not miss a fourth, since a missed `attempt` is what lets a late build
install an index of a document nobody is looking at.

**The rule is a field census, not a `Debug` rule.** A CENSUS is a walk
whose correctness argument is that its list IS the value's fields —
*forget everything*, *drop every pick*, *every number equality is on*,
*this sentence is the value's whole account*. Every census in this
crate destructures the value instead of listing its fields by hand, so
the list cannot fall behind the declaration; which trait the census
sits in decides only what a missed field COSTS, and the sharpest cost
is not a dump's.

**The rule that produces the table below**, stated here because a
population certified in prose is where the next defect hides. The
mechanical half is every line under `crates/viewer/src` that opens a
destructuring `let`, and the command IS the answer rather than
something to read an answer off:

    rg -n --no-heading 'let\s+&?([a-z_]\w*::)*[A-Z]\w*\s*\{' crates/viewer/src | wc -l

which prints **24**. Drop the `| wc -l` and it prints one line per
bind, and no hit is in a comment. The reading half sorts those 24, and
every member is named so the sort can be argued with rather than
trusted:

- **Four are `Debug` dumps**, the walks above: `Derived`, `LandedRun`
  and `DocSession` (`session`), `PickCache` (`pickcache`).
- **Four bind a parameter or a returned vocabulary struct to name its
  parts**, and claim nothing about completeness:
  `widgets::drag_gesture_ops` over `GestureVocabulary`,
  `widgets::value_field_ops` over `FieldShowing`, and the two
  `ProbeOps` unpacks in `pane::properties` and `widgets`.
- **Two destructure a TOOLKIT type at the chrome boundary** and are
  not rows: `pane::viewport`'s `viewer_modifiers` over
  `egui::Modifiers` and `scroll_event` over `egui::Vec2`. Each carries
  a completeness argument of its own and each is worth having, but the
  declaration they are held to is the toolkit's rather than this
  crate's — a field arriving there is a version bump's news, which is
  the upgrade hold `scroll_event`'s own doc calls nominal, and not a
  value of ours whose account has fallen behind it.
- **The remaining fourteen binds are the thirteen censuses below.**
  `PartialEq for Camera` spends two of them, the second over
  `Point3`'s coordinates, which is the one place a census here reaches
  past this crate's own fields.

What that rule cannot see, said rather than left: a census
destructuring in a `match` arm or a function's parameter pattern
instead of in a `let`, and one over a value reached through an
accessor. Neither exists under `src/` today and both would be members
if one did — so the table is the population of record and this
paragraph is how a reader re-takes it, not a claim that no other shape
could hold a census. The qualified-path arm of the pattern is there
because leaving it out is how the two toolkit binds above went unseen
by an earlier taking of this rule, which then reported a smaller
population with nothing to say it was short.

Thirteen censuses, none of them a dump:

| census | costs, if it misses a field |
|---|---|
| `PartialEq for Camera` | equality answers **wrong**. `camera::fold` is checked against sequential `apply` by comparing whole cameras, so a coordinate outside `eq` is a coordinate that property does not check |
| `DisplayState::clear` | display state survives into a different document — the stale-across-`Open` defect the `Derived` walk closed |
| `BlendTool::clear` | a pick survives `Clear picks`, so the tool is not the fresh tool the button promises and the next click is judged against something the panel says it is not holding |
| `Display for StoreError` | a store's failure carries a fact the sentence does not say |
| `Display for Message` | **nothing, by design** — this account is deliberately partial, and that is exactly why the tie is worth having: it makes the NEXT field's omission a decision someone made rather than one nobody noticed |
| `Display for Withdrawal` | a field joins a value whose whole job is to word itself and goes unworded |
| `Withdrawal::all` | a KIND of withdrawal reaches the chrome's notices and is never worded — the fan-out from a `PruneReport` that three hand-written `extend` calls in `app`-gated code used to do, where no row could execute it |
| `Display for Disagreement` | the doc above it argues both halves are load-bearing; a third field left out would falsify that sentence silently |
| `Display for BlendTarget` | a refusal names a scope narrower than the target it refused on |
| `PruneReport::is_empty` | a fourth kind of withdrawal leaves the revision where it was, which is the chrome not rebuilding a picture that changed |
| `PickCache::forget` | a fourth thing describing the picture outlives the picture — a missed `attempt` is what lets a late build install an index of a document nobody is looking at |
| `Display for Unusable` | the one sentence a refused preferences store shows says less than the value holds |
| `Unusable::refusal` | the refusal the `save` door returns and the sentence the read composes drift apart, which is the divergence the type exists to prevent |

`Camera`'s census reaches one type further out: `target` is a
`Point3<f64>` expanded coordinate by coordinate, so a second pattern
names `x`, `y` and `z` rather than reading them — the boundary is where
a census of this crate's fields would otherwise stop. It reads the
fields and not the six public accessors beside it for the same reason
it destructures at all: an accessor call is a field READ, so a census
assembled from accessors is a hand list again and a seventh field
would leave it silently short.

Three of the thirteen name a field the walk deliberately does not
spend. `PickCache::forget` binds `seam: _` and must: the seam is the
service, not the picture, so forgetting it would drop the worker along
with the answer it is holding. `DisplayState::clear` binds `revision`
and does not clear it: the
counter is the chrome's rebuild key, it is bumped when the reset was
visible, and a counter that went backwards would name a picture the
chrome has already drawn. `Display for Message` binds `subject: _` — a
bare `_`, with the argument in the doc above the impl rather than at
the arm — because the subject ROUTES the message: it is what retires
it (`frame::StatusUpdate::Expire`) and what a joined rank-2 line takes
as its own subject. It does not RANK; `frame::frame_status` ranks by
SOURCE. A line that printed its own routing would say to the user what
the chrome says to itself.

**A `match` is exhaustive over VARIANTS, not over a variant's FIELDS.**
The six `Display`s above are the struct half of a population of 41
`Display` impls under `src/` — `rg 'impl.*fmt::Display for '
crates/viewer/src | wc -l` prints that 41, and no hit of it is in a
comment — and the other 35 are over enums, where
being a `match` settles nothing about their fields. Sweeping those 35
for a
pattern that drops a field of the variant it renders — `{ .. }` or
`, ..}` in a pattern, a catch-all `_ =>` or bare-binding arm over the
subject enum, and a tuple variant matched at less than its arity —
finds **no catch-all over a subject enum, no tuple-arity drop, and
exactly two `..`**: `CameraOp::Frame` drops `bounds`, and
`MateToolEvent::PickLost` drops `resolution`. Both stay dropped —
rendering either would change what the chrome says — and both carry
the argument for the drop, `MateToolEvent`'s at its impl (the payload
stays typed and full in the value; the sentence is what a person
reads) and `CameraOp`'s at the arm. That is the property this rule is
after: an omission that is a decision someone made. The rule matches
one more site that is not an instance, and the distinction is the
usual one: `frame.rs`'s
`matches!(w.cause, AdmissionFault::FusedGeometry { .. })` is a variant
test on another type rather than a pattern over the subject. The arm
that words a withdrawal's count beside it matches an `Option`
exhaustively — the two kinds that are over a SET carry a plural and the
kind that is over the one gesture in flight carries `None` — so a
plural no constructor can reach is never worded.

**What was swept for the writing hat, and what it could not see.**
Every `fn` under `src/` naming two or more distinct `self.<field>`
assignments, `.clear()`s or `.take()`s, each hit read against its
struct's declaration: **not one of them is a census**. A converted
census does not match the rule at all — it has no `self.<field>` write
left — so a clean sweep is the receipt. The hits are bookkeeping, where
the field list comes from the walk's INPUTS rather than from the
declaration and a new field has no claim on it: `ViewerApp::sync_scene`
installs a rebuild's eleven outputs, `BlendTool::load_all_edges` seats
a computed pick set, `PickCache::sync` and `land` install a landing's
fate, and `evalseam`'s one `Coalescing::close` closes a channel and
leaves the language's own drop glue to be exhaustive.

**This sweep carries no number, and that is the finding rather than an
omission.** It said *23 hits* for a while. Two later takings of the
same words read **24** and **28**, and the second pair was over ONE
tree — so the disagreement is not drift. The rule as stated does not
determine which `fn` a `self.<field>` write belongs to: a closure
inside a `fn`, a `Drop` body in an `impl` block and a macro expansion
are each counted or not by the instrument rather than by the rule, and
three instruments gave three answers. What the sweep is FOR survives
that completely, because it is a claim about the population and not
about its size: every hit is bookkeeping and none is a census, and a
reader who re-takes the rule with their own instrument can check that
against whatever set it hands them.
`DocSession::clear_for_new_document` is the case the rule matches and
the design answers: its two statements are `Derived::none()` and
`display.clear()`, and its doc says so — the census is collapsed into
one value rebuilt from nothing rather than a field-by-field walk each
door has to remember. What neither rule can see: a census spelled
through accessors rather than fields (no grep for `self.` finds one), a
census over a value that is not `self`, and an impl written by a macro
— `vocab.rs` holds the crate's only `macro_rules!` and it generates
neither.

### The app's vocabularies

| Module | Holds |
|---|---|
| `forms` | What the panels offer for authoring, and how a typed field behaves. The vocabularies — `DatumKindChoice`, `ShapeKind`, `PatternKindChoice`, `MATE_PRIMITIVES` — mirror a kernel or session enum, and the MIRROR is what is hand-maintained: the three enums declare themselves and their `ALL` in one declaration (**Closed vocabularies are declared once**, below), so no membership list here can fall behind its own enum, while `MATE_PRIMITIVES` mirrors an enum in another crate deliberately partially and says so. A kernel vocabulary this crate offers WHOLE is not mirrored at all: the boolean form draws one button per entry of `topo::BooleanOp::ALL` and writes only the labels, at an exhaustive match, and the path form does the same over `profile::Verb::ALL` (whose `Display` is its word), `profile::ArcMode::ALL` and `profile::TargetKind::ALL`, editing the kernel's own `Step` rather than a copy of it. The field-writing family — `FieldWriting`, `drag_tick` and the four drag speeds — mirrors nothing and is a product decision on its own (how much of a unit one pixel of drag is worth). Both are decisions the toolkit does not make, which is what puts them here rather than in `app` |
| `drafts` | `Drafts`, `ProfileEdit` and `CommitFault`: the in-flight form state (`ProfileEdit` is the add-profile form's editor held over a committed profile, for the edit door), its defaults, and its lowering of typed field values to `Expr`, `LoopProgram` and the add-datum form's `session::DatumSpec` — the same layer as `session::author`, and today the larger half of it |
| `frame` | The per-frame policies the viewport runs, as values: hand one the values a frame holds and it answers the same way every time, with no window, no session and no process around it — which is what makes a rule about the chrome testable at all, and why the frame loop still decides WHEN to call one and no longer decides what it MEANS. What the chrome has to say and which of its two channels says it (`Subject`, `Message`, `StatusUpdate`, `Badge`, the doors that build one and the two that spend one — `apply` for a ranked verdict or a retirement, `deliver` for a policy that may or may not have news), `frame_status`'s ranking over a frame's news, the badge family including `product_badge`, the draft and the offer a refused batch leaves behind (`retype_draft`, `creation_offer`), what a folded event stream amounts to (`folded_moved`, `fold_status`), and what a frame says about work outstanding (`progress`). **The charter's exclusions are the half that was missing**: a concern that reads ambient process state is a function of the machine and lives in `platform`; a concern that carries state across frames is not a function of one frame and lives in `idpass`. Both are consumed here (`cursor_status` takes an `idpass::IdStep`) and neither is decided here. This row used to say the charter argues for taking each concern out of `app` and **not** for their being one module — `work/view/frame-module-has-eight-concerns-and-no-holds-row.md` owned the split that sentence deferred, and the split is taken: the charter above is now true of what is here, so the row covers the module rather than confessing that it cannot |
| `platform` | What the environment the process was started in offers the shell, read once before the first frame. Each value here — the chooser-backend verdict (`ChooserBackend`, `chooser_backend`, `chooser_backend_of` over `Zenity` and `SessionBus`), the XDG preferences path (`prefs_path`, `prefs_path_in`), the WSL probe (`running_under_wsl`) and the reason a dialog the environment cannot put up gives for being disabled (`NO_CHOOSER_BACKEND`) — takes the environment as its ARGUMENT, so none is a function of anything this crate holds and none can be replayed from a value a test builds. That is why they are not `frame`'s and why they are one module: `scripts/gates/no-ambient-env.sh` ratifies that the viewer's runtime environment reads have ONE home and allowlists this file as that home, and its argument against the gate's four rows is an argument about exactly these probes. A module that exists FOR the door is what makes that entry a door rather than a region inside something else |
| `idpass` | The GPU id pass's bookkeeping: what query is outstanding, what it was asked about, and what its answer is worth when it comes back (`IdQueryLog`, `IdSubject`, `IdStep`, `Disagreement`, `disagreement`). The id pass is a round trip — one frame issues a query, a later frame reads the answer, and in between the cursor can move, the picture can be rebuilt and the index can be replaced — so the only thing that can say whether an answer still describes its question is state carried ACROSS frames. That is what puts it here rather than in `frame`, whose policies are values precisely so they can be replayed: everything in this module exists because it REMEMBERS. The failure it remembers against is an answer outliving its question, which does not look like a fault — it reports as *the two picking paths disagree* |

### Two axes: which channel, and what retires it

The chrome has two channels for something it has to say, and a fact is
sorted twice. **The two sorts are independent, and both get stated.**

**Which channel.** A `frame::Badge` is a **read of held state a reader
consults**; a `frame::Message` on the status line is the **outcome of
something that just happened**. A badge therefore outlives the frame
that raised it and the line carries one frame's news — the lifetime is
the consequence of the test, not the test.

**The channel is decided by PROVENANCE** — what caused the sentence to
exist — and not by what it is about (Ev, 2026-09-06). Every refusal is
about something and caused by something, so both are coherent axes;
provenance wins because it is the only one a reader can SEE, in whether
the sentence exists on a frame where nobody acted.

**"Held state" is the mechanical shadow of that, a strong indicator and
not a decision procedure**, and the sweep that sorted eighteen writers
on this rule needed the three ways it falls short. It is a property of the FACT and not of a
signature — `frame::unindexed_refusal` takes a `&NotIndexed`, and what
makes it an outcome is that `pickcache::unindexed` raises it for a `Select`
and nothing else. Tracing to the raiser does not settle it either:
`idpass::Disagreement` reads only held state and is recomputed every
frame the cursor holds still, and what sorts it onto the line is *a
reader **consults** a badge*, because a claim about where the pointer
is this instant is something a reader is told rather than something
they keep open and act against. And whether a fact is held at all is a
choice the author makes — `ViewerApp`'s `scene_fault` and
`projection_fault` did not exist until the badges that read them did,
and any outcome can be made a read by storing it. The mechanical form
constrains the answer and never supplies it.

**What retires it.** Both channels carry a `frame::Subject`: the
recurring event stream whose next event makes the thing the wrong
answer, named by that stream rather than by who wrote the sentence —
the camera (the next camera event, issued by `frame::fold_status` on
every clean fold), the cursor (the next cursor move, issued by
`frame::cursor_status` off the id pass's own bookkeeping), the document
(the next act the document accepts), the picture drawn from it, and the
viewer's preferences. Carrying a subject never decided which channel a
fact goes to: the projection refusal is a **badge** that has the
subject **camera**, and the pick index's refused click is a **line
message** that has the subject **display**.

What differs between the channels is the ENFORCEMENT. A message is
stored as a message, so retiring it is the chrome's own bookkeeping:
`frame::StatusUpdate::Expire` retires one subject and `Clear` sweeps
the whole line, belonging to the acting batch alone because an act the
document accepted makes every held complaint stale. `frame::apply` is
the one place a verdict becomes the field. **No such machinery touches
a badge** — its subject names the event that changes the state it
reads, and the badge goes because the read does. The state itself may
still be bookkept by hand (`ViewerApp` clears `scene_fault` where a
rebuild lands, `pane::viewport` clears `projection_fault` where a
matrix forms); that is work about the seam, not about the chrome, and
no writer decides the fate of anyone else's sentence.

**Seventeen of the eighteen writers that can put a sentence on the line
now come through the ranking.** All eighteen used to reach the field
without it —
sixteen assignments, one struct-literal initializer at startup, and
`frame::fold_status`, which answers in the vocabulary and applied its
verdict at `pane::viewport` without asking. Each named its subject —
`Message` is the only spelling there is — but naming a subject is not
asking the ranking. Seventeen now push onto the frame's `notices`,
which is why `ViewerBehavior` carries that field.

**The eighteenth is the startup initializer**, `app::ViewerApp::new`'s
`status: frame::startup_notices(…)`: the preferences file's complaints
written into the field before the first frame, where the session's
first accepted act silently deletes them. Joining the notices is not
the fix — a complaint about the file as it stands is a read of held
state, so it wants a badge, and badging it means holding it and
deciding what retires it. That is
`work/view/startup-notices-need-holding-to-badge.md`.

**Applying a verdict outside the ranking is not the same as writing
one**, and the difference is what the count turns on. A retirement has
nothing to say and must NOT be ranked: `frame::cursor_status` returns
only `Keep` or `Expire`, so it can never put a sentence on the line and
was never one of these writers. `frame::deliver` is the door that
splits the two: news to the notices, retirement to the field;
`frame::apply` stays the door a retirement belongs at.

**A missing file-chooser backend is not on the line at all**, and the
provenance rule above is why rather than a reachability accident. It is
probed once at startup and true for the whole run, so it is held state
a reader consults — and the read is the disabled Open…/Save As…
control with `platform::NO_CHOOSER_BACKEND` as its
`on_disabled_hover_text`. A status route beside it once carried the
same sentence as a `frame::Message` with `Subject::Document`, i.e. on
the OUTCOME channel; no click could reach it, because one copy of
`ViewerApp::chooser` both gated the button and fed the policy. Ev ruled
the arm MISCLASSIFIED rather than merely unreachable (2026-09-09): a
whole-run environmental fact has no correct sentence on a line that
carries one frame's news, so the arm and its policy are gone and the
hover text is the whole surface. An empty-handed dialog under a
plausibly-present backend is a genuine cancel and was always silent.
**The sweep rule is over the FACT, not over the string.** A rule
ranging over readers of `platform::NO_CHOOSER_BACKEND` would leave the
universal above green while a future route built its own `Message` from
`chooser.usable()` — so the population is *every read of
`ViewerApp::chooser`, this crate's only value of type
`platform::ChooserBackend`*: one, `app.rs:1172`, consumed at `:1174` and
`:1193` as `add_enabled(chooser.usable(), …)` with
`platform::NO_CHOOSER_BACKEND` as the disabled reason and nowhere else. No
reader builds a `Message`, a `Badge` or a notice from it. What the rule
cannot see is a route that re-probes the environment instead of reading
the field — `platform::chooser_backend()` has one caller (`app.rs:688`,
the constructor), which is the fact that makes the field the whole
population rather than a sample. **This is the argument's one full
copy**: `platform::NO_CHOOSER_BACKEND`'s own doc and the toolbar comment
at the two controls point here rather than restating it.

**A store that keeps no preferences is on the toolbar too**, by that
same rule and with the opposite answer at the control. Whether a
`prefs::PrefsStore` can hold anything is settled when the store is
built and true for the whole run — `prefs::Absent` always keeps
nothing, and the native `file::FileStore` keeps nothing where
`platform::prefs_path` found no config directory — so it is held state a
reader consults and `frame::prefs_badge` is that read, with
`Subject::Preferences` and `Tone::Advisory`. A write that was attempted
and failed is the other channel's (`frame::store_refusal`), which is
why the subject wears both. **The control is ANNOTATED rather than
disabled**, and that is where this parts company with the chooser: a
file dialog with no backend can do nothing, while the palette picker
applies the theme to the screen on the frame it is chosen and loses
only the memory of it, so disabling it would cost a reader the half
that works to protect the half that does not.

**The store is the party that words the condition**, as
`prefs::Unusable`, and the chrome renders its words rather than
composing its own; the crate carried two hand-written refusals for
this condition and no sentence for the reader before that. What is
mechanically held is that the two renderings cannot **diverge** —
`Unusable::refusal` and the badge are asserted equal — and not that
the condition has a single spelling: an identical literal written back
at a `save` is green, which was measured rather than assumed.
**The sweep rule is over the FIELD, not over the string, the name or
the target**: the population is every read of `app::ViewerApp::store`,
this crate's only `PrefsStore` value — three, all in `app.rs`:
`store.unusable()` at the guard in `remember_prefs` and again at the
badge beside the picker, and the `store.save` that guard stands in
front of. `store.load()` in the constructor is not one of them — it
reads the LOCAL binding, before the struct literal that makes the
field exist — and counting it is how this sentence first said four.
The population is complete because the field is private to `app` and
`prefs_store` has that one caller, and the count is held by
`frame_policy.rs`'s `the_readme_counts_its_two_populations_correctly`
rather than by this sentence. **Nothing here keys on `target_family`**: the
`cfg` alias at `app.rs` decides only which store answers, both answers
can be `Some`, and a browser build given a `web_sys::Storage` store
would leave the class on its own.

**The badges.** A `frame::Badge` carries its subject, a `frame::Tone`
(`Advisory` for a report, `Actionable` for a verdict a reader may need
to act on — the rule that a poisoned row stays `Advisory` is STATED by
`tree::RowStatus::tone`, the one function outside `frame` that decides
a tone, and `pane::features` reads that value rather than arguing it at
its draw), an optional hover detail, and a `frame::Affordance`: `Read`
for a label, `Opens` for a control,
which the advisory-checks badge is because a tooltip is the wrong home
for text a reader keeps open while acting on it. There is one member
per read — the at-rest verdict, the advisory checks, the product
fault, the budget's δ, the store that keeps no preferences, the datums
this view draws nothing of, the committed profiles it cannot draw, and
the three display seams that hold a refusal (scene, pick index,
projection) — each a function of the typed value it reads, so each
one's SILENCE is a row a test can write. The datums and profiles
counts are the two members that HOLD nothing: each writer re-takes its
count every frame and the application zeroes it whether or not the
viewport drew, so it says what the last frame found. The toolbar draws
before the panes, so it trails the view it describes by one frame and
no more — a bounded lag, where a latch with no sweeper is unbounded. **The population is every
`frame` function returning `Option<Badge>`** — ten — and that rule
ranges over the property rather than over the `_badge` naming
convention it happens to agree with today; it is complete because
`Badge`'s fields and its three constructors are private to `frame`, so
no badge can be built anywhere else. The count is held by
`frame_policy.rs`'s `the_readme_counts_its_two_populations_correctly`,
which scans the return types rather than the names. A door answers
the subject from the refusal TYPE it was handed, and where one seam's
refusal arrives as two types both name one constant, so its two
channels move together. `app::draw_badge` is the single draw and
`app::toned` the single tone-to-chrome mapping, read by that draw and
by the feature tree's row badge, so what `Advisory` looks like is
changed in one place or nowhere; what a click on a control means stays
at the call site, which is why the draw hands the response back and
names no window. `tree::RowStatus::badge`
is the same shape at the row rather than the toolbar.

Notices — a tool's declined pick, a survival drop, a
`frame::Withdrawal` — are typed values with `Display`, joined into rank
2 by `frame_status` with one separator. None of them composes prose
about another value's failure: the failure renders itself, and what the
chrome adds is its own subject.

**The line is composed at two levels and they are two marks.**
`frame::NOTICE_SEPARATOR` goes between two of a frame's notices;
`frame::LIST_SEPARATOR` goes between the items of a list ONE notice
carries — a `Withdrawal`'s causes, which its counted preamble
introduces. One spelling served both until a frame could hold two
notices, and then a reader could not tell a boundary from the notice
talking, because a notice is free to write the mark inside its own
sentence and two of them do. The boundary is
`frame::NOTICE_MARK`, and `frame::Message::new` — the only door, the
fields being private — takes that mark out of every text that reaches
it, while the one constructor that writes it takes `Message`s rather
than strings, so the only way to a boundary mark is to have had two
notices and `line.split(NOTICE_SEPARATOR)` returns exactly the ones
that went in. **The enforcement is at the door and not at the join**: "no
notice contains the separator" is a claim about strings that no
signature carries, and the door is the one place where making it true
costs nothing a reader sees — no producer writes a bullet, and a door
that refused one would be reachable from the keyboard through the δ
field's echo of what was typed.

**One level in, the same claim is held by the ELEMENT TYPE, because
there is no mark left to take.** `Display for Withdrawal` joins a
withdrawal's causes with `LIST_SEPARATOR` and joins them flat, so a
cause whose own sentence writes one reads as an item more than it is.
Every mark still available there is punctuation a sentence is entitled
to, and a door that rewrote one would show a reader words its author
did not write — so what the crate holds instead is the population the
claim ranges over. `display::AdmissionFault` is what the two admission
tests answer and what `Withdrawn::cause` stores, so the sentences that
must not carry the mark are its four, a fifth cannot arrive without an
arm there, and `DisplayFault::NonRigidFrame` — which writes a
`LIST_SEPARATOR` inside one sentence — is outside the type the join
can reach. The claim over that population is
`a_withdrawn_cause_never_carries_the_list_mark`, and the join's
invertibility is `a_withdrawals_cause_list_splits_back_into_its_causes`
(`crates/viewer/tests/frame_policy.rs`).

**The third consumer was the second level misread.** The preferences
file's startup notices were joined with `LIST_SEPARATOR` as though they
were one notice's list. They are not: nothing counts them and no
preamble introduces them, so there is no enclosing sentence for them to
be the items of — an unknown key, an unresolved theme name and an
unresolved preset name are separate pieces of news that happen to share
a subject. Neither hold above was available to them either. Three of
`prefs::Notice`'s four arms write a `LIST_SEPARATOR` inside one
sentence, so no claim about the sentences holds; and two of the four
echo a TOML key straight out of the user's file, which may contain any
character, so no second mark could have been out of band. So
`frame::startup_notices` builds one `frame::Message` per notice and
joins them with `frame::Message::joined`, which puts the startup line
under the same hold as a frame's: the boundary is the bullet, the door
takes it out of every text that reaches it, and the line splits back
into the notices it was made from. The claim is
`a_startup_line_splits_back_into_the_preferences_notices_it_was_made_from`
and `a_startup_notice_echoing_a_key_that_holds_the_boundary_mark_still_splits_back`
(`crates/viewer/tests/frame_policy.rs`). The door still takes
`&[String]` from three types — `prefs::Notice`, which is what both the
file's own complaints and the theme and preset resolutions produce,
`prefs::PrefsError` and `prefs::StoreError` — because what holds the
line is the door each string passes through and not the type it arrived
as.

**What is NOT a consumer, and the test rather than the list.** Sharing
the spelling is not membership: `LIST_SEPARATOR` is `"; "`, two
characters any sentence may use. A site is a consumer only if the mark
separates the items of a list ONE `frame::Message` carries, introduced
by a counted preamble — the test the third consumer failed on, applied
forwards. Two sites in `crates/viewer/src` write those two characters
between items of their own and are NOT consumers by it, both because
they reach no `frame::Message` and nothing counts or introduces their
items: `seats::seat_line`'s panel label, whose doc comment carries the
argument, and `pane::create`'s mate-tool panel, which spells the same
line a second time inside a format string
(`work/vnews/mate-panel-hand-rolls-the-seat-line` is that duplication,
and a second copy is still not a consumer). **The list is disposed, not
swept**, and deliberately. Neither available pattern is the property:

    rg -n '"; "' crates/viewer/src

prints **3** — the constant, `seat_line`'s join and the doc comment
that argues about it — and misses the mate panel entirely, whose mark
is inside `"pick a: node {}; pick b: node {}"`; while

    rg -n '"[^"]*; ' crates/viewer/src

prints **43**, which is every sentence in the crate that uses a
semicolon. Between the two there is no pattern for *joins its own
items*, so what this section holds is the test, run against the two
sites it has been run against.

### The app driver, split for size

`app` is a driver, and a driver too large to read is still a driver.
`app.rs` keeps `ViewerApp`, `ViewerBehavior`, the frame loop,
`perform_batch`, `sync_scene`, `apply_status`, `Pane`,
`initial_layout` and the entry points; `pane::{viewport, features,
properties, create, view}` hold the `*_ui` functions that draw each
pane, one module per pane; `widgets` holds the free helpers over
`egui::Ui` that those panes share, and the one `app.rs` shares with them
(`widgets::message`, below); and `gpu` holds the wgpu viewport
renderer, which names `eframe::wgpu` and could not be a vocabulary
under any reading. The table above is where that roster is kept.

**Splitting a driver across modules does not make the pieces
vocabularies.** The test is a module's ROLE, not its size or its file:
each of these names `egui`, and `widgets::delete_button` takes a
`&DocSession` because the wording it draws is the session's own
answer. That is the driver side of the rule behaving normally. Reading
the `use` block still decides it — the check says what a module IS,
not merely whether it is a vocabulary.

`app.rs`'s header claim — *toolkit adaptation, and nothing else* — is
true of the file rather than a claim it has outgrown.

**Where a MESSAGE wraps is a question about the region, so it has one
home**: `widgets::message`, with `widgets::message_link` and
`widgets::message_toned` beside it. `egui::Ui::wrap_mode` answers from
the `Ui`'s own `egui::Style::wrap_mode` if something set one, else
`Extend` inside a grid, else the layout's — and nothing in this chrome
sets a style wrap mode, so a label's wrap is decided by the layout it
happens to be in. A whole sentence — a refusal, a fault, a check
finding, the status line — is not what either of the layout's two
answers is for: in an ordinary horizontal row it is laid out at
infinite width and drawn past the region's right-hand edge, and in the
toolbar's wrapping row every line after the first is placed at the
panel's left edge, which for a top panel is the window's.
`widgets::message` lays the sentence out at the region's own width and
hands it over already laid out, which is the one path egui neither
extends nor re-places; it asks for the wrap explicitly, so a future
context-wide `Style::wrap_mode` would move every other label in the
chrome and leave a message where it is. `widgets::message_toned` adds
the voice, through `app::toned`, so what `Advisory` looks like stays
decided in one place.

The region is taken down to a floor and no further:
`widgets::message_floor` is the widest number `readout::number`
returns (`widgets::widest_number`, over `readout::widest_render`) and
one space after it. Below the floor a message stops narrowing and the
pane's `ScrollArea::both()` scrolls, and a line never breaks inside a
number `readout` renders. `widgets::message`'s doc says why the space
is there and what the floor does not cover, and states the rule for
which texts are bounded by characters and which by their region.

Its call sites are `app.rs`, `pane/create.rs`, `pane/features.rs`,
`pane/profile.rs`, `pane/properties.rs` and `pane/view.rs` — a roster
this page states twice (here and in the module table above) and
therefore does not keep by hand:
`widgets::roster_tests::the_message_roster_is_what_the_crate_actually_calls`
re-derives it from the crate's own source.
`widgets::message_tests` holds the measurements — that the sentence
fills the region it is in rather than a width of its own, across three
region widths; that below the floor it stops narrowing and never breaks
inside a rendered number; that egui's own scroll container hands its content the
visible width rather than an infinite one, so a pane that scrolls both
ways still wraps its sentences instead of answering with a scrollbar —
and `app`'s `the_toolbars_status_line_wraps_under_itself_rather_than_at_the_windows_edge`
measures the status line in the real toolbar, which is where the
second symptom was reported.

**Startup is split by what it needs, and the two context-wide styles
are on the deviceless side.** `ViewerApp::new` takes an
`eframe::CreationContext` and does one thing with the device — building
the viewport renderer into the frame's render state — and
`ViewerApp::assemble` is everything else: the document, its evaluation
and tessellation, the camera, the preferences, and the two styles those
preferences set on the `egui::Context`. The resolved palette's polarity
is stated before anything is drawn, so a window cannot open on one
ground and turn over to the other a frame later; the chrome's numeric
rule goes onto the context's styles, so a field that never reached
`widgets::number_field` still says what it holds. **Each is held by a
row of `app`'s own** —
`startup_states_the_resolved_polarity_on_the_context` and
`startup_installs_the_number_rule_onto_both_of_the_contexts_styles`
— reading the context after `assemble` and before any frame, which is
where the installs claim to be in force. Both reads
are behavioural: a `NumberFormatter` compares by `Arc::ptr_eq`, and a
polarity is read as the preference the context states and the
`dark_mode` a first frame would paint. **The population is two because
the context reaches nothing else** — `assemble`'s `egui::Context`
parameter is used at those two calls and at no third — so the sweep
that would find a third install is a grep for that parameter. What the
device half installs is held by nothing here and cannot be: a render
state wants an adapter, which is the same wall
`gpu`'s `every_pass_builds_on_a_real_device` stands at.

Two items move out of `app` to modules that already own their
subject rather than to new ones: `datum_view` to `datums`, and
`heading` to `sketch` — both are geometry over values the receiving
module already defines, and neither names `egui`.

### What a vocabulary reads, it is handed

`pickcache` and `parts` each took a `&DocSession` as a read-only argument —
`PickCache::sync`, `PartChooser::opened` and `PartChooser::rescan` —
which made the rule above false of the tree at five sites, and false
before `viewer-module-kinds.sh` existed to find them. Ev ruled
(`#1883`) to **hoist the read**, not to widen the rule: the session
mints `pickcache::IndexInputs` (the landed pair, its generation, ε) and
`parts::PartCensus` (the directory scanned and what the scan answered),
and the two vocabularies take those. *No vocabulary may name a driver*
stays unqualified.

**The reason generalises and is worth more than the answer, and this
is its one home** — the values point here rather than restating it.
The two branches are not symmetric in reversibility. Hoisting keeps widening
available: if the values turn out to be a bad trade, the rule can still
be widened. Widening first does not keep hoisting available — the
clause gets relied on, and by the time anyone wants it back there is a
set of sites written against it. *"Easy to switch to b later and hard
to do the reverse"* is the test for any fork between a strict rule and
a rule with a clause, and this is its worked instance.

What it costs is a value per reader and the derivation moving into the
driver, which is the direction the boundary already allows: a driver
may name a vocabulary. What it buys, besides the rule, is that the
reads are now stated — `IndexInputs`'s four fields are read together
because they are SET together, which the old three-accessor
destructuring spelled out by hand at the call site.

The gate's exception machinery stays, empty. An entry is
`FILE|NEEDLE|COUNT` and is site-granular, so it cannot outlive its
reason: fixing a site without lowering the count reds, which is how
these two retired in the same PR as the seam they described. That
granularity was offered as evidence to `work/code-quality/D103.md`
(*"the allowlist is file-granular while its justifications are
per-seam, so later bounds inherit ratification"*). **D103 is unruled**,
and the entries retiring is more for it to weigh rather than a
withdrawal of the offer: a per-seam entry ended when its seam did, in
the same change, which a file-granular one would not have. That
argument is made where the entries were deleted, in
`scripts/gates/viewer-module-kinds.sh`.

### The seam modules are a chain; the crate is not acyclic

The rule above is about what a module NAMES and says nothing about
cycles between vocabularies, so a cycle here breaks no clause — and
`evalseam` and `pickcache` held one anyway, because the index seam's payload
and the policy that drives the seam were the same file. **Neither of
the obvious repairs reaches it**: a third module for the index seam
relocates the cycle (`IndexDone` carries a `PickIndex`), and hoisting
the seam's request and answer types does the same. What the cycle is a
symptom of is that one file held two layers with the seam running
between them.

So the modules are a chain, each naming only what is below it:

    generation  ←  pickindex  ←  evalseam  ←  pickcache

- `generation` is `Generation` and nothing else, depending on
  nothing. It is a request counter, not part of either seam's
  machinery, and six modules compare one;
- `pickindex` is the index and every query over it — the structure a
  build produces;
- `evalseam` keeps EVERY seam and therefore **every one of its
  threads**,
  which is the property that made this shape win: *the one place in
  this crate that owns a thread* stays one sentence;
- `pickcache` is the index's LIFECYCLE over the seam — what a build is
  handed, when one is asked for, and what a pick means while there is
  none.

The two moves only work together. `Generation` alone leaves
`PickIndex` beside `PickCache`, so `evalseam → pickcache → evalseam`
survives on the seam types; the split alone leaves `pickindex` needing
`Generation` from `evalseam`, which needs `pickindex`. (Ev, 2026-09-06.)

**The chain above is four modules, and the crate around them still
holds a ring** — said here because a picture of a chain is exactly the
sentence that stops the next reader looking:

    pickcache.rs:61   use crate::pickindex::{PickIndex, PickIndexError}
    pickindex.rs:81   use crate::session::{…, SessionOp}
    session.rs:78     use crate::pickcache       (for `IndexInputs`)

`pickcache → pickindex → session → pickcache` is live, it predates the
seam split — at that split's merge base the same ring was two modules
long, this file naming `session` and `session` naming it — and it is
held open **on purpose**, by the hoist argued for in *What a vocabulary reads, it is
handed* below: the session mints `pickcache::IndexInputs` so that the
vocabulary names no driver, and the price of not widening the rule is
that the driver's own module names the vocabulary back.

So the two rings are different diagnoses and only the first is fixed
here. `evalseam ↔ pickcache` was **one file holding two layers** with a seam
running between them, which no placement of the seam could repair —
that is what this section is about. `pickcache ↔ session` is **a vocabulary
and its driver trading a minted value**, which is the boundary rule
working rather than failing. Nothing in this section generalises to the
second, and `work/view/seam-split-leaves-a-cycle-through-the-session`
is where the question of whether it should be broken at all is kept.

### A pick id is one index's word

`PickIndex` holds an `IdMap` keyed by a `PictureKey` — the landed
generation and the δ its roots were tessellated at, one value because
it is one question — and every id in the drawn mesh's per-corner `ids`
was minted by the id map of the index that built it. So an id is only a name in the alphabet of the
index that minted it, and reading one through another index resolves it
to whatever that index happens to keep at the same number.

**The index in hand is not always the index on screen.**
`ViewerApp::sync_scene` marks the scene's `PictureKey` current
only on a successful rebuild — a refused one must not consume the pair,
or the stale picture stays marked as the current one and is never
retried — so a landed index over a refused rebuild leaves a newer index
beside an older picture, and nothing retries it while the display
revision and the focus set hold still. The startup mesh is the same
shape from the other end: `scene::scene_of` builds it before any index
exists and every corner carries `IdMap::NOTHING`.

**So a pane sorts its reads of the index by what they are about**, and
the sorting is a rule about currency rather than about which fields
happen to be in hand:

- A read about the **document** — what is under this cursor, what does
  a click mean — takes the index with the session's evaluation, because
  that is what resolves a ray into a face — `PickIndex::op_under` in
  the viewport, `BlendTool::load_all_edges` behind the create pane's
  all-edges button.
- A read about the **picture** — an id the id pass produced, or a mark
  the shader composites against the drawn corners — goes through
  `drawn_index`, which answers `None` unless the index in hand is the
  one whose id map minted those corners. The whole key is asked,
  through `PickIndex::current_for`, which takes a `PictureKey` and
  nothing smaller: a δ typed while the document stands rebuilds the
  index at the same generation over a different tessellation, so
  generations alone would read as co-identity while checking something
  else — and a door that takes one value cannot be handed one half of
  it.
- A read of the index's **identity alone** — `PickIndex::generation` as
  half the id query's key — resolves nothing and needs neither. It is
  the one read that wants less than a picture, and it says so by
  reaching past `PickIndex::key` for a named half rather than by
  comparing one.

**The pick asks `drawn_index` too, and for a different reason.** The
sorting above is about currency, and nothing about a pick is false by
construction across two pictures: a click resolves a ray through the
index and the evaluation with no id and no mesh in sight, and would
answer correctly about the document. What it would answer about is
geometry the screen is not showing, and a selection the user cannot see
is a worse outcome than a click that says why it did nothing — so **a
pick over a picture the index in hand did not draw is refused** (Ev,
2026-09-15). The predicate is the picture-side one, unchanged; what
differs is what happens on `None`. A picture-side read skips silently,
because a mark nobody can draw is nothing to say. The pick path refuses
**typed**, on the status line, because a click is an act the user made
and got nothing for — `pickcache::NotIndexed::AnotherPicture`, the one
arm of that vocabulary that is not about an absence, beside the two
that are. The create pane's all-edges button is NOT covered: it is a
button in a panel rather than a cursor over the picture, so the ruling's
premise — an answer about what the screen is not showing — is not made
there.

**What retires that refusal is a scene rebuild**, where the other two
arms wait on an index build. Both seams sit under `Subject::Display`,
so the subject `frame::unindexed_refusal` reads off the type is right
for all three arms; the arm's own doc says which event it is waiting
for, so a later split of that subject has the fact it would need.

**The id query's key is the picture AND the index**, which is the same
rule met from the other side. `idpass::IdQueryLog` holds a query open
while its answer still describes the cursor, and that answer is an id
the GPU read out of one picture, resolved through one index's id map —
so `idpass::IdSubject` carries both halves, `ViewerApp::revision` for the
picture and the index's generation for the alphabet. **Neither half
subsumes the other.** `sync_scene` rebuilds on a display-revision or
focus-set change at a standing generation, so hiding a part draws ids
the generation cannot distinguish from the ones before it; and a rebuild
`sync_scene` REFUSES does not bump the revision, so an index that landed
over one is a new generation beside the picture already on screen. A key
carrying one half holds a question that should be re-asked, and a held
query keeps the last answer MATCHED — so `idpass::disagreement` finds a
fresh ray answer against a GPU answer about a different picture and
reports it as *the two picking paths disagree*, which issue #1097 §4
tells an operator to read as an `R32Uint` clear fault.

**What produced the rule.** The population is *a site that uses the
`&PickIndex` a pane was handed*, and there are **eight**: five about the
picture, two about the document, one the identity. It is derived in two
steps, because neither alone produces it. `ViewerBehavior::index` is a
field, so `self.index` finds every place a pane takes one — four
bindings, in `pane::viewport` and `pane::create`, and a pane that grew a
fifth would appear there. It does **not** find the uses: the five
picture-side ones read a binding called `on_screen`, and a name is not
a pattern, so each binding's scope is read in order instead.

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
arms, so a new arm is compiler-caught. It is exhaustive one level down
too, on the one arm whose rank is a per-payload decision: `Display`
walks `DisplayFault` arm by arm and walks the admission family inside
it, so a new fault of either kind reds until its rank is chosen.
`Edit` and `SlotUnit` forward whole vocabularies at one rank each and
that IS a default, argued at the arm. The remaining cost — an arm
ranked wrongly — is accepted, because the alternative of deriving a
rank from the arm's shape would make the ordering unstateable, and the
ordering is the part users see.

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

### The G1 machine is held once

Two gestures implement G1's preview/commit shape — the value drag
`DocSession` owns over a slot or a document parameter, and the
free-move probe `DisplayState` owns over an instance's frame — and
their three transition rules are one value, `g1::Slot`:

- a **begin** refuses when one is already in flight, and validates its
  target only once the slot is known free;
- a **preview** REPLACES the value in flight rather than composing with
  it, and refuses when nothing is in flight or when the operation names
  another gesture;
- a **commit** lands exactly one value, and a gesture that never
  previewed lands nothing.

**They are not one type and this is not a step toward making them
one.** They own different value kinds (a `SlotValue` against a
`Frame`), different validation (a slot's driver and dimension against a
rigid-motion check on an unmated instance) and different side effects
(a scratch `Doc` and an evaluation request against a display revision).
What is shared is the transitions, and a generic over the rest would be
a type nobody has a use for. DI5 changes what a probe's commit LANDS
(`crates/editor-core/IDENTITY.md`: a `DocEdit::SetPlacement` rather
than a `moves` entry) and changes none of the three rules, which is why
holding them once did not wait for it — after DI5 the landing step that
moves is the caller's, and the machine it must not break is one
function rather than two.

**What the shape makes impossible**: `g1::Slot`'s in-flight state is
private to its module, so no caller can read it, take it or replace it
except through `begin`, `preview`, `commit`, `cancel` and `discard`. A
rule about the transitions cannot be spelled anywhere else, so one
cannot be fixed in one gesture and left broken in the other. **What it
does not do** is make a NEW shared rule land there rather than in both
callers: the closures each door takes are the caller's own, and a rule
written inside one of those is written for one gesture.

**The vocabularies stay apart, and are declared once each.**
`session::gesture_words` says `NoGesture` / `GestureInFlight` /
`WrongGesture` and `display::free_move_words` says `NoFreeMove` /
`FreeMoveInFlight` / `WrongFreeMove`; `g1::Refusals` is the struct they
are handed in as, named rather than positional because three arguments
of one type sit one transposition away from a door that says *finish
the drag first* where it means *no drag is in progress*. They are
different vocabularies about different subjects and holding the rules
once is not a reason to merge them.

The precedent is `widgets::drag_ops`, one layer up: one mapping from a
`DragValue` to the four operations, over both vocabularies, and its own
doc says what the two hand-written copies before it cost. This layer
had the same two copies and no such guard;
`tests/gesture_table.rs`'s `the_value_drag_answers_the_three_shared_rules`
and `the_free_move_probe_answers_the_three_shared_rules` drive one
script through both.

### Gesture safety is data

The mid-gesture policy is one exhaustive value,
`SessionOp::permitted_during_value_gesture`, checked once in `perform`
before dispatch: 24 operations refuse while a value gesture is open and
17 are permitted. A forty-second operation cannot be added without
answering for it, and the whole policy is readable in one place rather
than inferred from every dispatch target.

**What the table does not decide is rule 1.** A begin that arrives
under an open gesture is refused by that gesture's own door —
`g1::Slot::begin`, reached through `DocSession::start` for the value
drag and `DisplayState::begin_free_move` for the probe — so
`BeginGesture` and `BeginParamGesture` are permitted by this table and
refused anyway, one layer down, with the same `GestureInFlight` a row
here would raise off the same state. A row would be a second spelling
of one answer and would leave the door's own arm unreachable through
`perform`. The set of operations a value drag refuses is therefore this
table plus that one rule, and the rule is held once for both drags
rather than per gesture and per table.

It says nothing about the free-move gesture, which is a different value
with a different owner (`display::DisplayState`) and has a table of its
own, `SessionOp::permitted_during_free_move`. The name carries that
limit: a predicate reading as a general guarantee would be a table that
looks complete and is not. The two fields are spelled apart for the
same reason — `DocSession` holds `gesture` and `DisplayState` holds
`free_move` — so a reader who greps `self.gesture` gets one concept
back. What the value table's four `*FreeMove` rows permit, and the
identity that makes the overlap sound, is stated at
`permitted_during_value_gesture` itself, scoped to the tree DI5 has not
yet changed.

**The second table refuses two rows and has a name for them**:
`Open` and `NewDocument`, the operations that REPLACE the document
rather than move it. The asymmetry with the first table is the point —
a commit that lands under a probe is pruned and reported, and only a
replacement drops the display state whole with no document left to
report against (`DisplayState::clear` carries that argument). So the
free-move table is checked against the property rather than against a
second copy of 41 rows (`replaces_the_document`, in
`tests/gesture_table.rs`), and `perform` consults both tables in turn,
value gesture first.

`BeginFreeMove` is permitted by both tables and refused anyway, one
layer down: `DisplayState::begin_free_move` answers a second begin off
its own state with the same `FreeMoveInFlight`, through `g1::Slot`'s
first rule. A row in the table
would be a second spelling of one answer, and the test that exercises
the doors says so rather than smoothing it over. The value table's two
begins say the same about the other drag, so this is one rule about
rule 1 rather than one table's exception.

The table records behaviour rather than deciding it — `save` is
permitted mid-gesture and `open` is refused, which is what the code did
before the table existed. Whether that asymmetry is right is a separate
question with its own item.

### A driving operation names its own gesture

`PreviewGesture`, `CommitGesture`, `PreviewParamGesture`,
`CommitParamGesture`, `PreviewFreeMove` and `CommitFreeMove` each carry
the target they are driving, and each is refused when that is not the
gesture in flight — `Refusal::WrongGesture` for the value drag,
`DisplayFault::WrongFreeMove` for the probe, raised where the gesture's
own state lives.

The chrome emits a drag as a triple (`widgets::drag_gesture_ops`) and a
second drag's BEGIN is the only member of it the mid-gesture tables
refuse: the other two drive the gesture and a table that refused them
would leave every drag with no way to end. So without the target in the
payload, a field whose begin was refused still previews and still
commits — into whichever gesture happens to be open, with the new
field's number. The subject of a driving operation is the field the
user has hold of, and naming it is what makes the mismatch refusable.

**The probe's target is the instance, and that is the same rule rather
than a coarser one.** `PreviewFreeMove` names an instance because an
instance has exactly one probe, the way `PreviewGesture` names a slot
because a slot has exactly one drag. Each names the SUBJECT whose
gesture it drives, and how fine that subject is follows from what state
exists, not from how finely a chrome cuts the subject up: the three
millimetre boxes the panel draws are one chrome's decomposition of one
frame, and the op takes any rigid `Frame`. Mapped a triple per box
those boxes are three gestures over one probe and the payload cannot
separate them, both naming the same instance: the second box's begin is
refused `FreeMoveInFlight`, its preview then overwrites the first's
frame, and its commit lands it and CLOSES the probe the pointer is
still holding — a refusal describing a state its own batch destroyed.
The keyboard is what reaches that second box while the pointer holds
the first, because a `DragValue` enters edit mode the frame it takes
focus. So the row is mapped ONCE, as a row (`widgets::vec3_row_ops`),
and a keystroke on a sibling box is another hand on the open gesture
rather than a second gesture.

**The two cancels are the exception and name nothing**: their subject
is the session's state, because the state they exist for is a drag
whose field is no longer drawn (the cancel-door section below).

**A target, not a token — so a second DRIVER on one subject is not
refused, in either drag.** A gesture could instead be named by a handle
its begin mints, and the difference shows wherever a gesture outlives
the field it is being driven from. The reader's recovery there is to
drive that field again, which is a whole begin/preview/commit batch on
a gesture that is already open: the begin is refused, and the preview
and the commit are not, because they name the gesture that IS open. So
they land the value the user drove it to and end it. A token minted per
begin would refuse them and strand the reader a second time. Each drag
has its row.

For the value drag the undrawn field is a stranded drag's own
(`the_open_drags_own_field_dragged_again_lands_its_number`). The
identity that matters is *which field*, and nothing that moves the
document is permitted mid-drag, so the second drag's base document is
the first's.

For the probe it is the SELECTION that takes the field away
(`the_open_probes_own_instance_driven_again_lands_its_frame`), not an
evaluation: `SessionOp::Select` is permitted mid-probe and
`pane::properties`' `instance_ui` draws the probe row for
`selection().node()` alone, so selecting another feature under an open
probe leaves the drag live with nothing drawing it. The hand that
reaches it is the one that reaches a sibling box — the feature tree's
row is a `selectable_label(…).clicked()` (`pane/features.rs:59-60`),
and egui answers `clicked()` for a focused widget's Space/Enter and
for an AccessKit `Action::Click` with no pointer anywhere. Select the
instance again and drag a box, and the batch lands the frame and ends
the probe.

**One gesture per subject, driven by whoever names it**, is therefore
the rule the target spells, not a gap left in it. A door that refused a
second driver would need driver identity, which is the token — and the
token is what these two rows refuse.

**The table says what it says.** `permitted_during_value_gesture` is a
function of the operation alone, so it cannot answer a question about a
payload; the name check is `g1::Slot`'s, run against a predicate each of the
four doors — `DocSession::preview_gesture` / `commit_gesture` and
`DisplayState::preview_free_move` / `commit_free_move` — supplies for
its own subject, and the refusals are spelled apart from
`GestureInFlight` so the table's answer stays readable from the
outcome.

### Every gesture has a cancel door

A gesture's ordinary exit is the release event on the field that opened
it, and that exit exists only on a frame the field is drawn. The panel
is handed no slot row for a selection whose standing is not live, so a
slot drag whose own preview lands an evaluation its picked face does
not survive loses its only door under the pointer still holding it:
nothing reports the release, the drag stays open with no pointer behind
it, and `Refusal::GestureInFlight` — *"finish the drag first"* — then
answers every operation that moves the document. A refusal naming a
remedy that does not exist is the honesty rule inverted, and it was the
state of both gestures: `SessionOp::CancelGesture` and
`SessionOp::CancelFreeMove` each had an arm in `perform` and no emitter
anywhere in the crate.

**Every operation that cancels a gesture is a control in the toolbar.**
The population is the operations that cancel a GESTURE, not the
variants spelled `Cancel`: `CancelEvaluation` cancels a run and its
control is the one beside the spinner that reports the run. So the
sweep rule is a match over `SessionOp` the compiler completes —
`gesture_table.rs`'s `cancels_a_gesture`, checked against
`DocSession::cancel_doors` in both directions — and a third gesture
cannot join the enum with no door. What that rule cannot see is whether
the toolbar's read of `cancel_doors` is REACHED, because the control is
an `egui` closure: the emitter count is held against returning to zero
by `the_cancel_doors_have_a_reader_in_the_chrome`, not by a row that
draws the button.

**The doors are in the toolbar and not on the field**, which is the one
siting that answers the defect: the field is what can stop being drawn,
so a cancel beside it would vanish with the exit it replaces. And **a
door that cannot act says so rather than vanishing** — the posture the
two file-dialog controls take: each door is drawn whatever the
selection, the standing and the evaluation are, and enabled exactly
while its own gesture is in flight. **And reachable at every window
width**, which is a second claim and has its own holds: the toolbar is
laid out `horizontal_wrapped`, so a window narrower than the row gets a
second line rather than a clipped one — a clipped control is not
small, it is gone, and for these two that would mean no exit from the
gesture at all. `ViewerApp::toolbar_ui`'s two rows hold both halves: that the
row does not fit a narrow window, so the wrapping is answering
something, and that it stays inside one.

**How it says so is a different precedent from where it is drawn**, and
citing one for both is wrong: the dialog controls hand
`platform::NO_CHOOSER_BACKEND`, a `&'static str` composed at each button,
to `on_disabled_hover_text` — the shape
`environmental-facts-answer-usable-as-a-bool-with-the-reason-elsewhere`
is open about. A cancel door follows `pane::create`'s catalogue entry
instead — *carrying the op's own refusal, read off the entry, not
minted here* — so `CancelDoor::blocked` is a `Refusal` and the disabled
control's words are the refused operation's own. **That the two cannot
come to say different things is held by a test, not by the type**:
`gesture_table.rs`'s `a_closed_door_says_what_its_own_operation_refuses`
performs each door's operation with no gesture open and compares both
the rendering and the variant. Nothing structural stops a future door
composing its own sentence; that row is what would red.

**The door cannot be reached during a live pointer drag**, which is the
half of the defect it does not answer: it is a toolbar control, and
pressing a toolbar button costs the pointer release that lands the
value. Enabled and unreachable is the state a drag under the pointer
leaves it in, so the door is the STRANDED drag's exit and not the held
drag's.

**The held drag's exit is Escape, and it is read rather than bound.**
`egui` aborts a drag on Escape and on nothing else, by clearing the
dragged widget — so the abort arrives at `widgets::drag_gesture_ops` as
an ordinary `drag_stopped` frame, and the chrome that does not read the
key reports an abandonment as a release. That branch is the whole of
what this crate reads a key for: which of the two things the toolkit
already did is being reported. It decides no key→operation vocabulary
(`input::PRESETS` states what one would have to decide first), and it
is read directly from the key rather than inferred from the absence of
a pointer release, because a long touch also ends a drag with no
release and means something else.

A drag has two ends and they mean opposite things, so the emitted
operation is a parameter of the mapping beside the commit — and a
`SessionOp` rather than an `Option`, because every gesture vocabulary
has a cancel and `every_gesture_cancel_has_a_chrome_door` is the
exhaustive match that keeps it so. Both drags the panel maps run
through that one function, so both ends are the same rule at the slot
field, the parameter field and the free-move probe; the stake is
largest at the first two, where a commit reaches the document and costs
an undo step.

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
are its only readers: `Tools::open_kind` asks the open value which kind
it is instead of scanning the list for the first field that is set, and
the chrome names each kind it offers literally rather than iterating. A
kind missing from `ALL` would narrow those sweeps rather than make its
tool unreachable — and a kind cannot be missing from it, because `ALL`
is projected from `ToolKind`'s own declaration (**Closed vocabularies
are declared once**, below). `ToolKind::ordinal` and the row that read
it against the list are gone with the hand-written list they existed to
check. `ALL` stays `pub`: the suites that read it are integration
tests, which see only this crate's public surface, and a suite-local
copy would be the hand-written list again with nothing forcing it.

### Closed vocabularies are declared once

**Eleven** enums here are closed vocabularies: a fixed set of choices the
chrome offers, which something walks in order — a radio row, a combo's
options, a suite's sweep. Each carried a hand-written `const ALL`
beside it, and that second copy of the membership was free to fall
behind the first: adding a variant compiled, the radio row silently
lost a button, and every sweep keyed on the list quietly narrowed.
(At the conversion, ten `const ALL` tables existed under `src/` and
nine were of this kind. The tenth vocabulary is `frame::WithdrawalKind`
and it is not one of those ten: it carried no membership list at all
until the fan-out from a `PruneReport` needed holding to it, and the
list it got was projected rather than written. The eleventh is
`marks::EdgeLane`, a renderer's draw order rather than a choice the
chrome offers, declared through the macro from the start. Both censuses are stated
because this program's counts have gone wrong before — one is the tree
at the conversion, the other is the vocabularies today, and nothing
makes them the same number.)

**The enum and its `ALL` are now one declaration.** `src/vocab.rs`'s
`vocabulary!` takes one list of variants and expands it into both, so a
variant cannot reach the enum without reaching the list — the same
construction `crates/profile/src/path/program.rs` uses for the arc-mode
and verb vocabularies ("ONE declaration, THREE projections"), and no
new dependency in a crate whose default-feature graph is deliberately
the kernel's. Order is the declaration's, because these lists are read
in order and several say so in their own docs; where a form wants an
order the type did not grow in, the **enum** is written in the form's
order and says why.

**Two shapes, and the test that says which.** A LABELLED vocabulary
writes each variant's word in the declaration and projects
`[(Self, &'static str); N]`; a BARE one projects `[Self; N]` and keeps
its wording in a `label`/`name` method beside it. The test is neither
taste nor a count of readers: **does anything walk the table for its
WORDS?** If something does, the words are table data — they belong in
the declaration, and the walk reads each entry's word off the entry it
already holds. If nothing does, they are not table data at all and a
method beside the enum is the whole of it.

**The sweep that produces the population** is a walk of every loop over
a vocabulary's `ALL` — one of the nine declared by `vocabulary!`, so a
loop over `Theme::ALL`, `pncad`'s `Axis3::ALL` or the path form's
`profile::Verb::ALL` is outside it — read
for what the loop asks each entry for. It reads `src/` **and**
`tests/`, because the discriminator is about the words and a word read
in a suite is still a word read off the table; a sweep scoped to `src/`
would have nothing to discriminate on the two vocabularies it rules
bare, and the first tests-only word-walk would arrive unseen.

**Five of the nine are walked under `src/` for their words, and all
five ask for one.** Each binds `(value, label)` and puts that label on the control
it draws: `pane::create`'s datum row, profile row, pattern-rule row,
pattern-output row and blend-kind row. So all five are LABELLED, and
there is no shorter account of them than the sweep itself: their words
are table data because a table walk reads them.

**`ToolKind`, `Seat`, `WithdrawalKind` and `marks::EdgeLane` are the
remaining four, and are BARE.** `EdgeLane`'s list is named `DRAW_ORDER`
rather than `ALL`, because its order is the edge pass's priority; `gpu`
walks it to lay out the vertex buffer and the per-lane style rows, and
reads no word off it. `WithdrawalKind` is walked under `src/` and is
bare anyway,
which is the discriminator doing its job rather than an exception to
it: `frame`'s own `every_withdrawal_kind_has_a_producer` compares the
KINDS `Withdrawal::all` produced against the list, and reads no word
off it. A withdrawal's sentence is composed by `Withdrawal`'s `Display`
from the kind and the faults under it — a count, a consequence clause
and each cause's own `Display` — so the words were never table data to
begin with. The other two are bare because
no loop under `src/` walks their lists at all: a kind's word appears
inside a sentence `tools` composes, and a seat's inside the refusal
sentences `seats` and `session::refuse` write — wording read against
that discipline rather than against a row of buttons. The suites do
walk both lists, and walk them for the VALUES: `tests/combine_ops.rs`
maps kinds to booleans and drives one op per seat, where the seat's
word reaches only an assertion message about the single seat that
failed. A walk that would still do its job if the words did not exist
is not a reader of them. The suites also walk two ALREADY-labelled
lists for their words (`tests/combine_ops.rs`'s pattern-output row,
`tests/blend_authoring.rs`' blend-kind row), which confirms the shape
those two already have rather than deciding it — and is the only thing
the `tests/` half of the scope has yet had to report.

**Neither shape holds a second ordered list of the words.** A labelled
vocabulary's `ALL` carries each word beside its variant, so the order
and the reading are each declared once; a bare one
carries no word in its table, and its method is the only place its
words are written.

**What the macro cannot express**, stated because it is a one-way
door: fieldless variants only, and no explicit discriminants — the
labelled arm spends `= …` on the word, so `#[repr]` numbering means
un-converting the enum. `src/vocab.rs`'s own doc carries both, and the
rustfmt cost below.

**rustfmt does not reach inside the invocation**, so the variants and
variant docs of all nine are formatted by hand. Demonstrated rather
than assumed, and not fixable by making the body parse: `src/vocab.rs`
records the experiment and
`work/view/vocabulary-macro-bodies-are-outside-rustfmt.md` tracks it.

Two kinds of list stay hand-written, and each is a different answer
rather than an exception:

- **A registry of struct constants** (`Theme::ALL`) is not an
  enumeration of variants at all — there is no exhaustiveness for a
  match to borrow, and its doc already argues it is the single list.
- **A deliberately partial list** claims no completeness, so forcing it
  would force the wrong thing: `SUBJECTS_WITH_AN_EXPIRY_ISSUER` names
  two of five `Subject`s, each tool's seat list names its own seats,
  and `MATE_PRIMITIVES` offers three of four mate primitives because
  the fourth exists to be refused. Each says why in its own doc.

**A partial MIRROR is told when the enum it mirrors grows**, which is
the weaker thing that is true of it and the whole of what a mechanism
may force here. `src/vocab.rs`'s `partial_mirror!` holds a roster
classifying every variant of the mirrored enum as offered or as
deliberately absent WITH ITS REASON, over a match with no wildcard: a
variant added to that enum is neither until someone writes one of the
two, and the build says so. Three sites take it — `MATE_PRIMITIVES`
over `MatePrimitive`, `SUBJECTS_WITH_AN_EXPIRY_ISSUER` over `Subject`,
and `forms::DatumKindChoice` over `session::DatumSpec` — in two shapes,
because what a site OFFERS decides whether a seat of it can drift: a
hand-written list gets a per-seat assertion and a count check, while an
enum whose `ALL` is projected has no second copy of its membership to
hold, so its roster names a counterpart instead. A tool's seat list
takes neither and is not a mirror: it SPECIFIES that tool rather than
tracking `Seat`'s membership, so a new seat no tool asked for is
absent from it correctly.

A list that mirrors a vocabulary ANOTHER crate owns is not a third
kind, and the boolean form is why: **a mirror claiming completeness is
held by something the owner declares, never by hand.** What the owner
publishes decides the shape, and two shapes are in the tree.
`topo::BooleanOp` publishes its own `ALL` beside the declaration, so
this crate maps over it and writes only the words an op needs at an
exhaustive match. `egui::PointerButton` publishes a count and no list,
so `pane/viewport.rs` sizes its array by `NUM_POINTER_BUTTONS`, holds
the entries pairwise distinct, and converts through an exhaustive
match — length, distinctness and totality together leaving exactly one
permutation of the enum. Different declarations, the same guarantee.
A mirror that can point at neither is what this refuses; what stays
here is the partial case above, which wants no such list.

The second shape is held by the compiler rather than by the gate below
— a `fn` returning a sized array is no `const` item — so it earns no
allowlist row. That is the guarantee being stronger than the gate's,
not an exemption from it.

**A gate holds this, and the table below is its allowlist.**
`scripts/gates/viewer-vocab-declared-once.sh` scans `crates/viewer/src`
for a hand-written membership list in either of two shapes — a `const
ALL`, and any `const` array literal of two or more `Type::Variant`
entries, which is the same list under a different word — and reds on
one the table does not carry. `static` opens an item in both arms, for
the same reason the second shape exists. A converted vocabulary is not
a hit: `vocabulary!`'s `pub const ALL;` declares no array literal, so
the nine are quiet without an entry. What the gate reads is this
section rather than a list of its own: the ROWS below are the
allowlist, and the KINDS they may claim are the bullets of the
two-kinds list above — the list the sentence *"Two kinds of list
stay hand-written"* announces, and no other. That sentence is matched
only where it OPENS a paragraph, so quoting it in prose, as this one
just did, is a mention and not a second announcement. Both halves are
read only WITHIN this section, so the roster cannot drift onto another
page's heading and go on being read.

The kinds are read only under that sentence, and that scope is
load-bearing in the other direction too: **the prose in this section
may carry bulleted lists like any other prose, with one exception.** A
bulleted list that is the NEXT thing after the ratified list, separated
from it by nothing but blank lines, is not a second list at all:
CommonMark makes the two ONE loose list, every renderer draws them as
one, and the gate reads its items as ratified kinds and reds. The rule
that yields the exception is the renderer's rather than the gate's, and
it is the rule for finding the population too — anything that closes
the ratified list first, a paragraph or a heading or a table, makes the
next bulleted list a separate one, and a bolded bullet anywhere else in
this section is not a ratified kind. Indentation does not separate two
lists either: a marker up to three spaces in is an item of the SAME
list to a renderer and to the gate, and at four spaces it is the
bullet's own nested content, or the paragraph's, or a code block.

The gate pins the announcing sentence and carries the NUMBER of bullets
as its own constant, and it refuses those two to disagree — so a third
kind is an amendment argued here, rewording that sentence's number word
and adding a bullet, AND an edit to that file moving both of its copies
of the count, not a new word in a table cell.

The roster retires itself in both directions — a list added without a
row reds, and a row whose list has been converted reds too, because an
allowlist entry with nothing behind it is a ratification the next thing
written at that name inherits. It does not spread, either: **one row
ratifies one list.** Two rows for one list red, and so does one row for
a module that declares two lists under that name, because the row is
keyed on the module and the name and cannot say which of the two it
meant.

The gate runs in the `mirror` job, which carries no `if:`, because half
its subject is this page: a change set of only the README is TIER=docs,
and sited under `if: run_build` the arms that exist for an edit to this
table could not fire on an edit to this table.
`scripts/check-ci-mirror-parity.py`'s `TIER_BLIND` names it, so the
siting is enforced rather than remembered.

What it does not see is `crates/viewer/tests/`, deliberately: the
suites' hand-written variant lists are inline arrays in a row, not
`const` tables, so this scan would not find one if it looked —
`work/view/viewer-suites-hold-hand-written-complete-variant-lists.md`
is theirs. Nor does it see a list that is not a `const` or `static`
item, which is why each tool's seat list is named in the bullet above
rather than in the table. What it decides is that a list is
hand-written, never that the roster still says what was ratified here:
a bullet and the cells claiming it, reworded together, are consistent
and both green. The gate's own header states that blind spot; a review
is what covers it.

#### The lists that stay hand-written

| List | Module | Kind |
|---|---|---|
| `MATE_PRIMITIVES` | `forms` | A deliberately partial list |
| `SUBJECTS_WITH_AN_EXPIRY_ISSUER` | `frame` | A deliberately partial list |
| `Theme::ALL` | `theme` | A registry of struct constants |

**This table is the roster**, not a summary of one. `Module` is the
module the `const` is declared in, `List` is how it is written there
(an associated constant carries its type, `Theme::ALL`), and `Kind` is
the bullet of the two-kinds list above that ratifies it, word for
word.

The type in `List` is for a reader, not for the gate: what the gate
keys on is the module and the constant's own name, because an
associated constant's declaration says `[Self; 3]` and the type it
belongs to is the enclosing `impl` header, which only a parse would
find. That is why one row ratifying two same-named lists in one module
is a red rather than a silent second ratification — and why two lists
that a module and a name cannot tell apart need one of them moved or
renamed, not a second row.

### What the boundary does not decide

The rule says where things live. It does not say the wording family
has one shape. **The count below is over SHAPES, not over sites**, and
saying so is half the repair: the sentence used to read *composed six
ways across five modules* without stating what a *way* was, so a
reviewer meeting a new site could not tell whether it made seven or
was one more instance of a way already counted. A shape has no
syntactic marker and no grep produces the population, so the
enumeration rule is the list itself — **six shapes, composed in five
modules of this crate** — and a new site is a seventh member only when
its shape is not one of these:

- **A composer on the vocabulary type, one home, called by every
  surface that shows the sentence** — `Refusal::affordance`,
  `::exists_wording`, `::offer_wording` (`session::refuse`).
- **Composed in the vocabulary's own `Display`, riding the sentence**
  — `Refusal`'s `NoDocumentDirectory` arm, `FaceFrameFault`'s
  `NotOneBody` arm (`session::refuse`).
- **A named `&'static str` owned by the layer the fact belongs to and
  spent by more than one door** — `refuse::NO_FACE_PICKED` (spent at
  `FaceFrameFault`'s `NoFace` arm and by `forms`),
  `platform::NO_CHOOSER_BACKEND` (spent at two `on_disabled_hover_text`
  calls in `app`), and `editor_core::edit::UNDECLARED_PARAM_RECOURSE`,
  whose home is the crate that owns the fact.
- **A literal at the chrome site, composed where it is drawn** —
  `pane::create`'s *add a frame datum first*, `pane::profile`'s *its
  last step has to target the start*.
- **A reason accumulated as chrome-local data and rendered once** —
  `pane::create`'s `blocked: Option<&'static str>`.
- **Already stringified into the value and stored** —
  `session::AtRestBadge::Refused`, whose `message` `frame::at_rest_badge`
  renders unaltered.

The five modules are the ones a shape is COMPOSED in —
`session::refuse`, `platform`, `pane::create`, `pane::profile` and
`session` — not the ones that spend one: `app`, `forms` and `frame`
each render a sentence composed elsewhere and add no shape.
`AtRestBadge` used to sit OUTSIDE this count, named beside it as a
separate fact; it is a shape like the other five and is counted here,
so the population moved even where the digit did not. Naming ONE shape
for the whole family is still a separate question, and the move above
neither answers nor forecloses it.

## Rustdoc posture: the host all-features pass is the link gate

**At the browser target every link into host-only code is unresolvable
BY CONSTRUCTION, so a lint that cannot tell that from a broken link is
the wrong instrument for a browser pass.** This is the shape
`scripts/doc-gate.sh` already records one axis over, on features: with F
off, every link into F-gated code is unresolvable by construction, and
the answer there is to allow `rustdoc::broken_intra_doc_links` for that
pass ONLY — stated at the site as the cost of the widening rather than a
tidiness flag (`RUSTDOC_LINTS_INERT`). The target axis gets the same
answer for the same reason, and the reason is not that the browser docs
do not matter: it is that the lint cannot tell *this link is broken*
from *this link's target is in the other half*.

**So the link gate is the host pass at `--all-features`**, at
`-D warnings`, which CI runs. It holds every page it renders —
including the case no by-construction argument covers: a link that
resolves at NEITHER target is broken on a host page and reds there.

**There are TWO host passes, and only one of them judges links.**
`scripts/doc-gate.sh` documents this crate a second time at DEFAULT
features on any run whose change filter did not seed the toolkit
(`--skip-viewer-toolkit`), and that pass renders the renderer-free half
ALONE: `app`, `drafts`, `forms`, `gpu`, `pane` and `widgets` are not
compiled there, so a link into any of them is unresolvable by
construction — the same shape as the feature and target axes above, in
a third place. **Ev ruled on 2026-09-11 that the renderer-free half MAY
link into the `app`-gated half**, and that pass runs
`RUSTDOC_LINTS_INERT` accordingly, stated at its site. What is checked
where, exhaustively:

- **At `--all-features`** — every link in this crate, both halves. A
  branch whose diff touches `crates/viewer` takes this pass, because
  `scripts/ci-filter.py` seeds `RUN_VIEWER_TOOLKIT` off that diff, so
  whoever writes a broken link reds on their own branch.
- **At DEFAULT features** — every rustdoc lint EXCEPT
  `broken_intra_doc_links`. The renderer-free half's prose is held to
  all of the rest on every run, which is what that pass is still for.
- **Nowhere** — a link in the renderer-free half broken by its TARGET
  moving. Writing a link means diffing `crates/viewer`, which seeds the
  toolkit; but a link also breaks when the item it points at is renamed
  or deleted, and that happens on someone else's branch.
  `cargo_scope` is the dependent closure while `run_viewer_toolkit` is
  keyed on the SEEDS — `scripts/ci-filter.py`'s
  `res["RUN_VIEWER_TOOLKIT"] = "true" if seeds & VIEWER_TOOLKIT_SEEDS`
  (`:2495`), read by `ci.yml`'s `rustdoc (gate)` step, whose
  `if [ … run_viewer_toolkit = 'true' ]` picks `scripts/doc-gate.sh`
  over `scripts/doc-gate.sh --skip-viewer-toolkit` (`ci.yml:1890-1894`).
  So a branch seeded elsewhere takes skip mode **with `viewer` in
  scope** — and the default-features pass, link lint inert, is then the
  only rustdoc reading this crate.

  **That case is NOT empty, and it is the contingency that used to keep
  it empty that lapsed rather than anything about the mechanism.** The
  rule that produces the population is *every intra-doc link in a `///`
  or `//!` line under `crates/viewer/src`, outside the `app`-gated
  modules, whose first path segment is one of this crate's non-`app`
  dependencies*, which is

      rg -n -g '!{app,drafts,forms,gpu,widgets,pane}.rs' -g '!pane/**' -g '!bin/**' -e '(\[`|\]\()(pncad|bvh|editor_core|toml)::' crates/viewer/src

  and it prints **15** lines, one per site; add `| wc -l` for the
  number alone. **Fourteen target `pncad`**, which is itself a toolkit
  seed (`scripts/ci-filter.py:1436`,
  `VIEWER_TOOLKIT_SEEDS = {"viewer", "pncad", "bvh"}`), so every branch
  that can break one of those fourteen seeds the toolkit and takes the
  all-features pass. **The fifteenth does not.**
  `session::refuse`'s `Refusal::NoSuchParam` doc links
  `` [`editor_core::edit::UNDECLARED_PARAM_RECOURSE`] ``, and
  `editor-core` is not in the seed set — so a branch that renames or
  deletes that constant reaches `viewer` through the closure, takes
  skip mode, and nothing anywhere reports the break. **The hole this
  bullet once described as theoretical is open**, and the row that owns
  the repair owns this instance with it.

  **What the rule cannot match**, stated because the claim above it is a
  universal: a link whose target is reached through a `use`, since the
  link text then carries the in-scope name and not the defining crate.
  `sketch.rs`'s `` [`ProfileVertex`] `` is one — it resolves into
  `pncad`, and no path-shaped sweep can see it. A bare code span naming
  another crate is not a link at all and is unchecked in both passes.

  The ruling's second clause, `nightly.yml:291-293`'s
  `rustdoc (viewer, all features)`, does not close it: that row is
  `cargo doc -p viewer --all-features --no-deps` with no `RUSTDOCFLAGS`
  anywhere in the file, so a broken link there is a warning and the step
  exits 0 — measured, by planting one. It re-takes the RENDER, not the
  lint. The sweep rule this bullet owes is *the renderer-free half's
  cross-crate link targets, against `VIEWER_TOOLKIT_SEEDS`*, written
  above as the command that produces it, and
  `renderer-free-cross-crate-links-are-ungated-off-the-seed-set` — on
  MIRROR's slate, having moved there with CIW's cut — owns the repair.

Someone who runs `cargo doc` on this crate without `app` — the reader
the renderer-free half exists for — meets one of those links as the
literal text `[crate::app::…]`, brackets and all, because the target is
not there to link to. That is the cost and it is accepted: a
`cfg(not(feature))` configuration is allowed to render badly (Ev,
2026-09-11).

**The ruling settles the SPELLING as well as the legality: a doc comment
that names an `app`-gated ITEM links it, whatever the grammar of the
sentence around it.** Until that date a bare span was the only legal
spelling, so this crate acquired two for one relationship — a reference
written as one path became `` [`crate::app::FieldWriting`] ``, while the
same reference written as a possessive stayed `` `app` ``'s
`` `remember_theme` ``. A possessive is not a prose exception: its
second span names an item, so it takes the link too. **The sweep rule
that produces the population** is every `///` or `//!` line under `src/`
carrying a backtick span whose content is an `app`-gated module name
(`app`, `drafts`, `forms`, `gpu`, `pane`, `widgets`, with or without a
`.rs` suffix) immediately followed by a possessive — *whether or not a
second span follows it*, which is the widening that matters, because the
rule every earlier sweep here used (*whole span content is
`<mod>::<path>`*) cannot see this shape at all. Two things the rule
still does not reach, stated because a sweep whose blind spot is
unstated is not a negative result: a span naming one of those modules as
a CONCEPT rather than as a namespace (`sketch.rs`'s *"The forms' own
default is `app`'s, not this"*), which has no item to point at; and a
possessive whose first span is a type, a trait or another crate, which
is the same grammar over a different population.

**A PRIVATE target links, but only if it is nameable, and the two are
not the same test.** A private FIELD and a private METHOD resolve
(`ViewerApp::fit_delta_on_scene` and `ViewerApp::remember_prefs` are
both linked and both private), because rustdoc resolves an associated
item through its type and both host passes run
`--document-private-items` with `rustdoc::private_intra_doc_links`
allowed — the decision `scripts/doc-gate.sh`'s header argues and its own
selftest pins. **A module-scoped private `const` or `fn` does not**, and
`--document-private-items` does not change that: the flag decides what
rustdoc RENDERS, while a path is resolved by ordinary visibility, and
`crate::gpu::EDGE_CLIP_Z_LIFT` is not a path anyone outside `gpu` may
write. Measured by planting it: the all-features pass errors
*"no item named `EDGE_CLIP_Z_LIFT` in module `gpu`"*. So
`pickindex.rs`'s and `gpu.rs`'s deliberate pointer pair over their two
slack constants stays NAMED at both ends — the one population this
section's linking rule cannot reach, and the reason is visibility rather
than a feature gate. Widen the target to `pub(crate)` first if a link is
wanted, or leave it named; do not reach for `#[allow]`.

**A browser pass, if one is ever run, allows that one lint.** None is
run today, and the feature axis is better off here than this one:
doc-gate's pass 3 at least COMPILES the half it widens to, where
wasm-only items get no doc build at all.
`work/view/wasm-only-doc-comments-are-checked-by-nothing.md` owns that
gap.

**The population, dated and by name — with the instrument that takes
it, which belongs HERE.** A reading whose command is not named cannot be
re-taken, and the precedent keeps its pass in the same file as its
enumeration for that reason:

```sh
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' \
RUSTDOCFLAGS='-D warnings -A rustdoc::private_intra_doc_links' \
cargo doc --no-deps --document-private-items \
  -p viewer --features app --target wasm32-unknown-unknown
```

Read 2026-09-14 with that lint set: **eight sites over four identifiers
in two files**, and an identifier is a link SPELLING, so
`ThreadEvaluator` and
`crate::evalseam::ThreadEvaluator` count apart. `evalseam.rs`:
`ThreadEvaluator` ×2, `ThreadIndexer` ×1. `app.rs`: `ThreadEvaluator`
×1, `crate::evalseam::ThreadEvaluator` ×1, `StartupError::Worker` ×3.
The fit worker adds nothing but that third `StartupError::Worker`: its
own links sit inside the `cfg(not(wasm))` module, which the browser
pass does not render at all.
The enumeration is COMPLETE rather than illustrative, and it is a
reading of the tree rather than a property of it. **Line numbers are
deliberately not carried**: doc-gate's header gives the reason and has a
drifted citation to show for it, and this crate has spent four such
findings in a day.

**The one shape that is a DEFECT rather than a cost, and how to decide
it without an attribute grep.** Ask whether the host all-features pass
renders a page for the item the doc comment sits on — the same `cargo doc` without
`--target`, then look for the page under `doc/viewer/`; it is there or
it is not:

- **It does** — the host all-features pass holds that link, and a
  browser-pass error on it is by construction. Permitted.
- **It does not** — the item is absent at the host, so the browser pass
  is that link's ONLY reader and it has to resolve there. A bracket
  resolving at neither target spells a checked claim nothing anywhere
  checks; name the item instead, as `WINDOW_TITLE`'s doc does in the
  other direction (*"`run_web` — absent from this configuration, so
  named rather than linked"*).

The test is page existence in rustdoc's own output and **not** the `cfg`
on the item, because the two come apart inside this very file:
`WebStartupError` is `cfg(target_family = "wasm")` and its variant doc
comments carry no `cfg` of their own, so an attribute test reads them as
unconditional and reaches the wrong bullet. It is not a test on the
LINKED item's `cfg` either, and it needs no special case for the `app`
feature: `mod app` is `cfg(feature = "app")`, the host all-features pass
documents this crate WITH that feature, so its page exists and its links
are held — and the default-features pass beside it judges no link at
all, so there is no second answer for this test to disagree with.

## The design plan

The ratified clauses `G1`–`G5` and the GUI questions `GQ1`–`GQ7` live
in `crates/viewer/GUI-DESIGN.md`. This page is the record: what the
code does and why it is arranged this way, kept current by whoever
changes the code.
