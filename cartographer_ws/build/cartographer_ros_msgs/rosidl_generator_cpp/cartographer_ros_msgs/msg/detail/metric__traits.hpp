// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from cartographer_ros_msgs:msg/Metric.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "cartographer_ros_msgs/msg/metric.hpp"


#ifndef CARTOGRAPHER_ROS_MSGS__MSG__DETAIL__METRIC__TRAITS_HPP_
#define CARTOGRAPHER_ROS_MSGS__MSG__DETAIL__METRIC__TRAITS_HPP_

#include <stdint.h>

#include <array>
#include <cstddef>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>

#include "cartographer_ros_msgs/msg/detail/metric__struct.hpp"
#include "rosidl_runtime_cpp/buffer__traits.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'labels'
#include "cartographer_ros_msgs/msg/detail/metric_label__traits.hpp"
// Member 'counts_by_bucket'
#include "cartographer_ros_msgs/msg/detail/histogram_bucket__traits.hpp"

namespace cartographer_ros_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const Metric & msg,
  std::ostream & out)
{
  out << "{";
  // member: type
  {
    out << "type: ";
    rosidl_generator_traits::value_to_yaml(msg.type, out);
    out << ", ";
  }

  // member: labels
  {
    if (msg.labels.size() == 0) {
      out << "labels: []";
    } else {
      out << "labels: [";
      size_t pending_items = msg.labels.size();
      for (auto item : msg.labels) {
        to_flow_style_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
    out << ", ";
  }

  // member: value
  {
    out << "value: ";
    rosidl_generator_traits::value_to_yaml(msg.value, out);
    out << ", ";
  }

  // member: counts_by_bucket
  {
    if (msg.counts_by_bucket.size() == 0) {
      out << "counts_by_bucket: []";
    } else {
      out << "counts_by_bucket: [";
      size_t pending_items = msg.counts_by_bucket.size();
      for (auto item : msg.counts_by_bucket) {
        to_flow_style_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Metric & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: type
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "type: ";
    rosidl_generator_traits::value_to_yaml(msg.type, out);
    out << "\n";
  }

  // member: labels
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.labels.size() == 0) {
      out << "labels: []\n";
    } else {
      out << "labels:\n";
      for (auto item : msg.labels) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "-\n";
        to_block_style_yaml(item, out, indentation + 2);
      }
    }
  }

  // member: value
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "value: ";
    rosidl_generator_traits::value_to_yaml(msg.value, out);
    out << "\n";
  }

  // member: counts_by_bucket
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.counts_by_bucket.size() == 0) {
      out << "counts_by_bucket: []\n";
    } else {
      out << "counts_by_bucket:\n";
      for (auto item : msg.counts_by_bucket) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "-\n";
        to_block_style_yaml(item, out, indentation + 2);
      }
    }
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Metric & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

template<typename T, std::enable_if_t<std::is_same_v<std::decay_t<T>, cartographer_ros_msgs::msg::Metric>, int> = 0>
constexpr auto as_tuple_ref(T && msg)
{
  return std::forward_as_tuple(
    std::forward<T>(msg).type,
    std::forward<T>(msg).labels,
    std::forward<T>(msg).value,
    std::forward<T>(msg).counts_by_bucket);
}

}  // namespace msg

}  // namespace cartographer_ros_msgs

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<cartographer_ros_msgs::msg::Metric>()
{
  return "cartographer_ros_msgs::msg::Metric";
}

template<>
constexpr const char * name<cartographer_ros_msgs::msg::Metric>()
{
  return "cartographer_ros_msgs/msg/Metric";
}

template<>
struct has_fixed_size<cartographer_ros_msgs::msg::Metric>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<cartographer_ros_msgs::msg::Metric>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<cartographer_ros_msgs::msg::Metric>
  : std::true_type {};

template<>
struct MessageTraits<cartographer_ros_msgs::msg::Metric>
{
  static constexpr std::size_t member_count = 4;
  static constexpr std::array<std::string_view, member_count> member_names = {
    "type",
    "labels",
    "value",
    "counts_by_bucket",
  };
};

}  // namespace rosidl_generator_traits

#endif  // CARTOGRAPHER_ROS_MSGS__MSG__DETAIL__METRIC__TRAITS_HPP_
