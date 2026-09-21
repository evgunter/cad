# CHROME-ONE-NUMBER — one home for a number the viewer reads twice

**Rows** (read both in full first):
`work/chrome/bounds-reading-respells-the-panels-one-divide` (P1) and
`work/chrome/display-budget-rows-restate-three-private-constants`
(P1, E).

**Branch** `chrome/one-number-one-home`. **Never merge; I merge.**

**Standing discipline**: `docs/prompts/implementer-discipline.md`, in
full, before you start. It binds you alongside this spec.

## Why one unit

Both rows are the same defect in two directions: a number with one
home, read somewhere else by hand. One is a conversion re-spelled in
`src`; the other is three constants re-spelled as literals in a test
suite. They share a remedy shape (`Camera::pitch_limit()`'s accessor
argument) and they do not share a file, so one lane can do both
without two lanes in one widget.

## Half 1 — the panel's one divide, spelled twice

`BoundsReading::wording` (`crates/viewer/src/bounds.rs`, ~`:284`)
opens with

```rust
let written = crate::readout::number(unit.map_or(value, |u| value / u.factor()));
```

That `map_or` **is** `crate::props::shown_in(unit, value)`, which is
`in_written` over a field that may name no unit, which is
`canonical / unit.factor()`. Verified on `origin/main`:
`props::shown_in` is at `props.rs:267`, `in_written` at `:247`.
`shown_in`'s own doc says it is *"spelled once because both panel
fields need it and a hand-written `map_or` at each is the same
identity written twice, free to become two"* — this is the third site
and the hand-written `map_or` that doc names.

Nothing is wrong with the number today; what it costs is that the
divide has two homes, so a change to how a written value is derived
from a canonical one reaches the panel fields and not the range
reading. Route the reading through the door.

**Then sweep**: `props`' module doc claims *every value crossing the
module* goes through that pair. AUTH-2's sweep pattern was `factor()`
across `crates/viewer/src/` and found exactly these three. Run a
DIFFERENTLY SHAPED sweep of your own — `factor()` misses a site that
holds a factor in a local, and it misses `crates/viewer/tests/`
entirely — and put the hit list with its disposition in the PR, one
line per hit, plus **what your pattern could not match**.

## Half 2 — three private constants restated as literals

`crates/viewer/src/camera.rs` above `Camera::pitch_limit()` settles
what this costs: a test that restates a contract as a literal *"is a
hand-synced copy of a private constant — the defect this accessor
exists to remove. One home; read it."* That is the shape to copy.

Three rows in `crates/viewer/tests/display_budget.rs` are that copy,
each with a comment admitting it. **Line numbers re-derived on
`origin/main` for this dispatch; the row's own numbers have rotted:**

| copy | original |
| --- | --- |
| `const INITIAL_DELTA: f64 = 1.0e-4` (`display_budget.rs:36`) | `app::INITIAL_DELTA` (`app.rs:180`) — **`cfg`-gated behind the `app` feature**, so this one needs a door that survives the gate, not just a `pub` |
| `const PROBE_FACTOR: f64 = 8.0` (`display_budget.rs:42`) | `scene`'s private `PROBE_FACTOR` (`scene.rs:922`); the rung bound below it is read against the copy |
| `let scale = 1.0e9` (`display_budget.rs:1252`) | `scene`'s private `SCALE_PROBE_DELTA` (`scene.rs:934`) |

Each is a number the row's assertion DEPENDS on, so a stale copy does
not fail the build — it makes the row assert against a number the code
no longer uses.

**Three restatements in the same suite are the argued opposite and
must NOT be "fixed"** — the row says so and I reaffirm it:
`edge_pick.rs` spells its own `1e-6` band deliberately rather than
importing `OCCLUSION_SLACK_REL` (*"would make the row agree with the
code by construction"*), `datum_draw.rs` states the arms structurally
rather than against a copy of `FRAME_ARM_PX`, and `gesture_table.rs`
refuses to restate a 41-row table. **A constant a row DEPENDS on wants
one home; a threshold a row CHOOSES is the row's own.** If your sweep
turns up a fourth site, decide which of the two it is and say which.

**The row's own stated blind spots, which you are expected to close or
re-state as yours**: a literal spelled differently is invisible (`8.0`
does not find `8f64`, `8.` or `4.0 * 2.0`, and nothing finds a literal
that is an arithmetic CONSEQUENCE of a constant); the nine
`#[cfg(test)]` modules under `crates/viewer/src/` were never looked
at; other crates' suites were never swept.

## The trap

`docs/prompts/reviewer-style-lane.md` §1, last bullet: a lane closing
a duplication mints one, and a lane closing a hand-written census adds
a hand-written census. You are about to write accessors and a hit
list. Naming this in the PR body has never once prevented it — check
your own diff before you push.

## Scope

**In**: `crates/viewer/src/bounds.rs`, `crates/viewer/src/app.rs`,
`crates/viewer/src/scene.rs`, `crates/viewer/src/camera.rs` (read for
the shape), `crates/viewer/tests/display_budget.rs`.

**Out**: `crates/viewer/src/props.rs` — read it, call its doors, do
not change it (AUTHOR has open rows there). `pane/*`, `session*`,
`forms.rs`, `pickindex.rs`, `frame.rs`, `datums.rs` — other lanes are
live in several of those this hour.

`app.rs` and `scene.rs` are claimed by CHROME, VIEW, VSEAM and VGEOM;
`display_budget.rs` by CHROME, TCOST, TINT and VIEW. Double claims,
legitimate — what is owed is **awareness while a lane is live**. Run
`python3 scripts/work.py territory --base main` on your branch and put
what it printed in the PR. Merge `origin/main` before you push.

A finding outside this fence gets a ROW under implementer-discipline
§6, in this same PR — not a fix, and not a line in the PR body.

## Verification

Hosted CI is the verification of record: expect **twelve `test (…)`**
and **five `k-lint (gate, …)`** jobs, confirm the run's head SHA is
your branch head, and read the STEP for any row whose job name does
not name it. Poll the run in the FOREGROUND until it concludes and
report in the same turn — never end a turn with a wait armed.

The `app`-feature rows are the ones this unit touches and they gate at
ONE eps row only (`work/ciw/the-viewer-app-feature-rows-gate-one-eps-
of-three.md`), so a green run says less here than usual. Locally:
`cargo fmt --all --check`, `cargo clippy -p viewer --all-targets --
-D warnings` and again `--features app`, the `display_budget` suite,
`python3 scripts/work.py lint`.

`CARGO_TARGET_DIR=/home/user/chrome-one-home-target`, exported on
EVERY cargo invocation, **outside the worktree**. Scratch files in
`~/.local/share/cad-work/chrome-one-home/` — never the session
scratchpad, which is shared between lanes.

## Deliverable

A PR titled `CHROME: one home for the panel's divide and for three
constants a suite copied`, body carrying: both sweeps' hit lists with
dispositions and blind spots, how you made `app::INITIAL_DELTA`
readable across its feature gate, the territory output, the mutations
proving any new row can go red, and any §6 rows you filed. Update both
item files' bodies with what you measured. **Report to me; do not
merge.**
