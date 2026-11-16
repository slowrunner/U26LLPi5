// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from turtlebot4_msgs:msg/UserLed.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "turtlebot4_msgs/msg/user_led.hpp"


#ifndef TURTLEBOT4_MSGS__MSG__DETAIL__USER_LED__TRAITS_HPP_
#define TURTLEBOT4_MSGS__MSG__DETAIL__USER_LED__TRAITS_HPP_

#include <stdint.h>

#include <array>
#include <cstddef>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>

#include "turtlebot4_msgs/msg/detail/user_led__struct.hpp"
#include "rosidl_runtime_cpp/buffer__traits.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace turtlebot4_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const UserLed & msg,
  std::ostream & out)
{
  out << "{";
  // member: led
  {
    out << "led: ";
    rosidl_generator_traits::value_to_yaml(msg.led, out);
    out << ", ";
  }

  // member: color
  {
    out << "color: ";
    rosidl_generator_traits::value_to_yaml(msg.color, out);
    out << ", ";
  }

  // member: blink_period
  {
    out << "blink_period: ";
    rosidl_generator_traits::value_to_yaml(msg.blink_period, out);
    out << ", ";
  }

  // member: duty_cycle
  {
    out << "duty_cycle: ";
    rosidl_generator_traits::value_to_yaml(msg.duty_cycle, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const UserLed & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: led
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "led: ";
    rosidl_generator_traits::value_to_yaml(msg.led, out);
    out << "\n";
  }

  // member: color
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "color: ";
    rosidl_generator_traits::value_to_yaml(msg.color, out);
    out << "\n";
  }

  // member: blink_period
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "blink_period: ";
    rosidl_generator_traits::value_to_yaml(msg.blink_period, out);
    out << "\n";
  }

  // member: duty_cycle
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "duty_cycle: ";
    rosidl_generator_traits::value_to_yaml(msg.duty_cycle, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const UserLed & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

template<typename T, std::enable_if_t<std::is_same_v<std::decay_t<T>, turtlebot4_msgs::msg::UserLed>, int> = 0>
constexpr auto as_tuple_ref(T && msg)
{
  return std::forward_as_tuple(
    std::forward<T>(msg).led,
    std::forward<T>(msg).color,
    std::forward<T>(msg).blink_period,
    std::forward<T>(msg).duty_cycle);
}

}  // namespace msg

}  // namespace turtlebot4_msgs

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<turtlebot4_msgs::msg::UserLed>()
{
  return "turtlebot4_msgs::msg::UserLed";
}

template<>
constexpr const char * name<turtlebot4_msgs::msg::UserLed>()
{
  return "turtlebot4_msgs/msg/UserLed";
}

template<>
struct has_fixed_size<turtlebot4_msgs::msg::UserLed>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<turtlebot4_msgs::msg::UserLed>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<turtlebot4_msgs::msg::UserLed>
  : std::true_type {};

template<>
struct MessageTraits<turtlebot4_msgs::msg::UserLed>
{
  static constexpr std::size_t member_count = 4;
  static constexpr std::array<std::string_view, member_count> member_names = {
    "led",
    "color",
    "blink_period",
    "duty_cycle",
  };
};

}  // namespace rosidl_generator_traits

#endif  // TURTLEBOT4_MSGS__MSG__DETAIL__USER_LED__TRAITS_HPP_
