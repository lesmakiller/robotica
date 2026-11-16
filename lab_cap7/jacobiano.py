import numpy as np
import matplotlib.pyplot as plt

L1, L2 = 0.6, 0.4

def jac(q1, q2):
    return np.array([
        [-L1*np.sin(q1) - L2*np.sin(q1+q2), -L2*np.sin(q1+q2)],
        [ L1*np.cos(q1) + L2*np.cos(q1+q2),  L2*np.cos(q1+q2)],
    ])

q1, q2 = np.deg2rad(30), np.deg2rad(45)
J = jac(q1, q2)
print("Jacobiano:\n", J)
print("det(J) =", np.linalg.det(J), " L1*L2*sin(q2) =", L1*L2*np.sin(q2))
print("valores singulares:", np.linalg.svd(J, compute_uv=False))

# Determinante vs q2
q2v = np.linspace(-np.pi, np.pi, 400)
plt.figure(figsize=(8, 5))
plt.plot(np.rad2deg(q2v), L1*L2*np.sin(q2v))
plt.axhline(0, color="k", ls="--")
plt.xlabel("q2 [°]"); plt.ylabel("det J [m²]")
plt.title("Singularidades del manipulador 2R"); plt.grid(True)
plt.savefig("fig_3_determinante.png", dpi=150)

# Configuraciones singulares
fig, axes = plt.subplots(1, 2, figsize=(10, 5))
for ax, (q1d, q2d, nom) in zip(axes, [(30, 0, "Brazo extendido"), (30, 180, "Brazo plegado")]):
    a, b = np.deg2rad(q1d), np.deg2rad(q2d)
    x1, y1 = L1*np.cos(a), L1*np.sin(a)
    x2, y2 = x1 + L2*np.cos(a+b), y1 + L2*np.sin(a+b)
    ax.plot([0, x1, x2], [0, y1, y2], "o-", lw=3, ms=8)
    Js = jac(a, b)
    print(f"{nom}: q=({q1d}°, {q2d}°)  det={np.linalg.det(Js):.2e}\n{Js}")
    ax.set_title(f"{nom}\nq=({q1d}°, {q2d}°), det J = 0")
    ax.axis("equal"); ax.grid(True)
    ax.set_xlim(-0.2, 1.1); ax.set_ylim(-0.2, 0.7)
fig.savefig("fig_3_singularidades.png", dpi=150)
