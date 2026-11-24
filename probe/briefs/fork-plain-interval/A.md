Designer report: what a leaf at plain `Interval` is (item `a-chains-link-refuses-at-an-edges-stored-curve-with-the-box-arm-off`)

## For Ev

**Recommendation (likely).** A leaf at plain `Interval` is what it is at every certifying scalar: a finished body, tier 3, every enclosure containing its true value at every point of the box. It needs no meaning of its own, and the switch retires at plain `Interval` the same way it does at `Sym`. The chain's link does not refuse because plain `Interval` lacks a symbolic tier. It refuses because the `⊔` graft (the door that merges a product's placed copies into one body) finishes the merged body by re-deriving, from values, certificates that its finished, moved members already carry. Fix it where it starts. The finish reads every carried certificate whose support is unchanged, including an edge's endpoint checks and a planar face's pcurve incidence checks. It derives only what the merge adds: questions between members, and the aggregate's own shell roles. Then the chain certifies at plain `Interval` with tier 3 run, and the charter holds at that scalar with no exception.

Terms:
- **Support:** the data a certificate is a claim about. For an edge: its curve, its two vertex points and its two faces' surfaces.
- **Carried:** a certificate is read rather than re-derived because its support is bit-identical to the support it was certified over, or is that support's image under a certified rigid motion.
- **Identity:** a margin that is zero in exact arithmetic for every parameter value, such as "the carrier's start is the vertex".
- **Box scalars:** `Interval` and `Sym<Interval>`. `Sym<Interval>` carries each value's expression beside its enclosure.

**What each box scalar's answer claims.**
- Plain `Interval`: containment. A definite sign is a theorem over the box. A margin that encloses zero only says the two values are close; it never proves they are the same. Over a box of width w, an identity's margin encloses as `[0, c·w]`. It decides only when w is below about ε/c.
- `Sym<Interval>`: the same numeric channel, bit for bit. In addition, a margin whose expression is identically zero decides `Zero` as a theorem (E12). It is the only box scalar that can tell an identity from a coincidence.
- So plain `Interval` can prove an identity over a macroscopic box only by never asking it. It doesn't have to ask: the identity was decided once, where the door minted the edge. From then on the certificate is a fact about its support ([intrinsic.absolute]: "decided once … nothing re-asks the question from values").

**Premise check (likely; the face and edge counts were matched, not traced).**
- The refusing body is not "the link". It has 18 faces, 20 vertices and 30 edges in four shells: the bar (6, 8, 12) plus three cylinders (4, 6, 4 each). That is the product's merged graft of its placed members, finished once (`T::leaf(fused)` in `eval/wire.rs`).
- 24 edges refuse. That is exactly the bar's 12 edges plus the two pins' 6 each: every member the joint box moves. The one cylinder the joint does not move (the placed target, I infer) keeps its 6 edges, and they pass.
- Each member was finished at the origin, where the box does not touch it, so its endpoint checks are tight there. It was then moved by a rigid motion. `FinishedBody::moved` already carries every certificate through a motion and derives none.
- The graft maps the moved data and runs tier 3 again from scratch. Check 2 at rest runs every check again (`policy_lane::certifying_reattach`: "every check run again, the edge's locus certificate read where its support is unchanged"). It compares the moved carrier's start with the moved vertex: two enclosures of one real point, each as wide as the box. At `f64` that is only redundant work (the cost row's "check 2 repeats, 15% of a slab finish"). At `Sym<Interval>` the algebra proves again what the motion already guaranteed. At plain `Interval` nothing can.
- So this is not a question about the charter. It is the finish failing to meet the rule `[one-way.places]` already states: "a certificate is a fact about its support and is read wherever that support is bit-identical and derived wherever it is not … an edge's or pcurve row's certificate is carried by every door that keeps its support."
- With the switch on, the chain's "CERTIFIES" at plain `Interval` today is tier 2 alone. No body is finished at `Interval` anywhere in the tour. The drive's "what a certified leaf has not had" paragraph says so.

**The final state recommended.**
1. The finish reads a carried certificate wherever its support is unchanged. That covers every family, not only the embedding and locus facts it carries today: the endpoint checks and the pcurve incidence checks too.
2. The graft hands each member over as its `FinishedBody::moved` image, bit-identical, so step 1 applies. The finish asks only the questions between members.
3. Re-deriving a certificate over unchanged support is row 5 of [failure.taxonomy] (a kernel bug only re-derivation can detect). It becomes a `debug_assert` at the point scalars and nothing at the box scalars, where re-deriving cannot succeed ([simple.checks]).

Consequences:
- The chain certifies at both box scalars with tier 3 run, and the switch can retire at plain `Interval`.
- The same carrying removes part of the cost row's check-2 repeat at every scalar.
- Reversible: re-deriving can always be switched back on.

**What stays hard, honestly.** Where a door first mints an identity over a box that moves its construction (an extrude of a profile whose corners are parameters), plain `Interval` can only bound it. It escalates, the drive bisects, and the leaf ends as `Budget`. That is E12's ceiling ("correct and useless" over a macroscopic box). It is a true answer and needs no special case. "Every door finishes its own result at every scalar" means here: the finish runs, and its refusal is the scalar's limit, not something to hide.

**Should a drive replay at plain `Interval` at all? (secondary; unsure; your call).**
- No shipped code drives a box at plain `Interval`. The tier-off dial (`SymbolicDials::off`, `LeafLane::Numeric`) is built only by tests (the m10 `*_interval` rows) and by the drive's own word-for-word test. The tour's chain study builds its own plain-`Interval` leaf for a column of its table.
- `pncad::analysis::clearances_at_nominal`, which the viewer uses, evaluates at `Interval` over no box: certified rounding at the nominal point. That is a real need and this change does not touch it.
- Over a box, plain `Interval` certifies a subset of what `Sym<Interval>` certifies (same numeric channel), at about 1/20 of the cost.
- Options:
  - (a) Keep it as a named cheap lane: "certifies where no identity is first decided over a moving construction".
  - (b) Retire the dial. The drive replays only at `Sym<Interval>`, and the tests and the tour's column are re-cut.
- I lean (b) under [simple.checks]: a shipped dial that only tests and a demo read. Choose (a) if a cheaper, weaker certifier is something you want in the product.

**Rejected:**
- Keeping the switch at plain `Interval` only. That is the switch reborn, with a tier-2 body reported as certified.
- Provenance tokens for the identity (E12's reserve). A second way to decide what reading the carried certificate already decides ([one-way]).
- Excluding plain `Interval` from box replays by its type bound. Once carrying is fixed it has a true sentence over a box, so the bound would exclude a meaning.
- Accepting the refusal. It refuses a fact the kernel has already proved.

**Ratified text.**
- None needs to change in substance: `[one-way.places]` already says it, and the code falls short of it.
- The DESIGN exception for the switch goes with the switch, as planned.
- E6's leaf protocol still says "replay at `T = Interval`". E12 made `Sym<Interval>` the default; one word, if you want it in sync.

Confidence: recommendation likely. Graft identification likely (counted, not traced). Carrying is sound under an isometry: sure, since enclosures of the images contain the real images and the claim "within ε" is preserved by any isometry. The secondary question: unsure.

## For the orchestrator

- **Not checked:**
  - That the graft's mapped data is bit-identical to `member.moved(placement)`'s. Both seem to go through `moved_body`, but I did not trace `topo::boolean::graft_placed`.
  - Whether check 2's "locus certificate read where its support is unchanged" already has a support comparison that can be extended to the endpoint checks.
  - That the fourth cylinder is the target. `place_in_world(target)` in `demos/tour/src/chain.rs` suggests it; confirm with `BOXFINISH_TRACE` by printing the 6 passing edges' member.
- **The probe's box arm off is neither the switch-on tree nor the planned final state.**
  - With `Leaf = AtRestBody` at `Interval`, `LeafBody::moved_by` for `AtRestBody` (`topo/src/finish.rs`) re-finishes every moved leaf (`T::leaf(moved_body(..))`). The deletion row removes that.
  - The deletion row's "the `⊔` graft's `T::leaf` end becomes `T::finish`" would reproduce this refusal as written. Re-word that bullet to a finish that carries its members' certificates, and give the carrying its own row (topo, check 2 and the pcurve incidence checks). The chain item becomes `blocked_on` it.
- **Off my question:** the `Sym<Interval>` refusal on check 7's volume for the same body is probably the same class, with the width question not mine. `FinishedBody::moved` carries check 7's kept certificate, and the graft re-reads the union's volume from values instead of composing the members'. Worth one line on the volume row rather than a new design.
- **Provenance:** `git log -S` on the `[one-way.places]` phrases and on `SymbolicDials::off` turned up only merge and graft commits (a shallow checkout), so I could not establish whether Ev wrote them. The switch itself is Ev's (PR 6371, cited in `props.rs`).
- **Possible defect, not filed:**
  - `demos/tour/src/chaintol.rs` asserts that both lanes "certify the whole study". With the switch on, plain `Interval` certifies on tier 2 only, so the header overstates what it shows.
  - The demo also builds its own plain-`Interval` leaf rather than going through `eval::replay_leaf`, which that function's doc says is the one door.
