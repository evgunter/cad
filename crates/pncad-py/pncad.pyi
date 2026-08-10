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
`Start`, `circle`, `circle_split`) plus the straight-segment shortcut
`Node.polygon`; one loop, on a plane parallel to the world xy-plane.

Deliberately ABSENT, and tracked as named gaps in
`docs/guide/north-star-audit.md`: multi-loop profiles (holes), non-xy
sketch planes, loft/sweep/tube, tessellation and STL, selectors and
stable names, and contact declarations. Named document parameters
crossed in R1-PARAMS (`ParamName`, `DocParam`,
`DocEdit.set_doc_param` — guide §3.2).
"""

from typing import Any, Final, Optional, overload

# --- errors -----------------------------------------------------------
# Every subclass carries its refusal as ATTRIBUTES, never as parsed
# prose (§L4).

class PncadError(Exception):
    """Base class for every refusal this module raises."""

class EditError(PncadError):
    """The document layer refused an edit."""

    variant: str

class EvaluationError(PncadError):
    """A node produced no value, or produced the wrong kind.

    `reason` is `unknown_node`, `wrong_kind`, `empty_boolean`,
    `node_failed`, or `poisoned`. `kind` (the `NodeErrorKind`'s
    stable tag) and `through` (the nearest failed ancestor) are
    always present, `None` where the reason has neither (LIB-DOORS
    F3; attributes never go missing).
    """

    reason: str
    node: NodeId
    kind: Optional[str]
    through: Optional[NodeId]

class ValidationError(PncadError):
    """A body failed a validator, or mass properties could not be taken."""

    door: str
    failure_count: int

class DimensionError(PncadError):
    """A dimension mismatch at the quantity boundary."""

    op: str
    left: str
    right: str

class LiteralError(PncadError):
    """A value the expression layer refused (`Expr::literal`'s own
    curated error, LIB-DOORS F5). `value` is the offending number."""

    kind: str
    value: float

class PersistError(PncadError):
    """A save or load the persistence doors refused (LIB-DOORS F1)."""

    variant: str

class ExportError(PncadError):
    """The document-layer export door refused (LIB-DOORS F2).
    `through` (poisoning ancestor) and `kind` (the wrong-kind value's
    tag) are always present, `None` where inapplicable."""

    variant: str
    node: NodeId
    through: Optional[NodeId]
    kind: Optional[str]

class StepImportError(PncadError):
    """A STEP text the importer refused (`refused`), or one that
    parsed to a non-solid (`wireframe`)."""

    variant: str

class PathError(PncadError):
    """The PATHS authoring algebra refused the geometry, at the call
    site of the verb that wrote it."""

    variant: str

# --- quantities -------------------------------------------------------
# Canonical metres and radians underneath (GQ5). The arithmetic is
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

class Open:
    """The entry: nothing bound. A token, not a value you construct."""

    @staticmethod
    def at(p: tuple[Length, Length]) -> PathPoint: ...
    @staticmethod
    def angle(theta: Angle) -> PathAngle: ...
    @staticmethod
    def toward(dx: float, dy: float) -> PathAngle: ...
    @staticmethod
    def at_on(
        p: tuple[Length, Length],
        centre: tuple[Length, Length],
        winding: ArcSweep,
    ) -> PathDirected: ...

class PathOpen:
    """A fillet's freshly opened arrival side: nothing bound, and
    `Start` reachable because the entry is behind us."""

    def at(self, p: tuple[Length, Length]) -> PathPoint: ...
    def angle(self, theta: Angle) -> PathAngle: ...
    def toward(self, dx: float, dy: float) -> PathAngle: ...
    def to(self, target: StartToken) -> ClosedLoop: ...
    def at_on(
        self,
        p: tuple[Length, Length],
        centre: tuple[Length, Length],
        winding: ArcSweep,
    ) -> PathDirected: ...
    def to_on(
        self,
        target: StartToken,
        centre: tuple[Length, Length],
        winding: ArcSweep,
    ) -> ClosedLoop: ...

class PathAngle:
    """Direction bound, position pending."""

    def at(self, p: tuple[Length, Length]) -> PathDirected: ...
    def to(self, anchor: tuple[Length, Length]) -> PathDirectedPoint: ...

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
    def arc_to(
        self, target: tuple[Length, Length], bulge: float
    ) -> PathDirectedPoint: ...
    @overload
    def arc_to(self, target: StartToken, bulge: float) -> ClosedLoop: ...
    @overload
    def arc_via(
        self, via: tuple[Length, Length], target: tuple[Length, Length]
    ) -> PathDirectedPoint: ...
    @overload
    def arc_via(
        self, via: tuple[Length, Length], target: StartToken
    ) -> ClosedLoop: ...
    @overload
    def arc_center(
        self,
        centre: tuple[Length, Length],
        target: tuple[Length, Length],
        winding: ArcSweep,
    ) -> PathDirectedPoint: ...
    @overload
    def arc_center(
        self,
        centre: tuple[Length, Length],
        target: StartToken,
        winding: ArcSweep,
    ) -> ClosedLoop: ...

class PathDirectedPoint:
    """A leg end: position bound, and the leg's incoming end tangent
    available as read-only intrinsic data."""

    def angle(self, theta: Angle) -> PathDirected: ...
    def toward(self, dx: float, dy: float) -> PathDirected: ...
    def tangent(self) -> PathDirected: ...
    def turn(self, delta: Angle) -> PathDirected: ...
    def arc_continue(self, target: tuple[Length, Length]) -> PathDirectedPoint: ...
    @overload
    def line_to(self, target: tuple[Length, Length]) -> PathDirectedPoint: ...
    @overload
    def line_to(self, target: StartToken) -> ClosedLoop: ...
    @overload
    def arc_to(
        self, target: tuple[Length, Length], bulge: float
    ) -> PathDirectedPoint: ...
    @overload
    def arc_to(self, target: StartToken, bulge: float) -> ClosedLoop: ...
    @overload
    def arc_via(
        self, via: tuple[Length, Length], target: tuple[Length, Length]
    ) -> PathDirectedPoint: ...
    @overload
    def arc_via(
        self, via: tuple[Length, Length], target: StartToken
    ) -> ClosedLoop: ...
    @overload
    def arc_center(
        self,
        centre: tuple[Length, Length],
        target: tuple[Length, Length],
        winding: ArcSweep,
    ) -> PathDirectedPoint: ...
    @overload
    def arc_center(
        self,
        centre: tuple[Length, Length],
        target: StartToken,
        winding: ArcSweep,
    ) -> ClosedLoop: ...

class PathDirected:
    """Both bits bound — the only state legs and `fillet` consume.
    The outgoing angle slot is full, so no second director exists."""

    def line(self, len: Length) -> PathDirectedPoint: ...
    def fillet(self, radius: Length) -> PathOpen: ...
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
    """A recipe node's identity. NOT an arena key (§L3)."""

    def __hash__(self) -> int: ...

class BooleanOp:
    """The DOCUMENT-layer Boolean operator."""

    Union: Final[BooleanOp]
    Intersect: Final[BooleanOp]
    Subtract: Final[BooleanOp]

class Node:
    """A recipe node, before insertion."""

    @staticmethod
    def polygon(
        points: list[tuple[Length, Length]],
        elevation: Optional[Length] = None,
    ) -> Node: ...
    @staticmethod
    def profile(outline: ClosedLoop, elevation: Optional[Length] = None) -> Node: ...
    @staticmethod
    def extrude(profile: NodeId, distance: Length) -> Node: ...
    @staticmethod
    def revolve(profile: NodeId, axis: NodeId, angle: Angle) -> Node: ...
    @staticmethod
    def datum_axis(
        origin: tuple[Length, Length, Length],
        direction: tuple[float, float, float],
    ) -> Node: ...
    @staticmethod
    def boolean(op: BooleanOp, a: NodeId, b: NodeId) -> Node: ...

class ParamName:
    """A document-level parameter name (guide §3.2). NOT an arena
    key: the same plain name the recipe's expressions reference."""

    def __init__(self, name: str) -> None: ...
    @property
    def name(self) -> str: ...
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

class DocEdit:
    """A single edit — the G1 edit vocabulary (§L3)."""

    @staticmethod
    def insert_node(node: Node) -> DocEdit: ...
    @staticmethod
    def delete_node(id: NodeId) -> DocEdit: ...
    @staticmethod
    def set_tolerance(eps: float) -> DocEdit: ...
    @staticmethod
    def set_doc_param(name: ParamName, value: DocParam) -> DocEdit: ...

class Doc:
    """A parametric document: the recipe, not the geometry."""

    def __init__(self) -> None: ...
    def apply(self, edit: DocEdit) -> Optional[NodeId]: ...
    def insert(self, node: Node) -> NodeId: ...
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
    replayed edit count (LIB-DOORS F1)."""

    @property
    def doc(self) -> Doc: ...
    @property
    def snapshot(self) -> Doc: ...
    @property
    def edit_count(self) -> int: ...

def load(text: str) -> Loaded:
    """Parse, validate, and replay a saved document. Raises
    PersistError, typed."""

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
