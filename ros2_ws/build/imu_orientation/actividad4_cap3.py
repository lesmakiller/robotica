from roboticstoolbox import quintic, trapezoidal
import numpy as np

t = np.linspace(0, 1, 50)
traj_q = quintic(0, 1, t)
traj_t = trapezoidal(0, 1, t)

print("Quíntica:\n", traj_q.q)
print("Trapezoidal:\n", traj_t.q)
