# META log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/meta/plan.md`. A/B band 2800–2899
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opened (2026-09-04)

Opened on Ev's direction (in-chat, 2026-09-04: a meta/tracker track,
"unless it conflicts with CIW I'd rather make it a new track anyway")
after a re-read of `work/issues/` against every open program's `paths`.

**Checked for the conflict first, and there is none.** CIW's `paths`
enumerate `.github/workflows/*`, `local-scripts/*`, the demo shell and
Python and thirteen named `scripts/` files; `scripts/work.py`,
`scripts/ci-pin.py`, `docs/prompts/*`, `docs/MODEL-AB-LOG.md` and
`docs/DOC-LEDGER.md` are in none of them. CIW's `keep_out` already
cedes `scripts/work.py` in prose — "the tracker's own, changes only
with `work/README.md`" — naming an owner that did not exist until now.
The one touching surface is `implementer-discipline.md` §2 (what a run
gates), and it is written into this program's `keep_out` as **CIW's to
amend without waiting on this program**, so the fence reads the same
from both sides.

Three items re-homed at opening by header edit and `git mv`, ids
unchanged, plus custody of two cross-program registers:
`territory-cannot-see-a-path-two-programs-both-claim` (the opener),
`ab-log-v6-stream-is-past-its-stopping-rule-unadjudicated` (`[ev]`) and
`stale-track-t-citations-in-fillet-and-cert` (a routing record), with
`m6-carried-items-register` and
`decide-flagged-dimensional-debt-inventory` as registers this program
keeps accurate and does not execute.

A fourth, `fillet-specs-require-a-narrowing-ci-config`, was on the
sweep's first draft of this slate and came off it at the merge with
main: CIW's `delete-config-trailer` unit had closed it hours earlier by
deleting the `CI-Config:` trailer path itself. It stays closed in
`work/issues/`; the plan records why the class it belonged to is still
this program's.

**The overdue thing, stated plainly at opening so it is not buried in
the plan**: the v6 dual stream has run about nine times past its own
pre-registered stopping rule, with the readout the rule existed to
produce never taken. That is an `[ev]` PR on day one, in parallel with
the opener and not behind it.

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## First pass was a correction of its own opening (2026-09-05)

The opening commit's tree-wide double-claim census — the measurement
that was meant to settle the lint rule's shape — **miscounted three of
its own figures**: 17 program pairs for 16, four `keep_out`-recorded
overlaps for three, eleven `*/tests/*` pairs for nine. It also listed
DOCM/MSOLVE as a recorded overlap when it is recorded on **one side
only**. Corrected on the item by re-deriving against merged main at
`a2bcab785` rather than by patching the numbers, with the method
written down so the next run can be diffed against this one.

Two things came out of the correction that are worth more than the
numbers were:

- **DOCM/MSOLVE is the live one-sided record.** MSOLVE's `keep_out`
  names DOCM at length and says the overlap is "announced, not
  assumed"; DOCM's does not name MSOLVE at all. Neither program did
  anything wrong — MSOLVE opened a day ago and followed the rule — and
  that is the point: a one-sided record is invisible from the side that
  was there first, which is what the lint would have asked DOCM for on
  the same day. It is now the worked example inside item (1).
- **The lint's rule is settled by the data**: an overlap is an error
  unless BOTH programs' `keep_out` name the other. It passes three
  pairs today and fires on thirteen, nine of them the `*/tests/*`
  family, which wants the Track W seam encoded once rather than nine
  `keep_out` lines. That is the one question left inside the item.

TOPO's opening count was wrong the same way and in the same commit
(47 unowned `crates/topo/src` files for 55, and "~25 remaining" for
35 — a figure read off one line of a directory-grouped scan that
dropped two subdirectories). Corrected in `work/topo/plan.md`,
`work/topo/log.md` and the tracks addendum.

**The pattern is the program's own subject.** Both errors are
hand-run censuses published as measurements with nothing that reads
them back, which is the class this program exists to close — and the
mechanised version of the first one is exactly what item (1) builds.
Recorded here rather than quietly fixed, because a census that has been
wrong once is a census whose next reader should know it.

## An inbound edit to `docs/prompts/implementer-discipline.md` (2026-09-09, CIW, PR 2263)

CIW added one bullet to §2's "When you do run locally" list, under this
program's standing clause for §2 run-facts. Recorded here so the edit has
a record on META's side rather than only in CIW's tracker.

**What it says:** `cargo clippy --workspace --all-targets` compiles
nothing under the roots `Cargo.toml` excludes (`benches`, `demos`,
`tools`, `interval-transcendentals`); `demos/tour` and `demos/wild` are
ordinary consumers of the public API, so a signature change breaks them
the way it breaks a user; those two and `tools/*` are gated on every
code-tier run and by `ci-local.sh`, while `benches` (rustfmt only in the
PR gate, clippy in the nightly with no local mirror) and
`interval-transcendentals` (clippy only when the filter buys that job)
are weaker than that.

**It carries no count on purpose**, and tells the reader to derive the
list with `scripts/doc-gate.sh --print-roots`. That is this program's
own subject twice over: the bullet's first draft asserted "the hosted
gate and `ci-local.sh` both cover every root", which is false for two of
them, and named "five" roots where the derivation says seven. The same
PR found the count stale in five places inside CIW's own fence —
`ci.yml`'s cache-scope paragraph (six/seven), `ci-local.sh`'s rustdoc
note, and three lines of `scripts/doc-gate.sh` including a selftest
failure message — all of which have said "six" since before
`tools/tess-meter` landed. Every one now points at the derivation
instead of carrying a number. A prompt read at the start of every lane
is the last place a hand-run census belongs.

META owns the wording; CIW invites the rewrite.


## Four items closed in one PR — the program's first dispatch (2026-09-11)

Opened 2026-09-04; **no unit had ever been cut and no `meta/` branch
ever existed** until this one. This PR takes the four executable rows
on the slate together, because each is a small self-contained edit to
`scripts/work.py` or to a document this program owns, and because three
of the four are instruments that detect the same class — a tracker fact
nothing reads back.

**Branch: `claude/charming-archimedes-hyvb4v`, not `meta/…`.** Set by
the session harness rather than by the #396 convention, the way FIX
recorded the same deviation at its own pickup (`work/fix/log.md`,
2026-09-04). `meta/orchestrator` remains unused.

### What landed

1. **`territory-cannot-see-a-path-two-programs-both-claim`** (the
   opener) — both candidates. `territory` reports a path the branch's
   own program claims that another open program claims too, worded as
   a double claim rather than a crossing; and `lint` mechanises the
   at-rest census, naming every pair whose two `keep_out`s do not both
   name the other.
2. **`parked-on-an-int-is-invisible-to-the-fired-trigger-rule`** —
   shape (a): a number in `blocked_on` resolves through `github:` and
   reaches the fired-trigger rule.
3. **`work-set-accepts-a-scalar-for-a-list-field`** — both fixes: `set`
   coerces a scalar into a one-element list, and `lint` diagnoses a
   hand-written one instead of dying on it.
4. **`perf-plan-is-cited-by-twenty-nine-files-and-absent-from-tree-and-ledger`**
   — closed against a measurement that overturned it (below).

### The decision taken unilaterally, and it is the same one twice

**Both new detectors WARN where the plan and the item asked for an
error.** `plan.md` specified "a lint rule that errors on an unrecorded
double claim"; the int item's shape (a) said "applies the fired-trigger
rule", whose unblocked shape is an error. Neither could ship that way:

- The double-claim rule names **ten unrecorded pairs** on landing day.
  Every fix is a `keep_out` clause in another program's `program.md`.
- The int resolution found **two genuinely fired triggers** on its
  first run, `S190` and `S79`, both `work/code-quality/`'s.

In each case the error would have reddened `main` for rows this program
is forbidden to edit — by `work/README.md`'s one-file-one-item rule and
by this program's own `keep_out`, which says a stale row in another
program's slate is **routed, never fixed across the fence**. A check
whose first act is to break every program's CI for a change only its
owners can make would have been reverted or softened within the day,
and a softened check is worse than one that shipped honest.

So the rule is the plan's rule, in its warning phase, with the error
flip named and filed rather than assumed:
`double-claim-lint-rule-waits-on-the-tests-seam`. Nine of the ten pairs
are the `*/tests/*` family, and whether the check learns that seam once
or nine programs write it down is a change to what a `keep_out` means —
Ev's, not a sequencing call, with a recommendation on the item.

The int case gets a second, narrower reason written into the contract:
an id in `blocked_on` NAMES a row and an int MATCHES one through a
field the author did not write as a reference. A true inference is
still an inference, and an inference does not red `main`. Naming the
item promotes the row back under the error, which is the end state.

### The measurement that overturned an item

`docs/PERF-PLAN.md` was **renamed to `work/perf/plan.md` on 2026-09-03
with zero content change** (`4916f90c`, PR #1619) — not deleted. All
three claims in the item's title were false: 5 files cite the dead path
and not 29 (the other 27 name the document, which still carries that
name as its own title); it is in the tree; and the ledger does record
the move, in sweep 4's "Moved, not deleted". The lane that filed it
followed the ledger's own recovery recipe, `git log --diff-filter=D`,
which is **empty for a renamed file** — so the wrong conclusion was the
one its tools pointed at. The ledger now names the moved files
individually and tells the next reader to run `--all --full-history`
first.

This is the second time in this program's short life that a hand-run
census published as a measurement turned out wrong (the first was its
own opening count, 2026-09-05), and the second time the answer was to
mechanise it rather than patch the number. Both instruments in this PR
reprint their census on every `work.py lint`.

### Routed, not fixed (the fence, working)

`S190` and `S79` (code-quality, parked on fired numbers); `github: 1374`
claimed by both a CHROME row and a DOCM row, and `github: 1607` by both
a CIW row and a `work/issues/` row; the `docm`+`msolve` one-sided
`keep_out`, still one-sided six days after it was named; the `docm`+`lib`,
`chrome`+`tcost` and `tcost`+`view` second sides; and five `PERF-PLAN`
path citations owned by S-MESH/S-TCOST, PERF and two records. Each is
named on a closed item's `## Closed` section AND on a live item, because
`work/README.md` is explicit that a residue disclosed in prose is
invisible to the re-homing sweep.

### Slate after this

Two registers custodied (`m6-carried-items-register`,
`decide-flagged-dimensional-debt-inventory`), one routing record
(`stale-track-t-citations-in-fillet-and-cert`), the two residues filed
here, and **the overdue one**:
`ab-log-v6-stream-is-past-its-stopping-rule-unadjudicated`, which
`plan.md` said on day one was "an `[ev]` PR on day one, in parallel with
the opener and not behind it". It is now seven days behind it. That PR
is the next action on this program and nothing else should precede it.
