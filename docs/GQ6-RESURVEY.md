# GQ6 re-survey — toolkit, viewport, picking, wasm (2026-08-16)

**Status: SURVEY + RECOMMENDATION, ratifying nothing.** GQ6
(`docs/GUI-DESIGN.md`) was deferred to GUI time with one binding
instruction — *re-survey before committing*. This document is that
re-survey. It **supersedes the 2026-07 ecosystem snapshot** in
GUI-DESIGN's GQ6 section as the current factual record, and it
**does not decide GQ6**: per the repo's design convention, a
question this shape is ratified in conversation with Evan, not by
an agent writing a doc. Everything below is evidence plus a ranked
recommendation to argue with.

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

**Recommendation: egui, with iced as the named runner-up.** The
deciding factors are current-wgpu tracking, the docking ecosystem, a
production existence proof of exactly our shape (rerun = egui panels
+ wgpu viewport), and release cadence. The MVU-fit argument for iced
is genuine but is an argument about where the architecture *lives*,
not whether it works — and G1's architecture already lives in
`editor-core`, below any toolkit, by design. That is precisely what
makes this choice reversible: the toolkit sits above the layer that
holds the decisions.

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
(`pncad-py` is out of scope by construction: PyO3 targets a native
CPython.)

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

## 5. What would settle this, and what would flip it

The cheapest decisive experiment is **one spike, one afternoon, done
twice**: a window with a docked side panel and a wgpu viewport
drawing one M5 tessellation, with click → `ray → stable ref` through
the `editor-core` hit-test service — once in egui, once in iced.
That measures the two things this survey cannot: how much friction
the immediate-mode/retained-document seam actually creates, and how
much of the toolkit's version churn lands on us.

Tripwires that would change the recommendation:

- **iced ships 0.15 on current wgpu** with the release gap
  narrowing → the MVU-fit argument stops carrying a cadence penalty
  and iced becomes the leading candidate on merit.
- **egui's churn lands on the toolchain pin.** If an egui MSRV bump
  ever forces a compiler move that D9's bit-identity gate is not
  ready for, that is a direct conflict with L2 and worth re-opening.
- **We decide the browser is the primary target.** Then the toolkit
  question re-ranks around web ergonomics (both egui and iced run on
  wasm; the panel/text/IME story on mobile browsers differs), and
  the wgpu-30 web backend versus Firefox-on-Linux's timeline enters.

## Open questions for the ratification conversation

1. Is the toolkit decision taken now, or does it stay deferred with
   this document as the refreshed snapshot? (Nothing forces it: the
   GUI is unbuilt and `editor-core` holds the decisions. But the
   slate has shrunk from five to two, which is new information.)
2. Does the **wasm result change the sequencing** — i.e. is "the
   kernel runs in a browser" something to protect with a CI target
   check now (cheap, and it would catch a regression the day it
   lands) rather than rediscover at GUI time?
3. Is egui's quarterly-breaking-change tax acceptable against L2's
   pinned toolchain, or is that the axis on which iced wins?
