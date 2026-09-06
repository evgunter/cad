---
id: mate-contradiction-names-one-mate-twice
kind: issue
title: Mate-solve contradiction diagnostics: mates 6 and 6 cannot both hold, and a levered angle printed in metres
status: closed
opened: 2026-09-01
github: 1462
refs: [1461, levered-clash-margins-hide-their-arm]
branch: fix/mate-contradiction-display
pr: 1766
closed: 2026-09-04
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

**Measured first, and the item's premise was WRONG.** The clash is
written literally as `theta * arm` (`mate/solve.rs:548`), with `arm`
from `Alignment::lever_arm()` (`mate.rs:260`) — the larger of the two
frame origins' distances and the authored lengths, floored at 1 m. The
repro's frames both sit at the origin and `FrameCoincidence` has no
authored lengths, so the arm is exactly 1.0, its FLOOR, and that alone
is why `π/2 · arm` reads as a raw π/2 (probe: moving one origin to
`[0,0,3]` gives `arm = 3`, `clash = 4.712…`). But `Margin::levered` is
D4's own θ·r door, so θ·arm IS metres. **The metre is honest**; the
defect is that the figure is *unrecoverable* with the arm invisible.

**What landed.**

- `MateFault::Contradictory` gains `lever: Option<(f64, f64)>` —
  `(radians, arm in metres)`. Deliberately NOT a new named type: the
  export and binding gates key on names added to `editor-core`'s
  `pub use` list, so a field on the existing variant trips neither, and
  this change stays inside its own crate.
- The `Display` names ONE mate once when `held == added`, and keeps the
  pair sentence otherwise.
- A levered clash prints the product it COMPUTES from the two halves it
  shows ("a roll of … rad on a … m arm, a deviation of … m"), so the
  sentence cannot assert an identity the payload failed to keep.
- Whether there is a measurement at all is read from `predicate`
  against the shared `MATE_MEMBER_EMPTY` constant, never from the
  margin's value. `mate_member_empty` no longer prints "inf m"; a NaN
  or negative infinity under any other predicate no longer borrows the
  empty set's sentence.
- The clocking site reads `Margin::value()` instead of respelling
  `theta * arm` three lines after the door computed it.

**Rows.** `display_contract.rs` pins the corrected sentences plus the
product-is-the-product and non-finite-is-not-the-empty-set properties;
`asm_r2a_mate_solve.rs` row 7g drives the real repro through
`solve_document`.

**Swept for.** (1) A diagnostic naming one subject twice; (2) a levered
quantity reaching a message with its arm invisible. Hit lists and
per-hit disposition are in PR 1766.

**What the sweep could NOT match.** A pair rendered across two `write!`
calls, through a helper, or joined by punctuation rather than a word; a
pair whose `Display` forwards to a nested type. For the levered shape,
any levering done inside a named helper the `* arm` grep never sees
(`rotation_residual` was found by reading `member_of`, not by the
grep). Neither grep sees a value levered several frames above its
message.

**Residue, filed:** `work/fix/levered-clash-margins-hide-their-arm.md`
— three sibling margins in `mate/coset.rs` still arrive with
`lever: None`.

**Out-of-fence findings** (reported, not filed): three
`PoisonEnclosure` raise sites in `crates/editor-core/src/clearance.rs`
can name one `FaceKey` twice across bodies, in two renderings. Routed
by the orchestrator as a class.
