# SHELL-6 — the cone nappe has one home

**Status: BINDING at dispatch (SHELL orchestrator, 2026-09-08;
opens block SHELL-B2).** Unit `SHELL-6`, branch `shell/6-nappe-home`. Closes
`work/shell/mint-offset-ignores-cone-mirror-nappe.md` (issue record
1199) and item 2's nappe half of nothing else — the winding-predicate
rename in `shell-offset-three-followups` is NOT this unit (three
owners' files and a K-lint population; it stays on that item). Binds
the implementer of unit `SHELL-6`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md`
in full first. Survey against main `891db152`, 2026-09-08; citations
by symbol.

## 0. The question, and how many times the tree answers it

A cone is a double cone. `geom_brep::offset_surface` on a
`Surface::Cone` slides the apex by `−axis·(d / sin α)`
(`geom_brep::ConeOffset::apex`), which is the pushforward along the
OPENING nappe's normal field: material on the opening nappe moves
`+d` along its chart normal, material on the MIRROR nappe moves `−d`
along its own. `ConeOffset`'s header ratifies that `n₊` does not flip
across the apex and says the consequence in as many words: *the
consumer owes the turn* — which nappe a FACE lives on is a fact only
the face has.

The tree answers "which nappe is this face on" in four places, three
of them consumers of the same mint:

1. `topo::offset_axial::nappe_signed` — the sum of the face's corner
   stations `Σ (p − apex)·axis`, decided as `offset_axial_nappe`; a
   face at its own apex refuses `TogetherNotAxial`. **Discharges the
   obligation.**
2. `topo::replace_face::replace_faces_offset` → `mint_offset` — hands
   the caller's `d` to the mint UNTURNED (the comment at the site says
   so, citing 1199). **Does not discharge it.** Twenty lines below,
   the same door's apex-window gate decides the nappe AGAIN from the
   group's `v` window (`offset_apex_nappe` on `v_min` and `v_max`,
   `group_cone_v_window`) to pick which window end faces the apex —
   a second reading in the same function, of the same fact, used for a
   different purpose and never reconciled with the first.
3. `geom_brep::ConeOffset::displacement` — `cos α · copysign(h)`,
   a PER-POINT nappe read from the sign of the point's own station,
   consumed by `replace_face.rs`'s corner transport (`action.displacement(mid)`,
   `action.shift()`).
4. `geom_brep::ConeOffset::shift` — no nappe read at all
   (`d·cot α` at matched `u`), consumed beside 3 and by `apex_shift`.

Nothing wrong ships today (1199's two review arms measured: the
neighbouring caps refuse `ReanchorOffCarrier` first on every reachable
per-chart fixture). This unit makes the hazard unreachable by giving
the fact one home, and pins that the four readings agree.

## 1. The home

`topo::offset_nappe` (a small module; or in `replace_face.rs` if the
lane argues it belongs beside the door — say which and why):

```rust
pub enum Nappe { Opening, Mirror }
/// Which nappe `face` lies on, decided from the face's own corner
/// stations. A face straddling the apex has no nappe.
pub fn face_nappe<T: Decide>(body: &Body<T>, face: FaceKey, band: Band)
    -> Result<Nappe, NappeError<T>>;
impl Nappe {
    /// A face-outward distance turned into the mint's convention.
    pub fn turn<T: Real>(self, d: T) -> T;
}
```

`face_nappe` is `nappe_signed`'s decide, moved (same predicate name
`offset_axial_nappe` → renamed to `offset_nappe`, ONE K row; the K
population is re-derived per the K-REPORT runbook and the move is
disclosed as such). `nappe_signed` becomes `face_nappe(..)?.turn(d)`
and its own body is deleted; `mint_offset`'s caller turns `d` the same
way BEFORE the mint (the door's signature does not change: `d` is
still face-outward at the door, turned inside it), and the 1199
comment goes. The apex-window gate keeps deciding the WINDOW (it needs
`v_min`/`v_max` for the collapse margin) but reads the nappe from
`face_nappe` and refuses `ApexWindowStraddles` (the existing variant)
when the window disagrees with the decided nappe — the two readings
are reconciled by making one of them the authority.

`ConeOffset::displacement` loses its `copysign`: it takes the nappe
explicitly (`displacement(nappe, p)`), because a per-point sign read
is a third authority and the corner transport already knows the face.
`shift` is unchanged (it never read the nappe; its consumers pass the
turned `d`, which is the same fact spelled once).

## 2. Acceptance

1. **The two doors agree.** A cone frustum BELOW its apex (the sf2b
   fixture, `crates/sweep/tests/sf2b_r2_probes.rs`) offset inward
   through `replace_faces_offset` and through `offset_charts_together`
   yields the same minted cone (apex, half-angle bitwise equal), and
   the cavity is SMALLER than the operand on both doors.
   `sf2b_r2_probes::r2_per_chart_door_on_a_mirror_nappe_cone` is
   rewritten from "reports which way it went" to asserting the way;
   `sf2b_r1_probes::r1p3_the_cone_mint_is_nappe_blind_and_the_door_corrects_it`
   keeps its first half (the mint is nappe-blind by contract) and its
   second names the home.
2. **The frustum ABOVE its apex** (opening nappe) on both doors: the
   same closed-form cavity as today's axial door — the differential.
3. **Straddling refuses typed** at the home, on both doors, with one
   variant.
4. **`displacement` and the home agree**: for every corner of both
   frustums, `displacement(nappe, p)` equals the old `copysign` value
   bitwise (a row that dies with the `copysign` and is deleted after
   it passes once — no, keep it: it is the pin that the per-point read
   and the per-face read never disagree on a face that has a nappe).
5. **Sweep receipt** in the PR body: every `Surface::Cone` match arm
   in `topo/src/{replace_face,offset_axial,offset_together,shell}.rs`
   and `geom-brep/src/offset*.rs`, one line each: reads the nappe
   (from where), or does not need to and why.
6. The existing shell suites and the teapot/torus-vessel scenes are
   byte-identical (no cone in them, which is the point: the
   differential is zero and the rows say so).

## 3. Stops

STOP if `mint_offset`'s turned `d` makes a fixture that used to refuse
`ReanchorOffCarrier` BUILD: that is the latent hazard becoming
reachable in the other direction, and what it builds must be measured
against the axial door before it is called right.

## 4. Owed

`crates/geom-brep/README.md` O-clauses: the cone offset's consumer
obligation sentence becomes "discharged at `topo::offset_nappe`". The
K row rename is disclosed in the PR and the K-REPORT runbook is run.
SHELL-5 landed one `OffsetDoor` decision in `shell.rs` (cavity and
lift read it); the nappe turn sits BELOW that decision, inside the
doors, and this unit does not move it. `offset_axial.rs`,
`replace_face.rs`, `offset.rs` are SHELL's files; nothing here is a
seam. Lane rules as every SHELL brief: own worktree, own
`CARGO_TARGET_DIR`, one heavy cargo job at a time, no `Co-Authored-By`
trailer in lane commits, push after every coherent step, hosted CI is
the gate (nothing narrowed), report ≤ 150 lines.
