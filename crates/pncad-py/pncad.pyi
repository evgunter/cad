"""Type stubs for the `pncad` extension module (LIB-U9S scaffold).

LIBRARY-DESIGN §L4's two-layer story: these stubs are the STATIC
layer, checked by `ty`; the runtime layer lives once at the Rust
boundary and never re-verifies inside Python.

The PATHS lattice (`PathOpen`/`PathPoint`/`PathAngle`/`PathDirected`)
is deliberately ABSENT: profile authoring ships only against the v2
program representation (LQ4), so its stub lattice belongs to the unit
that binds it, not to this scaffold.
"""

from typing import Any, Final, Optional

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
    `node_failed`, or `poisoned`. A `node_failed`/`poisoned` refusal
    carries `kind`, the `NodeErrorKind`'s stable tag; a poisoning
    carries `through`, the nearest failed ancestor (LIB-DOORS F3).
    """

    reason: str
    node: NodeId
    kind: str
    through: NodeId

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
    curated error, LIB-DOORS F5)."""

    kind: str

class PersistError(PncadError):
    """A save or load the persistence doors refused (LIB-DOORS F1)."""

    variant: str

class ExportError(PncadError):
    """The document-layer export door refused (LIB-DOORS F2). A
    poisoning adds `through`; a wrong-kind value adds `kind`."""

    variant: str
    node: NodeId
    through: NodeId
    kind: str

class StepImportError(PncadError):
    """A STEP text the importer refused (`refused`), or one that
    parsed to a non-solid (`wireframe`)."""

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

class DocEdit:
    """A single edit — the G1 edit vocabulary (§L3)."""

    @staticmethod
    def insert_node(node: Node) -> DocEdit: ...
    @staticmethod
    def delete_node(id: NodeId) -> DocEdit: ...
    @staticmethod
    def set_tolerance(eps: float) -> DocEdit: ...

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
