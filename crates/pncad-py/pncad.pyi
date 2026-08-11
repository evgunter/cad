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
`Node.polygon`; one loop or a list of them, on any sketch plane.

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

    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

class BooleanOp:
    """The DOCUMENT-layer Boolean operator."""

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
    ) -> Node: ...

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

# --- selectors --------------------------------------------------------
# The narrowing language (LIB-PYSEL, audit G13). Patterns and
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

class OpGroup:
    """The op group a role segment belongs to (`SegPat.group`)."""

    Shared: Final[OpGroup]
    Extrude: Final[OpGroup]
    Revolve: Final[OpGroup]
    Boolean: Final[OpGroup]
    Split: Final[OpGroup]
    Fillet: Final[OpGroup]
    Pattern: Final[OpGroup]

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
