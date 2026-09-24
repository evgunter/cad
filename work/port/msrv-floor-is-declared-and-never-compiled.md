---
id: msrv-floor-is-declared-and-never-compiled
kind: issue
title: Cargo.toml's rust-version is a floor nothing verifies and nothing ever compiles at
status: closed
opened: 2026-09-11
refs: [seal-oracle-toolchain-read-first-match, 2327]
branch: port/msrv-floor-equality-gate
pr: 2676
closed: 2026-09-15
priority: P3
cost: E
---

Filed in `work/issues/` because no program obviously owns it: the subject is
the workspace manifest's MSRV declaration, not a workflow, a gate or a crate.

`Cargo.toml:50` declares `rust-version = "1.97.0"` in `[workspace.package]`,
inherited by every member. `rust-toolchain.toml` pins `channel = "1.97.0"`.
They carry the same string today **by coincidence of both having been set to
the current compiler**, and PR 2327 (CIW unit 7) made the distinction explicit
in `local-scripts/seal-oracle.sh`: the toolchain file is the compiler this
workspace builds with, and `rust-version` is a floor a consumer's compiler
must clear. Establishing that they are different claims is what surfaces this:

- **Nothing reconciles them.** The day the channel moves to 1.98, the MSRV
  declaration stays at 1.97.0 and nothing says so — which is CORRECT if the
  floor is a real promise, and silent drift if it was only ever a copy.
- **Nothing ever compiles at the floor.** Every hosted job, every local gate
  row and `local-scripts/ci-local.sh` build on the pinned channel. No row
  builds the workspace on 1.97.0-as-a-floor once the channel moves past it, so
  the promise `rust-version` makes to a consumer has never been tested and
  cannot fail. A `cargo build` on the declared MSRV is the only thing that
  would make it a claim rather than a number.

**The question to settle first is whether the floor is a promise at all.**
The repository publishes nothing (`publish = false`, and the project has no
name yet — Q9), so there is no consumer today whose compiler has to clear it.
Three honest answers:

1. It is a promise → it owes a row that builds on it (nightly, plausibly), and
   the row is the unit.
2. It is documentation of the current pin → then it should be derived from
   `rust-toolchain.toml` or deleted, not maintained by hand beside it.
3. It is dormant until the project publishes → say so at the declaration, and
   re-open on Q9.

Cheap either way; it is the decision that is the work, and it is a decision
about what this repo promises rather than about CI.

## Answered (Ev, in-chat, 2026-09-15)

Asked which of the three readings holds, Ev answered with a fourth that
subsumes them: **a check that the two strings are equal, and call it a
day.** That is answer (2) — the declaration is documentation of the pin
— made mechanical instead of maintained by hand, and it is a better
answer than (3) for a reason worth writing down:

**Pinned equal, the floor stops being untested.** The row's complaint is
that no job ever compiles at the declared MSRV. If the gate holds
`rust-version == channel`, then the channel *is* the floor, and every
hosted job, every local gate row and `local-scripts/ci-local.sh` already
compile at it — on every run, at every lane and eps point. The promise
`rust-version` makes to a consumer becomes exactly the promise CI
already proves, with no new build row and no nightly. Reading (1)'s
"owes a row that builds on it" is satisfied by the rows that exist.

**What it costs, stated so the deletion is deliberate when it comes.**
The gate forbids the floor from ever lagging the channel. The day this
repository wants to say *"we build with 1.99 and still support 1.97"* —
which is a real thing to want once Q9 lands and something is published —
the gate is wrong and has to be deleted, and at that moment the original
question comes back with a consumer attached to it. That is the right
time to answer it and the wrong time to be surprised, so the gate's
header says this and names Q9.

## The unit

**Shape: a `scripts/gates/` row, not a Rust `#[test]`.** Ev said "unit
test"; this is the repo's spelling of the same instrument, and the
substitution is disclosed here rather than made silently:

- The subject is two TOML manifests at the workspace root, not any
  crate's behaviour — a Rust test would need an arbitrary crate to host
  it and would have to walk up out of `CARGO_MANIFEST_DIR` to find its
  own subject.
- `scripts/gates/` is where checks over the tree already live, each with
  a `--selftest`, a real call in `.github/workflows/ci.yml` and a place
  in the directory loop `local-scripts/ci-local.sh` runs.
  `scripts/gates/gate-roster.sh` proves every gate in the directory is
  wired into both halves, so a gate cannot be silently dropped — a Rust
  test has no equivalent roster.
- A manifest edit is in the change set the per-PR gate runs on, which is
  where `docs/prompts/implementer-discipline.md` says a guard belongs.
  The gate therefore fires on the PR that moves the channel, which is
  the only moment it has anything to say.

Parse the TOML rather than grepping it, for the reason
`scripts/gates/kernel-serde-free.sh`'s header gives at length: a value
has more spellings than a regex has patience, and `rust-version` is
already one of the dotted-key fields that header names.

If Ev wants a Rust test instead, say so and it moves; nothing else in
the row changes.

**Announce to:** `Cargo.toml` and `rust-toolchain.toml` are CIW's and
META's between them (`work/port/program.md`'s `keep_out`), and
`scripts/gates/` plus `.github/workflows/ci.yml` are CIW's. Either may
take this row; it is a small one and belongs to whoever is already in
those files.

## Re-homed to PORT (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

PORT collects the crate-boundary doors — the Python surface, the
exchange crates and the façade refusals — where the thing a user meets
is a refusal. This row is one of them.

Its class at the cut was **M** — decision about what repo promises; edit
itself is one line or row. The class is a dispatch estimate made by
reading the row against the tree on 2026-09-11, not a verdict on the
finding, and a lane that finds it wrong says so in its PR. The id, the
`track:` letter where the row carries one, and the body above are
unchanged by the move.
