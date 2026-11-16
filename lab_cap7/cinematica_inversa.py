import numpy as np
import matplotlib.pyplot as plt

L1, L2 = 0.6, 0.4
puntos = {"A": (0.5, 0.5), "B": (0.95, 0.10), "C": (1.20, 0.20)}

def ik(xd, yd):
    c2 = (xd**2 + yd**2 - L1**2 - L2**2) / (2*L1*L2)
    if abs(c2) > 1:
        return c2, []
    sols = []
    for q2 in (np.arccos(c2), -np.arccos(c2)):
        q1 = np.arctan2(yd, xd) - np.arctan2(L2*np.sin(q2), L1 + L2*np.cos(q2))
        sols.append((q1, q2))
    return c2, sols

fig, axes = plt.subplots(1, 3, figsize=(15, 5))
for ax, (nombre, (xd, yd)) in zip(axes, puntos.items()):
    c2, sols = ik(xd, yd)
    print(f"\nPunto {nombre} = ({xd}, {yd})  r={np.hypot(xd, yd):.4f}  cos(q2)={c2:.4f}")
    if not sols:
        print("  Posición NO alcanzable (|cos q2| > 1)")
    for i, (q1, q2) in enumerate(sols):
        x = L1*np.cos(q1) + L2*np.cos(q1+q2)
        y = L1*np.sin(q1) + L2*np.sin(q1+q2)
        e = np.hypot(xd-x, yd-y)
        print(f"  Solución {i+1}: q=[{np.rad2deg(q1):.3f}°, {np.rad2deg(q2):.3f}°]  "
              f"pos=({x:.4f}, {y:.4f})  error={e:.2e}")
        ax.plot([0, L1*np.cos(q1), x], [0, L1*np.sin(q1), y], "o-", lw=3,
                label=f"Sol. {i+1}: ({np.rad2deg(q1):.1f}°, {np.rad2deg(q2):.1f}°)")
    ax.plot(xd, yd, "k*", ms=14, label="Objetivo")
    th = np.linspace(0, 2*np.pi, 300)
    ax.plot(1.0*np.cos(th), 1.0*np.sin(th), "k:", lw=0.8)
    ax.set_title(f"Punto {nombre}: ({xd}, {yd})")
    ax.axis("equal"); ax.grid(True); ax.legend(fontsize=8)
    ax.set_xlim(-0.2, 1.3); ax.set_ylim(-0.7, 1.1)
fig.savefig("fig_2_cinematica_inversa.png", dpi=150)
