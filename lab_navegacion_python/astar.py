import heapq
import numpy as np
import matplotlib.pyplot as plt

grid = np.zeros((20, 20))
grid[8:12, 5:15] = 1
start = (2, 2)
goal = (18, 18)

def heuristic(a, b):
    return np.linalg.norm(np.array(a)-np.array(b))

def astar(grid, start, goal):
    open_list = []
    heapq.heappush(open_list, (0, start))
    came_from = {}
    cost = {start: 0}
    
    while open_list:
        current = heapq.heappop(open_list)[1]
        if current == goal:
            break
            
        for dx, dy in [(-1,0), (1,0), (0,-1), (0,1)]:
            neighbor = (current[0]+dx, current[1]+dy)
            if (0 <= neighbor[0] < grid.shape[0] and
                0 <= neighbor[1] < grid.shape[1] and
                grid[neighbor[1], neighbor[0]] == 0):
                
                new_cost = cost[current] + 1
                if neighbor not in cost or new_cost < cost[neighbor]:
                    cost[neighbor] = new_cost
                    priority = new_cost + heuristic(neighbor, goal)
                    heapq.heappush(open_list, (priority, neighbor))
                    came_from[neighbor] = current
                    
    path = []
    node = goal
    while node != start:
        path.append(node)
        node = came_from.get(node, start)
        if node == start: break
    path.append(start)
    path.reverse()
    return path

path = np.array(astar(grid, start, goal))
plt.imshow(grid, origin='lower')
plt.plot(path[:,0], path[:,1], 'r')
plt.title("A* Path")
plt.show()
