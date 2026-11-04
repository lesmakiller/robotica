import numpy as np
import matplotlib.pyplot as plt

a, b, c = 1, -2, 4                 # recta ax + by + c = 0
x, y, theta = 8, 5, np.pi / 2
Kd, Kh, dt, L, v = 0.5, 1.0, 0.1, 2.0, 1
X, Y = [], []

for _ in range(200):
    distance = (a * x + b * y + c) / np.sqrt(a**2 + b**2)
    theta_line = np.arctan2(a, -b)
    delta = Kd * distance + Kh * (theta_line - theta)
    x += v * np.cos(theta) * dt
    y += v * np.sin(theta) * dt
    theta += v / L * np.tan(delta) * dt
    X.append(x); Y.append(y)

x_line = np.linspace(3, 25, 100)
y_line = (-a * x_line - c) / b
plt.plot(X, Y, label="Trayectoria del robot")
plt.plot(x_line, y_line, 'r--', label="Línea objetivo")
plt.axis('equal'); plt.grid(); plt.legend()
plt.savefig("act3.png", dpi=150)
plt.show()
