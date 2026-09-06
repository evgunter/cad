# GATES — the CI gate scripts (plan)

**STATUS: OPEN (2026-09-06).** Opened in the tracker-wide cut of
2026-09-06 (`docs/WORK-TRACKS-2026-09.md`, addendum 2), claiming
code-quality Track K's `scripts/gates/*` half. Live state is
`work/gates/log.md`'s tail and the item files beside this plan, never
this file.

Branch prefix (the #396 convention): **`gates/`** — unit branches
`gates/<unit>-<slug>`, orchestrator branch `gates/orchestrator`.
Away-channel tag `(GATES orchestrator)`. A/B ordinal band
**GATES = 3100–3199**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in the opening commit; infra-only units record no row, so the
band is claimed for bookkeeping.

## Charter

The gates are the invariants CI holds that Rust cannot express: who
may name a `Bounds`, what stays debug-only, which squares are
interval-safe, what a discard must justify. Every one is a grep with
a selftest, and every row here is a place a gate is blind, reads
something it does not re-derive, or is wired against one file by path.
The standing rule from Track K's own lanes: **a gate change carries
the before/after hit-set diff on the live tree**, and a matcher that
widens counts and grandfathers the population it reds (the S63 rule:
no cry-wolf-then-allowlist).

Territory: `scripts/gates/*` whole, including the two files retired
Track J left unfenced. The gates' wiring (`ci.yml`, `ci-local.sh`) is
CIW's and is one announced line per new gate.

## Review posture

The S-TCOST posture: one style review per unit against
`docs/prompts/reviewer-style-lane.md`; adversarial where a wrong
answer is reachable (a matcher that can be made to pass a planted
breach — every gate row, so every review plants one). No A/B rows.

## Unit order

`bounds-allowlist.sh`, one file, several rows — landed one at a time,
smallest first, so the redesign at the end starts from a gate whose
small defects are gone:

1. `trait-generic-sole-bracket` — skip a balanced `<…>` after the trait
   name; plant the `trait` form.
2. `unanchored-definition-skip` — a `DEFINITION_HOME_RE` anchor on the
   definition skip, the `no-extra-real-bounds.sh` shape; enumerate the
   other exact-text skips under `scripts/gates/`.
3. `D211` — the readings nothing re-derives: the interval-square
   allowlist's "none of these is live" scan gets an executable home;
   GAP 3's count re-derived or de-counted.
4. `D103` — allowlist granularity: line- or symbol-scoped entries, or a
   pinned per-file count, chosen and applied.
5. `D102` + `bounds-tripwire-blind-to-named-alias` — the matcher
   redesign (where-clauses, rustfmt multi-line, named compound
   bounds), with the grandfathering count; the one paragraph in
   `real.rs` (PROPS', by note) closes the tripwire item.

The other scripts:

6. `bit-identity-debug-only-gate-ends-an-item-at-a-semicolon` +
   `debug-only-counters-have-no-gate` — one lane: the awk's `;` branch
   fixed at depth zero, then the gate generalised to a (subject,
   symbol) list with `product.rs`'s gather counter as the second
   subject; wiring announced to CIW.
7. `gate-mod-path-resolved-textually` — `#[cfg(test)] mod` resolved the
   way rustc does; measure the population first; a fixture per
   direction.
8. `D109` — the F3 sweep's four blind spots in `lib.sh` and the two
   roster gates; each member its own small edit, (e) a cost note.
9. `S13` — the greps' known defects (the `x*x` lookahead, `Real +` not
   stripping comments, the `self.x * self.x` blind spot), then the
   AST-lint alternative actually evaluated and written down.
10. `clippy-panic-gate-blind-in-macros` — direction chosen by this
    program (a token-grep gate over `macro_rules!` bodies with a
    `#[cfg(test)]` allow — the stanza's per-module idiom, stricter
    than a `#[test]` allow — and the one-time audit in the PR body;
    thin-macro delegation not taken as a convention), built as
    `panic-free-macro-bodies.sh` (PR 2032).
11. `S49` — the deferral-register gate over every `LoopBoundary`
    discard, `probe-suite-census.sh`'s shape; the audit of the 26+15
    sites is riders on their owners.
12. `D212` — rides `G4`; lands with it and not before.

## Exit shape

The twelve land, Track K's `scripts/gates/*` half is empty; the walk
convention applies.
