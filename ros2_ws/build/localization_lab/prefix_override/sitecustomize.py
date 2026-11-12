import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/lesmakiller-u/robotica/ros2_ws/install/localization_lab'
