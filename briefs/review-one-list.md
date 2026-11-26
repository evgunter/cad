# Review: PR 7782 (`heldrecords/one-list`), frozen head `e60352af27`, base main — single FULL review

**Probes:** only to `heldrecords/review-one-list-<6 hex>`.

**Read first:**
- `CLAUDE.md`
- `docs/prompts/reviewer-style-lane.md` IN FULL
- the row `work/heldrecords/carried-records-keep-per-kind-vectors-beside-the-one-list.md` on the head
- D1
- `crates/topo/README.md`'s tier 3′ carry paragraph

**Review:** `git diff origin/main...e60352af27`.

**What it claims:** a behaviour-neutral refactor.
- `OperandRecords` and `ops::Rows` each become one list of `Contact` rows, filed in the carry's kind order (`Contact::carry_order`, `records::file_by_kind`).
- `ReductionContacts` becomes one list of operand-tagged cell pairs, with per-kind reads derived from it.
- The reduction's A×B face pair gets its own type, `TangentFaces`.
- `VeContact` and the carried `FfContact` are gone.
- Seven-plus per-kind arms collapse to one each.
- Kernel-visible notes, which the lane says move no test:
  - door `InvalidInput` messages are now per cell kind;
  - a record naming one cell twice is unbuildable (`Contact::new` refuses `OneCell`);
  - material re-keys are in canonical order;
  - `TouchesItself.faces` is canonical.

**Claims to falsify:**
1. **Behaviour-neutral.**
   - Per test, head vs base `ffc8c37d4c`: the full default profile with the slow set included, for topo, sweep and editor-core, and `ci` at 1e-12.
   - The lane reports an empty diff, with 10 reds identical on both sides. Confirm that.
2. **Order.** Every reader that depended on per-kind order sees the same order. Find any reader that iterates the one list and assumed kind-grouping, or a canonical-order change that alters output (records on a result, a material re-key, error text a downstream consumer parses: the GUI, pncad-py).
3. **No per-kind logic survives** outside the one derived read. Re-run the PR's source-scan grep, and look for per-kind matches it misses.
4. **The `OneCell` refusal.** A carried record naming one cell twice was buildable before. Can any real op produce one (a glue, a material re-key)? If so, the refactor now refuses where it carried.
5. **`TangentFaces`.** The A×B pair is never confused with a one-operand pair at any site.

**Running it:** own `CARGO_TARGET_DIR` outside the checkout, per crate. Foreground rule: never end a turn with background work.

**Report:** ≤150 lines, by `mcp__claude-code-remote__send_message` to `session_018u3gVKp4QiofNJ9Z1mzgD8`, then stop. Include:
- the verdict;
- claims held or broken;
- MAJOR, MINOR and NOTE findings;
- a `## Style` section, each item marked sure, likely or unsure;
- probe rows.

No email or personal identifier anywhere; no model names in commits.
