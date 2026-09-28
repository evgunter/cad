# GATHER — the log

## 2026-09-20 — opened

Cut out of WIRE, which was carrying 85.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed WIRE's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

13 rows arrived by `git mv` with their ids, bodies and history
unchanged. WIRE keeps its band 3700-3799; band 8400-8499 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## 2026-09-24 — an orchestrator picks the track up

Status `ready` → `active`. This orchestrator runs in a cloud session, so
PR subscriptions and scheduled check-ins stand in for the local monitors.
Unit lanes push `gather/<unit>` branches and open one PR each.

**Review posture, the question `plan.md` left open.** Each unit's tier is
set at spec time under `memories/orchestration-model.md`'s review tiers.
Every lane runs on Opus. The v7 triage stream is not re-opened for this
slate.

**First wave, run in parallel:**
- `product-refuses-naming-when-one-instance-is-placed-under-two-roots`:
  **single FULL review**. It adds an earlier refusal in the recipe's
  vocabulary and removes a late one, so correctness is at risk, but the
  design decision is already made (PR 2677).
- `member-space-look-through-stops-at-splits-containment-and-fragmented-merges`:
  **DUAL review**. It adds typed refusal vocabulary to the naming layer,
  which every member-space declaration reads and which would be hard to
  change later.
- The six `E` rows go in one batch unit, as `plan.md` suggests:
  **single STYLE review**. They are mechanical and local.

Load is 31/30. It is not split: this wave takes about 16 points off the
dispatchable count, which leaves a slate one session can hold.
`parallel-node-map-loses-the-funnel-and-the-symbolic-session` is next.

## 2026-09-28 — the name-carrying edge set lands (PR 3321)

`three-walks-over-the-name-carrying-edges` and
`select-refusal-coverage-is-not-compiler-enforced-from-the-test-crate`
closed on PR 3321. `verbatim_edge` (`names/role.rs`) is the one home
both editor-core walks read; the compiler holds the three together.
`SelectRefusal` has an in-crate census beside the enum.

Single STYLE review, verdict mergeable. All four claims held. The
reviewer mutated the census and confirmed it goes red for a missing
sample. Fix pass: S1 (the doc no longer says "every walk"), S3 (the
display-contract comment says what the census covers), S4 (the
census uses `test_utils::census::set_difference`), S6 (`Intact`
carries no unused field). No change: S5, S7, S8, S10.

Filed from the review, as classes:
- `verbatim-edge-is-not-tied-to-the-evaluator` (P1, M). Nothing ties
  `verbatim_edge` to what `eval/wire.rs` passes through; a runtime
  guard over the evaluated corpus is possible. The lane's "no runtime
  test can exist" was corrected in the PR body.
- STACK's `in-crate-census-hand-writes-the-set-comparison` (P4, E).
## 2026-09-28 — the second wave, and the rows re-priced

The first wave closed: #3141 (the E batch), #3142 (two roots),
#3143 (member-space look-through), #3145 (parallel node map), #3256
(split halves as roots), #3259 (the tag-vocabulary macro), #3264 (step
arg roles) and #3280 (the per-part gate's home). Residue filed along
the way is the slate now in `plan.md`.

The seven open rows were priced `D`, which before 2026-09-27 was
sometimes used to mean medium effort (#3308, Ev in chat). Re-priced by
reading each body:

- `product-gate-refuses-a-declared-cusp-sweep…` → **H**, no design flag.
  The decisions it lists (the record's shape on the node value, which
  gate reads it) are technical ones inside this program's own policy,
  and whether the census certifies a curve contact between two faces
  of one body is unmeasured, which is what makes it hard.
- `three-walks-over-the-name-carrying-edges` → **M**; its fix is stated.
- `select-refusal-coverage…` → **E**; its fix is written out in full.
- `product-per-part-gate-counts-solids…` → **E + design**; the row says
  so itself: option (b) changes a refusal callers observe.
- `the-gather-tie-merge…`, `loft-path-loses-nine-predicate-families…`
  → **M + design**; each needs a mechanism chosen (candidate identity;
  one of three dispositions across two programs).
- `wire-rs-accumulation-residue…` → **M + design**; whether a
  comment-ratio budget exists at all binds future work.

Load 17.5 → 17.

**Second wave, dispatched 2026-09-28.** Every lane runs on Opus.

- `product-gate-refuses-a-declared-cusp-sweep-the-verb-now-declares`
  on `gather/cusp-sweep-gate`: **single FULL review**. It moves a gate
  onto the declared door and carries a record across two crates, and
  whether the census certifies a curve contact inside one body is
  unmeasured, so believing it takes more than reading it.
- `three-walks-over-the-name-carrying-edges`, with
  `select-refusal-coverage-is-not-compiler-enforced-from-the-test-crate`
  riding along, on `gather/name-edge-home`: **single STYLE review**.
  Both fixes are stated in their rows, and the walks' semantics do not
  change.
- `product-per-part-gate-counts-solids-but-gates-sources`: to the two
  designers (Opus and Fable, `docs/prompts/designer.md`) before
  anything is built, because option (b) changes a refusal callers see.

The other three `+design` rows wait for this wave to land.

## 2026-09-28 — the per-part question converges; the cusp row forks

**Per-part gate: decided by the designers, not taken to Ev.** Both
designers (one Opus, one Fable) rejected the row's framing ("which
count does the product pass"). The product's per-part gate re-reads
every face the aggregate gate reads, for every product of two or more
solids, not only the lone multi-solid one. That is PERF-SCAN-2026-08
item 16 (`work/perf/plan.md`, "Tier-3 runs twice on the product path")
seen from the refusal side. First reports differed on attribution: one
re-gated sources on refusal, the other mapped the aggregate's findings
back to roots through `solid_roots`. Round 1 showed each the other's
report and the two contradicting claims. Executed and read: no
`ValidationError` accessor yields a solid (35 of 75 variants carry no
face, edge or solid key), and tier 3's early stops are body-wide, so a
grafted A+B reports A's `LoopRoleInverted` and not B's
`NegativeVolume`. The mapping designer withdrew, and both converged:

- gate the aggregate once; on refusal re-gate EVERY source and refuse
  with one finding list naming each failing root and output index;
- `SolidInvalid` retires into that list; `ProductInvalid` remains only
  for "the aggregate refused and every source passes" (a graft defect);
- the product stops calling `topo::per_part_gate_owed`, which keeps its
  STEP-import caller until `work/exch/the-per-instance-tier-3-gate-reads-every-assembly-face-twice.md`
  is answered (that row owns the ratified step-4 question).

No ratified text binds the product's two-gate shape, and with the two
converged there is one clear answer, so it proceeds as a unit with its
argument in the PR body. It changes a refusal callers see
(`ProductErrorKind`, the `pncad-py` tag, the viewer's kind lists); Ev
was told in chat. Not a design-fork row (the protocol counts forks put
to Ev). Re-priced E+design to M. Dispatched on
`gather/per-part-aggregate-gate`, **single FULL review**: the refusal
vocabulary moves across three crates. The body-wide suppression is
filed as ATREST's `check-7-stops-body-wide-so-one-solids-defect-hides-anothers-orientation`.

**Cusp sweep: stopped at a fork, now with the designers.** The lane
measured before building. The census certifies a strut or rim
`CurveContact` for extrude and revolve (revolve has the extrude defect
too), but refuses a loft's NURBS seam as
`CensusUnsupported(NotCertifiable)`. A cusp loft gathers today only
because tier 3 exempts NURBS edges by kind, so the row's shape would
turn a working document into a refusal. Row back to `open`, flagged
`design`. A designer pair was dispatched on the problem and the
measurements (branch `gather/cusp-sweep-gate` holds the probes); this
one may go to Ev, so its blinding byte is on
`analysis/design-fork/sweep-cusp-declarations` (drawn after dispatch,
before any report; disclosed there).

## 2026-09-28 — the product gates its aggregate once (PR 3323)

`product-per-part-gate-counts-solids-but-gates-sources` closed on PR
3323, implementing the design the two designers converged on. The
product gates the aggregate once, after every graft and before every
naming refusal. On refusal it re-gates every grafted source and refuses
`RootInvalid`, naming each failing root and output index with its own
findings. `SolidInvalid` retired. `ProductInvalid` is only the
graft-defect arm now. The product no longer calls
`topo::per_part_gate_owed`; STEP import is its only caller.
PERF-SCAN-2026-08 item 16 is marked resolved, with a source guard that
the gather gates once on success and re-gates only on refusal.

Single FULL review, verdict mergeable after fixes. All six claims held
under probe: a pattern's instances 0 and 2 are named, and a `.take(1)`
mutation reddens the cascade row. The fix pass took every item:
- rows pinning non-zero output indices;
- the r2_m10 `ProductInvalid` widening reverted;
- one skip rule instead of two;
- a header that counts roots;
- the stale "two callers" prose made true;
- the body-wide-stop argument given one home;
- the gate-once guard;
- the PR body now states the `SolidInvalid` → `Graft` class change for
  malformed sources.

CI was red at the first head on `test-utils`'s reader census. The
ledger named the renamed `per_part_gate_policy.rs`, and the lane had
not run the `test-utils` tests. The fix pass re-pointed the ledger.

Left as they are, noted: three near-parallel (root, output) attribution
shapes (`SourceFinding`, `SolidOrigin`, `CheckFinding`); Python binding
only the first failing root as `node`.

Filed by the lane and priced here: `assemble-runs-the-tier-3-local-battery-twice-on-one-aggregate`
(P3, M), which waits on the cusp row. The cusp row's pin now reads
`RootInvalid`.

## 2026-09-28 — the pass-through set is tied to the evaluator (PR 3335)

`verbatim-edge-is-not-tied-to-the-evaluator` closed on PR 3335. A
corpus guard checks every evaluated node against `verbatim_edge`'s
variant: `Whole`/`Selected` publish only rows other nodes head,
`Intact` publishes both, and `None` publishes none foreign. A census
forces every `Node` kind into the corpus with rows, or onto an explicit
row-free list asserted to publish none. `Sweep`, which never evaluates,
and `Shell`, which sits beside the registry, have one home in
`tests/corpus`, read by both corpus suites.

Single STYLE review, verdict mergeable. The reviewer's probes showed
the first guard blind to which edge a node returns (`Part`/`Transform`
misfiled as `Intact` stayed green; the walk suites caught them), and
six kinds sampled vacuously. The fix pass took all four items: the
guard reads the variant, row-free kinds are explicit, the frontier
has one home, and the two checks are separate tests. The lane
red-proofed each misfile.

CI was red on `main` meanwhile: a 3322 × 3331 semantic merge in
`topo`'s window-site census. Fixed by PR 3337 and ported here.

Filed: TINT's `corpus-node-kinds-roster-is-hand-written`
(`corpus::NODE_KINDS` is not welded to `Node`).
