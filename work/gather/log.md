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
