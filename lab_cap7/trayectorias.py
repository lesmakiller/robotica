import numpy as np
import matplotlib.pyplot as plt
from roboticstoolbox.tools.trajectory import jtraj

L1, L2 = 0.6, 0.4
q0 = np.deg2rad([0, 0])
qf = np.deg2rad([60, -45])

def simular(T, n=251):
    t = np.linspace(0, T, n)
    tr = jtraj(q0, qf, t)
    return t, tr.q, tr.qd, tr.qdd

res = {}
for T in (5.0, 3.0):
    t, q, qd, qdd = simular(T)
    res[T] = (t, q, qd, qdd)
    print(f"\nT = {T} s")
    print("  q(0) [°] =", np.rad2deg(q[0]), " q(T) [°] =", np.rad2deg(q[-1]))
    print("  qd(0), qd(T) [°/s] =", np.rad2deg(qd[0]), np.rad2deg(qd[-1]))
    print("  qdd(0), qdd(T) [°/s²] =", np.rad2deg(qdd[0]), np.rad2deg(qdd[-1]))
    print("  |qd|max  [°/s]  =", np.rad2deg(np.abs(qd).max(axis=0)))
    print("  |qdd|max [°/s²] =", np.rad2deg(np.abs(qdd).max(axis=0)))

# Gráficas articulares (T=5 y T=3)
for T, (t, q, qd, qdd) in res.items():
    fig, axes = plt.subplots(3, 1, figsize=(8, 9))
    for ax, v, ti, un in zip(axes, [q, qd, qdd],
            ["Posiciones articulares", "Velocidades articulares", "Aceleraciones articulares"],
            ["Posición [°]", "Velocidad [°/s]", "Aceleración [°/s²]"]):
        ax.plot(t, np.rad2deg(v[:, 0]), label="Articulación 1")
        ax.plot(t, np.rad2deg(v[:, 1]), label="Articulación 2")
        ax.set_title(f"{ti} (T = {T:.0f} s)"); ax.set_xlabel("Tiempo [s]")
        ax.set_ylabel(un); ax.grid(True); ax.legend()
    fig.tight_layout(); fig.savefig(f"fig_4_1_articulares_T{int(T)}.png", dpi=150)

# Trayectoria cartesiana (T = 5 s)
t, q, qd, qdd = res[5.0]
q1, q2 = q[:, 0], q[:, 1]
x = L1*np.cos(q1) + L2*np.cos(q1+q2)
y = L1*np.sin(q1) + L2*np.sin(q1+q2)
print("\nInicio:", x[0], y[0], " Final:", x[-1], y[-1])

# Desviación máxima respecto a la recta inicio-final
p0, pf = np.array([x[0], y[0]]), np.array([x[-1], y[-1]])
d = (pf - p0) / np.linalg.norm(pf - p0)
n = np.array([-d[1], d[0]])
dev = (np.c_[x, y] - p0) @ n
print("Longitud del segmento recto [m]:", np.linalg.norm(pf - p0))
print("Desviación máxima respecto a la recta [m]:", np.abs(dev).max())

plt.figure(figsize=(7, 7))
plt.plot(x, y, label="Trayectoria cartesiana")
plt.plot([p0[0], pf[0]], [p0[1], pf[1]], "k--", label="Segmento recto")
plt.scatter(x[0], y[0], c="g", zorder=3, label="Inicio")
plt.scatter(x[-1], y[-1], c="r", zorder=3, label="Final")
plt.xlabel("X [m]"); plt.ylabel("Y [m]")
plt.title("Trayectoria del efector final")
plt.axis("equal"); plt.grid(True); plt.legend()
plt.savefig("fig_4_2_trayectoria_cartesiana.png", dpi=150)
