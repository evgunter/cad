# JOIN-2 review — lane r2

PR #3880 at frozen head `17c254c99d` (base main `66bbdaa6bd`). Lane isolation: I read no other
review branch and no PR comments. I read the PR body through the API.

**Verdict: APPROVE-WITH-FIXES.** Counts: MAJOR 1 · MINOR 2 · NOTE 5.

No wrong body was found anywhere: 0 BAD on either tree over my 2,088-line battery. The MAJOR is a
SOUND→refusal regression that the PR does not disclose. The fix is to re-file it and pin it, not
to change code, unless the orchestrator wants the residue decided now.

How I worked. The probes are in `crates/sweep/tests/join2_r2_probes.rs` (ignored rows registered
in `all.rs`). Each prints one `common::differential::outcome` line per pose × order × op
(∪ ∖ ∖′ ∩), with closed-form volumes taken from the outlines (shoelace plus bulge segments). I built
the same file on main (worktree, `boolean_join_refusal` stubbed) and on head, and diffed the
lines. I ran an instrumented head build that logs every germ whose locus differs from main's
first-order reading, and every tie, over the full topo+sweep suites. I ran five mutations, each
with the full suite (4150 tests).

## MAJOR

**M1. The filed residue is a SOUND→refusal regression, and the PR says it is "as before".**
Executed (`join2_r2_like_far_end_touches`).
- *Pose.* A lower plate with a convex r = 1 fillet at (3, 0), followed by a notch open to the south.
  The upper plate's straight bottom edge is tangent at (2, 0) and re-meets the lower plate at the
  notch corner (4, 0) (a vertex pair, so `Boundary`). The upper plate's own notch floor `y = d`
  crosses the arc (also `Boundary`). The instrumented build logs `R2TIE Boundary` 72 times, all
  from this test, so the residue is reached.
- *Head.* Every ∪ refuses `Join(UnpairedLooseEnds{2})`. There is no wrong body: ∖, ∖′ and ∩ are
  SOUND.
- *Main.* Main builds 24 of the 36 ∪ lines **SOUND**: d = 0.3 at all three notches, d = 0.5 at
  (2.2, 3.5) and (2.5, 4.5), and d = 0.8 at (2.5, 4.5), in both orders and both stackings. Main's
  own segment enumeration (the facing test and the arc pass) built them. The tie site reads the same
  loci on both trees, so main's join refused them too, and main's zip took them over (inferred from
  the log).
- *What the PR says.* The PR body says "stay `OnEdge` both, as before". The residue file
  `work/join/two-tangent-edges-parting-with-like-far-end-touches-stay-on-edge-both.md` says no row
  reaches the shape. The "0 SOUND→refusal" claim holds only on the PR's batteries. This
  falsifies claim 4 ("nothing the deleted code decided is lost") on a reachable pose, and claim 5's
  "previously built right".
- *Fix.* Record in the residue file that main builds these poses, carry this fixture into it, and
  scope the PR body's claim. A non-ignored row pinning the refusal would keep the shape visible.

## MINOR

**m1. The named pins cannot go red under a revert of the switch** (claim 6). Executed by mutation:
I put main's `rest.rs`, `refusal_routes.rs` and `offer_rows.rs` onto head and kept the locus fix.
Under that revert all of these stay **green**: the re-pinned tangency row
(`reach_continuation.rs:492`), `a_shaft_off_the_bores_seam_is_built_by_the_zip`
(`full_turn_bore_mate.rs:175`), and every `boolean_join_refusal` pin. Main's zip builds those poses
too, so the pins only show that the zip built the union, not that it built it on the join's
segments. One row in all 4150 goes red: `join_rc_probes::flush_declared_reflex_unions_never_ship_the_overlap_twice`,
which pins BAD→refusal. No row pins a pose that builds *because of* the switch.

Under a revert of the locus fix (germ reads `OnEdge` on any On bound), the tangency row does go red,
at its new join assertion (`Ok(Some(UnpairedLooseEnds{8}))`). The other pins stay green, which is
expected because their poses have no tangent germ.

**m2. Ring-first ordering is needed but not sufficient** (claim 3).
- *Needed.* Executed: with ring-first disabled (`rest.rs:920`), `mate2_r1` and `mate2_r2` go red.
- *Not sufficient.* `join2_r2_ring_order_through_the_zip`: a channel with both arms crossing the
  plate's north edge, the plate's fillet tangent to one arm's end. It refuses
  `RestZipUnsupported{ChordBetweenIsolatedPierces}` in both orders. An order exists (join each arm's
  boundary-to-ring chord first), but the span between the two ring vertices comes first by index.
  Main refuses identically, so this is not a regression.
- *Doc.* The doc at `rest.rs:905` says these chords go first, "each joining its ring into the face's
  boundary before any chord divides that face". That is false when both ends are rings.

## NOTE

**N1. Claim 1 (ranking) holds; I found no misranking.** Executed: 1,920 lines.
- *Convex.* A sharp rect over and under a rect rounded at one corner, r ∈ {0.02, 0.25, 1, 2.5, 3.9}.
  The sharp rect's edge sits short of the fillet, on its tangent point, through the arc (0.3r,
  0.7r), at its far end, and past it, with overhang variants.
- *Concave.* A sharp-notch L over and under the filleted L, r ∈ {0.02 … 2.9}, notch corner
  c ∈ {2.5, 3, 3 + r/2, 3 + r, 3 + r + 0.25}.
- *Result.* Every line is SOUND or `EMPTY ok` and byte-identical to main; 220 of the unions are
  zip-built.
- *Why misranking cannot happen.* The sweep splits at every crossing. So the edge that leaves the
  partner ends `Apart` or `Boundary`, and the edge inside it ends `Boundary` or `Face`. Only a tie
  is possible. The "touches the partner elsewhere, past a gap" construction (M1's notch) produces
  exactly that tie, not a wrong rank.
- *Fail-safe.* A misread locus cannot pair: `partners` requires equal loci at both ends, and
  `locus_at_site` requires the edge at both sites (`join.rs:941`, `:990`). The outcome is a refusal,
  not a body.

**N2. Claim 2 holds.** Executed with the instrumented build over 4150 tests (all pass). The only test
whose loci move is the tangency row (304 diffs). On my battery, every line outside M1 matches main.

**N3. Claim 3: no chord in a wrong fragment.**
- `fragment_holding`: disabling the fragment lineage (`rest.rs:977`) turns 5 rows red, so it is
  exercised.
- `tangent_face`: forcing its fallback (`sectors.rs:993`) turns the tangency row red.
- An island plus a crossing component in one face (`join2_r2_island_*`) builds SOUND through the
  join. Through the zip it refuses `ChordBetweenIsolatedPierces` on both trees, before any `mef`
  could strand the island.

**N4. Claim 4: `joined` can be fooled, harmlessly.**
- `joined` (`rest.rs:1077`) answers yes for any edge between u and v.
- `pair_patches` (`rest.rs:1452`) compares vertex cycles only. So two distinct curves between one
  vertex pair would pass both checks.
- This is harmless as far as I can see, because paired patches are discarded whole. It is by
  inspection only: I built no pose with two curves there.

**N5. Claim 7 (sweep).** My grep was shaped on `incident_faces|faces_at|fv.contains|half_edge_end(..)==|.start ==|null_site(`
over `rest/sectors/join/insert/vtxfac`. It finds what the PR lists, plus the new face-by-vertex-pair
searches below (S1). Two producers of `Locus` exist (`insert.rs:229`, `vtxfac.rs:552`); the rest
are fixtures.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, and Q8 (`rest.rs` read end to end).

- **S1 (Q1, the fix mints a fresh instance) — likely.** The spec retires the "unique common face"
  search: `InFace(F)` was to replace it. But `tangent_face` (`sectors.rs:1027`) *computes* `F` by
  that same search: the faces at the site ∩ the faces at the far end, exactly one.
  `fragment_holding` (`rest.rs:969`) is the same search restricted to a lineage. `mirror_edges`
  (`rest.rs:1060`) keeps the original. That is three spellings of "the face holding both vertices".
- **S2 (Q1) — sure.** `faces_at` (`sectors.rs:1084`) and `incident_faces` (`rest.rs:1276`) are
  near-duplicate "faces around a vertex" walks. They differ in null-face skipping and in ring
  handling.
- **S3 (Q1) — likely.** `join_refusal` (`mod.rs:3155`) re-spells the door sequence: reduce, then
  the `null_pairs.is_empty` exit, then the join scopes. That sequence already exists in
  `ops::through_the_join` (`ops.rs:671`) and `mod.rs:3122`. If production's door changes, the pins
  read a sequence it no longer runs.
- **S4 (Q4, Q5) — sure.** "Two edges that part with their far ends recorded alike are left as they
  read" (`sectors.rs:926`), and the PR's "as before": what *main* did with these poses was build
  them (M1). The sentence is true of the locus and false of the outcome.
- **S5 (Q2) — unsure.** The join's module docs (`join.rs:114`) say the matching criterion reads
  nothing the surgery changes. Nothing enforces that. `section_segments` now pre-computes the
  matching, and `bool_connect` replays it over a second `open_records` state (`join.rs:521`) kept
  in step by hand. The byte-identical batteries are the only evidence.
- **S6 (Q7) — unsure.** "Depth" is recorded-contact rank (`Apart < Boundary < Face`), standing in
  for a geometric fact: which curve lies inside. The justification (`sectors.rs:918-927` plus the
  PR's two-case argument) is longer than the rule. A trim test would decide the tie too.
- **S7 (Q5) — unsure.** `germ_locus` for vertex-on-face sites applies the far-end rule with no
  ranking. A piercing edge that is only tangent but whose far end touches the partner anywhere
  still reads `OnEdge`. That matches main, so it is not a regression, but the promise "a germ only
  tangent to an edge does not lie along it" is narrower in practice.
- **S8 (Q2) — likely.** `Twin::of` (`rest.rs:1138`) now takes the edge by key and ignores `ov`.
  `start: if first == ou {u} else {v}` silently picks `v` when the edge reaches neither end. On the
  A side the twin is B's `OnEdge` edge, whose ends are checked only later, when B realizes its own
  seam.
- **S9 (Q6) — likely.** Acceptance (1) was rewritten from "built through the zip where main builds
  it through the zip" to "by whichever path". That is a narrowing. It is disclosed, but nothing
  schedules or ratifies it.
- **S10 (Q3) — sure.** See m1: the pins assert *who* built the body, not *from what*, so they are
  monotone in the wrong direction for the switch.
- **S11 (Q2) — unsure.** The OnEdge-ends mismatch in `realize_seam` (`rest.rs:934`) is typed
  `JoinDesync`, which the module calls kernel-bug class. By N1 that is right today, but only if
  ranking can never misread.

REVIEW COMPLETE
