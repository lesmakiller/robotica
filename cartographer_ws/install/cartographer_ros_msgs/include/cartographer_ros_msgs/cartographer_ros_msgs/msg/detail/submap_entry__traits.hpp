// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from cartographer_ros_msgs:msg/SubmapEntry.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "cartographer_ros_msgs/msg/submap_entry.hpp"


#ifndef CARTOGRAPHER_ROS_MSGS__MSG__DETAIL__SUBMAP_ENTRY__TRAITS_HPP_
#define CARTOGRAPHER_ROS_MSGS__MSG__DETAIL__SUBMAP_ENTRY__TRAITS_HPP_

#include <stdint.h>

#include <array>
#include <cstddef>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>

#include "cartographer_ros_msgs/msg/detail/submap_entry__struct.hpp"
#include "rosidl_runtime_cpp/buffer__traits.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'pose'
#include "geometry_msgs/msg/detail/pose__traits.hpp"

namespace cartographer_ros_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const SubmapEntry & msg,
  std::ostream & out)
{
  out << "{";
  // member: trajectory_id
  {
    out << "trajectory_id: ";
    rosidl_generator_traits::value_to_yaml(msg.trajectory_id, out);
    out << ", ";
  }

  // member: submap_index
  {
    out << "submap_index: ";
    rosidl_generator_traits::value_to_yaml(msg.submap_index, out);
    out << ", ";
  }

  // member: submap_version
  {
    out << "submap_version: ";
    rosidl_generator_traits::value_to_yaml(msg.submap_version, out);
    out << ", ";
  }

  // member: pose
  {
    out << "pose: ";
    to_flow_style_yaml(msg.pose, out);
    out << ", ";
  }

  // member: is_frozen
  {
    out << "is_frozen: ";
    rosidl_generator_traits::value_to_yaml(msg.is_frozen, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const SubmapEntry & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: trajectory_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "trajectory_id: ";
    rosidl_generator_traits::value_to_yaml(msg.trajectory_id, out);
    out << "\n";
  }

  // member: submap_index
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "submap_index: ";
    rosidl_generator_traits::value_to_yaml(msg.submap_index, out);
    out << "\n";
  }

  // member: submap_version
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "submap_version: ";
    rosidl_generator_traits::value_to_yaml(msg.submap_version, out);
    out << "\n";
  }

  // member: pose
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pose:\n";
    to_block_style_yaml(msg.pose, out, indentation + 2);
  }

  // member: is_frozen
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "is_frozen: ";
    rosidl_generator_traits::value_to_yaml(msg.is_frozen, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const SubmapEntry & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

template<typename T, std::enable_if_t<std::is_same_v<std::decay_t<T>, cartographer_ros_msgs::msg::SubmapEntry>, int> = 0>
constexpr auto as_tuple_ref(T && msg)
{
  return std::forward_as_tuple(
    std::forward<T>(msg).trajectory_id,
    std::forward<T>(msg).submap_index,
    std::forward<T>(msg).submap_version,
    std::forward<T>(msg).pose,
    std::forward<T>(msg).is_frozen);
}

}  // namespace msg

}  // namespace cartographer_ros_msgs

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<cartographer_ros_msgs::msg::SubmapEntry>()
{
  return "cartographer_ros_msgs::msg::SubmapEntry";
}

template<>
constexpr const char * name<cartographer_ros_msgs::msg::SubmapEntry>()
{
  return "cartographer_ros_msgs/msg/SubmapEntry";
}

template<>
struct has_fixed_size<cartographer_ros_msgs::msg::SubmapEntry>
  : std::integral_constant<bool, has_fixed_size<geometry_msgs::msg::Pose>::value> {};

template<>
struct has_bounded_size<cartographer_ros_msgs::msg::SubmapEntry>
  : std::integral_constant<bool, has_bounded_size<geometry_msgs::msg::Pose>::value> {};

template<>
struct is_message<cartographer_ros_msgs::msg::SubmapEntry>
  : std::true_type {};

template<>
struct MessageTraits<cartographer_ros_msgs::msg::SubmapEntry>
{
  static constexpr std::size_t member_count = 5;
  static constexpr std::array<std::string_view, member_count> member_names = {
    "trajectory_id",
    "submap_index",
    "submap_version",
    "pose",
    "is_frozen",
  };
};

}  // namespace rosidl_generator_traits

#endif  // CARTOGRAPHER_ROS_MSGS__MSG__DETAIL__SUBMAP_ENTRY__TRAITS_HPP_
