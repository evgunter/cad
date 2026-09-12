# DOOR log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/door/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3). Eleven rows moved in by
`git mv`, each with a `## Re-homed` record: five from `work/issues/`,
six from `work/code-quality/`.

This is the cut's cheapest track by a wide margin — seven `E` rows of the
nineteen the whole cut found — and that is the reason it exists as a
track rather than as eleven routings: the board had no way to say so.

No branch exists yet. First look: `S190`, whose park cites a closed
trigger.

## Read against the tree before the first dispatch (2026-09-11)

Orchestrator seated. Posture confirmed by Ev in-chat: **no A/B row**,
light style reviews by default, full correctness review only where a row
has real risk of being wrong. `plan.md`'s **Review posture** names the
four that qualify and why.

Every row was read against the tree before anything was dispatched,
which the charter's own test makes worth doing: a row asserting its fix
is written is a claim about the tree, and three of eleven did not
survive it.

**`S190` was already fixed — closed, not dispatched.** The plan called
it the first thing to look at, on its park citing a closed trigger; the
answer is that both halves landed and nothing was owed. The kernel half
gives `CensusUnsupported` a `CensusSubject` whose `FacePair` `PartialEq`
is **written rather than derived**, so the arena-order question the
finding was about is closed in the type. The consumption half resolves
through `assembly.rs`'s `by_pair`. And the row's load-bearing claim —
that no single-declaration fixture could tell a face-width lookup from a
pair lookup, so the defect had no test that could go red — is answered
by `a_face_in_two_declarations_answers_each_pair_to_its_own_mate`,
asserting in **both** arena orders. The row's file carries the citations.
Closing it also cleared the `work.py lint` warning it had been throwing
since #855 closed.

**`viewer-cannot-author-a-part-node` went to CHROME.** It is the row
that failed the charter test, and it failed it in the way worth
recording: it names four files, which reads like a diff, and does not
name what the `AddPart` op takes — which is the whole of the work. A
file list is not a fix. CHROME owns the territory and closed the sibling
gap (`placed-union-has-no-session-op`, PR 1762), which is both where it
belongs and the shape to copy.

**`viewer-pathverb-all-hand-written-seventeen` lost half its premise to
VIEW's `const ALL` unit.** `PathVerb` moved to `forms.rs` and is
declared through `vocabulary!`, so `ALL` is projected and the
hand-written seventeen the title names is gone. What survives is whole
and is the half that mattered: `PathVerb::of` is exhaustive over the
**viewer's own** `PathStep`, so the projection and the match hold the
viewer to itself and nothing relates either to `profile`'s nineteen
verbs. `CircleSplit` still has no viewer spelling. The row's cheaper fix
and its more expensive one have swapped places, which is the kind of
thing a stale premise does quietly.

**Two rows claimed from VIEW, before any code.** DOOR opened holding two
instances of a class whose head sat on VIEW's slate
(`hand-maintained-mirrors-of-a-kernel-enum-are-unforced`), and the fix
that closes either instance closes the head too — one publication of
`topo::BooleanOp::ALL` retires `forms::BOOLEAN_OPS` and the
`kernel_wire` copy together. Landing that from here while the head sat
elsewhere would have closed another program's row by side effect. Ev's
direction in-chat was to take the items and to take them **out of VIEW
first rather than after the work**, which is the order `work/README.md`
prescribes and the order followed here; VIEW was told.
`dimension-radio-row-is-an-unforced-mirror-of-a-kernel-enum` came with
the head on that row's own instruction (whoever takes either takes
both). Five stale `work/view/…` path citations were repointed, one of
them in `crates/viewer/src/forms.rs`.

**What did not come with them:** `forms::MATE_PRIMITIVES`, which the
head names and the site documents as riding it. It is **deliberately
partial** — `MatePrimitive` has a fourth variant the kernel represents
so it can refuse it — so it is a mirror no mechanism should force. The
class's PR argues that at the site rather than projecting it.

**The mirror class does not get one PR per row**, and this is the
program's one ruled departure from one-PR-one-row: two PRs, one per
kernel enum published, each naming the rows it closes. The posture
exists so a lane does not widen into a second row's file; here the
second row is the same file and the same diff, and splitting would mean
landing half a projection. Ruled in `plan.md`'s **Order** and nowhere
else — a lane does not mint a second exception for itself.

**`patherror-display-renders-float-noise` grew a correction before it
grew a sweep.** Ev's caution in-chat was to round at a principled point
rather than one that could obscure real geometry, and reading the helper
showed the caution was understated: `path::num` rounds at a *relative*
1e-9 while D4's ε is ~1e-9 m **absolute**, so at metre scale the
rounding point sits exactly on ε_precision and above it is coarser — and
these sentences mostly report margins *against* ε. Propagating that
constant to every arm first, then discovering this, was the expensive
order available.

**The rounding point needed both grids, and finding out why took the
call sites.** Ev's proposal was a fixed absolute quantum one decade
below ε, and above ε it is exactly right: the kernel cannot distinguish
finer, so further digits are noise a reader cannot act on. It fails
below ε, and it fails on the call sites that dominate this family —
`JunctionTangent` and its siblings report a `margin` that is *below the
threshold by construction*, since being below it is what makes the
junction tangent, so a 1e-10 grid renders the only number in the
sentence as `0 m`. The helper's own doc had already made that argument
correctly (*"never to the nearest nanometre and never to `0`"*) and then
undercut it with the constant it picked, which is a tidy example of a
justification outliving the number it defends. So: the finer of a 1e-10
absolute floor and a relative noise band, with the relative arm carrying
the sub-ε payloads.

**One call the orchestrator made on top of that**: the floor reads the
compile-time `DEFAULT_EPS`, never `Tolerance::eps()`. ε is a live
process value and a code-tier run gates {default, 1e-6, 1e-12}; reading
it would make every rendered refusal a function of process
configuration and every string assertion in the tree eps-sensitive
across three rows. That is the difference between this row being an `M`
and an `H`.

**No guard is owed on `run-on-whitespace`** (Ev, in-chat): fixing the
five literals is the row. A guard rides along only if it is trivial and
costless beside `error_display.rs`'s existing predicate; if it needs its
own corpus or its own pattern language it is neither, and the lane says
so rather than growing the row.

**The slate stands at eleven rows, eight `E` and three `M`** — the same
count it opened with, by coincidence and not by conservation. No branch
exists yet; dispatch follows this entry, viewer rows first.

## Two more rows overtaken, and a finding handed to FIX (2026-09-11)

The liveness check that closed `S190` was run over the rest of the
slate before dispatching any of it. It found two more.

**`run-on-whitespace-in-message-literals` was closed work.** FIX merged
PR #2364 at 15:15 UTC — 27 wrapped literals with their `\` restored —
and closed its row at 15:42. This directory was created at 16:11. The
row is a duplicate and was never dispatched. Its five-site list was a
subset of a real 27 with one entry naming a file the pick split had
already dissolved, which is FIX's own assessment of it: *"a third false
and missed nine genuine sites."* The guard question I had put to Ev was
answered there too, with a **measured** threshold — real sites carry
runs of ≥10 spaces, and the 4 this row's `rg` proposes sits inside the
deliberate-alignment cluster. Confirmed here before the duplication was
found: an accurate string-state scanner still returns 1237 hits at a
threshold of 10, because the population is legitimate multi-line
literals. That question is withdrawn.

**`patherror-display-renders-float-noise` is narrowed, not
dispatched.** Its motivating example already renders correctly, and all
38 call sites in `path.rs` already reach the helper — FIX closed
`path-error-numbers-below-1e-9-render-as-zero` (PR #2366) on the same
file today. What survives is the cross-crate half and the helper's
home.

**A correction to this program's own rounding ruling.** The hazard the
ruling was written against — an absolute grid flattening sub-ε margins
to `0` — is not a hypothesis. It is the exact bug #2366 fixed hours
earlier: `num` read `tol = 1e-9 * x.abs().max(1.0)`, and that
`.max(1.0)` pinned the tolerance absolute below a metre. The relative
form this program read and criticised IS that repair. The ruling's
substance survives because it caps the tolerance rather than flooring
it and the relative arm still wins below a decimetre — but it was
written as a derivation when it should have been written as a citation,
and the tree had already paid for the lesson.

**The surviving half is a real defect and went to FIX**, not onto this
slate: `num`'s relative 1e-9 crosses ε (a LENGTH, ~1e-9 m) at one metre
and is coarser above it, so two lengths the kernel can certify as
different render as one number — measured, 10 ε apart at 100 m and 1000
ε apart at 10 km both collide. Filed as
`work/fix/num-relative-tolerance-collides-above-a-decimetre` with the
table, the fix shape and the assertion it owes. FIX's file, FIX's
program, two of FIX's rows closed on that helper today.

**Where this leaves the program.** Ten rows, seven `E` and three `M`,
and five of the cut's original eleven have now been overtaken — three
of them by FIX and one by VIEW, all on the day DOOR opened. The cut
read `work/issues/` and `work/code-quality/` against the tree; it did
not read them against the live slates of the programs already working
the same ground, and FIX is the program DOOR's own charter names as its
precedent. **Whether DOOR should hold the remainder at all is a
question for Ev**, put to them in-chat: the mirror class and the four
independent rows are coherent here, and the rest may simply be FIX's.
No lane is dispatched until that is answered.

## The mirror class's first PR landed (#2387, 2026-09-11)

`topo::BooleanOp::ALL` is published and three hand-written complete
lists of that enum are retired — the two the row named plus a third the
lane found, `EVERY_OPERATION` in `crates/editor-core/tests/boolean_op_wire.rs`,
whose own doc asserted such a list *could not* be tied to the variant
list. Closes `boolean-op-has-a-third-hand-written-complete-list` and
`hand-maintained-mirrors-of-a-kernel-enum-are-unforced`, which is the
two-rows-one-PR departure `plan.md`'s **Order** rules and the reason
those rows were claimed from VIEW before any code was written.

**The brief's correction held.** `ContactClass::ALL`'s argument is
`#[non_exhaustive]`'s and does not transfer to a closed enum; the lane
wrote the narrower true one instead of copying it — a list cannot be
derived from a match in safe Rust, so only the declaring crate can site
the list beside the matches that force a new variant through the same
impl block.

**Retiring the mirror was not a viewer-local edit**, which nobody
predicted. `scripts/gates/viewer-vocab-declared-once.sh` reads
`crates/viewer/README.md`'s roster and reds on a row whose list is gone,
so the roster row had to go, and the ratified KIND it claimed then had
zero instances — a ratification amendment costing the gate's
`KIND_ANCHOR` and `KIND_COUNT`. The README documents that amendment
procedure for ADDING a kind; this ran it in reverse. Gate and
`--selftest` both green, checked by the orchestrator and again by the
reviewer, and the orchestrator has flagged the ratification change to Ev
rather than letting it pass unremarked.

**Ordering rule 5 fired, exactly as the plan warned it would.** The fix
for a hand-written list minted a hand-written census, and the first PR
body asserted *"No new census"* two lines after admitting it created
one. The reviewer caught it; the orchestrator had read the census,
called it a red-able guard, and not executed it. Executed, the hole is
plain: add a fourth variant, copy-paste the arm the failing match asks
for, and the row passes green with the variant absent from `ALL`. **The
census forces the visit, not the update.** Both docs now claim only
that, `kernel_wire`'s deleted "NOT the compiler: that a new operation
reaches `ALL`" bullet is restored and re-aimed, and the hole — which is
inherited verbatim at all three sites of the idiom
(`param_source.rs`, `verb.rs`, and now here) — is filed as
`all-census-idiom-forces-the-visit-not-the-update`.

The lesson generalises past this unit and is why the review posture
holds: **reading a guard is not running it.** Two lanes and an
orchestrator read that census and approved it; a `rustc` invocation
costing thirty seconds falsified it.

**Five more items filed**, four of them residue of the two closed rows,
given files at disclosure rather than left in a merged PR body. The best
is `surface-and-curve-kind-mirrors-have-a-tautological-guard`:
`the_surface_kind_mirror_is_complete` asserts
`count(of(ALL)) == len(ALL)` where both sides derive from the list under
test, so a kind missing from the list is missing from both and the row
stays green — and two doc comments state the guard as real. Verified
from scratch by the reviewer rather than taken on trust.

**Also learned, and worth the next lane's time:** running a gate is not
running its `--selftest`; the lane had all 21 gates green in plain mode
and was red on the self-test's planted README fixture. And this
program's `keep_out` did not anticipate `crates/viewer/README.md` or
`scripts/gates/` — territory named eight foreign paths where the brief
predicted three, all announced.

## The mirror class is closed (#2391, 2026-09-11)

`editor_core::Dimension::ALL` is published and the viewer's inline
four-entry mirror is gone. **All four rows of the class are closed** —
the head, the boolean instance, the third `editor-core` copy #2387
found, and this one — in the two PRs `plan.md`'s **Order** ruled.

**The lane refused an instruction in the brief and was right to.** The
brief said to copy #2387 and add a viewer-side `dimension_label`.
`BooleanOp` had no rendering, so `boolean_op_label` was its words' first
home; `Dimension` already has one, and `crates/editor-core/src/expr.rs`
calls its `Display` *"the one home of the dimension-in-prose rule for
the crate"* while `tests/display_contract.rs:408` lists
`["Length","Angle","Count","Scalar"]` as **dumps** it refuses in user
prose — the four strings the radio row was rendering. A
`dimension_label` would have been a fourth spelling of a word list the
crate deliberately owns. The row reads `dimension.to_string()`, which
also fixes a live violation of that contract in the GUI. The
"dispatch is a hypothesis" clause earning its place, against the
orchestrator.

**The orchestrator's framing of the review question was also wrong.**
It asked whether a radio label is "prose", on the premise that
`expr.rs`'s rule is scoped to refusal sentences. The clause says
*"wherever a dimension reaches a user"* and names the refusal channels
as an enumeration of the current reachers, not as the scope. The
question was still worth asking and the answer came from the tree
rather than from the clause: **every vocabulary label in the viewer is
lowercase** — all seventeen `PathVerb` words, `DatumKind`, `ShapeKind`,
`ArcMode`, `PatternKindChoice`, `boolean_op_label`, `MATE_PRIMITIVES`,
`BlendKindChoice`, `PatternOutputChoice` — and the only capitalised UI
strings in the crate are three pane titles and the action buttons. The
add-parameter row was the one vocabulary control rendering capitalised
variant identifiers. The seven drifted GUI cells are a genuine
re-baseline, through a mechanism verified rather than assumed
(`ci.yml:5109-5118`: PRs report, `main` commits).

**Ordering rule 5 fired again, and differently.** #2387 minted a census
and denied it. #2391's census is honest — but the class item
`all-census-idiom-forces-the-visit-not-the-update` was filed carrying
"three sites", #2391 made it four, and nothing updated it. **A row about
hand-maintained counts went stale by one within the hour of being
filed.** Measured since, with a structural scan: the population is at
least NINE, and the scan's own windowing means that is a floor. The item
now carries the table, the floor, and a refusal to carry a number —
establishing the count is the instrument's job.

The same shape a third time: `dimension-all-has-readers-outside-the-viewer`
said five, its sweep was shaped for bracketed arrays, and three more
complete four-variant enumerations written as consecutive statements
were invisible to it — two in `pncad-py`, **one in `crates/viewer`
itself**, so the closed row's own territory was not swept clean by the
PR that closed it. Both the sites and the blind spot are now in the
file rather than only in a PR body.

**Filed**: `unit-symbol-proptest-generators-under-cover-with-no-file`.
`expr.rs:296-300` has disclosed for some time that `u8a_parse.rs`'s two
`prop_oneof!` generators silently under-cover the unit symbols; the only
tracker row naming that file reaches the same two blocks for their
*Dimension* half only. Disclosed at the site, scheduled by nothing,
which is the one thing neither `work/README.md` nor the implementer
discipline permits.

**Corrected**: this program's `keep_out` asserted "crates/editor-core/*
is DOCM's". #2391 measured it false — DOCM's `paths:` is a specific file
list and `expr.rs` is on nobody's — and the next DOOR lane would have
read it. An `editor-core` path is owned or unowned file by file, and
`work.py territory` is the only authority.

**Where the program stands.** The mirror class, the reason DOOR claimed
two rows from VIEW, is done. What remains is the five independent rows
of the original cut plus the residue the class filed. Whether the five
belong here or on FIX's slate is still Ev's open question, put to them
in-chat and not blocking anything.

## D306 and S414 landed; three things worth the next lane's time (2026-09-12)

`D306` (#2394) and `S414` (#2395) are closed. Five of the cut's eleven
rows remain: `S114`, `step-adopt`, `patherror` (narrowed), `grid-pitch`
and `pathverb`, beside seven residue items the lanes filed.

### A regex-shaped sweep has now missed an instance in its own PR's fence, three units running

This is the program's standing pattern and it wants recording as one,
not as three facts about three files.

1. **The mirror class**: the readers sweep matched bracketed arrays, so
   three complete enumerations written as consecutive statements were
   invisible — **one of them in `crates/viewer`**, the crate the closed
   row was about.
2. **D306**: the orchestrator's sweep matched the
   `eval_in_span`/`deriv_in_span` adjacency and declared a blind spot of
   "a pair split across lines, or routed through a helper". It missed
   `crates/geom/src/curves/projection.rs:242`, which binds `C″` to `_`
   from `ders_in_span` — **the second consumer the door's own doc
   names**. The stated blind spot did not include the shape the
   documentation spelled out.
3. **D306 again, one level down**: the residue file's own sweep declared
   the same blind spot and missed `certify.rs:1743`/`:1805` — inside
   D306's own crate — and `sweep/src/skin.rs:1120`/`:1160`, the
   helper-routed case it had named and not looked for.

**The lesson is not "sweep harder".** In every one of the three the
missed shape was *already written down somewhere* — in a door's rustdoc,
in the sweep's own blind-spot sentence. The cheap discipline is: after
writing the blind-spot sentence, go and look for the shapes it names,
and read the doc of any door the change promotes, because a door that
says who should call it has already enumerated the call sites you are
sweeping for.

The same shape hit the counts three times too — `all-census-idiom…`
filed at "three sites" and stale by one within the hour (really ≥9),
`dimension-all-has-readers…` filed at five (really eight), and this
program's own slate paragraph, which the orchestrator got wrong twice
before deleting the number and pointing at `work.py status`.

### Two dispatch errors, both the orchestrator's

Recorded because they are cheap to repeat and neither is in any brief.

- **Two lanes were dispatched into one checkout.** The D306 lane found
  `HEAD` moved under it mid-task, the S414 lane's uncommitted edits in
  the shared tree, and its own `git add` staging into their index; the
  shared scratchpad overwrote its draft PR body between writing and
  reading. Both PRs' file lists were verified clean and disjoint
  afterwards, but under merge-only rules this was one `git add -A` from
  an unrecoverable commit. **Every parallel lane gets its own
  `git worktree`.**
- **A live lane's worktree was removed.** The D306 lane mentioned
  `/home/user/d306-wt` as a loose end and the orchestrator deleted it
  while the lane was mid-fix-list, discarding six uncommitted edits. The
  branch was intact and pushed, so nothing merged was lost, and the lane
  rebuilt from context and re-executed every claim rather than carrying
  one on trust — but the rework was avoidable. **A path a lane names is
  not thereby free.**

### What the two lanes did that is worth copying

Both refused an instruction from their brief and were right to; that is
now four lanes out of four. The one to copy is D306's: told to take a
site and warned the swap might not be free, it found the claim it needed
**did not exist** — the door is documented against `eval_in_span` +
`deriv_in_span`, not against `ders_in_span`'s first two components, a
different code path — so it wrote the missing pin, ran it green, **then
falsified it with a one-ulp perturbation**, and only then moved the
site. S414's is the same instinct in a different key: its replacement
refusal message named a cause it could not reach through the real door,
so rather than leave the clause as a claim it pinned it with in-source
rows that return `Ok((NaN, NaN))` and `Ok((1e308, inf))` on `main`.

## Two rulings from Ev (2026-09-12)

**The independent rows stay in DOOR.** The open question was whether
`S114`, `grid-pitch`, `patherror`, `pathverb` and `step-adopt` should go
to FIX, raised because FIX closed three of DOOR's opening rows on the
day DOOR opened and the two charters read alike. Ev's answer: they stay
— FIX is a grab bag of small things and DOOR is the more coherent home.

**That does not by itself close the duplication risk**, which was the
real subject. The mechanism that failed was one-time — the cut sorted
two directories against the tree without reading the live slates of
programs already working the same ground — but the adjacency is
permanent. What replaces the re-home is a standing instruction, now on
this program's `keep_out`: **read FIX's slate for a row before
dispatching it**, and file nothing there that DOOR could carry. The
per-row liveness check that caught all five overtaken rows is the same
practice and stays.

**README-level ratification changes need Ev's sign-off** — and Ev's
instruction was not to record that here but to find where the approval
rule lives, **decide whether it was genuinely a gap**, and only then
clarify it there. Doing the check changed the answer's shape.

**It was a gap, narrowly, and the diagnosis is this program's own
subject.** The spirit was already unambiguous: `CLAUDE.md` calls crate
README pages design docs, says settled decisions are not re-litigated
without Ev, and `memories/orchestration-model.md` ends with *"when
unsure which kind a decision is, treat it as a fork"* — which covers
this and should have stopped the orchestrator. That miss is a reading
failure, not the document's.

But the letter was **a hand-written enumeration missing a member**. The
Git-workflow exception named *"PRs that ratify OPEN design questions"*
and `memories/`; retiring a SETTLED clause is neither.
`orchestration-model.md` named *"changes to ratified DESIGN.md
decisions"* — that file specifically, not the README pages the same
document defines as design docs. A list of two homes where the
principle has four, `docs/prompts/` being the other one missing. **The
rule that governs DOOR had the defect DOOR spent the day closing.**

Closed in `CLAUDE.md` by stating the test rather than the list — *"the
exception is text that binds future work rather than describing this
change"* — with the four homes as what it covers today and a sentence
saying a new home is covered the day it exists, not the day the line is
updated. Nothing was added to this program's `keep_out`: a rule that
binds every program does not live in one program's fence.

**PR #2387 therefore broke no rule that existed**, and the ruling row is
reframed to what is actually open: the paragraph it put in place of the
retired kind claims generally that *"a mirror claiming completeness has
an answer one crate over"*, and #2391's review found that false in a
live case — the viewport's `egui::PointerButton` mirror, whose declaring
crate cannot publish an `ALL`. The kind was vacated of rostered
instances, not falsified. Fixing that sentence is itself a README
ratification amendment, so it goes to Ev under the rule just written.
