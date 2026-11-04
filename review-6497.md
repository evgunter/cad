# Review of PR 6497 (`spiricarm/spindle-apex`), reviewer R1

**Frozen head reviewed: `c7142ee5772809d2720dfcfbb84d15293cbd6d04`.** Brief sha256 `ed4748b2…f753f6`, verified.
Probes: branch `spiricarm-rev6497-r1/probes` (`aeefb9f6d8`): `crates/sweep/tests/review_6497_r1_probes.rs`, `crates/mesh/tests/review_6497_r1_cert_probes.rs`, and the mutation scripts in `review-6497-r1-mutants/`.

## Verdict: APPROVE-WITH-FIXES (0 MAJOR / 3 MINOR / 4 NOTE)

Every body I could build on an apex measures exactly: 22 probe bodies, |ΔV|, |ΔA| ≤ 1.2e-13 against `π∮ρ²dz` / `2π∫ρ ds` written in the probe. They are lemon and apple half-footballs (each half), a football split in two arcs, a sheet half under a hemisphere, a tilted and offset axis, R = ±1e-3, R = −0.99, wide apples to R = 1 − 1e-6 (tier 3 and props only), the lower needle, and a cylinder with a lemon or apple **cavity** on its axis. None is mis-meshed silently. The gaps are in the guards, not in the answers.

## Findings

**MINOR 1 — `geom-brep::props::curved::sheet_material_sign`: neither of its two sheet-specific branches is guarded by a PR row.** (confidence: sure, by execution)
- M7a, `(Ok(_), Ok(_)) => Ok(left)` in place of `Unencoded`, survives every PR row. All the PR's π-band faces have sense true, and the Left reading is Positive, so taking it is invisible.
- My `r1_football_cavity` (π-bands with the material outside, sense false) turns M7a red, and passes at head (V exact to 1e-13).
- M7g, the one-closing branch `(Ok, Err) | (Err, Ok)` made `Err`, survives every row, mine included. A refused derivation is exempt in check 6 (`validate.rs` ~8130), so the needle's "one closing reading encodes its side" is never compared with its bit. A flipped sheet face is still refused, but one layer away: `LaminaWedge` for one face, `NegativeVolume` for both (`r1_flipped_sheet_face`, lemon and apple).

**MINOR 2 — `sheet_material_sign` reads any `Err` as "this side does not close".** (confidence: unsure whether it is reachable)
- `(Ok(side), Err(_)) | (Err(_), Ok(side)) => Ok(side)` takes the other reading whatever the error was, including `PropsError::Escalated` from a mean-width or closure decision in band.
- An undecided reading on the face's true side would then hand back the other side's sign, encoded, against [loud-failure]. The doc's "where neither does, the face's own reading refuses, as the left side's" also returns Left's error whatever the face's side.

**MINOR 3 — the part-way row's band.** (confidence: likely)
- `a-football-revolved-part-way-refuses-at-its-cap-meridians-certificate` is P3. It is a user-drawable `revolve(Partial)` of a profile whose full revolve now builds, and it refuses with a kernel-defect message ("an Euler operation refused").
- `work/README.md` puts "verb breadth beyond the everyday shapes" at P1, and "something the GUI cannot author at all" at P0. P3 is "library usability; latent unsoundness".
- The row is accurate otherwise. `a_football_revolved_part_way_refuses_typed` passes at head, and the horn's `Partial` pinch is folded into it. That pinch is also kernel-defect-shaped (`Op`) for a non-manifold shape, so claim 7's question is answered yes, filed.

**NOTE 1 — `the_football_revolves::a_football_restored_on_the_other_sheet_is_refused` cannot go red for an apex regression.** (confidence: sure)
- It asserts only `is_err()`. The refusal at head is `EdgeCertification`/`Pcurve` `CarrierOffChart` (margin 1.0 for the lemon, 0.8 for the apple) and `JoinUndecidedAtRest`, so the edges' geometry refuses it, not any reading of the apex (`r1_other_sheet_reason`).
- The refusal is correct, and the row's own doc says the edges are off the sheet, but the PR body presents it as the wrong-apex guard. This is the same class as stage 1 R1's "the wrong-sheet row cannot go red alone".

**NOTE 2 — three more mutants survive every row, PR's and mine.** (confidence: sure on survival; unsure on importance)
- M7c drops the fold's `a.apex_after.is_none()` guard.
- M7d sets the apex `Joint`'s `flat` to zero.
- M7f levers the jump's `EdgeTurn` at `minor`.
- M7d and M7f move only the closure's slacks. M7c looks unreachable on a valid sheet: a carrier continuing past an apex leaves the sheet. Each is a guard nothing tests.
- M7b (the apex end's azimuth read at the edge's end, not its middle) and M7e (the apex edge turns dropped) go red.

**NOTE 3 — the horn snap certifies the stored horn, not the drawn arc.** (confidence: likely)
- Claim 1 holds. The sphere class at `rc ≈ 0` mints its centre at `frame.foot3(center_sk)` (`revolve/surfaces.rs:50`), the same move.
- The snap is taken only on `ArcReach` = Zero, so `|R − rc| ≤ band.zero()`, and the ladder's ±3z rows still refuse.
- After the snap, `mass_properties` reports the horn with `volume_pad = 0` at f64. The drawn arc's volume differs by up to ~`2|d|·A`, and that figure is stated only in the ladder test's allowance, not at the snap or on the enclosure.

**NOTE 4 — the wide-apple evidence's outcome reproduces; its mechanism I could not confirm.** (confidence: unsure)
- The lower needle builds, passes tier 3, and measures V = 0.30118555543831 (error 1.6e-15). `tessellate` then refuses `UnsupportedCurvedDomain { off_bbox: 31, first_uv: (3.246, −π/2) }`.
- The first off point is a `u` past π at `v = −π/2`, which is not obviously "`v < 1 − π` read a turn off".
- Apples R ≥ 0.7 refuse at the mesh in the same way, and pass tier 3 and props exactly; R = 0.6 builds through the mesh.

## Claims
1. **Horn snap: upheld.** Precedent, the band and certification are covered in NOTE 3.
2. **The apex as a vertex: upheld.**
   - `place_on_boundary` returns `Some` only as `AtVertex` or `InsideEdge` (`chart_singularity.rs:482/495`), so a non-vertex apex cannot reach `Ok(Some(_)) => {}`.
   - M2 (`singular_at` without the sheet) turns the football/apple and needle rows red.
   - `chart_iso_family`'s only non-test callers are the two `edge_join` sites, and both now pass `c.params()`.
3. **Props at an apex: upheld for the volume.**
   - Executed: the football's faces read `Unencoded`, and both are sense true. The bullets and the needle read `Encoded`; the needle is sense false, Negative.
   - "No side" is a correct reading: check 6 defers it to the seam region, and a flip is caught by `LaminaWedge` or `NegativeVolume`.
   - I found no building face whose side the rule gets wrong; the cavity (sense false) is exact. The guard gap is MINOR 1.
4. **Mesh: upheld.**
   - Sheet faces take `pole_columns`.
   - I fuzzed `cert_spindle` at 240k cases beyond the PR's range: horn, near-horn (0.98..1), slim lemon, `Δu ≤ 2`, same-row fans. 0 under-reads; worst distance/certificate 0.99997 at the horn, so the bound is tight and carries load there.
   - The cusp is listed once, and `check_mesh` passes on every probe body that meshes.
5. **Oracles and mutants: partly upheld.**
   - The oracles are independent of the kernel.
   - The other-sheet row is refused, but not by the apex (NOTE 1).
   - Of the PR's six mutants I executed two, and both turn red as listed: the horn snap removed reddens the needle and the ladder; `singular_at` reddens the football/apple and the needle. The other four I did not run.
   - Seventh mutants: M7a survives the PR's rows and my cavity kills it; M7g, M7c, M7d and M7f survive everything (MINOR 1, NOTE 2).
6. **Sweep: upheld.**
   - My shape: every `fn` named `*pole|apex|singular|cusp*` in `geom`, `geom-brep`, `topo`, `mesh` and `sweep` src whose next 80 lines name Sphere or Cone but not Spindle or Sheet. That gave about 60 hits.
   - None is on the football's or the apple's path. They are the split door (`ring_path::off_apex`), the boolean, replace-face and section doors, sphere-only props helpers (`pole_free_side`, `pole_side`, `require_cusp_free`) that only the sphere arm reaches, and tests.
   - A second grep, for `/ρ` divisions in the torus chart, closure, mesh sizing and cert code, found none at an apex on the path.
   - What the shape cannot see: an apex arm inline in a function whose name says none of those words.
7. **Re-pins: upheld.** Each moved only where the change moved it; `one_segment_loop`'s horn `Op` is filed (MINOR 3).
8. **Residue: the part-way row is accurate but banded low (MINOR 3); wide-apple evidence in NOTE 4; no silent mis-measure found.**

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7; Q8 partially. I read `props/curved.rs`'s torus/sheet chart section (about lines 5040–5730) end to end, but not all 6,550 lines, and not `validate.rs` (18,886 lines) whole.
- **Q1:** a meridian's azimuth "at its middle" for an apex end is read twice: `torus_chart_loop`'s `middle()`/`ua,ub` (`curved.rs` ~5502) and `sheet_apex_jumps`' `middle`/`radial` (~5672). The jump's `du` and the edge's turn must agree for the closure to close, and nothing makes them one reading. *likely*
- **Q7:** `topo::Chart::poles` recognises a horn by `(a.1 − b.1).norm() == 0.0`, bit equality of two computed points applied to every kind, rather than by the sheet's horn classification. It holds only because `geom::sheet_apex`'s clamp yields identical points; any other route to an accepted horn lists two. *unsure*
- **Q2/Q5:** `sheet_material_sign`'s doc ("the face's own reading refuses, as the left side's") does not match the code (MINOR 2). *likely*
- **Q3:** `cert_spindle_bounds_a_fan_triangle_on_lemon_and_apple_sheets` stops at `|R/r| ≤ 0.98` and `Δu ≤ 0.6`. The horn, which this PR newly meshes, is outside it, and the bound is tightest there (0.99997). *sure*
- **Q3:** every sheet face in the PR's rows has the same sense per body, so no row distinguishes Left from Right in the material sign (MINOR 1). This is the class: look for any other sense-dependent reading that is fixed by a single-sense suite. *sure*
- **Q4:** I grepped the retired `SheetApex` variant and prose ("no face reaching a sheet's apex", "stage 2's"). Only the dual-review log rows mention it, as history. Clean. *sure*
- **Q6:** the snap's volume shift (NOTE 3) is a claim that rests on a measurement, and its only guard is a test allowance. *unsure*
- **Q7:** `revolve/mod.rs`'s `# Errors` line was edited in place and left unwrapped ("sliver radii, non-manifold axis contact, sliver dihedrals, Newell failures, and"). *sure, trivial*
- **Q7:** `ApexSide` is derived from `sense` in `curved_face_holding` and passed as `Option<ApexSide>` to the torus arm, where `None` stands for "a ring torus". That invariant is held by convention where the kind could carry it. *unsure*

## Disclosures
- I did not fetch, check out or read any `spiricarm-rev6497-*` branch but my own, nor PR 6497's comments or reviews. I posted nothing to GitHub. No glimpses.
- The harness moved two of my foreground waiters to the background at its 120 s cap. I stopped one; the other finished on its own. Afterwards every build was either `setsid`-detached with its PID recorded and polled in the foreground, or run in the foreground under a long timeout.
- My first mutation batch had a revert bug: M7d's replacement text was not unique, so M7d stayed applied through M7e, M7f and M7g. I re-ran M7a, M7c, M7d, M7f and M7g each alone with strict probes. M7e ran only alongside M7d; its red stands, since M7d alone is green.
- The first version of my probes counted a refusal as a pass. Every mutant verdict above uses the PR's rows or the strict re-run (`R1_STRICT=1`).
