---
id: editor-core-suites-carry-eleven-part-resolver-stubs
kind: issue
title: Eleven editor-core suites carry their own PartResolver stub, in_part and PART_BODY; MSOLVE-4 hoists one into tests/fixture and the ten siblings owe a migration
status: closed
opened: 2026-09-06
closed: 2026-09-15
---


Found by MSOLVE-4's style review (PR 1960); filed by the MSOLVE
orchestrator. Test infrastructure, no obvious program owner (TCOST or
CIW by shape).

`impl PartResolver for StubStore` with an `insert` that pins a document
and a `resolve` that checks the pin, plus `in_part(instance, cap)` and
a `PART_BODY` constant, are copied across `mate1_member_vocab.rs`
("as in the sibling suites"), `mate1_r1_probes.rs`, `mate1r2_probes.rs`,
`mate6_gather_mints.rs`, `mate6r1_shared.rs`, `mate6r2_probes.rs`,
`asm_r2b_assembly.rs`, `fix_pattern_mate_crossing.rs`,
`rev_fix_xsplit_unreachable.rs`, `msolve1_transform_aware.rs` and (at
its review) `msolve4_mate_memo.rs`. MSOLVE-4's fix pass hoists one into
`crates/editor-core/tests/fixture/` and uses it from its own suite; the
ten siblings keep their copies until a migration lands. A copy that
drifts (a pin check dropped, a different failure text) is a suite
testing a different resolver than its neighbours believe.

## Same class, second instance (MSOLVE-2's style review, 2026-09-06)

`in_copy(pattern, i, master)` has seven private copies across the
mate suites (`fix_pattern_mate_crossing`, `mate1_member_vocab`,
`mate1_r1_probes`, `mate1r2_probes`, `msolve1_transform_aware`,
`rev_fix_xsplit_unreachable`, `msolve2_member_chain`), and `run`,
`gate`, `xform` travel with it. MSOLVE-2's fix pass hoists one set
into `tests/fixture/` beside the seat oracle for its own two suites;
the rest migrate with the resolver stub.

## Re-homed to SUITE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

SUITE collects the test-side rows — the suites, the fixtures and the
shared helpers — that S-TINT's and S-TCOST's own boards did not carry.
This row is one of them.

Its class at the cut was **M** — migrate ten-plus suites, reconcile
drift between copied stubs and helpers. The class is a dispatch estimate
made by reading the row against the tree on 2026-09-11, not a verdict on
the finding, and a lane that finds it wrong says so in its PR. The id,
the `track:` letter where the row carries one, and the body above are
unchanged by the move.

## Closed by the migration and its guard (2026-09-15, PR 2628)

`crates/editor-core/tests/fixture/resolver.rs` holds the one `PartStore`,
`in_part` and `PART_BODY`; seventeen suites across **two** crates read
it. The migration was the easy half.

**The census was wrong three times over, and each wrongness has a
lesson.** The row named eleven suites; so did the dispatch, from a
different and also wrong list. The real population at merge base was
**sixteen** in `editor-core`, plus a seventeenth in `crates/viewer`.
`grep -rln "impl PartResolver for"` misses `impl editor_core::PartResolver
for`, which six files spell path-qualified — a **regex** miss. All four
of the unit's own sweeps pointed at one crate, so the viewer copy was a
**path** miss. The sweep that found the real population keyed on the
**consumer** (`resolver: Some(`) rather than the definition, because a
consumer-shaped pattern cannot miss a store by type name.

**What the migration uncovered is worth more than the duplication it
removed.** `mate1_r1_probes` and `mate6r1_shared` spelled `in_part`
naming `RecipeNodeId(1)` — the **profile** node — where every sibling
names the extrude. Their "good" member names named a face that does not
exist. Correcting it changes **eight of `mate6r1_shared`'s fourteen
printed probe lines**, and `mate1_r1_probes::r1_which_branch_does_the_
consistent_loop_row_take` — the row whose name *is* its question — went
from `Err(… does not name a face of the product)` to `Ok(minted = 2)`.

**None of it gated, and that is the finding.** `mate6r1_shared` has
eleven tests and **zero assertions**; `mate1_r1_probes`' flip happened
under green CI. A planted `PART_BODY` mutation reddened 73 rows across
eleven suites and **zero** in `mate6r1_shared` (0/11),
`mate1_r1_probes` (0/8) and `rev_fix_xsplit_unreachable` (0/6). Three
consumers rode a shared constant that nothing held. Per
`memories/test-suite-cost.md`, green over an assertion-free suite is
evidence of a suite that cannot fail.

**The defect had been disclosed in prose and never read.**
`docm6_seam_declarations`' own header said *"the suites' `in_part`
spellings have already diverged once by a node index."* That is Q1's
self-declared-copy pattern exactly, and nothing in CI, review or the
logs had ever read it.

**So the fix is a guard, not a sentence.** `PartStore::insert` calls
`assert_part_body`: in a document that has an extrude at all, the extrude
is the node `in_part` names. The same planted mutation now reds **224
rows across 21 suites**, 222 of them carrying the guard's own message;
`mate6r1_shared` goes 11/11 and `mate1_r1_probes` 8/8. Measured live:
445 documents checked, 47 exempt, and the 47 are the stands and rows the
docstring names. Two exemption edges are stated rather than papered
over — it keys on "has an `Extrude`" as the proxy for "is a part
document", so a `Revolve`/`Loft`/`Sweep` part would be silently exempt
(`seat6_param_source`'s `filleted_lantern` is one suite away), and it
sits at `insert`, so it cannot reach `in_part` used with no store.

**The unit also removed the copies it had made collapsible**: twelve
local `fn run`, twelve `opts`/`with_resolver`, and fifteen open-coded
`EvalOptions { resolver: Some(…) }` literals — the same copy without a
function around it. `docm4`'s `run` stays, because `run(doc, prior, …)`
is a seven-file class, six of them outside this unit, filed as
`work/tint/run-with-a-prior-evaluation-has-seven-private-copies.md`
(three of the seven are byte-identical pairs: one function, three option
presets, written seven times).

**And it disclosed the blind spot it creates**, in the tree rather than a
PR body: `grep "resolver: Some("` no longer finds this population, and
worse, it returns only **non-members** — the deliberate stand-downs — so
the next lane's sweep comes back confidently wrong rather than short.
`with_resolver`'s docstring says so and names what to grep instead.

Residue on S-TINT: `mate6r1-shared-has-eleven-tests-and-no-assertions`
(widened to its class — `mate6r2_probes` is the twin, and
`rev_fix_xsplit_unreachable` the third consumer),
`three-part-resolver-stub-residues-resist-the-shared-fixture`, and the
`run(doc, prior)` class.
