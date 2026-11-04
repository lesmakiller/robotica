import numpy as np
import matplotlib.pyplot as plt

x, y, theta = 0.0, 0.0, 0.0
goal = np.array([5, 5])
Kv, Kh, dt, L = 0.5, 1.5, 0.1, 2.0
distance = 10.0
X, Y = [], []

while distance > 0.1:
    dx = goal[0] - x
    dy = goal[1] - y
    distance = np.sqrt(dx**2 + dy**2)
    theta_goal = np.arctan2(dy, dx)
    v = Kv * distance
    delta = Kh * (theta_goal - theta)
    x += v * np.cos(theta) * dt
    y += v * np.sin(theta) * dt
    theta += v / L * np.tan(delta) * dt
    X.append(x); Y.append(y)

plt.plot(X, Y)
plt.scatter(goal[0], goal[1], c='r')
plt.axis('equal'); plt.grid()
plt.savefig("act2.png", dpi=150)
plt.show()
