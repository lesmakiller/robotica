import numpy as np
import matplotlib.pyplot as plt

path = np.array([[0, 0], [2, 4], [6, 5], [8, 10]])
lookahead, Kd, dt, L, v = 0.5, 1.3, 0.1, 2.0, 0.5
x, y, theta = 0.0, 0.0, 0.0
current_index = 0
X, Y = [], []

def angle_wrap(a):
    return (a + np.pi) % (2 * np.pi) - np.pi

for _ in range(285):
    dx = path[current_index][0] - x
    dy = path[current_index][1] - y
    if np.hypot(dx, dy) < lookahead and current_index < len(path) - 1:
        current_index += 1
    target = path[current_index]
    theta_goal = np.arctan2(target[1] - y, target[0] - x)
    error = angle_wrap(theta_goal - theta)
    delta = Kd * error
    x += v * np.cos(theta) * dt
    y += v * np.sin(theta) * dt
    theta += v / L * np.tan(delta) * dt
    X.append(x); Y.append(y)

plt.plot(path[:, 0], path[:, 1], 'r--o', label="Trayectoria")
plt.plot(X, Y, label="Robot")
plt.axis('equal'); plt.grid(); plt.legend()
plt.savefig("act4.png", dpi=150)
plt.show()
