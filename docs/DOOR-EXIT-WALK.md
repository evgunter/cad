# DOOR — exit walk

STATUS: **RATIFIED** (Ev, in-chat 2026-09-21, before the walk was cut:
*"can you close `door`? it just had its last item close"* — so this
walk merges with the exit sweep and is DOOR's done-state of record. A
program is closed when its exit walk is ratified; this document is
deleted with the tracker directory in the sweep that carries it and is
recoverable at the SHA `docs/DOC-LEDGER.md` sweep 19 names).

DOOR = the doors whose fix is already written (`work/door/plan.md`;
opened 2026-09-11 in the eleven-program cut of that day,
`docs/WORK-TRACKS-2026-09.md` addendum 3; `paths` = **none**; A/B band
**3800–3899**).

**Twenty-two rows closed from the directory; twenty-seven passed
through it.** Eighteen issues, two units (`S190`, `D306`) and two
rulings. Five rows left rather than closed: one on the program's first
day (`viewer-cannot-author-a-part-node` → CHROME, the charter test
biting; the row sits on AUTHOR's slate today) and four in the design-free sweep of 2026-09-20. The band
3800–3899 was claimed at the cut and **never drew an ordinal** — the
program ran style reviews with no A/B row for its whole life, so
`docs/MODEL-AB-LOG.md` carries the band and no DOOR row, as FIX's and
VIEW's do. The directory leaves whole, with every row in it closed and
every disclosed residue already filed on the slate that owns it.

## The walk

Criteria verbatim from `program.md` and `plan.md`'s Charter, Territory,
Review posture and class-column sections. Dispositions: MET /
MET-WITH-RECORDED-HONESTY / REFUTED / CARRIED (named owner).

| # | Criterion | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | "**One-PR rows whose body already contains the fix**: no design question, no ruling, no census to build first. The test is **reading it tells you the diff**." | **MET-WITH-RECORDED-HONESTY** | True of what the program landed. Not true of what it held, and the honest number is that **the charter test was applied to this slate four separate times and bit every time**: once on day one (`viewer-cannot-author-a-part-node` names four files and never says what the `AddPart` op takes — a file list is not a fix) and three more on 2026-09-20. Worse, a row can pass the test on the day it is filed and fail it a week later: `patherror-display-renders-float-noise` arrived with its fix written and grew a design question when a second crate started consuming `path::num`. The test is about a row's CURRENT relationship to the tree, and nothing in the tracker re-runs it. |
| 2 | "**A row that grows a design question stops being this program's**, immediately and without exception" — retiring the charter's `M` clause, which had admitted rows carrying *"a small design call"*. "**It carries no design decisions**" (Ev, in chat, 2026-09-20). | **MET, and it is what made the last wave work** | Four rows left in one sitting — `S114` and `patherror-display-renders-float-noise` to PROPS, `all-census-idiom-forces-the-visit-not-the-update` to CENSUS, `unit-symbol-proptest-generators-under-cover-with-no-file` to TINT — each with a `## Re-homed` section and a note on the receiving log. **The structural reason is the one worth keeping**: FIX owned three files and could rule on its own ground, so its sweep left three rows behind; DOOR owns none, so the rule here has no exception and every decision leaves. The clause being retired had contradicted the program's own rule two paragraphs below it since the day it was written, and both rows it was written for turned out to ask the same question (where a shared thing lives in a `geom-core` file PROPS owns). |
| 3 | "**One PR is one row.** A lane that finds itself editing a second row's file has left this program's posture." | **MET, with one ruled exception that closed** | The mirror class took two PRs for four rows (#2387, #2391) — one per kernel enum published, because the second row was the same file and the same diff and splitting would have meant landing half a projection. Ruled in `plan.md`'s Order and nowhere else, so a lane could not mint a second exception for itself; the class closed on 2026-09-11 and the exception with it. Every later unit was one row. |
| 4 | "**Each PR draws its own fence and announces it** to the owning program... and takes the owner from `scripts/work.py territory`, **never from a row's own prose**." | **MET at the instrument, REFUTED at the prose, repeatedly** | This is DOOR's sharpest finding and it reads across the tracker. The instrument was right every time a lane ran it and the prose was wrong every time anyone trusted it. Measured: this program's own `keep_out` asserted *"`crates/editor-core/*` is DOCM's"* and #2391 proved it false (DOCM's `paths` was a specific file list; `expr.rs` was on nobody's). **Three rows named DOCM as their owner for a week after DOCM had left the tracker** (closed 2026-09-13, `docs/DOC-LEDGER.md` sweep 14) — territory says `node.rs` and `eval/parts.rs` are EDIT's and `mate/member.rs` is MSOLVE's. And the `keep_out` clause a reader would consult for the answer now *leads* with the instruction not to consult it. One PR's territory run named eight foreign paths where its brief predicted three. |
| 5 | "**Reviews are light by default; full where a row has real risk of being wrong**" (Ev, 2026-09-11, in-chat), one review per unit against `docs/prompts/reviewer-style-lane.md`. | **MET, and it paid on exactly the rows the posture predicted** | `viewer-grid-pitch-nonfinite-fallback` is the case that justifies the whole posture: row wrong about severity → first review found the invariant covered two of four datum kinds → the 148-line restructure that fixed that was itself reviewed, and the second pass found that `datums.rs`'s inclusive `for i in 0..=count` still draws one line at `t = inf`, because `inf - inf` is `NaN` and `NaN as usize` is `0`. **The PR's own census had examined that expression and cleared it.** The last wave ran four lanes with the orchestrator adjudicating from each diff and no review lane at all, which was right for two doc sentences, one field, one literal and one deletion. |
| 6 | "**Ordering rule 5**: where a fix closes a structural finding, check whether it mints a fresh instance of the defect it closes." | **MET, and it fired twice — differently each time** | #2387's fix for a hand-written list **minted a hand-written census**, and the PR body asserted *"No new census"* two lines after admitting it created one; the reviewer caught it. Executed, the census's hole was plain: add a variant, copy the arm the failing match asks for, and the row passes green with the variant absent from `ALL`. #2391's census was honest, and went stale a different way — the class row was filed at *"three sites"*, #2391 made it four within the hour, and the real population measured **at least nine**. The generalisation that survives is the one the log states: **reading a guard is not running it.** Two lanes and an orchestrator read that census and approved it; thirty seconds of `rustc` falsified it. |
| 7 | "**No A/B row on any unit of this program**" (Ev, in-chat 2026-09-11). | **MET** | Absolute for the program's life. No ordinal in 3800–3899 was ever drawn; the band is claimed and empty. |
| 8 | The class column: `E` the fix is written, `M` multi-file or a census first, `H` *"not this program's at all"*. A dispatch estimate, in no header, parsed by nothing. | **MET-WITH-RECORDED-HONESTY** | No `H` row was ever dispatched from here and the `M` rows all left or were narrowed, so the column did the job it was for. The honesty is that it was an estimate made by reading a row against the tree, and **the rows' own numbers were stale nearly every time a lane re-derived them**: `.slots()` counted 36 and measured 41; `Dimension::ALL`'s readers were filed at five and measured eight against a population of ten; `datum-axis`'s *"third and last copy"* became a four-pattern measurement. A cost estimate over a stale census is an estimate of the wrong work. |
| 9 | "**The count is not written here, deliberately**" — the slate's size is `work.py status --program door`, never a paragraph. | **MET, after the defect was committed twice** | The plan says a hand-maintained census of the table below it is *"the defect this program exists to close"*, and that sentence exists because the orchestrator got the number wrong twice before deleting it and pointing at the instrument. |
| 10 | The FIX adjacency (Ev, 2026-09-12: the rows stay in DOOR, *"FIX is a grab bag of small things and DOOR is the more coherent home"*), with the standing instruction to read FIX's slate before dispatching any row. | **CARRIED, and now moot** | Held and never reopened; FIX's own `plan.md` was corrected on 2026-09-20 for raising it again. FIX left the tracker on 2026-09-21 (sweep 18) and DOOR follows it on the same day and on a ruling that reads across, so the adjacency has no live side. The duplication risk the ruling was really about was answered by practice rather than by routing: **the per-row liveness check caught five of the opening eleven rows as already-overtaken work**, three by FIX and one by VIEW on the day DOOR opened, and `S190` by reading the tree. |
| 11 | "**The slate empties**" — the exit condition. | **MET** | Emptied 2026-09-21 with the four-row wave: **#2984** (`axis_datum`'s `expected:` comes from `phrase::DATUM_AXIS`), **#2985** (the placer docs say what the placers accept and yield), **#2986** (`PartFault::PartProduct` carries the class beside the sentence), **#2989** (`VectorSlot::slots` deleted). Each on its own green hosted head, full matrix, no narrowing. Nothing is carried into this sweep: every row in the directory is closed, and every residue disclosed in a closed row was given its own file on the slate that owns the ground — `work/exch/torus-rim-mint-abandons-a-half-applied-split`, `work/exch/arc-rim-gate-reports-a-degenerate-carrier-as-an-infinite-residual`, `work/scalar/curve3-eval-and-deriv-at-one-t-run-two-basis-passes`, `work/view/viewport-adapter-drops-part-of-two-toolkit-values`, `work/wire/node-operand-has-one-consumer-where-axis-datum-is-the-same-door`, and four on CHROME's slate. |

## What the program actually produced, beyond the twenty-two rows

**A change to the standing discipline every lane is handed.** DOOR's
`lane-cross-program-filing-two-binding-docs-conflict` found
`docs/prompts/implementer-discipline.md` §6 and `work/README.md` giving
a lane opposite instructions about filing on another program's slate,
and Ev ruled for the README on PR #2421: *"the prompt is wrong and
should be updated to agree with the README to have no reservations
about filing directly to other programs."* §6 now directs lanes to file
on the owner's slate in the same PR, without permission and without
routing. The orchestrator's proposal lost outright, which is the part
worth recording.

**A gap in the approval rule, found by the program it governs.**
`readme-ratification-amendments-need-ev` began as a confession (#2387
retired a ratified README kind without Ev) and ended as a diagnosis:
the git-workflow exception was **a hand-written enumeration missing a
member** — it named open design questions and `memories/`, where
retiring a settled clause in a crate README is neither, and
`docs/prompts/` was missing too. `CLAUDE.md` now states the test —
*"text that binds future work rather than describing this change"* —
with the four homes as what it covers today and a sentence saying a new
home is covered the day it exists, not the day the line is updated.
**The rule that governs DOOR had the exact defect DOOR spent the day
closing**, and nothing was added to this program's `keep_out`, because
a rule binding every program does not live in one program's fence.

**Three practices, each measured rather than asserted.**

- **Route from the instrument, never from a fence read in prose.**
  Criterion 4's evidence. A program that spans fences by construction
  lives or dies on this, and it is now FIX's exit walk and DOOR's
  saying the same thing from two different territories.
- **A negative result by grep is a claim about the pattern; a negative
  result by deletion is a fact about the tree.** #2989's lane deleted
  each sibling member of `VectorSlot` in turn and read the compiler —
  `slot` 2 errors, `label` 3, `dimension` 2 — turning "one unread
  member" from a reading into a measurement. The same shape is why
  *"reading a guard is not running it"* (criterion 6) is the program's
  most reusable sentence.
- **After writing a sweep's blind-spot sentence, go and look for the
  shapes it names.** A regex-shaped sweep missed an instance inside its
  own PR's fence three units running, and in every one of the three the
  missed shape was **already written down** — in a door's rustdoc, or
  in the sweep's own blind-spot sentence. A door whose docs say who
  should call it has already enumerated the call sites you are sweeping
  for.

**Lanes corrected the dispatching seat six times, and that is the
review posture working in the direction nobody plans for.** Four in the
program's first two days, two in the last wave: #2985's brief called
both placers *"shape-preserving over `Instances`"* (`wire_pattern`
returns `Instances` unconditionally — N bodies for a one-body master,
so the sentence is true of `Transform` and false of `Pattern`, and
writing it twice would have put a false sentence in a declaration
`node_value_kind` reasons from), and #2989's brief said to delete a
re-export that does not exist (the enum's is load-bearing, proved by a
revert-after-probe: `E0432` from the viewer).

**And two dispatch errors of the orchestrator's, recorded because they
are cheap to repeat and are in no brief**: two lanes were dispatched
into one checkout (shared index, shared scratchpad, one `git add -A`
from an unrecoverable commit under merge-only rules) — **every parallel
lane gets its own `git worktree`**; and a live lane's worktree was
deleted because the lane had mentioned it as a loose end — **a path a
lane names is not thereby free**.

## Why the program does not get a successor

The same reason FIX does not, sharpened by the fact that DOOR owned no
ground at all. Ev closed FIX hours earlier on 2026-09-21: *"most of
those were either mis-filed or should've been done as drive-by fixes."*
Every row DOOR ever held was on another program's file by construction,
and the last wave's four were two doc sentences, one field, one literal
and one deletion in files EDIT and MSOLVE own. Routing a one-line fix on
someone else's file through a separate program's slate, dispatch, lane,
PR and seam announcement costs more than the fix.

What was genuinely good here — the liveness check before dispatch, the
fence instrument, the pin question, ordering rule 5 — are practices, and
practices do not need a directory. `work/README.md` already carries the
rule that replaces this program: **file the row on the slate of the
program whose ground it lands on, the day it is found; take it in
passing if you are already in the file; and let `work/issues/` be the
last resort it was always meant to be.**
