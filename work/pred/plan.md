# PRED — one numeric fact, decided in several places (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3).

Branch prefix: **`pred/`**. Away-channel tag `(PRED orchestrator)`.
A/B ordinal band **PRED = 4400–4499**.

## Charter

Every row here is the same finding about a different quantity: a fact
the kernel decides — is this rim planar, is this cone one nappe or two,
does this period have headroom, is this lever arm folded — is decided in
several places, and the places do not agree on the margin. They are not
duplicated *code*; they are duplicated *judgements*, which is why a
mechanical de-duplication is the wrong fix and why these rows have sat
unclaimed while the crates they live in have all had owners.

The obligation this program takes on and that no row states for itself:
**a predicate that gets a shared home gets an agreement test in the same
PR**, showing that the old sites and the new one answer the same on the
cases each was written for. A shared home with no such test is the same
defect with one address, and the near-cases are exactly where the
margins differed.

## Territory — none, and why

This program claims **no paths**. Its rows reach PROPS', CURVED's,
S-BOOL's, S-MESH's and TRIM's files simultaneously — `props/curved.rs`,
`boolean/reduce.rs`, `mesh/curved.rs`, `pcurve_cache.rs` — which is the
class, not an accident of filing. Every unit announces to the owners it
touches and the fence is drawn in the PR.

## The slate

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `lever-arm-fold-six-hand-rolled-siblings` | **M** | Three sites left; two are swaps, one re-homes a decide-name predicate | `crates/topo/src/boolean/contact_verify.rs`, `crates/topo/src/boolean/ops.rs`, `crates/geom-brep/src/ssi.rs`, home at `crates/geom-brep/src/dihedral.rs` (`folded_lever_arm`), `crates/geom-brep`'s `classify_material_pairing` |
| `period-headroom-margin-has-no-shared-home` | **H** | Eighteen sites, ten files, three crates; new `Decide` API plus latent defects | `crates/geom-brep/src/{certify.rs,pcurve_cache.rs,props/curved.rs}`, `crates/topo/src/boolean/{reduce.rs,solid_contain.rs,contain.rs}`, `crates/topo/src/{chart_region.rs,chord_join.rs}`, `crates/sweep/src/{revolve/mod.rs,revolve/tube.rs,blend/surgery.rs}`, `docs/predicate-dimension-audit.md` |
| `cone-nappe-is-decided-in-five-places` | **H** | One numeric fact, five predicates, three programs' files; owner undecided, agreement tests missing | `crates/topo/src/offset_nappe.rs`, `crates/topo/src/boolean/solid_contain.rs`, `crates/geom-brep/src/pcurve_cache.rs`, `crates/geom-brep/src/props/curved.rs` |
| `S58` | **H** | Cross-crate predicate unification; numeric, and tangled with open #723/#726/#727. | `crates/geom-brep/src/props/curved.rs` (`props_rim_level`, `du_of_rims`), `crates/mesh/src/curved.rs` (`require_swept_rectangle`, `entries_off_bbox`), `crates/geom-brep/src/props/mod.rs` |
| `S18` | **H** | Six certified derivations unified across crates, DAG and genericity obstacles | `crates/geom-brep/src/ssi/enclose.rs`, `crates/mesh/src/{nurbs_cert.rs,chords.rs}`, `crates/geom-brep/src/edge_geometry.rs`, `crates/profile/src/seg.rs`, `crates/sweep/src/{skin.rs,revolve/upgrade.rs,extrude.rs}`, `crates/topo/src/validate.rs`, `crates/step-export/src/volume.rs`, `crates/topo/tests/{fixtures.rs,review_m1_pr*}` |
| `D292` | **H** | Closed-form directional extremum: numeric work, new geom door, census mirror, two tracks | `crates/topo/src/boolean/boxes.rs` (`edge_axial_span`), `crates/geom/src/curves/boxes*`, `crates/topo/src/census.rs` (`face_reach`) |
| `S66` | **H** | Interval-arithmetic bound fix under three refusing doors, plus new near-case rows. | `crates/topo/src/boolean/{boxes.rs,separation.rs,census.rs,ops.rs,reduce.rs}`, `crates/topo/tests/s16_box_soundness.rs` |
| `S82` | **H** | Asks whether a row is owed beside #893; numeric near-pole lever, two tracks' files. | — (ruling; fix, if rowed, would touch `crates/geom-brep/src/props/curved.rs` and `docs/predicate-dimension-audit.md`) |
| `S65` | **H** | Three-way debug/release/widen choice is #884's ruling; D2 row question undecided | — |
| `S116p` | **H** | Ev decides permanent refusal vs deferral; rests on unproved geometric claim | — |
| `S29` | **H** | Stating the policy is a design PR awaiting Ev; six constants have no venue | `crates/mesh/src/sizing.rs`, `crates/mesh/src/{chords,curved,nurbs_cert,trimmed,tessellate}.rs`, `tools/tess-meter/src/lib.rs`, `docs/TESS-BUDGET.md` |

## Order

`lever-arm-fold-six-hand-rolled-siblings` opens: three sites left, two of
them swaps, and it is the one row here where the shared home
(`dihedral.rs`'s `folded_lever_arm`) already exists. It sets the
agreement-test habit on the cheapest case.

Then the two big unifications, `period-headroom-margin-has-no-shared-home`
(eighteen sites, ten files, three crates, and a new `Decide` API) and
`cone-nappe-is-decided-in-five-places`. Then `S58` and `S18`.

`D292` is separable and numeric rather than structural — a closed-form
directional extremum replacing a sampled one, plus the census mirror.

**Four rows are not takeable as units.** `S82`, `S65` and `S116p` are
rulings; `S29` is work whose first step is a design PR to Ev stating the
sizing policy, and the six constants have no venue until it lands. They
belong on one `[ev]` sitting with SCALAR's three, because they ask the
same kind of question — how much does the kernel promise, and where is
that written.

`S66` is parked on #862 and its own body reads as verify-and-close
against the tree as it is now; that verification is the first cheap act
available here.

## Review posture

Full v6 dual with Fable specs on every unification: a wrong margin is
reachable and silent, so **the review is adversarial by default here
rather than by exception** — the standing rule is adversarial review
wherever a wrong answer is reachable, and on this slate it always is.
## How the class column is read

`E` / `M` / `H` is a **dispatch estimate**, made on 2026-09-11 by reading
each row against the tree, and it is the axis this program's order runs
on. It is not a verdict on the finding and it is not in any header: no
field carries it, `work.py` does not parse it, and this table is the only
place it lives. A lane that finds the estimate wrong says so in its PR
and this table is corrected in the same PR.

- **E** — the fix is written in the row or obvious from it: one or a few
  files, no design question, no ruling, small diff.
- **M** — multi-file, or a small design call (where a shared home lives,
  what a door looks like), or a census or instrument to build first.
- **H** — cross-cutting, numeric or algorithmic, gated on a ruling, or
  spanning several programs' territory.

The cut that opened this program is `docs/WORK-TRACKS-2026-09.md`
addendum 3; it is a survey, and this plan supersedes it as the charter.
