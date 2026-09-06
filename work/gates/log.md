# GATES log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/gates/plan.md`.

## Opened (2026-09-06)

Opened in the tracker-wide cut of 2026-09-06 (Ev's direction,
in-chat; `docs/WORK-TRACKS-2026-09.md` addendum 2). Code-quality's
Track K was "this program's to dispatch" and had `smell/k-*` branches
dated 2026-09-03 with no PR on any of them; the letter is claimed
whole, this program taking `scripts/gates/*` and METER the
instruments. Fourteen rows moved by `git mv` with a `## Claimed by`
record each: ten with `track: K` and four unlettered rows on K's
fence. No branch exists yet; the first dispatch is unit 1.

## First wave dispatched (2026-09-06)

Orchestrator's first session, on a remote box (PR subscriptions rather
than the local away-channel monitor). Three lanes on three disjoint
scripts, each in an isolated worktree, implementer briefs pointing at
`docs/prompts/implementer-discipline.md` by path:

- `gates/bounds-small` — `trait-generic-sole-bracket` then
  `unanchored-definition-skip`, one PR, two commits, on
  `bounds-allowlist.sh`; the enumeration of every other exact-text skip
  under `scripts/gates/` rides in the PR body.
- `gates/debug-only-subjects` — the `;`-at-depth-zero parser fix, then
  the (subject, symbol) list with `product.rs`'s gather counter as the
  second subject, on `bit-identity-debug-only.sh`; name and command line
  unchanged so CIW's wiring needs no edit.
- `gates/mod-path-resolution` — population first, then rustc's
  resolution, a fixture per direction, the hit-set diff, on
  `interval-square-allowlist.sh`.

Posture: one style review per PR against
`docs/prompts/reviewer-style-lane.md`, adversarial (a planted breach
each), no A/B row. Standing rule to every lane: a fix that reds live
code stops and reports — no allowlist entry buys green. Log entries are
the orchestrator's; lanes edit only their own item headers.

## Second wave dispatched (2026-09-06)

Two more lanes on files the first wave does not hold:

- `gates/d109-reader-blind-spots` — `D109`'s four members in order,
  on `lib.sh`, `probe-suite-census.sh` and `gate-roster.sh` (the two
  roster gates are this program's now); every gate re-run after each
  member since all of them source the reader.
- `gates/panic-free-macro-bodies` — `clippy-panic-gate-blind-in-macros`,
  direction decided by the orchestrator: the row's option (2), a
  token-grep gate over `macro_rules!` bodies with a `#[cfg(test)]`
  allow, the one-time audit in the PR body, option (4) not taken as a
  convention. The one wiring line in `ci.yml` and `ci-local.sh` is the
  announced CIW line the charter allows.

`S49` waits for the first wave's reviews; `S13`, `D211`, `D103` and
`D102` queue behind `gates/bounds-small` on the same file.
