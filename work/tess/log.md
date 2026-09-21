# TESS log

## Opened at S-MESH's exit (2026-09-16)

Opened by S-MESH's orchestrator in the PR that proposes
`docs/S-MESH-EXIT-WALK.md`, as `work/README.md`'s closing rule directs:
S-MESH's open mesh findings, its parked MESH-9 and its Track R rows
cohere into one track on one territory (`crates/mesh`,
`topo::coherence`), so the closing program opens the successor and
moves them here. Band 5100–5199 recorded in the ledger's banding entry
in the same commit. No unit dispatched; the first sitting picks from
`work/tess/plan.md` §The slate (the NURBS face bound's unsoundness
first — a wrong certificate outranks every style row). Ev's sign-off
on the exit walk ratifies the opening.

## First sitting (2026-09-18)

Orchestrator seated; branch `tess/orchestrator`, in a lane clone
(`cad-work/tess-orchestrator`), not the main checkout. S-MESH's exit is
ratified (`docs/DOC-LEDGER.md` sweep 16), so the opening stands.

**Sequencing (mine, per the standing rule; alternatives recorded).**

1. `nurbs-face-bound-unsound-on-a-random-rational` first, as the opening
   entry directs. It is TWO defects by the row's own evidence — a 1.7 %
   `vv` excess (a wrong bound) and a two-ULP `uu` excess (a rounding
   comparison) — so the unit is preceded by a **diagnostic lane**
   (`tess/nurbs-bound-diag`, no fix, no PR, no A/B row): reproduce both
   seeds, localise which hull claim of the rational recurrence the truth
   violates, decide for the ULP case whether the certificate or the
   test's bare `<=` is the defect, and measure the failure rate across
   seeds. The spec is written from its report. Rejected: dispatching an
   implementer straight at the row — the row's cause is inferred from
   the assertion text, and `patch_bound.rs` is PROPS', so which program
   owns the fix is not known until the wrong step is named.
2. Then the door/refusal rows, because each is a shape a user can reach
   today: `rim-only-sphere-cap-panics-at-census` (release build emits a
   non-manifold patch silently — measured first), then
   `tessellate-refuses-approx-face-without-caches` and
   `lofted-circle-sections-…` (the latter's two cheaper remedies are
   outside `crates/mesh`; its owner is settled when it is specced).
3. Then the contract rows (`mesh-materialized-form-is-writable`,
   `degenerate-triangle-normal-is-substituted` with its rider,
   `tessellation-chart-frame-…`, `dist-line-triangle-…`), then the
   test-tightness rows (D300, S237, S236), the small ones (D303, D304,
   `memo-dumps-…`, `chart-azimuth-…`, `one-element-grid-axes-…`, C23
   premise-first), and S28 last — it is the largest and every other row
   moves the code it would unify.

**Review tiers (Ev, in-chat, 2026-09-18).** TESS takes EDIT's tiers:
full v6 dual on a kernel unit; style review plus a correctness arm, no
row, for an easy-but-not-trivial unit; green CI and the orchestrator's
read for a rename or prose row. Written into `plan.md` §Process.

**Plan rot fixed.** `plan.md` said the Track R rows "land per §D's
conventions" and wanted them "empty in §D" at exit; the register §D
belonged to left the tree on 2026-09-11 (`work/README.md`, sweep 11) and
Ev did not recognise the reference. The clauses now say what they can
mean: the seven rows are closed in `work/tess/`.

## The diagnostic lane's report (2026-09-18)

`tess/nurbs-bound-diag` at `db4cb45a8`; report adjudicated the same
hour. **The first row was misfiled by its own evidence**: the assertion
prints `(sampled) vs (bound)`, the row read it the other way, and the
"1.7 % wrong `vv` bound" never existed — both hosted reds are one
defect, a 2-ULP `uu` excess on a bilinear rational patch. I checked the
message order and the refinement site against the tree before building
on either.

- **Cause**: `geom_brep::patch_bound::rational_cells` hulls the
  f64-refined net, so the described patch escapes the enclosure by
  insertion rounding ×16. Proven in exact rational arithmetic (the
  described surface's true `‖S_uu‖` exceeds `muu`). PROPS' file —
  filed on PROPS' slate with the numbers
  (`rational-cells-hull-the-f64-refined-net-…`), and the TESS row
  corrected in place with what stays ours (labelled 17-digit assertion
  messages; the header's "end to end" sentence; a bilinear-stratified
  census as the closing evidence).
- **CLASS, not instance — an assertion message that cannot show the
  margin it failed on, or which side is which.** Two in `mesh`
  (`r1_random_rational_soundness_sweep`, `assert_dominates`); the unit
  that fixes them sweeps `crates/mesh` for two-tuple `vs` messages and
  sub-17-digit formats on domination asserts and says what the sweep
  could not match.
- **Second finding filed**:
  `exact-zero-second-partial-leaves-cell-component-as-subnormal-dust` —
  `cell_component`'s load-bearing `== 0.0` arm is dead because the
  ring's `add` widens `0 + 0`.
- **Sequencing consequence.** The kernel fix is PROPS'; TESS does not
  cross into `patch_bound.rs` while PROPS has a live orchestrator. TESS'
  unit on this row is the message fix (bottom tier: test text only),
  dispatched now; the row stays open on PROPS' fix. Next kernel unit is
  `rim-only-sphere-cap-panics-at-census`, measure-first.

## Dispatched (2026-09-18, after PR 2845 merged)

- `tess/domination-assert-messages` — the message unit on the NURBS
  bound row (test text only; bottom tier, merges on green CI and my
  read of the hit list).
- `tess/rim-only-cap-diag` — measure-first on
  `rim-only-sphere-cap-panics-at-census`: what release emits, whether
  any of the kernel's own verbs mints a rim-only pole-containing face,
  and whether the sphere is one instance of a class (cone apex,
  seamless cylinder wall). The row's design question — admit and emit a
  cap, or refuse at a door — is decided from that report; if the facts
  leave both viable it goes to Ev as an `[ev]` PR, since a door's
  premise is PROPS' predicate and D2's refusal vocabulary.
- The diag lane's clone (`cad-work/tess-nurbs-diag`) is kept until
  PROPS answers on its row; its evidence is pushed.

## The cap survey adjudicated; TESS-1 specced (2026-09-18)

`tess/rim-only-cap-diag` at `83833e586`. With assertions off the face
meshes as a HOLE (`Ok`, zero triangles); in-repo release still panics
because `[profile.release]` keeps `debug-assertions`. The reach is STEP
import (since PR 2741) and the Euler door; no native verb mints the
face; it is a class (cone apex cap, one-rim cylinder). Numbers and
corrections are on the row.

- **TESS-1** (`docs/TESS-1-SPEC.md`): the class refuses typed on the
  structural fact — no meridian traversal. Pre-draw fields, logged
  before the block draw: **difficulty M, task-class structural** (the
  refusal reads the traversal list; no float decides it). Block
  TESS-B1, record branch-side on `tess/b1-block`.
- **Fork for Ev** (`[ev]` PR, `needs_ev` on the cap row): mesh learns
  the interior pole and emits the sphere cap, or import normalizes the
  rim-only statement into the seamed form and TESS-1's refusal stands.
  TESS-1 is true under either, so it does not wait.
- Filed: `check-mesh-passes-the-empty-mesh`.
- CLASS noted for the record: **a row's prose about another program's
  door goes stale the day that program lands** — the cap row said
  "import route dead" two days after PR 2741 opened it. Measure-first
  caught it; nothing else would have.

## Announced from TRIM (2026-09-20): a filed class on this slate

`chord-count-arithmetic-is-plain-f64-across-every-speed-arm` moved
here from `work/trim/` at TRIM's 2026-09-20 cut: TRIM-2 PR-2's dual
found that every chord-count arm in `crates/mesh/src/chords.rs`
multiplies a certified sup by a span in plain f64 and ceils it, with
`next_up` on the sup the only outward pad — a class, not that unit's
defect — and that the domination idiom is spelled four times in the
crate with the newest spelling (interior-knot sampling, vacuity
refusal) the one to propagate. The item names TESS the owner; no
action asked of TESS while paused.

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

**One row: `chart-coherence-ships-off-and-nothing-schedules-turning-it-on`.**
`CheckId::ChartCoherence` ships with severity `Off` — the only default on
`ChecksConfig` that is not `Warn` — which answers the no-consumer row it
closed with a check nobody runs.

It lands on TESS because both measurements that would decide the flip are
about `crates/topo/src/coherence.rs`, which territory says is yours: (1)
the resident's per-run cost, which nothing has measured though it reads
every face of every rest body, against a registry whose per-resident cost
is PINNED at `docm5_subject::the_registry_split_is_measured_at_a_pinned_point`;
and (2) whether the examination gets a shape door, without which
`Unexaminable::NonIsoCarrier` puts **two loops per body** into `unexamined`
for an ordinary trimmed cylinder — a lane boundary, not a defect, and not
actionable by the reader who would see it at `Warn`.

**The orchestrator's judgement, unchanged by the move: `Off` is right
until those exist.** `mesh8_corpus_coherence`'s own header states the
rule — *"A report that fires everywhere is not a report"*. The row exists
so that right-for-now does not become permanent silence by default.

The severity default itself is one line in
`crates/editor-core/src/checks.rs`, which territory says is FIX's — so
the flip, if it comes, is announced to FIX. What gates it is entirely
yours to measure.

Signed (FIX orchestrator).
