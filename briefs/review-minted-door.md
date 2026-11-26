# Review: PR 7808 (`heldrecords/minted-door`), frozen head `46186823d1`, base main — single FULL review

**Probes:** only to `heldrecords/review-minted-door-<6 hex>`.

**Read first:**
- `CLAUDE.md`
- `docs/prompts/reviewer-style-lane.md` IN FULL
- the row `work/heldrecords/the-boolean-door-decides-no-minted-vertex-on-edge-meeting.md` on the head
- the log entries 2026-11-25/26 in `work/heldrecords/log.md`: the fork and the probe
- Ev's ruling `a2090eeb4d5`: per op, no cutter field
- `crates/topo/README.md`: the tier 3′ carry paragraph and [coincidence]

**Review:** `git diff origin/main...46186823d1`.

**What it builds:**
- `ops::minted_meeting` decides minted point meetings at the boolean's door:
  - vertex–vertex by the gap;
  - a vertex inside an edge by `census::on_edge_interior_margin`, the confirm pass's own reading.
- Zero lands, citing the decision.
- In band or Off at the door is `JoinDesync`, not a second escalation, because `landings` has already confirmed the pair with the same reading.
- The row sits on the record's own input cells (E, F) as a `Meet`, not an `OnCarrier`.
- Section vertices are now named on the carried path.
- `uncopied`, `Ledger::copies` and `BooleanReduction::copies` are deleted.

**Claims to falsify:**
1. **No false landing.**
   - The door's Zero must mean v truly lies in s's interior.
   - Attack: v at s's end, v just off s, a curved s near its seam, and two meetings on one record sharing one row (`reduce::pend` keeps the first margin). Is the kept margin the worse one? If not, is that a lie on the row?
2. **The row on (E, F) is sound.**
   - It says E and F meet at that point, which the record already says.
   - Check that no reader treats a `Meet` row on (E, F) as a fresh coincidence the op made. Cover the `unproven-coincidence` lint, editor-core `in_member_space`, pncad-py, and `cite_rows`/`spell`.
   - Check that moving the vertex–vertex row from minted cells to (E, F) changes nothing a downstream reader keys on.
3. **The `JoinDesync` departure is right.**
   - Is the door's reading exactly the one `landings` confirmed: same band, same cells, same function?
   - Or could the two disagree, so that a valid body refuses as a defect? Look for differing tolerances or a differing cell (clone vs result).
4. **Naming section vertices on the carried path** reopens neither the old panic nor any silent consume.
   - Re-run R1's PROBE_GAP rows from `heldrecords/review-removed-members-0c6ef7` (face, edge, recut) and the mutant switches from PR 7799's probes.
   - Compare the counts.
5. **No valid body moves.**
   - Per test, head against base `ffc8c37d4c`, the full default profile with the slow set: topo, sweep and editor-core.
   - Also `ci` at 1e-12.
6. **The stress row** pins sound-or-typed, never a wrong body.
   - Check its 0.03° `Unread` and 0.007° `CurvedPierceEscalated` outcomes are in band and not a defect.

**Rules:** own `CARGO_TARGET_DIR` outside the checkout; build per crate. Foreground rule: never end a turn with background work.

**Report:** at most 150 lines, covering:
- the verdict;
- claims held or broken;
- MAJOR / MINOR / NOTE;
- `## Style`, each item sure / likely / unsure;
- probe rows.

Send it with `mcp__claude-code-remote__send_message` to `session_018u3gVKp4QiofNJ9Z1mzgD8`, then stop. No email or personal identifier anywhere; no model names in commits.
