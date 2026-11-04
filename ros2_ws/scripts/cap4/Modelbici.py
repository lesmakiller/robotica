import numpy as np
import matplotlib.pyplot as plt

L, v, delta, dt = 2.0, 1.0, 0.3, 0.1
x, y, theta = 0.0, 0.0, 0.0
X, Y = [], []

for _ in range(100):
    x += v * np.cos(theta) * dt
    y += v * np.sin(theta) * dt
    theta += v / L * np.tan(delta) * dt
    X.append(x); Y.append(y)

plt.plot(X, Y)
plt.title("Trayectoria del robot")
plt.axis('equal'); plt.grid()
plt.savefig("act1.png", dpi=150)
plt.show()
