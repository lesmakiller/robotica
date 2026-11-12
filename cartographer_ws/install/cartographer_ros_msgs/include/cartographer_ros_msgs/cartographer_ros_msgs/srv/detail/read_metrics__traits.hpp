// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from cartographer_ros_msgs:srv/ReadMetrics.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "cartographer_ros_msgs/srv/read_metrics.hpp"


#ifndef CARTOGRAPHER_ROS_MSGS__SRV__DETAIL__READ_METRICS__TRAITS_HPP_
#define CARTOGRAPHER_ROS_MSGS__SRV__DETAIL__READ_METRICS__TRAITS_HPP_

#include <stdint.h>

#include <array>
#include <cstddef>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>

#include "cartographer_ros_msgs/srv/detail/read_metrics__struct.hpp"
#include "rosidl_runtime_cpp/buffer__traits.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace cartographer_ros_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const ReadMetrics_Request & msg,
  std::ostream & out)
{
  (void)msg;
  out << "null";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ReadMetrics_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  (void)msg;
  (void)indentation;
  out << "null\n";
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ReadMetrics_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

template<typename T, std::enable_if_t<std::is_same_v<std::decay_t<T>, cartographer_ros_msgs::srv::ReadMetrics_Request>, int> = 0>
constexpr auto as_tuple_ref(T && msg)
{
  return std::forward_as_tuple(std::forward<T>(msg).structure_needs_at_least_one_member);
}

}  // namespace srv

}  // namespace cartographer_ros_msgs

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<cartographer_ros_msgs::srv::ReadMetrics_Request>()
{
  return "cartographer_ros_msgs::srv::ReadMetrics_Request";
}

template<>
constexpr const char * name<cartographer_ros_msgs::srv::ReadMetrics_Request>()
{
  return "cartographer_ros_msgs/srv/ReadMetrics_Request";
}

template<>
struct has_fixed_size<cartographer_ros_msgs::srv::ReadMetrics_Request>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<cartographer_ros_msgs::srv::ReadMetrics_Request>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<cartographer_ros_msgs::srv::ReadMetrics_Request>
  : std::true_type {};

template<>
struct MessageTraits<cartographer_ros_msgs::srv::ReadMetrics_Request>
{
  static constexpr std::size_t member_count = 1;
  static constexpr std::array<std::string_view, member_count> member_names = {
    "structure_needs_at_least_one_member",
  };
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'status'
#include "cartographer_ros_msgs/msg/detail/status_response__traits.hpp"
// Member 'metric_families'
#include "cartographer_ros_msgs/msg/detail/metric_family__traits.hpp"
// Member 'timestamp'
#include "builtin_interfaces/msg/detail/time__traits.hpp"

namespace cartographer_ros_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const ReadMetrics_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: status
  {
    out << "status: ";
    to_flow_style_yaml(msg.status, out);
    out << ", ";
  }

  // member: metric_families
  {
    if (msg.metric_families.size() == 0) {
      out << "metric_families: []";
    } else {
      out << "metric_families: [";
      size_t pending_items = msg.metric_families.size();
      for (auto item : msg.metric_families) {
        to_flow_style_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
    out << ", ";
  }

  // member: timestamp
  {
    out << "timestamp: ";
    to_flow_style_yaml(msg.timestamp, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const ReadMetrics_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "status:\n";
    to_block_style_yaml(msg.status, out, indentation + 2);
  }

  // member: metric_families
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.metric_families.size() == 0) {
      out << "metric_families: []\n";
    } else {
      out << "metric_families:\n";
      for (auto item : msg.metric_families) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "-\n";
        to_block_style_yaml(item, out, indentation + 2);
      }
    }
  }

  // member: timestamp
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "timestamp:\n";
    to_block_style_yaml(msg.timestamp, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ReadMetrics_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

template<typename T, std::enable_if_t<std::is_same_v<std::decay_t<T>, cartographer_ros_msgs::srv::ReadMetrics_Response>, int> = 0>
constexpr auto as_tuple_ref(T && msg)
{
  return std::forward_as_tuple(
    std::forward<T>(msg).status,
    std::forward<T>(msg).metric_families,
    std::forward<T>(msg).timestamp);
}

}  // namespace srv

}  // namespace cartographer_ros_msgs

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<cartographer_ros_msgs::srv::ReadMetrics_Response>()
{
  return "cartographer_ros_msgs::srv::ReadMetrics_Response";
}

template<>
constexpr const char * name<cartographer_ros_msgs::srv::ReadMetrics_Response>()
{
  return "cartographer_ros_msgs/srv/ReadMetrics_Response";
}

template<>
struct has_fixed_size<cartographer_ros_msgs::srv::ReadMetrics_Response>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<cartographer_ros_msgs::srv::ReadMetrics_Response>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<cartographer_ros_msgs::srv::ReadMetrics_Response>
  : std::true_type {};

template<>
struct MessageTraits<cartographer_ros_msgs::srv::ReadMetrics_Response>
{
  static constexpr std::size_t member_count = 3;
  static constexpr std::array<std::string_view, member_count> member_names = {
    "status",
    "metric_families",
    "timestamp",
  };
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'info'
#include "service_msgs/msg/detail/service_event_info__traits.hpp"

namespace cartographer_ros_msgs
{

namespace srv
{

inline void to_flow_style_yaml(
  const ReadMetrics_Event & msg,
  std::ostream & out)
{
  out << "{";
  // member: info
  {
    out << "info: ";
    to_flow_style_yaml(msg.info, out);
    out << ", ";
  }

  // member: request
  {
    if (msg.request.size() == 0) {
      out << "request: []";
    } else {
      out << "request: [";
      size_t pending_items = msg.request.size();
      for (auto item : msg.request) {
        to_flow_style_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
    out << ", ";
  }

  // member: response
  {
    if (msg.response.size() == 0) {
      out << "response: []";
    } else {
      out << "response: [";
      size_t pending_items = msg.response.size();
      for (auto item : msg.response) {
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
  const ReadMetrics_Event & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: info
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "info:\n";
    to_block_style_yaml(msg.info, out, indentation + 2);
  }

  // member: request
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.request.size() == 0) {
      out << "request: []\n";
    } else {
      out << "request:\n";
      for (auto item : msg.request) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "-\n";
        to_block_style_yaml(item, out, indentation + 2);
      }
    }
  }

  // member: response
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.response.size() == 0) {
      out << "response: []\n";
    } else {
      out << "response:\n";
      for (auto item : msg.response) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "-\n";
        to_block_style_yaml(item, out, indentation + 2);
      }
    }
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const ReadMetrics_Event & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

template<typename T, std::enable_if_t<std::is_same_v<std::decay_t<T>, cartographer_ros_msgs::srv::ReadMetrics_Event>, int> = 0>
constexpr auto as_tuple_ref(T && msg)
{
  return std::forward_as_tuple(
    std::forward<T>(msg).info,
    std::forward<T>(msg).request,
    std::forward<T>(msg).response);
}

}  // namespace srv

}  // namespace cartographer_ros_msgs

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<cartographer_ros_msgs::srv::ReadMetrics_Event>()
{
  return "cartographer_ros_msgs::srv::ReadMetrics_Event";
}

template<>
constexpr const char * name<cartographer_ros_msgs::srv::ReadMetrics_Event>()
{
  return "cartographer_ros_msgs/srv/ReadMetrics_Event";
}

template<>
struct has_fixed_size<cartographer_ros_msgs::srv::ReadMetrics_Event>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<cartographer_ros_msgs::srv::ReadMetrics_Event>
  : std::integral_constant<bool, has_bounded_size<cartographer_ros_msgs::srv::ReadMetrics_Request>::value && has_bounded_size<cartographer_ros_msgs::srv::ReadMetrics_Response>::value && has_bounded_size<service_msgs::msg::ServiceEventInfo>::value> {};

template<>
struct is_message<cartographer_ros_msgs::srv::ReadMetrics_Event>
  : std::true_type {};

template<>
struct MessageTraits<cartographer_ros_msgs::srv::ReadMetrics_Event>
{
  static constexpr std::size_t member_count = 3;
  static constexpr std::array<std::string_view, member_count> member_names = {
    "info",
    "request",
    "response",
  };
};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<cartographer_ros_msgs::srv::ReadMetrics>()
{
  return "cartographer_ros_msgs::srv::ReadMetrics";
}

template<>
constexpr const char * name<cartographer_ros_msgs::srv::ReadMetrics>()
{
  return "cartographer_ros_msgs/srv/ReadMetrics";
}

template<>
struct has_fixed_size<cartographer_ros_msgs::srv::ReadMetrics>
  : std::integral_constant<
    bool,
    has_fixed_size<cartographer_ros_msgs::srv::ReadMetrics_Request>::value &&
    has_fixed_size<cartographer_ros_msgs::srv::ReadMetrics_Response>::value
  >
{
};

template<>
struct has_bounded_size<cartographer_ros_msgs::srv::ReadMetrics>
  : std::integral_constant<
    bool,
    has_bounded_size<cartographer_ros_msgs::srv::ReadMetrics_Request>::value &&
    has_bounded_size<cartographer_ros_msgs::srv::ReadMetrics_Response>::value
  >
{
};

template<>
struct is_service<cartographer_ros_msgs::srv::ReadMetrics>
  : std::true_type
{
};

template<>
struct is_service_request<cartographer_ros_msgs::srv::ReadMetrics_Request>
  : std::true_type
{
};

template<>
struct is_service_response<cartographer_ros_msgs::srv::ReadMetrics_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

#endif  // CARTOGRAPHER_ROS_MSGS__SRV__DETAIL__READ_METRICS__TRAITS_HPP_
