// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from irobot_create_msgs:msg/Button.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "irobot_create_msgs/msg/button.hpp"


#ifndef IROBOT_CREATE_MSGS__MSG__DETAIL__BUTTON__TRAITS_HPP_
#define IROBOT_CREATE_MSGS__MSG__DETAIL__BUTTON__TRAITS_HPP_

#include <stdint.h>

#include <array>
#include <cstddef>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>

#include "irobot_create_msgs/msg/detail/button__struct.hpp"
#include "rosidl_runtime_cpp/buffer__traits.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__traits.hpp"
// Member 'last_start_pressed_time'
#include "builtin_interfaces/msg/detail/time__traits.hpp"
// Member 'last_pressed_duration'
#include "builtin_interfaces/msg/detail/duration__traits.hpp"

namespace irobot_create_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const Button & msg,
  std::ostream & out)
{
  out << "{";
  // member: header
  {
    out << "header: ";
    to_flow_style_yaml(msg.header, out);
    out << ", ";
  }

  // member: is_pressed
  {
    out << "is_pressed: ";
    rosidl_generator_traits::value_to_yaml(msg.is_pressed, out);
    out << ", ";
  }

  // member: last_start_pressed_time
  {
    out << "last_start_pressed_time: ";
    to_flow_style_yaml(msg.last_start_pressed_time, out);
    out << ", ";
  }

  // member: last_pressed_duration
  {
    out << "last_pressed_duration: ";
    to_flow_style_yaml(msg.last_pressed_duration, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const Button & msg,
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

  // member: is_pressed
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "is_pressed: ";
    rosidl_generator_traits::value_to_yaml(msg.is_pressed, out);
    out << "\n";
  }

  // member: last_start_pressed_time
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "last_start_pressed_time:\n";
    to_block_style_yaml(msg.last_start_pressed_time, out, indentation + 2);
  }

  // member: last_pressed_duration
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "last_pressed_duration:\n";
    to_block_style_yaml(msg.last_pressed_duration, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const Button & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

template<typename T, std::enable_if_t<std::is_same_v<std::decay_t<T>, irobot_create_msgs::msg::Button>, int> = 0>
constexpr auto as_tuple_ref(T && msg)
{
  return std::forward_as_tuple(
    std::forward<T>(msg).header,
    std::forward<T>(msg).is_pressed,
    std::forward<T>(msg).last_start_pressed_time,
    std::forward<T>(msg).last_pressed_duration);
}

}  // namespace msg

}  // namespace irobot_create_msgs

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<irobot_create_msgs::msg::Button>()
{
  return "irobot_create_msgs::msg::Button";
}

template<>
constexpr const char * name<irobot_create_msgs::msg::Button>()
{
  return "irobot_create_msgs/msg/Button";
}

template<>
struct has_fixed_size<irobot_create_msgs::msg::Button>
  : std::integral_constant<bool, has_fixed_size<builtin_interfaces::msg::Duration>::value && has_fixed_size<builtin_interfaces::msg::Time>::value && has_fixed_size<std_msgs::msg::Header>::value> {};

template<>
struct has_bounded_size<irobot_create_msgs::msg::Button>
  : std::integral_constant<bool, has_bounded_size<builtin_interfaces::msg::Duration>::value && has_bounded_size<builtin_interfaces::msg::Time>::value && has_bounded_size<std_msgs::msg::Header>::value> {};

template<>
struct is_message<irobot_create_msgs::msg::Button>
  : std::true_type {};

template<>
struct MessageTraits<irobot_create_msgs::msg::Button>
{
  static constexpr std::size_t member_count = 4;
  static constexpr std::array<std::string_view, member_count> member_names = {
    "header",
    "is_pressed",
    "last_start_pressed_time",
    "last_pressed_duration",
  };
};

}  // namespace rosidl_generator_traits

#endif  // IROBOT_CREATE_MSGS__MSG__DETAIL__BUTTON__TRAITS_HPP_
