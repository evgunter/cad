# Lane `heldrecords/one-list` — P1, H

**Row:** `work/heldrecords/carried-records-keep-per-kind-vectors-beside-the-one-list.md`. Read it in full, with D1, and `crates/topo/README.md`'s tier 3′ carry paragraph, which was just revised by PR 7745 (removed members, merged at `0ac627ec92`).

**Base:** branch `heldrecords/one-list` off `origin/main`, and the PR goes to main. PR 7124 (EMBED, `embed/records`) and HELDRECORDS's split stack (7421, 7481, the parting-division lane) all touch the carry too, so keep the refactor **behaviour-neutral** and merge main often. Conflicts are expected; make the change easy to port.

Owed:
1. **One list through the op door and the carry.** Replace the per-kind vectors in `OperandRecords` (`vv, vf, ve, ee, ef, ff`), `ops::Rows` and `ReductionContacts` with one list:
   - either of `Cited<Contact>`, or of cell pairs tagged with their operand;
   - per-kind reads, where a caller truly needs one, are derived from the list.
   The seven hand-written per-kind arms collapse into one:
   - `ContactRecords::carried`
   - `ops::carried_rows`
   - `ops::Rows::push`
   - `ops::Rows::of`
   - `ops::carry_rows`
   - `validate_declarations`
   - `OperandRecords::is_empty`

   Canonical order comes from `Contact::new`, decided once.
2. **The reduction's A×B face pair gets its own type**, distinct from a carried two-faces-of-one-operand `FfContact`.
3. **Behaviour-neutral, and shown to be:** zero test verdict moves across topo, sweep and editor-core `ci` at ε and 1e-12. Diff head against base, and explain every moved test. Any moved test is a stop-and-report.
4. **The source scan:** grep that no per-kind vector or per-kind match arm over record kinds survives outside the one derived read. Put the grep in the PR body.
5. **Docs:** update the READMEs that name the per-kind types. Only wording that follows from the change, with no design change.

Review tier: single FULL review. It is mechanical, but it is H and sits on the carry.

Standing rules: `briefs/common.md` on this branch.
