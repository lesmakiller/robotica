import numpy as np
import matplotlib.pyplot as plt
from roboticstoolbox import ET2

L1, L2 = 0.6, 0.4

# Modelo con transformaciones elementales (Robotics Toolbox)
robot = ET2.R() * ET2.tx(L1) * ET2.R() * ET2.tx(L2)

configs = {
    "A": [0, 0],
    "B": [30, 45],
    "C": [60, -30],
    "D": [90, 90],
}

# ---------- Actividad 1.1: cuatro configuraciones en una figura ----------
fig, ax = plt.subplots(figsize=(7, 7))
print(f"{'Conf':<5}{'q1[°]':>7}{'q2[°]':>7}{'x[m]':>9}{'y[m]':>9}{'phi[°]':>9}   (analítico vs RTB)")
for nombre, qdeg in configs.items():
    q1, q2 = np.deg2rad(qdeg)
    # Analítico
    x1, y1 = L1*np.cos(q1), L1*np.sin(q1)
    x2, y2 = x1 + L2*np.cos(q1+q2), y1 + L2*np.sin(q1+q2)
    phi = np.rad2deg(q1+q2)
    # Robotics Toolbox
    T = robot.fkine(np.deg2rad(qdeg))
    xt, yt = T.t
    phit = np.rad2deg(np.arctan2(T.R[1, 0], T.R[0, 0]))
    ok = np.allclose([x2, y2], [xt, yt])
    print(f"{nombre:<5}{qdeg[0]:>7}{qdeg[1]:>7}{x2:>9.4f}{y2:>9.4f}{phi:>9.1f}   RTB: ({xt:.4f}, {yt:.4f}, {phit:.1f}°) coincide={ok}")
    print(T)
    ax.plot([0, x1, x2], [0, y1, y2], "o-", lw=3, ms=8,
            label=f"{nombre}: ({qdeg[0]}°, {qdeg[1]}°)")

ax.set_xlabel("X [m]"); ax.set_ylabel("Y [m]")
ax.set_title("Manipulador 2R: configuraciones A–D")
ax.grid(True); ax.axis("equal"); ax.legend()
fig.savefig("fig_1_1_configuraciones.png", dpi=150)

# ---------- Actividad 1.2: espacio de trabajo ----------
theta = np.linspace(0, 2*np.pi, 500)
r_min, r_max = abs(L1-L2), L1+L2
fig, ax = plt.subplots(figsize=(7, 7))
ax.plot(r_max*np.cos(theta), r_max*np.sin(theta), label="Límite exterior")
ax.plot(r_min*np.cos(theta), r_min*np.sin(theta), label="Límite interior")
ax.fill(r_max*np.cos(theta), r_max*np.sin(theta), alpha=0.1)
ax.fill(r_min*np.cos(theta), r_min*np.sin(theta), color="white")
ax.set_xlabel("X [m]"); ax.set_ylabel("Y [m]")
ax.set_title("Espacio de trabajo del manipulador")
ax.axis("equal"); ax.grid(True); ax.legend()
fig.savefig("fig_1_2_espacio_trabajo.png", dpi=150)
print("r_min =", r_min, " r_max =", r_max)
