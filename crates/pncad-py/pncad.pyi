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

Deliberately ABSENT, and tracked as named gaps in
`docs/guide/north-star-audit.md`: sweep and tube, tessellation and
STL, the pattern node with its structural-parameter edit, and the
detect/declare protocol that would build the `Node.declare`
`Node.boolean`'s `declare=` consumes.
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
    """The contact class a declaration asserts. v1 carries `Rest`
    (coincident planes) — the only demand-evidenced detector."""

    Rest: Final[ContactClass]

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
    def recomputed(self) -> int: ...
    @property
    def reused(self) -> int: ...
    def step_string(
        self,
        node: NodeId,
        product_name: Optional[str] = None,
    ) -> str: ...

def evaluate(doc: Doc) -> Evaluation:
    """Evaluate a document. Total — never raises."""

def import_step(text: str) -> Body:
    """Parse a STEP text with the kernel's importer and adopt its
    solid — the round-trip oracle. Raises StepImportError, typed."""

__build_info__: Final[dict[str, Any]]
