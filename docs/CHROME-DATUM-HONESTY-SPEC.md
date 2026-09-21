# CHROME-DATUM-HONESTY — one finiteness door in `datums.rs`, and a cap that says it fired

**Rows** (read both in full first; each carries corrections to itself):
`work/chrome/four-spellings-of-one-finiteness-predicate-in-datums-rs`
(P1, E) and `work/chrome/max-grid-lines-truncates-a-ruling-and-calls-
it-one` (P3, E).

**Branch** `chrome/datum-honesty`. **Never merge; I merge.**

**Standing discipline**: `docs/prompts/implementer-discipline.md`, in
full, before you start. It binds you alongside this spec.

## Why one unit

Both rows are in `crates/viewer/src/datums.rs` and both are the same
class — *a number the module could not honestly compute, handed back
in the shape of one it did*. Splitting them puts two lanes in one
file. That is the whole reason; it is not a claim they are one defect.

## Two corrections to the rows, measured against the tree today

**The predicate row's census of four is now EIGHT.** The row lists
four sites (`metres_per_pixel_at`, `screen_metres_at`, `grid_pitch`,
`rule_patch`'s `rule` closure). `grep -n is_finite
crates/viewer/src/datums.rs` on `origin/main` returns eight: lines
~176, ~205, ~298, ~816, ~840, ~854, ~899, ~1093. The row PREDICTED
this (*"the next lane to touch this file will add a fifth unless there
is a door to route through"*) and it happened. **Re-take the census
yourself; do not trust these eight either.**

**`MAX_GRID_LINES` is 512, not 96.** The second row says 96 and builds
its reachability argument on it (*"about 26 lines on a 1280-pixel
window, so the cap is dead at every ordinary view"*). The constant on
`origin/main` is `512` (`datums.rs`, near the other patch constants).
The DEFECT is unchanged — a truncated ruling returned in the shape of
a complete one — but the row's arithmetic about how dead the cap is
was taken at a different number. Re-derive it or drop it; do not
repeat it.

## Half 1 — the finiteness question(s)

The eight sites are **not all one question**, and saying which are
which is the work:

- *finite AND positive* on a scale, a span, a pitch — two of these are
  the same question on the same quantity one call apart, and the third
  is its negation spelled the other way round;
- *finite* on coordinates and lattice bounds, where positivity is
  meaningless;
- one site raises a TYPED refusal (`CameraError::NotFinite`) rather
  than returning `None`, and it names WHICH side failed.

**Decide and say**: how many doors this file needs (one, two, or two
plus the typed one), and what each is called. The row names three
candidate outcomes including *"argue that two of the four are
different questions and say which"* — that is a legitimate answer for
any subset, provided you say it at the code and not only in the PR.

`crates/geom-core/src/real.rs`'s `is_finite_length` is the named
prior art. **Reading it is a read, not an edit.** The row already
argues the fit is not obvious (generic over `Real`, asks through the
scalar poison channel, contract is about a DECIDED length with a norm
witness; `datums.rs` is display code over plain `f64`). Reach your own
conclusion and state it; "does not fit, here is why" written beside
the door you build instead is a complete answer.

## Half 2 — `MAX_GRID_LINES`

`rule_patch`'s `rule` closure does
`((last - first) as usize).saturating_add(1).min(MAX_GRID_LINES)` and
returns a ruling the caller cannot tell from a complete one. The row
offers three dispositions — refuse, shrink the PATCH to what the cap
covers (so the drawing is complete for a smaller patch, which is a
true statement), or keep it and argue it at the site the way
`grid_pitch`'s rustdoc argues its refusal.

**My recommendation, which you may overturn with a reason: shrink the
patch.** It is the only one of the three that leaves the caller
holding a true statement AND a grid. If you overturn it, say why in
the PR.

**Nothing asserts the cap firing today.** `crates/viewer/tests/
datum_draw.rs` drives no patch past the cap in any direction. Whatever
you choose owes a row that goes red if the behaviour changes back —
and say which mutation you used to prove that row CAN go red.

## Sweep obligation

Both halves are class fixes, so §5 binds hard: grep for the SHAPE, not
the symbol, put the hit list and its disposition in the PR body one
line per hit, and **state what your pattern could not match**. `8.0`
does not find `8f64`; `is_finite` does not find `!(x - x == 0.0)` or a
`matches!` on a classification.

**The trap this program has hit repeatedly** (`docs/prompts/reviewer-
style-lane.md` §1, last bullet): a lane closing a duplication mints a
fresh one. You are about to give a predicate a home. Before you push,
check whether your own diff left a second spelling behind. Naming this
in a PR body has never once prevented it.

## Scope

**In**: `crates/viewer/src/datums.rs`, `crates/viewer/tests/
datum_draw.rs`, and a new test file under `crates/viewer/tests/` if
you need one.

**Out**: everything else. `datums.rs` is claimed by CHROME, VGEOM,
VIEW and (since 2026-09-21) AUTHOR — double claims, legitimate, and
what is owed is **awareness while a lane is live**: run
`python3 scripts/work.py territory --base main` on your branch and put
what it printed in the PR. AUTHOR's live lane is `author/profile-
frame` (AUTH-3) and its spec puts `datums.rs` outside its scope, so
you should not collide; merge `origin/main` before you push anyway.

A finding outside this fence gets a ROW under implementer-discipline
§6 — on the slate of the program whose ground it lands on, in this
same PR — not a fix and not a line in the PR body.

## Verification

Hosted CI is the verification of record: expect **twelve `test (…)`**
and **five `k-lint (gate, …)`** jobs, confirm the run's head SHA is
your branch head, and read the STEP for any row whose job name does
not name it. Poll the run in the FOREGROUND until it concludes and
report in the same turn — never end a turn with a wait armed.

Locally, as an iteration tool only: `cargo fmt --all --check`,
`cargo clippy -p viewer --all-targets -- -D warnings` and again
`--features app`, `python3 scripts/work.py lint`.

`CARGO_TARGET_DIR=/home/user/chrome-datums-target`, exported on EVERY
cargo invocation, **outside the worktree**. Scratch files in
`~/.local/share/cad-work/chrome-datums/` — never the session
scratchpad, which is shared between lanes and has already caused one
lane to read another's output.

## Deliverable

A PR titled `CHROME: one finiteness door in datums.rs, and a grid cap
that says it fired`, body carrying: your re-taken census, the design
call on how many doors and why, the `MAX_GRID_LINES` disposition and
its argument, the sweep hit list with dispositions and blind spots,
the territory output, the mutations proving your new rows can go red,
and any §6 rows you filed. Update both item files' bodies with what
you measured. **Report to me; do not merge.**
