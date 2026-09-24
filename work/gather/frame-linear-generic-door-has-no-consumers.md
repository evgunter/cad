---
id: frame-linear-generic-door-has-no-consumers
kind: issue
title: A CLASS - public generic doors with zero production call sites, kept alive by their own tests: Frame::linear<T> is DELETED; the two profile map_scalar rungs (PR 2409) are the half still open
status: open
opened: 2026-09-11
refs: [2375, 2409, 2475, 2487]
pr: 2487
priority: P4
cost: E
---


## Finding

From the full review of PR 2375 (S4, confidence `sure`). Accurate at
that PR's head.

`crates/editor-core/src/placement.rs:230-232`'s `pub fn linear<T: Real>`
has **no consumers anywhere**: PR 2375 moved `determinant` onto
`linear_f64` and `affine` onto `affine_f64`, and repo-wide the only
`.linear::<` call sites left are the three inside that file's own test
module. The reviewer proved it mechanically — dropping `pub` yields
`warning: method 'linear' is never used` from
`cargo check --workspace --all-targets`.

The reviewer's own sentence is the finding: *"a public generic door kept
alive by its own test is exactly what the item was retiring at the other
end."*

## Why it was not done in PR 2375's fix pass

Deliberate, orchestrator's call. It is a **public API removal**, the
review classed it as a style finding (non-gating), and a fix pass exists
to repair what its review found wrong — not to widen the PR into a
surface decision nobody asked for. It gets its own row so the decision
is made on its merits.

## What the decision actually is

Three answers and none is obviously right:

1. **Delete it.** Nothing uses it, the workspace proves it, and this
   project is pre-release with no external consumers to break. Fail-loud
   says an unused public door is a claim the library does not keep.
2. **Keep it as API.** `Frame::affine<T>` is the kernel's placement
   door; `linear<T>` is its other half, and a library that offers the
   affine map at the backend scalar but not the linear part is oddly
   shaped from outside. If that is the argument, it belongs at the site,
   because right now nothing says it.
3. **Demote it to `fn`** beside `linear_f64` and `affine_f64`, keeping
   it for whatever reads it next. Cheapest, and the one that leaves the
   question open rather than answering it.

Whoever takes it should check first whether `Frame` is re-exported on a
public path at all and whether the Python surface reaches it — the
review measured the *workspace*, which is the right instrument for (1)
but not for a claim about external users.


## Second instance, and that makes it a class (2026-09-12, PR 2409's full review)

`crates/profile/src/lib.rs:453` (`Profile::map_scalar`) and `:767`
(`ProfileLoop::map_scalar`). Proven the same way and mechanically:
making both private and running `cargo check -p profile` emits **two**
`method 'map_scalar' is never used` warnings. `Profile::map_scalar`'s
only callers are `crates/profile/tests/scalar_lift_door.rs`;
`ProfileLoop::map_scalar`'s only non-test caller is `Profile::map_scalar`.

**And PR 2409 removed the last production caller in the act of minting
the rung above it.** Before it, the loop rung had one (`sweep`'s
`end_profile`); after, neither rung has any. `D385`'s scheduled
conversion of the two test copies produces only **test** consumers, so
the schedule does not retire the shape.

### Adjudicated: the two instances are NOT the same defect

Kept in one file because the *shape* is one and a taker should see both,
but the dispositions differ and the difference is the useful part:

- **`Frame::linear<T>` LOST its consumers.** PR 2375 moved
  `determinant` and `affine` onto the private f64 doors and left a
  public generic door nothing calls. Nothing owes it an existence; the
  three answers on this row (delete / keep-as-API-with-the-argument-at-
  the-site / demote to `fn`) stand unchanged.
- **`profile`'s rungs were minted to a written convention.**
  `crates/geom/src/scalar_lift.rs:12-23` says *"one name, `map_scalar`
  on every geometry type and `map` on every leaf"* — so in a **library**
  crate the door is owed to an external caller whether or not an in-tree
  site wants it. That is a real argument and PR 2409 did not make it; it
  called the absence "deliberate" and left it there.

**What the class actually asks**, which neither instance answers alone:
does this project want `scalar_lift.rs`'s convention to mint public
doors ahead of consumers, and if so, is the `Frame::linear<T>` case a
violation of the same convention (a geometry type whose lift door should
therefore STAY) rather than dead weight? Answering that disposes of both
rows; answering either alone leaves the other unprincipled.

**Where else to look**: every type named in `crates/geom/src/scalar_lift.rs`'s
convention, checked for a `map`/`map_scalar` with no non-test caller.
The instrument is the one used twice here — drop `pub`, compile, read
the dead-code warnings — and it is exact, not a grep.

## Measured (2026-09-12, at `e25946743`) — WIRE's placement-prose lane

### `Frame` IS on a public path; the Python surface is NOT

`editor_core::Frame` is re-exported at the crate root (`crates/editor-core/src/lib.rs`,
`pub use placement::{AxisRefusal, Frame}`) and again on the facade's
curated document surface (`crates/pncad/src/document.rs`, the
`Frame`/`AxisRefusal` re-export group). So an external **Rust** consumer
of `pncad` can spell `linear::<T>()`.

The **Python** surface cannot. `crates/pncad-py/src/py/place.rs`'s
`#[pyclass] Frame` exposes five constructors (`translation`,
`rotate_then_translate`, `point_at`, `path_start_frame`,
`mirror_across_plane`), three reads (`columns`, `origin`, `determinant`)
and `__eq__`/`__repr__`. Neither `linear` nor `affine` crosses. That
closes the half of the question the workspace measurement could not
answer: there is no binding-side caller and no binding-side reason for
the door.

In-tree the count has fallen since this row was written: `.linear::<`
now has **one** call site, `placement.rs`'s own
`affine_at_f64_carries_the_stored_bits`, not three.

### The class's instrument, run over every rung of the convention

Two levels, because bare privatization answers the wrong question for a
rung whose callers are in its own crate but another module:

- **A** — drop `pub` outright. Answers "any consumer outside this
  module".
- **B** — `pub` to `pub(crate)`. Answers the question a *public* door's
  existence turns on: "any consumer outside this crate". An `E0624`
  naming a `tests/` file is a TEST-only consumer; one naming another
  crate's `src/` is a production consumer.

Instrument: `cargo check --workspace --all-targets`, one rung at a time,
reverted after each. Verbatim results:

| rung | level | verdict |
| --- | --- | --- |
| `Frame::linear` (`editor-core/src/placement.rs`) | A | `warning: method 'linear' is never used` — **no errors at all**. Zero consumers outside its own test module. |
| `Frame::affine` (control) | A | `error[E0624]` ×3 — `eval/wire.rs` ×2, `mate/solve.rs`. Live. |
| `Curve3::map_scalar` (`geom/src/scalar_lift.rs`) | A, B | `warning: method 'map_scalar' is never used` **both times**; every `E0624` is in `geom`'s own `tests/` (`curves/n1r1_lift_probes.rs`, `curves/n1r2_lift_probes.rs`, `net_placeholder_width.rs`). **Zero non-test consumers anywhere.** |
| `Surface::map_scalar` (same file) | A, B | identical shape — `warning: method 'map_scalar' is never used`, errors only in `geom`'s own `tests/`. **Zero non-test consumers anywhere.** |
| `NurbsCurve2`/`NurbsCurve3::map_scalar` (`geom/src/curves/nurbs.rs`, one macro) | B | `warning: method 'map_scalar' is never used` + `E0624` only in `geom`'s `tests/`. Dead **because** the rung above it is: its one lib caller is `Curve3::map_scalar`. |
| `NurbsSurface::map_scalar` (`geom/src/surfaces/nurbs.rs`) | B | `error[E0624]: crates/sweep/src/loft.rs:328`. **The one rung with a production consumer outside its crate.** |
| `SurfaceDescription::map_scalar` (`geom/src/surfaces/approx.rs`) | B | **clean** — no warning, no error. In-crate callers only; nothing outside `geom` names it, not even a test. |
| `ApproxSurface::map_scalar` (same file) | B | **clean**, same as above. |
| `Vec2::map` (`geom-core/src/linalg/vec.rs`) | B | `warning: method 'map' is never used`, **no errors**. Zero consumers workspace-wide, tests included. |
| `Vec3::map` (same file) | B | `E0624` ×15, all `crates/geom/src/scalar_lift.rs`, **and no `never used` warning** — the tell of an in-crate consumer level B cannot see. **CORRECTED 2026-09-12:** it has two, `Mat3::map` (`linalg/mat.rs`, ×3) and `Affine3::map` (`linalg/affine.rs`), and `Affine3::map` is live. **Live.** The struck claim was "its only consumers sit inside the dead ladder". |
| `Point2::map` (`geom-core/src/linalg/point.rs`) | B | `warning: never used` in `geom-core` + `E0624` from `geom/src/curves/nurbs.rs:619` and `profile/src/lib.rs:206`, `profile/src/validate.rs:843,844`. Live. |
| `Point3::map` (same file) | B | `warning: never used` in `geom-core` + `E0624` ×10, all inside `geom` (`scalar_lift.rs` ×8, `curves/nurbs.rs:619`, `surfaces/nurbs.rs:646`). Live only through the lift ladder. |
| `Mat3::map` (`geom-core/src/linalg/mat.rs`) | B | one `error[E0624]` — `crates/editor-core/src/placement.rs`, in `Frame::linear`'s body — **and no `never used` warning**, the tell of an in-crate consumer level B cannot see. **CORRECTED 2026-09-12:** it has one, `Affine3::map` (`linalg/affine.rs`), whose own doc says so; `Affine3::map` is live, so deleting `Frame::linear` did NOT orphan this. **Live.** The struck claim was "its only consumer workspace-wide". (The line number this table first carried was `:231`, read at the branch's first commit; the file has grown twice since, which is why the citation names the function. `implementer-discipline.md` §7.) |
| `Affine3::map` (`geom-core/src/linalg/affine.rs`) | B | `warning: never used` in `geom-core` + `error[E0624]: crates/profile/src/lib.rs:638` (`SketchPlane::map`). `Frame::affine` is a second consumer the run could not show — `profile` failed first, so `editor-core` was never checked. |
| `ProfileLoop::map_scalar` (`profile/src/lib.rs:453`) | B | `E0624` ×7, **all** in `profile/tests/` (`bool9_probes`, `bool9r1_probes`, `r2_bool9_review_probes`, `scalar_lift_door`). Test-only outside its crate. |
| `Profile::map_scalar` (`profile/src/lib.rs:767`) | B | `warning: method 'map_scalar' is never used` + `E0624` ×2, both `profile/tests/scalar_lift_door.rs`. Test-only outside its crate. |
| `ProfileVertex::map` (`profile/src/lib.rs:205`) | B | one `E0624`, `profile/tests/scalar_lift_door.rs:76`. Test-only outside its crate. |
| `SketchPlane::map` (`profile/src/lib.rs:637`) | B | `error[E0624]: crates/editor-core/src/eval/wire.rs:1515` (+ `profile/tests/sketch_plane.rs`). Live. |

Correction to the section above: this row's `:453` and `:767` labels are
swapped against the code — `:453` returns `ProfileLoop<U>` and `:767`
returns `Profile<U>`.

**What the instrument could not see**, stated so the negative result is
not read wider than it is:

- **Level B cannot see a consumer inside the rung's own crate**, because
  `E0624` is a privacy error and `pub(crate)` is not private to a
  same-crate caller. A level-B run answers *"any consumer outside this
  crate"* and nothing wider, so no level-B row licenses a
  *workspace-wide* verdict on its own. The tell that a level-B row has
  an in-crate consumer is already in this table: every level-B rung with
  none reports a `warning: … is never used` beside its errors, and the
  two rungs that report no warning at all — `Mat3::map` and `Vec3::map`
  — are exactly the two whose verdicts were wrong. See the correction
  section below.

- `Cargo.toml` `exclude`s `demos/`, `tools/`, `benches/` and
  `interval-transcendentals/`, so `--workspace` compiles none of them and
  `demos/tour`+`demos/wild` are exactly the outside-consumer seat this
  question is about. Checked by **grep** only (`.map_scalar`,
  `.linear::<`): no hit for either in those four roots. A grep is not the
  instrument; that line is weaker than the rest of the table.
  **Narrowed by the review (2026-09-12):** `.linear::<` cannot match a
  turbofish-free call, so the four roots were re-swept for the bare token
  `\.linear\b` — still zero hits.
- Cargo stops a crate's dependents once that crate fails, so every
  `E0624` list here is a **lower bound** on the consumer set — except
  where no crate failed at all, which the review checked: the
  `Frame::linear` run, the `Vec2::map` run and a second level-A run
  short-circuited nothing, so those three are **exact zeros**, not lower
  bounds. It does not
  weaken a `warning`-only or clean row, which is where every negative
  verdict above comes from.
- `dead_code` runs per target, so a method used only from its own crate's
  `#[cfg(test)] mod tests` still warns on the plain lib target. That is
  the signal, not noise: it is precisely "kept alive by its own test".

### What the class asked, answered

*Does this project want `scalar_lift.rs`'s convention to mint public
doors ahead of consumers?* The measurement says **it already does, at
most of its rungs** — of the eight `map_scalar` rungs the module names,
exactly one (`NurbsSurface::map_scalar`) has a production consumer
outside its own crate, and both TOP rungs (`Curve3`, `Surface`) have no
non-test consumer anywhere. So "the convention owes the door" cannot be
settled by pointing at the convention: the convention as practised is
mostly unconsumed, and adopting it as a rule would ratify that. That is
`geom`'s ground, not WIRE's — filed as
`work/props/the-scalar-lift-convention-mints-doors-faster-than-consumers.md`.

*And is `Frame::linear<T>` a violation of the same convention (a
geometry type whose lift door should therefore STAY)?* **No.** The
convention is "`map_scalar` on every geometry type and `map` on every
leaf", and `Frame` is neither: it is a document-layer placement record in
`editor-core`, its door is spelled `linear`, and `scalar_lift.rs` does
not name it. The convention does not reach this instance, so the three
answers this row already had are the whole decision.

### Recommendation: DELETE, with the counterarguments

**Delete `pub fn linear<T: Real>`.** Three reasons:

1. The consumer set is empty on every path measured: one in-tree caller,
   its own test; no Python reach; no `demos/` reach.
2. **The project has already ruled this way, in writing, at the same
   altitude.** `crates/pncad/src/lib.rs` refuses to re-export `bvh` —
   *"No demo scene, no export corpus, and no document-layer path names
   it, and that measurement is what decides the re-export. Re-export it
   the day a consumer needs it."* Same instrument, same answer: mint on
   demand. The narrowing of `profile` to a curated module in the same
   file, over one measured leak, is that position applied harder.
3. Fail-loud: an unused public door is a claim the library does not keep,
   and the row's own sentence — *a public generic door kept alive by its
   own test* — is what the item was retiring at the other end.

Honest counterarguments, none of which I think carries:

- **Symmetry.** `affine<T>` is the placement door and `linear<T>` is its
  other half; a library offering the affine map at the backend scalar but
  not the linear part is oddly shaped from outside. Real — but the same
  asymmetry is already shipped in the binding, which exposes `columns`
  and `determinant` and neither map, and nobody has hit it.
- **It exports a finding instead of closing one.** Deleting `linear`
  leaves `Mat3::map` with **zero** consumers workspace-wide (measured
  above). That is `geom-core`, PROPS's ground — so the delete owes the
  PROPS row a line, which the filing above gives it.
- **The test loses half its subject.** `affine_at_f64_carries_the_stored_bits`
  cross-checks `linear`'s columns against `affine`'s linear block; without
  `linear` it asserts only the affine door. Small: the affine assertion is
  the one the kernel's placement path depends on.
- **"Pre-release with no external users" is a prediction, not a
  measurement.** If the kernel is ever published, removing a public method
  is the breaking change and keeping it was free. This is the strongest
  of the four, and it is the argument for answer 3 (demote to `fn`) rather
  than for answer 2.

If the orchestrator will not spend a public-API decision here, **demote
to `fn`** beside `linear_f64`/`affine_f64`: it costs nothing, it makes
the workspace lint tell the truth about reachability, and it leaves the
question open. What should **not** happen is answer 2 without its
argument written at the site, which is the state the door is in today.


## Re-taken independently (2026-09-12, PR 2475's review)

Every row of the table above was reproduced exactly. The review also
sharpened two of this section's own hedges — both folded in above — and
took the disposition: **delete**, as its own unit, so the public-API
removal's blast radius is reviewed on its own terms, including the
finding it exports to PROPS that `Mat3::map` is then consumerless. This
row stays `open` until that unit lands; PR 2475 measured and recommended
and removed nothing.


## Half disposed (2026-09-12) — `Frame::linear<T>` deleted

**This row stays `open`.** It is a CLASS with two instances and this
unit disposes of one; closing it here would label a half-fix as a fix.

**Instance 1 — `Frame::linear<T>` (`crates/editor-core/src/placement.rs`): DONE.**
Deleted, on the recommendation this row already carries and the review
that re-took it independently. What went with it: the sole caller, the
`let l = f.linear::<f64>()` cross-check block inside
`affine_at_f64_carries_the_stored_bits`. The test itself STAYS — its
subject is `Frame::affine`, and only the half that read the deleted door
was removed. The `# Exactness` section's cross-reference (it named both
`Frame::linear` and `Frame::affine` as the read-back doors) now names
`Frame::affine` alone; a pointer at a deleted door is worse than none.
`linear_f64` stays, private, as `determinant` and `affine_f64` need it.

There is no assertion to add for this. The deletion's guard is the
compiler — a caller anywhere in the workspace is now a build error —
plus `pncad`'s `every_document_layer_root_export_is_carried_or_listed`
and `pncad-py`'s `test_every_curated_name_is_bound_or_listed` over the
public surface. A test asserting that a deleted function is absent is
documentation, and `docs/prompts/implementer-discipline.md` §2 says
deleting it is the repair.

**Instance 2 — `profile`'s two `map_scalar` rungs (`crates/profile/src/lib.rs`):
STILL OPEN, and untouched by this unit.** `crates/profile/` is not
WIRE's fence. Its disposition is the one this row's own adjudication
says differs: those rungs were MINTED to `crates/geom/src/scalar_lift.rs`'s
written convention rather than having LOST their consumers, so the
argument that decides them is the convention's, filed on PROPS's slate
as `work/props/the-scalar-lift-convention-mints-doors-faster-than-consumers.md`.

**The delete exported nothing.** The first version of this section
claimed it orphaned `Mat3::map`; that was false and is retracted — see
the correction section below. `Mat3::map`'s consumer is `Affine3::map`,
in its own crate, and `Frame::affine` — the door this unit KEEPS —
reaches it through exactly that path, so the test kept here still
exercises it.


## Correction (2026-09-12, PR 2487's review) — two verdicts were level-B results narrated as workspace-wide claims

**The instrument was right; the write-up overstated it.** Level B
(`pub` → `pub(crate)`) answers *"any consumer outside this crate"*.
Two rows in the table above were then written up as claims about the
whole workspace, which does not follow, and both are false:

- **`Mat3::map` is LIVE.** `crates/geom-core/src/linalg/affine.rs` —
  `Affine3::map`'s body — is `Affine3::from_parts(self.linear.map(&f),
  self.translation.map(&f))`, and `self.linear` is a `Mat3<T>`. The
  function's own doc comment says it in words: *"the linear part through
  [`Mat3::map`], the translation through [`Vec3::map`]"*. `Affine3::map`
  is live (`Frame::affine`, `SketchPlane::map`, `sweep`'s loft), so
  deleting `Frame::linear` left `Mat3::map` with a live consumer, not
  none.
- **`Vec3::map` is LIVE outside the dead ladder.** The same two call
  sites — `linalg/mat.rs`'s `Mat3::map` (three per call) and
  `linalg/affine.rs`'s `Affine3::map` — are in-crate, so "its only
  consumers sit inside the dead `Curve3`/`Surface` ladder" is false.

**Every other level-B row was re-read for the same substitution**, and
the sweep is complete over this table rather than stopping at the two
the review named. Pattern swept for: a level-B verdict phrased as a
claim about the workspace or about "anywhere", rather than about
consumers *outside the crate*. Dispositions:

- `Curve3::map_scalar`, `Surface::map_scalar` — **stand.** Both were run
  at level **A** as well, which does answer the module-scope question,
  and both warned there.
- `Frame::linear`, `Frame::affine` — **stand.** Level A only.
- `SurfaceDescription::map_scalar`, `ApproxSurface::map_scalar` — **stand.**
  Their verdicts already read clean-at-B correctly, as *in-crate callers
  only*.
- `NurbsSurface::map_scalar`, `Point2::map`, `Point3::map`,
  `Affine3::map`, `SketchPlane::map` — **stand.** Each says *live*, and
  an in-crate consumer level B cannot see could only make that more
  true.
- `NurbsCurve2`/`NurbsCurve3::map_scalar` — **stands**, and it is the
  transitive case rather than an exception: its in-crate caller
  (`Curve3::map_scalar`) is itself dead, which is why a warning fired
  anyway. The row already says so.
- `ProfileLoop::map_scalar`, `Profile::map_scalar`, `ProfileVertex::map`
  — **stand.** All three are phrased *"test-only outside its crate"*,
  which is exactly the claim level B supports.
- `Vec2::map` — **stands**, and it is the one remaining row carrying a
  genuine workspace-wide claim off a level-B run, so it was re-checked
  rather than assumed: `crates/geom-core/src/` contains no `Vec2::map`
  call site at all (every non-iterator `.map(` in that tree is
  `mat.rs`'s, `affine.rs`'s, or a `f64::to_bits` in a test), so the
  `never used` warning is genuine and not the transitive case, and the
  run's zero errors close the outside. **Zero consumers workspace-wide
  stands.**

What this sweep could NOT match: it re-reads the verdicts in THIS table
against the level each was run at. It does not re-run the instrument, so
a row whose recorded `E0624` list or warning was itself mis-transcribed
would survive it — the two corrections above were found by reading the
code, which is the check that catches that.


## State at 2026-09-13: instance 1 disposed (PR 2487), instance 2 live

**Instance 1, `Frame::linear<T>`, is deleted.** Its measurement was run
at level **A** — drop `pub` entirely, `warning: method 'linear' is never
used`, no errors, whole workspace compiling — which is the level that
answers *"any consumer anywhere"*, and the review confirmed the deletion
is complete: nothing in code, `docs/`, any `crates/*/README.md` or
`pncad.pyi` still names the method, and the surviving test's subject is
`Frame::affine`, which still reds for real reasons.

**Instance 2, the two `profile` `map_scalar` rungs, is untouched and
outside WIRE's fence.** The dispositions differ for a reason worth
keeping: `Frame::linear` **lost** its consumers and nothing owed it an
existence, while `profile`'s rungs were **minted to a written
convention**. That convention, and whether this project wants it to mint
public doors ahead of consumers, is
`work/props/the-scalar-lift-convention-mints-doors-faster-than-consumers.md`
— and neither it nor the opposing rule in `crates/pncad/src/lib.rs` is
ratified, so it is ordinary engineering rather than Ev's.

## The side-finding this row exported was FALSE, and the cause is the row's own instrument

The first cut of PR 2487 reported that deleting `Frame::linear` left
`Mat3::map` consumerless. It does not. `Affine3::map`'s body is
`Affine3::from_parts(self.linear.map(&f), self.translation.map(&f))`,
`self.linear` is a `Mat3<T>`, and that call **is** `Mat3::map`.

Three things make this worth a section rather than a line:

- **`Frame::affine`, the door this row's unit KEPT, reaches `Mat3::map`
  through exactly that path.** The deletion orphaned nothing.
- **The refutation was already on PROPS's own slate**, twice, in
  `affine3-try-map-the-fallible-walk-has-no-kernel-door.md`: *"descends
  through `Mat3::map` and `Vec3::map`."* The false claim was filed into
  the directory that already contained its refutation.
- **The instrument was right and the narration overstated it.** Level
  **B** (`pub` → `pub(crate)`) answers *"any consumer outside this
  crate"* and `E0624` cannot fire for a same-crate caller; the verdict
  was then written as *"workspace-wide"*. **A missing `never used`
  warning beside a level-B row is positive evidence of an in-crate
  consumer**, and two rows in this table had no warning and were read as
  if they did.

Corrected by sweeping **every** level-B row rather than the two named,
with each row's disposition recorded. What that sweep could not match is
stated on the row: it re-reads verdicts against the level each was run
at, and does not re-run the instrument, so a mis-transcribed `E0624`
list or warning would survive it.

## Read against the tree (2026-09-24)

**Left open. The remaining half needs a decision this row does not own.**
Re-measured on PR 3141's branch with instrument level B (make both rungs
`pub(crate)`). The run is `cargo check --keep-going --workspace
--all-targets --features interval`, for the reason in the next section.
`ProfileLoop::map_scalar` and `Profile::map_scalar`
(`crates/profile/src/lib.rs`) each report `warning: method 'map_scalar'
is never used`, and every `E0624` is in a test:

- `profile/tests/`: `bool9_probes` ×2, `bool9r1_probes` ×3,
  `r2_bool9_review_probes` ×3, `scalar_lift_door` ×4
- `sweep/tests/wire_loft_end_profile_lift.rs` ×1

Only `--keep-going` shows that last one, because without it cargo stops
at the first failing crate. So both rungs are still consumed by tests
only. This row's own adjudication says what decides them:
`scalar_lift.rs`'s convention against `pncad`'s mint-on-demand rule. That
question is
`work/verdict/the-scalar-lift-convention-mints-doors-faster-than-consumers.md`
(cost D), which already names both rungs. **Recommend parking this row on
that one.**

**The instrument has a feature-shaped blind spot, and this lane walked
into it.** A run at default features cannot see a consumer behind
`#[cfg(feature = "interval")]`. `AssertionVerdict::map`
(`editor-core/src/measure.rs`) gave `warning: method 'map' is never used`
and no errors at level A under default features, so the first cut of PR
3141 deleted it. The `interval` clippy lane then failed:
`eval/mod.rs`'s leaf replay calls it (`a.clone().map(&project)`). **It is
live, and the deletion was reverted in the same PR.** Every verdict in
this section was re-taken with `--features interval` and `--keep-going`.
Any earlier negative row in this file taken at default features alone
(`Vec2::map`, and the `geom` rungs) has the same gap. That ground belongs
to VERDICT's row, and the gap has been added there.

**Class sweep.** The pattern was
`pub fn (map_scalar|map|try_map|lift|linear|affine)<` outside `tests/`.
A second pass looked for any `pub fn` generic over `U`/`V`/`S`:

- `AssertionVerdict::map`: **live** under `interval`, as above. Kept.
- `ProfileLoop::map_scalar`, `Profile::map_scalar`: test-only. Waits on
  VERDICT's row.
- `ValidatedProfile::lift_onto` (`profile/src/validate.rs`): **live.**
  Level B gives `E0624` at `crates/sweep/src/loft.rs`.
- `SketchPlane::try_map`: **live** (`eval/wire.rs`'s `pinned_plane`).
  `Vec3`/`Mat3`/`Affine3::try_map` are live through it.
- `Margin::lift` (`geom-core/src/predicate.rs`): **live**
  (`geom-brep/src/props/quad.rs`).
- `Frame::affine`, `SketchPlane::map`, `Point2`/`Point3`/`Affine3::map`:
  live, per the table above.
- The `geom` rungs and `Vec2::map`: VERDICT's row. Not re-measured.
- `viewer`'s `ProfileDoors::map` is `pub(crate)`, so it is outside the
  class.

**What neither pass could match:** about 250 non-lift `pub fn …<T: Real>`
functions. Each needs its own compile to measure, so the sweep did not
reach them, and no grep stands in for that instrument.
