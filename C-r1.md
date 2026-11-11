# Fork `shared-vertex-pairing` — designer C, round 1

Harness token count (first report): 189,829.

## For Ev

**Recommendation (likely).** Build all of these ops. Neither class should stay refused. Two changes, one in each layer:

1. **Insertion: pair along the arcs inside the partner piece, read at the shared vertex.** The op stops choosing the pairing.
2. **Finish: drop the shape check on the seam vertex correspondence.** The zip already reads which vertices belong to one cone of the result and splits or fuses them.

Do not build nesting across pairs (the row's option 3). With the pairing above nothing needs it.

Measured with a local probe (pairing switched for the refusing pair, finish check off):

| what ran | builds sound | refuses |
|---|---|---|
| battery today | 4 208 | 982 |
| battery, probe | 5 190 | 0 |
| cleave witness, probe | 6 of 6 ops | 0 |

Every built volume equals the oracle's, and tier 2, 3′ and the certificate pass. That includes the 16 crossing-class lines and the 18 intersections. In the witness, `cube ∪ y` = `y ∪ cube` = 1.0336378, `y ∩ cube` = `cube ∩ y` = 0.0285733, and `cube ∖ y` = 0.9714267 = 1 − ∩.

**Terms.**
- *Vertex pair*: one vertex of A and one of B at the same point.
- *Shared vertex*: a vertex in several crossing vertex pairs, because the other operand has several pieces touching there.
- *Link*: the closed curve a piece's boundary draws on a small sphere round the point.
- *Germ*: a point where the two links cross.
- *Null edge*: the zero-length edge that splits a vertex between two germs. Its *run* is the arc of the link it cuts off.
- *Pairing*: which germs each null edge joins. At four germs there are two.
- *Cone*: one connected piece of the result's boundary at the point. The result has one vertex per cone.

**Premise check.** The row asks "which pairing, when the start rule's runs have no clear way round". That treats the start rule as the default and the refusal as an exception. I read it the other way: the start rule is the fault.

- **The op chooses the pairing, for the finish's sake.** `pairing_start_turns` starts where A's runs lie on the side the op keeps of A. For a union or a subtraction that is A's arcs *outside* its partner piece. The other pieces sit on exactly those arcs, so the runs hold their cuts.
- **The arcs inside the partner piece are always clear.** The other operand's pieces at the point are disjoint, so no other piece's cut lies on them. This is the row's own reading, and it covers the crossing class too.
- **The op is read nowhere else in the insertion.** Its doc says "A's is the one the finish needs". The finish is where the op belongs (it selects what to keep). The cuts should be the same for all three ops.
- **The `JoinDesync` is a whitelist, not an inability.** `setopfinish` lets a kept A vertex correspond to several B vertices only in two listed shapes. A pairing can leave one cone as one vertex in A and two in B, or two in A and one in B. Only the second is listed. With the check off, the zip builds both (table above).
- **The start is then free.** I ran the battery with the opposite of today's start for every pair, and with "A's arcs inside B" for every pair on every op. Both build 5 190 of 5 190, flipping only where a shared vertex's runs were not clear (526 and 54 pairs).

**Ratified text.** None governs this. The topo README has no clause on the insertion's pairing or the finish's correspondence, and `DESIGN.md` has none either. The contract lives in `insert.rs`'s module docs. The start rule came in an implementer commit (`c50f2ff22e6`, 2026-10-04) and its fix pass, two days before the cone split landed (`52cbed0cfa0`, 2026-10-06). So its reason predates the mechanism that removes it. The check dates from the first boolean PR and was widened case by case since. I found no words of yours behind either. The code credits "a pinch is one vertex per cone" to you (PR 4057); I lean on it and did not read that PR.

**The answers as final states.**

*A. Pair inside the partner; the finish takes the correspondence as given (recommended).*
- **Pairing.** Each null edge runs along an arc of the *lead* vertex's link that lies inside the partner piece. The lead is the vertex other crossing pairs share, or A's where neither is shared.
- **What that makes true.** At a shared vertex, every pair's runs are disjoint by construction, not by a search. The reduction no longer reads the op.
- **Finish.** The correspondence is whatever the pair records give. `split_cones` reads the cones. "Conflicting seam vertex correspondence" is retired.
- **Reconcile.** It keeps the tie reading and a dangling null edge nested in another. Turning a run the other way round, and re-nesting a turned run's own runs, should have nothing left to do. Arms that go dead are removed, or become invariants.
- **Still refused.** Two dangling null edges on one segment (its own closed row), and two crossing pairs that share both vertices (see below).
- **Poses that build today.** Same bodies by volume and tiers on the battery. Vertex keys and cone-split lineage rows can differ, since a different vertex is the one split.
- **Reversible.** Yes: it is one rule and one check.

*B. Keep the op's start, nest across pairs (the row's option 3).*
- Covers the 966 laminar lines by argument only; it is unbuilt. The 16 crossing lines stay refused, with no body-level reason.
- Adds a mechanism to the reconcile and keeps the op in the insertion.
- I do not recommend it.

*C. A plus one tree of runs per vertex, across pairs.*
- The general form: today four separate mechanisms each hold part of that tree.
- Needed only if two crossing pairs sharing both vertices are to build. Today that refuses `NonManifoldResult`. Nothing reaches it in the battery, so I would not build for it yet.
- A does not block it.

**Worked example (the cleave witness).** The band crosses the cube's corner four times. Walking the cube's link the germs are g1, g2, g3, g4. The cube's arcs inside the band are g1–g2 and g3–g4. The other two corners sit on the arcs between.
- Today, for `cube ∖ y`: the pairing is g2–g3 and g4–g1, the arcs outside the band. Each holds another corner's cut both ways round, so it refuses.
- Under A: the pairing is g1–g2 and g3–g4. Both runs are clear. The cube's vertex keeps everything outside `y`, and the zip splits it into the result's two cones.
- For `cube ∪ y` the same cut gives one cube vertex against two band vertices. That is the shape the finish check refuses and the zip builds.

**Is a typed refusal acceptable for either class?** No. Both operands are valid bodies, the result is defined, and the kernel builds it once the two faults are out. The refusal is true of the plan, and the plan is the defect.

**Confidence.**
- The refusals are a pairing fault, and the `JoinDesync` is the finish check alone: **sure** on the battery and the witness (measured).
- Removing the check is safe: **likely**. No test pins that message. The topo suite ran 1 232 passed, 3 failed with the probe; one failure reproduces without it, one passes alone, the third I did not rerun. What the check caught in the past I did not trace.
- The lead rule as worded: **likely**. I measured a proxy (below).
- The reconcile's turning arms go dead under A: **unsure**. Argued, not measured.

## For the orchestrator

**What I measured, and how it differs from what I recommend.** The probe is the lane's patch plus two env switches. `SVC_C_RELAX` turns off the `bs.len() > 1 && !shared_cut && !one_ring` refusal in `setopfinish`. `SVC_C_START` picks the start (`flip`, `in`, `out`). The lane's retry still flips a pair's start when the reconcile refuses.
- So what ran is "some A-consecutive start, flipped on refusal", not "consecutive in the lead's walk from its inside arc".
- At four crossings there are only two pairings, so the two agree.
- At six or more, with the shared vertex in B, the lead's inside pairing can be nested in A's walk, which no A-consecutive start gives. The 40 six-crossing lines all built under the proxy; I did not check which pairing each took.
- The real rule needs the plan to pair in the lead's walk order, with today's `b_runs` nesting reading the other solid, whichever that is. I did not build it.

**Runs.**
- Cleave witness, debug: no env passes as pinned; retry alone gives 4 builds and 2 `JoinDesync`; retry plus relax gives 6 builds.
- Battery, release, retry plus relax: 5 190 `OK SOUND`, 0 `ERR`, 998 flips, no volume differing from `want`.
- Battery with `SVC_C_START=in`: 5 190, 0 `ERR`, 54 flips.
- Battery with `SVC_C_START=flip`: 5 190, 0 `ERR`, 526 flips.
- Topo `all`, debug, retry plus relax: 1 232 passed, 3 failed, 15 ignored.
  - `a_stated_wrong_general_row_keeps_its_term_at_f64` fails the same with no env set.
  - `a_turned_half_cap_at_a_tight_k_keeps_its_twin` passes alone, with and without the env. It failed under load.
  - `a_stated_wrong_general_row_keeps_its_term_at_interval` I did not rerun.
  - These look unrelated to the boolean; the first is worth a look on main.

**Not checked.**
- Whether results under A are the same topology as today's on building poses. I compared volume and the tier verdicts only, not face or vertex counts.
- Any battery other than `four_pairs_battery`, and the rest of the sweep suite.
- The reversed-order harness (`with_vertex_pairs_reversed`) under the probe.
- PR 4057 itself. I took the code's attribution.
- The `NonManifoldResult` refusal for two crossing pairs sharing both vertices. Its doc reasons from a single result vertex, which the cone split may have overtaken. It is the one place where pairs' runs can nest across pairs under A. I would file it as its own row rather than fold it in here.

**Brief.** No errors found. One note: the brief says the start is "chosen from A's kept side, for the finish". That is accurate, and it is the thing I recommend changing.

**Process.**
- Three cargo builds and one test run exceeded the 10-minute foreground limit and the harness moved them to the background. I waited for each to finish before going on; nothing is running now.
- The release build took 19 and 34 minutes, the second while the other two designers' builds waited on the slot.
- `/home/user/fork-svc-c-target` is deleted. The clone at `/root/.local/share/cad-work/fork-svc-c/cad` holds the uncommitted probe. Nothing was pushed.

**Files** (all in `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/`): `fork-svc-c-probe.patch`, `fork-svc-c-relax.log`, `fork-svc-c-in.log`, `fork-svc-c-flip.log`, `fork-svc-c-topo-all.log`.
