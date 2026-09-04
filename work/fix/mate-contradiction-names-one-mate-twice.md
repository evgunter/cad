---
id: mate-contradiction-names-one-mate-twice
kind: issue
title: Mate-solve contradiction diagnostics: mates 6 and 6 cannot both hold, and a levered angle printed in metres
status: review
opened: 2026-09-01
github: 1462
refs: [1461, levered-clash-margins-hide-their-arm]
branch: fix/mate-contradiction-display
pr: 1766
---

## From GitHub issue 1462

Opened 2026-09-01; 0 comments.

Found by the `story_assembly` integration lane (same repro as issue 1461: a FrameCoincidence mate committed with a nonzero clocking rider, refused by `mate_clocking_redundant` at the next evaluation). Two message defects in the refusal's rendering:

1. **The contradiction names one mate twice.** The self-contradictory-rider arm reports `held == added`, so the sentence reads "mates 6 and 6 cannot both hold" — to a user that reads like an indexing bug, and it hides the actual shape (one mate contradicting *itself* via its rider), which the arm knows.

2. **The clash magnitude is a levered angle labeled metres.** The disagreement prints as `1.5707963267948966 m` — θ·arm dressed as a length. Whatever the internal currency, a roll disagreement surfaced to a user wants either the angle with its own unit or an honest "levered by the contact arm" phrasing; a raw π/2 wearing metres is the D6 display discipline stopping one formatter short.

Both are presentation-only — the refusal itself is correctly typed and correctly fires.

(story-suites orchestrator)

## Home

`work/lib/` — S-MATE's `keep_out` puts the refusal-display prose with LIB, and this is entirely presentation of a refusal's rendered message (the D6 display discipline), not the solve itself.

## Closed

Both defects were presentation, and the refusal still fires exactly
where it did. What the measurement changed is the second one's
diagnosis.

**Measured first.** The clash magnitude is written literally as
`theta * arm` at `crates/editor-core/src/mate/solve.rs:548`, with `arm`
from `Alignment::lever_arm()` (`crates/editor-core/src/mate.rs:260`) —
`max(‖a.origin‖, ‖b.origin‖, 1.0, authored lengths)`. The repro's frames
both sit at the origin, so the arm is exactly 1.0, its FLOOR, and
`π/2 · 1.0` is why the printed figure reads as a raw π/2. So the item's
"levered angle labeled metres" is right about the lever and wrong about
the unit: `Margin::levered` is D4's own door
(`crates/geom-core/src/predicate.rs:499`) and θ·arm IS metres — the
point deviation the roll induces at the arm. The defect is not a wrong
unit but an **unrecoverable** number: the arm is invisible, so a reader
cannot get θ back, and the metre figure equals π/2 only by the
coincidence of a unit arm.

**What landed.**

- `MateFault::Contradictory` gains `lever: Option<ClashLever>`
  (`ClashLever { disagreement, unit, arm }`). `clash` is unchanged — the
  metres the predicate decided on — so every consumer keeps reading what
  it read. The clocking arm fills it; the two fold sites pass `None`.
- The `Display` arm names ONE mate once when `held == added` ("mate 6
  contradicts itself — the constraints it declares admit no common
  pose") and keeps the pair sentence otherwise.
- A levered clash prints both halves and the product: "measured a
  disagreement of 1.5707963267948966 rad levered by a contact arm of 1 m
  — that is a clash of 1.5707963267948966 m". The angle-unit branch and
  the honest-lever branch the issue offered are BOTH taken, because
  printing the angle alone would drop the metres the band actually
  compared against.
- `mate_member_empty` carries `f64::INFINITY` and used to print "a clash
  of inf m". It now says the cosets meet in the empty set — a structural
  refusal with no margin to measure. Same sentence, same class.

**Rows.** `crates/editor-core/tests/display_contract.rs` pins both
corrected sentences plus the dimensionless and structural cases;
`asm_r2a_mate_solve.rs` row 7g drives the real repro through
`solve_document` and checks `disagreement * arm == clash`; the two
existing `Contradictory` rows now assert `lever.is_none()` for the
length-measuring predicates.

**Swept for.** Two shapes. (1) A diagnostic naming one subject twice —
grepped `{} and {}` / `{} (and|vs|against|with) {}` templates across
`crates/*/src` and `demos/`. (2) A levered quantity reaching a message —
grepped `Margin::levered`-family call sites (203) and `* arm` into a
payload. Hit lists and per-hit disposition are in the PR body.

**What the sweep could NOT match.** A pair rendered across two `write!`
calls or through a helper rather than one template; a pair joined by a
word other than and/vs/against/with, or by punctuation (an arrow, a
slash); a pair whose `Display` forwards to a nested type that names the
subjects; and, for shape 2, any levering done in a named helper whose
body the `* arm` grep never sees (`rotation_residual` was found only by
reading `member_of`, not by the grep) or performed in a different unit
system. Neither grep can see a value that becomes levered several frames
above the message.

**Residue, filed:** `work/fix/levered-clash-margins-hide-their-arm.md` —
three sibling margins in `mate/coset.rs` still arrive with `lever: None`
because filling them is solve-internal plumbing in S-MATE's live
territory, not refusal prose.

**Not taken, reported:** `crates/pncad-py/src/py/mate.rs:672` exposes
`clash` as a `Length` with no lever getter, so the Python consumer meets
the same unrecoverable metre figure one layer out; `pncad-py` is LIB's.
