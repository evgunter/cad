"""Type stubs for the `pncad` extension module.

`pncad` is a B-rep CAD kernel. Python speaks its DOCUMENT layer: you
insert nodes describing what to build, evaluate the document, and read
typed values out. `docs/GUIDE.md` §2.8 walks the canonical journey and
`crates/pncad-py/examples/bracket.py` is a complete script.

LIBRARY-DESIGN §L4's two-layer story: these stubs are the STATIC
layer, checked by `ty`; the runtime layer lives once at the Rust
boundary and never re-verifies inside Python. `tests/test_stubs.py`
compares this file name-for-name against the compiled module, so the
two cannot drift.

Refusals are typed: every exception below carries its payload as
ATTRIBUTES, never as prose to be parsed.

Profile authoring is the PATHS lattice (`Open`, `PathOpen`,
`PathPoint`, `PathDirectedPoint`, `PathAngle`, `PathDirected`,
the arrival builders, `Start`, `circle`,
`circle_split`) plus the straight-segment shortcut `Node.polygon`;
one loop or a list of them, on any sketch plane. Arcs — sharp legs
and the fillet family alike — are authored by SPEC MODE (`Bulge`,
`Via`, `Center`, `Radius`, `Sweep`, `ArcLen`).

Selection is materialize-then-store: the `Evaluation.all_*` doors
answer a whole kind, and `Evaluation.select` / `select_where` narrow
one — structurally (`Selector`, a union of `NamePat` role-path
shapes) and geometrically (`GeomPred`: carrier kind, adjacent-surface
kinds, datum distance). Every answer is opaque name TEXT, the one
alphabet `Node.fillet` reads; narrowing happens through these doors,
never by parsing a name.

The mesh door is `Body.tessellate(chordal)` and the `Mesh` it
answers: one shared position buffer, one triangle patch per face, and
STL out. The mesh-vs-exact cross-check is a computation the CALLER
writes over those triangles — `docs/guide/meshing.md` shows it — so
it is genuinely a second measure and not a second reading.

Deliberately ABSENT, and tracked as named gaps in
`docs/guide/north-star-audit.md`: sweep and tube, the pattern node,
chamfer and shell (which have no recipe node at all), and the whole
assembly series.
"""

from typing import Any, Final, Generic, Optional, TypeAlias, TypeVar, overload

# --- errors -----------------------------------------------------------
# Every subclass carries its refusal as ATTRIBUTES, never as parsed
# prose.

class PncadError(Exception):
    """Base class for every refusal this module raises."""

class EditError(PncadError):
    """The document layer refused an edit."""

    variant: str

class EvaluationError(PncadError):
    """A node produced no value, or produced the wrong kind.

    `reason` is `unknown_node`, `wrong_kind`, `empty_boolean`,
    `node_failed`, or `poisoned`. `kind` (the `NodeErrorKind`'s
    stable tag), `through` (the nearest failed ancestor) and
    `finding` (the refusal-menu payload) are always present, `None`
    where the reason has none (attributes never go missing).

    `finding` is the boolean's refusal MENU: when
    `kind == "undeclared_contact"`, it carries the candidate
    declaration as a typed `FlushFinding` — the same value
    `Evaluation.find_flush_candidates` answers with, ready for
    `Node.declare` / `Doc.declare`. The menu has exactly two arms:
    declare that finding, or move the geometry.
    """

    reason: str
    node: NodeId
    kind: Optional[str]
    through: Optional[NodeId]
    finding: Optional[FlushFinding]

class ValidationError(PncadError):
    """A body failed a validator, or mass properties could not be taken."""

    door: str
    failure_count: int

class DimensionError(PncadError):
    """An operator applied to two QUANTITIES whose dimensions do not
    admit it — `1 * m + 1 * rad`.

    The quantity boundary only, and not the library's only dimension
    check. The document layer's own refusal type reaches Python
    through literal construction (as LiteralError) and through `load`,
    where a save file's ill-dimensioned expression arrives as
    PersistError with `variant == "parse"` rather than as any
    dimension class (issue #694)."""

    op: str
    left: str
    right: str

class LiteralError(PncadError):
    """A value the expression layer refused (`Expr::literal`'s own
    curated error). `value` is the offending number.

    Not DimensionError, which is the quantity boundary's operator
    check. The expression layer's refusal type has dimension-mismatch
    arms too, and `load` does reach them from a hand-edited save file
    — but they arrive as PersistError with `variant == "parse"`, not
    here (issue #694). Every `kind` raised on this class is a
    literal-value refusal."""

    kind: str
    value: float

class PersistError(PncadError):
    """A save or load the persistence doors refused."""

    variant: str

class ExportError(PncadError):
    """The document-layer export door refused.
    `through` (poisoning ancestor) and `kind` (the wrong-kind value's
    tag) are always present, `None` where inapplicable."""

    variant: str
    node: NodeId
    through: Optional[NodeId]
    kind: Optional[str]

class TessellateError(PncadError):
    """The tessellator refused a body.

    `variant` is the refusing arm's tag —
    `invalid_chordal_tolerance`, `unsupported_surface`,
    `unsupported_nurbs_face`, `unsupported_curve`,
    `null_scaffold_edge`, `ring_on_curved_face`, `empty_loop`,
    `missing_entity`, `resolution_overflow`, `certificate_exceeded`,
    `triangulation`, `self_touching_trim_loop` or
    `unsupported_curved_domain`.

    The offending face or edge is an arena KEY and does not cross, so
    what a caller reads beside the tag is the arm's NUMBERS, always
    present and `None` where inapplicable: `value` (the refused
    budget, the overflowed count, or the curved domain's worst
    off-box distance in metres), `bound` / `requested` (a failed
    deviation certificate against the budget it was checked to), and
    `note` (the arm's own prose about which lane would be needed)."""

    variant: str
    value: Optional[float]
    bound: Optional[float]
    requested: Optional[float]
    note: Optional[str]

class StlError(PncadError):
    """An STL export refused.

    `variant` is `degenerate_triangle`, `index_out_of_range`,
    `too_many_triangles`, `io` or `not_utf8` from the writers, or
    `solid_name_unrepresentable`, `binary_header_too_long` or
    `binary_header_sniffs_ascii` from the two validated option values
    — which are keyword arguments here, so they refuse the same call
    and share this class and its tag namespace."""

    variant: str

class StepImportError(PncadError):
    """A STEP text the importer refused, or one that parsed to a
    non-solid.

    `variant` is the importer's own refusal tag — `syntax`,
    `dangling_reference`, `wrong_entity_type`, `malformed_record`,
    `unsupported_entity`, `unsupported_unit`, `nothing_to_import`,
    `structure`, `missing_uncertainty`, `invalid_eps_override`,
    `declaration_unresolved`, `malformed_real`, `topology`,
    `assembly`, `adoption`, `rim_off_wall_boundary`,
    `recognition_ambiguous`, `pcurves`, `placement`, `instance` or
    `tier_invalid` — or `wireframe`, which is not a refusal at all:
    the file parsed, to something this door does not adopt."""

    variant: str

class PathError(PncadError):
    """The PATHS authoring algebra refused the geometry, at the call
    site of the verb that wrote it."""

    variant: str

class SelectRefusal(PncadError):
    """`Evaluation.select_where` could not answer — the Rust door's
    own typed refusal, crossing under its own name.

    `reason` is `in_band`, `tied_disagrees`, `unreadable`,
    `not_a_datum`, `not_a_length`, `pair_in_band`, `bad_value`, or
    `band`. The other attributes are the refusing arm's payload,
    always present and `None` where inapplicable: `name` (the
    candidate's opaque name text), `predicate` (the funnel site),
    `matched`/`candidates` (a tied name's disagreement counts),
    `datum` (the non-datum reference), `found` (what it evaluated to),
    `dim` (a non-length comparand's dimension tag)."""

    reason: str
    name: Optional[str]
    predicate: Optional[str]
    matched: Optional[int]
    candidates: Optional[int]
    datum: Optional[NodeId]
    found: Optional[str]
    dim: Optional[str]

class MateError(PncadError):
    """The mate solve could not place an instance.

    `variant` is the refusing arm's stable tag; `fault` is the
    `MateFault` VALUE carrying the arm's payload.

    The solve itself is TOTAL and never raises — a refusing cluster
    must not fail an unrelated one, so `solve_document` records the
    fault per node and `SolvedPoses.fault` hands back the same value
    this exception carries. Raised only where an answer is a pose or
    nothing: `SolvedPoses.placement`."""

    variant: str
    fault: MateFault

class AssemblyError(PncadError):
    """The at-rest assembly gate refused.

    `variant` is the refusing arm's stable tag; `mate`, `side`,
    `name`, `why`, `class_`, `findings`, `node`, `through` are the
    arms' payloads, present on every arm and `None` where that arm
    does not carry one.

    The two verdict arms are NOT interchangeable. `at_rest` is a
    finding AGAINST the document — a refuted declaration or an
    undeclared contact. `uncertified` is the declared direction's
    FRONTIER: nothing refuted, nothing undeclared, the census simply
    declined to certify, so nothing was decided about the geometry
    either way. A gather refusal arrives under the GATHER's own tag
    (`no_body_roots`, `root_failed`, ...), not a wrapper tag."""

    variant: str
    mate: Optional[NodeId]
    side: Optional[MateSide]
    name: Optional[str]
    why: Optional[RefusedRef]
    class_: Optional[ContactClass]
    findings: Optional[list[AtRestFinding]]
    node: Optional[NodeId]
    through: Optional[NodeId]

class ProductError(PncadError):
    """The whole-document gather refused. A product is all of the
    roots or none of them — there are no partial products."""

    variant: str
    node: Optional[NodeId]
    through: Optional[NodeId]
    name: Optional[str]

class SplitError(PncadError):
    """The `split` refactoring refused."""

    variant: str
    node: Optional[NodeId]
    consumer: Optional[NodeId]
    input: Optional[NodeId]
    gauge: Optional[NodeId]
    instance: Optional[NodeId]
    param: Optional[str]
    name: Optional[str]
    id: Optional[str]

class InlineError(PncadError):
    """The `inline` refactoring refused.

    Inline crosses the same document seam evaluation does, so a
    reference that will not resolve refuses under the SEAM's tags —
    `part_pin_mismatch`, `part_epsilon_seam`, `part_unresolved`. A
    stale pin is refused, never silently retargeted."""

    variant: str
    node: Optional[NodeId]
    by: Optional[NodeId]
    name: Optional[str]
    param: Optional[str]
    key: Optional[str]
    root: Optional[NodeId]
    host_epsilon: Optional[float]
    part_epsilon: Optional[float]

class UpdateError(PncadError):
    """A whole-document pin update produced no edit list.

    `variant` is `no_such_reference` (a typo or a stale id — never a
    silent success) or `already_pinned` (a completed update). Both
    name `id`, because "which part did you mean" is the only question
    an author can act on here; `pin` rides `already_pinned` alone."""

    variant: str
    id: str
    pin: Optional[ContentPin]

class FrameError(PncadError):
    """A frame constructor refused its inputs — a direction that was
    not DEFINITELY usable, or a tolerance yielding no usable band.

    `variant` is `degenerate_aim`, `degenerate_tangent`,
    `degenerate_roll_reference`, `degenerate_reference_ladder`,
    `degenerate_mirror_normal`, or `band`."""

    variant: str

class IdentityError(PncadError):
    """A document identity could not be minted. Identity is never
    defaulted — two documents sharing an id are the same part, and a
    workspace refuses to hold both.

    `variant` is the workspace refusal's own tag:
    `randomness_unavailable`, `io`, `duplicate_id`, `header`,
    `unknown_id`, `load`, `pin`, `pin_mismatch`, `save` or `update`.
    Only `randomness_unavailable` is reachable through this door today
    — minting an identity has one failure mode, the OS entropy source
    refusing — but the tag names the refusal that actually occurred, so
    a second one would arrive under its own name rather than this
    one."""

    variant: str

class WorkspaceError(PncadError):
    """The workspace store refused. Fail-loud throughout: no scan is
    best-effort and no reference is silently retargeted.

    `variant` is the refusing arm's stable tag: `io`, `duplicate_id`,
    `header`, `unknown_id`, `load`, `pin`, `pin_mismatch`, `save`,
    `randomness_unavailable` or `update`.

    The arm's payload rides as attributes, every one present on every
    arm and `None` where that arm does not carry it — so error
    handling reads `err.wanted` without first branching on
    `variant`. `path` is the file or directory the door touched;
    `id` the document identity at issue; `first`/`second` the two
    files of a `duplicate_id`; `wanted`/`found` the two pins of a
    `pin_mismatch`.

    `pin_mismatch` is the arm the store exists to make loud: a
    `DocRef` names a VERSION, so a document edited since it was
    pinned refuses rather than resolving to the new content. The
    message ends on `PIN_MISMATCH_RECOURSE`."""

    variant: str
    path: Optional[str]
    id: Optional[str]
    first: Optional[str]
    second: Optional[str]
    wanted: Optional[ContentPin]
    found: Optional[ContentPin]

# --- quantities -------------------------------------------------------
# Canonical metres and radians underneath. The arithmetic is
# exactly `crates/quantity`'s infallible subset; anything else raises
# DimensionError.

class Length:
    """A length. Construct as `25 * mm`."""

    @property
    def meters(self) -> float: ...
    def in_unit(self, unit: LengthUnit) -> float: ...
    def __add__(self, other: Length) -> Length: ...
    def __sub__(self, other: Length) -> Length: ...
    def __neg__(self) -> Length: ...
    def __mul__(self, other: float) -> Length: ...
    def __rmul__(self, other: float) -> Length: ...
    def __truediv__(self, other: float) -> Length: ...
    def __lt__(self, other: Length) -> bool: ...
    def __le__(self, other: Length) -> bool: ...
    def __gt__(self, other: Length) -> bool: ...
    def __ge__(self, other: Length) -> bool: ...
    def __hash__(self) -> int: ...

class Angle:
    """An angle. Construct as `90 * deg`."""

    @property
    def radians(self) -> float: ...
    def in_unit(self, unit: AngleUnit) -> float: ...
    def __add__(self, other: Angle) -> Angle: ...
    def __sub__(self, other: Angle) -> Angle: ...
    def __neg__(self) -> Angle: ...
    def __mul__(self, other: float) -> Angle: ...
    def __rmul__(self, other: float) -> Angle: ...
    def __truediv__(self, other: float) -> Angle: ...
    def __lt__(self, other: Angle) -> bool: ...
    def __le__(self, other: Angle) -> bool: ...
    def __gt__(self, other: Angle) -> bool: ...
    def __ge__(self, other: Angle) -> bool: ...
    def __hash__(self) -> int: ...

class Count:
    """A dimensionless integer count.

    Has no arithmetic, mirroring `quantity::Count`: D4's checked count
    algebra lives in the expression layer, not in this newtype.
    """

    def __init__(self, value: int) -> None: ...
    @property
    def value(self) -> int: ...
    def __hash__(self) -> int: ...

class LengthUnit:
    @property
    def symbol(self) -> str: ...
    @property
    def factor(self) -> float: ...
    def __mul__(self, value: float) -> Length: ...
    def __rmul__(self, value: float) -> Length: ...

class AngleUnit:
    @property
    def symbol(self) -> str: ...
    @property
    def factor(self) -> float: ...
    def __mul__(self, value: float) -> Angle: ...
    def __rmul__(self, value: float) -> Angle: ...

mm: Final[LengthUnit]
cm: Final[LengthUnit]
m: Final[LengthUnit]
inch: Final[LengthUnit]  # `in` is a Python keyword; `quantity` spells it IN
deg: Final[AngleUnit]
rad: Final[AngleUnit]

# --- profile authoring: the PATHS lattice ------------------------------
# PATHS-DESIGN §2. The tip's state is exactly which of {position,
# angle} it has bound, and each state is its OWN class exposing only
# its legal continuations — so a double director, `.tangent()` on a
# plain point, or a leading `.fillet` is a static error here and an
# AttributeError at runtime, exactly as it is an E0599 in Rust.
#
# Two spellings of one verb (an authored point, or `Start`) are
# @overload pairs, because the RETURN follows the target: targeting
# `Start` closes the loop.

class StartToken:
    """The type of `Start`, the bound entry as a value."""

class ArcSweep:
    """Travel sense about a centre — structural, never a value."""

    Ccw: Final[ArcSweep]
    Cw: Final[ArcSweep]

class ClosedLoop:
    """A closed loop and the program that produced it."""

    @property
    def vertex_count(self) -> int: ...
    @property
    def step_count(self) -> int: ...

_T = TypeVar("_T")

class ArcSide:
    """Which half-plane of the departure tangent a DERIVED centre sits
    on — structural, the one discrete bit the derived modes carry."""

    Left: Final[ArcSide]
    Right: Final[ArcSide]

# The `ArcData` spec modes (PATHS-DESIGN §2c). Each is a standalone
# value; which ones a verb accepts IS the admissibility matrix, and the
# target parameter carries whether the mode ENDS the chain: `p=Start`
# closes, so the mode's own type says what the verb returns.

class Bulge(Generic[_T]):
    """`Bulge(p, b)` — chord-relative: legs and fused incomings only."""

    def __init__(self, p: _T, b: float) -> None: ...

class Via(Generic[_T]):
    """`Via(q, p)` — the arc through an authored point."""

    def __init__(self, q: tuple[Length, Length], p: _T) -> None: ...

class Center(Generic[_T]):
    """`Center(c, winding, p)` — the arc about an authored centre."""

    def __init__(
        self, c: tuple[Length, Length], winding: ArcSweep, p: _T
    ) -> None: ...

class Radius:
    """`Radius(r, side)` — centre DERIVED from the directed anchor."""

    def __init__(self, r: Length, side: ArcSide) -> None: ...

class Sweep:
    """`Sweep(r, side, angle)` — endpoint-free, extent as an angle."""

    def __init__(self, r: Length, side: ArcSide, angle: Angle) -> None: ...

class ArcLen:
    """`ArcLen(r, side, len)` — endpoint-free, extent as arc length."""

    def __init__(self, r: Length, side: ArcSide, len: Length) -> None: ...

_Pt: TypeAlias = tuple[Length, Length]
# The endpoint-full modes at an interior target, at `Start`, and the
# incoming-side set (a fused verb's incoming ends at its own anchor, so
# `Start` is not one of them).
_PointLeg: TypeAlias = Bulge[_Pt] | Via[_Pt] | Center[_Pt]
# A DIRECTED-POINT tip's fused-incoming set: the endpoint-full trio
# plus `Radius` — arc extension (carrier derived from the tip's own
# position and tangent; the incoming side's anchor is the tip).
_LegEndIncoming: TypeAlias = Bulge[_Pt] | Via[_Pt] | Center[_Pt] | Radius
_PointClose: TypeAlias = Bulge[StartToken] | Via[StartToken] | Center[StartToken]
_Tangent: TypeAlias = Sweep | ArcLen

class Open:
    """The entry: nothing bound. A token, not a value you construct."""

    @staticmethod
    def at(p: tuple[Length, Length]) -> PathPoint: ...
    @staticmethod
    def angle(theta: Angle) -> PathAngle: ...
    @staticmethod
    def toward(dx: float, dy: float) -> PathAngle: ...
    @staticmethod
    def arc_fillet(spec: Center[_Pt], radius: Length) -> PathOpen: ...
    @overload
    @staticmethod
    def arc_fillet_arc(
        spec: Center[_Pt], radius: Length, spec2: Center[_Pt]
    ) -> PathDirectedPoint: ...
    @overload
    @staticmethod
    def arc_fillet_arc(
        spec: Center[_Pt], radius: Length, spec2: Center[StartToken]
    ) -> ClosedLoop: ...
    @overload
    @staticmethod
    def arc_fillet_arc(
        spec: Center[_Pt], radius: Length, spec2: Radius
    ) -> PathRadiusArrival: ...
    @overload
    @staticmethod
    def arc_fillet_arc(
        spec: Center[_Pt], radius: Length, spec2: Via[_Pt]
    ) -> PathViaArrival: ...
    @overload
    @staticmethod
    def arc_fillet_arc(
        spec: Center[_Pt], radius: Length, spec2: Via[StartToken]
    ) -> PathViaArrivalStart: ...

class PathOpen:
    """A fillet's freshly opened LINE arrival side: nothing bound, and
    `Start` reachable because the entry is behind us. An ARC arrival is
    authored in the fillet verb itself, never here."""

    def at(self, p: tuple[Length, Length]) -> PathPoint: ...
    def angle(self, theta: Angle) -> PathAngle: ...
    def toward(self, dx: float, dy: float) -> PathAngle: ...
    def to(self, target: StartToken) -> ClosedLoop: ...

class PathAngle:
    """Direction bound, position pending."""

    def at(self, p: tuple[Length, Length]) -> PathDirected: ...
    def to(self, anchor: tuple[Length, Length]) -> PathDirectedPoint: ...

class PathRadiusArrival:
    """A `Radius` arrival awaiting both binders, in either order."""

    def at(self, p: tuple[Length, Length]) -> PathRadiusArrivalAt: ...
    def angle(self, theta: Angle) -> PathRadiusArrivalDir: ...
    def toward(self, dx: float, dy: float) -> PathRadiusArrivalDir: ...

class PathRadiusArrivalAt:
    """A `Radius` arrival with its anchor bound, director pending."""

    def angle(self, theta: Angle) -> PathDirectedPoint: ...
    def toward(self, dx: float, dy: float) -> PathDirectedPoint: ...

class PathRadiusArrivalDir:
    """A `Radius` arrival with its director bound, anchor pending."""

    def at(self, p: tuple[Length, Length]) -> PathDirectedPoint: ...

class PathViaArrival:
    """A `Via` arrival: the anchor rides the spec, one director left."""

    def angle(self, theta: Angle) -> PathDirectedPoint: ...
    def toward(self, dx: float, dy: float) -> PathDirectedPoint: ...

class PathViaArrivalStart:
    """A `Via` arrival that CLOSES: one director left, at the entry."""

    def angle(self, theta: Angle) -> ClosedLoop: ...
    def toward(self, dx: float, dy: float) -> ClosedLoop: ...

class PathPoint:
    """A plain point: position bound, no incoming carrier. There is
    nothing to inherit here, so `tangent` and `turn` are absent."""

    def angle(self, theta: Angle) -> PathDirected: ...
    def toward(self, dx: float, dy: float) -> PathDirected: ...
    @overload
    def line_to(self, target: tuple[Length, Length]) -> PathDirectedPoint: ...
    @overload
    def line_to(self, target: StartToken) -> ClosedLoop: ...
    @overload
    def arc_to(self, spec: _PointLeg) -> PathDirectedPoint: ...
    @overload
    def arc_to(self, spec: _PointClose) -> ClosedLoop: ...
    def arc_fillet(self, spec: _PointLeg, radius: Length) -> PathOpen: ...
    @overload
    def arc_fillet_arc(
        self, spec: _PointLeg, radius: Length, spec2: Center[_Pt]
    ) -> PathDirectedPoint: ...
    @overload
    def arc_fillet_arc(
        self, spec: _PointLeg, radius: Length, spec2: Center[StartToken]
    ) -> ClosedLoop: ...
    @overload
    def arc_fillet_arc(
        self, spec: _PointLeg, radius: Length, spec2: Radius
    ) -> PathRadiusArrival: ...
    @overload
    def arc_fillet_arc(
        self, spec: _PointLeg, radius: Length, spec2: Via[_Pt]
    ) -> PathViaArrival: ...
    @overload
    def arc_fillet_arc(
        self, spec: _PointLeg, radius: Length, spec2: Via[StartToken]
    ) -> PathViaArrivalStart: ...

class PathDirectedPoint:
    """A leg end: position bound, and the leg's incoming end tangent
    available as read-only intrinsic data."""

    def angle(self, theta: Angle) -> PathDirected: ...
    def toward(self, dx: float, dy: float) -> PathDirected: ...
    def tangent(self) -> PathDirected: ...
    def turn(self, delta: Angle) -> PathDirected: ...
    def arc_continue(self, target: tuple[Length, Length]) -> PathDirectedPoint: ...
    def fillet(self, radius: Length) -> PathOpen: ...
    @overload
    def fillet_arc(self, radius: Length, spec: Center[_Pt]) -> PathDirectedPoint: ...
    @overload
    def fillet_arc(self, radius: Length, spec: Center[StartToken]) -> ClosedLoop: ...
    @overload
    def fillet_arc(self, radius: Length, spec: Radius) -> PathRadiusArrival: ...
    @overload
    def fillet_arc(self, radius: Length, spec: Via[_Pt]) -> PathViaArrival: ...
    @overload
    def fillet_arc(
        self, radius: Length, spec: Via[StartToken]
    ) -> PathViaArrivalStart: ...
    @overload
    def line_to(self, target: tuple[Length, Length]) -> PathDirectedPoint: ...
    @overload
    def line_to(self, target: StartToken) -> ClosedLoop: ...
    @overload
    def arc_to(self, spec: _PointLeg) -> PathDirectedPoint: ...
    @overload
    def arc_to(self, spec: _PointClose) -> ClosedLoop: ...
    def arc_fillet(self, spec: _LegEndIncoming, radius: Length) -> PathOpen: ...
    @overload
    def arc_fillet_arc(
        self, spec: _LegEndIncoming, radius: Length, spec2: Center[_Pt]
    ) -> PathDirectedPoint: ...
    @overload
    def arc_fillet_arc(
        self, spec: _LegEndIncoming, radius: Length, spec2: Center[StartToken]
    ) -> ClosedLoop: ...
    @overload
    def arc_fillet_arc(
        self, spec: _LegEndIncoming, radius: Length, spec2: Radius
    ) -> PathRadiusArrival: ...
    @overload
    def arc_fillet_arc(
        self, spec: _LegEndIncoming, radius: Length, spec2: Via[_Pt]
    ) -> PathViaArrival: ...
    @overload
    def arc_fillet_arc(
        self, spec: _LegEndIncoming, radius: Length, spec2: Via[StartToken]
    ) -> PathViaArrivalStart: ...

class PathDirected:
    """Both bits bound — the only state legs and `fillet` consume.
    The outgoing angle slot is full, so no second director exists."""

    def line(self, len: Length) -> PathDirectedPoint: ...
    def fillet(self, radius: Length) -> PathOpen: ...
    @overload
    def fillet_arc(self, radius: Length, spec: Center[_Pt]) -> PathDirectedPoint: ...
    @overload
    def fillet_arc(self, radius: Length, spec: Center[StartToken]) -> ClosedLoop: ...
    @overload
    def fillet_arc(self, radius: Length, spec: Radius) -> PathRadiusArrival: ...
    @overload
    def fillet_arc(self, radius: Length, spec: Via[_Pt]) -> PathViaArrival: ...
    @overload
    def fillet_arc(
        self, radius: Length, spec: Via[StartToken]
    ) -> PathViaArrivalStart: ...
    def arc_to(self, spec: _Tangent) -> PathDirectedPoint: ...
    def arc_fillet(self, spec: _Tangent, radius: Length) -> PathOpen: ...
    @overload
    def arc_fillet_arc(
        self, spec: _Tangent, radius: Length, spec2: Center[_Pt]
    ) -> PathDirectedPoint: ...
    @overload
    def arc_fillet_arc(
        self, spec: _Tangent, radius: Length, spec2: Center[StartToken]
    ) -> ClosedLoop: ...
    @overload
    def arc_fillet_arc(
        self, spec: _Tangent, radius: Length, spec2: Radius
    ) -> PathRadiusArrival: ...
    @overload
    def arc_fillet_arc(
        self, spec: _Tangent, radius: Length, spec2: Via[_Pt]
    ) -> PathViaArrival: ...
    @overload
    def arc_fillet_arc(
        self, spec: _Tangent, radius: Length, spec2: Via[StartToken]
    ) -> PathViaArrivalStart: ...
    @overload
    def tangent_arc_to(self, target: tuple[Length, Length]) -> PathDirectedPoint: ...
    @overload
    def tangent_arc_to(self, target: StartToken) -> ClosedLoop: ...

Start: Final[StartToken]

def circle(centre: tuple[Length, Length], radius: Length) -> ClosedLoop: ...
def circle_split(
    centre: tuple[Length, Length],
    radius: Length,
    n: int,
    phase: Angle,
) -> ClosedLoop: ...

# --- document ---------------------------------------------------------

class NodeId:
    """A recipe node's identity. NOT an arena key."""

    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class BooleanOp:
    """The regularized Boolean operator."""

    Union: Final[BooleanOp]
    Intersect: Final[BooleanOp]
    Subtract: Final[BooleanOp]

class SketchPlane:
    """The rigid placement of a sketch plane in 3-space.

    Sketch (x, y) maps to world `origin + x*u + y*v`, and the plane's
    NORMAL is `u x v` — the direction `Node.extrude` runs, so the
    plane is what chooses an extrusion's axis. The named frames are
    cyclic (x->y->z->x): `xy` has normal +z, `yz` (u = y, v = z) has
    normal +x, `zx` (u = z, v = x) has normal +y.

    Rigidity — u, v unit and perpendicular — is CONVENTIONAL DATA,
    UNCHECKED, exactly as in Rust: nothing here verifies it, and a
    non-rigid frame yields a well-defined skewed sketch rather than
    poison. Geometric certification is the kernel's tier-3 validation
    at rest; the binding adds no orthogonality predicate of its own.
    """

    @staticmethod
    def xy() -> SketchPlane: ...
    @staticmethod
    def yz() -> SketchPlane: ...
    @staticmethod
    def zx() -> SketchPlane: ...
    @staticmethod
    def from_frame(
        origin: tuple[Length, Length, Length],
        u: tuple[float, float, float],
        v: tuple[float, float, float],
    ) -> SketchPlane: ...
    @property
    def origin(self) -> tuple[Length, Length, Length]: ...
    @property
    def u(self) -> tuple[float, float, float]: ...
    @property
    def v(self) -> tuple[float, float, float]: ...
    @property
    def normal(self) -> tuple[float, float, float]: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

    # Equality is BIT-exact — Rust's `SketchPlane::bit_eq`, crossing
    # unchanged. A sketch plane carries no epsilon, so `-0.0` keeps its
    # own identity rather than being folded into `0.0`.

class Frame:
    """An ABSOLUTE placement: a linear part and a translation.

    The value `PatternKind.explicit` lists and `Node.placed_union_at`
    places a prototype at. It is authored through constructors rather
    than field by field — a linear part is a MATRIX, and that is how a
    mirror or a shear arrives by accident. Improper frames
    (determinant <= 0) are refused at the edit door, not admitted.

    Rigidity is not claimed here: the edit door checks that the
    coordinates are finite and the determinant positive, and the
    kernel's placement door owns the rest.
    """

    @staticmethod
    def translation(v: tuple[Length, Length, Length]) -> Frame: ...
    @staticmethod
    def rotate_then_translate(
        axis: tuple[float, float, float],
        angle: Angle,
        v: tuple[Length, Length, Length],
    ) -> Frame:
        """Rotate about `axis` through the WORLD ORIGIN, THEN
        translate — `Node.transform`'s own order, so a placement and a
        modeled transform of the same part agree BIT FOR BIT. A
        zero-length axis yields a non-finite frame, refused typed at
        the edit door."""

    @staticmethod
    def point_at(
        eye: tuple[Length, Length, Length],
        target: tuple[Length, Length, Length],
        roll_reference: tuple[float, float, float],
    ) -> Frame:
        """The frame at `eye` whose local +Z aims at `target`, rolled
        by `roll_reference`. A reference parallel to the aim, or a
        zero-length aim, raises FrameError."""

    @staticmethod
    def path_start_frame(
        origin: tuple[Length, Length, Length],
        tangent: tuple[float, float, float],
    ) -> Frame:
        """The frame at `origin` whose local +Z is `tangent`, so the
        local XY plane is a swept profile's plane. The roll comes from
        a stated ladder (world +Z, then world +X); if neither is
        definitely off the tangent line it raises FrameError."""

    @staticmethod
    def mirror_across_plane(
        point: tuple[Length, Length, Length],
        normal: tuple[float, float, float],
    ) -> Frame:
        """Reflection across the plane through `point` — an IMPROPER
        frame (determinant -1). Constructible as a value; NOT
        placeable, since the edit door refuses a mirror placement."""

    @property
    def columns(self) -> tuple[
        tuple[float, float, float],
        tuple[float, float, float],
        tuple[float, float, float],
    ]: ...
    @property
    def origin(self) -> tuple[Length, Length, Length]:
        """The image of the coordinate origin — the Rust value's
        `translation` field. Spelled `origin` because `translation` is
        already this class's pure-translation constructor."""

    @property
    def determinant(self) -> float: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

    # Equality is BIT-exact — Rust's `Frame::bit_eq`, crossing
    # unchanged: a frame carries no epsilon, so `-0.0` keeps its own
    # identity rather than being folded into `0.0`.

class PatternKind:
    """A pattern's replication rule: how a prototype's placements are
    generated.

    Three rules and no fourth. The two PARAMETRIC ones step and take
    their count from the node's structural slot; the EXPLICIT one
    lists absolute frames, and the list IS the count — which is why
    `Node.placed_union_at` takes no count at all.
    """

    @staticmethod
    def linear(
        direction: tuple[float, float, float], spacing: Length
    ) -> PatternKind: ...
    @staticmethod
    def circular(axis: NodeId, step: Angle) -> PatternKind:
        """Stepped around `axis`, an upstream `datum_axis` node."""

    @staticmethod
    def explicit(frames: list[Frame]) -> PatternKind:
        """Instances at the frames listed, in order — the index is
        what a name's instance segment carries. An empty list raises
        EditError (`empty_placement_list`) at insert."""

class Node:
    """A recipe node, before insertion."""

    @staticmethod
    def polygon(
        points: list[tuple[Length, Length]],
        elevation: Optional[Length] = None,
        plane: Optional[SketchPlane] = None,
    ) -> Node: ...
    @overload
    @staticmethod
    def profile(
        outline: ClosedLoop,
        elevation: Optional[Length] = None,
        plane: Optional[SketchPlane] = None,
    ) -> Node: ...
    @overload
    @staticmethod
    def profile(
        outline: list[ClosedLoop],
        elevation: Optional[Length] = None,
        plane: Optional[SketchPlane] = None,
    ) -> Node: ...
    @staticmethod
    def extrude(profile: NodeId, distance: Length) -> Node: ...
    @staticmethod
    def revolve(profile: NodeId, axis: NodeId, angle: Angle) -> Node: ...
    @staticmethod
    def loft(profiles: list[NodeId], v_degree: int) -> Node: ...
    @staticmethod
    def datum_axis(
        origin: tuple[Length, Length, Length],
        direction: tuple[float, float, float],
    ) -> Node: ...
    @staticmethod
    def datum_plane(
        origin: tuple[Length, Length, Length],
        normal: tuple[float, float, float],
    ) -> Node: ...
    @staticmethod
    def fillet(target: NodeId, radius: Length, selection: list[str]) -> Node:
        """Constant-radius blends on named edges of `target`.

        `selection` is edge names as TEXT — the strings
        `Evaluation.all_edges` answers with. The set FREEZES at
        authoring time; an empty one, an unresolvable name, or an edge
        the roller cannot enter refuses typed at `evaluate`.
        """

    @staticmethod
    def split(target: NodeId, tool: NodeId) -> Node:
        """Split `target` by `tool` (a `datum_plane`). The value is a
        split — read it with `Value.split()`, not `Value.body()`."""

    @staticmethod
    def transform(
        input: NodeId,
        translation: tuple[Length, Length, Length],
        rotation_axis: tuple[float, float, float],
        rotation_angle: Angle,
    ) -> Node:
        """A rigid placement: rotate about `rotation_axis` through the
        WORLD ORIGIN by `rotation_angle`, then translate. A pure
        translation passes any non-degenerate axis and a zero angle;
        a zero-length axis refuses rather than meaning "no rotation".
        """

    @staticmethod
    def boolean(
        op: BooleanOp, a: NodeId, b: NodeId, declare: Optional[NodeId] = None
    ) -> Node:
        """A Boolean of two upstream solids. `declare` names a
        `Declare` node whose coincidence pairs this boolean consumes;
        without one, operands that merely TOUCH refuse with the typed
        menu (`EvaluationError`, `kind == "undeclared_contact"`,
        `finding` attached) — the kernel never infers that two faces
        are the same face."""

    @staticmethod
    def declare(findings: list[FlushFinding]) -> Node:
        """The `Declare` node built from INSPECTED findings; its
        inserted id feeds `Node.boolean`'s `declare=`. Nothing here
        detects (the ruled no-fusion boundary), and an empty list
        raises EditError (`no_findings`)."""

    @staticmethod
    def placed_union(input: NodeId, count: int, kind: PatternKind) -> Node:
        """The group boolean over a PARAMETRIC rule: one prototype,
        `count` placements stepped by `kind`, ONE body out.

        The value is an ordinary body, so every downstream door
        consumes it with no new arms — which is exactly what a
        pattern's plural payload cannot do. `count` is a plain `int`,
        the structural-slot exception to the typed-quantity rule
        (`Node.loft`'s `v_degree` precedent): a Count is an integer in
        the kernel's own expression language, not a measurement.

        Disjointness is CERTIFIED: overlapping placements raise
        EvaluationError (`placements_uncertified`) naming the pair,
        and the certificate is sufficient-not-necessary, so a
        touching-but-disjoint arrangement refuses too. An `explicit`
        rule raises EditError (`placement_rule_mismatch`) here — it
        carries its own count, and `placed_union_at` is its door."""

    @staticmethod
    def placed_union_at(input: NodeId, frames: list[Frame]) -> Node:
        """The group boolean over LISTED absolute frames. No count
        argument, because the list IS the count. An empty list, a
        non-finite frame, or an improper one raises EditError at
        insert."""

    @staticmethod
    def instantiate_part(reference: DocRef) -> Node:
        """An instance of another document's product: a LEAF whose
        material crosses the document seam.

        `reference` is which part, at which version — Cargo.lock
        semantics, so an edit to the referenced document never
        retargets it and moving the pin is its own recorded edit
        (`DocEdit.update_reference`, or `update_references` for every
        site at once).

        No frame argument: placement lives on the CLUSTER, which is
        what makes zero-anchor and multi-anchor states
        unrepresentable rather than merely refused —
        `DocEdit.set_placement` is the door. No interface record
        either: an AUTHORED instance crosses nothing, and a non-empty
        record is mintable only by the `split` that observed
        declarations crossing its cut.

        Evaluating one needs a resolver: `evaluate(doc,
        resolver=workspace)`. Without one it refuses typed
        (`part_no_resolver`) rather than pretending the part is
        empty."""

    @staticmethod
    def mate(
        a: str,
        b: str,
        class_: ContactClass,
        alignment: Alignment,
    ) -> Node:
        """A mate between two instances: ONE node carrying both the
        placement constraint and the contact declaration.

        `a` and `b` are instance-qualified names — an entity of one
        instance's product and an entity of the other's, the text
        `Evaluation.select` answers with when queried on an
        instantiate node. They are name REFERENCES, not recipe edges:
        inserting a mate transfers no root.

        `class_` is the declared contact class; ask `class_admission`
        BEFORE authoring, because a class the solve folds may still
        mint nothing at the at-rest gate. `alignment` is AUTHORED
        data — nothing checks it against the faces `a` and `b` name,
        so a mate can solve cleanly and still be refuted at the gate.

        A dangling reference head is not refused here: the solve
        refuses typed naming it (`mate_dangling_head`)."""

class ParamName:
    """A document-level parameter name (guide §3.2). NOT an arena
    key: the same plain name the recipe's expressions reference."""

    def __init__(self, name: str) -> None: ...
    @property
    def name(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class DocParam:
    """A named parameter's declared dimension and exact stored value
    (guide §3.2): what `DocEdit.set_doc_param` writes. Continuous
    values arrive as typed quantities, so the dimension rides the
    constructor. A non-finite value is refused typed at `Doc.apply`
    (`non_finite_doc_param`), not pre-checked here."""

    @staticmethod
    def length(value: Length) -> DocParam: ...
    @staticmethod
    def angle(value: Angle) -> DocParam: ...
    @staticmethod
    def scalar(value: float) -> DocParam: ...
    @staticmethod
    def count(value: int) -> DocParam: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

    # Equality mirrors Rust's `PartialEq` — the IEEE comparison of the
    # stored value, NOT `DocParam::bit_eq`'s. So the two spellings of
    # zero are the same parameter, and the hash folds `-0.0` to match.

class DocParamValue:
    """The VALUE half of a document parameter: what
    `DocEdit.set_doc_param_value` writes into an ALREADY-DECLARED one.

    The safe "just change the number" spelling. It carries no
    declaration, so it cannot replace one — the parameter keeps its
    dimension and any distribution a file gave it."""

    @staticmethod
    def length(value: Length) -> DocParamValue: ...
    @staticmethod
    def angle(value: Angle) -> DocParamValue: ...
    @staticmethod
    def scalar(value: float) -> DocParamValue: ...
    @staticmethod
    def count(value: int) -> DocParamValue: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class DocEdit:
    """A single edit — the one edit vocabulary the GUI, the bindings and
    headless tests all speak."""

    @staticmethod
    def insert_node(node: Node) -> DocEdit: ...
    @staticmethod
    def delete_node(id: NodeId) -> DocEdit: ...
    @staticmethod
    def set_tolerance(eps: float) -> DocEdit: ...
    @staticmethod
    def set_doc_param(name: ParamName, value: DocParam) -> DocEdit: ...
    @staticmethod
    def set_doc_param_value(name: ParamName, value: DocParamValue) -> DocEdit:
        """Write a new VALUE into an already-declared parameter, keeping
        its declaration — dimension and distribution alike.

        Prefer this over `set_doc_param` whenever the parameter already
        exists: that one is create-or-replace, so rebuilding a
        `DocParam` to move a number DELETES any distribution the
        parameter carried, with no refusal. Refuses typed on an
        undeclared name (`doc_param_not_declared`) and on a kind
        mismatch (`doc_param_value_kind_mismatch`)."""
    @staticmethod
    @staticmethod
    def set_roots(roots: list[NodeId]) -> DocEdit:
        """Set the document's ordered PRODUCT ROOTS outright.

        The designate/undesignate door: one TOTAL edit rather than
        partial add/remove arms, so the product's solid order is
        always stated rather than inferred from an edit sequence. The
        four root invariants refuse under their own tags on
        `EditError` — `root_not_live`, `root_duplicate`,
        `root_ancestor` (one root upstream of another would gather its
        material twice), `root_uncovered` (a live node reaching no
        root is a silently dead subgraph)."""

    @staticmethod
    def set_placement(node: NodeId, frame: Frame) -> DocEdit:
        """Place an instance's CLUSTER.

        The frame REPLACES whatever was recorded. Placement is
        per-cluster, not per-instance: an instance coupled to others
        by mates shares their frame, and `gauge_of` says which node
        the registry is actually keyed by. Refuses typed on
        `EditError`: `placement_on_non_instance`,
        `non_finite_placement`, `improper_placement`."""

    @staticmethod
    def update_reference(node: NodeId, new_pin: ContentPin) -> DocEdit:
        """Move ONE instance's pin to a new version of the same
        document. The id does not move.

        The new pin is RECIPE DATA, not a resolution: `apply` has no
        resolver and no store, so a pin naming content that does not
        exist is accepted here and refused at EVALUATION. Checking at
        the edit door would make the edit's meaning depend on which
        store was mounted when it was recorded — which is exactly what
        a recorded, replayable log must not carry.

        Refuses `update_on_non_instance`, and `pin_unchanged` when the
        site already names that version."""

    @staticmethod
    def bind_count_param(node: NodeId, name: ParamName) -> DocEdit:
        """Bind `node`'s STRUCTURAL count slot to the document
        parameter `name`, so one `set_doc_param` re-counts the
        placements and recomputes exactly what is downstream.

        Deliberately narrow: the slot is the count and the expression
        is a parameter reference, so no expression algebra crosses and
        the edit cannot be aimed at a continuous slot. The edit's own
        refusals stay live — a node with no count slot, an unknown
        parameter, a parameter of the wrong dimension."""

class Doc:
    """A parametric document: the recipe, not the geometry."""

    def __init__(self, label: Optional[str] = None) -> None:
        """An empty document.

        `Doc()` mints a FRESH random identity, so two documents
        authored here are two parts and one workspace holds both.
        `Doc(label)` derives the id from the label instead — same
        label, same id, on every platform, which makes it the
        reproducible spelling and, deliberately, the one that makes
        two same-label documents the SAME part. Raises IdentityError
        if the OS entropy source refuses."""
    @property
    def id(self) -> str:
        """This document's identity as 32 lowercase hex digits — the
        save file's `id:` header, and the workspace store's key.
        Identity survives every edit; it is not a content hash."""
    def apply(self, edit: DocEdit) -> Optional[NodeId]: ...
    @property
    def last_maintenance(self) -> list[ClusterMaintenance]:
        """The cluster-record maintenance the LAST accepted edit
        performed. Empty after an edit that moved no mate graph, and
        on a document that has applied none; a REFUSED edit leaves it
        untouched, as it leaves the document untouched."""

    @property
    def roots(self) -> list[NodeId]:
        """The document's ordered product roots — what `product` and
        `assemble` gather, in this order. Set through
        `DocEdit.set_roots`; maintained by every other edit, so a
        document always states its product rather than leaving it to
        be inferred."""

    def placement(self, node: NodeId) -> Frame:
        """An instance's CLUSTER frame, or the identity when nothing
        was recorded. Total — use `placements` to tell "placed at the
        identity" from "carries no frame of its own". This is the
        AUTHORED frame; a mated instance's world pose is
        `SolvedPoses.placement`."""

    def placements(self) -> dict[NodeId, Frame]:
        """The placement registry itself: every node with a recorded
        cluster frame. A mated instance that is not its cluster's
        gauge is ABSENT here however it is posed."""

    def reference(self, node: NodeId) -> Optional[DocRef]:
        """The `(id, pin)` an instantiate node carries, or `None` for
        any other node — the read side of `Node.instantiate_part`."""

    def interface(self, node: NodeId) -> Optional[InterfaceRecord]:
        """The interface record an instantiate node carries, or `None`
        for any other node. Empty for a directly-authored instance;
        non-empty only on one a `split` minted."""

    def insert(self, node: Node) -> NodeId: ...
    def declare(self, finding: FlushFinding) -> NodeId:
        """Insert a `Declare` node for ONE inspected finding and
        return its id for `Node.boolean`'s `declare=` (the
        detect/declare protocol's declare arm). Raises EditError,
        typed."""

    def declare_all(self, findings: list[FlushFinding]) -> NodeId:
        """`declare` for a SET of findings in one `Declare` node —
        arity, not fusion. An empty list raises EditError
        (`no_findings`)."""
    @property
    def node_count(self) -> int: ...
    def order(self) -> list[NodeId]: ...
    @property
    def epsilon(self) -> float: ...
    def bit_eq(self, other: Doc) -> bool: ...
    def save(self) -> str: ...
    def __len__(self) -> int: ...

class Loaded:
    """A loaded document: snapshot, replayed current state, and the
    replayed edit count."""

    @property
    def doc(self) -> Doc: ...
    @property
    def snapshot(self) -> Doc: ...
    @property
    def edit_count(self) -> int: ...

def load(text: str) -> Loaded:
    """Parse, validate, and replay a saved document. Raises
    PersistError, typed."""

# --- the workspace store ----------------------------------------------
# Three vocabularies, kept apart on purpose. A document's ID answers
# WHICH PART and survives every edit (`Doc.id`, 32 hex digits). A
# `ContentPin` answers WHICH VERSION of it — the SHA-256 of the
# canonical semantic bytes, so it moves whenever the content does. A
# `DocRef` pairs them, and that pair is what a cross-document
# reference carries: editing the referenced document never silently
# retargets the reference; the store refuses the stale pin instead.

class ContentPin:
    """Which VERSION a document is: the SHA-256 of its canonical
    semantic bytes.

    Compared and stored, not read into. Construct one from the
    canonical 64-hex-digit text (`ValueError` on anything else), or
    get one from `content_pin(doc)` / `Workspace.current_pin(id)`."""

    def __init__(self, hex: str) -> None: ...
    @property
    def hex(self) -> str:
        """The canonical text form: exactly 64 lowercase hex digits."""

    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class DocRef:
    """A cross-document reference: which part, and which version of
    it.

    A VALUE — nothing here consults a store. Whether any store holds
    that version is `Workspace.resolve`'s question, and a `DocRef`
    that resolves nowhere is exactly what a stale reference is."""

    def __init__(self, id: str, pin: ContentPin) -> None:
        """Pair a 32-hex-digit identity with a pin. Raises ValueError
        if the id is not the canonical spelling."""

    @property
    def id(self) -> str:
        """The referenced document's identity, 32 lowercase hex
        digits."""

    @property
    def pin(self) -> ContentPin:
        """The pinned version."""

    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class Workspace:
    """A directory of `*.pncad` save files, scanned into an
    identity -> path map.

    The write side is deliberately minimal — `create` and `resave`,
    and no general mutation API."""

    def __init__(self, path: str) -> None:
        """Scan `path`, reading each `*.pncad` file's `id:` header
        line and never its body.

        Everything CLAIMING to be a document must scan clean: an
        unreadable header refuses the whole open, and two files
        claiming one id refuse naming both. Non-`.pncad` entries and
        subdirectories are ignored. Raises WorkspaceError, typed."""

    @property
    def root(self) -> str:
        """The scanned directory."""

    def documents(self) -> dict[str, str]:
        """The scan, as identity -> file path. Ordered by id, off a
        path-sorted scan, so it does not depend on readdir order."""

    def resolve(self, reference: DocRef) -> Doc:
        """Load the document a `DocRef` names — through the full door
        sequence `load` runs — and hand it back IFF its recomputed
        pin is the pin the reference carries.

        A moved pin raises WorkspaceError with
        `variant == "pin_mismatch"`, carrying `wanted` and `found`.
        A reference is never silently retargeted."""

    def current_pin(self, id: str) -> ContentPin:
        """The pin the store's CURRENT content for `id` hashes to —
        the version a reference would move onto. Differs from
        `resolve` by exactly one thing: no expected pin is supplied,
        so there is nothing to disagree with. Raises WorkspaceError,
        typed."""

    def create(self, doc: Doc) -> str:
        """Write `doc` into the store as a new `{id}.pncad` and answer
        its path. An id the store already holds refuses
        (`duplicate_id`) and nothing is written. Raises
        WorkspaceError, typed."""

    def resave(self, doc: Doc) -> str:
        """Rewrite an EXISTING document's file with `doc`'s current
        state, keeping its path, and answer that path. Never creates:
        an unknown id refuses. The identity is unchanged and the
        content is not, so references by id stay valid and references
        by PIN go stale — which is the point. Raises WorkspaceError,
        typed."""

    def update_to_store(self, doc: Doc, id: str) -> list[DocEdit]:
        """The edits that move every reference to `id` onto the
        version the STORE currently holds — `update_references` with
        the pin computed from disk.

        WHEN THIS READS THE STORE: once, now. The pin is recomputed
        from the file at this call and the returned edits carry it as
        a literal. Nothing re-reads later, so a resave between this
        call and the caller's `apply` leaves the applied pin naming
        the older version — the edits are a snapshot, not a
        subscription. Nor does applying them check that the pin
        resolves: a pin is recipe data, and whether it resolves is
        evaluation's question.

        What it does NOT read is `doc` from the store: the document
        passed here is the caller's in-memory value and the edits are
        computed against ITS sites, so a document the store has never
        seen is legal and normal.

        Pure — nothing is applied, nothing is written. Raises
        WorkspaceError, typed; the elaboration's own refusal arrives
        under `variant == "update"`."""

    def __len__(self) -> int: ...

def random_document_id() -> str:
    """Mint a fresh random document identity from OS randomness — the
    interactive-authoring constructor, and the door `Doc()` uses.
    Raises IdentityError if the entropy source refuses; identity is
    never defaulted."""

def content_pin(doc: Doc) -> ContentPin:
    """The document's content pin: which VERSION it is. Raises
    PersistError, typed."""

def canonical_bytes(doc: Doc) -> bytes:
    """The document's canonical semantic bytes — what the pin is the
    SHA-256 of. The same document authored two ways serialises to the
    same bytes. Runs the shared validator first. Raises PersistError,
    typed."""

def header_document_id(text: str) -> str:
    """Read a save file's identity out of its header alone, without
    parsing the body — the store's scan door. Raises PersistError,
    typed."""

PIN_MISMATCH_RECOURSE: Final[str]
"""The recourse sentence a `pin_mismatch` message ends on: pins never
move silently, and accepting a new version is a recorded edit."""

# --- selectors --------------------------------------------------------
# The narrowing language. Patterns and
# predicates are VALUES built here and evaluated in Rust by
# `Evaluation.select` / `select_where`; nothing on this side reads
# geometry or name text. The builder verbs return NEW values (Rust's
# consuming builders, crossed as immutable ones).

class EntityKind:
    """Which entity kind a name denotes."""

    Body: Final[EntityKind]
    Face: Final[EntityKind]
    Edge: Final[EntityKind]
    Vertex: Final[EntityKind]

class SegTag:
    """Which role-segment variant a `SegPat` names — one tag per
    op-minted role, grouped by the op that mints it."""

    OutputBody: Final[SegTag]
    Cap: Final[SegTag]
    Lateral: Final[SegTag]
    RimEdge: Final[SegTag]
    LateralEdge: Final[SegTag]
    CapVertex: Final[SegTag]
    Band: Final[SegTag]
    BandRim: Final[SegTag]
    BandRimPi: Final[SegTag]
    BandPi: Final[SegTag]
    Meridian: Final[SegTag]
    MeridianVertex: Final[SegTag]
    RevolveCap: Final[SegTag]
    Pole: Final[SegTag]
    AxisEdge: Final[SegTag]
    FromA: Final[SegTag]
    FromB: Final[SegTag]
    Seam: Final[SegTag]
    Merged: Final[SegTag]
    Fragment: Final[SegTag]
    SplitBody: Final[SegTag]
    SectionFace: Final[SegTag]
    SectionEdge: Final[SegTag]
    SplitFragment: Final[SegTag]
    CrossingVertex: Final[SegTag]
    OnToolVertex: Final[SegTag]
    FromTarget: Final[SegTag]
    BlendFace: Final[SegTag]
    CornerFace: Final[SegTag]
    TrimEdge: Final[SegTag]
    FootVertex: Final[SegTag]
    CornerArc: Final[SegTag]
    BandFace: Final[SegTag]
    BandTrim: Final[SegTag]
    BandFoot: Final[SegTag]
    BandCross: Final[SegTag]
    BandCut: Final[SegTag]
    BandSlit: Final[SegTag]
    Instance: Final[SegTag]
    InPart: Final[SegTag]

class OpGroup:
    """The op group a role segment belongs to (`SegPat.group`)."""

    Shared: Final[OpGroup]
    Extrude: Final[OpGroup]
    Revolve: Final[OpGroup]
    Boolean: Final[OpGroup]
    Split: Final[OpGroup]
    Fillet: Final[OpGroup]
    Pattern: Final[OpGroup]
    InstantiatePart: Final[OpGroup]

class CapEnd:
    """An extrude/revolve cap end (`SegPat.side`)."""

    Top: Final[CapEnd]
    Bottom: Final[CapEnd]

class MeridianEnd:
    """A revolve meridian end (`SegPat.side`)."""

    Start: Final[MeridianEnd]
    End: Final[MeridianEnd]
    Seam: Final[MeridianEnd]
    Pi: Final[MeridianEnd]

class SplitHalf:
    """A split output half (`SegPat.side`)."""

    Above: Final[SplitHalf]
    Below: Final[SplitHalf]

class RimSupport:
    """Which support of a rim blend (`SegPat.side`)."""

    Plane: Final[RimSupport]
    Curved: Final[RimSupport]

class CurveKind:
    """Which curve variant an edge's certified carrier is — the EXACT
    atom `GeomPred.curve_kind` matches on."""

    Line: Final[CurveKind]
    Circle: Final[CurveKind]
    Ellipse: Final[CurveKind]
    Nurbs: Final[CurveKind]

class SurfaceKind:
    """Which surface variant a face is (`GeomPred.surface_kind`,
    `GeomPred.adjacent_kinds`)."""

    Plane: Final[SurfaceKind]
    Cylinder: Final[SurfaceKind]
    Cone: Final[SurfaceKind]
    Sphere: Final[SurfaceKind]
    Torus: Final[SurfaceKind]
    Nurbs: Final[SurfaceKind]
    Approx: Final[SurfaceKind]

class Cmp:
    """The comparison `GeomPred.datum_distance` makes: the sign
    trilean, never a bare float equality. A candidate whose margin
    lands INSIDE the ε-band answers neither strict arm and refuses
    (`SelectRefusal`, `reason="in_band"`)."""

    Approx: Final[Cmp]
    Greater: Final[Cmp]
    Less: Final[Cmp]

class SegPat:
    """A pattern over ONE role segment: which variant, which side,
    what its sub-name arguments look like."""

    @staticmethod
    def any() -> SegPat: ...
    @staticmethod
    def tag(tag: SegTag) -> SegPat: ...
    @staticmethod
    def group(group: OpGroup) -> SegPat: ...
    def side(self, side: CapEnd | MeridianEnd | SplitHalf | RimSupport) -> SegPat: ...
    def of(self, args: list[NamePat]) -> SegPat:
        """Constrain the segment's sub-name arguments, positionally,
        as a PREFIX."""

class NamePat:
    """A pattern over a whole stable name: entity kind, minting node,
    and the exact shape of the role path."""

    @staticmethod
    def any() -> NamePat: ...
    @staticmethod
    def of_kind(kind: EntityKind) -> NamePat: ...
    def node(self, node: NodeId) -> NamePat: ...
    def path(self, path: list[SegPat]) -> NamePat: ...
    def seg(self, seg: SegPat) -> NamePat: ...
    def matches(self, name: str) -> bool:
        """Whether a materialized name matches. `name` is the opaque
        text a materializer answered with — the BINDING reads it (the
        one licensed reader), your code never does."""

class Selector:
    """A UNION of name patterns — the STRUCTURAL selector language,
    matched on name shape alone. Geometry is `select_where`'s second
    stage, never a pattern field."""

    @staticmethod
    def of(pat: NamePat) -> Selector: ...
    @staticmethod
    def any_of(pats: list[NamePat]) -> Selector:
        """The union of these patterns. An EMPTY selector matches
        nothing."""

    def or_(self, pat: NamePat) -> Selector:
        """The union with one more pattern — Rust's `.or`; the
        trailing underscore is the `inch` precedent (`or` is a Python
        keyword)."""

    def matches(self, name: str) -> bool: ...

class GeomPred:
    """One geometric atom; a `list[GeomPred]` is their CONJUNCTION.

    The atoms split EXACT/DECIDED and the split is typed, not
    flattened: the three kind atoms read the carrier's enum tag —
    total, cannot refuse — while `datum_distance` is a real length
    comparison through the margin funnel, whose in-band candidates
    refuse as `SelectRefusal` rather than being silently included or
    dropped."""

    @staticmethod
    def curve_kind(kinds: CurveKind | list[CurveKind]) -> GeomPred:
        """EXACT: the edge's certified carrier is one of these kinds.
        A list is a SET (one predicate says "a line or an arc"); an
        empty list matches nothing."""

    @staticmethod
    def surface_kind(kinds: SurfaceKind | list[SurfaceKind]) -> GeomPred:
        """EXACT: the face's surface is one of these kinds."""

    @staticmethod
    def adjacent_kinds(
        a: SurfaceKind | list[SurfaceKind],
        b: SurfaceKind | list[SurfaceKind],
    ) -> GeomPred:
        """EXACT: the two faces across an EDGE have kinds drawn one
        from each set — UNORDERED, so `(Plane, Sphere)` matches a rim
        whichever side carries which."""

    @staticmethod
    def datum_distance(datum: NodeId, cmp: Cmp, value: Length) -> GeomPred:
        """DECIDED: the entity's distance to a datum node against a
        stated `Length` — signed to a datum plane, unsigned to an axis
        or point. The datum is a node reference like every other
        input, which keeps the rule equivariant."""

# --- values -----------------------------------------------------------

class MassProperties:
    """Volume in m^3 and area in m^2 — dimensions outside D6's closed
    set, so they cross as canonical-unit floats."""

    @property
    def volume(self) -> float: ...
    @property
    def surface_area(self) -> float: ...
    @property
    def volume_pad(self) -> float: ...
    @property
    def area_pad(self) -> float: ...

class Body:
    """A solid body — an opaque handle with curated doors."""

    def mass_properties(self) -> MassProperties: ...
    def validate(self) -> None: ...
    def validate_closed(self) -> None: ...
    def validate_geometric(self) -> None: ...
    def tessellate(self, chordal: Length) -> Mesh:
        """Triangulate every face within `chordal` of the exact
        surface — the ladder's step 4.

        `chordal` is a DISTANCE (δ), and it is not the kernel's ε:
        δ says how coarsely a VIEW of the model may approximate it,
        ε says what the model IS. Two budgets see the same body, and
        no kernel state depends on δ.

        Nothing is pre-checked: a zero, negative or non-finite budget
        is the kernel's own `TessellateError`, raised here."""

class Mesh:
    """A tessellated body: one shared position buffer, and one
    triangle patch per face.

    Adjacent faces share position INDICES along their common
    boundary, so a closed body's mesh is watertight by construction —
    which is why a Python-side check of that contract compares
    indices and never coordinates.

    The picking chain does not cross. A patch's face, a boundary's
    edge and their vertex back-references are arena keys, so a patch
    is addressed by INDEX here and the per-edge boundary polylines
    are not bound at all."""

    @property
    def positions(self) -> list[tuple[Length, Length, Length]]:
        """The shared position buffer, in the kernel's minting order:
        topology vertices, then per-edge chord points, then per-face
        interior grid points."""

    @property
    def triangles(self) -> list[tuple[int, int, int]]:
        """Every patch's triangles, concatenated in the fixed export
        order — the same walk the STL writers make, so these and an
        exported file agree facet for facet.

        Indices point into `positions`, and the winding is OUTWARD
        (counterclockwise seen from outside the material). That is
        what makes a divergence-theorem volume over these triangles
        POSITIVE for a closed body, and it is already stated in the
        outward frame: do not re-apply a face sense on top of it."""

    @property
    def patch_count(self) -> int: ...
    @property
    def triangle_count(self) -> int: ...
    def patch(self, index: int) -> list[tuple[int, int, int]]:
        """One face's triangles. Raises `IndexError` past the end."""

    def to_stl_ascii(self, solid_name: str = "") -> str:
        """The ASCII STL text, `solid <name>` first line.

        The name is validated, not sanitized: a character outside the
        printable ASCII the single-line grammar admits raises
        `StlError`."""

    def to_stl_binary(self, header: str = "") -> bytes:
        """The binary STL bytes.

        `header` is the 80-byte header field's free text —
        conventionally the producer. A header that does not fit, or
        that would make the file sniff as ASCII STL, raises
        `StlError` rather than being truncated or written."""

class Datum:
    @property
    def kind(self) -> str: ...
    @property
    def origin(self) -> tuple[Length, Length, Length]: ...
    @property
    def direction(self) -> Optional[tuple[float, float, float]]: ...

# --- detect / declare -------------------------------------------------
# The flush-contact protocol's value vocabulary. A finding is a
# REPORT: `Evaluation.find_flush_candidates` answers with them, the
# caller inspects, and `Node.declare` / `Doc.declare` /
# `Doc.declare_all` turn inspected findings into the `Declare` node
# `Node.boolean`'s `declare=` consumes. The same value rides the
# boolean's refusal menu (`EvaluationError.finding`). Detection and
# declaration are separate doors ON PURPOSE: no fused
# detect-and-declare door exists.

class PlaneRelation:
    """The verify door's relation verdict: `SameOpposite` = resting
    contact (opposed outward normals), `SameOriented` = flush walls
    (the merge-stage flavor). `Distinct` exists as vocabulary; a
    finding never carries it."""

    SameOriented: Final[PlaneRelation]
    SameOpposite: Final[PlaneRelation]
    Distinct: Final[PlaneRelation]

class ContactClass:
    """The contact class a declaration asserts. `Rest` (coincident
    planes) is the only class the flush DETECTOR mints, so it is the
    only one a `FlushFinding` from `find_flush_candidates` carries;
    `Tangent` crossed the mirror with M9-1 and is nameable here
    because a class the binding cannot name would refuse typed at the
    crossing instead."""

    Rest: Final[ContactClass]
    Tangent: Final[ContactClass]

class FlushRung:
    """Which rung of the verify ladder decided a finding:
    `SharedSource` = syntactic recipe identity (zero numerics),
    `DecidedCoincident` = the geometric trilean's coincident arm."""

    SharedSource: Final[FlushRung]
    DecidedCoincident: Final[FlushRung]

class FlushFinding:
    """One flush-plane finding: "this face pair would verify as
    declared contact" — a VALUE to inspect and declare, never itself
    a declaration. `a`/`b` are the pair's names in the same OPAQUE
    text alphabet every materializer speaks (store them, hand them
    back; never parse). `class_` spells `class` (a Python keyword)
    with the `or_` trailing-underscore precedent."""

    @property
    def a(self) -> str: ...
    @property
    def b(self) -> str: ...
    @property
    def relation(self) -> PlaneRelation: ...
    @property
    def class_(self) -> ContactClass: ...
    @property
    def rung(self) -> FlushRung: ...
    def __eq__(self, other: object) -> bool: ...

class Value:
    """A node's successful value."""

    @property
    def kind(self) -> str: ...
    def body(self) -> Body: ...
    def bodies(self) -> list[Body]: ...
    def split(self) -> tuple[Optional[Body], Optional[Body]]: ...
    def datum(self) -> Datum: ...

class Evaluation:
    """The per-node result DAG."""

    def value(self, node: NodeId) -> Value: ...
    def succeeded(self, node: NodeId) -> bool: ...
    def order(self) -> list[NodeId]: ...
    def all_edges(self, node: NodeId) -> list[str]:
        """Every edge name of `node`'s output, as of THIS evaluation —
        the strings `Node.fillet` selects with. A MATERIALIZER: store
        what it answers, because a recipe holds no live selection.

        Each string is an OPAQUE identifier. Its internal structure is
        not API — it may change without notice — so the supported
        operations are equality, ordering, storage, and handing it back
        to `Node.fillet`. Narrowing the set is a SELECTOR's job:
        `select` and `select_where`, which answer in this same
        alphabet.
        """

    def all_faces(self, node: NodeId) -> list[str]: ...
    def all_vertices(self, node: NodeId) -> list[str]: ...
    def all_bodies(self, node: NodeId) -> list[str]: ...
    def select(self, node: NodeId, selector: Selector) -> list[str]:
        """Every name of `node`'s output matching `selector`'s
        role-path shape, as of THIS evaluation — a materializer with
        `all_edges`' exact contract (canonical order, the caller
        stores it, frozen thereafter), answering in the same opaque
        alphabet `Node.fillet` reads. Infallible: empty when nothing
        matches."""

    def select_where(
        self,
        node: NodeId,
        selector: Selector,
        geom: list[GeomPred],
    ) -> list[str]:
        """`select`, then filter the survivors by GEOMETRY: each is
        resolved to its entity in THIS evaluation and tested against
        the CONJUNCTION `geom`. An empty `geom` is exactly `select`;
        run twice and concatenate for a union.

        Raises `SelectRefusal`, typed, where the Rust door refuses:
        exact atoms are total, but a decided atom's in-band margin, a
        tied name whose candidates disagree, or an unreadable
        candidate refuse rather than silently including or dropping
        anything."""
    def find_flush_candidates(self, a: NodeId, b: NodeId) -> list[FlushFinding]:
        """The cross-body flush-plane candidates between `a`'s and
        `b`'s outputs, as of THIS evaluation — the detect arm of the
        detect/declare protocol, run by the C4 verifier itself (a
        finding cannot disagree with the boolean's verify-at-use).
        Findings are DEFINITE and canonically ordered; empty when
        either node has no value. Raises `SelectRefusal`, typed
        (`pair_in_band`, `tied_disagrees`, `unreadable`, `band`) —
        an ambiguous pair is never silently included or dropped."""
    @property
    def recomputed(self) -> int:
        """How many nodes ran their op. With no `prior=` that is every
        node that ran; with one it is the changed cone.

        `recomputed + reused` is the nodes that RAN OR WERE REUSED —
        the live node count only when nothing refused. A POISONED node
        (one that never ran because an ancestor failed) is counted by
        neither, so on a refusal path the sum undershoots
        `len(order())` by exactly the number of poisonings. A node that
        ran and FAILED counts here: it ran."""
    @property
    def reused(self) -> int:
        """How many nodes came from `evaluate`'s `prior=` memo without
        re-running their op. Zero when no prior was passed.

        A reused `InstantiatePart` node did not ask the resolver — see
        `evaluate`'s `prior=` on what that means for the seam's
        availability refusals."""
    @property
    def part_evaluations(self) -> int:
        """How many REFERENCED documents this run crossed the seam to
        evaluate — `resolver=`'s sharing evidence. N instances of one
        part count 1; a part that instantiates a part counts here too.
        Zero without a resolver: nothing crosses. Zero, too, when every
        instance was a memo hit: a reused node never asks."""
    def step_string(
        self,
        node: NodeId,
        product_name: Optional[str] = None,
    ) -> str: ...

def evaluate(
    doc: Doc,
    *,
    resolver: Optional[Workspace] = None,
    prior: Optional[Evaluation] = None,
) -> Evaluation:
    """Evaluate a document. Total — never raises.

    `resolver` is the DOCUMENT SEAM: what an `InstantiatePart` node
    reaches the document it pins through. A `Workspace` IS a resolver,
    so the store is passed as itself. `None` — the default — is a
    kernel-only evaluation, in which every instantiate node refuses
    typed (`EvaluationError`, `kind == "part_no_resolver"`) rather
    than pretending a part is empty.

    `prior` is the MEMO: a node whose content and naming keys match
    its result in `prior` reuses that value instead of re-running its
    op, so only the changed cone costs anything. `Evaluation.reused`
    and `Evaluation.recomputed` count it.

    The memo is PER DOCUMENT and node-id-keyed: the lookup finds
    `prior`'s result for the SAME node id and then certifies it by
    content. An evaluation of a different document is a legal prior
    that reuses nothing — ids are minted per document, and two
    assemblies over the same parts at the same pins share none. Pass
    the prior evaluation of THIS document.

    A MEMO HIT IS SERVED WITHOUT RE-RUNNING THE SEAM'S GATES. A reused
    `InstantiatePart` node never asks the resolver, so the availability
    refusals — `part_pin_mismatch`, `part_unresolved`,
    `part_no_resolver` — are raised only for nodes that actually
    re-resolve. What is served is what the document's own `DocRef`
    PINS, certified by content key: never a different part, and not
    re-checked against the store. So editing a part on disk and
    re-evaluating WITH a prior succeeds, serving the previously pinned
    body, where the same call without the prior refuses
    `part_pin_mismatch` — stale relative to the store, pinned relative
    to the document. "A pin that moved refuses, and is never silently
    retargeted" holds for evaluations that cross the seam; a run that
    never asks does not re-assert it. Pass no prior when the question
    is whether the document still resolves against the store as it
    stands."""

# --- the assembly vocabulary ------------------------------------------
# Two part documents in a store, instances of them in a third, mates
# saying how the instances meet, and one gate that says whether the
# result is valid at rest. Authoring is `Node.instantiate_part` +
# `Node.mate` + `DocEdit.set_placement`; reading is `solve_document`,
# `product` and `assemble`; refactoring is `split` / `inline`.
#
# Placement lives on the CLUSTER, never on the instance: mated
# instances share one recorded frame — the earliest of them in
# document order, their GAUGE — and every other member's world pose is
# SOLVED from the mates and composed outward. That is why
# `Doc.placement` and `SolvedPoses.placement` are two different
# questions, and why a document can carry three instances and one
# frame.

class MateFrame:
    """One side's mate frame, in that instance's own part coordinates.

    AUTHORED data, not geometry read back: the solve is structural
    plus decided predicates over exactly these numbers, so a frame
    that does not match the face its mate names is a disagreement
    nothing here can see.

    `axis` need not be unit and `reference` need not be perpendicular
    to it — only the axis's direction and the reference's
    perpendicular part are read. Both are plain numbers (a direction
    carries no dimension); `origin` is three lengths."""

    def __init__(
        self,
        origin: tuple[Length, Length, Length],
        axis: tuple[float, float, float],
        reference: tuple[float, float, float],
    ) -> None: ...
    @property
    def origin(self) -> tuple[Length, Length, Length]: ...
    @property
    def axis(self) -> tuple[float, float, float]: ...
    @property
    def reference(self) -> tuple[float, float, float]: ...
    def placement(self) -> Frame:
        """The rigid placement this frame denotes: local +Z is `axis`,
        roll fixed by `reference`. Raises FrameError when the axis has
        no definite direction or the reference no definite
        perpendicular — the refusal the solve would meet, reachable
        BEFORE authoring the mate that carries it."""

class AxisSense:
    """Which way the two sides' axes point at each other. `Opposed` is
    what kills every pi-flip ambiguity: the senses are AUTHORED, never
    inferred."""

    Aligned: Final[AxisSense]
    Opposed: Final[AxisSense]

class MateSide:
    """Which side of a mate a diagnostic is about."""

    A: Final[MateSide]
    B: Final[MateSide]

class MatePrimitive:
    """Which coset of rigid motions a mate pins the pair's relative
    pose to.

    Four constructors and no bare enum, because one carries a length.
    `clocking` is representable precisely so it can be REFUSED — the
    coset table has no entry for a bare angular relation, and an
    unrepresentable refusal is an untestable one."""

    @staticmethod
    def frame_coincidence() -> MatePrimitive:
        """The two mate frames coincide outright — residual trivial."""

    @staticmethod
    def coaxial() -> MatePrimitive:
        """The two axes coincide as a LINE — residual cylindrical."""

    @staticmethod
    def planar_rest(offset: Length) -> MatePrimitive:
        """`b`'s plane rests on `a`'s, displaced by `offset` along
        `a`'s axis — residual planar. Zero is the flush rest; nonzero
        is an authored standoff, never a designed clearance."""

    @staticmethod
    def clocking() -> MatePrimitive:
        """Clocking with no carrying primitive: refused at the
        solve."""
    @property
    def variant(self) -> str: ...
    @property
    def offset(self) -> Optional[Length]:
        """The planar rest's standoff, `None` for the others."""

class Alignment:
    """The alignment datum: which frames coincide, at which axis
    sense, with which clocking rider.

    `clocking` is a RIDER, never a primitive: on `coaxial` it cuts the
    residual to prismatic; on `frame_coincidence` it is
    redundant-or-contradictory and gets decided; on a planar rest the
    table has no entry and the solve refuses typed."""

    def __init__(
        self,
        a: MateFrame,
        b: MateFrame,
        primitive: MatePrimitive,
        sense: AxisSense,
        clocking: Optional[Angle] = None,
    ) -> None: ...
    @property
    def a(self) -> MateFrame: ...
    @property
    def b(self) -> MateFrame: ...
    @property
    def primitive(self) -> MatePrimitive: ...
    @property
    def sense(self) -> AxisSense: ...
    @property
    def clocking(self) -> Optional[Angle]: ...
    @property
    def lever_arm(self) -> Length:
        """The largest distance in this mate's own authored data over
        which an angular error accumulates into a gap. Floored at one
        metre, so a mate authored AT the origin cannot claim an
        arbitrarily tight angular threshold."""

class ClassAdmission:
    """How far a contact class gets in v1, as a value BOTH enforcing
    doors read.

    The solve needs a coset the alignment table can fold; the gate's
    mint needs a kernel record type that can carry the declaration at
    rest. A class can satisfy the first and not the second — which is
    why a tool asks this table before committing an edit rather than
    discovering the refusal after it lands."""

    @property
    def variant(self) -> str:
        """`mints`, `no_at_rest_record`, or `not_admitted`."""

    @property
    def mints(self) -> bool:
        """Whether both doors admit it."""

    @property
    def solves(self) -> bool:
        """Whether the SOLVE admits it. A class the solve refuses
        never reaches the gate at all."""

    @property
    def why(self) -> Optional[str]:
        """Why the gate carries nothing at rest for this class, in the
        class's own terms; `None` for `mints`, which carries one."""

def class_admission(class_: ContactClass) -> ClassAdmission:
    """How far `class_` gets in v1 — the table, read. Nothing is
    restated here: `assemble` and `solve_document` read this same
    table."""

CLASS_DEFERRAL: Final[str]
"""The v1 class restriction, named: what a class outside the admitted
vocabulary refuses with."""

UNDER_RECOURSE: Final[str]
"""The recourse an under-determined tree mate's refusal ends on."""

class MateRole:
    """What a mate did in the solve: `Determining` (a tree mate — it
    placed its child), `Declaring` (it solved nothing and is carried
    to evaluation as a pure contact declaration), `Refused`."""

    Determining: Final[MateRole]
    Declaring: Final[MateRole]
    Refused: Final[MateRole]

class Subgroup:
    """A residual subgroup: what a fold left free.

    `normal`, `point` and `direction` are present on every arm and
    `None` where that arm does not carry one. `planar` and `prismatic`
    are point-FREE on purpose: rotations about any parallel axis, and
    translations along any parallel line, are in the group."""

    @property
    def variant(self) -> str:
        """`se3`, `planar`, `cylindrical`, `prismatic`, `revolute`,
        `trivial`, or `empty`."""

    @property
    def normal(self) -> Optional[tuple[float, float, float]]: ...
    @property
    def point(self) -> Optional[tuple[Length, Length, Length]]: ...
    @property
    def direction(self) -> Optional[tuple[float, float, float]]: ...

class MateFault:
    """Why the solve refused for one node — a VALUE, because the solve
    is total and records a refusal per node rather than failing the
    document. `str(fault)` is the kernel's own prose."""

    @property
    def variant(self) -> str: ...
    @property
    def mate(self) -> Optional[NodeId]: ...
    @property
    def side(self) -> Optional[MateSide]: ...
    @property
    def head(self) -> Optional[NodeId]: ...
    @property
    def instance(self) -> Optional[NodeId]: ...
    @property
    def parent(self) -> Optional[NodeId]: ...
    @property
    def child(self) -> Optional[NodeId]: ...
    @property
    def residual(self) -> Optional[Subgroup]: ...
    @property
    def held(self) -> Optional[NodeId]: ...
    @property
    def added(self) -> Optional[NodeId]: ...
    @property
    def predicate(self) -> Optional[str]: ...
    @property
    def clash(self) -> Optional[Length]: ...
    @property
    def what(self) -> Optional[str]: ...

class SolvedPoses:
    """The document's solved poses: each instance's pose relative to
    its cluster gauge, each mate's role, and the per-node refusals."""

    def fault(self, node: NodeId) -> Optional[MateFault]:
        """The node's recorded fault. Recorded against the refusing
        MATE and against every instance in its cluster that
        consequently has no pose — and no further."""

    def role(self, mate: NodeId) -> Optional[MateRole]: ...
    def gauge(self, instance: NodeId) -> Optional[NodeId]:
        """The instance's cluster gauge. A singleton is its own."""

    def relative(self, instance: NodeId) -> Optional[Frame]:
        """Its pose relative to that gauge. The gauge's own entry is
        the identity, bit-exactly."""

    def placement(self, doc: Doc, instance: NodeId) -> Frame:
        """The instance's WORLD placement: the cluster's recorded
        frame composed onto the solved relative pose. A singleton
        returns its recorded frame verbatim.

        `doc` must be the document this solve is OF — passing another
        composes this document's relative poses onto that one's
        cluster frames, which is a pose of neither, and nothing here
        can check it. Raises MateError when the cluster did not
        solve."""

def solve_document(doc: Doc) -> SolvedPoses:
    """Solve the document's mates: the per-pair coset fold along a
    deterministic spanning tree.

    TOTAL — this never raises. A refusing cluster must not fail an
    unrelated one, so refusals are read back through
    `SolvedPoses.fault`.

    Nothing here inspects geometry. In particular it does NOT check
    that a mate's frames match the faces its references name, which is
    why a document can solve cleanly and still refuse at the gate."""

def clusters(doc: Doc) -> list[list[NodeId]]:
    """The placement clusters: instances coupled by mates, members in
    document order. The partition placement is keyed by."""

def gauge_of(doc: Doc, instance: NodeId) -> NodeId:
    """An instance's cluster GAUGE — the document-order-first instance
    of its cluster, whose recorded frame places the whole cluster.
    Answers the node itself when it is in no cluster."""

def reading_edges(doc: Doc) -> list[tuple[NodeId, NodeId]]:
    """For each mate, the instantiate node each of its references
    resolves through. Recomputed every time, never stored."""

def relative_freedom_components(doc: Doc) -> list[list[NodeId]]:
    """The relative-freedom partition: components over consuming
    union reading edges, so mates couple what they constrain. Coarser
    than `clusters`, which partitions instances alone."""

class ClusterMaintenance:
    """One recorded act of cluster-record maintenance: what an
    ordinary edit's motion of the mate graph forced on the placement
    registry.

    It rides the accepted edit rather than being an edit of its own —
    deterministic from the edit, so a replay reproduces it and undo
    restores it exactly. What the record adds is VISIBILITY: an
    absorbed cluster's frame is consumed here.

    `source` and `target` rather than `from`/`to`: `from` is a Python
    keyword."""

    @property
    def variant(self) -> str:
        """`join`, `split`, `gauge_rewrite`, or `drop`."""

    @property
    def survived(self) -> Optional[NodeId]: ...
    @property
    def absorbed(self) -> Optional[NodeId]: ...
    @property
    def absorbed_frame(self) -> Optional[Frame]: ...
    @property
    def source(self) -> Optional[NodeId]: ...
    @property
    def target(self) -> Optional[NodeId]: ...
    @property
    def frame(self) -> Optional[Frame]: ...
    @property
    def gauge(self) -> Optional[NodeId]: ...

# --- the gather and the at-rest gate ----------------------------------

def product(doc: Doc, evaluation: Evaluation) -> Body:
    """The document's PRODUCT: every body-denoting root's solids,
    gathered in root-list order into one body.

    What a document IS — and for an assembly the only useful reading,
    because an assembly's nodes are instances and mates and no single
    node's value is the assembly. A pure function of the root list and
    the evaluation.

    `evaluation` must be an evaluation OF `doc`: node ids are minted
    per document, so a foreign one refuses `unknown_node` rather than
    answering about the wrong document. Raises ProductError, typed."""

def product_named(doc: Doc, evaluation: Evaluation) -> tuple[Body, list[str]]:
    """The product with the stable names its entities answer to —
    same gather, one more field. The names are the product's own
    alphabet, so "the third post's top cap" is one name rather than a
    coordinate. Raises ProductError, typed."""

class RefusedRef:
    """Why a mate reference named no product face."""

    @property
    def variant(self) -> str:
        """`ref_node_gone`, `ref_vanished`, `ref_ambiguous`, or
        `ref_not_a_face`."""

    @property
    def width(self) -> Optional[int]:
        """How many entities a tie holds. A mate declaration must name
        ONE face, and a tie is never broken by picking."""

    @property
    def kind(self) -> Optional[str]:
        """What a non-face reference did name."""

class MintedDeclaration:
    """One declaration the gate minted from a solved mate.

    The face keys the kernel matched do NOT cross — arena keys never
    leave the document layer — so the pair is identified by the two
    stable names, the alphabet the mate was authored in."""

    @property
    def mate(self) -> NodeId: ...
    @property
    def a(self) -> str: ...
    @property
    def b(self) -> str: ...
    @property
    def class_(self) -> ContactClass: ...

class Attribution:
    """What a kernel finding says about the document's declarations.

    One value rather than a declaration plus a flag: the relation and
    the declaration it names are decided together and cannot
    disagree."""

    @property
    def relation(self) -> str:
        """`refuted` (the faces do not meet as declared — a finding
        against the document), `declined` (the census has no certifier
        lane for a face the declaration names, so nothing was decided
        either way), or `unattributed` (no declaration answers — an
        UNDECLARED contact, the hard error by definition)."""

    @property
    def declaration(self) -> Optional[MintedDeclaration]: ...

class AtRestFinding:
    """One at-rest refusal. `str(finding)` composes it the way the
    library renders one: the mate a user can act on, then the kernel's
    own story, which carries its own recourse."""

    @property
    def attribution(self) -> Attribution: ...

class Assembly:
    """A validated assembly: the gathered body, its product names, and
    one minted declaration per solved mate.

    Reaching one means the kernel's at-rest door PASSED over the
    product and its records together."""

    @property
    def body(self) -> Body: ...
    @property
    def names(self) -> list[str]: ...
    @property
    def minted(self) -> list[MintedDeclaration]:
        """Empty for a mate-less assembly, which is what a disjoint
        layout is."""

def assemble(doc: Doc, evaluation: Evaluation) -> Assembly:
    """The AT-REST ASSEMBLY GATE: gather the product, mint every
    solved mate's declaration into its contact records, and run the
    kernel's own at-rest door over the two together.

    The check the authoring vocabulary can otherwise construct and
    never make. `evaluation` must be an evaluation of `doc` that
    RESOLVED — an instantiate node with no resolver produced no body,
    so the gather refuses `root_failed` before the gate runs.

    Raises AssemblyError, typed. Read `variant` first: `at_rest` is a
    verdict AGAINST the document, `uncertified` is the declared
    direction's FRONTIER where nothing was decided either way, and the
    remaining arms (`mate_reference_refused`, `no_at_rest_record`, the
    gather's own tags) refuse before any verdict."""

# --- the recorded refactorings ----------------------------------------
# Both are PURE: they hand back the new document VALUES plus the
# ordinary recorded edits that produce them, and mutate nothing. That
# is what makes them atomic at the caller's single step. Persisting a
# result is the store's write side.

class InterfaceCrossing:
    """One declaration that crossed a split's cut: a mate whose two
    ends landed on opposite sides.

    `outer` stayed in the remainder; `inner` moved into the part and
    is spelled in the PART's own names, unwrapped, because that is
    what the part's product answers to."""

    @property
    def variant(self) -> str:
        """`mate` — a crossing is whatever KIND of edge crossed, and
        mates are the only kind that can."""

    @property
    def mate(self) -> NodeId: ...
    @property
    def class_(self) -> ContactClass: ...
    @property
    def outer(self) -> str: ...
    @property
    def inner(self) -> str: ...

class InterfaceRecord:
    """The interface record of an instantiate seam: the declarations
    that crossed the cut when the referenced document was split out.
    Empty for a directly-authored instance."""

    @property
    def crossings(self) -> list[InterfaceCrossing]: ...
    def __len__(self) -> int: ...

class SplitOutcome:
    """What a split produced: the two document VALUES and the recorded
    edits that make each. Nothing is persisted and nothing is
    mutated."""

    @property
    def remainder(self) -> Doc:
        """The original with the cut nodes replaced by ONE instance of
        the new part."""

    @property
    def part(self) -> Doc:
        """The new part document, carrying the cut nodes."""

    @property
    def remainder_edits(self) -> list[DocEdit]: ...
    @property
    def part_edits(self) -> list[DocEdit]: ...
    @property
    def instance(self) -> NodeId:
        """The instantiate node left behind in the remainder."""

    @property
    def node_map(self) -> list[tuple[NodeId, NodeId]]:
        """Cut node -> its id in the part document."""

def split(doc: Doc, cut: list[NodeId], part_id: str) -> SplitOutcome:
    """Cut a closed node set out into a NEW document, leaving one
    instance of it behind.

    `part_id` is the new document's identity, supplied by the caller
    (`random_document_id()` for interactive authoring): identity is
    never defaulted, and a fresh one is what lets both documents live
    in one store.

    The cut must be ancestor- and consumer-closed and a union of WHOLE
    placement clusters. Pure — `doc` is untouched. Raises SplitError,
    typed, naming the offending edge, cluster, parameter or name."""

class InlineOutcome:
    """What an inline produced: the spliced document value and the
    recorded edits that make it."""

    @property
    def doc(self) -> Doc: ...
    @property
    def edits(self) -> list[DocEdit]: ...
    @property
    def node_map(self) -> list[tuple[NodeId, NodeId]]:
        """Part node -> its id in the spliced document."""

def inline(doc: Doc, instance: NodeId, resolver: Workspace) -> InlineOutcome:
    """Splice a referenced document back in, replacing the instantiate
    node with the part's own nodes — `split`'s inverse.

    `resolver` crosses the document seam AT THIS CALL, under the full
    pin gate: a reference whose pinned version is not what the store
    holds refuses `part_pin_mismatch`, never silently splices the
    version on disk. Pure — `doc` is untouched. Raises InlineError,
    typed."""

# --- the pin-update door ----------------------------------------------

class PinSites:
    """One pin of one id, and the instantiate nodes that name it."""

    @property
    def pin(self) -> ContentPin: ...
    @property
    def nodes(self) -> list[NodeId]:
        """The referencing nodes, in document order. Non-empty by
        construction."""

class PinMultiplicity:
    """One referenced document id carrying more than one pin, with the
    sites holding each."""

    @property
    def id(self) -> str: ...
    @property
    def pins(self) -> list[PinSites]:
        """Its pins, ascending. At least two by construction — a
        single-pin id is not a multiplicity and is not reported."""

def mixed_pins(doc: Doc) -> list[PinMultiplicity]:
    """The mixed-pin LINT: every referenced id whose pin multiplicity
    exceeds one.

    REPORTS, never gates. Nothing calls this from `apply`, `load` or
    evaluation — a document in mixed-pin state is valid at all three
    and stays that way, because a staged migration IS this state. A
    clean document returns an empty list, which is the whole
    difference between "checked and fine" and "not checked".

    Reads the DOCUMENT and no store. Deterministic: ids ascending,
    pins ascending within an id, nodes in document order."""

def update_references(doc: Doc, id: str, new_pin: ContentPin) -> list[DocEdit]:
    """The edits that move EVERY reference to `id` onto `new_pin`, in
    document order, one per site whose pin actually moves.

    PURE, AND IT READS NO STORE. `doc` is untouched, nothing is
    applied, and `new_pin` is taken as given — whether it names
    content anything holds is not asked here and cannot be, because
    this layer has no store. Where that pin came from and how stale it
    is stays the caller's fact; `Workspace.update_to_store` is the
    door that computes one from disk, and it says exactly when it
    reads.

    The caller applies the whole list or none of it, and that
    all-or-nothing is what atomic means here. A site already pinning
    `new_pin` contributes NO edit, so "update everywhere" stays usable
    from the staged state where some sites already moved.

    Raises UpdateError, typed: `no_such_reference` when no live node
    instantiates the id, `already_pinned` when every site already
    names it. Separate arms because the recourses differ."""

def import_step(text: str) -> Body:
    """Parse a STEP text with the kernel's importer and adopt its
    solid — the round-trip oracle. Raises StepImportError, typed."""

__build_info__: Final[dict[str, Any]]
