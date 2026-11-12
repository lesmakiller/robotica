# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target cartographer_ros_msgs::cartographer_ros_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${cartographer_ros_msgs_TARGETS}.
if(cartographer_ros_msgs_TARGETS AND NOT TARGET cartographer_ros_msgs::cartographer_ros_msgs)
  add_library(cartographer_ros_msgs::cartographer_ros_msgs INTERFACE IMPORTED)
  set_target_properties(cartographer_ros_msgs::cartographer_ros_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${cartographer_ros_msgs_TARGETS}")
endif()
