(4c implementer) **6e9fa7091b is pushed: one commit on a020a3edca.** `may_make` reads no ε.

**The site.** `may_make(neighbour, a, b, delta)` keeps a side uncut only where `|m − foot(m)| > δ·(1 + ALLOWANCE_SHARE) + 2·cert::rounding_allowance(cert::coordinate_scale([a, b, foot]))`. Everything else cuts.
- The doc says why two allowances: one covers the certificate's allowance on the neighbour's computed bound, and one covers this distance's own rounding. That rounding is a chain of the certificate's kind (midpoint, closed-form foot, difference, norm), which the certificate's 8 ulps bound with room (`cert::ROUNDING_ULPS`).
- The planar-neighbour sentence is placed.
- **The bare-rounding gate passes this form:** "bare-rounding-allowlist OK … at their listed counts".
- **One residual I state at the site rather than claim.** The allowance is read at the magnitude of the points this test reads: the side's two ends and the foot. The neighbour's verdict also reads its triangle's third corner and its chart's anchor (`cert::position_scale`). Where those stand farther from the origin, its allowance is larger than the one read here, by at most δ·ALLOWANCE_SHARE, past which the verdict refuses the triangle. A side excluded on that difference is the census's to refuse. No row reaches it. Nothing rests on Kε any more.

**What follows from it:**
- `k_stats::decide` has left `may_make`; the `Margin`, `Sign` and `k_stats` imports are gone from trimmed.rs; `walk_face` no longer passes `tol.band`.
- `SizingTols::band`'s doc is main's text again: three consumers, and "mesh decides nothing against it". The route's apex read takes the route's band, not this field.
- `FaceRoute`'s doc: the walk records nothing of its own.
- **The K row re-pinned:**
  - the frustum still has 36 apex reads and the same stream (0xd351_00ba_acb8_3421);
  - the bored ball at δ 0.1 has 0 apex reads and 0 reach decisions; its stream is now 0xb042_5d70_935d_9752, the same at all three ε.
- **`a_side_whose_reach_the_band_cannot_decide_is_cut` is gone.** In its place: `the_walks_cut_is_the_same_at_every_eps`. The sign +1 bored ball, seam split at 0.5, at δ 0.036955 cuts 0 sides (digest 0x5fdc_3103_244e_21ec) and at δ 0.036957 cuts 36 (digest 0x5942_fa6e_f91c_0eb2). Both pins are identical at ε 1e-9, 1e-6 and 1e-12. The threshold's δ is stated once (≈ 0.036 955 758).
- **The bored-ball rows' pinned cut counts did not move** at any δ they run, 1.05 and 1.3 balls, three ε. The 218-cell row did not move either.
- Docs: the bored-ball row, ruled_walk's module doc, the four-use bore item's cure line, and the filed `a-walked-sides-reach-exclusion…`. Its band and ε_min paragraph is restated: the first form's soundness gap and its ε dependence are named as gone, and the residual above is stated.

**Sweep for ε choosing between two meshes:**
- **`walkable`'s undecided apex.** The route's `at_a_pole` and the structural lane's `singular_vertices` are one function on one vertex at one band. So where the route is undecided the lane is too, and it refuses `SphereTrimShape { SingularUndecided }`, typed, on every cell: never a different mesh. On this unit's earlier head, the 81-gap sweep at the default ε refused at exactly gap 1e-8, on both δ, and meshed elsewhere. That is ε moving a cell between a mesh and a refusal, which the page allows.
- **One case where ε does choose between two meshes, which this unit did not introduce.** Where the band decides a split vertex ON the apex at one ε and OFF it at another (a split nearer the apex than 10ε at 1e-6 but farther than that at 1e-12), the walk meshes it differently: as an apex row, or as a rim point. Main's structural lane makes the same choice for every cone face in `singular_vertices`: the kernel's coincidence decision is what ε is for. I did not change it. Say if you want it filed.
- Nothing else in the unit reads ε. `cert::FaceVerdict` reads it only for the `RoundingUnresolved` refusal, a mesh against a refusal.

**Runs on 6e9fa7091b:**
- `bash scripts/run-gates.sh` rc=0; fmt clean; clippy (mesh, all-targets, all-features) clean; work lint ok.
- `mesh` whole with the slow set: **546/546 at the default ε, 1e-6 and 1e-12.**
- **Mutants touching the cut** (split_rulings + route_rows):
  - never: 3 red (the bored ball, the 218-cell row, the ε row), at the default ε and 1e-6;
  - always: 2 red (the ε row, the bored ball);
  - an ε read put back (`+ 10·ε` on the reach): green at the default ε, where 10ε is below the cell's spacing, and **red at 1e-6** on `the_walks_cut_is_the_same_at_every_eps`.

**What moved:** no mesh at any δ a pinned row runs. Only the cut threshold, by the band's undecided width: about 8e-9 in δ at the default ε, 8e-6 at 1e-6, and nothing at 1e-12.
