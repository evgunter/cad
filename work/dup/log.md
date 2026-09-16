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

### The measurement unit B owed, and why B is now three units (2026-09-16)

**Answer: one fixture, not two.** `topo`'s Euler-built box and
`sweep`'s extrude-built box are the same solid — same counts, same
keys, same arena order in every topological arena, same face surfaces
and senses, same `mass_properties` bits. Two axes differ and neither is
geometry: the curve arena holds the same key set permuted, and four of
twelve edges carry `Intersection`'s `(s1, s2)` swapped. Nothing in the
tree reads either; `validate.rs` and `boolean/ops.rs` both accept
either arrangement. So the row's "equal" branch applies and the remedy
is the move down.

**The measurement was worth more than its answer**, which is the
argument for the row's own rule that a measurement comes before a fix.
Three things it found that no amount of reading the row would have:

- **A kernel finding.** `describe_as_intersections`' rule
  (`s1 = surface(face(he_plus))`) holds on 12/12 edges of the Euler
  body and **8/12** of the extruded one, with `he_plus`/`he_minus`
  identical between them — so extrude names the pair by something other
  than the edge's own direction on the cap it closes, and the pair has
  no ratified order to violate. Filed on BLEND's ground as
  `intersection-pair-order-is-unpinned-and-extrude-disagrees-with-itself`.
  It is an unordered pair carried in two ordered fields, with three
  consumers that each check both ways — Q7's shape exactly.
- **Two gates in front of the move, both MEASURED.**
  `scripts/gates/witness-not-ambient.sh` puts
  `test_support_impl.rs` in its production set (the `any(...)` mount is
  explicitly excluded from `GATE_CFG_TEST_NOT_RE`'s test-only
  narrowing), and the family calls `Tol::witness()` 24 times. The lane
  established this by **planting a violation and watching the gate
  fire**, then reverting — method item 4, on a gate rather than a
  constant. Plus the `#![allow(unwrap_used, expect_used, panic)]` that
  is unremarkable in `tests/` and is not in `src/`.
- **The honest set is the whole file**, including three members the row
  had not listed (`straddle_seat`, `flush_declarations`,
  `assert_every_chord_named_by_both_rules`).

**So B decomposes into three, and B is the last of them**: reconcile
`geometric_cube`/`cube_into`; thread `tol: Tol` through the family;
then move and unify. Each link is worth doing on its own merits whether
or not the next happens — which is the test that a decomposition is
real rather than a large unit made to look small. Threading the
tolerance also lands `prism_z`'s signature on
`sweep::test_support::brick`'s, convergent evidence that the two are
one door.

Ev ratified "A now, B next unit" before the measurement existed. Taken
as a sequencing decision with a recommendation rather than put back to
him (`memories/orchestration-model.md`), and reported in the same
sitting. The alternative was dispatching B as specced, which would have
moved a duplication into a new home and met the first gate at the
24th `Tol::witness()`.

**Next unit dispatched**: `topo-tests-geometric-cube-and-cube-into-are-one-sequence-twice`,
claimed onto this slate, full review tier.

### Link 1 ran (2026-09-16, PR #2727) — and X4 was done right this time

`geometric_cube` and `cube_into` now share one private `cube_ops`;
`geometric_cube` calls it without `describe_as_intersections` and
`cube_into` with it. **The lane's factoring beat the orchestrator's
suggestion and the argument is worth keeping**: a `describe: bool`
parameter would make the difference between the two doors *a literal at
a call site* rather than a line, and the whole risk in this unit was a
reader not noticing that step. A brief's suggested shape is a
hypothesis; the lane is closer to the code.

**The full-tier concern was discharged by measurement, not assertion.**
Thirteen bodies dumped as `format!("{body:#?}")` and compared by
SHA-256 before and after: all thirteen byte-identical.
`geometric_cube` still carries 12/12 `Scaffold(ExtrudedPoint …)`/
`Declared` and everything else 12/12 `Intersection`/`Derived`. That is
the axis the unit could have silently destroyed while staying green.

**X4, done the way the previous unit's failure taught.** Not "check the
neighbour I was thinking about" but *enumerate every sibling door and
group them by hash*. Result: **six doors, one body** — `brick`,
`prism`, `prism_z`, `mapped_cube`, `cube_into` into a fresh body, and
`review_m2_pr7`'s private copy. Checked away from the unit ranges
(`brick((1,3),(0,3),(-0.5,0.5))` against an affine `mapped_cube`) so
the agreement is a fact about the domain rather than an artifact of one
fixture. `review_m2_pr7`'s copy folded in and was deleted.

**A finding that changes link 3's value.** `crates/topo/src/cert_m3r1_probes.rs`
holds a verbatim in-src copy of `GeoCube`, `line`, `plane`,
`geometric_cube` and `describe_as_intersections`, and says so in its own
doc twice. It cannot be reached from `tests/`, and it **unblocks with
link 3 exactly** — so the move down is now worth more than one brick:
it closes an in-`src` copy that no test-side unit can touch. Recorded
against the brick-homes row's move set, which had not listed it.

**Adjudications on the lane's open questions**, recorded because two of
them are general:

- **Closing a row your own diff resolved is right**, even on another
  program's slate, when your program filed it there. An `open` finding
  that is fixed is a lie on the board, and the board is the only record
  of what is left. The row moves to the closing program's slate so the
  board also says who closed it.
- **A claim widened past its measurement owes a guard, not a
  narrowing.** The lane widened `brick`'s doc from "at the unit ranges"
  to "wherever their domains meet" and flagged it as stronger than what
  it measured. The remedy is not to weaken the sentence but to make it
  falsifiable: a row pinning the six-door identity at several boxes,
  **with `geometric_cube` as the negative row in the same test** — a
  test that only pins agreement goes green if someone makes all six
  doors identical by deleting the distinction, which is this unit's
  nearest failure mode. Sent back as a fix pass.
- **A row that asserts an equality it established by shape rather than
  by execution carries the defect this program exists to remove.** The
  lane's own new prism row does; it is now required to say so and to
  name the instrument it owes. The next lane must not inherit a
  confidence nobody earned.
