import rclpy
from rclpy.node import Node
from sensor_msgs.msg import Imu
from geometry_msgs.msg import TransformStamped
from tf2_ros import TransformBroadcaster
from scipy.spatial.transform import Rotation as R
import numpy as np


class IMUIntegrator(Node):
    def __init__(self):
        super().__init__('imu_integrator')
        # Suscriptor IMU
        self.sub = self.create_subscription(
            Imu,
            '/imu',
            self.imu_callback,
            10
        )
        # TF broadcaster
        self.br = TransformBroadcaster(self)
        # Estado (cuaternión)
        self.q = R.identity()
        # Tiempo
        self.last_time = None
        self.get_logger().info("IMU Integrator iniciado")

    def imu_callback(self, msg):
        current_time = self.get_clock().now()

        # Primera iteración
        if self.last_time is None:
            self.last_time = current_time
            return

        # Delta de tiempo
        dt = (current_time - self.last_time).nanoseconds * 1e-9
        self.last_time = current_time

        # Evitar dt inválido
        if dt <= 0:
            return

        # Velocidad angular (rad/s)
        w = np.array([
            msg.angular_velocity.x,
            msg.angular_velocity.y,
            msg.angular_velocity.z
        ])

        # Integración (cuaternión incremental)
        dq = R.from_rotvec(w * dt)

        # Actualizar orientación
        # (scipy normaliza internamente, no hace falta .normalized())
        self.q = self.q * dq

        # Publicar TF
        self.publish_tf(current_time)

    def publish_tf(self, current_time):
        t = TransformStamped()
        t.header.stamp = current_time.to_msg()
        t.header.frame_id = 'world'
        t.child_frame_id = 'imu_link'

        # Sin traslación (solo orientación)
        t.transform.translation.x = 0.0
        t.transform.translation.y = 0.0
        t.transform.translation.z = 0.0

        q = self.q.as_quat()
        t.transform.rotation.x = float(q[0])
        t.transform.rotation.y = float(q[1])
        t.transform.rotation.z = float(q[2])
        t.transform.rotation.w = float(q[3])

        self.br.sendTransform(t)


def main():
    rclpy.init()
    node = IMUIntegrator()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()
