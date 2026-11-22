# M-a: why the wedge reach's bound from above has no cover

## What was counted

The instrumentation is `measure.patch` beside this file.
- **Each read.** At every `WedgeReach::reach` call, each of its faces whose
  `face_cover` refused counts once. It is keyed by:
  - the door: where the `WedgeReach` was made (`#[track_caller]`);
  - the lane;
  - the face's surface kind;
  - the refusal's text;
  - for a NURBS face, `NurbsSurface::net_state()` (`Placeholder`: every
    channel of every control point is poison, the `mvfs` seed's "no
    description yet"; `Poisoned`: some channel is poison; `Described`).
- **The denominator** is every reach call at that door.
- **Coverage.** ε = 1e-9, every test in topo, step-import, sweep and
  editor-core: 12,157 tests run, of which 31 timed out under the measurement
  profile's 6-minute cap.

## Result

Every uncovered read is a face whose carrier names no region yet:
- **a NURBS face in `NetState::Placeholder`**, or
- **an unbounded analytic carrier** (a plane, a cylinder or a cone) whose face
  has no bounding loop yet.

No finished, described spline face was uncovered (no `Described` net among
the refusals). The few `Poisoned` reads are one at-rest probe of a poisoned
net, `cert_n2r2_consumer_probes::n2r2_class9_validate_geometric_and_pseudomanifold`.

| Door | uncovered face reads / reach calls | What they are |
|---|---|---|
| `euler::certify_edge_spec` | 1,436,200 / 6,789,246 | Placeholder NURBS (f64 1,330,944; Interval 31,354; Sym 52,427; Dual 18,877), and unbounded cylinder/cone/plane faces (≈2,600), mid-construction: Euler ops certifying an edge onto a face whose surface is still the seed |
| `euler::certify_rebased_run` | 19,200 / 1,015,422 | Placeholder NURBS, mid-construction |
| `seqgen` (1094, 1273, 1376) | ≈14,000 | Placeholder NURBS: the random op-sequence generator's bodies (`seqgen::tests::*`, `validate::tests::random_*`), which never describe their seed surfaces |
| `cover::cone_may_hold_apex` (the lift's turns) | 1,392 / 1,727,152 | Placeholder NURBS neighbours of a cone face (`a_cone_apex_meets_a_vertex::*`, `a_conical_ended_football_revolves::*`) |
| `chart_window` | 872 / 1,019,380 | Placeholder NURBS neighbours (`one_region_rows::*`, `the_football_revolves::*`) |
| `validate` tier 3 | 224 / 5,987,390 | At rest: placeholder NURBS (`boolean::ops::tests::nurbs_faces_refuse_typed_at_both_doors`, `validate::tests::check_9_*`) and the one poisoned probe above, all fixtures that hold such a face at rest on purpose |
| every other door | 0 | |

## So

"NURBS faces have no cover" was the wrong reading.
- **What the refusals are.** They are the seed's placeholder surfaces read
  mid-construction, by the doors that certify an edge before its faces are
  described, plus placeholder fixtures at rest. `Surface::cover` refuses them
  because a placeholder net names no region.
- **What a bound from above there means.** For those doors it is not a bound
  on any region yet. The face the verdict will govern has no geometry. A door
  that needs the bound from above there can only refuse.
- **Who that touches.** Of the doors in the table, only CERT, which takes the
  bound from below, reads such faces in bulk.
