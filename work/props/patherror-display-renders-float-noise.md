---
id: patherror-display-renders-float-noise
kind: issue
title: `PathError`'s Display arms render scalars with `{:?}`, so refusal sentences carry round-tripped float noise
status: open
opened: 2026-08-30
github: 1282
refs: [1267]
priority: P1
cost: E
---

## From GitHub issue 1282

Opened 2026-08-30; 0 comments.

A **class**, split out of the BLEND-7 review (PR #1267) rather than swept in there.

`Real` carries `Debug` and no `Display`, so every arm of `impl Display for PathError<T>` reaches its scalar payloads through `{:?}`. For `f64` that is the shortest round-tripping form, which is exactly right for a diagnostic dump and wrong in a sentence a person reads: an 8 mm radius that arithmetic produced renders as

> …tangent setback 0.008000000000000002 m exceeds the 0.0034999999999999996 m the anchor pins…

The same applies to `ProfileError` and to the other doors' error types that carry scalar payloads.

## What PR #1267 did

Only the arm it added (`FilletEnclosesLegCarrier`) renders through a small private helper, `path::num`, which prints the shortest decimal that still names the same number to a relative 1e-9 and passes non-`f64` `Debug` forms (intervals, duals) through untouched. It is a display choice only — the payload keeps the exact scalar, and nothing branches on the string.

## What is open

Whether to apply the same treatment across the existing arms, and where the helper belongs if so (several crates have the same shape, so `geom-core` beside `Real` is the obvious home). Worth deciding once: a per-arm trickle would leave the two spellings side by side indefinitely, which is its own smell.

— Filed by the BLEND-7 implementer lane, adjudicating both blinded reviews.

## Home

`work/code-quality/` — this is a structural finding about two spellings of one job (`{:?}` scalars versus `path::num`) living side by side across crates, which is the register's stated subject.

## Re-homed to DOOR (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

DOOR collects the rows whose fix is already written in the row — one PR
each, no design question left open. This row is here because a lane can
take it and land it without deciding anything first.

Its class at the cut was **M** — decide helper's home once, then
mechanical sweep of Display arms. The class is a dispatch estimate made
by reading the row against the tree on 2026-09-11, not a verdict on the
finding, and a lane that finds it wrong says so in its PR. The id, the
`track:` letter where the row carries one, and the body above are
unchanged by the move.

## Narrowed (2026-09-11, the DOOR orchestrator) — the `profile` half is done

Read against the tree before dispatch. **The row's motivating example
already renders correctly**, and most of what it asks for landed while
it sat in the pile.

Executed over the current `num`'s exact body:
`0.008000000000000002` renders `0.008`, `0.0034999999999999996`
renders `0.0035`. FIX's closed row
`path-error-numbers-below-1e-9-render-as-zero` (PR #2366) records that
`num` already backs **38 call sites across three `Display` impls** in
that file — `CornerRefusal`, `CornerReason` and `PathError` — so *"apply
the same treatment across the existing arms"* is answered for
`crates/profile/src/path.rs`: every arm goes through the helper.

**What is left of this row is its second half only**: `ProfileError` and
the other crates' error types that carry scalar payloads, and where the
helper lives if it is to serve them. That half is untouched and is
still `M`. The helper is still private to `path.rs`, so a second
consumer is what forces the home question, and `geom-core` beside
`Real` remains the obvious answer.

**Two things this row must NOT do, both learned from FIX's two closed
rows on this helper:**

- **Do not reintroduce an absolute floor at the small end.** `num` used
  to read `tol = 1e-9 * x.abs().max(1.0)`, and that `.max(1.0)` pinned
  the tolerance absolute for `|x| <= 1`, rendering every sub-nanometre
  margin as `0` — the margins these messages exist to report, since a
  junction is tangent precisely when its margin is below threshold.
  #2366 removed it. The purely relative form is the repair.
- **Do not propagate the constant without reading it.** The relative
  1e-9 is correct below a decimetre and coarser than ε above one; that
  is a separate defect, filed on FIX's slate as
  `num-relative-tolerance-collides-above-a-decimetre` rather than
  carried here, because the helper is FIX's ground. A lane widening
  `num` to a second crate should land after it, or carry it.

## Re-homed to PROPS, 2026-09-20

(DOOR orchestrator) Ev, in chat, 2026-09-20: *"can you kick all the design decisions back to
the track they actually belong to, leaving fix design-free?"* — asked of
FIX and applied to DOOR in the same sitting. **DOOR claims no paths**, so
unlike FIX it can never be the owning track for any decision: there is no
row here whose surface this program owns. A row needing a decision
therefore always leaves. That also retires the charter clause admitting
*"a small design call (where a shared helper's home goes, what a door
looks like)"* — DOOR's own rule already said *"a row that grows a design
question stops being this program's"*, and the two clauses contradicted
each other.

**The decision this row is blocked on:** where the `num` helper lives once a second crate consumes it —
`crates/geom-core/src` beside `Real` is the row's own obvious answer — and
whether the remaining error types get the same treatment.

**Why PROPS.** The helper's candidate home, `crates/geom-core/src/real.rs`, is **owned by
props**, and the row's remaining half is *`ProfileError` and the other
crates' error types that carry scalar payloads*. The `profile` half is
already done and is not PROPS's: all 38 arms across three `Display` impls in
`crates/profile/src/path.rs` (PATHS's) go through the helper.

**A correction that matters more than the routing, and it is why this note
is long.** `work/door/plan.md` carried a long *Review posture* section
prescribing the rounding point this row should adopt — an absolute cap at
`DEFAULT_EPS / 10` met with a relative arm, the finer grid winning, spelled
`tol = (DEFAULT_EPS * 0.1).min(x.abs() * RELATIVE)`, with `RELATIVE` left for
the lane to pick and argue. **That is not an open choice: it is the code in
the tree.** `crates/profile/src/path.rs`'s `num` reads
`let tol = (DEFAULT_EPS * 0.1).min(x.abs() * 1e-9);` today, landed by FIX's
PR 2399 on 2026-09-12 closing
`num-relative-tolerance-collides-above-a-decimetre`, together with the
reasoning now in the helper's own doc comment (why the cap is compile-time
`DEFAULT_EPS` and never the run's live `Tolerance::eps()`). The plan was
written the day before that landed and was never re-read against the tree.
The section is lifted here rather than deleted with the plan, as a RECORD of
what was decided and where it landed — not as an instruction to a lane.

**So the blocker this row names has fired.** Its body says a lane widening
`num` to a second crate *"should land after it, or carry it"*, naming the
relative-tolerance row; that row closed 2026-09-12.

**The trap in the vocabulary, which cost a reader once already.** The plan
calls `DEFAULT_EPS / 10` *"an absolute floor"* while this row's body says
*"do not reintroduce an absolute floor at the small end"* — they mean
opposite things. `min` CAPS the tolerance, making the grid finer at large
magnitudes; the defect PR 2366 removed was `1e-9 * x.abs().max(1.0)`, which
FLOORED the tolerance and rendered every sub-nanometre margin as `0`. The
helper's doc now states this at the site: *"a FLOOR under the tolerance is
the mirror defect and is precisely what a `min` cannot become"*. Read the
doc, not either summary.

**What is actually left**, therefore: the home question above, and the sweep
of the other crates' error `Display` arms. FIX kept the one PATHS-side
instance of the same class as a written fix
(`fillet-leg-carrier-renders-raw-float-noise`,
`crates/profile/src/validate.rs`) — worth reading beside this row, since a
second consumer of `num` outside `path.rs` is exactly what forces the home
question this row asks.

Nothing about the finding is changed by the move: same id, same
evidence, still `open`, and no part of its question is answered for you.

## Evidence added 2026-09-21 (FIX, `fix/fillet-leg-carrier-num`)

FIX's `fillet-leg-carrier-renders-raw-float-noise` landed. `num` is now
`pub(crate)` in `crates/profile/src/path.rs` and has a second consumer:
`FilletLegCarrier`'s `Display` in `crates/profile/src/validate.rs`.

**That second consumer is WITHIN the crate, so it does not answer this
row's question and was deliberately not made to.** The home question
this row names is where the helper lives *once a second CRATE consumes
it*; `pub(crate)` is the narrowest visibility that serves a sibling
module and moves nothing across a crate boundary. The lane read the
fence that way on purpose and stopped at it.

Two facts this row can now use:

- **The `f64`-typed door exists too.** This row's body motivates the
  helper from `Real` carrying `Debug` and no `Display`. `FilletLegCarrier`'s
  fields are plain `f64`, whose own `Display` is the same shortest
  round-tripping spelling, so a concretely-typed scalar payload reaches
  the identical defect without `Real` being involved. A sweep of the
  other crates' error types should look for `f64` fields, not only
  `T: Real` ones.
- **`crates/profile/src/` is now clean of the class.** Swept at merge
  base `57b0f7844` for `{ident} m` / `{ident} rad` and by enumerating
  every `Display` impl in the crate: every scalar-bearing refusal
  rendering routes through `num`. `ProfileError`, `LiftRefusal`,
  `StructureRefusal`, `ReplayError` and the vocabulary enums render
  only indices, counts and words — no scalars. What is left of this row
  is entirely outside `crates/profile/`.

## The shape this actually takes (2026-09-20, read against the tree)

**Do not mint a third helper.** The "where does the helper belong"
question the section above leaves open is already answered twice in the
tree, and the job is to unify those two rather than add to them.

**The kernel already has one.** `path::num`
(`crates/profile/src/path.rs`, used by `profile::validate`) is the
helper PR #1267 added, and **FIX owns its history** — three rows closed
on it: `path-error-numbers-below-1e-9-render-as-zero` (PR #2366, which
removed a `.max(1.0)` pinning the tolerance ABSOLUTE at 1e-9 for
`|x| <= 1` and so rendered every sub-nanometre margin as `0` — exactly
the margins these messages exist to report), `verb-and-dimension-render-through-debug`
(PR #2347), and `num-relative-tolerance-collides-above-a-decimetre`.
The purely relative form is right at the small end and must stay.

**The GUI already has the fuller one.** `viewer::readout::number`
(`crates/viewer/src/readout.rs`) is declared a *vocabulary* module: one
pure function over `f64` plus two constants. Its rule is **the shortest
decimal spelling that reads back as this value**, with a scientific form
when no decimal spelling does, bounded by `REL_TOLERANCE` — which is not
a taste but the four-significant-figure scientific form's own worst
case. Its doc states the property that makes it the right thing to
lower: *"Nothing here is a threshold, so there is no magnitude to go
stale against a format."* It is read-back fidelity, not an epsilon
cutoff. Its two doors are `scene::DisplayTolerance::render_mm` (the
delta-facing one, the millimetre conversion and nothing else) and
`widgets::number_text` (the editable fields').

**Ev's steer, in chat 2026-09-20:** the GUI's display policy is relevant
to any user-facing render and could plausibly be lowered into the
kernel; it crosses program lines and that is fine, the conflict risk
there being low.

Scope correction (2026-09-21, from the FIX evidence section above):
`crates/profile/src/` is now **clean of the class**, so what is left of
this row is entirely outside it — and the sweep must look for plain
`f64` fields, not only `T: Real` ones, since `FilletLegCarrier` reached
the identical defect with concretely-typed scalars.

So the unit is: **one vocabulary module, kernel-side, and two spellings
retired onto it** — then every `Display` arm carrying a scalar payload
re-pointed at it, across `PathError`, `ProfileError` and the other
doors' error types. It stays low-risk in the sense that matters (nothing
branches on the string; payloads keep the exact scalar; no predicate, no
comparand, no verdict moves), but it is **not the cost-E row the front
matter claims** — it is a three-owner consolidation.

### Seams to announce before landing

- **FIX — CLOSED, so this seam has no owner to announce to.** Corrected
  2026-09-21 on merge: FIX swept out at sweep 18 and `work/fix/` is
  deleted, `docs/DOC-LEDGER.md` being its done-state of record. Its three
  rows on `path::num` still bind as *constraints* even though no program
  holds them: the purely relative small-end form must stay, and PR
  #2366's removal of a `.max(1.0)` — which had pinned the tolerance
  ABSOLUTE at 1e-9 for `|x| <= 1` and so rendered every sub-nanometre
  margin as `0` — is the row a careless unification would most easily
  undo. Whoever takes this row inherits those without a counterparty to
  ask, so they are written out here rather than left as a pointer.
- **VIEW / VIEWER** — owns `readout.rs`, `DisplayTolerance::render_mm`
  and `widgets::number_text`. Lowering the module must leave those two
  doors working, and `number_text` is on a COMMIT path (an
  `egui::DragValue` seeds its keyboard edit with the text it last showed
  and writes the parse back on blur), so `REL_TOLERANCE` bounds what
  gets committed and not only what gets shown.
- **PROPS** — this row, and the `Display` arms themselves.

### Review posture

DOOR's, which Ev set in-chat at that program's opening: **no A/B row**,
light style review by default, full correctness review only where a row
has real risk of being wrong. That is the right posture here and is also
what the A/B hold of 2026-09-20 requires.

## Correction: the "Re-homed to DOOR" paragraph above is STALE

That paragraph records the 2026-09-11 cut. It was superseded by
`272f2c038` ("door: re-home every design decision to the track that owns
it"), which moved this row back OUT of DOOR and into `work/props/`,
where it is now. The row is PROPS'. The DOOR paragraph is left in place
rather than deleted because it is the record of a real move; this
heading is what makes it not read as current.
## Reference note (FIX's sweep, 2026-09-21)

`num-relative-tolerance-collides-above-a-decimetre` was dropped from this row's `refs` because the row closed with **FIX**, which left the tracker at sweep 18 — `work/fix/` is deleted and `docs/DOC-LEDGER.md` is its done-state of record. The finding is unchanged and still readable: `git show 6f0e04ce1534:work/fix/num-relative-tolerance-collides-above-a-decimetre.md`.

## The second CRATE consumer has landed (2026-09-22, VGEOM, `vgeom/render-grid`)

**This row's trigger has fired.** Its question is *"where the `num`
helper lives once a second CRATE consumes it"*, and the 2026-09-21
entry above records that the `validate.rs` consumer did not count
because it is inside `crates/profile`. This one is outside it:
`crates/viewer/src/readout.rs` now computes `num`'s grid.

**What was adopted, exactly.** `readout`'s private `tolerance(value)`
is `EPS_CAP.min(value.abs() * REL_TOLERANCE)` with
`EPS_CAP = DEFAULT_EPS * 0.1` — the same two-armed shape as
`path::num`'s `(DEFAULT_EPS * 0.1).min(x.abs() * 1e-9)`, the same
`min`, the same compile-time `DEFAULT_EPS` rather than the run's live
`Tolerance::eps()`, and the same refusal to put a FLOOR under it. The
reason is the one `num`'s doc gives and this row's body preserves: ε
is a length, a purely relative rule crosses it and is coarser above
the crossing, and the cap is what makes a difference the kernel can
decide a difference the render spells.

**Nothing was unified, deliberately.** `num` is `pub(crate)` in
PATHS's ground and its named home is `crates/geom-core/src/real.rs`,
which is this program's. The viewer lane was fenced out of both. It
put the grid in ONE private function, called from one predicate, so
a consolidation is a deletion rather than a rewrite, and it marked the
three differences at that function's own doc so a later lane can tell
a decision from drift.

**The three differences, and which are decisions:**

| | `profile::path::num` | `viewer::readout` | decision? |
|---|---|---|---|
| notation | follows the `Debug` form's own choice; only shortens the mantissa Rust already picked | chooses: shortest decimal that reads back, scientific when none does | **decision** — a refusal sentence quotes a magnitude nobody chose; a chrome value is in a notation the person chose, and a length written in millimetres wants to read back in millimetres |
| relative arm | `1e-9` | `5e-4` (`readout::REL_TOLERANCE`, the four-figure scientific form's own worst case) | **decision** — see below |
| character bound | none | `readout::MAX_CHARS`, 22 | **decision** — `readout`'s texts go in boxes and a clipped render reads as a different value; a refusal sentence is as wide as it needs to be |

**The relative arm is the one a consolidation must not resolve by
accident.** Neither site derives it from ε, and they cannot: the arms
cross at `EPS_CAP / relative`, and below that crossing the relative
arm is finer than the cap by construction, so the cap has already
guaranteed ε-separation and the relative arm decides only how many
figures a sub-crossing value is spelled to. `num` can afford ten
because it has no width bound; `readout` keeps four because below its
crossing (2·10⁻⁷ of a display unit) a length is below ε in every
notation the chrome writes one in, and a fifth figure there is
precision no probe established — the false-precision defect
`readout::number` exists to cut, arriving by the other door. **So a
shared helper takes the arm as a parameter, or takes one of the two
knowingly and says which.** Taking `1e-9` silently would put nine
figures into every GUI label below a tenth of a display unit.

**One thing this evidence adds to the sweep the row's second half
asks for.** `readout`'s grid is a LENGTH grid (ε is a length) and the
chrome renders angles and scalars through the same door; that is
filed as VGEOM's
`the-render-grids-cap-is-a-length-and-angles-go-through-it`. A shared
kernel-side helper inherits the same question the day a non-length
payload reaches it, and `ProfileError` and the other doors' error
types — this row's remaining half — carry angles.
