---
id: msrv-floor-is-declared-and-never-compiled
kind: issue
title: Cargo.toml's rust-version is a floor nothing verifies and nothing ever compiles at
status: open
opened: 2026-09-11
refs: [seal-oracle-toolchain-read-first-match, 2327]
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
