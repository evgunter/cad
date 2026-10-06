# kef / kfmrh round 5: a derived restatement that certifies but is wrong in kind

## For Ev

**Recommendation (likely): the restatement derives each moved edge's kind from the surfaces its faces wear after the move, through the one predicate tier 3 already applies at rest. It never copies the stored kind. No new assertion.** Then a restatement cannot be "wrong in kind" because its kind is the predicate's answer, the same answer tier 3 re-derives and certification meters: one computation, nothing kept in step with it. The `debug_assert` you asked about is that same computation run a second time, after the write, in some builds; I weigh it in §3 and recommend against it as a final state.

**Terms.** *Kind*: which arm of `EdgeDescription` an edge carries: `Intersection` (transverse pair), `TangentIntersection` (first-order tangent pair, second-order separated), `Chart` (an image in one face's chart, with a seam flag), `Scaffold` (transient). *Certification*: `EdgeCurve::certify`, the sampled checks a description passes before it is stored. *Restatement*: a stored description re-expressed after a move and re-certified. *Demanded kind*: what the dihedral between the edge's two surfaces says the kind must be, tier 3's check 4. *Chartless*: a face wearing the "no chart yet" placeholder.

### 1. Premise check: the residual the brief names does not exist; the real gap is one shelf over

- **Certifying an intrinsic description is already a dihedral check** (sure). `certify.rs` runs `wedge_decided` at the seven interior samples of every `Intersection` and refuses `NotTransverse` where the pair reads smooth, and escalates in band; `TangentIntersection` refuses a non-parallel normal (`TangentParallel`) and a collapsed second-order margin. So "`Intersection` carried onto a pair the move made tangent" is a typed refusal at the twin (`RechartFalsifies`), listed or not. Under choice 3 = A, a caller that forgets such an edge gets a refusal, never a wrong answer. The blend is the live instance: wall–cap `Intersection` restated onto the fillet refuses, and the blend must list `TangentIntersection`.
- **What certification cannot see** (sure): adjacency, which choices 1 and 2 settle; and a `Chart`'s kind. A chart image certifies "this carrier lies on chart S" and reads no second surface. Carried by a key swap onto a pair that is now definitely transverse, or jet-determinate tangent, it still certifies. That is the one wrong-in-kind description a restatement can produce: true as far as it goes, weaker than the geometry, and the description D2's prefer-intrinsic rule says must not rest.
- **It is silent end to end today** (sure). Tier 3's `TransverseNotIntrinsic` and `TangentNotIntrinsic` fire only when the edge's authority is *declared* (`validate.rs`, check 4). A derived chart image on a definitely-transverse edge is tier-3 valid, pinned by sweep's `a_rim_in_the_caps_chart_is_restated_as_the_section_with_the_minted_wall`: a cylinder's cap rim re-described as an image in the cap's chart passes `validate_geometric`. D2's text says the opposite; §4.
- **Mid-operation readers of kind do not compute on a kind-weak chart** (likely). `mate_surface` (`pcurves.rs`) answers `None` for anything but `Intersection`, so the fitted pcurve lane refuses typed. `tangent_struts` (`boolean/mod.rs`) reads only `TangentIntersection`, so a tangent edge resting as a chart is not a tangency source: the boolean loses a one-sided cover it was owed and keeps a typed frontier. The citing checks in `edge_join.rs`, `merge_faces_kept_rows.rs`, the boolean's describe pass and the split finish treat a description that does not cite the pair as "restate", never as a fact. So the cost of the gap is lost strength and a body resting with a description D2 forbids, not a wrong geometric answer.
- **Where the copy really is.** Every restater copies the stored kind: `carried_spec` (`attach.rs`), `remap_description` (`replace_face.rs`), `loop_rekeyed` (`shell.rs`), `offset_restate::restate`. The kind is a function of adjacency and geometry, kept as a second copy by hand. That is the B+ shape one level down, and it is what to remove.

### 2. The design: derive the kind

One function in `geom-brep` beside `must_carry_over_edge`, call it `demanded_kind(s1, s2, carrier, t0, t1, extent, band)`: tier 3 check 4's per-edge reading given a name. It answers `AllTransverse`, `JetDeterminate`, `UnderDetermined`, `Mixed` (transverse at some stations, smooth at others), or `InBand` with the station's escalation.

The one restatement function (the describing twins, `set_face_surfaces_describing`, the shell's rim glue, the offset doors, the boolean's describe pass, the split finish) states each moved edge from the surfaces its faces wear after the move:

- `AllTransverse` → `Intersection(after-plus, after-minus, witness = carrier mid)`.
- `JetDeterminate` → `TangentIntersection`, same pair and witness.
- `UnderDetermined` or `Mixed` → `Chart` on a side the edge lies on, image kept iff `same_chart`, else derived at the door; the seam flag travels, and a seam whose two sides are no longer one chart is refused.
- `InBand` → typed refusal (D4 ¶3, as the boolean's `seam_refusal` does today).
- A chartless or spline side: no dihedral can be read, so `Chart` on the real side, image as above; tier 3 exempts the same edges.
- `Scaffold` and null edges: nothing, as now.

Carrier, interval and authority travel verbatim. Then certify, as now. A caller lists only what the body cannot derive: a spline chart's image. Nothing else I can find. That is exactly the PR 2527 ruling as A read it, "a carrier the door cannot derive", so **choice 3 = A**, with the caller's list shrunk from "edges whose kind or geometry changes" to "what cannot be derived". A listed spec whose kind is not the demanded kind is refused, so the listed path is as safe as the derived one.

**What it makes true.** The stored kind is never an input to a restatement, so a copied kind cannot be wrong. The kind of every restated edge equals the predicate's answer, which tier 3 re-asks at rest and certification meters, so there is no second notion to drift. The boolean's `describe_edges` is already this function on its worklist (classify, must-carry, then `Intersection` / `TangentIntersection` / `Chart`); the design moves it to the attach layer and the boolean's seam half becomes the describing kill's.

**Worked examples.** Blend: wall–cap `Intersection`, the cap's loop moved onto the fillet. Key swap gives `Intersection(fillet, cap)`, refused `NotTransverse`, and the blend must know to list `TangentIntersection`. Derived: `JetDeterminate`, `TangentIntersection`, certified, nothing listed. Coplanar merge: `Intersection(absorbed, wall)` → `AllTransverse` → `Intersection(kept, wall)`, as the key swap gives. The kept face against a coplanar neighbour outside the group: `UnderDetermined` → `Chart(kept)`, where the key swap carries an `Intersection` that fails `NotTransverse`. A second edge the dying face shares with the survivor: both sides become the survivor's key; `UnderDetermined` → `Chart`, where the key swap carries `Intersection(k, k)` and refuses.

**Cost.** Seven dihedral classifications per moved edge, which certifying an `Intersection` already pays; new only for chart-described edges. **Reversible**: one function; removing the derivation returns to the key swap. **What it leaves possible that should not be**: a chart image on a chartless or spline side is still asked nothing on that side until rest (today's gap, not widened).

### 3. The `debug_assert`, weighed

- **What it checks**: for every edge a door moved, the demanded kind against the stored kind, after the write; tier 3's prefer-intrinsic rule extended to derived authority and run early. **Where**: the twin's postcondition, or the outermost surgery scope's close (`surgery.rs`, beside the tier-1 sweep).
- **Cost**: the same seven classifications as §2's derivation, run a second time per moved edge; at scope close, check 4 over the whole body, O(edges).
- **Catches**: the chart-on-transverse case, and a listed chart of the wrong kind.
- **Does not catch**: anything in a build without assertions. `[profile.release]` turns them on today, so CI's release legs check; the benches turn them off, and the stanza itself says it comes out before publishing. And nothing before it runs: the description is written and readable by the rest of the composing door first, which is the window you asked about.
- **As a final state**: §2's computation run as a check instead of as the source, two computations of one fact compared afterwards. That is runtime discipline where §2 is definition. With §2 built, it checks a tautology on derived edges. The standing at-rest check belongs in tier 3 (§4), which already runs the classification once per edge. **Not recommended** (likely).

### 4. Ratified text: D2's prefer-intrinsic sentence and tier 3 disagree

D2 reads: "At rest, every definitely-transverse edge must carry `Intersection` … The check reads the edge's authority record, so a declared conventional description is exempt by its own declaration." The code refuses only a *declared* one and passes every derived chart. PCURVE-UNIFY's ratification says the record "replac[es] `MappedCurve`'s negative space", which is the code's reading, so the D2 sentence is the drift. Provenance: this is a shallow checkout and `git log -S` finds nothing; I could not see which words you wrote.

I lean (likely) to D2's headline, "an unenforced preference drifts silently": the rule reaches derived descriptions too, the declared case stays refused, the sentence is corrected and tier 3 gains the derived arm. The sweep already upgrades its rims and struts to intrinsic through `set_edge_curve`, so I expect few fixtures to move (unsure on the count). Under §2 this decides nothing for restated edges, which are intrinsic wherever demanded either way; it decides what every other constructor may leave at rest.

**Alternative considered**: one intrinsic arm with the order in the certificate. Rejected: certification already refuses a mismatched intrinsic kind, so the split costs no wrong-answer path, and collapsing edits D2's listing for nothing.

**Confidence.** §1 first three bullets: sure. Readers: likely. §2 derive-the-kind over copy-the-kind: likely. §3 against the assertion as final state: likely. §4 which side drifted: likely; which way to rule: likely.

## For the orchestrator

- **Brief error.** Residual (c)'s example is refused by certification (`certify.rs`, `Transversality` at interior samples, `NotTransverse`; `TangentParallel`). The wrong-kind path that exists is a chart image where an intrinsic description is demanded, and tier 3 does not catch it for derived authority.
- **Defect to file** (I am read-only): `docs/DESIGN.md` D2's authority sentence contradicts `validate.rs` check 4; the sweep test named in §1 pins the code. Owner: whoever owns D2's prose; it needs Ev's word on which authority the rule reaches.
- **Three spellings of one classification.** `validate.rs` check 4 (inline `all_transverse` / `all_smooth`, mixed exempt), `geom_brep::must_carry_over_edge` (answers `Transverse` if any station is transverse, so a mixed edge would be demanded `Intersection` and then fail `NotTransverse`), and `boolean::ops::describe_edges` (classifies at the witness only, then must-carry). §2's `demanded_kind` is the one home; the mixed reading must be tier 3's (conventional), not must-carry's.
- **Chartless side.** The placeholder is a poisoned NURBS net (your round-4 finding); `classify_dihedral` on it poisons. The restater must test for the placeholder, not read the escalation as "in band".
- **D10 HOLD**: nothing here uses declared contact, placement or params.
- **Assumed**: choices 1 and 2 land as decided; the twin certifies in its plan phase before any write, as `set_face_surfaces_describing` does, so no reader sees an uncertified restatement. **Not checked**: whether any production fixture rests a derived chart on a definitely-transverse edge (needs a run with tier 3's derived arm added); whether the blend's `kef_minted` faces are the chartless class.
