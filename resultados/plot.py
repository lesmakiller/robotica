import os
import numpy as np
import matplotlib.pyplot as plt

d = os.path.expanduser('~/robotica/resultados')
load = lambda n: np.genfromtxt(f'{d}/{n}.csv', delimiter=',', names=True)

xs = np.linspace(-1, 6, 50)
cases = {
    'go_to_goal':    lambda: plt.scatter(2, 2, c='r', label='meta'),
    'line_follower': lambda: plt.plot(xs, (xs + 1) / 2, 'r--', label='línea'),
    'pure_pursuit':  lambda: plt.plot([0, .5, 1.5, 2], [0, 1, 1.25, 2.5], 'r--o', label='ruta'),
}
for name, overlay in cases.items():
    try:
        r = load(name)
    except OSError:
        continue
    plt.figure()
    plt.plot(r['x'], r['y'], label='robot')
    overlay()
    plt.axis('equal'); plt.grid(); plt.legend(); plt.title(name)
    plt.savefig(f'{d}/{name}.png', dpi=150)
plt.show()
