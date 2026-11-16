// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from irobot_create_msgs:msg/IrIntensityVector.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "irobot_create_msgs/msg/ir_intensity_vector.hpp"


#ifndef IROBOT_CREATE_MSGS__MSG__DETAIL__IR_INTENSITY_VECTOR__TRAITS_HPP_
#define IROBOT_CREATE_MSGS__MSG__DETAIL__IR_INTENSITY_VECTOR__TRAITS_HPP_

#include <stdint.h>

#include <array>
#include <cstddef>
#include <sstream>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>

#include "irobot_create_msgs/msg/detail/ir_intensity_vector__struct.hpp"
#include "rosidl_runtime_cpp/buffer__traits.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

// Include directives for member types
// Member 'header'
#include "std_msgs/msg/detail/header__traits.hpp"
// Member 'readings'
#include "irobot_create_msgs/msg/detail/ir_intensity__traits.hpp"

namespace irobot_create_msgs
{

namespace msg
{

inline void to_flow_style_yaml(
  const IrIntensityVector & msg,
  std::ostream & out)
{
  out << "{";
  // member: header
  {
    out << "header: ";
    to_flow_style_yaml(msg.header, out);
    out << ", ";
  }

  // member: readings
  {
    if (msg.readings.size() == 0) {
      out << "readings: []";
    } else {
      out << "readings: [";
      size_t pending_items = msg.readings.size();
      for (auto item : msg.readings) {
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
  const IrIntensityVector & msg,
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

  // member: readings
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    if (msg.readings.size() == 0) {
      out << "readings: []\n";
    } else {
      out << "readings:\n";
      for (auto item : msg.readings) {
        if (indentation > 0) {
          out << std::string(indentation, ' ');
        }
        out << "-\n";
        to_block_style_yaml(item, out, indentation + 2);
      }
    }
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const IrIntensityVector & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

template<typename T, std::enable_if_t<std::is_same_v<std::decay_t<T>, irobot_create_msgs::msg::IrIntensityVector>, int> = 0>
constexpr auto as_tuple_ref(T && msg)
{
  return std::forward_as_tuple(
    std::forward<T>(msg).header,
    std::forward<T>(msg).readings);
}

}  // namespace msg

}  // namespace irobot_create_msgs

namespace rosidl_generator_traits
{

template<>
constexpr const char * data_type<irobot_create_msgs::msg::IrIntensityVector>()
{
  return "irobot_create_msgs::msg::IrIntensityVector";
}

template<>
constexpr const char * name<irobot_create_msgs::msg::IrIntensityVector>()
{
  return "irobot_create_msgs/msg/IrIntensityVector";
}

template<>
struct has_fixed_size<irobot_create_msgs::msg::IrIntensityVector>
  : std::integral_constant<bool, false> {};

template<>
struct has_bounded_size<irobot_create_msgs::msg::IrIntensityVector>
  : std::integral_constant<bool, false> {};

template<>
struct is_message<irobot_create_msgs::msg::IrIntensityVector>
  : std::true_type {};

template<>
struct MessageTraits<irobot_create_msgs::msg::IrIntensityVector>
{
  static constexpr std::size_t member_count = 2;
  static constexpr std::array<std::string_view, member_count> member_names = {
    "header",
    "readings",
  };
};

}  // namespace rosidl_generator_traits

#endif  // IROBOT_CREATE_MSGS__MSG__DETAIL__IR_INTENSITY_VECTOR__TRAITS_HPP_
