from scipy.spatial.transform import Rotation as R

r1 = R.from_euler('z', 90, degrees=True)
r2 = R.from_euler('x', 90, degrees=True)

r_total_12 = r1 * r2  # composición r1 luego r2
r_total_21 = r2 * r1  # composición r2 luego r1

print("R1 * R2:\n", r_total_12.as_matrix())
print("R2 * R1:\n", r_total_21.as_matrix())
