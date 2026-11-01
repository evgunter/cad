# REACHHOLD — the plan

REACH's leftovers: the in-band reach stage 4 released, the census at the door, and the P4 tail.

REACH's closing cut on its priority seam (`work/README.md`, Track size)
opened REACHHOLD and REACHTAIL on 2026-10-08. REACH then measured 139
budget points against 30, with its six charter rows closed. REACHHOLD's
rows were parked on the D10 hold. INTENT's stage 4 E (glue on Zero) and
F (`declared-pairs-retire`) released them on 2026-10-09 and 2026-10-11,
and each released row carries a dated note saying what E or F left.
REACHTAIL's rows moved in on 2026-10-26 by `mv`, ids and bodies
unchanged, because both tracks were REACH's leftovers on one ground and
together they come in under one budget. REACHTAIL's plan is folded in
below, and `docs/doc-ledger/reachtail-leaves-the-tracker.md` records it.
Nothing has been dispatched.

## The slate

**29 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P2 | `a-settled-declared-coincidence-crosses-a-tight-volume-bound` | H | A Zero glue (formerly a declared in-band pair) moves a correct result past a tight volume bound, and the backstop refuses it |
| P2 | `maximal-faces-curved-arm-cannot-tell-a-licensed-curved-skip` | M | The maximal-faces gate has no curved arm; the rounded stack no longer ships a skip, so re-measure first |
| P3 | `a-box-corner-on-a-declared-tangent-ruling-refuses-curved-boolean-unsupported` | M | A box corner on a fillet's tangent ruling refuses CurvedBooleanUnsupported |
| P3 | `an-uncovered-edge-tangent-to-a-fillet-at-the-curved-operands-vertex-refuses` | M | A point touch at the curved operand's vertex refuses in both orders |
| P3 | `covered-endpoint-arms-read-a-non-convex-touch-at-the-ends-only` | M | The covered endpoint arms read a torus line or an arc at its ends only |
| P3 | `rounded-stack-subtract-and-intersect-refuse-fallback-extent` | M | Aligned continuations refuse FallbackExtentUnsupported: no certificate of abutment |
| P3 | `edge-face-crossing-cut-and-pass-five-decide-one-crossing-twice` | M | A straight crossing is decided twice, and nothing ties the two |
| P4 | `apply-cut-ins-walks-its-loops-twice-and-overloads-its-predicate-names` | M | apply_cut_ins walks the loops twice and overloads one predicate name |
| P4 | `line-quadric-root-code-and-the-cone-form-have-several-homes` | M | Line × quadric root code has two homes, the cone form three spellings |
| P4 | `full-turn-question-has-three-spellings` | E | An arc's gap to a full turn is asked under four spellings |
| P4 | `sphere-chart-trim-folds-any-number-of-rim-levels` | E | sphere_chart_trim misreads a stepped outline; no door builds one today |
| P4 | `split-insert-crossings-second-edge-clears-arm-is-unpinned` | E | insert_crossings re-checks edge_clears behind the gate's check, unpinned |
| P4 | `the-sweep-preamble-and-the-cone-extreme-read-are-copied` | E | sweep_split is a third copy of the sweep preamble; conic_clearance re-spells the extreme read |

Parked, each on its named trigger:

- `boolean-door-runs-the-census-over-its-result` (P0 H): parked on
  RESTREAD's `census-cross-solid-curved-pairs-undecidable-on-shell-results`.
  FLUSH's two P0s are parked on this row.
- `the-plate-document-never-cuts-its-holes` (P3): parked on TALLY's
  `a-hole-wholly-inside-its-target-ties-the-subtract-volume-bound`.

Deferred: `spiric-operand-edges-reopen-with-their-first-producer` (its
body cites the ruling).

## Order

- **Measure first, every stage-4 row.** Each released row says what E or
  F did and did not re-measure. The declare surface they were filed
  against is gone, so every repro that says "declared" is now
  undeclared: the glue door decides the pair itself.
- `maximal-faces-…` before any curved-arm build. Its witness, the
  rounded stack's recorded skip, no longer ships.
- `a-settled-…` is a design row (`design: true`). Its forgiveness has to
  be local to the glued face, and it shares that shape with HONE's
  `the-subtract-volume-backstop-holds-flush-operands-bit-exact-and-refuses-a-correct-bar`
  and TALLY's `a-hole-wholly-inside-its-target-ties-the-subtract-volume-bound`.
  Weigh the three together.
- The P4 tail batches into one or two lanes.
- When a parked row's trigger fires, the row is built here or goes to
  the live program whose charter it fits.

## Ratified ground (cited, not re-litigated)

- `docs/DESIGN.md` D1–D10 as ratified; the DEV-1 set; the C6
  interference-fit era; the merge door's `RecordsASkip` doctrine (#2105).
- Shared ground: `crates/topo/src/boolean/*` and `crates/topo/src/splitting/*`
  are shared with REACH's sibling successors (ORBIT, ROOTS, TALLY, APEX,
  GAUGE) and with CLEAVE, HONE, GERM, ZIP, BOXES, PIN, TANG, SECT and
  FLUSH. Run `scripts/work.py territory` on your branch and announce
  the seam in the PR.

## Review posture

REACH's, inherited. The review tiers are those of
`memories/orchestration-model.md`: style, full single, or a dual under
`docs/DUAL-REVIEW-PROTOCOL.md`, logged in `docs/DUAL-REVIEW-LOG.md`. Any
unit a review passes with fixes gets an independent verifier. A lane
measures its row's repro on current main before it builds, because the
ground has moved since most of these rows were filed.

Two practices REACH's last sitting learned the hard way:
- every worktree builds in its own target directory (a shared
  `CARGO_TARGET_DIR` serves one tree's crate to another's build);
- the ε knob is `CAD_TOLERANCE_EPS`.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`,
so the program closes without an exit walk.
