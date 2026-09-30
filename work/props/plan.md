# PROPS — the plan

enclosure certificates and interval honesty

Re-scoped 2026-09-20 by PROPS's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**32.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `cone-apex-cap-refuses-degenerateface` | H | props: a cone face bounded by one rim with the apex interior refuses DegenerateFace; its missing extreme is the apex, and the guard against its unbounded complement needs a sense bit fn cone does not take |
| P0 | `rim-level-rule-manufactures-its-error-by-feeding-nan-into-classify` | H | The rim-level rule's structurally-impossible arm throws by feeding f64::NAN into classify, and unreachable_zero returns a 4-tuple of NaNs into live flux arithmetic |
| P0 | `rim-side-and-rim-dir-group-signs-are-facts-about-cycle-order` | H | props_rim_side and props_rim_dir_group read whichever rim the loop walk from Cycle::first meets first, so their recorded signs are facts about cycle order rather than about the face |
| P0 | `sphere-flux-arm-refuses-partial-bands` | H | the sphere flux arm's coplanar premise leaves lune-family bodies outside tier 3 |
| P0 | `sphere-wedge-arm-does-not-fold-split-meridians-by-lineage` | H | sphere()'s wedge arm reads a two-edge boundary and refuses a meridian that arrives in lineage pieces, where the torus arm folds pieces by lineage |
| P0 | `stored-spans-read-raw-past-winding-bound` | D | props: two more stored spans read raw past the winding bound (torus single-edge meridian; the rim Δu sum for all four kinds) |
| P0 | `the-shape-door-could-take-the-sense-free-rim-side-residue` | D | props: require_iso_rectangle admits a face whose rims encode different material sides; the sense-free residue unanimous_rim_side already decides it in the gate arm |
| P3 | `the-gating-corpus-reaches-no-collapsed-arm-gate` | D | The gating corpus reaches no collapsed-arm gate: five of the eight predicates have zero rows, three have only positive |
| None | `PROPS-1` | None | The lost-correlation members of the linalg audit — mirror_across_plane and reject_from respelled, one re-baseline pass |
| None | `affine3-try-map-the-fallible-walk-has-no-kernel-door` | None | Affine3::try_map: the fallible per-coordinate walk over an Affine3 has no kernel door, so editor-core keeps a private one |
| None | `band-has-no-door-for-an-explicit-eps-with-the-runs-k` | None | Band has no door for an explicit eps with the run's K — four suites open-code Band::new(eps, k*eps) |
| None | `band-linear-errors-doc-is-false-empty-is-reachable-at-subnormal-eps` | None | Band::linear's AND Band::angular_at's # Errors both say BandError arises only on K-epsilon overflow; BandError::Empty is reachable from a validated tolerance with no overflow, and angular_at reaches it at an ORDINARY epsilon |
| None | `budgetexhausted-conflates-three-terminations` | None | BudgetExhausted conflates three terminations (rounds out, sample cap reached, unmarked) — a cap-named refusal would name the knob |
| None | `coefficients-carry-their-knot-vector` | None | The coefficient↔knot-vector pairing is length-only |
| None | `escalation-channel-misses-op-minted-indeterminates` | None | k_stats: the escalation channel misses op-minted Indeterminates (eight sites), two raw sign_within calls, and the unbracketed mate solve |
| None | `indeterminate-error-arms-sweep` | None | The ~40 Indeterminate-carrying error variants the escalation channel makes unnecessary to match on: a deletion sweep |
| None | `k-stats-escalation-channel-and-redo` | None | k_stats: an escalation channel beside the verdict log (and the redo that channel is already owed) |
| None | `lily-authoring-needs-shadow-vector-algebra` | None | API friction — authoring the lily naturally meant building a shadow vector algebra beside Vec3 |
| None | `map-affine-retires-into-affine3-try-map` | None | anchor::map_affine retires into Affine3::try_map in the PR that adopts it |
| None | `metered-margin-doc-promises-an-inf-bound-three-sites-pass-a-sup` | None | Margin::metered's doc promises a certified speed LOWER bound; three sites push a certified UPPER bound through it |
| None | `offset-fit-mignitude-floor-on-norm-e` | None | offset_fit small-|d| certificates are floored by the componentwise mignitude lower bound on ‖E‖, not by rounding |
| None | `register-equals-witness-limits-citation-names-no-file` | None | Real::register_equal's doc cites geom-core/tests/m10_9_witness_limits_interval.rs, a file that has never existed |
| None | `rim-stores-its-traversal-direction-twice` | None | Rim stores the traversal direction twice (d_u: T and d_u_sign: Sign) and du_of_rims compares the exact one through the tolerance funnel |
| None | `rimless-polar-cap-refuses-degenerateface` | None | props: a rimless-boundary polar cap — one circular edge, no meridian — refuses DegenerateFace; its extent has no arc to derive from |
| None | `rotation-about-diagonal-width-floor` | None | Mat3::rotation_about's diagonal carries a width floor at exact angles (1 − cos plus cos's own enclosure), and Affine3 composition through MappedCurve::restrict grows it per split |
| None | `span-carries-its-knot-vector` | None | Consider giving Span its KnotVector — close the unbranded-pairing hole structurally |
| None | `three-per-node-verdict-shapes` | None | Three shapes for per-node verdicts: consolidate or record the split deliberately |
| None | `two-face-sphere-split-measures-zero-volume` | None | props: a closed sphere split into two faces by the same two meridian arcs measures volume 0.0 — one parse hands both faces the same levels |
| None | `vec3-point3-const-and-conversion-doors` | None | Vec3::new/Point3::new are not const fn and there is no Vec→Point conversion door — the two spellings the lily rewrite could not route through a door |

## Order

`rim-side-and-rim-dir-group-signs-are-facts-about-cycle-order` first:
`props_rim_side` and `props_rim_dir_group` read whichever rim the loop
walk from `Cycle::first` meets first, so their recorded signs are
facts about CYCLE ORDER rather than about the face. Everything else
this program computes about a rim rests on those two.

Then `rim-level-rule-manufactures-its-error-by-feeding-nan-into-classify`
— `unreachable_zero` returning a 4-tuple of NaNs into live flux
arithmetic is the one row here that corrupts a shipped number rather
than refusing.

The three refusal rows on ordinary revolved shapes
(`cone-apex-cap-refuses-degenerateface`, `sphere-flux-arm-refuses-partial-bands`,
`sphere-wedge-arm-does-not-fold-split-meridians-by-lineage`) are one
family: each is a face the kernel builds and the property layer will
not measure.

## Review posture

OPEN, for this program's first dispatch. PROPS inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
