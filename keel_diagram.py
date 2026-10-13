import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d.art3d import Poly3DCollection
from matplotlib.patches import Polygon, Rectangle, FancyArrowPatch, Arc

OUT = "/tmp/claude-0/-home-user-cad/13a560da-7ecc-50fe-9b1a-ae4821afb423/scratchpad/keel-geometry.png"

# Trench: prism over the (y,z) triangle (2,0),(2.6,2),(1.4,2), x in [1.5,2.5].
# Box: [0,4]x[0,4]x[-1,1].  Split plane z = 0.  Keel: (1.5,2,0)-(2.5,2,0).
x0, x1 = 1.5, 2.5
def trench_y(z):  # half-width of the trench at height z
    return 0.3 * z

fig = plt.figure(figsize=(13, 10.5))

# ---------- panel 1: 3-D view of the operand ----------
ax = fig.add_subplot(2, 2, 1, projection="3d")
ax.set_title("1. The body: box [0,4]×[0,4]×[−1,1] less a V-trench\n"
             "(trench bottom = the keel, lying in the split plane z = 0)", fontsize=10)
faces = []
# top face of box (z=1) drawn as polygon with the trench opening cut out (draw as 3 rects)
w1 = trench_y(1.0)
top = [
    [(0, 0, 1), (4, 0, 1), (4, 4, 1), (0, 4, 1)],
]
box_col = Poly3DCollection([
    [(0, 0, -1), (4, 0, -1), (4, 0, 1), (0, 0, 1)],
    [(4, 0, -1), (4, 4, -1), (4, 4, 1), (4, 0, 1)],
], alpha=0.12, facecolor="tab:blue", edgecolor="tab:blue", linewidths=0.6)
ax.add_collection3d(box_col)
# top face outline
for (a, b) in [((0, 0), (4, 0)), ((4, 0), (4, 4)), ((4, 4), (0, 4)), ((0, 4), (0, 0))]:
    ax.plot([a[0], b[0]], [a[1], b[1]], [1, 1], color="tab:blue", lw=0.8)
    ax.plot([a[0], b[0]], [a[1], b[1]], [-1, -1], color="tab:blue", lw=0.8)
for (px, py) in [(0, 0), (4, 0), (4, 4), (0, 4)]:
    ax.plot([px, px], [py, py], [-1, 1], color="tab:blue", lw=0.8)
# trench walls (inside box only, z in [0,1])
wallL = [(x0, 2, 0), (x1, 2, 0), (x1, 2 - w1, 1), (x0, 2 - w1, 1)]
wallR = [(x0, 2, 0), (x1, 2, 0), (x1, 2 + w1, 1), (x0, 2 + w1, 1)]
endA = [(x0, 2, 0), (x0, 2 - w1, 1), (x0, 2 + w1, 1)]
endB = [(x1, 2, 0), (x1, 2 - w1, 1), (x1, 2 + w1, 1)]
ax.add_collection3d(Poly3DCollection([wallL, wallR], alpha=0.55, facecolor="tab:orange",
                                     edgecolor="darkorange", linewidths=0.8))
ax.add_collection3d(Poly3DCollection([endA, endB], alpha=0.35, facecolor="tab:orange",
                                     edgecolor="darkorange", linewidths=0.8))
# split plane
P = [(-0.3, -0.3, 0), (4.3, -0.3, 0), (4.3, 4.3, 0), (-0.3, 4.3, 0)]
ax.add_collection3d(Poly3DCollection([P], alpha=0.10, facecolor="gray", edgecolor="gray"))
ax.plot([x0, x1], [2, 2], [0, 0], color="red", lw=3)
ax.text(2.0, 2.0, -0.35, "keel", color="red", fontsize=9, ha="center")
ax.text(4.3, 4.3, 0.05, "split plane z = 0", color="dimgray", fontsize=8)
ax.set_xlim(0, 4); ax.set_ylim(0, 4); ax.set_zlim(-1, 1.2)
ax.set_xlabel("x"); ax.set_ylabel("y"); ax.set_zlabel("z")
ax.view_init(elev=24, azim=-58)
ax.set_box_aspect((4, 4, 2.2))

# ---------- panel 2: cross-section at x = 2 ----------
ax = fig.add_subplot(2, 2, 2)
ax.set_title("2. Section across the keel (x = 2): the material meets\n"
             "itself along the keel in TWO wedges, with the trench between", fontsize=10)
ax.add_patch(Rectangle((0, -1), 4, 2, facecolor="tab:blue", alpha=0.18, edgecolor="tab:blue"))
ax.add_patch(Polygon([(2, 0), (2 + w1, 1), (2 - w1, 1)], closed=True, facecolor="white",
                     edgecolor="darkorange", lw=1.5))
ax.plot([2 - w1 * 1.15, 2, 2 + w1 * 1.15], [1.15, 0, 1.15], color="darkorange", lw=1.5, ls=":")
ax.axhline(0, color="gray", lw=1, ls="--")
ax.text(3.95, 0.04, "split plane z = 0", color="dimgray", fontsize=8, ha="right", va="bottom")
ax.plot([2], [0], "o", color="red", ms=7, zorder=5)
ax.text(2.05, -0.12, "keel (seen end-on)", color="red", fontsize=8, va="top")
ang = np.degrees(np.arctan(0.3))  # trench half-angle 16.7 deg
ax.add_patch(Arc((2, 0), 0.9, 0.9, theta1=0, theta2=90 - ang, color="black", lw=1))
ax.add_patch(Arc((2, 0), 0.9, 0.9, theta1=90 + ang, theta2=180, color="black", lw=1))
ax.text(2.55, 0.18, "wedge b\n≈73°", fontsize=8)
ax.text(1.12, 0.18, "wedge a\n≈73°", fontsize=8)
ax.text(2, 0.7, "trench\n(void)", fontsize=8, ha="center")
ax.text(1.0, 0.75, "Above", fontsize=10, color="tab:blue", weight="bold")
ax.text(1.0, -0.6, "Below", fontsize=10, color="tab:blue", weight="bold")
ax.set_xlim(0, 4); ax.set_ylim(-1.1, 1.25); ax.set_aspect("equal")
ax.set_xlabel("y"); ax.set_ylabel("z")

# ---------- panel 3: Above's section face, slit form (recommended) ----------
ax = fig.add_subplot(2, 2, 3)
ax.set_title("3. Above's bottom face (z = 0), SLIT FORM (this PR):\n"
             "the keel is a zero-area ring of two coincident edges a, b", fontsize=10)
ax.add_patch(Rectangle((0, 0), 4, 4, facecolor="tab:blue", alpha=0.18, edgecolor="tab:blue", lw=1.5))
off = 0.05  # drawn apart only so both edges are visible; they coincide
ax.annotate("", xy=(x1, 2 - off), xytext=(x0, 2 - off),
            arrowprops=dict(arrowstyle="->", color="red", lw=2))
ax.annotate("", xy=(x0, 2 + off), xytext=(x1, 2 + off),
            arrowprops=dict(arrowstyle="->", color="purple", lw=2))
ax.plot([x0, x1], [2, 2], "o", color="black", ms=5)
ax.text(2.0, 1.72, "edge a: floor | wall a", color="red", fontsize=8, ha="center", va="top")
ax.text(2.0, 2.28, "edge b: floor | wall b", color="purple", fontsize=8, ha="center", va="bottom")
ax.text(x0 - 0.08, 2, "v₁", fontsize=9, ha="right", va="center")
ax.text(x1 + 0.08, 2, "v₂", fontsize=9, ha="left", va="center")
ax.text(0.15, 3.7, "outer loop: the square", fontsize=8, color="tab:blue")
ax.text(0.15, 0.25, "a and b lie on the same segment\n(drawn slightly apart only to show both);\n"
        "the ring bounds no area. Each edge's\nwedge is its own ≈73° of material.", fontsize=8)
ax.set_xlim(-0.1, 4.1); ax.set_ylim(-0.1, 4.1); ax.set_aspect("equal")
ax.set_xlabel("x"); ax.set_ylabel("y")

# ---------- panel 4: rider form (what the Boolean builds today) ----------
ax = fig.add_subplot(2, 2, 4)
ax.set_title("4. RIDER FORM (what subtract() builds today): the keel is ONE\n"
             "edge (wall a | wall b) lying inside the plain-square floor", fontsize=10)
ax.add_patch(Rectangle((0, 0), 4, 4, facecolor="tab:blue", alpha=0.18, edgecolor="tab:blue", lw=1.5))
ax.plot([x0, x1], [2, 2], color="darkgreen", lw=2.5)
ax.plot([x0, x1], [2, 2], "o", color="black", ms=5)
ax.text(2.0, 2.12, "keel: wall a | wall b\n+ edge-on-face record", color="darkgreen",
        fontsize=8, ha="center", va="bottom")
ax.text(0.15, 0.25, "Its wedge claims ≈327° of material (all\nbut the trench), though the floor runs\n"
        "through it. Trench wall-to-wall: the same\nform returns ONE solid for two pieces\n"
        "touching along an edge (against D1).", fontsize=8)
ax.set_xlim(-0.1, 4.1); ax.set_ylim(-0.1, 4.1); ax.set_aspect("equal")
ax.set_xlabel("x"); ax.set_ylabel("y")

fig.suptitle("Keel wrapped round at both ends: split of the trenched box by z = 0", fontsize=13)
fig.tight_layout(rect=(0, 0, 1, 0.96))
fig.savefig(OUT, dpi=110)
print(OUT)
