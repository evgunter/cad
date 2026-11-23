# The class sweep (asked by the orchestrator after M2)

Branch `oneround/conic-harmonic-probe`, ε 1e-9 unless said, K = 10, `f64`.

## 1. The uncertified roots' callers

### Door A, the plane door

| caller | what it does with a root `t` | what stands between the root and a built result |
|---|---|---|
| `topo::boolean::reduce` (the conic-against-plane arm) | `p = carrier.eval(t)`; `contfp_decided(y, face, normal, p)` places `p` in the face; on `In`/`OnEdge`/`OnVertex` it **splits the edge at `t`** (`split_at`, `split_logged`) and records the contact | nothing re-reads `p`'s height above the plane: `contfp` takes `q` "already on the plane". The vertex is the carrier's own point at `t`, so it is on the edge exactly and off the plane by the root's height |
| `topo::boolean::factored_crossing` | the roots against two planes; two within the band are "a node"; then `landing(y, face, carrier.eval(t))`, a `CurvedEvent::Pierce` at `t` | `bool_factored_crossings_apart` decides two roots apart; no height re-read that I found |
| `topo::boolean::slit` (`Placed::On` / `Off`) | reads only `InPlane` and `PlaneParallel`; every other answer is `Off` | no root is used |
| `topo::splitting` | through `curve_face::cross()`, the same pose | not read further |

So the question is the root's **height off the plane**, since the built
vertex is the carrier's own point. Door A's worst height near the anchor,
(i), from M1's oracle: 2.3e-12 m at `a` = 1e4, 2.7e-11 at 1e5, 3.5e-10
at 1e6, 2.7e-9 at 1e7, 3.1e-8 at 1e8, 4.6e-7 at 1e9. At ε 1e-9 that is
inside ε to 1e6, in the band's gap at 1e7, and past Kε from 1e8.

**But no public door builds a body carrying such an edge at those
sizes.** `topo::splitting::split` of a frustum by a plane toward the
parabola (`sweep`'s `conic_past_band_probe`, half-angles 0.46, 1.0, 1.3,
unit scale):

| ε | largest section built | first refusal |
|---|---|---|
| 1e-9 | `a` = 1.1e5 (α 1.3), 3.0e5 (α 0.46), 6.1e5 (α 1.0) | `a` ≈ 1.1e6: `Finish(ResultNotAtRest … VolumeUncomputable)`; from 3e6: `Certification(AnalyticRung3(KernelLimit { operand: Plane, bound: 3.7e-9 }))` |
| 1e-12 | `a` = 1.1e2 (α 1.3) | `a` = 1.1e3: the same two refusals |

**The probe at `a` = 1e6 and 1e7: refused**, at the mint of the operand
(the first split), before any root is read. At the largest sizes that
do build, end to end through `split` twice
(`crates/sweep/examples/conic_harmonic_e2e.rs`: the frustum rotated off
the axes, split near the parabola, then the part carrying the section
split again by a plane crossing the section's edge at 90°, 30°, 5°), the
truth being the body's own stored edge carrier against the second plane
in `mpmath`:

| a | second split | built vertex off the true crossing | off the plane |
|---|---|---|---|
| 6.1e3 | 3 split, 15 refuse (`OpenConicChord`, the second plane's hyperbola on the cone face, unrelated) | 4e-12 … 2e-11 m | 1e-12 … 2e-12 m |
| 6.1e4 | 3 split, 15 refuse (the same) | 9e-12 … 6e-11 m | 8e-13 … 8e-12 m |
| 6.1e5 | the first split refuses under the rotation (`KernelLimit { operand: Cone, bound: 1.2e-9 }`) | | |

**Built rightly**: every vertex is within 0.06 ε of the true crossing.
I found no wrong build. What keeps it so is the edge certificate's
`KernelLimit`, which refuses the carrier at about the size where the
root's height would reach ε; it is not a re-read of the root. A door
that minted such an edge without that certificate would have nothing
behind it. I did not probe the boolean's own mint (`boolean::join`
calls `plane_cone_section` too) or `factored_crossing`.

### Door E, `ring_path::plane_roots`

Callers: `path_parity` only (two sites; a section arc's plane, and a
meridian piece's own plane). Each root goes to `span_readings`, which
decides it inside or outside the arc's span, levered by the crossing's
slope and charged the same `slack`, and to `on_own`; the crossings flip a
parity. No point is built from the root: it decides a containment. Not
probed end to end: the operand would be the same far-section body, which
`split` builds only to `a` ≈ 6e5, where M1 read the root 2.6e-7·(a/1e6)
off at a 3e-8 gap and 1e-9 at a 1 mm gap. Whether a parity can flip on
that needs a probe I did not build.

### `topo::boolean::reduce` on M2's refusals

A limit of the door (`Unsettled::Tangency`, `Unsettled::Meter`) is read
`off_face()`: where the span may meet the carrier is localized and read
against the face; off it, `SpanVerdict::OffFace`, else
`SpanVerdict::Unsettled`. `AtApex` is its own verdict. An escalation
returns `door_escalated`. So M2's refusals stay refusals unless the
localized reading clears the face; I did not read `off_face`.

## 3. Was the `curve_pair::meet` miss wrong before PR 7203?

Tree `194131e40` (the first parent of `2b9cd8b24c`), where the cone's
section is held at its centre. The same harness file on both trees
(`analysis/oneround/conic-harmonic/probe_pre7203.rs.txt`, run as a
`#[cfg(test)]` module of `geom-brep`): the thin ladder's cone and plane,
`plane_cone_section`'s ellipse, and a line through `p`, the point where
a cone generator meets the plane, in the plane. `p` is judged against
the cone and the plane themselves in `mpmath`
(`oracle_sweep3.py`, no ellipse in it): within 4e-16 m of both for the
two generators beside the near vertex (1.9 and 2.4 m from the apex), and
within 2e-13 m for one 2.8 km out.

| a | ε 1e-9, before 7203 | ε 1e-9, the branch | ε 1e-12, before | ε 1e-12, the branch |
|---|---|---|---|---|
| 1e4 | 3 meet | 3 meet | 2 meet, 1 escalates | 3 meet |
| 1e5 | 3 meet | 3 meet | 3 escalate | 3 escalate |
| 1e6 | 3 meet | 3 meet | **3 miss** | **3 miss** |
| 1e7 | 2 meet, 1 escalates | 2 meet, 1 escalates | **3 miss** | **3 miss** |
| 1e8 | 3 escalate | 3 escalate | **3 miss** | **3 miss** |
| 4.8e8 (ε 1e-9), 1e9 (ε 1e-12) | **3 miss** | **3 miss** | **3 miss** | **3 miss** |

**Yes: the same verdicts, miss for miss, before PR 7203.** What 7203
changed is where the stored curve stands, not what `meet` answers:

| a | the stored section's foot off the true point, before | the branch |
|---|---|---|
| 1e4 | 6e-13 m | 1e-16 m |
| 1e5 | 2e-11 | 2e-16 |
| 1e6 | 1e-10 | 6e-16 |
| 1e7 | 2e-9 | 2e-16 |
| 1e8 | 3e-8 | 9e-16 |
| 1e9 | 2e-7 | 6e-16 |

So before 7203 every miss stood where the stored carrier itself was
past Kε from the true section (1e-10 against Kε = 1e-11 at 1e6 and
ε 1e-12; 1.4e-7 against 1e-8 at 4.8e8 and ε 1e-9): wrong of the cone and
the plane, and not provably wrong of the stored numbers. Since 7203 the
stored curve passes within 1e-15 m of the line and the same answer is
wrong of the stored numbers too. Where a meeting is found, it stands
2e-11 … 2e-9 m off `p` before and 8e-14 … 5e-11 m after, near the
vertex.
