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
