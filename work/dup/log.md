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

### Link 1's full review (2026-09-16) — mergeable, and the concern in the brief was real

**Verdict: mergeable, no MAJOR.** All six falsification claims hold.
Two were corrected in passing (the standing guard has five doors, not
six — the sixth was in the throwaway probe only; and "reds only the
new row" is true of the new file and false of the tree, where the two
mutations red 23 and 4 rows respectively, mostly pre-existing). The
reviewer re-ran both mutations rather than accepting the lane's word,
and verified `geometric_cube`'s unchanged body by executing at both
SHAs and hashing the dumps rather than by reading the PR.

**The dispatch was wrong about the instrument and the reviewer said
so.** The brief worried that asserting on `format!("{body:#?}")` would
red on changes that mean nothing and train people to re-baseline. It
will not: there is no stored baseline, the row compares five **live**
dumps to each other in one process, so a new `Body` field or a `Debug`
reformat moves all five sides identically. **A dispatch is a
hypothesis and a reviewer correcting it is the lane working**, which is
`reviewer-style-lane.md` §1's "the dispatch is a hypothesis" doing its
job in the direction nobody plans for.

**The Q1 concern the brief flagged was real, and the evidence is
stronger than the suspicion.** `cube_ops` and `prism_z` are the same
function — hand-traced at `n = 4`, operator for operator, down to the
`f_bottom.he_plus` special case at `i == n-1` — and **`prism_z`'s doc
has said so since `0765b4617`**. Link 1's justification for keeping
both was *"`prism_z` can neither write into an existing body nor take a
tilt map, which are the two things every `cube_into` call site uses"*,
and both halves fail: the first describes a signature link 1 had just
changed on the other function, and the second is factually wrong —
three of four direct `cube_into` call sites take axis-aligned affine
maps. **A justification written at the copy site by the author of the
copy**, which is the shape only an outside reader catches, one level up
from the instance the unit closed.

**Inserted as link 1b, before link 2**, so the tolerance is threaded
through one builder family rather than two. Not folded into link 1: the
unification wants its own review, and link 1's guard is what makes it
checkable — which is the argument for merging link 1 first rather than
sending it back.

**A row that fails the rule it was written to enforce.** The same lane
filed `topo-prism-z-is-hand-written-again-in-four-places`, naming four
re-writes of `prism_z`'s construction, all unmeasured, and offering
*"`prism_z`'s own `cube_ops` treatment"* as the model for the fix.
`cube_ops` is a **fifth member of that class** and the only one whose
equality is proved. The row treats the instance as the model, so its
census reads four when it is five and leaves out the one with a
receipt. Worth keeping as a general shape: **when a unit's own output
is an instance of a class the unit is filing, it is the member most
likely to be left out**, because the author is looking at it as a
solution.

**Prose the unit shipped that hides its own residue.** `common/mod.rs`
says "the Euler sequence **both** of this file's cube doors run" where
the diff's own guard proves five run it, and three hand-written counts
of one door set already disagree across two files in a single diff
("three cube doors", "five box doors", six actual builders). Sent back
with the rest of the prose corrections; the standing lesson is that a
hand-written census that has gone stale is the defect rather than a
description of it, so the instruction was to prefer not counting in
prose at all over counting correctly.

### Link 1 merged (2026-09-16, PR #2727); link 1b claimed and dispatched

Run 35063417572 green over the full code-tier matrix on the exact
branch head. Closing state-sync rode the unit's PR, per
`memories/orchestration-model.md`; `work/tint/topo-tests-review-m2-pr7-rederives-the-shared-cube.md`
moved onto this slate with it, because the unit that closed a row is
the program the board should name.

**Closing the row tripped the documented lint ERROR**, and that is the
check working rather than a nuisance: `brick-has-two-constructions-and-two-homes`
was `parked` on the cube-sequence row, so closing the trigger made the
park false and `work.py lint` said so. `work/README.md` anticipates
exactly this and says the answer is to fix the stale row, not to soften
the check. Re-parked on link 1b, which is the real gate now.

**A second-order version of the same rule bit on the orchestrator
branch**: the re-park could not be made there until the unit merged,
because the row it points at existed only on the unit's branch and
`lint` refuses a reference that does not resolve. Two branches, one
item — the cost `work/README.md` describes, paid in the small.

### Operational: this session was running blind on two channels

Ev, 2026-09-17: subscribe to your PRs, and set an hourly check-in on
the lanes (the remote analogue of `local-scripts/monitors/hourly-checkin.sh`,
which does not exist on a hosted box). Both were missing for this
program's whole first sitting, and the cost was real though small — the
session sat idle on a concluded CI run it never saw conclude. **A
remote orchestrator has no away-channel monitor and no
`hourly-checkin.sh`; the substitutes are `subscribe_pr_activity` per
open PR and a `create_trigger` Routine bound to this session.** Arm
both at the first dispatch, not after the first stall.

### Link 1b's full review (2026-09-17) — mergeable, and it measured what the brief only suspected

**Verdict: mergeable, no MAJOR**, all seven falsification claims confirmed —
and confirmed by re-measurement rather than by reading the PR: 105 661
lines of derived-`Debug` dump compared byte-for-byte at both SHAs,
mutation (c) re-run independently, the interval 5-rows-vs-3 split
reproduced locally. The claim CI could not make, the review made.

**The finding worth keeping: a shared home costs you an oracle, and
this unit paid it without noticing.** Link 1's guard compared doors
*against each other*, which silently catches a surface regression in
any one of them. Once the doors share a core, a change that moves every
door alike is invisible to that shape — the lane saw this and added an
independent row that re-derives the body from `profile`/`z`/`map`. The
review confirms the row really is independent, and then measured what
it does **not** cover: it never reads a surface. Changing `plane(&rev)`
to `plane(&bot)` reds `every_box_door_builds_one_body` at the merge
base and **nothing at head**. So the unification traded away an axis,
the compensation restored most of it, and only a reader who went
looking for the gap found the rest. **Generalisation for this program:
when a unit gives n spellings one home, ask what the n-way comparison
was silently buying, because the shared home cannot buy it back.**

**Three false statements this unit wrote**, all caught by the review and
none by CI: a fresh causal claim in `review_m3_pr3_consumer` naming the
describe step where the `&mut Body` seat is what matters; a dead first
arm promoted from `prism_z` into the one core behind eight doors; and
`cube_doors_agree.rs`'s header still asserting the two-route property
**twenty lines above a new paragraph that says the opposite**. A file
that contradicts itself within thirty lines is what accumulation looks
like when two units write the same header.

**Two durable records were wrong where the PR body was right.** The row
claimed mutation (c) "reds only that row" (it reds four); and it retired
`triangle_prism` on three reasons, one of which is not a difference —
`[c,b,a]` and `[a,c,b]` are cyclic rotations and `newell_plane` anchors
at the centroid over a cyclic cross-product sum, so both name the same
plane. **The PR body is not the record; the row is.** Worth stating as a
rule: when a lane writes the same fact into both, the row is the one to
check, because it is the one that survives.

**Two brief corrections, both mine.** `pub` on `prism_ops` serves two
sibling suites, not one, and every top-level item in `common/mod.rs` was
already `pub` — `cube_ops` was the file's only private item, so the
shape is the file's convention rather than an exception. And the
arena-order question I raised is settled by `DESIGN.md:202`'s ratified
*"deterministic minting order (documented per op — D9 lineage replay)"*:
the composite order is a **derived consequence of a stated contract**,
not an invented one, so pinning it is right and the maintenance cost on
links 2 and 3 is near zero.

**X4, fourth instance, and the first one no instrument could have
caught.** `REFLEX_L` is digit-for-digit `stl`'s `l_prism`, and the same
profile literal appears seven times across four crates — including a
byte-identical pair inside one test binary. Every instrument this
program has used keys on a builder (`mvfs(`, `find_half_edge(seed`, a
name); **none can see a duplicated profile *literal***. The class needs
an instrument that greps the constant, not the construction.

### Link 2 landed (2026-09-18, PR #2839) — and the row's own count was a file read twice

`tol: Tol` threaded through the prism fixture family, **727 call sites
across 75 files**, every one passing `Tol::witness()`.

**The gate premise was re-confirmed rather than inherited.** Planting a
`Tol::witness()` in `test_support_impl.rs` fired
`witness-not-ambient.sh` at the named line; reverting returned it green
over **436** source files, against 434 two days earlier — the
production set grew by two while nobody was looking, which is the
reason to re-check a premise rather than cite it.

**The count was wrong in a new way.** This row said "24 across the
family with 17 in this file", which reads as a total against a
subtotal. It was neither: both are the **same file at different
commits** — 24 before S-DUP, 17 after link 1, 10 after link 1b, which
is where link 2 found it. Link 1b had already done the larger half of
link 2's work before link 2 was written. A lane taking the row at its
word would have hunted seven witnesses outside the file and found none.
**Five units in, every count has been wrong; this is the first one
wrong about its own SHAPE rather than its magnitude.**

**Why this was safely style tier, stated better by the lane than by the
brief.** `Tol::witness()` returns `Self(())` — `Tol` is a ZST with
exactly one inhabitant, so `tol == Tol::witness()` at every threaded
site **by construction**, not by convention. The unit is provably
verdict-neutral rather than merely tested-green. Verified at
`crates/geom-core/src/tolerance.rs`. Three independent checks anyway:
that argument, the `cube_doors_agree` guard link 1b built for exactly
this, and a whitespace-normalised differ finding 71 of 75 changed files
byte-identical once witness calls and commas are stripped (the other
four being the threaded signatures and three rustfmt reflows).

**Two operational findings worth carrying forward.**

- **The compiler is an enumerator, and one configuration is not the
  set.** Default went green with 727 sites rewritten; `--features
  interval` then produced **42 more** and `probe` **7 more**. A lane
  that stopped at `cargo test -p topo` would have pushed a red branch.
- **rustc's missing-argument placeholder has two spellings** —
  `/* Tol */` where the type is imported, `/* geom_core::Tol */` where
  it is not. A rewrite keyed on the first leaves literal placeholders
  that are a **parse error**, so the compiler stops before reporting
  the remaining sites and repeated passes converge on a fixed point
  that is not green. Caught by reading error TEXT, not error counts.

**Two corrections to things this orchestrator wrote.** The
`cert_m3r1_probes` copy does not unblock with link 2 on a gate
argument: `lib.rs:169` mounts it `#[cfg(test)] mod`, so the gate never
reads it and its twelve witnesses are legal and stay legal — what
forces that copy is **namability**, not the gate. And link 3's
destination is not settled: `crates/topo/src/fixtures.rs` already
exists, already holds fixture vocabulary, already carries the
`#![allow]` with its argument, and already has the post-link-2
signature; it is `pub(crate)`, which is precisely why
`tests/common/mod.rs` exists separately. Recorded on link 3's row as
one of three options, with the measurement that decides between them.

**Link 3 is now open** — its last blocker closed, so the row is
dispatchable rather than parked. That is the third face of
`work/README.md`'s fired-trigger rule this program has hit: re-park,
re-park, and now simply open.

## 2026-09-18 — the link-3 home measurement, dispatched twice

The first home-measurement lane was **killed by an account session
rate limit** partway through, before it ran a single probe. Its last
words were *"Now the two probes."* and the only thing in its worktree
was a `work/dup/` row file holding frontmatter and no body — a title
asserting "at least eight times inside topo, and two of the copies are
byte-identical" with **no citations and no measurement behind it**.
That stub was discarded rather than carried: a claim with no evidence
under it is worth nothing regardless of how plausible its title reads,
and this program exists to catch exactly that kind of inherited
number. The lane was re-dispatched from scratch on the same brief.

**The operational rule this confirms**, which is the second outage of
the sitting (the first killed link 1b's fix pass mid-verification):
treat every pre-outage local result as stale. The difference between
the two cases is what the worktree held. Link 1b's held real,
uncommitted work including a planted mutation, so it was *resumed*;
this one held a stub, so it was *restarted*. The discriminator is
whether there is evidence on disk, not whether the lane sounded
confident when it died.

## 2026-09-18 — link 3's home, and an error of the orchestrator's own

The re-dispatched measurement came back and **settled the home**:
option 3, a sibling module re-exported through `test_support`. Options
1 and 3 are invisible to consumers and differ only in whether the
three-lint `#![allow]` also covers `ArenaCounts`, which earns none of
the 31 lints it allows (25 `unwrap_used`, 6 `expect_used`, 0 `panic` —
that arm is unearned and should not travel).

**Option 2 died on the number the row itself named.** The row said the
deciding figure was how much of `crates/topo/src/fixtures.rs`
duplicates `tests/common/mod.rs`. It is **zero** — the two files share
no item. What they share is the name `prism`/`Prism`, with disjoint
meanings: `fixtures::prism` has no mass properties at all, its eight
points are collinear in `y = 0`, its surfaces are `NaN`-control-point
`Nurbs` and its carriers are `Circle`/`Scaffold`/`Declared`. A
skeleton, not a box. The file says so at `:492` — *"indexed
placeholders … structural validation never reads them."*

**I wrote option 2 into that row, and the reasoning was bad in a way
this program exists to name.** I had two observations — the file holds
fixture vocabulary, and it exports a `prism(n, tol)` matching the
signature link 2 had just converged the `tests/` family onto — and I
treated them as evidence of sameness. They were three readings of one
surface: a name, a signature, a neighbourhood. The convergence I read
as "these are one door" was a convergence onto `(count, tol)`, which
after link 2 is what nearly every fixture builder in this tree takes.

S-DUP normally catches *different names for one thing*. Here the
orchestrator nearly landed a unit on *one name for two things*, and
what caught it was not judgement but the row's standing demand that
the measurement precede the choice. The companion to the
five-instruments result, recorded on the row: **a name, a signature
and a neighbourhood are three readings of the same surface, and three
surface readings do not make a measurement.**

**One correction back to the lane.** It characterised `fixtures.rs` as
"the raw-insertion home" against `tests/common`'s "Euler-op home".
True of `fixtures::prism`, false of the file: `ops_cube`,
`ops_holed_box`, `ops_genus2`, `ops_ring_bridge` and `ops_strut_cube`
are all operator-built with real coordinates. The verdict stands — the
item overlap is still zero — but the boundary between the two files is
**reachability**, not construction style. `fixtures.rs` is
`#[cfg(test)] pub(crate)`, so `tests/` cannot name it; `tests/common`
is a separate binary, so `src/` cannot name it. Each exists because
the other is unreachable. That is the wall link 3 takes down, and the
same wall holds `cert_m3r1_probes.rs`'s copy in place.

**A sixth spelling, filed rather than disclosed.** The lane measured,
while settling `review_m1_pr3::build_box`, that `fixtures::ops_cube`
is `geometric_cube` with the face geometry declined — byte-identical
dumps in `points`, all 1199 lines of `curves`, `half_edges`, `loops`,
`edges`, `vertices`, all seven provenance maps, `curve_origins` and
`surgery` — and that `build_box` is `ops_cube` at a uniform 2× scale.
It correctly left that in prose and flagged that prose is not
scheduling (`work/README.md`). It is now
`work/dup/the-cube-sequence-is-written-five-times-and-twice-inside-src.md`,
parked behind link 3, because nothing in `src/` can name the shared
builder until link 3 lands.

Three things the lane retired for link 3, each measured rather than
argued: the blocked-from-`src/` set is **empty** (`cargo check -p topo
--lib --features test-support` with the family mounted: 0 errors, 0
warnings, and `--lib` excludes dev-deps); `tests/fixture/mod.rs` moves
**nowhere** (one SSI acceptance fixture, 2 consumers, zero overlap,
slated for deletion by its own header); and the witness gate is
**already discharged** — link 2 took `tests/common` to 0
`Tol::witness()` calls, so only the `#![allow]` remained of this row's
"two gates at the door".

## 2026-09-18 — the program's own subject, found in the program's own tracker

Noticed while checking whether link 3 was clear to dispatch:
`work/dup/topo-tests-brick-copies.md` still carried **24** in its
title, and twice as a live fact inside the very section that corrects
24 to 23. The row is closed; its closing table is right (23 named, 17
renamed/inline, 26 let-bound, **66** total); the wrong number was
sitting in the one field `work/STATUS.md` renders.

That is this program's subject, in this program's own file, written by
this orchestrator. A number gets corrected where the correction is
argued and not where it is *used*, because the two are different
sentences and only the first is what the author is thinking about.

Fixed: title now states the 66/23 split; the two live uses in the
correction section now say 23. **Deliberately not fixed**: the 24s in
the original finding and the original option list. Those are the claim
that was corrected, and the correction quotes them — rewriting them
would erase what the row is evidence of. The intermediate **34**
(23 named + 11 renamed, the figure before the reviewer's sweep found
26 more) is likewise left standing, with a supersession note pointing
at the closing table, because the progression 23 → 34 → 66 is the
row's evidence for method item 1.

The rule this hardens, for every row this program closes: **correct the
number everywhere it is asserted, and leave it everywhere it is
quoted.** A closed row's title is an assertion.

## 2026-09-19 — what the full review found, and the four shapes of a stale number

Link 3's review came back with no correctness defect and no false
guard entry — the implementer's three test counts reproduced to the
test, every gate green, and the two guard-table entries it flagged for
scrutiny turned out **true and load-bearing** (the reviewer planted a
deletion and watched
`every_public_mutation_path_preserves_tier1` red naming exactly
`test_support_fixtures.rs::cube_into`). What it found instead is worth
more to this program than a bug would have been.

**1. A constant that records a measurement is a stale number that CI
cannot see.** `crates/topo/src/source_walk.rs:422` holds
`DOORS_MEASURED = 48`. The move added three `pub fn`s taking
`&mut Body`, so the walk now finds 52 — and the constant's own doc says
*"It is re-measured, never left behind."* The assertion is
`out.len() + 2 >= DOORS_MEASURED`, so the floor is the constant less
two: at 48 against a real 52 the walk could lose **six** doors before
reddening, and at 52 it can lose **two**. **Nothing reds.** The guard
does not break; it gets slacker. The PR body discussed the two tables it had to edit at
length and never mentioned the constant, because the tables refused to
compile and the constant did not. That is the whole mechanism: **what
a change is forced to notice is what fails loudly, and a measurement
recorded as a constant fails quietly by construction.**

**A correction this orchestrator owes on the same finding.** The first
version of the paragraph above had the slack **backwards** — it said
the walk could previously lose 3 doors and could now lose 6, which
reads the fix as loosening a guard when it tightens one. The
arithmetic is the assertion's: `out.len() + 2 >= DOORS_MEASURED` sets
the floor at the constant less two, so a *low* constant is the slack
one. The lane correcting the PR body caught it and gave the measured
numbers. Worth recording rather than quietly fixing, because it is
this program's own subject a third time in one sitting: I wrote a
paragraph about the cost of not re-taking a measurement, and got the
measurement's direction wrong without re-deriving it. The rule stands
against its author — **a number you did not derive is a number you are
quoting**, and I was quoting my own summary of a review.

Two smaller ones from the same pass, both mine and both the same
shape. I told the body lane that five family items are named from
`src/`; the real set is **three** (`geometric_cube`,
`describe_as_intersections`, `face_surface_of_he`) — `line` and
`plane` are internal to `geometric_cube`. And I carried a review
phrase, *"the precedent is three lines above the `topo` dependency"*,
into the sweep row without opening the manifest: the forward is
`crates/sweep/Cargo.toml:28` in the `[features]` table and the `topo`
dependency is `:63`. Corrected on the row in its own commit.

**2. "True and checkable" is half true.** The guard tables check rot in
the *name* direction — renamed, deleted, started asserting. The reason
string is never read against the body. The reviewer planted a raw
arena write in `prism_ops` — exactly the orphaned-key violation the
`ALLOWED` reason disclaims — and **all three guards stayed green**.
That is pre-existing and true of every entry, but the existing entries
are one-to-three-line delegations whose whole body fits beside the
entry, and `prism_ops` is a hundred-line generic builder that future
lanes will edit *as a fixture, not as a kernel door*, with nothing at
the function saying an exemption rides on it.

**3. Disclosing a blind spot is not compensating for it.** The row said
of its shape census: *"it undercounts every builder that loops."* Then
the count was declared closed. `crates/topo/src/splitting/reassembly.rs`'s
`quad_prism` is a **seventh** copy of the moved builder, in `src/`, whose
own doc calls it *"the tests/common builder's minimal in-crate copy"* —
naming a path this very diff deleted. It is loop-written, so the shape
census scored it 1/3/2; it has a new name, so the name census missed
it. A census shaped on **geometry rather than arity** (files holding
both `mvfs(` and `newell_plane`) puts it directly beside the family, in
about thirty seconds, as does `rg 'in-crate copy'` over the prose that
declares it. The five-instruments result now has its sharpest
corollary: **a disclosed blind spot is an instruction to run a third
instrument, not a licence to publish the count.**

**4. A name census cannot close a class that is not name-shaped.** The
unit folded four copies of `face_surface_of_he` and recorded the class
closed. The class is the half-edge → loop → face walk, and at least
twelve more spellings survive under other names — including
`topo/src/shell.rs` and `topo/src/replace_face.rs`, which are
**byte-identical closures under two names**, both in `topo/src`,
mutually reachable, neither disclosed. Relabelled a half-fix.

**And the trap does not care that the file names it.** Two of the
review's findings are fresh X4 instances minted by the diff, and both
landed in the files that state the rule against them: two new
guard-table comments restate one paragraph in two phrasings, in the two
files whose docs say *"This paragraph is the one statement of that
decision"* and *"The reason a posture is SAFE lives once."* Naming the
trap in the header does not stop the author walking into it four
hundred lines below.

**One correction the review made to this orchestrator's brief.** I sent
it to check three sweep-deviation reasons; one of the three is
overstated and one should not have been a reason. The gate the PR says
forbids a `sweep` feature forward **skips a forward from a test-only
feature by construction**, and `crates/sweep/Cargo.toml:28` already
does exactly that for `profile`, three lines above the dependency the
PR cites. And a byte-golden corpus that moves is never a cost to weigh
against a change that makes the code right — `implementer-discipline.md`
§3 says so, and I adjudicate against it. The deviation still stands,
carried by reason 1 alone: `sweep`'s `brick` takes `(T, T)` extents and
`Real` declares `from_f64` with no inverse.

So the running tally of stale-number shapes this program has now met:
a title (`topo-tests-brick-copies`, 24), a live use inside its own
correction (same row), an intermediate figure left standing as if
final (34), and now **a constant that records a measurement nobody
re-took**. The first three are prose. The fourth compiles.

## 2026-09-19 — link 3 merged; the brick unit is closed and the wall is down

PR #2842 merged green on the full code tier (12 `test`, 5 `k-lint`, 35
success, 4 skipped, zero failures). `crates/topo/tests/common/mod.rs`
is now `crates/topo/src/test_support_fixtures.rs`, re-exported through
`topo::test_support` — option 3, the sibling module, so `ArenaCounts`
keeps its allow-free file. `cert_m3r1_probes.rs`'s in-`src` copy is
folded and its row closed. **All three links of the brick unit are
done**, and the row that opened this thread —
`brick-has-two-constructions-and-two-homes` — is closed.

**One `mod` declaration moved and 279 references did not.** `mod
common;` became `use topo::test_support as common;` at `tests/all.rs`;
a crate-root `use` is private but visible to descendants, so all 74
suite files compiled unchanged. The row's *"read, not compiled"*
caveat is retired by the build.

**The collision was resolved by renaming the incumbent.**
`fixtures::prism`/`Prism` are `raw_prism`/`RawPrism` — 12 sites against
125 the other way — so each name has one definition in the crate. Four
proofs, none count-shaped, and the fourth is a test that asserts the
two families agree on *every arena length* before separating them on
surfaces, carriers and volume. The review then sharpened what it
guards: bodies converging, not names re-colliding. Names are covered
by the structural three.

**A false CI failure worth remembering.** A `check_run.completed` wake
said `gate ok` **failed** on the previous head. The run it belonged to
had concluded **cancelled** with `failed_jobs: 0` — my own next push
superseded it mid-flight and the aggregate reported failure because
its dependencies were cancelled under it. The check-run layer and the
run layer disagreed, and only the second is a fact about the code.
That is exactly why the check-in discipline says read the workflow
**runs** list rather than the PR's checks list.

**The slate after this unit.** Six rows closed, four open:
`the-cube-sequence-is-written-five-times-and-twice-inside-src`
(unparked by this merge — all five copies are now under
`crates/topo/src/` and the shared builder is nameable from every one
of them), `half-edge-to-face-walk-is-spelled-once-per-suite` (56
tracked files, filed by the review), `sweep-test-support-brick-is-\
still-a-second-box-construction`, and
`f6-display-predicate-is-spelled-three-times-with-no-home`.

**Next unit: the cube sequence**, taken as a sequencing decision with
a recommendation per `memories/orchestration-model.md`. It is the
direct payoff of link 3 rather than a new front — the wall link 3 took
down is precisely what blocked it, all five copies now sit in one
crate, and it closes the class this program opened on. The half-edge
walk follows, starting at its cheapest pair (`shell.rs`'s `face_of_he`
and `replace_face.rs`'s `face_of`, byte-identical closure bodies under
two names, both in `topo/src`). The `sweep` row waits: what holds it
is a reading of the adjudication's *"no manifest edge is added at any
step"* against a feature appended to an existing forward list, and
that is Ev's sentence to interpret, not mine to reinterpret in my own
favour.

## 2026-09-19 — the cube-sequence fold

`dup/fold-the-cube-sequence`. `prism_ops` gains the declined axis as a
`FaceGeometry` parameter; `splitting::reassembly::quad_prism`,
`fixtures::ops_cube`, `review_m1_pr3::build_box`, `mesh`'s MESH-6
scaffold probe, `review_m3_pr1::ops_cube_public` and
`interval_body`'s interval cube all fold onto it. Every body
byte-identical (`deep_snapshot`, all ten arenas + provenance).
`cube_independent.rs` untouched, as its header requires.

Two of the row's three unmeasured questions are now measured and
written into it: the placeholder surfaces ARE depended on as
placeholders (10 rows red when handed planes), `build_box`'s 2x scale
is read by NO assertion but is kept because the hole recipes planted on
it live outside a unit cube and nothing would go red. The X4 re-census
turned up the §9.3 holed-box class, filed as
`work/dup/the-9-3-holed-box-sequence-is-written-out-four-times.md`.

The `mesh` dev-edge question the brief flagged is NOT the `sweep` one:
`sweep` needs the feature at LIBRARY build time (its `test_support` is
a `src/` module), `mesh` needs it on a dev edge only, and
`crates/step-export/Cargo.toml` and `crates/step-import/Cargo.toml`
already carry exactly the shape `mesh` takes — a featureless `topo`
in `[dependencies]` beside a `topo = { features = ["test-support"] }`
in `[dev-dependencies]`. `scripts/gates/test-features-dev-only.sh`
passes.

## 2026-09-19 — three times in one sitting, I treated unratified text as binding

Ev stopped me on a sentence I had attributed to him. `git log -S` puts
*"No manifest edge is added at any step"* in `1f3fbc3c8`, 2026-09-16,
**written by an agent in this session** — my own adjudication of the
brick row. CLAUDE.md has a rule for exactly this and I skipped it:
*"Check that Ev ever agreed, before you wait for Ev."*

And I had truncated it. In full: *"No manifest edge is added at any
step — **every consumer already depends on `topo`**."* That is a
**justification**, not a prohibition: it argues the plan is cheap
because the edges already exist. I quoted the first clause, read it as
a rule about what is permitted, and then declined to interpret it "in
my own favour" — deferring to Ev over a cost argument an agent wrote
three days earlier. The claim is still true on its own terms:
`crates/sweep/Cargo.toml:63` already carries `topo`, so a feature
appended to an existing forward list adds no edge. **The sentence
never conflicted with the fold; it described it.**

The row itself was honest (*"whether that sentence reaches it is a
reading of the brief and a small one"*) and so was PR #2842's body
(*"the brief's own sentence"*). The escalation happened only in what I
said to Ev — the one channel with no reviewer.

**Then the same error twice more, in the opposite direction.** The
cube-fold lane asked for a second reader on `review_m1_pr3.rs`'s header
— *"do not 'simplify' them to match the implementation's comments"* —
and I went looking for its provenance instead of taking it. It cites
`memories/review-and-dependency-policy.md`, which is Ev's-call text, and
that memory says:

> **Reviewer tests are ordinary tests (Ev, 2026-09-04).** … An earlier
> version of this memory made reviewer suites a protected class …
> **"never simplify to match shipped fixtures"**; **that reading was
> withdrawn**.

So the phrase is retracted, the lane's fold is what the surviving clause
*directs* rather than an exception to it — and **seventeen files under
`crates/` still state the withdrawn rule**, filed as
`work/dup/the-withdrawn-never-simplify-rule-still-stands-in-seventeen-files.md`.

The third instance is mine again: I have twice written that
`cube_independent.rs` is exempt *"per Ev's request (PR #17 thread)"*,
sourced from that file's own header, never checked. It may well stay
unfolded — but on the surviving clause (*its row's claim needs its own
derivation*, which for an independence cross-check holds), not on a
protected-class rule Ev withdrew.

**The shape, named once for all three.** The day's earlier findings were
stale *numbers*: a title, a live use, an intermediate figure, a
constant. These are stale **modality** — text whose force changed while
its words did not. A justification read as a constraint; a rule read as
still standing after its retraction. A number goes wrong when the world
moves under it. A modality goes wrong when nobody re-asks *who said
this, and does it still bind* — and the answer is one `git log -S` away
every time.

The memory that carries the withdrawal also carries the remedy, two
paragraphs up: *"When you retract one, grep for the claim, not the
sentence: a correction made where you first wrote it leaves every other
copy standing."* The retraction was made where it was first written.
Seventeen copies stood.

## 2026-09-19 — the fold's fix pass

Same branch. The fold's own disclosed residue — `fixtures::ops_cube`
and `OpsCube` reduced to names for `declined_cube::<f64>` and
`CubeOps` — is deleted rather than carried: 74 call sites across 20
files re-pointed, both destructuring sites with them. The argument is
the two module headers, which after the fold said incompatible things
about one function.

A fifth instrument, structural rather than arity/name/import/prose —
`git grep -n 'find_half_edge(seed.face'`, 26 hits — found two more
members (`tests/m3_pr1_surgery.rs`'s outer cube, `boolean/ops.rs`'s
`far_cube`), both folded and both proved body-identical by
`deep_snapshot` BEFORE the edit. It also turned up two classes that are
not cubes, filed as their own rows: the quad-sheet helper (3 copies)
and the two-rim cylindrical patch (**9 copies, seven files, eight of
them closures**). That count is the largest this program has opened a
row with, and every one of the nine is invisible to all four of the
censuses links 1–3 ran.

**The correction that matters for method.** The row said
`review_m3_pr1.rs` was missed because it was loop-written and under the
arity threshold. Re-run at the merge base it scores 20 `mev` / 14 `mef`
— three times the threshold. It was matched and then lost inside an
unnamed four-file bucket, and a second file from that same bucket
(`m3_pr1_surgery.rs`) was still in it. **A bucket disposition is where
a census loses things**; one line per hit is what this program already
asks for, and this is the receipt for why.

## 2026-09-19 — the cube fold merged; what five instruments cost and what they bought

PR #2843 merged green on the full matrix (12 `test`, 5 `k-lint`, 36
success, 3 skipped). The §9.4.2 class is closed: eight spellings folded
onto `prism_ops`, every body proved **byte-identical before and after**
by `deep_snapshot` over all ten arenas — and the two late members proved
identical **before** the edit rather than after, by a scratch probe
building each hand-written body beside its replacement.

**The unit's own count went 5 → 8 while it was being worked.** Three
members were not on the row: `review_m3_pr1.rs::ops_cube_public`,
`interval_body.rs`'s interval cube, and — after the style review —
`m3_pr1_surgery.rs`'s outer cube and `boolean/ops.rs::far_cube`. The
row's census had been re-run at the head each time. **A class does not
hold still while you close it**, and the count on a duplication row is
a lower bound with a date on it, never a total.

**Three refusals from lanes, all of them better than what this
orchestrator asked for.** I told the fix pass to wire
`cube_doors_agree.rs` to the newly exported `UNIT_SQUARE`; it refused,
because that file's own doc says a guard reaching for the builder's
constant compares it against itself, and wiring it would delete the
row's independence. It un-exported the constant instead — the third
option neither I nor the reviewer had offered. I told it to delete
three `expect`s as documentation; it showed that `<[T; N]>::try_from`
returns a `Result` so something must consume it, and that the very
convention I cited **keeps** its own `expect`. And it declined to fold
the nine-spelling cylindrical-patch class it found, on the ground that
the class straddles `src/` and `tests/` on another program's territory
and its bodies were never dumped.

**The instrument story, which is this program's real output.** Four
censuses (arity, geometry, name, prose) closed the unit; a **fifth,
structural** one — `git grep 'find_half_edge(seed.face'`, the sequence's
distinctive bottom-close step — found two more members after the first
review passed. Its 26 hits then paid for themselves twice over: three
are a quad-sheet helper written three times across two files, and
**nine are one two-rim cylindrical-patch builder spelled nine times
across seven files**, seven of them with a token-identical closing
`mef` block. Eight of those nine are **closures**, so no name census can
see them; they loop, so no arity census can; their surface is a
cylinder, so the geometry census cannot. Both filed.

So the five-instruments result now has its own measurement attached:
this unit ran five, and the **fifth found members the other four could
not**, in a class four instruments had already declared closed. The
corollary stands and hardens — *no single instrument has ever found even
half of any class* — with the practical form: **when a census closes a
class, the next instrument is not optional work, it is the check.**

**And the bucket lesson, which is new.** The row's census had matched
`review_m3_pr1.rs` at 20 `mev` / 14 `mef` — three times its threshold —
and then lost it inside an unnamed line reading *"four `topo/tests/`
suites already dispositioned"*. Two of those four were mis-dispositioned;
the first pass found one and re-buried the other. All four are now named
with their reasons so the bucket cannot swallow a third.
**A bucket disposition is where a census loses things** — not the
threshold, which is where everyone looks.

## 2026-09-19 — the walk unit, and a baseline this orchestrator propagated without measuring

PR #2857 (`Body::face_of_half_edge`, `topo/src` folded onto it) came
back green on the full matrix. The lane **refused two parts of the
row's plan on measurement**, and both refusals were right:

- **"`Result` spellings as thin wrappers over the `Option` door" holds
  only where the error variant is entity-agnostic.**
  `splitting/join.rs`'s `he_face` raises `corrupt_he(he)` at hop 1 and
  `corrupt_loop(l)` at hop 2; an `Option` door refuses with `None` and
  can name neither. **The lane folded it anyway, as a planted mutation,
  and all 727 `topo` lib tests stayed green** — the "changes a verdict
  while everything stays green" defect, demonstrated live rather than
  argued. Three guards now red on it. `editor-core`'s `emit.rs` has the
  same shape and no census in the row had reached it.
- **The `.surface` hop does not belong on the door**: 10 of 73 sites
  carry it in the same statement, and `topo/src` carries it at **1 of
  17** — that one wanting the `Face`, not the surface key.

**A fourth instrument shape, and it is the compiler.** The lane put
`#[deprecated]` on `HalfEdge::parent_loop` and `Loop::face` and paired
the warning spans over `cargo check --workspace --all-targets`.
Deprecation *warns* rather than erroring, so the build does not stop at
`topo` and the whole dependent graph is read: **103 files / 145 sites**
against the row's regex at 56/73, including two buckets the row's table
had no row for. The regex is a floor and the probe a ceiling. **The
compiler is a census instrument, and it reads what no regex can** —
this program's fifth instrument shape and the first that is not a
pattern over text.

Its own method note is worth keeping: rustc attributes a chained field
read's span to the *start* of the expression, so `.face` can be
reported on an earlier line than `parent_loop`, and forward-only
pairing missed nine files the regex had. An instrument has a reading
convention, and getting that wrong undercounts exactly like a bad
regex.

### The baseline was mine, and it was wrong

I briefed the lane with baselines **3195 / 1356 / 1296**. It measured
at its own merge base instead of taking them, and reported them stale.
**It was right**: `cargo nextest run -p topo -p sweep -p stl
-p step-export -p mesh` on `origin/main` gives **3209 passed, 14
skipped**, measured here, twice. The branch gives 3212 — exactly the
lane's three guards.

Where 3195 came from is **unexplained, and I am not going to invent an
account of it**. `3195 + 14 = 3209` is suggestive and it is not
evidence. One hypothesis was testable and is **refuted**: reverting
`crates/mesh/Cargo.toml`'s `topo = { features = ["test-support"] }` dev
edge — added by PR #2843, and exactly the feature-unification hazard
`sweep`'s manifest comment warns about — leaves the count at 3209
either way, so that edge did not move the population.
`memories/review-and-dependency-policy.md` says it directly: *"Never
enshrine a causal story you have not checked."*

**What this orchestrator did wrong is simpler than the mystery.** The
3195 was a lane's self-reported figure. I verified its *delta* — one
`#[test]` added, none removed, by diff — and then carried the
*absolute* into the next brief as fact. **A delta can be right while
the baseline under it is wrong**, and checking the delta feels like
checking the number. Every number a lane reports is a claim; the ones
that get propagated into the next brief are the ones that need
measuring, and a diff check does not measure a total.

So the running tally of stale-number shapes gains a fifth: a title, a
live use inside its own correction, an intermediate figure left
standing, a constant recording a measurement — and now **a baseline
inherited from a report and re-issued as an instruction.** The first
four rotted in place. This one was propagated by the person whose job
is to catch that.

## 2026-09-19 — the same error a third time, and it was in my review brief

The walk unit's style review found no unsafe code — every fold is
behaviour-preserving and the three guards are real guards. What it
found is that **the claims are false**, and they sit in a row that
stays `open` and binds future work.

**"The 3-hop walk is spelled once in `topo/src`" is false.** I checked
two myself. `crates/topo/src/boolean/rest.rs:1498-1505` is literally
`body.get_loop(body.get_half_edge(mate)?…parent_loop)?.face` — the exact
form the PR's own structural instrument reports as having **one** hit in
`topo/src` afterwards. And `boolean/reduce.rs:547` is a plain `Option`
`face_of` closure with **no posture argument at all**, sitting in a file
the PR folded two other sites in. The reviewer counts at least ten
residual spellings.

**And the hazard population was measured at two.** The PR's central
argument — that folding an entity-naming refusal onto the `Option` door
changes a verdict with every test green — is now *established*, and it
applies to at least eight sites, seven of them inside `topo/src`,
including `shell.rs`, which the row itself named as half of its
"cheapest pair" and which this PR edited. **Exactly one is guarded.**

**The error is mine, and it is the third of its shape today.** My
review brief said the class had three members and asked whether there
was "a fourth flattening". I took that population from the
implementer's report and built the review's question on it — the same
move as the baseline I propagated unmeasured two units ago, and the
same move as the `17` I wrote into a row after a single-line grep.
Three times in one sitting: **a number arrived in a report, I used it
to frame the next step, and I never re-derived it.** The guard budget
was sized to a figure nobody had measured.

**A ratified rule I let a lane talk me out of.** The lane filed no new
rows, reasoning that separate rows would mint the duplicate this
program exists to prevent. It is a sympathetic argument and
`work/README.md` settles it the other way (Ev, 2026-09-06, quoted in
CLAUDE.md): *"That sweep sees items, not sentences … Disclosing a
residue is therefore not scheduling it — give it its own file at the
moment you disclose it."* The argument against duplicate rows is an
argument for **one row per seam owner**, not for zero. I read that
reasoning in the hand-back and did not check it against the rule,
which is the orchestrator's one job at that moment.

**Two more instruments, and the better one turns the change on
itself.** The reviewer re-censused the door this PR cites as its
*precedent* — `Body::solid_of_face` — and found four hand-written
face → shell → solid walks outside it, two of them byte-identical
`faces_of` helpers. The instrument is: **take the door a change cites
as precedent and re-census that door's own walk.** The second is a
closure-name census over `let face_of = |…`, which is cheap, over-fires,
and reaches `demos/` and feature-gated files no compiler probe can —
because `cargo check --workspace` does not compile four cargo roots and
feature-gated code never type-checks, so it never warns. `demos/tour`
spells this walk three times, two of them a byte-identical twin pair,
and **no census in this program has ever had a `demos` bucket**.

So the type-directed probe's "103/145 is the ceiling" is not a ceiling,
and the row's "20 of the 145 sites" divides folded 3-hop reads by a
denominator that also counts field *writes* and 2-hop reads. Against
the comparable instrument it is 20 of 32 — a different sentence
entirely. **Two numbers of different kinds, divided.**

## 2026-09-19 — a posture asserted in a brief, and the fence a door inherits

The `solid_of_face` lane came back with the class at **fourteen, not
eleven** — and with **two of the eleven not members at all**. The row's
structural regex had matched a *handle field's name* (`t.shell`) rather
than a `Face::shell` read, so `sweep/tests/revolve_ring.rs` and
`verbs_tubewall.rs` were never in the class. Two more of the same shape
stand in `demos/tour`, recorded so the next lane does not re-find them
as members. An instrument that over-fires costs exactly as much as one
that under-fires; this program has mostly met the second.

**My brief asserted a posture, and it was wrong.** I told the lane that
`seqgen.rs`'s four sites *"carry `expect(...)` with a distinct message
per hop"*. They do not — each has exactly one lookup and one `expect`,
because the face datum arrives from the `body.faces()` iterator. They
fold cleanly and did. I took that from the row's shape table and
restated it as fact about the code.

That is the sixth propagated-number error of the sitting and the first
that was not a number: **a posture is a claim about code, and it rots
the same way a count does.** The rule generalises — *a fact you did not
derive is a fact you are quoting* — and quoting a shape table is
quoting.

**The hazard was real, just somewhere else.** The one site that must not
fold is `offset_together::scope_of_moves`, the class's **only production
site**: hop 1 refuses `StaleFace { face }` — the caller's own key named
back to it — and hop 2 refuses `Corrupt`, nullary, because no key the
caller holds is wrong. The lane planted the fold and **nothing red
across 4405 tests** in three crates. So the guard is the deliverable
again, and it reds on both flattenings, each on its own arm.

### The finding worth keeping: a door inherits its minting pass's fence

`solid_of_face` cites no model, so the sibling-door re-census that found
this class cannot be run on it. Its own provenance answers instead:
`docs/MODEL-AB-LOG.md`'s BOOL4 row says the door was minted in **PR
#2767's fix pass**, out of a bilateral review finding —
*"face→shell→solid spelled four times"* — and the fold was scoped to
that PR's own four sites. The doc sentence that reads as a survey
(*"the one spelling … the census, the point-in-solid door and their
suites read"*) was never a survey. It was a report of one PR's reach.

**A door minted by a fix pass inherits that pass's fence, and its doc
sentence inherits it silently.** The tree had fourteen. That is a
general instrument for the next door: when a claim of the form "the one
spelling of X" turns up, find the commit that minted the door and ask
what that commit's scope was — the claim is true inside the fence and
says nothing outside it, and nothing in its wording marks where the
fence is.

Which is the same defect as the withdrawn no-simplify rule, the "no
manifest edge" justification and the `DOORS_MEASURED` constant, in a
fourth costume: **text whose scope was true when written, read later as
though it had none.**
