# Your review: PR 6523 (`splice/spiric-on-cap-plane`), a single FULL review

Frozen head: `6d9a323f89268c082faa2ad784f05c32d0ac8f00`. Item: `work/splice/a-section-spiric-on-a-cap-plane-reads-unread-against-the-wall-it-ends-on.md` (P1). Evidence branch: `splice/review-6523`.

The tilted rod's β 0.7 union stopped at `"unread rider"`: a section spiric minted in the rod's cap plane, asked against the rod's cylinder wall, which holds its two ends, read `CarrierUnread`. The PR adds one arm to `slit::conic_or_line_on`, `(Spiric, Sphere | Cylinder | Cone) → slit::spiric_on_quadric`. It reads `Placed::Off` or `Unread`, never `On`. The certificate is a degree bound:
- in the two-oval regime `0 < |d| < R − r`, decided by the arm itself (`bool_rider_spiric_regime`), a spiric section is an irreducible quartic (the PR argues it at the site);
- a sphere, cylinder or cone meets the spiric's plane in a curve of degree ≤ 2;
- so by Bézout they share at most 8 points, and the spiric lies along no piece of the surface.
A band guard adds one requirement: one of three interior samples (¼, ½, ¾) must be decided definitely off the surface, or the result stays `Unread`. `(Spiric, Torus)` stays Unread. The β 0.7 union now stops at `"spiric wall volume"` (`STEEP_UNION_STOP`, SPIRICARM's).

Claims to falsify:
1. **The irreducibility argument is right, and the gate matches it.**
   - Check the factorisation argument at the site: real quadric factors of the form `s + a·x + c·z + b`, the forcing to `d = 0`, and the complex-conjugate case. Is anything assumed about the frame (`u_ref`, the offset's sign, a torus not centred at the origin)?
   - Does `bool_rider_spiric_regime` decide exactly `0 < |d| < R − r` with margins? Is each excluded offset (0, in band of 0, R − r, beyond it, R + r) Unread?
   - Can a minted spiric carrier carry an offset outside the regime that the mint labelled two-oval?
2. **Each admitted surface meets the plane in degree ≤ 2.** Sphere, cylinder and cone, including the degenerate sections: a plane through the cone's apex, and a plane parallel to the cylinder's axis giving two lines, one or none. Is there a surface variant behind `Cylinder`/`Cone`/`Sphere` (for example an elliptic cylinder, or a spindle-like cone variant) for which this fails?
3. **Off cannot hide a near-coincidence.** `Off` is about the carriers. The band guard keeps a spiric in band along its whole span `Unread`. The PR admits a residue: a shorter in-band stretch between the samples (a near touch) still reads `Off`. Follow that case downstream:
   - what reads a spiric edge against a cylinder, sphere or cone face for crossings and touches (`geom_brep::curve_face::route`'s `Door::Spiric`, which the PR says is unbuilt);
   - does every such pose refuse typed there, or can `Off` let `slit::holding` or `sectors::closed_reach` build past a near touch without any reading?
   Build a near-touch pose and see where it stops. A silent wrong build here is a MAJOR.
4. **One-way, and confined.** `carrier_on_surface` has exactly two consumers, `slit::holding` and `sectors::closed_reach`. Does either treat `Off` as something stronger than "not on this surface"? Is the arm reachable from anywhere else?
5. **The rows can go red, and pin by identity.**
   - Mutants: drop the regime gate; drop the band guard (the PR says the in-band row goes red); admit Torus; return Off for offset 0. Each must turn a row red.
   - `STEEP_UNION_STOP` pins the full refusal at both spins. Check it is by identity, and that the `OTHER_SPINS_STOPS` change is the only moved cell.
6. **The certificate homes.** Is this honestly a degree certificate, not a third home of the polynomial-identity certificate (ONECOPY's two-homes row)? Does the PR's sampled band guard duplicate `spline_off_carrier`'s or `carrier_on_surface`'s sampling machinery, and should it reuse one?
7. **Inherited reds:** the topo and sweep failures named in the PR body. Reproduce at least `every_door_that_reads_a_box_is_inventoried` on main.
