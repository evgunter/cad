(4c implementer) Measurement note (a). The container survived, and nothing is lost.

**(a) Bored ball on main 54e6a5c5b8 (no walk).** Radius 1.05, bore 1, placed obliquely, seam unsplit / split at 0.5 / split at five points, δ 3, 1, 0.3, 0.1, 0.05, 0.04, 0.035, 0.03, default ε. Triangle totals; the old branch head 7a6f5bfe6e (walk + its cut) follows the slash.
- **Unsplit (rectangle lane), both trees agree from the threshold down:**
  - sign +1: `OverusedSide` at δ 3, 1, 0.3 (old main and the old head: `Ok` + NonManifoldEdge count 4). It meshes at δ 0.1 to 0.03 (132, 186, 210, 296, 320).
  - sign −1: `OverusedSide` at δ 3, 1, 0.3, 0.1 (old: Ok non-manifold). It meshes at δ 0.05 to 0.03 (90, 102, 108, 114).
  - So the census turns old main's silent non-manifold `Ok` into a typed refusal on exactly 7 cells. 7198's "the rectangle lane needs no rule" misses this case: two rim chains held by ONE neighbour (sphere/torus round the bore). Its own argument (one chain on one iso line) does not cover two chains.
- **Split @0.5, main / old head:**
  - sign +1: 62/64, 62/64, 102/104, 152/220, 216/310, 242/350, 330/298, 358/322.
  - sign −1: 62/64, 62/64, 62/64, 64/88, 120/120, 134/136, 142/110, 152/116.
- **Split @5 points:** the same shape. sign +1: 70/72, 70/72, 110/112, 160/228, 224/318, 250/358, 338/306, 366/330. sign −1: 70/72 ×3, 72/96, 128/128, 142/144, 150/118, 160/124.
- **Every split cell meshes on main** (chart lane, CDT + 7198's rule; `UnclearedSharedSide` never; `OverusedSide` never).
- Main is coarser than the old walk at δ 0.1 to 0.04, where the old cut rule over-cuts (sign +1 δ 0.04: 242 against 350). Main is denser at δ ≤ 0.035, where the walk's δ target beats the chart lane's headroom (sign +1 δ 0.03: 358 against 322; sign −1: 152 against 116).

**Router defect:** main has no variant. Its "one router-defect refusal" (1353d24225) is `MissingEntity { what: "… (router defect)" }`, with no face key. Mine is `RouterDefect { face, what }` with its own tag, census entry and Python projection. What differs: main's names no face and is sorted under MissingEntity, as "a key no live face holds". My plan, unless you say otherwise: fold mine into main's form, so `MissingEntity { what: "a face the route walks … (router defect)" }`, and delete my variant, tag, census entry and projection. That gives one refusal and drops the face key. The other choice is to keep my variant and move main's two sites onto it, which touches main's code. Say if you want that.

Next: the merge (9 files), then (b), the walk with its cut off.
