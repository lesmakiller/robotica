import numpy as np
import matplotlib.pyplot as plt

grid = np.zeros((20, 20))
grid[8:12, 5:15] = 1 # obstáculo
start = np.array([2, 2])
goal = np.array([18, 18])
pos = start.copy()
path = [pos.copy()]

def is_free(p):
    if 0 <= p[0] < 20 and 0 <= p[1] < 20:
        return grid[p[1], p[0]] == 0
    return False

for _ in range(500):
    direction = goal - pos
    if abs(direction[0]) > abs(direction[1]):
        step = np.array([np.sign(direction[0]), 0])
    else:
        step = np.array([0, np.sign(direction[1])])
        
    next_pos = pos + step
    
    if is_free(next_pos):
        pos = next_pos
    else:
        for move in [np.array([1,0]), np.array([0,1]), np.array([-1,0]), np.array([0,-1])]:
            if is_free(pos + move):
                pos = pos + move
                break
                
    path.append(pos.copy())
    if np.linalg.norm(pos - goal) < 1:
        break

path = np.array(path)
plt.imshow(grid, origin='lower', cmap='gray_r')
plt.plot(path[:,0], path[:,1], 'g')
plt.scatter(start[0], start[1], c='blue')
plt.scatter(goal[0], goal[1], c='red')
plt.title("Bug2 Simplificado")
plt.show()
