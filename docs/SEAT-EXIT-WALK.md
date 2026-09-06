# SEAT — exit walk

**Status: DRAFT for Ev's ratification** (`[ev]` PR). Walked against
main after SEAT-9's merge (PR #1995) by `git show origin/main:…`,
`python3 scripts/work.py status`, and the ledger rows — not from
memory. The program closes when this walk is ratified as
`docs/SEAT-EXIT-WALK.md`; the closing sweep then deletes
`work/seat/` (program, plan, log, closed items) and this walk,
records them in `docs/DOC-LEDGER.md` at the sweep SHA, re-homes the
open items listed in §4, and moves `docs/VERB-SEAT-DESIGN.md` beside
the code it governs as `crates/verbs/README.md` (present tense,
clause ids kept; DESIGN.md's companion table updated) per
`CLAUDE.md`'s rule for finished design docs. One structural note:
`docs/VERB-SEAT-DESIGN.md` is the ratified contract (#1388); this
walk closes the PROGRAM that executed it, clause by clause, and says
where each clause's text now lives.

## 1. Program units — the done-state per unit

Every unit below ran the v6 blinded dual (two concurrent reviewers
on a frozen head, arms by parity byte, implementer-inherited fix
pass, state-sync riding the unit PR). Ordinal band 1000–1099.

| unit | contract | state |
|---|---|---|
| SEAT-1 | §1 S4 | DELIVERED #1399, ordinal 1000, sample #77 — `band: Band` dropped from the blend and shell doors; each derives `Band::linear(tol)` at entry; 421 door calls rewritten, every one the linear derivation (the no-numeric-change proof); the demo frictions the doctrine names retired |
| SEAT-2 | §1 S1+S2 | DELIVERED #1521, ordinal 1001, sample #94 — `topo::query`: materializers in arena order, EXACT kind predicates, the DECIDED datum-distance door through the relocated `sel_datum_distance` funnel site (name byte-identical, one site before and after); `candidate_matches` delegates atom by atom; the prelude exports the query module |
| SEAT-3 | §1 S3 | DELIVERED #1531, ordinal 1002, sample #97 — `topo::flush`: the verify rung relocated byte-equivalent, `find_flush_candidates` over two bodies, `declare`/`declare_all` → `BooleanDeclarations`, one `FlushFinding<P>` generic in the pair vocabulary (keys at the body seat, names upstairs); both ~55-line hand declarers deleted; the issue-757 producer gap retired. Planar scope inherited deliberately — measured, and widened later by SEAT-FW |
| SEAT-4 | §2 V1–V4 (the substrate, the blend pair) | DELIVERED #1547, ordinal 1003, sample #103 — `crates/verbs` (VS-Q1 answered: its own crate), `Verb<T>` Fillet/Chamfer, `run`/`VerbOut`/`VerbError`, `param_flow` as data, the layer guard; `editor-core`'s `verb_content_tag` with the existing tags pinned to the merge-base constants, the per-instance `BlendVerb` correspondence, `wire_fillet`/`wire_chamfer` collapsed onto one generic `wire_blend`; wire format byte-identical, digests reproduced on the extracted base; VS-Q5 decided in the unit spec |
| SEAT-DV | issue 1527's ruling (side unit) | DELIVERED #1564, ordinal 1004, sample #105 — `DatumValue` normals unit-by-construction (`UnitVec3<T>`, normalize-or-refuse typed); the SEAT-2 tripwire retired; ONE disclosed observable move (the length decision reaches the funnel), pinned |
| SEAT-5 | §2 V4 (boolean) | DELIVERED #1581, ordinal 1005, sample #109 — `Verb::Boolean{op, declare}`, `Arity` with per-arity doors and a typed mismatch refusal both ways, the closed per-family `VerbRecord`, `PairOut`, the `PairVerb` correspondence driving `wire_boolean` as a second lowering; tags 8/9 pinned |
| SEAT-6 | §3 P1–P3 (issue 1372) | DELIVERED #1593, ordinal 1006, sample #131 — the opaque `ParamSource` token (VS-Q4 REVISED at this unit with Ev's sign-off, PR #1870: a canonical injective encoding scoped by parameter table, not an interner), per-field side records, attach-at-mint driven by the declared flow, propagation by key identity, the first consumer (`cylinder_cylinder_section`'s `RadiusEvidence`); the absence row pinned (P3) |
| SEAT-7 | §2 V4 (extrude, revolve) + §6 end to end | DELIVERED #1910, ordinal 1007, sample #137 — `Arity::Profile` as a third operand shape with `run_profile`, `FlowSource::ProfileEdge` (a circle edge's radius → the wall it sweeps), the sweeps' lowering; the §6 row: one declared `r` reaching the cyl×cyl germ from a document, the literal twin `None` |
| SEAT-8 | §2 V4 (split) | DELIVERED #1950, ordinal 1008, sample #139 — `Verb::Split`, `Arity::Split`, `SplitOut{above, below, record}`, `run_split`, the split lowering; the "adding a verb after this" baseline amended (a door's impl bound is part of its signature) |
| SEAT-FW | §1 S3's scope, widened | DELIVERED #1974, ordinal 1009, sample #143 — the flush detector detects what the `Rest` verifier verifies at the curved rungs too: one identifier moved (`pair_finding` asks `carrier_pair_relation`), the anti-twin rule becomes identity; twopeg's 18 cylindrical hand declarations and the lily's socket assemble through detector + declare; the tour byte-identical; one Ev-gated stop honored (§4, S3's parenthetical) |
| SEAT-DN | Ev's ruling (B), `[ev]` #1902 | DELIVERED #1987, ordinal 1010, sample #146 — one decide/normalize/refuse body (`topo::query::decide_unit_direction`) under the two ratified funnel names, no K-REPORT row moved; DN-3 measured, neither branch applied; the overflow class found one level up and filed as a five-member class |
| SEAT-9 | §2 V4 (shell) + Ev's ruling (i), `[ev]` #1904 | DELIVERED #1995, ordinal 1011, sample #147 — `Verb::Shell` under its own bound in a second impl block, `Arity::Shell` (argued deviation), the tag censuses learning a kernel-only verb as closed data; the shell doors drop `tolerance: f64` and `Tol` travels to `geom-brep`'s production fit doors with one `eps()` read beside the classification; 22/24 `FIT_TOL` retired; Ev's suspicion (PR 1904) answered on the base by both arms: no over-ε geometry was ever handed back — tier-3 validation already re-certified at the run's ε; the ruling removes the caller's second epsilon and moves the refusal earlier |

Plan rows not cut as units: none. The plan's "SEAT-5+ per-verb
migrations … extrude/revolve/split/shell as their own units, each
costed" is complete (SEAT-5/7/8/9); every verb the document layer
authors is on the substrate, and the one kernel-only verb (shell)
declares itself so.

## 2. The design contract — clause by clause

| clause | disposition | text now |
|---|---|---|
| §1 S1 query module | executed (SEAT-2) | `topo/src/query.rs` module header |
| §1 S2 `select_where` wrapper | executed (SEAT-2); behaviour pinned unchanged | `editor-core/src/names/geompred.rs` |
| §1 S3 flush detector at the body seat | executed (SEAT-3), widened to curved rungs (SEAT-FW); the clause's parenthetical names `oriented_plane_eq` and "lily's six" where the code is `carrier_pair_relation` and the socket is three — the `[ev]` correction is PR #1983 (open at this writing; its merge or this walk's ratification settles the text) | `topo/src/flush.rs`; SELECT-DESIGN §3 |
| §1 S4 band derived at entry | executed (SEAT-1); the acceptance "no public kernel verb takes a `Band` beside a `Tol`" is a measurement with no mechanical guard (recorded in the 1409 item, now closed with the tolerance half) | the doors' headers |
| §2 V1 closed kernel-side declaration | executed (SEAT-4); nine verbs, five doors, the `ALL` censuses | `crates/verbs/src/verb.rs` |
| §2 V2 owner-held stable-tag commitments | executed (SEAT-4/5): `verb_content_tag` beside the memo, every existing number pinned digit for digit; kernel-only verbs declare `None` (SEAT-9) | `editor-core/src/eval/mod.rs` |
| §2 V3 per-verb correspondence in `editor-core` | executed: `BlendVerb`, `PairVerb`, `ProfileVerb`, `SplitVerb` — per-instance data, no vocabulary match in the module | `editor-core/src/verbs/` |
| §2 V4 per-verb, additive migration | executed for every document verb; each costed against the chamfer baseline (§6) | the ledger rows' costing tables |
| §3 P1 lowered expression identity | executed (SEAT-6); VS-Q4 revised | `editor-core/src/verbs/param_source.rs` |
| §3 P2 attach / propagate / consume | executed (SEAT-6/7): attach at mint by declared flow incl. profile-edge flow; propagation by key identity; the germ consumer | same |
| §3 P3 absence refuses permanently | executed and pinned (`the_same_geometry_without_the_channel_refuses` — the kernel-door twin standing for the imported posture; no STEP round trip in the row) | `editor-core/tests/seat6_param_source.rs` |
| VS-Q1 | answered: `crates/verbs` | — |
| VS-Q2 | answered: typed per-verb `Node` variants kept; the macro deferral never triggered — the arm noise after six migrations is the routed `verb_refused` arm and four record projections, paid per verb | — |
| VS-Q3 | stands: kernel-derived fields have no source (SEAT-9's empty thickness row is the latest instance) | — |
| VS-Q4 | REVISED (encoding), Ev's sign-off #1870 | doc text |
| VS-Q5 | decided in SEAT-4's spec | — |
| VS-Q6 | followed | — |
| §5 out of scope | #1345 (2)/(3) stay deferred — see §4; the #917 rename untouched (its own scale); **the post-publish stable-tag discipline was NOT folded into DESIGN.md's "Before publishing" list at ratification** — this walk's PR adds the bullet | DESIGN.md |
| §6 acceptance | costing: every recipe door after §2 costed (SEAT-5/7/8/9) with `node.rs`'s projection matches untouched; the germ end to end from a document (SEAT-7) and the absence row (SEAT-6); the demo frictions retired at their sites (SEAT-1/2), twopeg's and the lily's declarations through detector + declare (SEAT-3/FW), the two declarers deleted (SEAT-3); N4 untouched — every unit's wire-format and digest pins | — |

## 3. The A/B instrument — state of record

- Twelve v6 duals, ordinals 1000–1011, twelve FAIR pairs (every
  disclosure was command-line or listing class, adjudicated at the
  pair).
- Blocks and draws, all published: B1 byte 85 (SEAT-1 opus · SEAT-2
  fable · SEAT-3 opus · SEAT-4 opus); B2 byte 165 (SEAT-DV opus ·
  SEAT-5 fable · SEAT-6 opus · SEAT-7 opus); B3 byte 8 (SEAT-8 fable
  · SEAT-FW opus · SEAT-DN opus · SEAT-9 opus).
- **Tally candidates: five** — SEAT-DV #1 (R1 opus), SEAT-5 #2 and
  #3 (R1 opus), SEAT-6 #4 (R1 fable) and #5 (R2 opus); none in
  B1 or B3. Coding rules applied consistently across the program:
  doc-only MAJORs excluded; a unilateral executed MAJOR counts; two
  arms finding one class at different instances is converged.
- Orchestrator brief errors, each corrected by the arms and recorded
  in the rows: SEAT-6 (parameter names are length-prefixed), SEAT-FW
  (the undeclared-contact recourse is two-armed), SEAT-DN (the
  "status flip-flop"), SEAT-9 (tour shells fit nothing).
- Method lessons banked in the ledger and `work/seat/log.md`: never
  reclaim an implementer's worktree before its fix pass; reviewer
  differentials build extracted bases in separate targets;
  worktree-local scratch only; a dirty PR gets no Actions run; a
  scope-claim sweep must cross line breaks and include every spelling
  of the shape.

## 4. Open list at close — disposition for the sweep

| item | disposition |
|---|---|
| `verb-seat-design-s3-names-the-planar-verifier` (needs_ev) | `[ev]` PR #1983 carries the doc correction; closes when Ev merges it — the sweep waits on that or folds it here at Ev's word |
| `two-verb-seats-do-not-compose` (#1345 items 2/3) | deferred by the design (§5): a seatless call log needs a replay consumer that does not exist; re-home to `work/issues/` with the §5 text as its trigger |
| `flush-pair-relation-has-no-caller` | S-BOOL's module (`topo/src/boolean/rest.rs`); re-home to `work/bool/` |
| `two-d-director-doors-skip-the-finiteness-question` (five-member class) | FIX's `is-finite-length-homed-in-the-query-seat` is the ruling that closes it; re-home to `work/fix/` |
| SEAT-originated items already homed elsewhere | `work/issues/axis-flavoured-declarations-have-no-channel`, `node-tag-space-census-blind-to-tags-outside-sentinels`, `two-public-verb-types-verbs-and-profile`, `dirty-pr-gets-no-actions-run`, `approx-surface-tolerance-is-now-always-the-runs-eps`, `offset-fit-at-tight-eps-refuses-every-curved-nurbs-chart`; `work/lib/lib-g17-is-parked-on-a-fired-trigger`; `work/curved/c5a2-ledger-sample-143-collides-with-seatfw` — nothing to move |
| Ev's retroactive review of the compound-`Bounds` allowlist entries SEAT-4 and SEAT-9 amended (`geom-core/src/real.rs`, the self-merge convention's flag) | stands as flagged in the file; this walk's ratification is a natural moment to take it, or leave the flag |
| LIB-G17 `Node::Shell` | LIB's row; its enabler (`VerbRecord::Shell`) is in place |

## 5. What this program leaves behind

`crates/verbs` (the vocabulary seat, nine verbs, five doors, the
flow declarations, the layer guard), `topo::query` and `topo::flush`
(the kernel query seat), `editor-core/src/verbs/` (four
correspondences and the parameter-identity channel), and one
tolerance witness travelling every offset signature from the shell
door to the fit's classification. The ratified design's text moves
beside `crates/verbs` at the sweep; its clause ids survive there.
