import numpy as np
import matplotlib.pyplot as plt

def sensorfield(x, y):
    xc, yc = 60, 90
    return 1000 / ((x - xc)**2 + (y - yc)**2 + 50)

dt, Ks, Kv = 0.1, 17, 14
x, y, theta = 10, 10, 0
traj_x, traj_y = [], []

for i in range(222):
    d = 2
    s_left = sensorfield(x - d*np.sin(theta), y + d*np.cos(theta))
    s_right = sensorfield(x + d*np.sin(theta), y - d*np.cos(theta))
    
    v = -Kv * (s_left + s_right)
    omega = Ks * (s_right - s_left)
    
    x += v * np.cos(theta) * dt
    y += v * np.sin(theta) * dt
    theta += omega * dt
    
    traj_x.append(x)
    traj_y.append(y)

plt.plot(traj_x, traj_y)
plt.scatter([60], [90], c='red')
plt.title("Navegación Braitenberg")
plt.axis('equal')
plt.show()
