// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from irobot_create_msgs:msg/IrOpcode.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "irobot_create_msgs/msg/ir_opcode.hpp"


#ifndef IROBOT_CREATE_MSGS__MSG__DETAIL__IR_OPCODE__TRAITS_HPP_
#define IROBOT_CREATE_MSGS__MSG__DETAIL__IR_OPCODE__TRAITS_HPP_

#include <stdint.h>

#include <array>
#include <cstddef>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>

#include "irobot_create_msgs/msg/detail/ir_opcode__struct.hpp"
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
  const IrOpcode & msg,
  std::ostream & out)
{
  out << "{";
  // member: header
  {
    out << "header: ";
    to_flow_style_yaml(msg.header, out);
    out << ", ";
  }

  // member: opcode
  {
    out << "opcode: ";
    rosidl_generator_traits::value_to_yaml(msg.opcode, out);
    out << ", ";
  }

  // member: sensor
  {
    out << "sensor: ";
    rosidl_generator_traits::value_to_yaml(msg.sensor, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const IrOpcode & msg,
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

  // member: opcode
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "opcode: ";
    rosidl_generator_traits::value_to_yaml(msg.opcode, out);
    out << "\n";
  }

  // member: sensor
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "sensor: ";
    rosidl_generator_traits::value_to_yaml(msg.sensor, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const IrOpcode & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

template<typename T, std::enable_if_t<std::is_same_v<std::decay_t<T>, irobot_create_msgs::msg::IrOpcode>, int> = 0>
constexpr auto as_tuple_ref(T && msg)
{
  return std::forward_as_tuple(
    std::forward<T>(msg).header,
    std::forward<T>(msg).opcode,
    std::forward<T>(msg).sensor);
}

}  // namespace msg

}  // namespace irobot_create_msgs

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<irobot_create_msgs::msg::IrOpcode>()
{
  return "irobot_create_msgs::msg::IrOpcode";
}

template<>
constexpr const char * name<irobot_create_msgs::msg::IrOpcode>()
{
  return "irobot_create_msgs/msg/IrOpcode";
}

template<>
struct has_fixed_size<irobot_create_msgs::msg::IrOpcode>
  : std::integral_constant<bool, has_fixed_size<std_msgs::msg::Header>::value> {};

template<>
struct has_bounded_size<irobot_create_msgs::msg::IrOpcode>
  : std::integral_constant<bool, has_bounded_size<std_msgs::msg::Header>::value> {};

template<>
struct is_message<irobot_create_msgs::msg::IrOpcode>
  : std::true_type {};

template<>
struct MessageTraits<irobot_create_msgs::msg::IrOpcode>
{
  static constexpr std::size_t member_count = 3;
  static constexpr std::array<std::string_view, member_count> member_names = {
    "header",
    "opcode",
    "sensor",
  };
};

}  // namespace rosidl_generator_traits

#endif  // IROBOT_CREATE_MSGS__MSG__DETAIL__IR_OPCODE__TRAITS_HPP_
