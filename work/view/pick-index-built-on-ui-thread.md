---
id: pick-index-built-on-ui-thread
kind: issue
title: The pick index is built on the UI thread, so a landing that costs seconds is a frame that costs seconds
status: open
opened: 2026-08-29
github: 1259
refs: [1217, 1247]
needs_ev: true
---

## From GitHub issue 1259

Opened 2026-08-29; 0 comments.

## What happens

`ViewerApp::sync_scene` runs at the top of `eframe::App::ui`, and inside it `PickCache::sync` calls `PickIndex::build`, which per product root runs `NodePick::build_all` → `mesh::tessellate` + `MeshPick::build` (the triangle BVH). All of it on the UI thread. While it runs the window does not repaint and still shows the previous document.

GUI-3 moved the **evaluation** off-thread — `EvalService`, `Inline` + `Thread`, cancel-and-restart, per-job `CancelToken`, the busy/canceled chrome. The tessellation and index that follow it did not come with it, so the seam that exists to keep the frame loop responsive stops one step short of the expensive step.

## What it costs

Measured on a 2.8 GHz Xeon, release, after the median-partition BVH build ([#1217](https://github.com/evgunter/cad/pull/1217)) and with the display budget ([#1247](https://github.com/evgunter/cad/pull/1247)) choosing δ:

| | triangles | tessellate | index total |
|---|---|---|---|
| `hollowring` at δ = 0.1 mm (what it asked for before the budget) | 3 984 276 | 6.5 s | 13.4 s |
| `hollowring` at δ = 0.400 mm (what the budget opens it at) | 998 576 | — | ~2.3 s |

End to end, Open… → the new document on screen went 25 s → 16 s (the BVH partition) → 8 s (the budget). **The remaining 8 s is mostly this**, and the budget cannot take it further without giving up picture quality it should not have to trade.

The budget also only binds when a document ARRIVES, deliberately (#1247's design: it sets a default, it is not a cap), so an edit that makes a document much denser at the δ in force reaches the UI thread with nothing in front of it.

## Shape of the fix

The worker returns the index with the run; a δ change re-submits. `PickCache`'s retry policy ("at most one attempt per (generation, δ), a refusal is kept and readable rather than retried into a stall") moves with it, and a δ change while a build is in flight wants the same cancel-and-restart the evaluation already has.

Two things already exist that make this less of a leap than it sounds: an `Evaluation` already crosses the seam, so the payloads are `Send`; and the chrome already knows how to draw a picture older than the document — that is the `canceled — showing an older result` state, spinner, Cancel and Re-evaluate included.

## Measured against the tree (2026-09-04): the expensive step is not cancelable

"Shape of the fix" above says "a δ change while a build is in flight
wants the same cancel-and-restart the evaluation already has". Two of
the three things that rests on hold and one does not.

**What holds.** The seam is `submit` / `poll` / `cancel` over a
`Generation`, and a submit while a run is in flight cancels that run
and starts the new one (`crates/viewer/src/evalseam.rs:146`). An
`Evaluation` already crosses it, so the payloads are `Send`.

**What does not.** The cancelation "is the shipped `CancelToken` and
nothing else: it is checked between nodes"
(`crates/viewer/src/evalseam.rs:42`). The step this unit moves has no
nodes to be checked between, and neither `mesh::tessellate`
(`crates/mesh/src/tessellate.rs:43`) nor anything in `crates/bvh`
takes a `CancelToken` at all. So the 6.5 s tessellate this item
measures is uninterruptible as it stands, and moving it to the worker
moves an uninterruptible 6.5 s rather than a cancelable one.

Three ways out, and they are not equivalent — this is a question for
6a, not a choice to make at implementation time:

1. **Cancel between ROOTS only.** Cheap and entirely inside this
   program's ground, but a product with one big root — which
   `hollowring` is — gets no cancel point at all, so it does not
   answer the case the measurement is about.
2. **`mesh` and `bvh` grow cancel points.** The honest fix, and both
   are other programs' territory (`crates/mesh/*` is MESH's,
   `crates/bvh/src/*` is CERT's), so it is two announces and two
   programs' schedules before this unit can start.
3. **Restart without cancel**: a δ change lets the in-flight build run
   to completion and discards its result. Costs one wasted build and
   nothing else, needs no door, and is a weaker promise than the
   evaluation seam makes — which is exactly the kind of asymmetry §5's
   inventory should state rather than leave to a reader to notice.

## Why this is filed and not done

It changes the seam GUI-3 §5 ratified, and the §5 re-take ("GO ON EGUI, AUTHORITATIVE") rests on a complete frame-state inventory that this would extend — an index that lands asynchronously is new frame state with a new staleness rule. That wants a ruling, not a commit.

## Home

`work/issues/` — the GUI-3 §5 seam is GUI-era ground and that program is closed; PERF's keep_out cedes per-frame rendering and hover-picking to the viewer.

## 6a — the seam question, put to Ev (VIEW orchestrator, 2026-09-04)

This section is 6a: the ruling the plan owes before 6b or 6c can
start. No code. It is stated here rather than in a spec because the
spec that ratified the seam (`docs/GUI-3-SPEC.md`) was deleted at
merge per `docs/DOC-LEDGER.md`, and the clause that survives it is
`crates/viewer/README.md`'s **Toolkit and CI posture (GQ6)**.

### What the ratification actually rests on

The §5 re-take was "**GO ON EGUI, AUTHORITATIVE**", and
`docs/MODEL-AB-LOG.md`'s GUI-3 row records what it rested on: *a
complete frame-state inventory (both reviewers' adversarial sweeps
confirm; no document shadows, no interior mutability)*. The live form
of that ruling is GQ6's first named condition for abandoning egui:

> the immediate-mode loop needing **ad-hoc frame-to-frame state to keep
> `Doc` authoritative** … None is met.

So the inventory is not decoration: it is the evidence for a toolkit
decision, and every field added to `ViewerApp` is an entry in it. The
inventory as it stands is `crates/viewer/src/app.rs:330-380` — each
non-document field carrying its own justification.

### Why an off-thread index is a new KIND of entry

Every existing entry is one of two shapes:

- **deferred intent** — `pending_fit`, `fit_on_scene`: a thing owed,
  taken on a later frame. It cannot be wrong about the document; it
  only has not happened yet.
- **environment or preference** — `chooser`, `show_datums`,
  `checks_shown`, `store`: not about the document at all.

`notices` looks like a third but is not: it is drained every frame and
cannot survive into the next one, which its doc comment says.

**An asynchronously-landing pick index is neither.** It is derived data
that can be *behind the document currently on screen* — the first
entry in the inventory that can be WRONG rather than merely late. That
is what makes it a ruling and not a commit: the question is whether
that is still "no document shadows", and a shadow that is a stale copy
of a derivation is exactly the shape GQ6's condition is about.

**The orchestrator's reading, offered as an argument and not a
finding:** it does not trip the condition, because `Doc` stays
authoritative and the staleness is not ad-hoc — the seam already
carries a `Generation`, and *"the index is for generation N, the
document is at N+1"* is a comparison of two values the tree already
has, not a hand-maintained flag. But that reading is exactly the kind
of thing that should not be self-certified into a ratified clause,
which is why this is here.

### Q1 — does the inventory admit a derived value that can be stale?

If yes, GQ6 gains a sentence naming the third shape and its rule. If
no, 6b does not happen and the 8 s stays.

### Q2 — what does the viewport do while the index is behind?

Three answers, and the tree supports all three:

1. **Pick against the stale index.** Cheapest; the user picks the
   entity they saw a frame ago, which is usually the one they aimed
   at. Silently wrong when the edit that changed the document is the
   one they are picking to inspect.
2. **Refuse the pick, typed**, until the index catches up. Fail-loud,
   which is the project's standing posture, and the chrome already
   knows how to say *"canceled — showing an older result"*.
3. **Fall back to the GPU id-buffer pass**, which GQ6 already ships as
   *advisory* beside the authoritative `Bvh::ray` query — and
   `frame` already owns a rule for reporting the two paths as
   disagreeing. This is the only answer that keeps picking working and
   keeps it honest, and it is also the one that promotes an advisory
   path to load-bearing, which GQ6 ratified as advisory on purpose.

### Q3 — cancelation, which the 2026-09-04 measurement forced

The section above this one measured it: the step 6b moves is
**uninterruptible**. `evalseam`'s cancelation "is the shipped
`CancelToken` and nothing else: it is checked between nodes"
(`crates/viewer/src/evalseam.rs:42`), and the moved step has no nodes
to be checked between — neither `mesh::tessellate`
(`crates/mesh/src/tessellate.rs:43`) nor anything in `crates/bvh`
takes a `CancelToken` at all. So "cancel-and-restart like the
evaluation seam" is not available. The three ways out are stated
above and are **not** equivalent:

1. cancel between ROOTS only — inside this program's ground, and
   answers nothing for `hollowring`, which is one big root;
2. `mesh` and `bvh` grow cancel points — the honest fix, and it is
   **two other programs' territory and two other programs' schedules**
   (MESH and CERT) before this unit can start;
3. restart without cancel — one wasted build, no door needed, and a
   **weaker promise than the evaluation seam makes**. If this is the
   answer, the asymmetry belongs in the inventory as a stated rule
   rather than left for a reader to discover.

### What lands when this is answered

One paragraph in `crates/viewer/README.md` under **Toolkit and CI
posture (GQ6)**, extending the inventory with the third shape and its
staleness rule, plus whichever of Q2 and Q3's answers Ev takes. Then
6b and 6c, or 6b alone if Q1 rules the staleness rule is not
expressible as frame data.
