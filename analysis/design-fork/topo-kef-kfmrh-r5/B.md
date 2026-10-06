# kef/kfmrh round 5: a restatement that certifies but is wrong in kind

## For Ev

**Recommendation (likely): no debug assertion and no caller list for kind. The one restatement function picks the kind itself, using the same rule tier 3 enforces, and certification stays the guard.**
1. **Restate means: the description tier 3 would accept on the edge's faces after the move, as close to the stored one as that allows.** Keep the stored kind wherever the rule admits it. Where it does not, take the kind the rule demands:
   - definitely transverse → `Intersection`;
   - smooth, and the surfaces determine the locus at second order → `TangentIntersection`;
   - smooth and under-determined → a chart image;
   - in band → a typed refusal (D4 ¶3).

   The rule is `must_carry_over_edge` plus the transverse rule. Today tier 3 check 4 spells it out by hand; it moves into one predicate that both call.
2. **Every restated description is certified on the surfaces it now names**, as choice 3 (A) already says.
3. **The vouch stays a separate step.** A restatement answers only for the surfaces a description names. The other side is the vouch's question (below).

Callers then list only what the door cannot derive: new carriers and new declarations. Kind drops out of choice 3's caller list.

**Premise correction (sure): the residual as worded cannot happen.** Certification checks the kind of an intrinsic description.
- Certifying an `Intersection` requires the two named surfaces to be definitely transverse at every interior sample. Otherwise it fails with `NotTransverse`, or escalates.
- Certifying a `TangentIntersection` requires parallel normals, a definitely positive second-order margin, and the hull and tube bounds.

So if a twin carries an `Intersection` onto a pair the move made tangent, the twin's own certification refuses it, typed. Nothing is silent. An intrinsic kind is a fact about the surfaces the description names plus its carrier, and certification proves that fact. With choice 2 (a description names exactly what its faces wear, at tier 1), intrinsic kind is already structural.

**Terms.**
- *Named side*: a face whose surface the description names.
- *Unnamed side*: the face a chart image does not name. A chart image names one surface; intrinsic descriptions name both.
- *Admissible*: the description kind tier 3 accepts for this edge at rest (the prefer-intrinsic rule).

**Where the problem really is (likely).** Certification proves facts about named surfaces only. Two facts concern the unnamed side, and certification cannot see either:
- **(a) Containment.** Take an edge described as an image on K3's chart, whose other face moves from K2 onto K1. Restating it changes nothing and certifies trivially. Only the vouch asks whether the edge lies on K1: on a plane it reads the residuals, on a curved chart it refuses (`RechartUnvouched`).
  - If a twin took "restated and certified" to mean "vouched", this would be a real silent wrong answer: a face bounded by an edge that does not lie on its surface.
  - So under A the vouch must stay a separate step.
- **(b) Admissibility.** Whether a chart image is allowed depends on the unnamed side's surface, through the dihedral.
  - Tier 3 checks it at rest, and nothing else does.
  - No composing door runs tier 3 when it closes. The boolean's gate is tiers 1 and 2.
  - So a twin can return a body that fails tier 3, and no door notices. **This is the residual worth closing**, and choosing the kind in the restatement closes it.

**Mid-operation readers of kind (sure for the list, likely for the conclusion).**
- `mate_surface` reads an `Intersection`'s second surface.
- The boolean's tangency source (`tangent_struts`) reads `TangentIntersection`.
- The pcurve-row mint reads a chart image on the face's own chart.
- The boolean describe pass and the split's finish re-derive kind themselves, from the dihedral.

None of these computes a wrong answer from a certified restatement, because each reads only what certification proved. A chart image where an intrinsic description is owed loses information (a tangency strut is not seen) but states nothing false. Readers already tolerate that, because tier 3 accepts derived chart images on tangent edges at rest.

**Ratified text that contradicts the code (sure that they disagree; unsure which is right).**
- **D2's prefer-intrinsic paragraph** says "a declared conventional description is exempt by its own declaration". It opens with "wherever an intrinsic description is certifiable, it *is* the stored description".
- **The code** (`validate.rs`, check 4) does the reverse. It refuses `TransverseNotIntrinsic` / `TangentNotIntrinsic` only when the authority is *declared*, and exempts derived chart images. A tier-3 test pins this ("prefer-intrinsic names every declared transverse chord").
- **Why it matters here:** the admissibility rule is what the restatement derives from, so whichever of the two stands decides which chart images a move must turn intrinsic. I could not trace who wrote the D2 sentence (the checkout is shallow). The design above holds either way, but the rule has to be one thing.

**The options as final states.**
- **O1. The restatement chooses the kind (recommended).**
  - **Makes true:** no restated description is inadmissible on its post-move pair, and no caller has to remember kind. The same rule runs in the restater, the boolean describe pass, the split's finish and tier 3, so what is demanded and what is stored are one set.
  - **Leaves open:** a caller-listed spec can still be inadmissible, caught at rest. That is any `set_edge_curve` caller's exposure today, not the restatement's.
  - **Costs:** one dihedral reading per restated chart image (7 stations). Intrinsic kinds already pay for this in certification.
  - **Subtle case:** a *declared* chart image on a pair the move made transverse. Under the code's rule it must become an `Intersection`, which has no slot for a declaration. So the door either drops the declaration or refuses. I lean refuse: dropping a user's declaration is not derivable.
- **O2. The restatement keeps the kind verbatim, and the attach layer refuses an inadmissible kind (typed) after certifying.**
  - Also never silent, and it covers caller-listed specs too.
  - But it refuses where the door could have derived the answer, so callers are back to listing edges by kind: the ceremony A removes.
  - A close second. It is reversible toward O1, and O1 can add it later for listed specs.
- **O3. An expensive `debug_assert`** (below). A detector, not a guard.
- **Status quo A** (callers list kind changes, tier 3 at rest): wrong state (b) leaves composing doors unnoticed.
- **Rejected: kind not stored, derived when read.** That is C again. The kind is what the certificate proved, so deriving it at each read means re-certifying at each read.

**The `debug_assert` Ev asked about.**
- **What it checks:** for every edge a describing door restated, right after the write, the per-edge parts of tier 3:
  - check 2: re-certify on the faces' surfaces, plus adjacency;
  - check 4: the dihedral, with prefer-intrinsic and the material arm;
  - check 5: planar containment on both sides.
- **Where:** each describing door's close, beside `assert_tier1_postcondition`, over the restated set only. The variant over every edge at the outermost surgery-scope close is tier 3's checks 2, 4 and 5 once per composing door, O(edges) full re-certification: the truly expensive one.
- **Cost:** roughly twice the twin's own certification work on those edges, because it re-runs what the door just did, plus 7 dihedral classifications per edge (and the jets on smooth edges). Not measured.
- **Where it runs today:** `[profile.release] debug-assertions = true` (`Cargo.toml`, marked PRE-PUBLISH: it comes out before publishing). The only CI leg without it is `topo-release` (`CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false`, corrupt-input suites only). So "skipped in production" today means it runs everywhere, demos, wheels and benches included. After publishing it runs nowhere a user is.
- **Would catch:** (b) and a planar (a), early and attributed to the door.
- **Would not catch:**
  - (a) on a curved unnamed side: tier 3 does not check that either (#638);
  - anything outside the restated set;
  - anything in a published build.
- **Compared:** for intrinsic kinds it duplicates certification, which already guards in every build. For admissibility it is a second spelling of the rule O1 makes the restater apply. It detects the state rather than making it unwritable, so as a final state it is strictly weaker than O1 or O2. It would only earn its place as the agreed scope-close check, and choice 2 already moved that check into tier 1.

**Reversibility.** O1 and O2 are local to the restatement function and the attach layer. Moving from O1 to O2 means deleting the derivation and adding a refusal.

**Confidence:**
- intrinsic kind is certified: **sure**;
- (a) and (b) are the real residuals: **likely**;
- O1 over O2: **likely**;
- O3 weaker as a final state: **sure**.

## For the orchestrator

- **Read-only, nothing run.** The checkout is shallow, so `git log -S` on D2's "exempt by its own declaration" returns only STATUS render commits. Trace it with full history before Ev rules. The contradiction is a real question for Ev, and it decides O1's rule.
- **Kind derivation needs one rule first.** `dihedral.rs` (`tangent_second_order` docs) names tier 3's check-4 walk and `contact_verify` as two hand-rolled siblings of the must-carry rule (issue 1439). O1 needs at least check 4 folded into a shared predicate. Do not touch `contact_verify`'s `ContactClass` side: it is HOLD (D10).
- **Restaters and kind.** `carried_spec` (attach), `remap_description` (`replace_face`, reached from `shell::loop_rekeyed`) and the graft's remap in `boolean/combine.rs` all keep kind verbatim. Only `offset_restate::held_neighbour_image` chooses a kind, and it does so by rule, without reading the dihedral.
- **Possible defect (unverified).** `offset_restate::held_neighbour_image` maps a *declared* chart image to a declared chart image on the moving side of what is usually a transverse section pair. Under the code's check-4 rule that is `TransverseNotIntrinsic` at rest. Worth a fixture; file under the offset doors' owner if it reproduces.
- **Seam flag (unsure).** `seam: true` is certified (half-plane and side). `seam: false` on an edge that lies on the destination chart's seam is checked nowhere I found. The restaters copy the flag verbatim, and `mesh::memo` and `chart_iso` read it. Probably unreachable through the kills; not chased.
- **Error in a round-3 report:** "builds without debug assertions (the benches)". `[profile.bench]` inherits release, which carries `debug-assertions = true`.
- **For whoever builds A:** the twin must still run `unvouched` on edges whose description names only the unmoved side. `carried_redescriptions` already leaves those edges out of its carry set, so this is about keeping that split when the twin starts calling the restater itself.
- **HOLD respected:** nothing here uses declared pairs, `ContactClass`, D10 placement or the other held items. Tier 3's `ContactMark` is read-only background.
