# PATHS log

## Opened at S-BOOL's exit (2026-09-16)

Opened by S-BOOL's orchestrator in the PR that proposes
`docs/S-BOOL-EXIT-WALK.md`, as `work/README.md`'s closing rule directs:
S-BOOL's eight open lattice items cohere into one track on one
territory (`crates/profile`), so the closing program opens the
successor and moves them here. Band 5000–5099 recorded in the ledger's
banding entry in the same commit. No unit dispatched; the first sitting
picks from `work/paths/plan.md` §The slate. Ev's sign-off on the exit
walk ratifies the opening.

## Rows arriving from FIX, 2026-09-20

Ev ruled in chat on 2026-09-20 that **FIX carries no design decisions**:
*"can you kick all the design decisions back to the track they actually
belong to, leaving fix design-free?"* FIX is the program for rows whose
fix is already written; three waves closed those and left a slate that
had drifted into decisions. Fourteen rows moved out by `git mv` to the
track owning the surface each decision is about, each carrying a
`## Re-homed` section stating the question, the routing basis, and what
was NOT decided for the receiver. Routing was taken from
`python3 scripts/work.py territory --files -` over every path the rows
cite, not from FIX's `keep_out` prose — two of that clause's fence
claims were stale and are corrected in the moved rows.

**One row: `arc-carrier-refusal-register-misses-two-format-arms`.**
`docs/PATHS-DESIGN.md` is yours by `paths`; the row was on FIX's slate
only because FIX took the two units that moved the arms the register
omits, and its own Fence section says so.

Its two halves separate cleanly. The cheap one: the *"Typed runtime
errors, from geometry"* register names neither `PathError::NonFiniteDirection`
(landed by PR 2359, never named) nor `PathError::UnderflowedDirection`
(PR 2415), both raised from inside that document's own surface — so it
wants the two arms plus the sentence distinguishing them from
`ZeroDirection` (not a coincidence at any tolerance; the recourse is
scale, not eps). The design half: whether that register is checked at all,
given `PathErrorKind` is a fieldless mirror and `pncad-py`'s
`TAG_INVENTORY` already pins the full arm set at the Python door — or
whether it is prose by design and should say so, so the next reader does
not take its completeness for a claim.

FIX keeps `fillet-leg-carrier-renders-raw-float-noise`
(`crates/profile/src/validate.rs`, your ground) because its fix IS
written — route the two `f64` fields through `path::num` — and FIX spans
fences by announcement for written fixes. Say if you would rather hold it
beside `validate-rs-hosts-a-quarter-of-the-fillet-subsystem-it-never-runs`.

Signed (FIX orchestrator).

## Announced seam from FIX (2026-09-21)

**`crates/profile/src/validate.rs` and `crates/profile/src/path.rs` —
PR 2946.** FIX's `fillet-leg-carrier-renders-raw-float-noise`, one of
the five written-fix rows left on its slate after the design-free
sweep. The whole unit is on your ground, crossed by announcement.

`impl Display for FilletLegCarrier` rendered `Arc`'s `radius` and
`angular_margin` through `f64`'s own `Display` — the shortest
round-tripping spelling, i.e. the arithmetic's noise unshortened. Both
now go through `path::num`, which changes from `fn` to `pub(crate) fn`
so a sibling module can reach it. **The rounding grid itself is
untouched**: the `min`, the compile-time `DEFAULT_EPS` cap and the
floor-is-the-mirror-defect clause are exactly as PR 2399 left them.

**Two things worth your attention beyond the one-line fix.**

- **The carrier's sentence is not only its own.** It is interpolated as
  `CornerReason::AnchorOutsideTrimmedExtent`'s `{carrier}`, beside a
  `{setback}` and `{available}` that `num` already shortened. So a
  `path` refusal with no noise in any scalar of its own was still
  carrying noise in its carrier clause.
- **`crates/profile/src/` is now clean of this class**, and the claim
  has two instruments behind it rather than one: a sweep for
  `{ident} m` / `{ident} rad` (26 lines, 36 interpolations, all but
  this one already routed), plus a reading of all 22 `Display` impls in
  the crate — which is what caught `CornerRefusal` rendering two
  ordinates with no unit word, invisible to the pattern by
  construction. It was already routed. What neither instrument can see
  is a scalar reaching prose through a containing type's `Debug`; that
  residual is stated on the item rather than left implied.

**`num`'s doc comment gained two paragraphs** — that a plain `f64`
field reaches the same defect by its own `Display`, and that the
helper's reach is now the crate's refusals rather than `path.rs`'s arms
(the old closing line said *"every arm below"*, which became narrower
than the truth once a second module called it). Checked before
editing: `num` is discussed in no `docs/` page and not in
`crates/profile/README.md`, so this is a source comment following its
code, not a design amendment.

**Nothing was re-baselined**, because nothing in the tree had ever
pinned this sentence — every consumer matches on the enum's fields. The
new pin uses subtracted rather than literal scalars, since `0.008` and
`0.0035` are exactly representable and a literal would have rendered
correctly with no helper at all.

Signed (FIX orchestrator).

## 2026-09-25 — a PATHS orchestrator picks the track up

Status `ready` → `active`. Ev's first ask for this sitting is
`lower-profiles-to-carrier-and-interval-not-vertex-and-bulge` (EMIT's
filing from #3202): let a circle be one edge by lowering to a
carrier + interval form instead of vertex + bulge. First step is the
survey the row asks for — a read-only lane mapping every reader of the
bulge form (profile, sweep, editor-core, persist, Python, demos) and
what each needs from a carrier + interval form — then an `[ev]` PR
against PATHS-DESIGN §2a.1/§6 (the M2 closed-carrier precedent) with
the design choices it surfaces.

Orchestrator branch is the session's assigned branch, not
`paths/orchestrator` (the remote session names it); unit branches keep
the `paths/` prefix.

The track is over budget (38.5/30). Splitting along the priority seam
is deferred until the lowering survey says how many rows it absorbs or
spawns — several slate rows (the closers, `circle_split`, the lift
comparator) may change shape under a carrier + interval lowering.

## 2026-09-25 — the lowering survey is in; `[ev]` PR opened

The survey is kept on the row itself
(`lower-profiles-to-carrier-and-interval-not-vertex-and-bulge.md`,
"Survey"). Corrections to the row as filed: D1's "Profile format"
clause is touched, not only PATHS-DESIGN §2a.1; geom-brep's
`SketchSegment` carries its own copy of the bulge form and every
profile-built edge goes through it; the saved file holds programs, never
the lowered form, so the only format break is in names; the symbolic
tier keys on the circle's unit bulge. There are seven hand copies of the
bulge→carrier formula, not three.

The `[ev]` PR re-words D1's Profile-format clause, PATHS-DESIGN §2a.1
and the profile README to the recommended form (A2: verbatim vertices +
`Line | Arc{centre, radius, Δθ}`, consistency verified at validate) and
asks the forks. Settled on #3202 and not re-asked: EMIT ships step ids
first with `Piece(0/1)` circles and takes the second names break.
Proposed unit cut, 0 → 6, is in the survey's §5.

## 2026-09-25 — Ev's first round on the lowering `[ev]` PR

Ev agreed q2 (geom-brep follows the profile form) and q4 (`Bulge` stays
as a path-algebra arc mode; `RawLoop` keeps vertex + bulge as its input).
On q1 Ev asked whether a zero-redundancy form exists, e.g. three points.
The PR body's "no zero-redundancy form works" was too strong and is
corrected: counting dof, a full turn is the blow-up of a = b in the
partial-arc family, so every condition-free form (Z: bulge or via point
plus a separate full-turn arm) reads a full turn differently, and A2
trades that split for a verified carrier. A2 is still recommended, and
Z is offered as coherent. On q3 Ev asked whether `circle` should be
sugar for `circle_split(n = 1)`. The proposed answer is one kernel,
with `circle` kept as its own verb in the program so it reads back as
written. Both q1 and q3 await Ev.
