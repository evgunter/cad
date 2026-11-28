"""Apply one named mutant (or the base's spelling) to the tree in place.
Usage: python3 -I mutants.py <name>. Restore with git checkout."""
import sys

Q = "crates/geom-brep/src/quartic.rs"
L = "crates/geom-brep/src/curve_face/line_quadric.rs"

NEW_Q = """    let (b_le, b_gt) = (b.min(T::zero()), b.max(T::zero()));
    // `√disc − b` adds magnitudes for `b ≤ 0`, `−b − √disc` for `b > 0`.
    let (plus, minus) = (root - b_le, T::zero() - b_gt - root);
    [
        b.select_le_zero(c / plus, minus / a),
        b.select_le_zero(plus / a, c / minus),
    ]
}"""

MUT = {
    # The merge base's quadratic_roots.
    "base-roots": [(Q, NEW_Q, """    let (minus, plus) = (T::zero() - b - root, root - b);
    [
        b.select_le_zero(c / plus, minus / a),
        b.select_le_zero(plus / a, c / minus),
    ]
}""")],
    # The merge base's nearest point (shift clamped, b and c off w).
    "base-nearest": [(L, """    let w = w0p + dp * shift;
    Centred::new(
        (w0, d.get(), shift, w),
        [a2, slope, square - radius.powi(2)],""", """    let _ = (slope, square);
    let w = w0p + dp * shift;
    Centred::new(
        (w0, d.get(), shift, w),
        [a2, w.dot(dp), w.norm_squared() - radius.powi(2)],""")],
    # 1: the excess forced to zero.
    "m1": [(L, """                (a2 * hi + g).min(T::zero()) - (T::zero() - g - a2 * lo).min(T::zero()),""",
            """                T::zero(),""")],
    # 2: the cancelling spelling.
    "m2": [(Q, NEW_Q, """    let _ = (b_le_unused(), c);
    [(T::zero() - b - root) / a, (root - b) / a]
}
fn b_le_unused() {}""")],
    # 3: the roots not moved back by the shift.
    "m3": [(L, "(roots.map(|r| r + self.shift), slack)", "(roots, slack)")],
    # 4: |w|^2 without the slope term.
    "m4": [(L, "square: (w0.cross(d).norm_squared() + slope.powi(2)) / a2,",
            "square: w0.cross(d).norm_squared() / a2,")],
}

for path, old, new in MUT[sys.argv[1]]:
    s = open(path).read()
    assert s.count(old) == 1, (sys.argv[1], path, s.count(old))
    open(path, "w").write(s.replace(old, new))
print("applied", sys.argv[1])
