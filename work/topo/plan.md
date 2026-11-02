# TOPO — the topology core (plan)

**STATUS: OPEN (2026-09-04).** Live state is `log.md`'s tail and the
item files beside this plan, never this file.

Branch prefix: **`topo/`**. Away-channel tag `(TOPO orchestrator)`.
A/B band **2700–2799** (`docs/MODEL-AB-LOG.md` owns every live
experiment number).

## Why this program exists

`docs/WORK-TRACKS-2026-09.md` (the 2026-09-03 cut) recorded that **no
program owns 37 `crates/topo/src` files**, and cut eleven tracks around
that gap without closing it. Re-measured on 2026-09-05 against merged
main: **55 of the 102 `.rs` files under `crates/topo/src/` fell in no
open program's `paths`** — 35 of them still do, this program having
taken the other 20. (PR #1899 said "47", a figure read off one line of
a directory-grouped scan that dropped the `review_m1_pr2/` and
`seqgen/` subdirectories; corrected here by re-deriving.) It is the
largest unowned kernel territory in the tree, and it is where the
Euler surgery, tier-3 validation and the birth-identity channel live —
the layer every other kernel program builds on.

The same gap shows up on the code-quality side. **Track P** — fourteen
open rows whose fence is exactly `euler*.rs`, `validate.rs`, `live.rs`,
`seqgen.rs`, `merge_faces.rs` and the review/fixture readers — is
listed as claimed by "this program" (code-quality itself) and **has
never had a lane**: no `smell/p-*` branch has ever existed, while K, X
and T lanes have run. The rows are not stalled on a ruling or a
dependency. They are stalled on an owner.

This program is that owner, and it takes Track P whole, per the cut's
rule that a program claiming a code-quality letter takes the whole
letter so the schedule stays single-owner.

## The fence

The `paths` list is **enumerated file by file**, not globbed. That is
deliberate and it is the one thing a reader should not "tidy":
`crates/topo/src/*` is an `fnmatch` glob that crosses `/`, so it would
silently double-claim `boolean/`, `splitting/`, `query.rs`,
`coherence.rs`, `shell.rs`, `pcurves.rs` and `props.rs` — five
programs' ground plus two code-quality letters — and
`scripts/work.py territory` **cannot see a double claim** (that is the
finding in `work/meta/territory-cannot-see-a-path-two-programs-both-claim`,
raised by exactly this failure between FIX and SHELL). Until that lint
exists, the only honest fence is an enumeration.

What this program does **not** take is written in `program.md`'s
`keep_out` in full. The two seams worth naming here:

- **`query.rs` is SEAT's.** `face-kind-read-has-two-homes` is a
  question about a pair of doors, one of which is SEAT's. The ruling
  can be made here; the edit on SEAT's side is announced on SEAT's
  board before it lands, and never taken silently.
- **The remaining 35 `topo/src` files are unowned and NOT finished.**
  `body.rs`, `entity.rs`, `geometry.rs`, `instance.rs`, `null.rs`,
  `lib.rs`, `contact.rs`, `separation.rs`, `ray_walk.rs`,
  `offset_axial.rs`, `iso.rs`, the four `sector_*`/`chart*` files, the
  eleven `review_m1_*`/`review_d21`/`r2_probes` readers,
  `test_support_impl.rs`, and `param_source.rs` (new on main with
  SEAT-7). Three of the 35 — `chart_region.rs`, `face_normal.rs`,
  `props.rs` — sit inside a code-quality letter's fence (Q, Q, M) while
  no *program* claims them, which is a different kind of unowned and is
  the letter-holder's to take. A row landing on one of them draws the fence
  in the PR that mints the row — the rule the code-quality plan states
  for the `geom-brep` seam. Extending this program to the whole crate
  is a later decision and not this opening's.

## The slate

Twelve live rows, P2 to P4, three of them with no `cost` (`python3 scripts/work.py
status --program topo` is the live view, and this file does not restate
it). The opening slate (the three re-homed issues and code-quality Track
P's fourteen rows) has closed or moved to the 2026-09-20, 2026-10-11 and
2026-10-26 successors; the last of those sent the describing-door family
to RESTATE (`work/topo/plan.md`) and PIPE's two layering rows to
CORES. What is left falls into three groups:

- **P2, on the goal's ground.** `revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet`
  (H: the re-pairing CLEAVE's PR 4795 ruling unblocked),
  `mfkrh-leaves-an-orphan-surface-under-the-per-op-scalpel`,
  `movefac-torn-bodies-fuzz-seed-panics-at-the-door-postcondition` and
  `a-lone-site-placeholder-has-no-type-of-its-own` (a representation
  change; designers first, and Ev weakly prefers to hold that round until
  D10 settles).
- **Endings and the lift** (P3): `split-param-not-interior-…`,
  `merge-door-bend-ending-…`, `merge-faces-undescribed-ending-…`,
  `offset-derive-swallows-…` (E each but the bend ending), and
  `euler-driver-lift-is-held-by-convention` (M).
- **Cost and hygiene** (P4 and unpriced):
  `a-joined-finisher-mints-the-whole-body-twice`,
  `topo-internal-stored-curve-ladders-…`, and topo's lib-test compile
  cost (`topos-unit-tests-recompile-…`).

## Order

By priority, with the cheap E rows taken as drive-bys:

- The P2 rows first. `revert-re-pairs-…` is a unit of its own.
  `mfkrh-…` and `movefac-…` are a replay and a fix each.
- The endings and the lift next; the E endings ride beside any unit.
- Cost and hygiene last. Price the three rows with no `cost` at pickup.

## Review posture

Per unit, at spec time, on the tiers in
`memories/orchestration-model.md`. The orchestrator's read covers
mechanical changes; a single review (style, or full with claims to
falsify) is the default; a dual review covers especially tricky logic
or an architectural decision that would be hard to change later. The
tier and its one-line reason sit on the unit's item or in the log at
dispatch. A dual is a row in `docs/DUAL-REVIEW-LOG.md`, run by
`docs/DUAL-REVIEW-PROTOCOL.md`.

## RESTATE's slate, merged in 2026-11-02

RESTATE merged into TOPO on 2026-11-02 (`docs/doc-ledger/restate-leaves-the-tracker.md`). Its plan follows as it stood, headings one level down; its rows are now in `work/topo/`.

Opened 2026-10-26 by TOPO's priority-seam cut. TOPO's history is its
own `log.md`; this slate was TOPO plan's "describing-door family".

Branch prefix: **`restate/`**. Away-channel tag `(RESTATE
orchestrator)`. A/B ordinal band **16600–16699**.

### Charter

A door that moves an edge, loop or run from one face's chart to
another's, or mints an edge under a caller's description, has to leave
two things true: the edge's description names the surfaces of the faces
it bounds, and every pcurve row the destination face stores is
certified against the chart that face now wears. The rows here are the
places one of those two is not yet true, or is true only by a caller's
ordering.

### The slate

All P3 unless marked. `python3 scripts/work.py status --program
restate` is the live view.

- **The kills across keys and the restater** (ruled in outline on PR
  3970, Ev 2026-10-06; details wait on the intent refactor):
  - `kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers`
    (design; the PR 4562 fork F1/F2 is open on it);
  - `restatement-derives-each-moved-edges-kind`.
- **The describing door** (`set_face_surfaces_describing`, `attach.rs`):
  - `set-face-surfaces-describing-keeps-a-moved-edges-rows-on-a-kept-chart`;
  - `the-describing-door-refuses-a-wall-re-charted-reversed-in-u`;
  - `a-listed-spec-cannot-name-a-fresh-chart-a-neighbour-keeps-the-old-key-of`;
  - `set-edge-curve-scans-the-edge-arena-per-call` (E, its cost).
- **The moving doors that half-mint** (`kef`, `kfmrh`, `ring_move`,
  the `_minting` bands):
  - `a-moving-door-carries-minted-rows-onto-an-unminted-face-and-half-mints-it`
    (design);
  - `a-moving-door-leaves-a-complete-spline-destination-half-minted`
    (design);
  - `kef-minting-clears-a-complete-face-whose-merged-loop-it-cannot-chart`.
- **The minting doors and the re-basing gate:**
  - `minting-doors-take-a-callers-description-unasked-where-the-new-face-keeps-the-key`;
  - `the-re-basing-gate-reads-a-derived-image-where-the-mint-read-the-stored-one`
    (unpriced; price it at pickup).

### Order

One designer-first thread: the rows share one question, how an edge a
door moves gets described and which rows travel with it. Re-read
`kef-and-kfmrh-across-keys-…`'s ruled outline and the PR 4562 fork
first; the half-minting rows and the restater follow from its answer.
The describing-door rows and the E cost row can be taken beside it.
Where a row's detail is one Ev said the intent refactor will change,
check INTENT's state before building.

### Review posture

Per unit, at spec time, on the tiers in
`memories/orchestration-model.md`, as TOPO's: the orchestrator's read
for mechanical changes, a single review by default, a dual for an
architectural decision that would be hard to change later (the
restater is one). The tier and its one-line reason sit on the unit's
item or in the log at dispatch.

### Exit shape

Every row merged or re-homed with a reason. No exit criteria are set,
so no walk is owed.

## TOPOTAIL's slate, merged in 2026-11-02

TOPOTAIL merged into TOPO on 2026-11-02 (`docs/doc-ledger/topotail-leaves-the-tracker.md`). Its plan follows as it stood, headings one level down; its rows are now in `work/topo/`.

Opened 2026-10-11 by TOPO's priority-seam cut. TOPO's history is its own `log.md`.

### The slate, in the order that unblocks the most

1. **The binding value, P2.** `valued-offer-raised-at-a-loops-first-in-band-reading-is-not-the-binding-one`: D4 ¶1 (i) offers a tolerance that does not decide the decision.
2. **Torn records read as absent, P3.**
   - `torn-hops-read-as-absent-across-the-boolean`
   - `torn-body-refusal-families-beyond-the-six-doors`
   - `public-body-option-getters-answer-none-for-a-torn-record`
   - `vertex-orbit-answers-part-of-a-torn-orbit` (P4)
3. **Refusals that end on their lever or say the wrong thing, P3.**
   - `circle-torus-lane-escalates-without-its-rung`
   - `coincidence-tangent-locus-contact-section-escalate-without-their-rung`
   - `curved-pierce-frontier-tells-one-story-for-several-decisions`
   - `classify-dihedral-callers-do-not-all-establish-its-spline-premise`
   - `merge-coplanar-refusals-open-with-a-stage-label`
   - `boolean-unreadable-norm-ends-as-a-kernel-defect`
   - `offset-held-neighbour-image-keeps-a-declared-chart-on-a-transverse-section`
   - `dihedral-arm-lever-says-longer-for-an-arm-built-from-the-extent` (P4)

   Read each against INTENT stage 4 first: a row whose recourse offers a declaration may close with `declared-pairs-retire`.
4. **Cost, P3.** `the-site-mints-plan-reads-the-rewired-loop-whole-on-every-op` (design).
5. **Dead code and mismetered levers, P4.** The rest; each fix is written in its row.

Parked: five boolean declaration rows on `intent-stage4-is-built`; `a-boxed-rotation-refuses-not-rigid-at-every-placer` on `intent-stage3-is-built`.

### Review posture

The repo-wide tiers apply (`docs/DUAL-REVIEW-PROTOCOL.md`, `memories/orchestration-model.md`):
- an M unit gets one review, plus a second after the fix pass if the first raises a MAJOR;
- E rows take the orchestrator's read.

### Exit shape

Every row merged or re-homed with a reason. No exit criteria are set, so no walk is owed.
