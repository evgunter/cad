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

## 2026-09-19 — the solid_of_face fold merged, and the instrument that ends the argument

PR #2865 merged green on the full matrix. Eight walks folded, **four
kept with their reasons stated**, one guarded. The class went **11 →
14 → 17** across three independent re-derivations in one unit.

**The review found the PR committing its own headline finding.** The
PR documented that *"a door minted by a fix pass inherits that pass's
fence, and its doc sentence inherits it silently"* — then wrote a fresh
unscoped sentence of its own (*"Every spelling in THIS CRATE that
refuses uniformly across the two hops reads through here"*) whose
counterexample, `seqgen::fusion_remake_shell`, sat **in a file the same
PR edited, between two sites it folded**.

**And the fix pass refused the review's remedy, with a better reason
than either the review or I had.** The review said the site "folds with
no behaviour change"; I caught that `shell2` is reused once more and
flagged the fork. The lane read it properly: `shell2` is read **twice**
more, so folding cannot delete the `let` and would *add* a second
resolution of a key the function already holds. The four `seqgen` folds
each **replaced** a lookup; this one would only add one. So the site
stays and the sentence narrows — and the carve-out is written as a
**shape** (*"a caller still using the intermediate shell key"*) rather
than as a site, which is what stops it going stale again.

**The door now claims no census at all.** Three populations, each with
members, and at the claim site: *"No census is claimed here … held true
by no mechanical guard."* That is a better answer than guarding the
sentence — a rustdoc on a three-line `pub fn` should carry an invariant
for a user, not a measurement for a future lane. The six lines counting
the project's own test suites moved to the row. Filed as a class:
**two doors in a row shipped an unguarded census sentence**, and
`face_of_half_edge`'s "sixteen sites" is still one.

### Denominator-first classification, and why it ends the argument

Five instruments had run on this class and the count kept moving. The
sixth settles it by inverting the question. Instead of searching for the
*walk*, enumerate the **terminal read** — every textual `.solid` in
every tracked `.rs` file, all roots, all cfgs, all features, 149 hits —
and classify each one **backwards** by where its receiver came from.

It cannot miss what a `Face::shell`-oriented instrument misses, because
**every member must terminate in a `Shell::solid` read**. Its closable
blind spots were closed by measurement rather than asserted: no
`.solid()` accessor exists, no `Shell { solid, .. }` destructuring
exists, eight `shell_of` helpers exist and none feeds a `.solid`. What
remains is a macro-assembled walk, which the method cannot falsify and
says so.

**The general form: when a class keeps growing under every instrument
you point at it, stop searching for the pattern and enumerate the
narrowest thing every member must contain, then classify backwards.**
A search over a shape has a blind spot for every way the shape can be
written; an enumeration over a required atom has one only where the
atom itself can hide.

### The receipt rule earned its keep

`implementer-discipline.md` §5 — *"a pattern with no hits recorded is a
claim; a hit list is a receipt"* — caught the whole thing. The PR gave
instrument 2's pattern and its delta but not its hits. Reconstructed,
it returns 14 hits and **fires on the missed site**: the member was
inside the instrument's reach and simply was not dispositioned. That
also exposed a blind spot nobody had stated — a face datum arriving
from a `body.faces()` **iterator** rather than a lookup, which is **six
of the eight sites this PR folded**. The instrument could not see most
of what the unit closed.

## 2026-09-19 — the exemption this program twice called Ev's was a lane's own recommendation

PR #2866 merged green on the full matrix. Twenty-one carriers retired —
the instruction deleted, the provenance kept, and **nothing rewritten
into a new rule**. Four rows filed where the remedy would have been to
*write* a standing instruction rather than remove a withdrawn one.

**The attribution, checked at last, and it does not hold.** This
program has twice written that `review_m1_pr2/cube_independent.rs` is
exempt *"per Ev's request (PR #17 thread)"*. I verified the commit that
minted it, `e9eeace50`, 2026-07-16 — which wrote the seven headers
**and** the memory clause they copied, in one commit. The clause reads:

> **Reviewer suites get promoted into CI.** … promoted into the repo as
> `crates/topo/tests/review_m1_prN*.rs` **(Evan, PR #17 thread)**. The
> suites are independent derivations — that independence is their
> regression value, so do not "simplify" them to match shipped
> fixtures…

**The citation attaches to the promotion. The no-simplify sentence is
the next sentence and carries no citation at all.** The headers copied
the attribution onto the whole paragraph.

And the thread itself settles where the phrase came from. Ev's own
words are a question: *"do reveiwer artifacts feed acceptance tests? we
may want to keep them as an auxiliary source of tests even if we don't
run them in ci"*. **Eighty-nine seconds later**, a long status report
from the same account answers it with *"recommendation is to promote
reviewer suites into the repo as labeled integration tests that DO run
in CI"* and proposes the provenance header verbatim — *"independent
derivations — do not simplify to match shipped fixtures, the
independence is the value"*. Ev's next message is *"how's it going on
pr 3?"*.

So the rule was **a lane's own recommendation, cited back to the person
it was recommended to**, and then quoted as his ruling by three
subsequent units of this program, mine included. That is the exact
failure the same memory names two paragraphs above the withdrawal:
*"Never enshrine a causal story you have not checked."*

Nothing downstream changes: Ev withdrew the reading in 2026-09-04
regardless of where it came from, and `cube_independent.rs` keeps its
own code on the clause that survived — its claim **is** the
cross-check, evidenced by the file's own description of its different
addressing. What changes is that the exemption never rested on anything
Ev ratified. The lane correctly did **not** edit the attribution line:
the promotion half is true, and narrowing it is a provenance claim on
an authorship question one account cannot settle. Filed for Ev.

### The ratification procedure is weaker than CLAUDE.md implies

Two measured findings about the check itself, both worth more than this
unit:

- **This checkout has 149 shallow grafts over 18,910 commits.** So
  `git log -S` does not merely bottom out at one bot render commit — it
  returns a long list of grafted commits in which every file reads as
  newly added, and path-scoping does not fix it. What works is
  `git log --all --format=… -- <path>` read oldest-first, then reading
  the actual diff.
- **`git log -S` misses wrapped text exactly as a grep does.**
  `-S"nothing here is a protected class"` returns nothing, because the
  phrase spans two `//!` lines; `-S"protected class"` finds the commit.
  **The tool CLAUDE.md prescribes for checking a sentence's provenance
  is a line-shaped instrument with the same defect as the grep it is
  meant to check.**

### And the population was 21, not 19, in a shape nobody had assumed

The sentence census confirmed the row's 19 and its 16/3 split exactly.
Two more came from paraphrase needles — *"promoted as-is"*, *"keep
verbatim"* — which no sentence grep reaches. The twenty-first was found
only on the **post-edit re-sweep**: an inline `//` comment mid-file, not
a `//!` header. **The whole program, this brief included, had been
reading the class as a header class.** It is not.

The denominator-first instrument then answered the sharper question:
30 citations of the memory in 27 files, of which **8 cite it for
something it no longer says — and only 2 of those 8 are among the 19.**
Six are structurally invisible to any grep for the sentence, four of
them a family nobody had looked at, and two sit in `work/*/plan.md` —
one in a program's **exit criteria**. A source header states a rule
where a lane *may* read it; a plan states it where a lane *must*.

## 2026-09-19 — the sweep brick: every premise the row was parked on was false

PR #2877 merged green on the full matrix. `sweep::test_support::brick`,
`block` and `cube` are now `topo::test_support::brick` — **one
construction of the axis-aligned box in the tree**, which is what the
brick unit set out to do on 2026-09-16 and could not reach until the
home existed.

This row had sat on three stated blockers. **All three dissolved under
measurement, and none of them had ever been run.**

**§3 — "the delegation re-authors the committed `.step` corpus."** It
moves nothing: all 17 files byte-identical before and after. And the
probe was **live** — mutating the delegation by `+0.001` on the mapped
z moves 5 of the 17. A zero-diff from a probe that never executed is
the failure this guards against, and the lane ran the mutation to
prove its instrument worked before trusting its null result.

**§1 — "at least one call site passes a genuinely `T`-typed value."**
Type-directed census across three lanes: **not one does.** All four
candidates are an `f64` constant lifted at the call site — exactly what
the door now does internally — and all four got *shorter*. **Two were
invisible to the row because they sit behind `interval` and `probe`,
which `cargo check --workspace` compiles neither of** — the same blind
spot that hid a member from the half-edge unit's compiler probe.

**§1's second claim — "a change to what that paragraph says."** Also
false. The same module's `corners` already takes `f64` pairs at every
scalar, with the reason written out: *"a fixture's outline is a set of
chosen constants, and a chosen constant is an `f64` whatever the lane's
arithmetic is."* The `(T, T)` extents were the outlier in that module,
not the convention they were defended as.

### A working option, rejected for the right reason

The row's numerical objection to widening `topo`'s door —
`x.0 + u·(x.1 − x.0)` is not `x.1` in floating point — is **true of
that formulation and false of the approach**. `prism_ops` evaluates its
map only at the profile's own corners, so a **selector** map
(`if t == 0.0 { lo } else { hi }`) is exact with no arithmetic at all;
measured, it builds an arena-identical body and moves none of the 17
files. The lane then rejected it anyway, because it would widen
`topo`'s door away from the convention to serve **zero** call sites.

Worth keeping: **an objection to a formulation is not an objection to
the approach**, and the way to tell is to build the other formulation.
Rejecting a thing that demonstrably works, for a reason that is not
"it doesn't work", is the shape of a good design call.

### The pair order is now measured, not read

The 2026-09-16 measurement disclosed *"whether the (s1, s2) order
matters is read, not measured"* and it stayed that way for three days.
The lane made `describe_as_intersections` — the step every
`topo::test_support` box, prism and cube builder ends with — write the
pair reversed on **every edge of every fixture in the tree**, and re-ran
the workspace: **8231 passed, 0 failed, identical to the unmutated
run.** Not one row in the tree can see it.

That lands on S-CARVE's existing row as evidence for its fork (2) — a
type that cannot carry an order — over fork (1), a ratified order:
**a ratified convention with nothing enforcing it is the same
unenforced convention under a better name.**

### X4 fired on the diff, and the catch was the census

Deleting `rect` left `square` spelling out the corner map that
`corners` owns **four lines above it**. Method item 5 exactly — the
fold nearly minted a copy inside the paragraph naming the trap, and it
was the self-census that caught it rather than the writing.

Residue filed: seven private extruded-box builders the structural
needle found outside the door, on S-DUP's slate with the reason it is
one row rather than four.

## 2026-09-19 — F6's third spelling folded; the strengthening is a null result, the mutation is not

`f6-display-predicate-is-spelled-three-times-with-no-home` closed, the
program's oldest open row. `crates/viewer/tests/panel_edits.rs` now
calls `test_utils::f6::assert_f6` with an `f6_variants!` census over
all 23 `Refusal` arms instead of banning one identifier per arm.

**The strengthening reddened nothing** — 626 passed / 0 failed / 1
ignored on `viewer --test all`, identical at the merge base and after.
The measurement the unit owed was whether the whole-roster ban catches
a real leak in what `viewer` renders, and it does not: the six sampled
renderings are clean.

**Method item 11 is what made that an honest null rather than a
guess.** Two mutations:

- a sibling identifier planted in `Refusal::NoSuchParam`'s `Display`
  is **green under the old per-arm form and red under the new one**.
  The delta is real even though the tree does not currently exercise
  it.
- a twenty-fourth `Refusal` arm reds the `f6_variants!` block with
  `E0004`, so the ban list is rustc's, not a hand-kept mirror. That is
  the half `assert_f6`'s `dumps: &[&str]` parameter cannot give a
  caller on its own, and the reason a site with 23 identifiers is
  cheaper to keep right than one with 6.

### The census was stale in both directions, which is item 1 again

The row named three copies and two more "outside its scope" in `topo`
and `mesh`. **The topo and mesh copies had already been folded** by
S-TINT's PR #2694 four days before this brief was written; reading the
row as current would have sent a lane to convert two files that are
already converted.

Two members the row never named are live, and both are the shapes this
program keeps meeting:

- **a paraphrase no grep for the predicate reaches** —
  `crates/viewer/tests/error_display.rs` spells the brace clause
  `" { "`, so the `contains('{')` instrument that found every other
  member walks straight past it. 29 call sites, one file over from the
  one the row was about, in the same crate, with the same per-arm
  approximation. → S-TINT.
- **a spelling inline mid-file rather than in a header or a helper** —
  `crates/quantity/src/tests.rs` transcribes `assert_f6`'s whole body,
  **panic wording included**, inside a `src/` unit-test module. No
  sweep for `dumps`, `guts`, `assert_f6` or a `tests/` path reaches a
  `src/` file. → S-FIX.

Both shapes are already in method item 10's roster. Both still cost a
census that did not deliberately run an instrument against them. The
instrument that found `quantity` was **prose** — its doc comment
announces the rule it copies.

### On the size of the row

One call site and a measurement is thinner than a unit, and the unit
said so rather than dressing it up. What earned it its own PR is the
census: the row's stated population was wrong in both directions, and
the two rows filed out of it are each larger than the fold was.

**Three things the orchestrator would add**, rather than a second
entry about one PR — this log is the program's narrative and two
accounts of one unit is the defect the program exists to close.

- **A census can be stale in the CLOSED direction too.** This program
  has found counts too low seven times running. This is the first time
  a row's population was too HIGH, because other programs had already
  done part of the work. **A count is a claim about a date, in both
  directions**, and a row that has sat for thirteen days has had
  thirteen days for the tree to move under it either way.
- **The denominator argument was made with its limits named.** The
  brace is the terminal atom because `assert_f6` bans it
  unconditionally, so every full copy carries it; the alternative
  terminal — a variant identifier — is **unbounded and therefore not
  enumerable**, which is why it could not be the denominator. The lane
  then stated the one blind spot it could not close rather than
  publishing past it. That distinction, between a blind spot disclosed
  and a blind spot closed, is what method item 8 asks for and it is
  rarely this cleanly done.
- **X4 fired on the lane's own doc comment.** It had written *"a
  twenty-fourth arm stops this file compiling"* — a number with
  nothing holding it — two paragraphs below where the same PR deletes
  a stale *"`Refusal` has 18 arms"* (it has 23). It removed the number
  rather than writing `24` in its place, which is the correct repair:
  the `match` the macro writes is what holds the roster complete, and
  a doc comment carries an invariant, not a measurement (item 13).

## 2026-09-19 — two lanes dispatched: the box-builder residue and the cylindrical rim

Slate after F6 closed: 12 open rows, 11 closed. Two dispatched together
because their territories do not overlap.

- **`private-extruded-box-builders-outside-the-brick-door`** →
  `dup/private-box-builders`, worktree `/home/user/dup-boxbuilders`.
  The residue of PR #2877: the tree's box door no longer extrudes, so
  the private builders that still do are one layer out from the class
  this program just closed. The row publishes a **floor** and says so —
  its needle was one literal spelling of the extrusion call, and it
  names by hand four ways a member can decline that spelling. The brief
  therefore asks for a second instrument before any count is published
  (method item 8), and for a **per-site disposition rather than a
  blanket fold**: a suite whose subject IS the extrusion loses its
  subject by taking an Euler-built box, and the row is explicit that
  deciding which is the unit's question, not the row's.
- **`the-cylindrical-patch-rim-builder-is-written-nine-times`** →
  `dup/cyl-rim-builder`, worktree `/home/user/dup-cylrim`. Nine
  spellings, and nearly every one a **closure**, which is why the name,
  arity and geometry censuses all missed them — the second piece of
  evidence this program has that a structural needle catches what three
  name/arity censuses do not. The row names three things it never
  measured (body identity, the home, whether the surface must be a
  parameter), and those three ARE the unit.

**Neither brief carries a count or a posture as fact** (method item
15). Both cite the row's numbers as the candidate list method item 1
says they are, and tell the lane to re-take the census at its own merge
base. Six times this sitting a number arrived in a report, went into
the next brief as fact, and was wrong.

**Neither row was set to `dispatched` by the orchestrator.** One file,
one item: the lane owns its row file and sets `status`, `branch` and
`pr` in its own PR, the way F6 and the brick row did. An orchestrator
editing the same file on a parallel branch is the merge conflict
`work/README.md` describes, and this sitting has already paid once for
committing conflict markers into this log.

Review tier is deliberately **not** fixed in either brief. Ev's rule is
style-only by default with a full review reserved for units whose logic
is tricky, and `plan.md`'s restatement makes the test *can this unit
change a verdict?* Both units can, if the fold turns out to move what a
suite measures — and whether it does is exactly what the lanes were
sent to measure. The tier is decided on the hand-back, from the
measurement, rather than asserted now from the row.

### A third lane, and a row whose headline had gone stale underneath it

`the-withdrawn-never-simplify-rule-still-stands-in-seventeen-files` and
`viewer-review-suites-cite-the-withdrawn-independence-reading` are one
class and went out as one unit → `dup/policy-memory-citations`,
worktree `/home/user/dup-citation`.

Before briefing it I re-took both rows' counts at `5b4979ef2`, and the
first row's headline no longer describes the tree:

- **The withdrawn sentence survives in three source files, not
  nineteen.** `rg -U 'do not\s*(//!)?\s*"simplify"'` over every tracked
  file returns `topo/src/review_m1_pr3.rs`, `topo/src/review_m1_pr4.rs`
  and `topo/tests/review_m1_pr5.rs` — which is precisely the group the
  row itself identifies as stating a **surviving** rule scoped to the
  derivations. The sixteen group-A carriers were cleared by the row's
  own carrier PR. What is still open is not the sentence, it is the
  per-file judgement the row says it owes, and the paraphrase carriers
  no sentence-grep reaches.
- **The citation denominator is 35 tracked paths**, against the viewer
  row's four. It reaches `editor-core`, `step-export`, a `sweep`
  example, `docs/REVIEW-STYLE-DISPATCH.md` and two `memories/` files
  that neither row names.

This is method item 12 (**stale modality**) happening to S-DUP's own
slate, and it is the third time this sitting a count has been stale in
the **closed** direction — the tree moved under the row while the row
sat. It is also why the brief hands the lane both measurements *with
the instrument and its blind spot named, and an instruction to re-take
them anyway*: a sentence-shaped grep cannot see a paraphrase, and a
line-shaped grep cannot see a citation that wraps across `//!` lines,
which is the defect that undercounted this very row three separate
times.

Two guards in that brief the other two did not need:

- **`memories/` carriers are read-only for the lane.** Some of the 35
  sit in files whose contents are Ev's call. The lane quotes what it
  finds and hands it back; it does not edit it. A lane correcting a
  memory would be exactly the permission-laundering shape this program
  keeps writing about.
- **The seven PR #17 attribution sentences are fenced off**, including
  in headers the lane is otherwise editing. That row is already waiting
  on Ev, and settling it means writing a new provenance claim about
  Ev's own words — which is not a lane's call and not mine.

## 2026-09-19 — the cylindrical-patch rim builder (`dup/cyl-rim-builder`)

**The count moved in both directions, which is item 1 with a twist.**
The row opened at nine spellings across seven files. Re-taken at the
merge base with four instruments over every tracked file and no path
argument, the class — one construction, one home — is **five**, in
four files, and it includes one member the row never named: a
`#[cfg(feature = "interval")]` `wall` in `mate5_cyl_eps_rung.rs`,
which no census that reads a default build can reach because the
compiler never type-checks it.

The row's OTHER four turned out not to be members. Two are the `src`
`cyl_sheet` pair, which builds a **measurably different body** — 1
solid, 2 faces, 4 vertices against the `tests/` family's 3, 4 and 6 —
because `Body::add_surface` is `pub(crate)` and a `tests/` binary
cannot call it, so every `tests/` spelling mints a scaffold `mvfs` per
rim to give the rim plane a face. **The two constructions diverged
along the crate boundary, not along intent.** Two more are torus
patches in `boxes.rs`: the row guessed one "may be a genuine sibling",
and measured it is a different class outright — four curved sides,
closing on a curved meridian spec where every member of this class
closes on a chord line.

**Instrument 3 is the one that earned its keep**, and it was chosen
because item 8 says a disclosed blind spot is an instruction: the row
disclosed that its structural needle keys on `find_half_edge(seed.face`,
so a second needle went at the surface key instead —
`FaceSurface::Shared(cyl)` — and that is what surfaced the interval
member. Its own blind spot (a key bound to another name) is
demonstrated rather than asserted: it misses both `boxes.rs` torus
patches, which say `Shared(torus)`.

**The mutation that stayed green is the residue worth having.**
Deleting the source write from the shared builder — so no sheet's
cylinder key carries a `GeomSource` at all — left **all 617
integration rows and 744 lib rows green at both lanes**, although
three suites name the distinct-`GeomSource` fingerprint as their
subject in prose. Filed on S-TINT. The mutation that DID work (the
descending rim's axis left unnegated) reds 23 rows across all four
folded suites, the interval member included.

**A live probe the fold did not have to plant.** Moving a fixture
builder into `topo/src` makes it a `pub fn … &mut …` under that tree,
which is the population `review_m1_pr5_internal`'s two door tables
walk — so the fold redded on a table with no entry for the new door
until it declared how tier 1 survives it. Worth knowing before the
next `tests/` → `src` fold: the home is not free, and what it costs is
an entry, not a feature.

**X4, caught in the diff by re-reading it**: the door's own
`lift_point` / `lift_vec` are the 23rd and 24th componentwise
`T::from_f64` lifts in the tree with no shared home. Filed. So is the
second X4 the fold left standing: `try_wall_sheet`, token-identical in
two probe suites, whose reason to exist was the local builder and
which now wraps the shared one.

### The style-review fix pass, same day

Nine items, and two of them are this program's own method firing on
this unit.

**X4 a third time, and I did not self-report it.** The unit unified
`CylFrame` the TYPE and left its CONSTRUCTORS duplicated — and put the
shared home in the file it was editing, so the duplication had a home
and did not go there. Re-censused: twelve spellings in three families
(the review said seven; two families had a second parameterisation
inside one file, which is item 1 one level down). All twelve now sit on
`CylFrame::canonical` / `::tilted` / `::opposed`. **The reader who did
not write the fix is the only one who has ever caught an X4 here, three
units running.**

**"A second construction" was wrong, and my own row said so two
paragraphs above the phrase.** The `src` `cyl_sheet` pair is the same
construction as the folded door with one difference — two scaffold
`mvfs` calls that exist only because `add_surface` is `pub(crate)`. The
row measured that and then labelled it "a second construction", which
is what a future lane would have read. **A measurement and a summary of
it can disagree inside one file**, and the summary is the part that
travels.

**The scar is inert, and now that is measured rather than asserted.**
Switching the door's rim planes to `add_surface` — the `src` form —
reds only the unit's own arena row and leaves all 617 integration rows
green at both lanes. So the word "inert" in the door's rustdoc, which
the review correctly called an unmeasured claim, is now a row's
measurement and not the door's prose (method item 13).

**Two more silent mutations.** The reviewer changed
`GeomSource::minted(source, 0)` to `minted(source, 7)` and got 566 + 730
green: the minted INDEX of every sheet's source is asserted by nothing,
as its `node` already was. The unit's row now asserts the whole
`GeomSource`, and the S-TINT row was widened from one claim to three —
`node`, the index, and the pairwise distinctness a single-sheet row
cannot reach at all. **A row about one field of a struct is a half-fix
of a class.**

**A gate decided an open question.** The review asked whether `tol` —
passed as `Tol::witness()` at all 23 call sites and never anything else
— is a knob that is never varied. It is, and it cannot go:
`scripts/gates/witness-not-ambient.sh` forbids `Tol::witness()` under
`crates/*/src` and does not exempt the `#[cfg(any(...))]` mount, which
`work/dup/thread-the-tolerance-through-the-prism-fixture-family.md`
established by planting a violation. Planted one in the door; the gate
fired and named the line. So the readability fix is the other arm — the
suite that had no local adapter got one, and its eleven eight-line call
blocks are one-liners again with each row's two chart windows adjacent.

**And one guard that cannot see what it says.** The unit's own
allowlist entry in `review_m1_pr5_internal` was materially false and
both guards reading that table went green, because they check
membership and never an entry's reason. Filed on GUARD, with the two
entries anyone has checked named and no count of the rest published,
because nobody has measured one.

### Third pass: a blanket claim replaced by a wider blanket claim

The fix pass corrected `review_m1_pr5_internal`'s section comment
because it was false for one of the four entries under it. **The
replacement was false for all four**, including `prism_ops` — the entry
that was the correct precedent. It said each builder "writes only
through doors already on this list", and `mvfs`, `mev` and `mef` are not
on that list and *cannot* be: it is by construction the doors that do
NOT assert, and the guard's other-direction row reds an asserting door
that appears on it. The true statement is the union of the two halves,
and it is two words longer than the false one.

**The failure is not the wording.** A blanket claim was rewritten and
not re-checked against every member it now covered — which is a census
published without being re-taken, one level up from code. The rule that
catches it is already item 1; what is new is that it applies to a
SENTENCE's scope as much as to a count's. A per-entry rewrite is
checkable by a reader in one step and a blanket one is not, which is why
the entry-level fix in the same pass was right and the section-level one
was not.

Two more from the same pass, both minted by it:

- **A bound promoted out of its scope.** `CylFrame::tilted`'s doc
  carried `radius·(1 − cos θ)` as the distance between the two loci.
  True at `v = 0` only — the tilt displaces a point at height `v` by
  `v·sin θ` — and the consuming suites compute exactly that, one of
  them calling it "the tilt's first-order transfer error `r·θ`" where
  the promoted bound is second order. It was inherited from a deleted
  LOCAL helper where its scope was one fixture. **A sentence true of a
  fixture becomes a claim when the fixture becomes a door**, which is
  item 12 minted inside the pass that was fixing item 12. Dropped
  rather than qualified: a caller of a test fixture does not reason
  with a displacement bound.
- **The unfolded duplicate under the folded one.** Two token-identical
  fifteen-line closures in `mate5_cyl_eps_rung.rs`, differing only in a
  source id. The pass edited BOTH of them — rewriting the frame
  expression inside each — and folded neither. The judgement that the
  inline frame expression should stay was right and was recorded; the
  duplicate one level out was not seen, because the edit was scoped to
  the line being changed rather than the block containing it.

The fold keeps `src` per-call deliberately: nothing asserts that two
sheets carry distinct `GeomSource`s, so merging two ids would erase the
subject of the S-TINT row this unit filed before anyone measures it.

### Orchestrator addendum on PR #2887 — three things the lane's entry does not carry

Merged 2026-09-19 as `967741f59`, after **two** review rounds: a style
review of the unit, then a delta read of the fix pass by the same
reader. Everything below is about the second round, because the second
round is what this unit taught.

- **Every unit gets a reader. This evening establishes that every FIX
  PASS needs one too.** The fix pass for an X4 defect minted three
  fresh instances, two of them in the exact classes it was fixing: a
  blanket claim corrected *because it was false for one member* was
  replaced by one false for all four — including the entry that was the
  correct precedent the brief pointed at — and a bound whose scope was
  one fixture in a deleted local helper was promoted into a `src`
  door's rustdoc, where it does not hold. Method item 12 and item 13,
  minted inside the pass fixing item 12 and item 13. The third was a
  pair of token-identical closures in a file the pass edited **in both
  copies** without seeing the block containing them.
  The lane's own diagnosis is the one to keep: *a per-entry claim is
  checkable by a reader in one step where a section-level one is not*,
  which is why the entry-level fix in that same pass survived and the
  section-level one did not. **Item 1 applies to a sentence's scope as
  much as to a count.**
  The delta read cost little: the same reader resumed with its context
  and was told to read one diff, not the PR. That is the cheap half of
  the lesson and the reason this is a method note rather than a
  complaint.
- **One count passed through three hands and was low at every hand.**
  The row opened at nine and the class was five; the reviewer named
  seven frame constructors and there were twelve, because two families
  carried a second parameterisation *inside a single file*, which no
  cross-file read catches; the orchestrator relayed twenty-two call
  sites and there were twenty-three, the extra being the witness
  tolerance behind a local alias. Three readings, three undercounts,
  every one corrected by the same act — re-taking rather than
  inheriting. Item 15 is not a caution about lanes; it is a caution
  about every hand a number passes through, this one included.
- **The orchestrator flattened a modality in relay, which is item 12 in
  miniature.** The lane's row says the visibility scar is inert *to the
  `tests/` side*, and that the `src` side is what still blocks the
  fold. The orchestrator's summary said "inert". One-directional claims
  lose their direction when they are restated by someone who is not
  holding the measurement, and the restatement is what travels — the
  same mechanism this unit's own wording defect turned on, where a row
  measured one thing and summarised it as the opposite two paragraphs
  apart. The lane's wording stands over the orchestrator's.

**On the review tier, for the next dispatch.** Style tier was right and
held through both rounds: the fold was proved bit-identical, so it
could not move a verdict, and nothing either round found rose to a
correctness MAJOR. What earned its keep was naming, in the brief, the
two hunks where a green diff tells you least — the guard-table entry
the move forced, and the mutation that reddened nothing. **Both rounds'
findings concentrated there.** A style brief that names its two
suspicious hunks is not a full review and does most of what one would
have done here.

## 2026-09-20 — the sitting's close: three units merged, and what the fix passes taught

`967741f59` (#2887, cylindrical rim), `3d4ab7595` (#2886, citation
census), `cb38b6c82` (#2891, private box builders). Slate: **15 closed,
19 open** — four new rows filed out of the three units, which is this
program's normal yield and not a sign anything went wrong.

### The finding of the sitting: a fix pass needs a reader as much as a unit does

Every unit here got a style review, and **every fix pass written
against that review's findings minted fresh defects** — three, two and
two, plus one more on a fourth pass. The classes:

- **Code and prose duplication**, the ordinary X4: a unified type left
  twelve constructors duplicated; three private re-spellings of a const
  the shared home exports, in files the lane had just censused; a
  fourteen-line operand written twice inside the file being rewritten.
- **The defect being fixed, committed inside the fix.** Twice. A lane
  told to remove unguarded counts wrote **twelve** fresh ones, one of
  them the same sentence byte-identical in five files. A lane told to
  fix stale modality promoted a bound out of a deleted local helper's
  scope into a `src` door, where it does not hold.
- **A rewritten blanket claim, false for more members than the one it
  was corrected for.** A section comment false for one entry was
  replaced by one false for all four, including the entry the brief
  named as the correct precedent.
- **A claim about a measurement** — the fourth pass's, and the hardest
  of the four to see. The measurement was real, was run, came back
  green, and the sentence describing *what it discriminated* was false:
  a control offered as varying two axes varied neither.

The mechanism is not carelessness. A fix pass is written fast, against
a list, by someone who has just been told what the defects are — which
is exactly the state in which the next one is invisible. **Only a
reader who did not write the fix has ever caught one**, in this program
or in the two tracks before it, and that now holds for fix passes as
firmly as for units.

The cost is small: the same reviewer resumes with its context and is
told to read one diff. Three delta reads this sitting, ≤80 lines each,
and each found something the lane had not.

### Counts, one hand at a time

A number was low at every hand it passed through, repeatedly, and the
orchestrator was one of the hands:

- nine → five (row → lane); seven → twelve (reviewer → lane);
  twenty-two → twenty-three, ~fifteen → twenty-five, ~forty-eight →
  forty-seven (orchestrator → lane); seven → thirty-four → forty-four
  (row → lane → lane); eleven → twelve (reviewer → lane).
- **The instrument defect worth keeping**: a denominator of 18 where
  the suite has 9, from `grep -c '^fn r1_\|^#\[test\]'` — an
  alternation that double-counts a row whose test fn is both attributed
  and named at column 0. Exactly one file in that crate has the shape,
  so **three of the four figures taken the same way were right, and the
  wrong one looked sound**. An instrument that fails on one input in
  four does not look broken; it looks like it works.
- Method item 15 is therefore not about lanes. It is about every hand a
  number passes through, and re-taking is the only act that has ever
  fixed one.

### An orchestrator failure mode, named because it happened twice

**A modality dropped in relay.** A lane wrote that a visibility scar is
inert *to the `tests/` side*, with the `src` side still blocking the
fold; the orchestrator's summary said "inert". A lane wrote that two
constructions of one box feed a certified width identically; the
orchestrator called it *"the first end-to-end check of the parent
unit's arena-level claim through a certified predicate"*, which the
measurement did not support. Both times the lane's text was correctly
scoped and the restatement was stronger. **The lane writes the careful
version; the orchestrator writes the quotable one, and the quotable one
is what travels** — which is this program's own subject (a row that
measured one thing and summarised it as the opposite) happening one
level up.

The repair is the same as everywhere else here: quote the row's own
sentence rather than paraphrasing it, and when a lane scopes a claim
down, carry the scope.

### Two things the reviewers established that outlive these units

- **A style brief that names its two suspicious hunks does most of what
  a full review would.** Style tier held on all three units; in each,
  both rounds' findings concentrated in the hunks the brief named as
  the places where a green diff tells you least.
- **Verify the reviewer too.** One delta read reported a wrong count as
  living in the lane's hand-back rather than in the tree. It was
  committed, in a row, on line 139. Merging on that summary would have
  landed a wrong count in the tracker of the program whose subject is
  counts that do not survive re-taking.

## 2026-09-20 — the `src` cylinder sheet: one construction, and the scar was not what held it apart

`dup/src-cyl-sheet`, three rows as one unit. `crates/topo/src` held
four spellings of the cylinder-wall sheet (`census::cyl_sheet`,
`census::cyl_sheet_b`, `chart_region::cyl_sheet`, and the door PR #2887
minted) and now holds one, with 316 lines gone.

**The finding: a "visibility scar" that was measured, named and wrong.**
PR #2887 measured one difference between the `src` pair and the
`tests/` door — the rim plane's minting route — and read it as a
`pub(crate)` scar. There are two. The scar is inert in both directions
(1 lib row, 0 of 566 integration). The other, where the cylinder key
lives, reds **1** integration row one way and **6** census rows the
other, and it is what actually kept the families apart. The scar's own
stated reason was false of the door the moment the door existed: it
lives in `src` and could always call `add_surface`; the scaffolds were
the `tests/` closures' workaround, carried into `src` with a
justification that the move had already dissolved.

**Method item 12, in a shape worth naming: a justification that
survives its own premise.** The sentence was true of the code it was
written about and false of the code it was written INTO, in one commit,
by the lane that wrote both. Item 12's earlier instances were text
going stale over time; this one was stale on arrival, and no re-reading
of the diff would have caught it — only building the thing the sentence
said was impossible. **The instrument for that class is the plant, not
the read.** The reviewer found the sharper half: the paragraph **named
its own escape in its own parenthetical** — *"a caller inside it does
not have to (`crate::census`'s own sheets call `add_surface`
directly…)"* — and kept the scaffolds anyway. It did not go stale; it
contradicted itself at the moment it was written.

**A mutation is not a proof when a suite can swallow it.** The fold's
proof plant reds 22 of 566 — and the two rows built on
`try_wall_sheet`'s `catch_unwind` stand down and report ok. A wrapper
whose premise is one failure mode catches every failure mode, so those
two rows are green over a correct builder and over a broken one alike.
Filed on S-TINT. **Item 11 needs the corollary: after a plant, check
the rows you EXPECTED to red and did not, not only the count.** And the
reviewer's half, which is the cheaper instrument and was sitting in the
same terminal: **the panic hook prints a full backtrace to stderr
before the row passes**, so the hole is not silent — it is loud and
reported green. *A passing row that emitted a panic backtrace is a
swallowed failure*, and that reads off a run nobody had to design.

Counts that moved, per item 15: the parent's "617 integration rows
green at both lanes" is the INTERVAL lane's count; the default lane is
566 and the probe lane 571, so a figure quoted without its feature row
names a set nobody can reproduce. The `try_wall_sheet` row's "token-
identical (one md5)" and "their doc comments differ in substance" were
both false at the merge base — and backwards: the doc comments were
byte-identical, the bodies were not, and `r1`'s copy cites a map that
lives in `r2`.

### Fix pass — four classes, and one of them is this program's own subject

A reader found eight. Three are worth the log.

- **A fold can move code out of a guard's reach, and the guard stays
  green.** `source_walk::public_fns` reads `pub fn` and rejects
  `pub(crate) fn`, so hoisting the sheet's Euler sequence into a
  `pub(crate)` shared body took it out of the tier-1 mutation-door
  population entirely — invisibly, because the `pub fn` it was cut from
  is allowlisted and an allowlisted door's body is never read. The lane
  then **edited that allowlist's prose to describe the code that had
  left**. Restored: the shared body is a `pub` door named by both door
  tables. **The generalisation the lane wrote was wrong in the axis**:
  the gates key on the `cfg` MOUNT, not on visibility — a `pub(crate)
  fn` under `#[cfg(test)]` is still skipped — and the door walk is the
  one place visibility decides, and it decides the other way. A fold
  out of a `#[cfg(test)]` mount has to be checked in both directions.
- **A fold that re-mints the thing it folded, inside the home it just
  made.** The shared stand-down wrapper was written spelling the door's
  argument list rather than calling the adapter already beside it —
  token-for-token the adapter it had just homed. Before the fold one
  called the other; after it they were two copies in one binary. This
  is X4 at its most ordinary and no census would have found it: the
  duplicate was minted by the fix.
- **A duplicate row, filed by the duplication program.** The lane
  opened a canonical-cylinder row against a class S-TINT has carried
  since 2026-09-03; neither referenced the other and they proposed
  different homes. Deleted before it reached the board and its evidence
  appended to the open row. **Item 14 applies to the tracker, not just
  to code**: grep the other program's directory before filing, which
  `work/README.md` already says and which is easy to skip when the
  finding feels new.

## 2026-09-20 — the second sitting: three more units, and the shape that repeated four times

`70941cdd0` (#2898, `faces_of_solid`), `1efaf7996` (#2899, the one-line box
wrappers), `820492add` (#2900, the viewer fixture sugar). Slate: **18
closed, 27 open, 1 parked**. Three method items were added from
measurements taken during these units (16, 17, 18) rather than from
argument.

### The finding: a fold leaves a member behind IN A FILE IT HAD OPEN

Four consecutive units, and by the fourth it was predictable enough to
brief for:

- `solid_contain.rs:2415` — the same arena scan under the negated
  predicate, **three lines below** the lines the fold rewrote. The
  lane's own instrument fired on it as a distinct hit; the hit list
  demoted it to a parenthetical. Not a miss — a **method item 9
  bucketing**.
- `props.rs` — an inline, unnamed copy that an *earlier* fold had
  edited around three lines away without seeing.
- `common/asm.rs` and `msolve5` — the viewer unit reached into both,
  pulled three helper classes out, and left a fourth member of its own
  class sitting in each.
- `mate_tool_flow.rs` and `review_gui4_r1.rs` — three longhand copies
  of the construction behind a door **the same commit had just
  minted**, one directly below and two sandwiched between calls that
  commit wrote.

The mechanism is not carelessness and it is not the instrument. It is
that a lane editing a file for one member has its attention on the
member, not the file — so the strongest single instruction to a lane
is **re-read every file you touched, for the class you are closing**,
and the strongest single question to a reader is *what else is in the
files this diff opened?*

### Instruments: the converse needle

The viewer unit's class was "apply an edit and take the result". Its
needle was the edit's constructor, which structurally cannot see a
re-spelling of the **write-back** — `= applied.doc` is what finds
those, and it is what closed the class. A needle aimed at a
construction's *entry* has a blind spot at its *exit*, and the two
needles are cheap to run together. (Method item 10's roster gains this
by example rather than by a new line.)

### Counts, again, and one new way to be wrong

- **A figure published from a census taken before the unit's own last
  fold**, never re-taken — the same shape as the ambient `.expect`
  rate measured after the fold it was the denominator for. A count is
  a claim about a **tree state**, not only about a date.
- **Two right numbers that agree by accident.** A bucket split was
  published as 8/5 twice over — but the two 8/5 partitions were
  different cuts of the same thirteen, one by window and one by shape.
  The coincidence is what made it read as verified. Neither implied
  the other.
- **A wrong total in the table whose job is to prove the folds live**:
  a plant made a suite panic, and the harness read the **first**
  `test result:` line, which a panicking shard emits before the
  suite's. The reader's guard is free and complete: **every row must
  sum to the baseline run count**, and the defect violates that by
  construction. Cheaper than a re-run, and it is how a reader checks
  any table taken before a harness fix.

### The orchestrator's own errors this sitting

- **Three lanes were briefed to read a log entry that was not on
  `main`** — it was on this branch and merged an hour later. One lane
  said so and read it out of the orchestrator's checkout; two said
  nothing, so whether they read it is unknown. **Check that briefed
  reading is on the branch the lane will cut from.**
- **A diffstat read against a moved `main`**, twice, once nearly
  reported as a lane deleting a tracker row. Other programs' merged
  work reads as the lane's deletions. Measure from the merge base.
- **A misread hand-back** turned into a brief item that was simply
  false (a fixture reported as declined had been folded). The lane
  corrected it.

### A merged unit left a member behind

#2886 closed two rows claiming a helper pair was folded.
`review_gui2_r1` still carried a private copy — path-qualified and
**broken across two `//!` lines**, which that unit's line-shaped
census could not see. Found by the next lane on the same territory,
with a whole-function scan. **A closed row is not evidence the class
is empty; it is evidence of what one instrument could see.**

## 2026-09-24 — the third sitting closes, and a fourth batch goes out

A weekly API limit stopped all three third-batch lanes mid fix pass on
2026-09-20. Each worktree was clean and pushed, and none had lost
committed work. Resumed today, each lane first checked what it had
actually done against HEAD. The viewer pass was complete. The
cylinder-sheet and shells passes had landed more than their last notes
said, but the shells lane found two earlier corrections an edit script
had silently dropped (method item 25).

While the lanes were down, main changed the tracker contract (priority
and cost bands, track budgets, "the tracker is not comprehensive",
`log.md` merges by union). All three branches conflicted on row
headers, and the viewer branch also conflicted on two suites main had
edited. Each lane merged main. Under the new contract, four filed rows
became commits instead (a stale doc line, a shared `REACH`/`aimed_along_y`,
`ring_delta`, `len_mm`), and one S-TINT row was deleted for the same
reason.

Merged: **#2929** (viewer doors), **#2925** (one cylinder-wall sheet in
`topo/src`), **#2926** (`Body::shells_of_solid`), and **#3144** (method
items 22–24). #2926's CI ran against main before #2925 landed and both
touch `topo/src`, so the orchestrator ran clippy on the merged tree
before merging. It was clean, and neither PR touches the door tables.

- **One lane closed and reopened its PR to kick CI.** That is never
  allowed. The run it wanted was missing because the PR conflicted, as
  the implementer discipline now says in §2. It had no effect beyond
  noise, and the brief now states the rule.
- **Lint accepts a row with no `priority`/`cost`.** `REQUIRED` is still
  `id, kind, title, status, opened`, so a green lint does not show the
  bands are there. Two lanes noticed independently. The lanes priced
  every row they touched anyway.

### The slate

`dup` measures **41/30** on the board: three P1 rows (4.5 points) and a
P4 tail of 36.5. The contract says an over-budget track splits on its
priority seam. **Chosen instead, for this sitting:** work the whole P1
spine and batch the cheap P4 rows, which brings the load under budget
without a split. A split into a P1 program and a P4 program would leave
a P4 program still over budget on its own and a P1 program of three rows
that are all in flight right now. If the load is still over 30 when this
batch lands, the P4 tail is cut into its own program then.

### Dispatched (fourth batch), with review tiers

| branch | rows | tier, and why |
| --- | --- | --- |
| `dup/owner-index-divergence` | `two-spellings-of-the-face-to-solid-owner-index` (P1/D), `shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim` | single FULL: it changes what a `pub` door answers about a lone vertex |
| `dup/scalar-lift-home` | `the-componentwise-scalar-lift-has-no-shared-home` (P1/E) | single FULL: a new public API on a `geom-core` type, folded across many crates |
| `dup/viewer-insert-doors` | `three-doors-named-insert-mean-two-different-constructions` (P1/E), `the-rectangle-profile-is-still-written-longhand-beside-its-door` | single STYLE: a rename and a fold, readable in full |
| `dup/topo-fixture-batch` | `the-9-3-holed-box-sequence-…`, `the-quad-sheet-helper-…`, `the-same-solid-two-shell-body-…`, `a-doors-rustdoc-carries-an-unguarded-census-sentence` | single STYLE: four mechanical folds |

No unit this batch met the dual-review bar (especially tricky logic, or
a broad design decision that would be hard to reverse). The owner-index
choice is narrow: one entity kind, in a skeletal state, on one door.

## 2026-09-25 — the fourth batch lands

A second weekly limit stopped all four lanes on 2026-09-24. When usage
reset (Ev, 2026-09-25) every worktree was pushed and clean except the
scalar-lift lane's in-flight edit, and each lane resumed from HEAD.

Merged: **#3150** (the viewer `insert` doors renamed for the door they
drive; rectangle longhands folded), **#3151** (`SolidOwners` places lone
vertices, one forward walk shaped like the offset scope's, pinned against
it; `shell10_r2_probes` folded), **#3152** (the §9.3 hole/ring/plating
surgery, the chart_region sheet helpers and the two-shell refile homed
once each; door census sentences rewritten), and **#3149** (the third
sitting's records). #3242 (scalar lift, no new door: `x.map(T::from_f64)`)
is in full review.

Every unit's review found something, and the findings rhyme:

- **The recurring fold defect, again, three times.** #3150's fold minted a
  byte-identical private twin (`add_instance` in two suites). #3152 moved
  surgery into `test_support_fixtures.rs` without re-measuring
  `DOORS_MEASURED` (57, not 54) or adding the new file to two suites'
  `gated_to!` lists, which would have silently skipped them on an edit to
  the moved bodies. #3151 corrected a stale premise in a row while
  leaving the same premise in the doc it edited. The move-a-body case has
  a follow-through of its own: **every check that keys on the file the
  body left** (door tables, gated lists) is part of the move.
- **Kept copies on recited reasons.** #3152 kept `review_f7`'s ring-face
  plant on a header's "preserved verbatim", while exporting the door that
  plant should have used with zero consumers. Folded in the fix pass.
- **Load-bearing kind assertion.** #3150's fix-pass plants showed the new
  `commit_mate` kind check is what keeps six rows honest: without it,
  `fault(mate).is_none()` holds for any non-mate id.

### The orchestrator's own errors this batch

- The dispatch brief cited method items 1–25 while item 25 was only on
  this branch (#3149 unmerged at the cut): item 20, repeated. Caught by
  #3150's reviewer.
- The topo-batch brief named `review_m1_pr3.rs` as a PR #17 attribution
  file; the seven are `review_m1_pr1.rs` and `review_m1_pr2/*`. The #3152
  reviewer brief asserted main had a "new copy" of `plane_every_face`;
  main had only moved it. Both are transposed facts in a brief (item 20's
  second clause), both corrected by the lane or reviewer.
- The standing "read the LAST `test result:` line" was wrong under
  `--no-fail-fast` (item 26), and the orchestrator's job-count check had
  gone stale under RING-4 (item 27).
- #3151's lane closed nothing it shouldn't have, but one lane last sitting
  closed and reopened a PR to kick CI; the brief now forbids it and no
  lane did it this batch.

### The slate

`dup` reads **39.5/30**: the batch closed seven rows and its lanes filed
nine (every one of them priced, most P4/E, one P1/D:
`step-program-embed-has-no-map-door`). Per the third sitting's log, the
P4 tail is cut into its own program once #3242 lands.

## 2026-09-26 — the fifth batch: two drains, and gating on local CI

Hosted CI's queue ran hours deep. Ev: run CI locally (a new override
sentence landed as #3276 so the certification is true), merge on local
green, mark such commits `[skip ci]`, combine units where convenient,
and use line tables only for disk (#3296 made that the repo default).

Merged: **#3242** (scalar lift, hosted green), **#3285** (viewer drain:
four rows closed, one left open for a design decision, none filed) and
**#3284** (sweep/topo drain: six rows closed, four filed). Both drains
went through a style review and a fix pass; both merged on a hosted
green that landed before the local run finished, with the local run's
completed rows (8161 and 8192 tests at two eps rows, the viewer app row
978/978) agreeing.

- **Fold, don't file held better, and the recurring defect still
  appeared in both.** The viewer drain's first cut stopped three class
  sweeps at the first file; the sweep drain minted a second home for the
  three-arc cylinder beside `mate2_common::three_arc` and left two
  byte-identical `ball_poled` twins in files it edited. Both fix passes
  went past their findings (the sweep pass folded eleven more cylinder
  spellings) and every fold was planted.
- **Local CI, runtime parity.** Static parity passed; the first local run
  still reddened the viewer app row for want of a Vulkan adapter the
  hosted half installs (method item 28). With it installed the row
  matches. On 4 cores a full local matrix is 4-6 hours, one eps test row
  25-70 minutes; with line tables a run's target fits in ~15 GB.
- **Method item 13 amended**: no rustdoc pointer to a tracker row.

### The slate, and a decision deferred to Ev's view

`dup` reads **40.5/30** after two drain batches that closed ten rows:
the lanes filed four, three of them D-cost populations (a y-poled ball
~50 sites, three-arc cylinders ~20, private `extruded` wrappers ~40).
The census-first method surfaces a class's remainder as fast as a
drain closes its head, and a priority-seam split is degenerate (the
slate is all P4 but one P1 and one P3). Recommendation, put to Ev in
chat: cut the population rows (the ones that are "route the rest of a
class onto a door that already exists") into their own program, which
lands under budget, and let S-DUP keep the rows that still need a door.

## 2026-09-26 — S-REROUTE is cut, and the sixth batch goes out as one PR

Ev approved splits of this kind without asking first ("yes you can do such splits without
asking"). #3298 moved seven route-the-rest rows by `git mv` into the new
`work/reroute/` program (11.5/30, ready), and `dup` now reads 29/30.
The cut is on the kind seam, stated in `work/reroute/plan.md`.

The sixth batch is three lanes on disjoint crates. It lands as one
combined PR gated on a local run with `[skip ci]` (Ev, 2026-09-26):

| lane | rows | fence |
|---|---|---|
| a | edges at a vertex (P1 D); a solid's charts as a move set (P4 D) | sweep |
| b | corpus pick walks; viewer/src literal doors; cross-crate pick rays (P4 D ×3) | viewer, bvh + editor-core pick helpers |
| c | the stale-vs-foreign key clause (P4 D); the Step-program map door (P4 D) | topo/src Body doors, profile, editor-core Step walks |

These rows are held back, and why:
- The two P4 H/D cross-crate populations (`Expr::literal` in 63 files,
  per-component point lifts) touch every fence, so they go after this batch.
- Two rows need Ev: the policy-memory citation and the fixture routing rule.
- The error-arm row needs a designer pass before its `[ev]` PR.
- The PR 17 attribution row touches the attribution sentence the orchestrator is
  told to leave alone.

### The full local matrix, measured end to end (#3284's merge ref)

This is the first complete local run: 39 rows, about 7.5 hours on 4 cores
with lane builds beside it.
- **Test matrix:** all three eps rows ran 8192/8192, and the viewer app row
  978/978.
- **Other passes:** doc-tests, rustdoc, wasm, k-lint, tess-budget and python.
- **Two FAILs, neither the tree's:**
  - *corrupt input (release profile).* The local half had rotted away from
    hosted. It lacked `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false`, so the
    release-only row never compiled, and it lacked the `review_d18` filter.
    The parity reader passed both. The batch-6 PR fixes the row and adds the
    variable to the reader's `SEMANTIC_ENV`, which a plant confirms. The
    filter half is filed to `mirror`.
  - *sheet drift.* A render re-baseline landed on main after the ref was
    cut, and the local output is byte-identical to main's. A gating run
    must be taken on a fresh merge with main.
- **step import** is a loud SKIP-as-PASS, because this box has no FreeCAD.
  Hosted is the gate of record for STEP.
