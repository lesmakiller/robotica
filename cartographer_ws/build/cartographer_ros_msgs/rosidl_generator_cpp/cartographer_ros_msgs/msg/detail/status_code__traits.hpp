// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from cartographer_ros_msgs:msg/StatusCode.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "cartographer_ros_msgs/msg/status_code.hpp"


#ifndef CARTOGRAPHER_ROS_MSGS__MSG__DETAIL__STATUS_CODE__TRAITS_HPP_
#define CARTOGRAPHER_ROS_MSGS__MSG__DETAIL__STATUS_CODE__TRAITS_HPP_

#include <stdint.h>

#include <array>
#include <cstddef>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>

#include "cartographer_ros_msgs/msg/detail/status_code__struct.hpp"
#include "rosidl_runtime_cpp/buffer__traits.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace cartographer_ros_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const StatusCode & msg,
  std::ostream & out)
{
  (void)msg;
  out << "null";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const StatusCode & msg,
  std::ostream & out, size_t indentation = 0)
{
  (void)msg;
  (void)indentation;
  out << "null\n";
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const StatusCode & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

template<typename T, std::enable_if_t<std::is_same_v<std::decay_t<T>, cartographer_ros_msgs::msg::StatusCode>, int> = 0>
constexpr auto as_tuple_ref(T && msg)
{
  return std::forward_as_tuple(std::forward<T>(msg).structure_needs_at_least_one_member);
}

}  // namespace msg

}  // namespace cartographer_ros_msgs

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<cartographer_ros_msgs::msg::StatusCode>()
{
  return "cartographer_ros_msgs::msg::StatusCode";
}

template<>
constexpr const char * name<cartographer_ros_msgs::msg::StatusCode>()
{
  return "cartographer_ros_msgs/msg/StatusCode";
}

template<>
struct has_fixed_size<cartographer_ros_msgs::msg::StatusCode>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<cartographer_ros_msgs::msg::StatusCode>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<cartographer_ros_msgs::msg::StatusCode>
  : std::true_type {};

template<>
struct MessageTraits<cartographer_ros_msgs::msg::StatusCode>
{
  static constexpr std::size_t member_count = 1;
  static constexpr std::array<std::string_view, member_count> member_names = {
    "structure_needs_at_least_one_member",
  };
};

}  // namespace rosidl_generator_traits

#endif  // CARTOGRAPHER_ROS_MSGS__MSG__DETAIL__STATUS_CODE__TRAITS_HPP_
