from scipy.spatial.transform import Rotation as R

# Rotación de 90° en Z
r = R.from_euler('z', 90, degrees=True)
q = r.as_quat()  # [x, y, z, w]
R_mat = r.as_matrix()

print("Cuaternión:", q)
print("Matriz:\n", R_mat)
