# parte1.py
import numpy as np
import matplotlib.pyplot as plt
from roboticstoolbox import Bicycle

V = np.diag([0.2, np.deg2rad(0.5)])**2      # [σ_d², σ_δθ²]
print(V)

robot = Bicycle(covar=V)                    # con ruido
robot_ideal = Bicycle(covar=np.zeros((2, 2)))  # sin ruido

qp_real, qp_ideal = [0, 0, 0], [0, 0, 0]
x_real, y_real, x_ideal, y_ideal = [], [], [], []

for _ in range(200):
    odo_real = robot.step((1.0, 0.2))
    qp_real = robot.f(qp_real, odo_real)
    odo_ideal = robot_ideal.step((1.0, 0.2))
    qp_ideal = robot.f(qp_ideal, odo_ideal)

    x_real.append(qp_real[0]);   y_real.append(qp_real[1])
    x_ideal.append(qp_ideal[0]); y_ideal.append(qp_ideal[1])

plt.figure(figsize=(8, 6))
plt.plot(x_ideal, y_ideal, label='Odometría sin ruido', linewidth=1)
plt.plot(x_real, y_real, label='Odometría con ruido', linewidth=1)
plt.xlabel('X [m]'); plt.ylabel('Y [m]')
plt.title('Comparación de trayectorias')
plt.grid(True); plt.axis('equal'); plt.legend()
plt.savefig('parte1_trayectorias.png', dpi=150)

# Extra para el informe: error vs tiempo
err = np.hypot(np.array(x_real) - x_ideal, np.array(y_real) - y_ideal)
plt.figure(figsize=(8, 5))
plt.plot(np.arange(200) * 0.1, err)
plt.xlabel('Tiempo [s]'); plt.ylabel('Error de posición [m]')
plt.title('Crecimiento del error de odometría'); plt.grid(True)
plt.savefig('parte1_error.png', dpi=150)
plt.show()
