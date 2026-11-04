import numpy as np
import matplotlib.pyplot as plt

grid = np.zeros((20, 20))
grid[8:12, 5:15] = 1 # obstáculo
goal = (2, 2)

dist = np.ones_like(grid) * np.inf
dist[goal[1], goal[0]] = 0

# Propagación
for _ in range(100):
    for x in range(20):
        for y in range(20):
            if grid[y,x] == 0:
                neighbors = []
                for dx, dy in [(-1,0), (1,0), (0,-1), (0,1)]:
                    nx, ny = x+dx, y+dy
                    if 0 <= nx < 20 and 0 <= ny < 20:
                        neighbors.append(dist[ny, nx])
                if neighbors:
                    dist[y,x] = min(dist[y,x], min(neighbors) + 1)

# Seguimiento
start = (18, 18)
pos = start
path = [pos]
for _ in range(200):
    x, y = pos
    neighbors = []
    for dx, dy in [(-1,0), (1,0), (0,-1), (0,1)]:
        nx, ny = x+dx, y+dy
        if 0 <= nx < 20 and 0 <= ny < 20:
            neighbors.append(((nx, ny), dist[ny, nx]))
    pos = min(neighbors, key=lambda p: p[1])[0]
    path.append(pos)
    if pos == goal:
        break

path = np.array(path)

plt.figure(figsize=(6,6))
plt.imshow(dist, origin='lower', cmap='hot')
obs_y, obs_x = np.where(grid == 1)
plt.scatter(obs_x, obs_y, c='black', s=20)
plt.plot(path[:,0], path[:,1], 'cyan', linewidth=2, label="trayectoria")
plt.scatter(start[0], start[1], c='blue', s=80, label="inicio")
plt.scatter(goal[0], goal[1], c='green', s=80, label="objetivo")
plt.colorbar(label="Distancia al objetivo")
plt.title("Distance Transform")
plt.legend()
plt.show()
