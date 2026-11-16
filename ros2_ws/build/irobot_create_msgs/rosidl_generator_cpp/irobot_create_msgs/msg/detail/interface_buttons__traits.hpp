// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from irobot_create_msgs:msg/InterfaceButtons.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "irobot_create_msgs/msg/interface_buttons.hpp"


#ifndef IROBOT_CREATE_MSGS__MSG__DETAIL__INTERFACE_BUTTONS__TRAITS_HPP_
#define IROBOT_CREATE_MSGS__MSG__DETAIL__INTERFACE_BUTTONS__TRAITS_HPP_

#include <stdint.h>

#include <array>
#include <cstddef>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>

#include "irobot_create_msgs/msg/detail/interface_buttons__struct.hpp"
#include "rosidl_runtime_cpp/buffer__traits.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__traits.hpp"
// Member 'button_1'
// Member 'button_power'
// Member 'button_2'
#include "irobot_create_msgs/msg/detail/button__traits.hpp"

namespace irobot_create_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const InterfaceButtons & msg,
  std::ostream & out)
{
  out << "{";
  // member: header
  {
    out << "header: ";
    to_flow_style_yaml(msg.header, out);
    out << ", ";
  }

  // member: button_1
  {
    out << "button_1: ";
    to_flow_style_yaml(msg.button_1, out);
    out << ", ";
  }

  // member: button_power
  {
    out << "button_power: ";
    to_flow_style_yaml(msg.button_power, out);
    out << ", ";
  }

  // member: button_2
  {
    out << "button_2: ";
    to_flow_style_yaml(msg.button_2, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const InterfaceButtons & msg,
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

  // member: button_1
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "button_1:\n";
    to_block_style_yaml(msg.button_1, out, indentation + 2);
  }

  // member: button_power
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "button_power:\n";
    to_block_style_yaml(msg.button_power, out, indentation + 2);
  }

  // member: button_2
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "button_2:\n";
    to_block_style_yaml(msg.button_2, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const InterfaceButtons & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

template<typename T, std::enable_if_t<std::is_same_v<std::decay_t<T>, irobot_create_msgs::msg::InterfaceButtons>, int> = 0>
constexpr auto as_tuple_ref(T && msg)
{
  return std::forward_as_tuple(
    std::forward<T>(msg).header,
    std::forward<T>(msg).button_1,
    std::forward<T>(msg).button_power,
    std::forward<T>(msg).button_2);
}

}  // namespace msg

}  // namespace irobot_create_msgs

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<irobot_create_msgs::msg::InterfaceButtons>()
{
  return "irobot_create_msgs::msg::InterfaceButtons";
}

template<>
constexpr const char * name<irobot_create_msgs::msg::InterfaceButtons>()
{
  return "irobot_create_msgs/msg/InterfaceButtons";
}

template<>
struct has_fixed_size<irobot_create_msgs::msg::InterfaceButtons>
  : std::integral_constant<bool, has_fixed_size<irobot_create_msgs::msg::Button>::value && has_fixed_size<std_msgs::msg::Header>::value> {};

template<>
struct has_bounded_size<irobot_create_msgs::msg::InterfaceButtons>
  : std::integral_constant<bool, has_bounded_size<irobot_create_msgs::msg::Button>::value && has_bounded_size<std_msgs::msg::Header>::value> {};

template<>
struct is_message<irobot_create_msgs::msg::InterfaceButtons>
  : std::true_type {};

template<>
struct MessageTraits<irobot_create_msgs::msg::InterfaceButtons>
{
  static constexpr std::size_t member_count = 4;
  static constexpr std::array<std::string_view, member_count> member_names = {
    "header",
    "button_1",
    "button_power",
    "button_2",
  };
};

}  // namespace rosidl_generator_traits

#endif  // IROBOT_CREATE_MSGS__MSG__DETAIL__INTERFACE_BUTTONS__TRAITS_HPP_
