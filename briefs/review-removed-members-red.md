# Review: PR 7799 (`heldrecords/removed-members-red`), frozen head `17237bee03`, base main — single FULL review, P0 (main red)

**Probes:** only on `heldrecords/review-removed-members-red-<6 hex>`.

**Read first:**
- `CLAUDE.md`
- `docs/prompts/reviewer-style-lane.md` IN FULL
- the row `work/heldrecords/main-red-removed-members-refuses-three-slow-carry-rows.md` on the head
- `crates/topo/README.md`, the tier 3′ carry paragraph
- PR 7745's body, for the removed-members design it fixes

Review `git diff origin/main...17237bee03`.

## Background

PR 7745 (merged at `0ac627ec92`) moved three slow-set rows from building to refusing. This PR fixes them:
1. **Glued structure** consumes on OneCell on any image, piece or minted. Incidence counts for nothing.
2. **A removed-member read** takes `ContactContradicted` as touched, in that read only.
3. **Ruling (C).** A minted point meeting that is not two vertices is set aside only when the same record also lands on a non-minted pair. Nothing is recorded and nothing is consumed. Where it is the only landing, it still refuses: `MintedMeeting` at the public door, `JoinDesync` in the boolean.

The orchestrator ruled (C) with t3′ on the result as the backstop.

## Claims to falsify

1. **No silent drop.**
   - (1) must not consume a record whose touch lies off the glued cell. Attack with a OneCell piece where the record's touch is elsewhere: overlapping edges glued, the record at a far point.
   - (2) must not consume a record that the removed member contradicts because the touch is NOT there. Attack with a removed fragment whose census finding contradicts the record's kind for a reason other than the two-whole-solids class.
   - Re-run R1's PROBE_GAP switches from `heldrecords/review-removed-members-0c6ef7` (face, edge, recut). Count silent consumes against PR 7745's head `a27ce80982`.
2. **(C) is sound.**
   - A set-aside meeting must never hide a coincidence the result then ships unrecorded. Build a pose where the set-aside minted vertex really touches the section edge in the result, and check that t3′ refuses it or that it is recorded.
   - Check that the pierce row's t3′ findings are only its pinned ones.
3. **No valid body moves.**
   - Run a full **default** profile (slow set included), per test, for topo and sweep on the head and on main's parent of `0ac627ec92` (`cc90e26a`). Use the same cargo profile on both.
   - Also run `ci` at 1e-12 on both.
   - Built → refused must be empty, except main's known reds, which must be listed.
4. **The pins and mutants catch what they claim.** Re-run SETASIDEALONE, CONTRALEAK, ANYINC, NOSECT, ANYREM, IGNREM and JOINTBAD. Add one mutant that loosens (1) to incidence and one that applies (2) outside removed reads.

Own `CARGO_TARGET_DIR` outside the checkout; per crate. Foreground rule: never end a turn with background work.

## Report

At most 150 lines:
- verdict
- claims held or broken
- MAJOR / MINOR / NOTE
- `## Style`, each sure / likely / unsure
- probe rows

Send it by `mcp__claude-code-remote__send_message` to `session_018u3gVKp4QiofNJ9Z1mzgD8`, then stop. No email or personal identifier anywhere; no model names in commits.
