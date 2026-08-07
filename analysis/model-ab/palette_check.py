#!/usr/bin/env python3
"""Palette gate check, reimplemented (no node on this box).

OKLab delta-E between series, under normal vision and simulated
protanopia/deuteranopia/tritanopia (Machado 2009, severity 1.0), plus
WCAG contrast against the chart surface.
"""
import math

CVD = {
    "protanopia": [[0.152286, 1.052583, -0.204868],
                   [0.114503, 0.786281, 0.099216],
                   [-0.003882, -0.048116, 1.051998]],
    "deuteranopia": [[0.367322, 0.860646, -0.227968],
                     [0.280085, 0.672501, 0.047413],
                     [-0.011820, 0.042940, 0.968881]],
    "tritanopia": [[1.255528, -0.076749, -0.178779],
                   [-0.078411, 0.930809, 0.147602],
                   [0.004733, 0.691367, 0.303900]],
}


def hex_rgb(h):
    h = h.lstrip("#")
    return [int(h[i:i + 2], 16) / 255.0 for i in (0, 2, 4)]


def to_linear(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def lin_rgb(h):
    return [to_linear(c) for c in hex_rgb(h)]


def matmul(M, v):
    return [sum(M[i][j] * v[j] for j in range(3)) for i in range(3)]


def oklab(rgb_lin):
    r, g, b = rgb_lin
    l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b
    m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b
    s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b
    l_, m_, s_ = [(-((-x) ** (1 / 3.0)) if x < 0 else x ** (1 / 3.0))
                  for x in (l, m, s)]
    return [0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
            1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
            0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_]


def dE(h1, h2, sim=None):
    a, b = lin_rgb(h1), lin_rgb(h2)
    if sim:
        a, b = matmul(CVD[sim], a), matmul(CVD[sim], b)
    la, lb = oklab(a), oklab(b)
    return 100.0 * math.sqrt(sum((x - y) ** 2 for x, y in zip(la, lb)))


def luminance(h):
    r, g, b = lin_rgb(h)
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def contrast(h, surface):
    a, b = luminance(h), luminance(surface)
    hi, lo = max(a, b), min(a, b)
    return (hi + 0.05) / (lo + 0.05)


def report(name, series, surface):
    print("\n=== %s (surface %s) ===" % (name, surface))
    ok = True
    n = dE(series[0], series[1])
    print("  normal-vision dE      %.1f   (floor 15)  %s"
          % (n, "PASS" if n >= 15 else "FAIL"))
    ok &= n >= 15
    for sim in ("protanopia", "deuteranopia", "tritanopia"):
        d = dE(series[0], series[1], sim)
        verdict = "PASS" if d >= 8 else ("WARN" if d >= 6 else "FAIL")
        print("  %-14s dE      %.1f   (target 8)  %s" % (sim, d, verdict))
        ok &= d >= 8
    for h in series:
        c = contrast(h, surface)
        print("  contrast %s        %.2f:1 (>=3)  %s"
              % (h, c, "PASS" if c >= 3 else "RELIEF-RULE"))
    return ok


if __name__ == "__main__":
    a = report("light", ["#2a78d6", "#eb6834"], "#fcfcfb")
    b = report("dark", ["#3987e5", "#d95926"], "#1a1a19")
    print("\nOVERALL:", "PASS" if (a and b) else "CHECK ABOVE")
