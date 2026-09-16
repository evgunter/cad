# S-DUP log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/dup/plan.md`.

## Opened (2026-09-15)

SUITE's successor, on Ev's approval on `[ev]` PR #2685 ("a successor
program sounds good"). SUITE closed eight rows and established that
**each was a sample rather than the population**; twelve rows naming
the classes behind them sit on S-TINT's slate and are this program's
opening slate.

The alternative was folding into S-TINT, and the reason not to is the
same one that opened SUITE: S-TINT's charter is that a row cannot go
red, and this one is that a thing is spelled *n* times. They coincide
often and are not the same question.

No branch exists yet. **First act:** re-take the census on
`topo-tests-brick-copies` — 24 `fn brick` in 22 suites is the largest
measured population, and its blocker (whether `topo` may dev-depend on
`sweep`, which depends on `topo`) is a crate-graph decision that should
be settled before anything is moved, not after.

## First sitting (2026-09-16)

**Review posture set by Ev in chat**: no A/B protocol; **style-only
reviews by default**, a full review reserved for units whose logic is
tricky to get right. That narrows `plan.md`'s "one style review per
unit, and a full review where a unit changes manifests, feature gates,
or what a suite measures" to the same rule stated from the other side,
and the two agree: a unit that changes what a suite MEASURES is a unit
whose logic is tricky.

**Branch prefix `dup/` confirmed with Ev** against the harness-assigned
`claude/...` branch this remote session opened with.

### `topo-tests-brick-copies` claimed, and its premise corrected

Census re-taken at `95bb4ba0`: **24 declarations across 23 files**, not
22 suites. More importantly the row's own alternatives rest on a false
premise — all 24 copies are already a one-line delegation to
`common::prism_z`, which already sits in a home every one of those
suites already imports. So the collapse needs no decision from anyone
and mints no new spelling; it is the row's option 3 with its objection
removed.

Option 1's blocker was also not what it said. `crates/topo/Cargo.toml`
already carries `mesh`, `stl` and `step-export` in `[dev-dependencies]`
with the comment *"dev-dependency cycles are cargo-legal and never
reach production dependents"* — so a `sweep` edge would be a fourth of
the same kind, and what was left of the objection was build cost, not
the crate graph.

**Put to Ev, who asked whether there was a structure that sidesteps it.
There is, and it is the tree's own rule applied downhill**: `topo`'s
`src/test_support_impl.rs` is a home every crate above `topo` already
reaches, so the shared brick moves DOWN rather than the dependency
moving UP. Split into two rows on Ev's ratification ("A now, B next
unit"):

- **A** — `topo-tests-brick-copies`, 24 → 1 at the home the builder
  already sits in. Dispatched.
- **B** — `brick-has-two-constructions-and-two-homes`, filed today. It
  owes a MEASUREMENT before a fix: are the Euler-built and
  extrude-built boxes equal bodies? Equal means one fixture moving
  down; not equal means two fixtures that must be named for their
  construction, and the duplication claim retires as false. The row
  says explicitly that the lane may not choose the shape first.

Method note for the program: **this is the third time in two programs
that a row's stated blocker was not its real one.** SUITE's lesson was
that a row's COUNT is a candidate list; this sitting adds that a row's
REASON is one too.

### Unit A landed green — and the census was defeated twice more

PR #2720, CI run 35047764757 `success` over the full code-tier matrix
(12 `test (…)`, 5 `k-lint (gate, …)`; the four skips are the
interval-transcendentals rows the change filter does not buy for a
tests-only diff). 34 files, +212/−262.

**The count, third and fourth revisions.** The row said 24 across 22
suites. The orchestrator's re-take said 24 across 23 files. The truth
is **23 across 23**: the 24th hit is `brick_with_torus_face_at`, a
different fixture that calls `brick`, and it survives
`git grep "fn brick"` but not `git grep -E '\bfn brick\b'`. The
dispatch carried the unbounded figure and the lane caught it.

**And in the other direction, the class was larger than every count of
it.** Beyond the 23 named copies the lane converted **11 renamed or
inline box spellings** — `distant_brick`, two byte-identical `bx`
twins, `box_at`, `corner_table`'s `top` and `leg`, two `bx` closures
and six inline pairs. **34 spellings removed.** Every one of them was
invisible to a name-shaped census and visible to a `prism_z` one:
method item 2 paying for itself on the program's first unit.

So both halves of method item 1 fired on one row: the count was wrong
**low** on the population and wrong **high** on the pattern, and
neither error was the one the row warned about.

**X4 answered rather than assumed.** `brick((0,1),(0,1),(0,1))` is NOT
a fifth copy of `common::geometric_cube()`, and the lane established it
by running both rather than by reading: same topology (`v8 e12 f6 s6`)
and the same operator sequence, but `prism_z` ends with
`describe_as_intersections` so a brick's transverse edges carry
`Intersection{..}/Derived`, while `geometric_cube` stops before that
step and keeps `Scaffold(ExtrudedPoint ..)/Declared` — which is exactly
what its own rows assert on. The answer is written into `brick`'s doc,
not left in a PR body.

**Operational note**: `work.py territory --base main` fails with "no
merge base" inside a fresh agent worktree; `--base origin/main` is what
to pass there.

### Unit A closed (2026-09-16, PR #2720) — and what the first unit taught

**66 spellings, not 24.** Final tally in the row. The census was wrong
high on the pattern and wrong low on the population by nearly 3x, and
three different instruments were needed to find the class: a
word-bounded name grep (23), a construction grep on `prism_z` (17), and
the reviewer's shape — *parse every rectangle-profile site, then check
whether the binding is ever read for anything but `.body`* (26). **No
one of the three would have found half of it.** That is the sharpest
form yet of method items 1–3, and it belongs in any brief this program
writes from here: name the instrument, not just the pattern and the
scope.

**X4 fired inside the paragraph naming it, exactly as item 5 predicts.**
The lane proved `brick ≠ geometric_cube` and stopped, in a file with
three cube doors. `mapped_cube(Point3::new)` and `brick` at the unit
ranges are **arena-identical** — proved by execution on the fix pass,
after the reviewer proved it by reading. The unit's own new door was a
fourth spelling of a builder already in its file, and its doc asserted
a partition that was false. **Only the reader who did not write the fix
caught it**, for the sixth time across SUITE and S-DUP. The rule holds
with no exceptions recorded against it.

Method item 5 should be read as stronger than it is written: it is not
that a lane *might* mint a fresh instance, it is that **the X4 check
itself is where the instance hides**, because a lane checks the
neighbour it was thinking about. The instruction to a lane is therefore
not "check for a fresh copy" but "**enumerate every sibling door in the
file and check each**".

**Review tier confirmed by outcome.** This was a style-only unit by the
`plan.md` tiering and the style lane returned thirteen findings, four
of them structural, none of them a correctness MAJOR. The tier was
right and the review was not a formality: Q1 (a differently-shaped
sweep) found the 26 leaked sites, and Q8 (read the whole file once)
found the `geometric_cube`/`cube_into` duplication that nothing else
would have.

### A cross-program hazard surfaced by this unit, on S-MESH's ground

`crates/mesh/src/nurbs_cert_fuzz.rs`'s
`r1_random_rational_soundness_sweep` **draws a fresh seed every run**,
so any lane's PR can draw a failing one. This unit's second CI run did,
at `0x5ca58da03160d407`, with `crates/mesh` byte-identical to main on
the branch. The lane reproduced it deterministically instead of
re-running it away, and found the assertion's `{:.3e}` formatting hides
the margin it fails on: sampled `wuu` exceeds the certificate's `muu`
by **two ULPs**. The existing row's recorded instance is a 1.7 %
overshoot — a genuinely wrong bound — so **fixing that bound will not
stop this row reddening other programs' PRs**. Evidence added to
`work/mesh/nurbs-face-bound-unsound-on-a-random-rational.md` rather
than a second file. Flagged to Ev; S-MESH's to fix.
