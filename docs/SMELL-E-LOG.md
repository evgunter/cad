# SMELL-SCAN Track E — orchestrator log

**Constituted 2026-08-20.** Track E owns what was left of the smell-scan
register once Track A, Track B and Track D completed and Track C claimed its
own rows. The schedule itself lives in `docs/SMELL-SCAN-2026-08.md` §D, under
**Track E**; this file is the orchestrator's record — rulings, lane state,
review outcomes and incidents. **Live status is here and in §D, never in
`memories/`.**

**What this track is not.** It runs **beside** the model A/B experiment, not
inside it. No dispatch here is an A/B row, no result of it is comparable with
one, and **no lane on this track reads or edits `docs/MODEL-AB-LOG.md`**. The
experiment is paused on a model limit; the cheapest guarantee that pausing it
stays clean is that this track never touches the file. A lane that thinks it
needs to touch it is wrong and should ask.

---

## Review policy for this track

**Style review only, except at three rows.** The default is
`docs/prompts/reviewer-style-lane.md` (dispatcher notes:
`docs/REVIEW-STYLE-DISPATCH.md` — read once, then point by path, never paste).

**ADVERSARIAL additionally at D21, D25 and D27**, and at **D31** only if the
union's sort order turns out to be load-bearing. Those are the rows where a
wrong answer is *reachable*: D21 and D25 convert garbage-out into panics across
`crates/topo`'s Euler surgery surface — where #720's own per-site standard was
falsified across roughly half its sites — and D27 puts a newtype between a
refusal and a panic in the crate whose D9 rule is *never a panic*. Everywhere
else the risk is that the fix is incomplete or that a better fix existed, which
is the style lane's question and not the falsification lane's.

**The two questions this track adds to the style brief**, on every dispatch:

1. Is the finding's **original** stylistic problem now *completely* gone — not
   narrowed, not relocated, not half-closed in a way that reads as closed
   (§C13)?
2. Was it closed in the **best** way available, or merely in a way that
   compiles?

Both are phrased to require taste. Do not let them become checklist ticks —
§REVIEW-STYLE-DISPATCH §1 is the standing warning, and it binds the dispatcher
harder than the reviewer.

---

## Recording convention

**Each unit records its own completion in its own PR.** At the finding: a
bolded `FIXED by #NNN` lead, and the **original problem statement removed** —
version control keeps it, and a closed finding that still carries its problem
statement reads as open. The row is struck from §D's Track E table in the same
PR.

Every Track E PR therefore edits `docs/SMELL-SCAN-2026-08.md`, and they conflict
with each other by construction. **Merge one at a time**, and re-merge
`origin/main` into every open lane whenever one lands. A PR that goes
CONFLICTING runs *no* check runs at all — it looks like CI is absent rather than
failing (`memories/agent-lane-operations.md`).

**Row numbers are global and assigned centrally.** A lane that finds something
its own PR cannot carry does not invent a number: it asks, and gets the next
unassigned one in the D-sequence, which §D's *"No row number is reserved"*
paragraph owns. Three lanes collided on this once already.

---

## The state this track was handed, verified against the tree

Checked at `23b830e` (main, 2026-08-20), not transcribed from the records —
Track C's own §C-log lesson is that *a row minted from a review finding is
re-derived against the tree before it is written*.

- **Open PRs: five, all Track C's** — #731, #732, #734, #737, #738. Nothing on
  any other track is in flight.
- **Track A is complete.** A2 merged as #714 (residues → C11), A3's #678 landed
  as #684, A4's #667 landed as #683 and is **closed** — its remainder is #681,
  which had no owner and is now **E-l**.
- **Track D's live table had 16 rows, all unstarted.** Fourteen came here; D30
  and D32 went to Track C because each is a Track C lane's whole file.
- **Track C's row C8 has no lane** in that track's roster
  (`docs/SMELL-C-LOG.md`) — it was parked as *"fold into whoever next opens
  `step-import`"* and nobody has. Taken as **E-m**.
- **A doc defect, fixed while lifting the table**: §D's live-rows table carried
  **two `D24` rows**, the second (#748's, with `BlendArm::name` as the class's
  second witness) appended as a separate line while the first (#735's, one
  witness) was concatenated onto the end of the `D23` row — so the D23 row
  rendered with eight fields and swallowed a stale duplicate. The stale copy is
  gone; the current one stands.

---

## Container facts

This orchestrator runs in a Claude-Code-on-the-web container, not on the box
`memories/agent-lane-operations.md` was written for. What differs:

- **No `ssh`**, so `local-scripts/new-lane.sh` cannot run — it clones over SSH.
  Lanes clone the orchestrator checkout locally and repoint `origin` at HTTPS,
  which is the credentialed path here. This is a container fact, not a repo one;
  the committed script is right for the environment it was written for.
- **`~/.local/share/cad-work/` starts empty**, and nothing in it survives the
  container. *A pointer at an uncommitted file in a container home directory is
  a pointer at nothing, one preemption later* — review artefacts worth surviving
  go in a commit.
- **4 CPUs, ~15 GB RAM, ~29 GB writable.** A lane's `target/` runs 4–8 GB, so
  three or four concurrent build-carrying lanes is the ceiling here, not the
  ten the wave layout would otherwise allow. Doc-only lanes are free.
- **Hosted CI is the gate and the cheap option** (`memories/local-battery-scope.md`).
  It is also the only place a timing means anything, which is why **E-n (D20)
  closes on an attribution measured there** and not in this container.

---

## Lane roster

Letters are Track E lanes; the row IDs are §D's and did not change. Gates are
file-overlap and instrument-ordering gates — the dependency structure was
discharged before this track existed.

| lane | row(s) | scope | gate | review | state |
|---|---|---|---|---|---|
| **E-a** | D22 + D34 | `scripts/gates/`, `.github/workflows/ci.yml` | none | style | **in flight** |
| **E-b** | D23 | `docs/` + suite headers; code set is what the re-derivation finds | none | style | **in flight** |
| **E-c** | D26 | `docs/SMELL-SCAN-2026-08.md` §D and §S19 | none | style | **#752, in review** |
| **E-d** | D33 | `docs/predicate-dimension-audit.md` | none | style | **in flight** |
| **E-e** | D28 + issue #693 | `editor-core/src/eval/` | **confirm against C-f (#731)** — same crate, disjoint files | style | unstarted |
| **E-f** | D25 | `topo/src/euler.rs` and every `link_half_edges` caller | none | **ADVERSARIAL** | **#755, in review** |
| **E-g** | D27, then D29 | `sweep/src/fillet/{build,surgery,mod}.rs` | none | **ADVERSARIAL** (D27), style (D29) | unstarted |
| **E-h** | D21 | `topo/src/{split,attach,movefac,revert}.rs`, `splitting/finish.rs`, `boolean/combine.rs` | **E-f, for file overlap on `split.rs`** — see E-R4 | **ADVERSARIAL** | unstarted |
| **E-i** | D24 | `Cargo.toml` workspace lints, or `.github/workflows/` | none | style | unstarted |
| **E-j** | D31 | `sweep/src/skin.rs`, `geom/src/curves/fit.rs`, home in `geom-core/src/spline/algebra.rs` | **Track C (C-l, C-g)** | style, escalates if the sort order is load-bearing | unstarted |
| **E-k** | D35 | `docs/DESIGN.md`'s D2 addendum, and whatever the answer names | **E-g**, **E-h** | style | unstarted |
| **E-l** | #681 | everything outside `crates/*/src` | none | style | unstarted |
| **E-m** | #711 | `step-import/src/recognize.rs`, `docs/ASM-R2A-SPEC.md` | none | style | unstarted |
| **E-n** | D20 | `topo/src/seqgen.rs` | none | style; closes on an attribution off hosted CI | unstarted |

**Not taken by Track E:** D30 and D32 (Track C's files — C-m, C-q); C11's #726
and #727 (Track A's residues, in `mesh/` and `props/`, which are C-k's and C-m's
scopes); L1, L2 and L3, which are deliberately last and stay that way.

---

## Incidents and corrections

### E-R4 — a gate whose stated reason is refuted is not a gate that has fallen (2026-08-20)

**E-f (#755) refuted the reason I gave for E-h's gate, and then removed the
gate.** The register said D21's half-edge sites *"inherit D25 for free"*, so
sequencing D21 after D25 would discharge them structurally rather than site by
site. E-f checked it against the tree and it is **false**: not one of D21's 14
sites is a half-edge key handed to a splice — three in `split_edge` are an edge
and two vertices, `attach.rs`'s are a face and an edge, `movefac.rs`'s a shell,
a face and a solid, and the one half-edge-arena lookup on the list
(`revert.rs:206`) is a field write into a clone of the arena it iterates, not a
splice. That correction is right, it is exactly what a lane is for, and it
stands.

**But the gate survives its reason.** Track E's gates are **file-overlap and
instrument-ordering** gates, not dependency gates — §D's edge list says so in
terms — and #755 rewrites `crates/topo/src/split.rs`, where D21's file set
opens. Two lanes rewriting one function collide whether or not either inherits
anything from the other. The lane was asked to record **both** claims: that D21
inherits nothing (its finding, in its words) and that E-h still sequences after
E-f for file overlap, which is a different and weaker claim than the one it
removed.

*Generalisable, and it is the dispatcher's error as much as the lane's:* **I
wrote a gate down with one reason attached, and the reason was load-bearing for
the gate's survival in a way I did not intend.** A gate stated as *"X because
Y"* invites a lane that disproves Y to delete X. State the mechanism a gate
protects, not the story you happen to have for it — and when a lane refutes the
story, the dispatcher re-derives the gate rather than accepting its removal.

**Corrected in `147e3a4`**, and the lane's own account of how it happened is the
sharper half: *"I read 'the gates are not dependency gates' in the roster, found
the dependency was fictional, and deleted the gate rather than checking whether
the gate rested on the dependency at all. The file overlap was visible in my own
diff — `split.rs` is on both scopes — and I reported the line numbers moving
inside `split_edge` as a **courtesy to E-h** in the same breath as cutting the
edge that would have kept E-h from colliding with it."* The evidence against the
deletion was in the deleting lane's hands, in the same paragraph. That is not
carelessness; it is what happens when a refutation and its consequence are
reasoned about separately, and it is why a gate removal is the dispatcher's call
and not a lane's.

**One byproduct worth carrying, for E-a.** This unit's `doc-gate.sh` red — a
deletion orphaning a rustdoc intra-doc link, invisible to both `cargo build` and
`clippy -p topo --lib` — is a **third** instance of the class D34's third
decision turns on, after #740 and #744, and the first whose shape is *a lane
creating the defect precisely by doing the right thing*. Forwarded to E-a as
evidence, explicitly not as an instruction: the row closes on a verdict either
way, and the cost side is that lane's to measure.

---

## The standing lane header

**Every Track E dispatch carries this by reference** — *"read
`docs/SMELL-E-LOG.md` § The standing lane header in full before you start"*. It
is committed rather than kept in a container home directory on purpose: a
pointer at an uncommitted file is a pointer at nothing, one preemption later.

**1. Read these, by path, before touching anything.**
`docs/prompts/implementer-discipline.md` in full; your row in
`docs/SMELL-SCAN-2026-08.md` §D Track E, and the **finding** it came from
(follow the `Was` column); `CLAUDE.md`; and `memories/MEMORY.md`'s index,
following the pointers your row actually touches.

**2. The A/B fence.** This track runs **beside** the model A/B experiment. Do
not read, cite, or edit `docs/MODEL-AB-LOG.md`, and do not describe your unit
as an A/B row anywhere. If something seems to require it, stop and ask the
orchestrator.

**3. Your branch and your lane.** Branch `smelle/<row>` (e.g. `smelle/d25`),
off current `origin/main`. Use **your own** `CARGO_TARGET_DIR`; never one shared
with another worktree. Anything to be published — a PR body, a review findings
file — goes to a **lane-private** path, never the session scratchpad, which is
shared between concurrently running agents.

**4. Commit and push at every coherent seam**, not at the end. Two lanes have
been lost mid-unit with everything uncommitted; a pushed commit is the only
state that survives.

**5. Re-derive; do not transcribe.** Your row was written from another lane's
record. Records are accurate about the moment they describe, and a fix pass may
have moved the tree since. **Check every citation in your brief and in your row
against the tree before you build on it** — five briefs on the sibling track
carried a citation that did not resolve, and one carried a claim that was simply
false on main. Checking rather than complying is the behaviour that is wanted;
say in your report what did not resolve.

**6. State what your sweep cannot match.** If your unit closes an instance of a
class, name the pattern and its blind spot. A path glob is part of the pattern.
A count is not a deliverable; a sweep with a stated blind spot is.

**7. Merge `origin/main` immediately before opening the PR, and re-merge
whenever main moves while it is open.** A PR that goes CONFLICTING runs **no**
check runs at all — it reads as CI absent, not CI failing. After any push,
confirm checks actually **started** by reading the workflow *runs* list.

**8. Record your completion at the finding, in your own PR.** A bolded
`FIXED by #NNN` lead at the finding in `docs/SMELL-SCAN-2026-08.md`, the
**original problem statement removed** (version control keeps it), and your row
struck from §D's Track E table. Every Track E PR edits that file, so expect to
re-merge; merges are serialized by the orchestrator.

**9. If your unit finds something it cannot carry, it is a row, not a
footnote.** Ask the orchestrator for a number — do not invent one, and do not
leave it in the PR body. *A verdict is not a placement* (§D ordering rule 4).

**10. Your report is a claim site too.** ≤150 lines. Name the questions you
actually exercised, what you could not check, and anything in this header or in
your brief that turned out to be wrong. Reviewers correcting the dispatcher is a
working lane, not a malfunction.

---

## Rulings made in this track

| # | Ruling |
|---|---|
| **E-R1** | **Row numbers do not get renumbered by a change of track.** Merged PR bodies cite `D21`, `D27`, `D35`; a Track-E renumbering would fork the one register for a cosmetic gain. Track letters name *ownership*; row numbers name *placements*, and the two are not the same axis. |
| **E-R2** | **A track with no orchestrator is not a schedule.** Track D's rows were correct, placed and edge-free, and would still have been unstarted a week later. The audit that produced Track D found this once already (§C3); this track is the same finding applied one level up — a *track* that does not execute is not a register either. |
| **E-R3** | **A row whose files belong to a live lane goes to that lane, not to a second track.** D30 → C-m and D32 → C-q. The alternative is two tracks editing one file with no shared orchestrator, which is the collision the whole sequencing exists to prevent. If Track C declines, the row returns here — never to nobody. |

---

## Row-number reservations

Blocks handed to lanes so they do not round-trip for a number mid-unit. A lane
uses only what it needs and hands the rest back; anything past its block comes
from the orchestrator. **D35 was the highest number placed when Track E was
constituted.**

| block | lane | state |
|---|---|---|
| D36–D39 | E-c (D26) | **all four used** — D36 `UnsupportedCarrier`, D37 `tags.rs`'s residue, D38 `SkippedMerge`, D39 `ProgramRefusal::Geometry` |
| D40–D41 | E-a (D22 + D34) | reserved 2026-08-20 |
| D42–D43 | E-f (D25) | **returned unused** — the unit's two findings were corrections recorded at their own entries, and neither leaves work behind |
| D44–D45 | E-b (D23) | reserved 2026-08-20 |
| D47–D48 | E-c's fix pass (#752) | assigned 2026-08-20 — the `pncad-py` `Debug`-dump class and its unguarded rule; `select_refusal_tag`'s vacuous alarm |
| D46 | E-d (D33) | reserved 2026-08-20 |

Next unassigned: **D49**; D42 and D43 are back in the pool and deliberately not
re-issued — a number that has appeared in a lane's report, even as *unused*, is
cheaper to skip than to explain.

**D36–D39 were assigned by the orchestrator**, in E-c's dispatch, not minted by
the lane. #752's style review could not tell from outside and asked; recording
it here is the answer, and the fact that it was not visible from the PR is
itself worth knowing — a lane that follows the rule should be able to *show*
that it did.

**D36–D39 are placed and unstaffed**, and they are Track E's to schedule.
E-c's report says D37(a) and D39 want **one** lane, not two: both need a
fieldless `Copy + Eq` discriminant on `PathError`, which `pncad-py/src/tags.rs`
has already hand-written once. D37 is gated on **D28** (E-e). Schedule them
once #752 lands and the review has had its say about whether these four are
rows a taker can act on — which is one of the two things that review owes.

---

## In flight

**Wave 1 opened 2026-08-20**, three lanes, no shared file except
`docs/SMELL-SCAN-2026-08.md` — which every Track E PR edits, so merges are
serialized here and each lane re-merges `origin/main` when one lands.

| lane | row(s) | branch | PR | state |
|---|---|---|---|---|
| **E-c** | D26 | `smelle/d26` | **#752** | **CLEARED**; fix pass running (8 findings, 2 → rows D47/D48). Merges first |
| **E-a** | D22 + D34 | `smelle/d22-d34` | — | implementing |
| **E-f** | D25 | `smelle/d25` | **#755** | style lane **CLEARED** (1 must-fix, 11 findings); **adversarial lane still running**. One combined fix pass when it reports |
| **E-b** | D23 | `smelle/d23` | — | dispatched. **Fenced off `scripts/gates/probe-suite-census.sh` and `ci.yml`** while E-a holds them |
| **E-d** | D33 | `smelle/d33` | — | dispatched |

Not yet dispatched: E-e and E-g are wave-1-eligible and held only by this
container's build capacity. E-h waits on E-f, E-k on E-g and E-h, E-j on a
Track C confirmation. E-i, E-l, E-m are wave 2; E-n is last.

---

## Reviews

### #752 (E-c / D26) — style lane, 2026-08-20: **CLEARED**, fix pass running

All seven claims held. The two questions this track adds both came back
positive, and the second is the one worth recording: **D36–D39 do not reproduce
the defect they close.** Each carries a deliverable, a closing condition, a
scope and a named constraint the taker hits first; D36 names its *decision*
rather than its edit; D37 and D39 each admit a written scope verdict as a pass.
None reads as *"someone should look at this"*, which is what D26 exists to stop.
The `tags.rs` refutation was judged the **best** available answer rather than a
compiling one, and its residue was preserved rather than dissolved.

**Eight style findings, and the shape they share is the review's real result.**
Three of the eight are *the finding recurring inside its own fix*:

- **A gate inherited as prose and re-published as verified.** D37's gate on D28
  said the payload is discarded *before any tag is computed*, so nothing in
  `tags.rs` can recover it. `node_error_tag` takes `&NodeErrorKind` — the typed
  value — and `Display` renders at a different call site. The sentence came
  verbatim from D28's row, and the PR presented repeating it as *"the
  re-derivation confirms it from the other side"*. **A restatement is not a
  check**, and this is the one thing a re-derivation lane was specifically there
  to catch.
- **A stated negative result that was false.** The lane declared it had checked
  for closure factories bound to another name and found none; `pcurve_cache.rs`
  has two, one of them fifteen lines from the sibling D36's whole asymmetry
  argument rests on, with eight call sites. It cost D36 a count.
- **Citations pointing at the enclosing construct** — in the PR whose deliverable
  was correcting exactly that defect in S19.

Two findings became rows (**D47**, **D48**); the rest are fixes in place.

*Generalisable:* **the class to watch is a claim that travels between documents
without being re-derived at each stop.** All three above are one mechanism —
prose is cheap to move and expensive to check, so it moves. §D's own rows are
now a place this happens, which is new: the register was built to stop findings
being lost, and it can lose their *accuracy* instead.

### #755 (E-f / D25) — style lane, 2026-08-20: **CLEARED**, one must-fix; adversarial lane still running

**Question 1 came back split, and the split is the useful part.** The
*precondition* is genuinely no longer prose — the compiler carries it at all 44
sites, and that half is completely gone, not narrowed. But **prose reappeared
wearing a module-doc hat, three times bigger**: across `crates/topo/src` the
diff is **+152 comment lines against −51**, so ~78% of the unit's net growth is
comment. `live.rs` retired ~22 doc lines on `link_half_edges` and shipped ~82.
Two pieces of it argue rather than describe — a **rejected alternative** (the
GhostCell brand) and an anticipated objection — which is §S38's pattern
exactly, and S38 is ACCEPTED with Evan's *"should definitely be trimmed"*. L2
inherits it.

The lane judged the residue an **honest, correctly-scoped narrowing** rather
than the defect re-minted, and the reasoning is worth keeping: the obligation
went from *seven callers must each prove a key resolves* to *one crate-wide
ordering rule whose violation is loud*, and the failure mode is unchanged
because slotmaps version their keys. What shrank was the obligation; what grew
was the word count. Those are different axes and the review is the first thing
on this track to separate them.

**MAJOR (must fix before merge): `docs/DESIGN.md` still says this cannot be
done.** The D2 addendum reads *"…carry a precondition rather than a per-site
proof, **and cannot carry one** … Retiring that asymmetry — a `Live` key type
that makes the discharge structural — **is** `SMELL-SCAN-2026-08.md`'s D25."*
The unit removed the identical sentence from `link_half_edges`' own doc and did
not sweep for it. It is in the **ratified contract**, and it is precisely what
this track's recording convention exists to prevent: a closed finding that still
reads as open. **The trap is worth more than the instance** — a citation sweep
that filters out lines mentioning `SMELL-SCAN-2026-08.md` hides this line,
*because the line contains that string*. The reviewer's own first grep missed it
for that reason.

**Question 2 found a better option available and not taken: the name.**
`certify` was already this codebase's word for **geometric** certification —
`geom-brep/src/certify.rs`, `certify_edge_spec`, `CertifiedEnclosure`, 279
occurrences of *certified* in `topo` alone, essentially all geometric. In one
function, `split.rs:162` (`certify_half_edge`, liveness) and `:224`
(`certify_edge_spec`, geometry) sit sixty lines apart with comments at `:158`
and `:222` both saying *certify* about different things. The type is already
called `Live`; `require_live` / `Live::of` / *proven* were free. **This is §S4
inverted — one spelling, two concepts** — and it is the clearest thing on this
track so far where the best answer was available and a working one was taken.

**Two findings feed the adversarial lane and were forwarded mid-review**, per
the precedent: *"one constructor"* is false (four mint doors, and
`resolve_half_edge_live` mints raw rather than through `Live::certify`, at ten
call sites), and the unforgeability claim is guarded by nothing, with the
crate's usual `compile_fail` instrument unable to reach a `pub(crate)` type.

**The class the review would look at next** (orchestrator's call whether it is a
row): the literal block D25 named survives ~9 times for the **other five
arenas** — shells, faces, vertices, loops, solids, surfaces. The *token* half
correctly does not transfer, and the PR says so; the **token-free** half does —
`certify_half_edge` is now a one-line refusal helper and the other five arenas
still spell four lines out longhand.

---

## Landings

*(none yet)*
