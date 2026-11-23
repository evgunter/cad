---
id: main-red-the-python-suite-errors-on-a-union-that-reads-a-family-whole
kind: issue
title: main red: the python suite ends with 18 errors, a union handed a family whole refused as a list of bodies where it takes a body
status: open
opened: 2026-11-23
priority: P1
cost: E
---


Found by the ONEROUND orchestrator on 2026-11-23 from two PRs' hosted runs, neither of which touches the code or the tests that fail.

**What is red.** The hosted job `python suite (wheel + guide + north-star)`, step "run the Python suite": `Ran 998 tests … FAILED (errors=18)`. The errors read alike:

```
pncad.EditError: Union 213562b2f2b5's member 1 reads #100:9958ca42ae8b3701, which is a list of bodies, where it takes a body. Recourse: splice it, `…#100:9958ca42ae8b3701`
```

Among the 18 (the tail of the log; the head was not read): `test_union.TestTheUnion.test_one_read_of_a_family_is_its_members` and `test_a_family_spelled_by_index_is_the_family_read_whole` (`crates/pncad-py/tests/test_union.py`, at `doc.insert(Node.union(family))`), and `test_pattern.TestTheRowIsAFamily.test_the_fin_union_is_one_body` and `test_the_fin_union_equals_the_placement_union_chain` (`crates/pncad-py/tests/test_pattern.py`, `fins`, at `doc.insert(Node.union(row))`). The tests' own docstrings say a single operand that is not a list "reads a family whole"; the door now refuses that and asks for a splice.

**Why it is main's.** Both runs test a PR's merge with main as it stood after 20:29Z:

- PR 7510 (`oneround/conic-readers`, head `1332f63ebc`), run 45260186497, job 137994850225, the suite's step 21:02Z to 21:07Z. Its diff against main touches `crates/pncad-py/src/lib.rs` by one line and no union or pattern code.
- PR 7225 (`oneround/box-region`, head `606af1e32c`), run 45261854002, job 138007112618, ended 21:15Z. Its branch holds main only as of `74feb4851c`; its diff touches pncad-py's tag tables and the binding census, not `test_union.py`, `test_pattern.py` or the union's seats.

The same 18 errors in both, in tests neither PR edits.

**Suspected cause, not bisected.** PR 7506 (`formwords/binders`, merged 20:29Z as `1cf86c9e96`), whose commits read "binders: pattern callers yield; Python DocEdit.set_yields and the tag tables; the tour's patterns yield and splice" and a merge of main for PR 7426 ("a subtract's tool is a list seat … a yield is spliced"). No run of main itself has concluded since: each push to main cancels the run before it (runs 45262861929, 45264237292 and 45266616496 all cancelled), so main's own python suite has not been read after that merge. Either the tests are stale against a seat that now wants a splice, or the Python door lost the read of a family whole; the lane that changed the seat knows which.

P1 (a red main; if the Python door really no longer takes a family at a union's seat where the guide says it does, that is a normal verb broken and the row is P0).
