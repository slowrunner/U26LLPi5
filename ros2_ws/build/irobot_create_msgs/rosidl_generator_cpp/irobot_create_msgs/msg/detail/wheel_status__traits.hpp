// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from irobot_create_msgs:msg/WheelStatus.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "irobot_create_msgs/msg/wheel_status.hpp"


#ifndef IROBOT_CREATE_MSGS__MSG__DETAIL__WHEEL_STATUS__TRAITS_HPP_
#define IROBOT_CREATE_MSGS__MSG__DETAIL__WHEEL_STATUS__TRAITS_HPP_

#include <stdint.h>

#include <array>
#include <cstddef>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>

#include "irobot_create_msgs/msg/detail/wheel_status__struct.hpp"
#include "rosidl_runtime_cpp/buffer__traits.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__traits.hpp"

namespace irobot_create_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const WheelStatus & msg,
  std::ostream & out)
{
  out << "{";
  // member: header
  {
    out << "header: ";
    to_flow_style_yaml(msg.header, out);
    out << ", ";
  }

  // member: current_ma_left
  {
    out << "current_ma_left: ";
    rosidl_generator_traits::value_to_yaml(msg.current_ma_left, out);
    out << ", ";
  }

  // member: current_ma_right
  {
    out << "current_ma_right: ";
    rosidl_generator_traits::value_to_yaml(msg.current_ma_right, out);
    out << ", ";
  }

  // member: pwm_left
  {
    out << "pwm_left: ";
    rosidl_generator_traits::value_to_yaml(msg.pwm_left, out);
    out << ", ";
  }

  // member: pwm_right
  {
    out << "pwm_right: ";
    rosidl_generator_traits::value_to_yaml(msg.pwm_right, out);
    out << ", ";
  }

  // member: wheels_enabled
  {
    out << "wheels_enabled: ";
    rosidl_generator_traits::value_to_yaml(msg.wheels_enabled, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const WheelStatus & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: header
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "header:\n";
    to_block_style_yaml(msg.header, out, indentation + 2);
  }

  // member: current_ma_left
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "current_ma_left: ";
    rosidl_generator_traits::value_to_yaml(msg.current_ma_left, out);
    out << "\n";
  }

  // member: current_ma_right
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "current_ma_right: ";
    rosidl_generator_traits::value_to_yaml(msg.current_ma_right, out);
    out << "\n";
  }

  // member: pwm_left
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pwm_left: ";
    rosidl_generator_traits::value_to_yaml(msg.pwm_left, out);
    out << "\n";
  }

  // member: pwm_right
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "pwm_right: ";
    rosidl_generator_traits::value_to_yaml(msg.pwm_right, out);
    out << "\n";
  }

  // member: wheels_enabled
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "wheels_enabled: ";
    rosidl_generator_traits::value_to_yaml(msg.wheels_enabled, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const WheelStatus & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

template<typename T, std::enable_if_t<std::is_same_v<std::decay_t<T>, irobot_create_msgs::msg::WheelStatus>, int> = 0>
constexpr auto as_tuple_ref(T && msg)
{
  return std::forward_as_tuple(
    std::forward<T>(msg).header,
    std::forward<T>(msg).current_ma_left,
    std::forward<T>(msg).current_ma_right,
    std::forward<T>(msg).pwm_left,
    std::forward<T>(msg).pwm_right,
    std::forward<T>(msg).wheels_enabled);
}

}  // namespace msg

}  // namespace irobot_create_msgs

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<irobot_create_msgs::msg::WheelStatus>()
{
  return "irobot_create_msgs::msg::WheelStatus";
}

template<>
constexpr const char * name<irobot_create_msgs::msg::WheelStatus>()
{
  return "irobot_create_msgs/msg/WheelStatus";
}

template<>
struct has_fixed_size<irobot_create_msgs::msg::WheelStatus>
  : std::integral_constant<bool, has_fixed_size<std_msgs::msg::Header>::value> {};

template<>
struct has_bounded_size<irobot_create_msgs::msg::WheelStatus>
  : std::integral_constant<bool, has_bounded_size<std_msgs::msg::Header>::value> {};

template<>
struct is_message<irobot_create_msgs::msg::WheelStatus>
  : std::true_type {};

template<>
struct MessageTraits<irobot_create_msgs::msg::WheelStatus>
{
  static constexpr std::size_t member_count = 6;
  static constexpr std::array<std::string_view, member_count> member_names = {
    "header",
    "current_ma_left",
    "current_ma_right",
    "pwm_left",
    "pwm_right",
    "wheels_enabled",
  };
};

}  // namespace rosidl_generator_traits

#endif  // IROBOT_CREATE_MSGS__MSG__DETAIL__WHEEL_STATUS__TRAITS_HPP_
