---
id: k-lint-predicate-roster-unpinned
kind: issue
title: k-lint's EPS_COUPLED_PREDICATES rosters the kernel's predicate vocabulary with no pin in either direction
status: closed
opened: 2026-09-03
refs: [D204, k-lint-eps-coupled-criterion-unwritten, k-lint-csv-header-unpinned-against-five-producers]
closed: 2026-09-07
branch: meter/klint-roster-pin
---

## Was

unrowed. Raised by the `D204` lane's cross-root-constant sweep as
**exactly `CHART_TAGS`' shape**, one tool over.

## Finding

`tools/k-lint/src/lib.rs:257` holds
`EPS_COUPLED_PREDICATES = ["props_quad_converged"]` — a roster of the
*kernel's* predicate vocabulary, held in a workspace-excluded consumer.
The name it rosters is minted at `crates/geom-brep/src/props/quad.rs:560`
and `:2993`.

Nothing pins the two together in either direction. `tools/k-lint`
contains no `include_str!` — unlike `tools/tess-meter`, which reaches
into `tools/tess-lint`'s source precisely to pin a constant across a
cargo-root boundary without taking a dependency — and nothing in
`crates/` mentions the roster. So:

- a predicate the kernel **renames** leaves the roster naming nothing,
  and the ε-coupled arm silently stops applying to it;
- a predicate the kernel **adds** to that class is absent from the
  roster and is judged by the wrong rule.

Both directions are silent, which is the half that matters: this is a
lint whose whole job is to decide which margins are ε-coupled, and its
input vocabulary can drift out from under it without anything reddening.

**Why it is not `D204`'s work.** `D204` pinned `CHART_TAGS` from the
meter's side, where an `include_str!` reader already existed to extend.
Closing this one needs either an edit under `crates/` (Track Q's
`props/quad.rs`) or a new reader on the k-lint side, and `tools/k-lint`
has none to extend. A row landing on it draws that fence first.

**Note on the sweep that found it.** The pattern was `const`
declarations, and it cannot see a shared vocabulary that is not a
`const` — `Chart::tag`, the producing half of the constant `D204`
pinned, is match arms returning string literals and was invisible to
the same sweep. Expect siblings spelled that way.

## Claimed by METER (2026-09-06)

Moved from `work/code-quality/` to `work/meter/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, `track:` letter and body unchanged. Unlettered; `tools/k-lint` is METER's and the `props/quad.rs` names are PROPS' — the seam is drawn first.

## Closed

`tools/k-lint/tests/predicate_roster.rs` — the `D204` shape, from the
consumer's side because this crate is the side that has one:
`geom_core::k_stats::decide` takes a `&'static str`, so the kernel's
predicate vocabulary has no enumerable definition to import, and the
only route left is `include_str!` over the mint source lexed through
`test-utils`' `source` into blanked views. `tools/k-lint/Cargo.toml`
gains that one dev-dependency; it is a zero-dependency leaf, so the
lockfile grew by exactly one package.

**Cited lines, re-read.** The roster is at
`tools/k-lint/src/lib.rs:283` (the Finding says `:257`), and the mint
sites are `crates/geom-brep/src/props/quad.rs:570`, `:3231`, `:3490` and
`:3568` — four, not the two at `:560`/`:2993` the Finding names. The pin
parses the call rather than the line, so it carries all four and does
not restate any of them.

Four tests. `every_rostered_predicate_is_still_minted_by_the_kernel`
locates every `classify_len::<T>(` call in the code view of
`crates/geom-brep/src/props/quad.rs` and reads its first argument out of
the literal view, then asserts every `EPS_COUPLED_PREDICATES` entry is
among them. `the_rostered_familys_margin_is_still_eps_scaled_at_its_mint`
pins rule (4)'s premise as well as its key, **per mint**: the margin
uses the WHOLE identifier `target_len`; that identifier is bound exactly
once in the mint's own enclosing `fn`, to `QUAD_TARGET_LEN_FACTOR * eps`;
a mint whose `target_len` arrives as a PARAMETER is refused rather than
answered; and the factor is a finite positive literal.
`every_target_len_mint_is_rostered_or_excused` runs the converse — a
mint that acquires such a margin and is neither rostered nor excused by
name reds. `the_pin_reads_a_mint_a_respelling_and_a_missing_name_red_it`
is the negative control for all three: a doc comment and a quoted string
spelling the same call above two real ones, a call respelled out of the
parse, and a mint short the rostered name, each asserted to red — the
last through the same function the first test asserts on, not a copy of
its rule.

**The parse's coverage is pinned too**, which the first version of this
pin left open: every `classify_len` in the source must be a call the
parse read or the function's own declaration, so a site respelled
`classify_len:: <T>(` reds instead of quietly leaving the population the
other three quantify over.

Each red was demonstrated by mutating the tree and reverting: a
`fixed_target_len` bound beside the real one and read by a rostered
margin; a rostered name moved onto the mint whose `target_len` is a
parameter; one call respelled out of the parse; and the negative
control's own fixture literal changed. The first version of this pin
stayed GREEN under the first three of those, which is how they were
found (style review, 2026-09-07). Nothing under `crates/` is edited in
the PR.

**The finding's framing was wrong on one point and short on another,
and both are written up in `k-lint-eps-coupled-criterion-unwritten`'s
correction section.** "Both directions are silent" does not hold: a
roster omission — a rename as much as an addition — leaves the family
under rules (2) and (3), where its ε-scaled margins flag at both tight
rows. `tools/k-lint/src/lib.rs:160` claims it, `tests/review_probes.rs:77`
measures it, and `docs/K-REPORT.md:645` ratifies it. What the pin buys is
therefore a DIAGNOSIS: the gate fired in the wrong voice, sending its
reader to re-derive a baseline that never moved. The direction that was
genuinely silent is neither of the two named here — a rostered predicate
that stops being ε-coupled while keeping its name — and it is the second
test above.

**Residue, each with its own file.** The general ε-coupling criterion
stays open as `k-lint-eps-coupled-criterion-unwritten` (the pins are
hand-written against one family's spelling of ε-coupling and do not
extend to an entry the kernel has not minted yet).
`k-lint-last-round-is-eps-coupled-but-unrostered` is what the converse
test found: one mint that is ε-coupled by the same reading, excused
rather than rostered because rule (4)'s floor has never been measured on
it. `k-lint-roster-wants-a-kernel-side-vocabulary` records that a Rust
parser is the instrument METER's fence left, not the one the problem
asked for, and names the two cheaper shapes a PROPS unit could land.
`reader-census-shared-disposition-survives-partial-reversion` is the
census gap this PR walked through on its way in. The sibling the sweep
turned up is `k-lint-csv-header-unpinned-against-five-producers`.
