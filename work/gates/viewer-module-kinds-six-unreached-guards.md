---
id: viewer-module-kinds-six-unreached-guards
kind: issue
title: six of viewer-module-kinds.sh's gate_error guards are reached by no selftest case
status: closed
opened: 2026-09-06
refs: [D109]
branch: gates/viewer-module-kinds-guards
pr: 2057
closed: 2026-09-06
---

## Finding

`lib.sh` says a guard never shown to fire is not a guard. Re-deriving
`D109(d)`'s reading on 2026-09-06 — the declared population is
`grep -nE '(^|[[:space:];&|])gate_error "' scripts/gates/*.sh`, traced
against every `--selftest` run with `gate_error` instrumented — found
**12 of 105 sites** unreached at `origin/main` (fcbd8942e), not 6 of 82.
`D109` planted the six it names. **These six are the rest**, and all of
them are in one gate:

- `scripts/gates/viewer-module-kinds.sh:251` — `$SRC` does not exist
  (the subject gone).
- `:263` — no modules under `$SRC` besides `lib.rs` and `bin/` (the
  scanned-nothing guard).
- `:383` — every driver in the README's table also hosts a vocabulary,
  so check 6's path arm matches nothing.
- `:427` — no vocabulary modules under `$SRC`; every module declares
  itself a driver.
- `:459` — the exception list names a file that is not under `$PWD`.
- `:464` — the exception list names a module that no longer declares
  `//! Module kind: **vocabulary**`.

The last two are the shape `D109` already corrected once elsewhere: an
exception list bounded by a check that is itself never shown to fire is
bounded by nothing. `:383` and `:427` are the vacuity guards — the gate
reporting green because its matcher had no population to decide over —
which is the same class as `gate_require_crate_sources`'s two.

**The count is a reading, not a register**: the population grows every
time a gate gains a guard (105 at that base, 106 after `D109` added the
shared reader's own death guard), so re-derive rather than trusting the
number.

**And re-derive it with the messages, not the line numbers alone.** A
`gate_error` inside a command substitution reports the line of the
ENCLOSING FUNCTION CALL — bash resets the call stack in the subshell —
so a line-only trace names a line holding no `gate_error` and reports
that site unreached however many fixtures fire it. Match a fired message
to a site by ALL of the site's literal fragments appearing in ONE
message; any-of scores a site reached off a fragment two messages share.
`scripts/gates/lib.sh`'s `gate_selftest_clean` carries this note.

## Why not fixed here

`D109`'s fence is `lib.sh`, `probe-suite-census.sh` and
`gate-roster.sh`. `viewer-module-kinds.sh` is this program's ground and
nobody's open lane; six fixtures is a unit, not a rider.

## Landed (2026-09-06)

Six fixtures on `gates/viewer-module-kinds-guards`, one per site.

**The reading, re-derived at merge base 607ecfe2a** the way this row
says: `gate_error` instrumented with `BASH_SOURCE`/`BASH_LINENO` AND the
message, every `--selftest` in `scripts/gates/` traced, a site scored
reached when ALL of its literal fragments appear in ONE fired message.
**113 sites, 6 unreached** — the six named above, at the lines named
above. The population moved 105 -> 106 -> 113 across D109 and the lanes
that landed beside it, which is why it is re-derived rather than
carried: it is a reading, not a register.

**Five sites took a fixture where they stood. One could not.**
`:427` — no vocabulary modules under `$SRC` — sat below check 4, which
requires every row of the README's vocabulary tables to name a module
declaring `vocabulary` and exits if one does not, over tables already
proved non-empty. Below that exit an empty vocabulary set is
unreachable, so the guard could not fire for any tree. It now sits
directly under check 1, where the tree's own answer is known, and its
fixture is a tree whose every module declares `driver`.

**The two exception-list guards are the shape `D109` corrected
elsewhere**: an exception list bounded by a check never shown to fire is
bounded by nothing. Both plant the ENTRY and not a file — it reaches the
gate through `GATE_SELFTEST_VOCAB_EXCEPTIONS` while `gate_plant_clean`
plants from the in-process list, so an entry can name a path the fixture
never writes.

Every guard was backed out on a scratch copy in turn and each time
exactly its own case went red. **After: 113 sites, 0 unreached, in this
gate and in `scripts/gates/` as a whole.** The live pass is unchanged —
same counts, same 42 scanned modules.
