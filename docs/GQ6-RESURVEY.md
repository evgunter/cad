# GQ6 re-survey — toolkit, viewport, picking, wasm (2026-08-16)

**Status: the survey, plus one RATIFIED row.** GQ6
(`docs/GUI-DESIGN.md`) was deferred to GUI time with one binding
instruction — *re-survey before committing*. This document is that
re-survey, and it **supersedes the 2026-07 ecosystem snapshot** in
GUI-DESIGN's GQ6 section as the current factual record.

**Toolkit: RATIFIED 2026-08-16 (Evan) — egui, falling back to iced
if egui does not work out.** Evan's ruling on this survey: "egui
sounds enough better than iced that I'd switch the framing to
'egui, unless that doesn't work, then try iced'." §1 is written in
that frame. The remaining rows are unchanged in status: viewport
(§2) and picking (§3) are recommendations carrying no ratification,
the wasm row (§4) is measurement rather than decision, and GQ7 is
untouched.

## Method, and what is *not* evidence here

- **Version/license/MSRV data** comes from the crates.io API on
  2026-08-16 (authoritative for release dates and SPDX fields),
  not from recollection; the previous snapshot's weakness was that
  it was recalled, which is exactly why it was dated and marked
  non-binding.
- **Project liveness** comes from the upstream repositories
  (README status text, latest releases, recent commit activity).
- **The wasm row was measured, not surveyed** — `cargo check
  --target wasm32-unknown-unknown` was run against this tree at
  `a1fb025` on the pinned 1.97.0 toolchain. Commands and results
  are in §4 so the claim is reproducible and falsifiable.
- **No toolkit spike was built.** Nobody has written a panel + a
  wgpu viewport + a hit test in any of these toolkits inside this
  repo. Every ergonomics claim below is inference from upstream
  APIs and third-party production use, and the recommendation
  names the spike that would settle it (§5).

## 0. What actually changed since 2026-07

Five deltas, three of them load-bearing:

1. **The wasm caveat in the snapshot is obsolete, and the answer
   flipped in our favor.** "The `interval` feature is not
   [wasm-friendly], per issue #4" described the `inari` backend.
   M5 PR 1 (#127) replaced it with the in-repo
   `interval-transcendentals` — pure `libm`, `forbid(unsafe_code)`,
   no target-cpu floor, no C. Measured today: **the entire kernel
   *and* `editor-core` compile to `wasm32-unknown-unknown`,
   including `--features interval`** (§4).
2. **Fornjot is dead, formally.** The repository was archived
   2026-06-19 under a README that reads "This project has been
   shut down. Its goals were never reached." The snapshot cited it
   as a viewport data point; it is now a postmortem data point (and
   DESIGN.md Band-3 already cites it as scale evidence).
3. **CADmium is dead too** — archived 2025-09, "this repo is
   inactive", and licensed Elastic License 2.0, which was never
   compatible with our MIT-OR-Apache-2.0 posture anyway. Two of
   the three prior-art reference points in the old snapshot have
   exited. The snapshot's core finding — *the substrate is
   reusable, the CAD-ness is not* — is now **stronger**, not weaker.
4. **Slint's license question, which the snapshot flagged as
   "needs checking", resolves against adoption** (§1).
5. **Browser WebGPU became universal.** Chrome/Edge (long since),
   Safari 26 (macOS 26/iOS 26), Firefox 141+ on Windows and 145+
   on Apple-silicon macOS; Firefox on Linux is still in progress.
   The web viewport is no longer a bet on a pending API.

## 1. Toolkit

Facts as of 2026-08-16 (crates.io):

| Toolkit | Latest | Released | License | MSRV | wgpu pin |
|---|---|---|---|---|---|
| `egui` / `eframe` | 0.36.1 | 2026-08-07 | MIT OR Apache-2.0 | 1.95 | **30** (current) |
| `iced` | 0.14.0 | 2025-12-07 | MIT | 1.88 | 27 (3 behind) |
| `slint` | 1.17.1 | 2026-07-07 | GPL-3.0-only OR royalty-free OR commercial | 1.92 | none (femtovg/skia/software) |
| `gpui` | 0.2.2 | 2025-10-22 | Apache-2.0 | — | — |
| `bevy` | 0.19.1 | 2026-08-13 | MIT OR Apache-2.0 | 1.95 | 29 |

**Slint is disqualified, and the reason is the one the snapshot
suspected.** Its triple license is GPL-3.0-only *or* a
royalty-free license *or* a paid commercial license. The only
OSI-approved branch is GPL-3.0-only, which cannot ship inside an
MIT-OR-Apache-2.0 product; the royalty-free branch is a
proprietary grant, not an open-source license, and taking it would
mean our GUI binary's licensing is a vendor's revocable policy
rather than our own. Independently: Slint's renderers are
femtovg / Skia / software, so a wgpu CAD viewport has no first-class
seat. Two independent disqualifiers; drop it from the slate.

**GPUI is disqualified on maintenance posture.** `gpui` 0.2.2 is
from 2025-10 and upstream is explicit that it is pre-1.0, breaks
between versions, has no documentation outside reading Zed's source,
and that the Zed team *does not think it has the resources* to
maintain it as a standalone library. A community fork exists
(`gpui-ce` 0.3.3, 2026-07) and is a genuinely new development since
the last snapshot, but "a fork of an unmaintained-for-outsiders
toolkit" is not a foundation for a project whose kernel is designed
in decades. Keep as a curiosity; do not bet on it.

**bevy stays on the slate but demoted.** 0.19.1 is healthy and
`bevy_picking` is now in-engine, so it still offers free picking and
camera — the snapshot's reason for listing it. The cost is unchanged
and structural: an ECS worldview and a game-engine dependency tree
imported to host what is fundamentally a document editor, plus a
wgpu version (29) we would not control. Its real advantage — picking
— is the row where we are least short of options (§3).

**The live comparison is egui vs iced, and the axes have moved.**

- *Release health.* egui ships roughly quarterly and is on the
  current wgpu (30). iced's last release was 0.14.0 in **2025-12** —
  eight months ago — and pins wgpu **27**. iced's master is active
  (commits through 2026-08-14, largely a text-editing overhaul), so
  this is slow-release, not abandonment, but there is no announced
  0.15, and the README still calls iced "experimental software."
- *The wgpu pin is the real coupling, and it cuts for egui.* GUI-
  DESIGN commits to a thin custom wgpu viewport regardless of
  toolkit — but "regardless" is not "decoupled": the viewport shares
  the toolkit's `wgpu` types, so the toolkit's pin *is* our wgpu
  version. Choosing iced today means writing the viewport against
  wgpu 27 and waiting on someone else's release cadence to move.
- *Philosophical fit still favors iced, and it is not nothing.* G1
  *is* MVU: `Doc` is a value, `DocEdit` is a message, `apply` is the
  update function. In iced that is the framework's own shape. In
  egui we would be running an immediate-mode loop over a retained,
  authoritative document value — workable (rerun does exactly this
  at production scale), but the architecture lives in our code
  rather than in the toolkit's.
- *Custom-viewport support is a wash on capability.* Both host a
  custom wgpu pipeline first-class: `egui-wgpu`'s paint callbacks,
  and iced's `shader` widget (`iced::widget::shader` with a
  `Program`; the upstream `custom_shader` example is a 3D scene with
  its own pipeline and depth buffer).
- *Ecosystem for CAD-shaped chrome favors egui, concretely.*
  Docking/panel layout — which a feature tree + viewport + property
  panel needs on day one — has two live egui options (`egui_tiles`
  0.17.0, MIT/Apache, and `egui_dock` 0.21.1, MIT, both released
  this month), plus `egui_extras`. iced's equivalents are thinner,
  which is what the old snapshot meant by "thinner ecosystem"; that
  has not changed.
- *The cost of egui is churn.* 0.34 moved the primary entrypoint
  from `Context` to `Ui` and unified the panel API; MSRV went
  1.88 → 1.92 → 1.95 across three releases. A GUI on egui signs up
  for a migration every few months and for a toolchain pin (D9/L2)
  that must keep moving. This is a real recurring tax on a project
  that pins its compiler for bit-identity reasons — it is the
  strongest argument iced has left, and it is an argument about
  *our* maintenance budget, not about capability.

**New entrants checked, none promoted.** `xilem`/`masonry` 0.4
(Linebender, Apache-2.0) is the architecturally interesting one and
the team's pedigree is real, but its last release was 2025-10 and it
is openly pre-production — revisit, do not adopt. `dioxus` 0.7.10 is
healthy and irrelevant: a DOM/web-tech component model in which a
CAD viewport is a foreign object. `floem` 0.2.0 has not released
since 2024-11. None of these changes the shape of the decision.

**Ratified (Evan, 2026-08-16): egui — and if egui does not work
out, iced.** Not a tie to be broken later by a bake-off: egui is the
toolkit the GUI is built in, and iced is the named fallback if
building in egui goes badly. The deciding factors are current-wgpu
tracking, the docking ecosystem, a production existence proof of
exactly our shape (rerun = egui panels + wgpu viewport), and release
cadence. The MVU-fit argument for iced is genuine but is an argument
about where the architecture *lives*, not whether it works — and
G1's architecture already lives in `editor-core`, below any toolkit,
by design.

That layering is what makes committing now cheap rather than brave,
and it is why the fallback is a real option rather than a
face-saving clause: the toolkit sits **above** the layer that holds
the decisions, so switching to iced later costs the interaction
layer and nothing beneath it. What a fallback would cost concretely
is the viewport's wgpu version (30 under egui, 27 under iced 0.14)
and the docking chrome — both interaction-layer work.

## 2. Viewport

**Unchanged: thin custom wgpu, and the case is now stronger.**
`wgpu` 30.0.0 (2026-07-01, MIT OR Apache-2.0, MSRV 1.87) is
healthy and is the browser's own path (Firefox implements WebGPU
*with* wgpu). Nothing has appeared worth coupling to: `rend3` has
not released since 2022, `three-d` (0.19.0, MIT) is small and
OpenGL-shaped, and the CAD-adjacent renderers are attached to
kernels we are not adopting — `truck` (Apache-2.0) is alive and has
a wgpu renderer, but taking `truck-rendimpl` means taking truck's
geometry vocabulary, which is the coupling we refused. Fornjot's
`fj-viewer` is archived.

The only live viewport decision is therefore **which wgpu major**,
and per §1 that is a consequence of the toolkit choice rather than
an independent one.

## 3. Picking

**Unchanged in strategy: GPU ID-buffer pass for hover/click
exactness, CPU ray-cast for snapping.** Both still sit on the M2
PR 6 mesh back-references, which ship.

**One recommendation changes: prefer extending our own BVH over
adopting `parry3d`.** The snapshot named `parry3d` (0.30.2,
Apache-2.0; `parry3d-f64` exists) as the CPU ray-cast, written
before `crates/bvh` existed. That crate now ships a deterministic
AABB BVH with a documented conservative-superset contract, built for
the boolean sweep and *already naming viewport picking as a future
duty*. It has `overlapping(&Aabb)` and no ray query — the gap is a
ray-slab test and a traversal, not a library. Adopting `parry3d`
would instead import nalgebra and a second geometry vocabulary
alongside our own `Point3`/`Aabb`, for a query we can write. Keep
`parry3d` as the fallback if snapping grows into a toolbox of exotic
proximity queries (that is where it would earn its weight); start
with a `Bvh::ray` query.

*(Note for whoever implements it: picking is a UI concern, so the
ray query is under no D9 obligation — but our BVH is deterministic
anyway, so we get repeatable hit ordering for free.)*

## 4. Web/wasm — measured, and the caveat is gone

The snapshot's claim: "pure-Rust `libm` means D9 accidentally made
the f64 lane wasm-friendly; the `interval` feature is not, per
issue #4." **The second half is no longer true.** Issue #4 was met
by removal at M5 PR 1: `inari` and its gmp/MPFR LGPL stack left the
tree, and the replacement backend is pure `libm` with proven outward
pads, `forbid(unsafe_code)`, and no AVX/FMA floor.

Measured at `a1fb025`, toolchain 1.97.0:

```
rustup target add wasm32-unknown-unknown
cargo check -p <crate> --target wasm32-unknown-unknown
```

| Crates | Result |
|---|---|
| `topo`, `geom-core`, `geom-curves`, `geom-surfaces`, `geom-brep`, `mesh`, `profile`, `sweep`, `bvh`, `quantity`, `stl`, `step-export`, `step-import`, `editor-core`, `test-utils` | **clean** |
| `geom-core --features interval` | **clean** |
| `pncad` | fails on `getrandom` 0.3's wasm backend gate |

The `pncad` failure is a build-configuration item, not a design
problem: it is ASM-1's OS-randomness dependency for interactively-
authored document ids. Verified fix — with `getrandom`'s `wasm_js`
feature enabled and `RUSTFLAGS='--cfg getrandom_backend="wasm_js"'`,
`pncad` and its full dependency closure **check clean on wasm32**.

**`pncad-py` crosses too, and the reason it was set aside does not
apply to the default path** (#807). `pyo3` is an *optional* dependency
behind that crate's non-default `python` feature, so a default-feature
check has no Python involvement at all and passes on wasm32 with the
same backend cfg. "PyO3 targets a native CPython" is true of
`--features python`/`extension-module`, which is where the exclusion
belongs; it is not a reason to exclude the crate from a wasm check.

**Guarded since #807** (§Q6). This table was a reading of one tree at
one revision on one toolchain and nothing re-took it, so any dependency
bump could turn a `clean` cell red while every existing row stayed
green. `ci.yml`'s `doc` job now re-takes it on every code-tier run, in
three legs: the kernel plus `editor-core` unflagged, the same set with
`--features interval`, and the whole workspace under the `wasm_js`
backend cfg. The unflagged legs run **first** on purpose — a kernel
crate that grew its own `getrandom` edge would be masked by the third
leg's flag, which was demonstrated by planting exactly that. What the
guard establishes is *compiles clean*: `cargo check` runs no linker, so
neither of the two caveats below is covered by it.

So: **the whole product below the GUI compiles to wasm today,
certified-interval lane included.** That is a materially different
strategic position from the one the snapshot recorded, and it was
free — a side effect of D9's libm-only discipline and of resolving
#4 by removal.

Two caveats, flagged and not resolved:

- **Threads.** The D9 rayon idioms (evaluation service) have no
  wasm equivalent without cross-origin isolation plus
  `wasm-bindgen-rayon`. A wasm build needs a single-threaded lane
  or that shim; nobody has measured either.
- **`mul_add`.** wasm has no FMA instruction, so `f64::mul_add`
  lowers to a software implementation. It is correctly rounded, so
  **bit-identity holds** (which is what D9 requires); the cost is
  performance, unmeasured, and the exactness witnesses in the
  interval backend lean on `mul_add`.

**Strategic note worth having in the ratification conversation:**
Zoo, the best-funded web CAD, does *not* run its kernel in the
browser — Design Studio streams video frames from a hosted geometry
engine over WebSockets. Our kernel compiles to wasm as it stands.
In-browser modeling is genuinely available to us in a way it was not
to the reference implementations, and it costs a build lane rather
than an architecture. G1 stays agnostic; the option is real.

## 5. What "doesn't work out" would look like

The toolkit is decided, so the spike is no longer a bake-off — the
first GUI increment is simply built in egui: a docked side panel
plus a wgpu viewport drawing one M5 tessellation, with click →
`ray → stable ref` through the `editor-core` hit-test service. What
that increment measures is the one thing this survey could not —
how much friction the immediate-mode/retained-document seam actually
creates.

This section exists so the fallback has teeth: the conditions below
are what would send us to iced, written down in advance so the
switch is a recorded judgement rather than a mood.

- **The immediate-mode seam fights the document.** If holding
  `Doc` authoritative under an immediate-mode loop needs
  ad-hoc frame-to-frame state to keep widgets coherent, egui is
  costing us the thing G1 exists to protect, and iced's MVU shape
  stops being merely philosophical.
- **egui's churn lands on the toolchain pin.** MSRV went
  1.88 → 1.92 → 1.95 across three releases. If a bump ever forces a
  compiler move that D9's bit-identity gate is not ready for, that
  is a direct conflict with L2 — and unlike the seam risk, it is
  visible early, from the MSRV row of each egui release.
- **Rendering-integration breakage becomes chronic.** The bet on
  egui is partly that it tracks current wgpu; if paint callbacks or
  the wgpu pin become a recurring migration cost, that bet failed on
  its own terms.

Two things that would NOT reopen this on their own: iced shipping
0.15 on current wgpu (welcome, but "the runner-up improved" is not a
reason to move a working GUI), and a decision to target the browser
(both toolkits run on wasm; that would re-rank web ergonomics
without re-ranking the toolkit).

## Questions, answered and open

1. *Is the toolkit decision taken now, or deferred with this as the
   refreshed snapshot?* **Answered 2026-08-16: taken** — egui, iced
   as fallback (§1). The slate shrinking from five to two was the
   new information that made deferral pointless.
2. *Is egui's quarterly-breaking-change tax acceptable against L2's
   pinned toolchain?* **Answered by the same ruling: yes**, and §5
   keeps it under watch as the fallback condition with the earliest
   warning signal.
3. **Still open:** does the wasm result change sequencing — is "the
   kernel runs in a browser" something to protect with a CI target
   check now (cheap, and it would catch a regression the day it
   lands) rather than rediscover at GUI time? Note this is now a
   question about a *property we have*, not a strategic bet: nothing
   in the toolkit ruling depends on it, since egui runs on wasm too.

What stays deferred regardless: the viewport (§2) and picking (§3)
recommendations are engineering calls for whoever builds the first
GUI increment, and GQ7 (selection mechanics) is untouched.
