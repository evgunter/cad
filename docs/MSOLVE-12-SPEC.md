# MSOLVE-12 — Near-parallel planes in their own words; a mate's `Band` reached by state; the maintenance's own file (spec)

Unit of the `msolve` program. Item `work/msolve/MSOLVE-12.md`. It
gathers three rows:
- `near-parallel-planes-refuse-under-a-false-predicate` (P0);
- `mate-band-fault-unreachable-on-a-mate` (P0);
- `mate-solve-carries-the-cluster-maintenance-half` (P1).

Read all three in full.

**Track:** one defect in the coset fold, one missing row, and one
file split. No ratified sentence moves.

**Review:** one review, full, covering style and the claims below.

## What the tree says now

1. **A false cause at the parallel edge.** `coset::intersect` takes
   two planar cosets whose levered sine sits just past K·ε.
   `intersect_subgroups` rightly answers `Prismatic`. Then
   `candidate_translation` assembles
   `free1·P2·free1 + P1 + residual freedom`. For this pair the
   assembly is singular to rounding: `free1·P2·free1` is of the order
   of the squared sine. `inverse3` yields non-finite entries, and the
   membership check refuses the added mate as
   `Indeterminate { margin: Invalid, predicate:
   "mate_member_translation_in_plane" }`.
   - That predicate measured nothing.
   - The planes are decidably non-parallel: the table just said so.
   - `candidate_translation`'s doc says the system is "nonsingular by
     construction", and this measurement refutes it.
   - The row's reproduction is in the item:
     `msolve8_levered_clash.rs`, `c2_parallel_boundary_direct`'s
     search.

2. **`MateFault::Band` on a mate is reached by no row.** It is
   reached only by a loaded state. The insert door's `admit_mate`
   refuses `Band` under a bandless tolerance, so no insert or replay
   lands a mate there. A SNAPSHOT is a state, though, and loads under
   a bandless ε. Its mate then evaluates to `Band`, and the tree draws
   that mate row through `blamed_mates`' empty answer. Only hand-built
   values exercise that arm today.

3. **The maintenance shares `mate/solve.rs`.** The D-3 cluster-record
   maintenance is about 28% of the file, from the `// ---- D-3`
   region to the end: `ClusterMaintenance`, `Maintain`, `maintain`,
   `registry_after`, `unsolved_because`, `undecided` and `reconcile`.
   - The module doc gives it one bullet.
   - It shares only `SolvedPoses` and the cluster root lookup with
     the solve.

## What the unit builds

**1. The translation stage answers the question it was asked.**
- Measure the pair first: reproduce the row's refusal on `main`.
- Then make the final state one in which every refusal names the
  cause it measured. Two final states are admissible; the measurement
  chooses between them:
  - **(a) Solve in the line's own frame.** The planes are decidably
    non-parallel, so the intersection line has a direction:
    `d = n1 × n2`, normalized through the direction door. The
    candidate translation can be solved in the frame `(n1, d, n1 × d)`
    and its counterpart, where the system's conditioning is the sine
    itself rather than its square.
    - If the solved translation is finite and passes membership, the
      pair solves, and the pose is the true intersection. That may be
      far away: a near-parallel pair with an offset meets a long way
      off, and that is the true answer.
    - If it is not finite, the refusal is `RANGE_RECOURSE`'s class,
      named as such.
  - **(b) Decide the conditioning under a name of its own.** For
    example, a `mate_translation_system_singular` split on the
    determinant, levered honestly, which refuses with that cause.
- **Rule.** Prefer (a) if it removes the non-finite path for every
  pair the table calls non-parallel. The defect then starts nowhere.
  Fall back to (b) only for what (a) cannot reach, and say why.
- **Either way:**
  - `inverse3`'s doc and `candidate_translation`'s "nonsingular by
    construction" say what is true;
  - no `Indeterminate` names a predicate that measured nothing.
- **Sweep.** Every other stage that inverts or divides inside the
  fold: `inverse3`'s callers, the rotation stage, `clocking_about`'s
  `atan2`. Ask the same question of each: can a pair the table
  decided reach a non-finite there? Name the blind spot and a second
  pass (discipline §5).

**2. A mate's `Band`, reached by a state.**
- Write the row the item asks for:
  - a document holding a mate, saved under a sound ε;
  - loaded in a process whose witness admits no band, using the
    `tree_badges` band child's re-exec shape with a snapshot in hand;
  - asserting `Band` on the mate's own value, and the tree's no-blame
    rendering of the mate row.
- If the kernel half and the viewer half want separate rows, write
  both. Retire nothing: the hand-built values in `display_contract.rs`
  and the Python arm table stay as the arm's spelling.

**3. `mate/maintain.rs`.**
- Move the maintenance half into `mate/maintain.rs`, with a module
  doc of its own saying three things:
  - what it re-keys;
  - when the edit door runs it;
  - why a solve with no verdict refuses the edit.
- `solve.rs` keeps the solve and its module doc, minus the bullet.
- The visibility of what moves stays as narrow as it is.
- No behaviour moves: the move is a cut and paste plus `use`s, in its
  own commit, so a reviewer can read it as a move (`git diff -M`).

## Acceptance

- **A1.** The row's pair either solves, with a finite pose that
  passes membership, or refuses under a cause that is its own. The row
  is pinned in `msolve8_levered_clash.rs` or a new
  `msolve12_*` suite, and the PR says which final state was chosen
  and why.
- **A2.** No `Indeterminate` with `margin: Invalid` names a predicate
  whose margin the unit can show was not measured. The PR lists the
  §1 sweep's hits.
- **A3.** The `Band` row, or rows, reach the mate's `Band` through a
  loaded snapshot.
- **A4.** `mate/maintain.rs` exists, and the move commit is a pure
  move.
- **A5.** All three items close with `## Closed` sections citing the
  rows. `work.py lint` is clean.

## Constraints, binding

- **Discipline.** `docs/prompts/implementer-discipline.md` applies
  in full, by path. Hosted CI is the verification of record. Poll it
  in the foreground. Read `mergeable_state` before concluding
  anything from a run that has not appeared.
- **Git.** Merge-only. Push early. Open the PR through the GitHub MCP
  tools.
- **Build.** Use the private `CARGO_TARGET_DIR` named in the brief.
  Run `git status` before every `git add`; never `git add -A`.
- **Fence.** You may change:
  - `crates/editor-core/src/mate/coset.rs`, `mate/solve.rs`, the new
    `mate/maintain.rs` and `mate.rs`, for a new arm or its docs;
  - `crates/viewer/tests/` for the band row;
  - `pncad-py`, if a new `MateFault` arm crosses (tag, payload,
    `.pyi`, census);
  - tests and the items.

  `geom-core` is outside the fence. If the fix needs a door there,
  STOP and say what door.
- **Comments state the invariant** (discipline §4).
- **Stop clause.** Write what you measured into the PR as a draft and
  end your turn if either of these holds:
  - neither (a) nor (b) yields a refusal whose cause is its own;
  - a verdict moves anywhere in the mate suites beyond the row's pair
    and the rows its sweep names.

## Out of scope

- `mate-primitive-unit-variants-load-from-a-null-payload`, which
  waits on `[ev]` PR 3681: under it no unit variant remains.
- Plan items 19, 21 and 22, which are all with Ev.

## Review

One review, full. The claims to falsify:

- **C1.** No pair the table calls non-parallel reaches a non-finite
  translation unless it refuses under its own cause. Probe the
  boundary from both sides, at several arms.
- **C2.** The chosen final state's pose is the true intersection
  wherever it solves. Check it against an independent closed form.
- **C3.** The `Band` row reaches the arm through a state, not a
  hand-built value.
- **C4.** The maintenance move is a pure move.
- **C5.** No verdict moved, except the rows the PR names.
