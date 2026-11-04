# parte2.py
import numpy as np
import matplotlib.pyplot as plt
from roboticstoolbox import Bicycle, EKF, RandomPath

V = np.diag([0.02, np.deg2rad(0.5)])**2
robot = Bicycle(covar=V, animation=None)
robot.control = RandomPath(workspace=10)

P0 = np.diag([0.05, 0.05, np.deg2rad(0.5)])**2
ekf = EKF(robot=(robot, V), P0=P0)
ekf.run(T=20)

plt.figure(figsize=(8, 6))
robot.plot_xy(color='blue', label='Real')
ekf.plot_xy(color='red', label='Estimada')
plt.title('Trayectoria real vs estimada'); plt.grid(True); plt.legend()
plt.savefig('parte2_trayectoria.png', dpi=150)

plt.figure(figsize=(8, 6))
plt.plot(ekf.get_t(), ekf.get_Pnorm(), linewidth=2)
plt.xlabel('Tiempo [s]'); plt.ylabel('Incertidumbre')
plt.title('Crecimiento de incertidumbre'); plt.grid(True)
plt.savefig('parte2_incertidumbre.png', dpi=150)
plt.show()
