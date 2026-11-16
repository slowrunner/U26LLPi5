// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from irobot_create_msgs:msg/LightringLeds.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "irobot_create_msgs/msg/lightring_leds.hpp"


#ifndef IROBOT_CREATE_MSGS__MSG__DETAIL__LIGHTRING_LEDS__TRAITS_HPP_
#define IROBOT_CREATE_MSGS__MSG__DETAIL__LIGHTRING_LEDS__TRAITS_HPP_

#include <stdint.h>

#include <array>
#include <cstddef>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>

#include "irobot_create_msgs/msg/detail/lightring_leds__struct.hpp"
#include "rosidl_runtime_cpp/buffer__traits.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__traits.hpp"
// Member 'leds'
#include "irobot_create_msgs/msg/detail/led_color__traits.hpp"

namespace irobot_create_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const LightringLeds & msg,
  std::ostream & out)
{
  out << "{";
  // member: header
  {
    out << "header: ";
    to_flow_style_yaml(msg.header, out);
    out << ", ";
  }

  // member: leds
  {
    if (msg.leds.size() == 0) {
      out << "leds: []";
    } else {
      out << "leds: [";
      size_t pending_items = msg.leds.size();
      for (auto item : msg.leds) {
        to_flow_style_yaml(item, out);
        if (--pending_items > 0) {
          out << ", ";
        }
      }
      out << "]";
    }
    out << ", ";
  }

  // member: override_system
  {
    out << "override_system: ";
    rosidl_generator_traits::value_to_yaml(msg.override_system, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const LightringLeds & msg,
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

  // member: leds
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.leds.size() == 0) {
      out << "leds: []\n";
    } else {
      out << "leds:\n";
      for (auto item : msg.leds) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "-\n";
        to_block_style_yaml(item, out, indentation + 2);
      }
    }
  }

  // member: override_system
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "override_system: ";
    rosidl_generator_traits::value_to_yaml(msg.override_system, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const LightringLeds & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

template<typename T, std::enable_if_t<std::is_same_v<std::decay_t<T>, irobot_create_msgs::msg::LightringLeds>, int> = 0>
constexpr auto as_tuple_ref(T && msg)
{
  return std::forward_as_tuple(
    std::forward<T>(msg).header,
    std::forward<T>(msg).leds,
    std::forward<T>(msg).override_system);
}

}  // namespace msg

}  // namespace irobot_create_msgs

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<irobot_create_msgs::msg::LightringLeds>()
{
  return "irobot_create_msgs::msg::LightringLeds";
}

template<>
constexpr const char * name<irobot_create_msgs::msg::LightringLeds>()
{
  return "irobot_create_msgs/msg/LightringLeds";
}

template<>
struct has_fixed_size<irobot_create_msgs::msg::LightringLeds>
  : std::integral_constant<bool, has_fixed_size<irobot_create_msgs::msg::LedColor>::value && has_fixed_size<std_msgs::msg::Header>::value> {};

template<>
struct has_bounded_size<irobot_create_msgs::msg::LightringLeds>
  : std::integral_constant<bool, has_bounded_size<irobot_create_msgs::msg::LedColor>::value && has_bounded_size<std_msgs::msg::Header>::value> {};

template<>
struct is_message<irobot_create_msgs::msg::LightringLeds>
  : std::true_type {};

template<>
struct MessageTraits<irobot_create_msgs::msg::LightringLeds>
{
  static constexpr std::size_t member_count = 3;
  static constexpr std::array<std::string_view, member_count> member_names = {
    "header",
    "leds",
    "override_system",
  };
};

}  // namespace rosidl_generator_traits

#endif  // IROBOT_CREATE_MSGS__MSG__DETAIL__LIGHTRING_LEDS__TRAITS_HPP_
