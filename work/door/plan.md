# DOOR — the doors whose fix is already written (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3). Live state is
`work/door/log.md`'s tail and the item files beside this plan.

Branch prefix: **`door/`** — unit branches `door/<unit>-<slug>`.
Away-channel tag `(DOOR orchestrator)`. A/B ordinal band
**DOOR = 3800–3899**.

## Charter

A row belongs here on one test: **reading it tells you the diff.** Not
"the problem is understood" — the *fix* is written, in the row, and a
lane can land it without deciding anything first. Most of the cut's
rows are `E` on that test; the `M` rows are here because the fix is
still written, and what they add is a second file or one small call
(where a shared helper's home goes, what a refusal's signature becomes),
not a question.

**The test bites, and it bit on the first day.** One of the cut's rows
failed it — `viewer-cannot-author-a-part-node` named its files but not
what its new op takes — and left for CHROME. See **Opening
corrections**.

This is FIX's idea (the 2026-09-03 survey's third track) applied to what
has accumulated in `work/issues/` and `work/code-quality/` since. It
exists because *cheap* was not a visible property of the board: these
eleven rows sat interleaved with roll-ups and rulings, and nothing about
where they sat said one of them is an afternoon and another is a
quarter.

## Territory — none, and why

This program claims **no paths**. Every row sits on a file some live
program owns, which is why none of them were claimed: each is one small
thing in someone else's house. The rule that replaces a fence is the
one FIX ran on and it is strict:

- **One PR is one row.** A lane that finds itself editing a second row's
  file has left this program's posture and should say so rather than
  widen. **The one standing exception is the mirror class**, where two
  rows are the same defect in the same file and one projection closes
  both; it is ruled in **Order** and nowhere else, and a lane does not
  mint a second exception for itself.
- **Each PR draws its own fence and announces it** to the owning program
  in the PR body, naming the file and the owner.
- **A row that grows a design question stops being this program's.** It
  is re-homed — to the owner, or to the track whose subject it has
  turned into — rather than carried here at the wrong class.

## The slate

**The count is not written here, deliberately.** `python3 scripts/work.py
status --program door` derives it; a number in this paragraph would be a
hand-maintained census of the table below it, which is the defect this
program exists to close, and the orchestrator got it wrong once already
in the paragraph this one replaces.

Three of the cut's eleven rows are gone and two came in from VIEW — see
**Opening corrections** below. The mirror class's first PR (#2387,
`topo::BooleanOp::ALL`) then closed two rows and filed five, four of
them residue of the rows it closed, given files at the moment they were
disclosed rather than left in a merged PR body (`work/README.md`):
`mate-primitives-is-a-partial-mirror-with-no-growth-alarm`,
`viewport-pointer-buttons-mirror-a-toolkit-enum-by-hand`,
`surface-and-curve-kind-mirrors-have-a-tautological-guard`,
`vectorslot-all-has-no-reader`, and from the review
`all-census-idiom-forces-the-visit-not-the-update`.

**Those five are not on the table below.** The table is the cut's
slate and the two rows claimed from VIEW; the residue is on the board
through `work.py status`, which reads the directory. Anything else is a
second roster to keep in step by hand.

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `step-adopt-let-ok-iso-discards` | **E** | Two sites in one file; keep-with-comment or typed refusal, both options spelled out | `crates/step-import/src/adopt.rs:711`, `:877` (and its error enum if converting) |
| `S114` | **M** | **Re-classed E → M**: eighteen definitions across four crates, three in `src/` — not two in one crate. Measured on the row. | `crates/geom/src/curves.rs:1246`, `surfaces.rs:1263`, `crates/geom-core/src/spline/net.rs:365`, plus fifteen in `geom`/`geom-brep`/`sweep` tests. Home undecided: `geom-core` beside `Interval`, or `test-utils` (not a dev-dep of `geom` today). |
| `viewer-grid-pitch-nonfinite-fallback` | **M** | Small refusal-shaped signature change, but caller and tests ripple; unowned, fence drawn in the PR | `crates/viewer/src/datums.rs:159-163` (`grid_pitch` + its sole caller), `crates/viewer/tests/datum_draw.rs` |
| `patherror-display-renders-float-noise` | **M** | **Narrowed**: the `profile` half is done; the cross-crate half and the helper's home are what is left. | `ProfileError` and other crates' error `Display`s; helper home `crates/geom-core/src` beside `Real`. NOT `path.rs`'s arms — all 38 already go through `num`. |
| `viewer-pathverb-all-hand-written-seventeen` | **M** | The census half only; the hand-list half is closed in the tree. | `crates/viewer/src/forms.rs:145` (`PathVerb`), `crates/viewer/src/sketch.rs:147` (`PathStep`), reads `crates/profile/src/path/program.rs`'s `Verb::ALL` |

## Opening corrections (2026-09-11)

The cut's eleven were read against the tree before the first dispatch.
Three did not survive that reading as written, and the class the survivors
belong to pulled in two rows the cut had not seen.

- **`S190` was already fixed** and is closed, not dispatched. Both halves
  landed — `CensusUnsupported` carries `CensusSubject::FacePair` with an
  unordered `PartialEq` written rather than derived, `assembly.rs`
  resolves through `by_pair`, and the two-declaration fixture the row
  said no test could reach exists. The row's own file records the
  evidence. It was the plan's "first thing to look at", and the answer
  was that nothing was owed.
- **`viewer-cannot-author-a-part-node` went to CHROME.** It failed the
  charter test: it names four files but not what the `AddPart` op takes,
  and the seat and instance arguments are the work rather than a detail
  below it. CHROME owns `crates/viewer/src/*` and closed the sibling gap
  (`placed-union-has-no-session-op`, PR 1762), which is both the
  precedent and the argument.
- **`viewer-pathverb-all-hand-written-seventeen` lost half its premise.**
  `PathVerb::ALL` is projected by `vocabulary!` now, so the hand-written
  seventeen is gone; what survives is the kernel-mirror census, which is
  the class below. Its file carries the correction.
- **`run-on-whitespace-in-message-literals` was a duplicate** of a FIX
  row closed the same day, PR #2364 — 27 sites, of which this row named
  five and one of those pointed at a dissolved file. Closed, never
  dispatched. The guard question it carried is FIX's too, with a
  measured threshold this row's `rg` would have got wrong.
- **`patherror-display-renders-float-noise` is narrowed, not
  dispatched as written.** Its motivating example already renders
  correctly and all 38 arms in `path.rs` already go through the helper;
  the cross-crate half is what survives. A NEW defect at the other end
  of the range — the relative tolerance exceeding ε above a decimetre —
  was filed on FIX's slate rather than carried here.
- **Two rows came in from VIEW** — `hand-maintained-mirrors-of-a-kernel-enum-are-unforced`
  and `dimension-radio-row-is-an-unforced-mirror-of-a-kernel-enum` —
  claimed with Ev's direction in-chat and VIEW told. See the class rule
  in **Order**.

## Order

Any order for the seven rows that stand alone; there are no dependencies
between them, which is most of what makes the track worth having. Four
notes:

- **The mirror class is four rows and does NOT get four PRs.** The class
  head, `boolean-op-has-a-third-hand-written-complete-list`, and
  `dimension-radio-row-…` are one defect at three sites, and one
  publication closes more than one row: `topo::BooleanOp::ALL` retires
  `forms::BOOLEAN_OPS` **and** the `kernel_wire` copy, which is the head
  and the boolean row together. So the class lands as **two** PRs — one
  per kernel enum published (`BooleanOp` from `topo`, `Dimension` from
  `editor-core`) — and each says in its body which rows it closes.
  **This is a deliberate departure from one-PR-one-row**, ruled by the
  orchestrator at the claim: the posture exists so a lane does not widen
  into a second row's file, and here the second row IS the same file and
  the same diff. Splitting them would mean landing half a projection.
  `viewer-pathverb-…` is the class's fourth row and is NOT in either PR:
  the GUI legitimately narrows the kernel's verb vocabulary, so its
  answer is a census and not a projection.
- **The three viewer rows go early.** This program's `keep_out` binds
  them to CHROME's sequencing rule, and that rule has already fired —
  `viewer-session-god-module-split` closed 2026-09-04 and CHROME's units
  are all answered — so the rows are free rather than blocked. They go
  first anyway, because CHROME's residue is parked on the same files.
- **`patherror-display-renders-float-noise` corrects the helper before it
  propagates it.** See **Review posture**.

## Review posture

One review per unit against `docs/prompts/reviewer-style-lane.md`, no
A/B row — the FIX and CHROME posture. **Light by default; full where a
row has real risk of being wrong** (Ev, 2026-09-11, in-chat).

The four that take a full correctness review beside the style lane, and
why each:

- **`viewer-grid-pitch-nonfinite-fallback`** — the signature change
  ripples to the sole caller and the tests, and the failure mode the row
  describes is a hang (~1e323 grid lines per direction), not a wrong
  picture.
- **`patherror-display-renders-float-noise`** — it moves every refusal
  sentence a user reads, so assertions move with it, and it changes a
  rounding point. Re-baselining what moves is not a cost to weigh
  (`memories/output-stability-as-justification.md`); getting the
  rounding point wrong is.
- **`S414`** — it changes which imports refuse. A refusal that starts
  firing is a behaviour change at the door even when the row calls it
  diagnostics-quality.
- **The mirror class's two PRs** — a new public item on a kernel type,
  and `kernel_wire`'s `untag` table is a persisted wire format.

The other rows take the style lane alone.

**`patherror-display` corrects the helper before it propagates it.**
`crates/profile/src/path.rs:1411`'s `num` rounds at `1e-9 * x.abs()` —
a *relative* 1e-9, with nothing principled behind the constant. D4's ε
is ~1e-9 m **absolute**, so at metre scale that rounding point sits
exactly on ε_precision and above metre scale it is coarser (1e-6 m at
km scale, a thousand ε). These sentences mostly report margins *against*
ε, which makes that the worst band to round.

**The rounding point is the finer of two grids, and it needs both.**

- **An absolute floor of `DEFAULT_EPS / 10` = 1e-10 m** — one decade
  below the ratified ε. Above ε the kernel cannot distinguish finer, so
  digits past this grid are noise a reader cannot act on. (Ev's
  proposal, 2026-09-11 in-chat.)
- **A relative arm, so a sub-ε payload keeps its own magnitude.** The
  absolute grid alone would destroy the dominant call-site shape: the
  `margin` of `JunctionTangent`, `JunctionCusp` and their siblings
  (`path.rs:1460`, `:1473`, `:1486`, `:1501`, `:1590`) is **below the
  threshold by construction** — that is what makes the junction tangent
  — so a 1e-10 grid renders the only number in the sentence as `0 m`,
  which is both false and useless. The helper's own doc already argues
  this and is right: *"a picometre margin is rounded to nine significant
  figures of a picometre, never to the nearest nanometre and never to
  `0`."* What it gets wrong is only the constant it then picks.

So `tol = (DEFAULT_EPS * 0.1).min(x.abs() * RELATIVE)`, the finer grid
winning at each magnitude. `RELATIVE` replaces today's 1e-9 and is the
noise band, not a readability knob — the row's own examples are 1 ULP
and 0.5 ULP off their clean forms, so a few ULP is the defensible floor;
the lane picks it, argues it at the site, and is constrained by the
pinned rows in `num_renders_a_sub_nanometre_payload_at_its_own_magnitude`
(`5e-324` needs its own subnormal arm).

**Use the compile-time `DEFAULT_EPS`, not `Tolerance::eps()`** — one
call made by the orchestrator, and the reason is the gate. ε is a live
process value and a code-tier run gates every point of {default, 1e-6,
1e-12}; reading it here would make every rendered refusal string a
function of process configuration and every string assertion in the tree
eps-sensitive across three eps rows. The grid is a **display** choice
stated once against the ratified default, which is what keeps this row
an `M` instead of an `H`. A lane that finds this wrong says so rather
than quietly reading the live tolerance.

**Ordering rule 5 applies hardest here**: the fix mints a fresh instance
of the defect it closes, and on rows this small the reviewer is the only
one who has ever caught it.

## How the class column is read

`E` / `M` / `H` is a **dispatch estimate**, made on 2026-09-11 by reading
each row against the tree, and it is the axis this program's order runs
on. It is not a verdict on the finding and it is not in any header: no
field carries it, `work.py` does not parse it, and this table is the only
place it lives. A lane that finds the estimate wrong says so in its PR
and this table is corrected in the same PR.

- **E** — the fix is written in the row or obvious from it: one or a few
  files, no design question, no ruling, small diff.
- **M** — multi-file, or a small design call (where a shared home lives,
  what a door looks like), or a census or instrument to build first.
- **H** — cross-cutting, numeric or algorithmic, gated on a ruling, or
  spanning several programs' territory.

The cut that opened this program is `docs/WORK-TRACKS-2026-09.md`
addendum 3; it is a survey, and this plan supersedes it as the charter.
