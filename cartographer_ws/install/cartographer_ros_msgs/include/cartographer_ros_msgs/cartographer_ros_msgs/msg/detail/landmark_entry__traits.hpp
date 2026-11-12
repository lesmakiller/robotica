// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from cartographer_ros_msgs:msg/LandmarkEntry.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "cartographer_ros_msgs/msg/landmark_entry.hpp"


#ifndef CARTOGRAPHER_ROS_MSGS__MSG__DETAIL__LANDMARK_ENTRY__TRAITS_HPP_
#define CARTOGRAPHER_ROS_MSGS__MSG__DETAIL__LANDMARK_ENTRY__TRAITS_HPP_

#include <stdint.h>

#include <array>
#include <cstddef>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>

#include "cartographer_ros_msgs/msg/detail/landmark_entry__struct.hpp"
#include "rosidl_runtime_cpp/buffer__traits.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'tracking_from_landmark_transform'
#include "geometry_msgs/msg/detail/pose__traits.hpp"

namespace cartographer_ros_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const LandmarkEntry & msg,
  std::ostream & out)
{
  out << "{";
  // member: id
  {
    out << "id: ";
    rosidl_generator_traits::value_to_yaml(msg.id, out);
    out << ", ";
  }

  // member: tracking_from_landmark_transform
  {
    out << "tracking_from_landmark_transform: ";
    to_flow_style_yaml(msg.tracking_from_landmark_transform, out);
    out << ", ";
  }

  // member: translation_weight
  {
    out << "translation_weight: ";
    rosidl_generator_traits::value_to_yaml(msg.translation_weight, out);
    out << ", ";
  }

  // member: rotation_weight
  {
    out << "rotation_weight: ";
    rosidl_generator_traits::value_to_yaml(msg.rotation_weight, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const LandmarkEntry & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "id: ";
    rosidl_generator_traits::value_to_yaml(msg.id, out);
    out << "\n";
  }

  // member: tracking_from_landmark_transform
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "tracking_from_landmark_transform:\n";
    to_block_style_yaml(msg.tracking_from_landmark_transform, out, indentation + 2);
  }

  // member: translation_weight
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "translation_weight: ";
    rosidl_generator_traits::value_to_yaml(msg.translation_weight, out);
    out << "\n";
  }

  // member: rotation_weight
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "rotation_weight: ";
    rosidl_generator_traits::value_to_yaml(msg.rotation_weight, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const LandmarkEntry & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

template<typename T, std::enable_if_t<std::is_same_v<std::decay_t<T>, cartographer_ros_msgs::msg::LandmarkEntry>, int> = 0>
constexpr auto as_tuple_ref(T && msg)
{
  return std::forward_as_tuple(
    std::forward<T>(msg).id,
    std::forward<T>(msg).tracking_from_landmark_transform,
    std::forward<T>(msg).translation_weight,
    std::forward<T>(msg).rotation_weight);
}

}  // namespace msg

}  // namespace cartographer_ros_msgs

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<cartographer_ros_msgs::msg::LandmarkEntry>()
{
  return "cartographer_ros_msgs::msg::LandmarkEntry";
}

template<>
constexpr const char * name<cartographer_ros_msgs::msg::LandmarkEntry>()
{
  return "cartographer_ros_msgs/msg/LandmarkEntry";
}

template<>
struct has_fixed_size<cartographer_ros_msgs::msg::LandmarkEntry>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<cartographer_ros_msgs::msg::LandmarkEntry>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<cartographer_ros_msgs::msg::LandmarkEntry>
  : std::true_type {};

template<>
struct MessageTraits<cartographer_ros_msgs::msg::LandmarkEntry>
{
  static constexpr std::size_t member_count = 4;
  static constexpr std::array<std::string_view, member_count> member_names = {
    "id",
    "tracking_from_landmark_transform",
    "translation_weight",
    "rotation_weight",
  };
};

}  // namespace rosidl_generator_traits

#endif  // CARTOGRAPHER_ROS_MSGS__MSG__DETAIL__LANDMARK_ENTRY__TRAITS_HPP_
