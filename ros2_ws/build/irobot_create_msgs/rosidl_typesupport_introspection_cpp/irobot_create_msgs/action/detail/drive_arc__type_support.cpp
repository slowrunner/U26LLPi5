// generated from rosidl_typesupport_introspection_cpp/resource/idl__type_support.cpp.em
// with input from irobot_create_msgs:action/DriveArc.idl
// generated code does not contain a copyright notice

#include "array"
#include "cstddef"
#include "string"
#include "vector"
#include "rosidl_runtime_c/message_type_support_struct.h"
#include "rosidl_typesupport_cpp/message_type_support.hpp"
#include "rosidl_typesupport_interface/macros.h"
#include "irobot_create_msgs/action/detail/drive_arc__functions.h"
#include "irobot_create_msgs/action/detail/drive_arc__struct.hpp"
#include "rosidl_typesupport_introspection_cpp/field_types.hpp"
#include "rosidl_typesupport_introspection_cpp/identifier.hpp"
#include "rosidl_typesupport_introspection_cpp/message_introspection.hpp"
#include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
#include "rosidl_typesupport_introspection_cpp/visibility_control.h"

namespace irobot_create_msgs
{

namespace action
{

namespace rosidl_typesupport_introspection_cpp
{

void DriveArc_Goal_init_function(
  void * message_memory, rosidl_runtime_cpp::MessageInitialization _init)
{
  new (message_memory) irobot_create_msgs::action::DriveArc_Goal(_init);
}

void DriveArc_Goal_fini_function(void * message_memory)
{
  auto typed_message = static_cast<irobot_create_msgs::action::DriveArc_Goal *>(message_memory);
  typed_message->~DriveArc_Goal();
}

static const ::rosidl_typesupport_introspection_cpp::MessageMember DriveArc_Goal_message_member_array[4] = {
  {
    "translate_direction",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_INT8,  // type
    0,  // upper bound of string
    nullptr,  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_Goal, translate_direction),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  },
  {
    "angle",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    nullptr,  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_Goal, angle),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  },
  {
    "radius",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    nullptr,  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_Goal, radius),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  },
  {
    "max_translation_speed",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    nullptr,  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_Goal, max_translation_speed),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  }
};

static const ::rosidl_typesupport_introspection_cpp::MessageMembers DriveArc_Goal_message_members = {
  "irobot_create_msgs::action",  // message namespace
  "DriveArc_Goal",  // message name
  4,  // number of fields
  sizeof(irobot_create_msgs::action::DriveArc_Goal),
  false,  // has_any_key_member_
  DriveArc_Goal_message_member_array,  // message members
  DriveArc_Goal_init_function,  // function to initialize message memory (memory has to be allocated)
  DriveArc_Goal_fini_function  // function to terminate message instance (will not free memory)
};

static const rosidl_message_type_support_t DriveArc_Goal_message_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &DriveArc_Goal_message_members,
  get_message_typesupport_handle_function,
  &irobot_create_msgs__action__DriveArc_Goal__get_type_hash,
  &irobot_create_msgs__action__DriveArc_Goal__get_type_description,
  &irobot_create_msgs__action__DriveArc_Goal__get_type_description_sources,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace action

}  // namespace irobot_create_msgs


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<irobot_create_msgs::action::DriveArc_Goal>()
{
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_Goal_message_type_support_handle;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, irobot_create_msgs, action, DriveArc_Goal)() {
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_Goal_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "array"
// already included above
// #include "cstddef"
// already included above
// #include "string"
// already included above
// #include "vector"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_interface/macros.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__functions.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__struct.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/field_types.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_introspection.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/visibility_control.h"

namespace irobot_create_msgs
{

namespace action
{

namespace rosidl_typesupport_introspection_cpp
{

void DriveArc_Result_init_function(
  void * message_memory, rosidl_runtime_cpp::MessageInitialization _init)
{
  new (message_memory) irobot_create_msgs::action::DriveArc_Result(_init);
}

void DriveArc_Result_fini_function(void * message_memory)
{
  auto typed_message = static_cast<irobot_create_msgs::action::DriveArc_Result *>(message_memory);
  typed_message->~DriveArc_Result();
}

static const ::rosidl_typesupport_introspection_cpp::MessageMember DriveArc_Result_message_member_array[1] = {
  {
    "pose",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<geometry_msgs::msg::PoseStamped>(),  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_Result, pose),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  }
};

static const ::rosidl_typesupport_introspection_cpp::MessageMembers DriveArc_Result_message_members = {
  "irobot_create_msgs::action",  // message namespace
  "DriveArc_Result",  // message name
  1,  // number of fields
  sizeof(irobot_create_msgs::action::DriveArc_Result),
  false,  // has_any_key_member_
  DriveArc_Result_message_member_array,  // message members
  DriveArc_Result_init_function,  // function to initialize message memory (memory has to be allocated)
  DriveArc_Result_fini_function  // function to terminate message instance (will not free memory)
};

static const rosidl_message_type_support_t DriveArc_Result_message_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &DriveArc_Result_message_members,
  get_message_typesupport_handle_function,
  &irobot_create_msgs__action__DriveArc_Result__get_type_hash,
  &irobot_create_msgs__action__DriveArc_Result__get_type_description,
  &irobot_create_msgs__action__DriveArc_Result__get_type_description_sources,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace action

}  // namespace irobot_create_msgs


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<irobot_create_msgs::action::DriveArc_Result>()
{
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_Result_message_type_support_handle;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, irobot_create_msgs, action, DriveArc_Result)() {
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_Result_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "array"
// already included above
// #include "cstddef"
// already included above
// #include "string"
// already included above
// #include "vector"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_interface/macros.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__functions.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__struct.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/field_types.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_introspection.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/visibility_control.h"

namespace irobot_create_msgs
{

namespace action
{

namespace rosidl_typesupport_introspection_cpp
{

void DriveArc_Feedback_init_function(
  void * message_memory, rosidl_runtime_cpp::MessageInitialization _init)
{
  new (message_memory) irobot_create_msgs::action::DriveArc_Feedback(_init);
}

void DriveArc_Feedback_fini_function(void * message_memory)
{
  auto typed_message = static_cast<irobot_create_msgs::action::DriveArc_Feedback *>(message_memory);
  typed_message->~DriveArc_Feedback();
}

static const ::rosidl_typesupport_introspection_cpp::MessageMember DriveArc_Feedback_message_member_array[1] = {
  {
    "remaining_angle_travel",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_FLOAT,  // type
    0,  // upper bound of string
    nullptr,  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_Feedback, remaining_angle_travel),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  }
};

static const ::rosidl_typesupport_introspection_cpp::MessageMembers DriveArc_Feedback_message_members = {
  "irobot_create_msgs::action",  // message namespace
  "DriveArc_Feedback",  // message name
  1,  // number of fields
  sizeof(irobot_create_msgs::action::DriveArc_Feedback),
  false,  // has_any_key_member_
  DriveArc_Feedback_message_member_array,  // message members
  DriveArc_Feedback_init_function,  // function to initialize message memory (memory has to be allocated)
  DriveArc_Feedback_fini_function  // function to terminate message instance (will not free memory)
};

static const rosidl_message_type_support_t DriveArc_Feedback_message_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &DriveArc_Feedback_message_members,
  get_message_typesupport_handle_function,
  &irobot_create_msgs__action__DriveArc_Feedback__get_type_hash,
  &irobot_create_msgs__action__DriveArc_Feedback__get_type_description,
  &irobot_create_msgs__action__DriveArc_Feedback__get_type_description_sources,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace action

}  // namespace irobot_create_msgs


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<irobot_create_msgs::action::DriveArc_Feedback>()
{
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_Feedback_message_type_support_handle;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, irobot_create_msgs, action, DriveArc_Feedback)() {
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_Feedback_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "array"
// already included above
// #include "cstddef"
// already included above
// #include "string"
// already included above
// #include "vector"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_interface/macros.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__functions.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__struct.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/field_types.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_introspection.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/visibility_control.h"

namespace irobot_create_msgs
{

namespace action
{

namespace rosidl_typesupport_introspection_cpp
{

void DriveArc_SendGoal_Request_init_function(
  void * message_memory, rosidl_runtime_cpp::MessageInitialization _init)
{
  new (message_memory) irobot_create_msgs::action::DriveArc_SendGoal_Request(_init);
}

void DriveArc_SendGoal_Request_fini_function(void * message_memory)
{
  auto typed_message = static_cast<irobot_create_msgs::action::DriveArc_SendGoal_Request *>(message_memory);
  typed_message->~DriveArc_SendGoal_Request();
}

static const ::rosidl_typesupport_introspection_cpp::MessageMember DriveArc_SendGoal_Request_message_member_array[2] = {
  {
    "goal_id",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<unique_identifier_msgs::msg::UUID>(),  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_SendGoal_Request, goal_id),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  },
  {
    "goal",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<irobot_create_msgs::action::DriveArc_Goal>(),  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_SendGoal_Request, goal),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  }
};

static const ::rosidl_typesupport_introspection_cpp::MessageMembers DriveArc_SendGoal_Request_message_members = {
  "irobot_create_msgs::action",  // message namespace
  "DriveArc_SendGoal_Request",  // message name
  2,  // number of fields
  sizeof(irobot_create_msgs::action::DriveArc_SendGoal_Request),
  false,  // has_any_key_member_
  DriveArc_SendGoal_Request_message_member_array,  // message members
  DriveArc_SendGoal_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  DriveArc_SendGoal_Request_fini_function  // function to terminate message instance (will not free memory)
};

static const rosidl_message_type_support_t DriveArc_SendGoal_Request_message_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &DriveArc_SendGoal_Request_message_members,
  get_message_typesupport_handle_function,
  &irobot_create_msgs__action__DriveArc_SendGoal_Request__get_type_hash,
  &irobot_create_msgs__action__DriveArc_SendGoal_Request__get_type_description,
  &irobot_create_msgs__action__DriveArc_SendGoal_Request__get_type_description_sources,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace action

}  // namespace irobot_create_msgs


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<irobot_create_msgs::action::DriveArc_SendGoal_Request>()
{
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_SendGoal_Request_message_type_support_handle;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, irobot_create_msgs, action, DriveArc_SendGoal_Request)() {
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_SendGoal_Request_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "array"
// already included above
// #include "cstddef"
// already included above
// #include "string"
// already included above
// #include "vector"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_interface/macros.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__functions.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__struct.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/field_types.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_introspection.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/visibility_control.h"

namespace irobot_create_msgs
{

namespace action
{

namespace rosidl_typesupport_introspection_cpp
{

void DriveArc_SendGoal_Response_init_function(
  void * message_memory, rosidl_runtime_cpp::MessageInitialization _init)
{
  new (message_memory) irobot_create_msgs::action::DriveArc_SendGoal_Response(_init);
}

void DriveArc_SendGoal_Response_fini_function(void * message_memory)
{
  auto typed_message = static_cast<irobot_create_msgs::action::DriveArc_SendGoal_Response *>(message_memory);
  typed_message->~DriveArc_SendGoal_Response();
}

static const ::rosidl_typesupport_introspection_cpp::MessageMember DriveArc_SendGoal_Response_message_member_array[2] = {
  {
    "accepted",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    nullptr,  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_SendGoal_Response, accepted),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  },
  {
    "stamp",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<builtin_interfaces::msg::Time>(),  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_SendGoal_Response, stamp),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  }
};

static const ::rosidl_typesupport_introspection_cpp::MessageMembers DriveArc_SendGoal_Response_message_members = {
  "irobot_create_msgs::action",  // message namespace
  "DriveArc_SendGoal_Response",  // message name
  2,  // number of fields
  sizeof(irobot_create_msgs::action::DriveArc_SendGoal_Response),
  false,  // has_any_key_member_
  DriveArc_SendGoal_Response_message_member_array,  // message members
  DriveArc_SendGoal_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  DriveArc_SendGoal_Response_fini_function  // function to terminate message instance (will not free memory)
};

static const rosidl_message_type_support_t DriveArc_SendGoal_Response_message_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &DriveArc_SendGoal_Response_message_members,
  get_message_typesupport_handle_function,
  &irobot_create_msgs__action__DriveArc_SendGoal_Response__get_type_hash,
  &irobot_create_msgs__action__DriveArc_SendGoal_Response__get_type_description,
  &irobot_create_msgs__action__DriveArc_SendGoal_Response__get_type_description_sources,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace action

}  // namespace irobot_create_msgs


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<irobot_create_msgs::action::DriveArc_SendGoal_Response>()
{
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_SendGoal_Response_message_type_support_handle;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, irobot_create_msgs, action, DriveArc_SendGoal_Response)() {
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_SendGoal_Response_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "array"
// already included above
// #include "cstddef"
// already included above
// #include "string"
// already included above
// #include "vector"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_interface/macros.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__functions.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__struct.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/field_types.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_introspection.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/visibility_control.h"

namespace irobot_create_msgs
{

namespace action
{

namespace rosidl_typesupport_introspection_cpp
{

void DriveArc_SendGoal_Event_init_function(
  void * message_memory, rosidl_runtime_cpp::MessageInitialization _init)
{
  new (message_memory) irobot_create_msgs::action::DriveArc_SendGoal_Event(_init);
}

void DriveArc_SendGoal_Event_fini_function(void * message_memory)
{
  auto typed_message = static_cast<irobot_create_msgs::action::DriveArc_SendGoal_Event *>(message_memory);
  typed_message->~DriveArc_SendGoal_Event();
}

size_t size_function__DriveArc_SendGoal_Event__request(const void * untyped_member)
{
  const auto * member = reinterpret_cast<const std::vector<irobot_create_msgs::action::DriveArc_SendGoal_Request> *>(untyped_member);
  return member->size();
}

const void * get_const_function__DriveArc_SendGoal_Event__request(const void * untyped_member, size_t index)
{
  const auto & member =
    *reinterpret_cast<const std::vector<irobot_create_msgs::action::DriveArc_SendGoal_Request> *>(untyped_member);
  return &member[index];
}

void * get_function__DriveArc_SendGoal_Event__request(void * untyped_member, size_t index)
{
  auto & member =
    *reinterpret_cast<std::vector<irobot_create_msgs::action::DriveArc_SendGoal_Request> *>(untyped_member);
  return &member[index];
}

void fetch_function__DriveArc_SendGoal_Event__request(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const auto & item = *reinterpret_cast<const irobot_create_msgs::action::DriveArc_SendGoal_Request *>(
    get_const_function__DriveArc_SendGoal_Event__request(untyped_member, index));
  auto & value = *reinterpret_cast<irobot_create_msgs::action::DriveArc_SendGoal_Request *>(untyped_value);
  value = item;
}

void assign_function__DriveArc_SendGoal_Event__request(
  void * untyped_member, size_t index, const void * untyped_value)
{
  auto & item = *reinterpret_cast<irobot_create_msgs::action::DriveArc_SendGoal_Request *>(
    get_function__DriveArc_SendGoal_Event__request(untyped_member, index));
  const auto & value = *reinterpret_cast<const irobot_create_msgs::action::DriveArc_SendGoal_Request *>(untyped_value);
  item = value;
}

void resize_function__DriveArc_SendGoal_Event__request(void * untyped_member, size_t size)
{
  auto * member =
    reinterpret_cast<std::vector<irobot_create_msgs::action::DriveArc_SendGoal_Request> *>(untyped_member);
  member->resize(size);
}

size_t size_function__DriveArc_SendGoal_Event__response(const void * untyped_member)
{
  const auto * member = reinterpret_cast<const std::vector<irobot_create_msgs::action::DriveArc_SendGoal_Response> *>(untyped_member);
  return member->size();
}

const void * get_const_function__DriveArc_SendGoal_Event__response(const void * untyped_member, size_t index)
{
  const auto & member =
    *reinterpret_cast<const std::vector<irobot_create_msgs::action::DriveArc_SendGoal_Response> *>(untyped_member);
  return &member[index];
}

void * get_function__DriveArc_SendGoal_Event__response(void * untyped_member, size_t index)
{
  auto & member =
    *reinterpret_cast<std::vector<irobot_create_msgs::action::DriveArc_SendGoal_Response> *>(untyped_member);
  return &member[index];
}

void fetch_function__DriveArc_SendGoal_Event__response(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const auto & item = *reinterpret_cast<const irobot_create_msgs::action::DriveArc_SendGoal_Response *>(
    get_const_function__DriveArc_SendGoal_Event__response(untyped_member, index));
  auto & value = *reinterpret_cast<irobot_create_msgs::action::DriveArc_SendGoal_Response *>(untyped_value);
  value = item;
}

void assign_function__DriveArc_SendGoal_Event__response(
  void * untyped_member, size_t index, const void * untyped_value)
{
  auto & item = *reinterpret_cast<irobot_create_msgs::action::DriveArc_SendGoal_Response *>(
    get_function__DriveArc_SendGoal_Event__response(untyped_member, index));
  const auto & value = *reinterpret_cast<const irobot_create_msgs::action::DriveArc_SendGoal_Response *>(untyped_value);
  item = value;
}

void resize_function__DriveArc_SendGoal_Event__response(void * untyped_member, size_t size)
{
  auto * member =
    reinterpret_cast<std::vector<irobot_create_msgs::action::DriveArc_SendGoal_Response> *>(untyped_member);
  member->resize(size);
}

static const ::rosidl_typesupport_introspection_cpp::MessageMember DriveArc_SendGoal_Event_message_member_array[3] = {
  {
    "info",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<service_msgs::msg::ServiceEventInfo>(),  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_SendGoal_Event, info),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  },
  {
    "request",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<irobot_create_msgs::action::DriveArc_SendGoal_Request>(),  // members of sub message
    false,  // is key
    true,  // is array
    1,  // array size
    true,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_SendGoal_Event, request),  // bytes offset in struct
    nullptr,  // default value
    size_function__DriveArc_SendGoal_Event__request,  // size() function pointer
    get_const_function__DriveArc_SendGoal_Event__request,  // get_const(index) function pointer
    get_function__DriveArc_SendGoal_Event__request,  // get(index) function pointer
    fetch_function__DriveArc_SendGoal_Event__request,  // fetch(index, &value) function pointer
    assign_function__DriveArc_SendGoal_Event__request,  // assign(index, value) function pointer
    resize_function__DriveArc_SendGoal_Event__request,  // resize(index) function pointer
    false  // is_rosidl_buffer
  },
  {
    "response",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<irobot_create_msgs::action::DriveArc_SendGoal_Response>(),  // members of sub message
    false,  // is key
    true,  // is array
    1,  // array size
    true,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_SendGoal_Event, response),  // bytes offset in struct
    nullptr,  // default value
    size_function__DriveArc_SendGoal_Event__response,  // size() function pointer
    get_const_function__DriveArc_SendGoal_Event__response,  // get_const(index) function pointer
    get_function__DriveArc_SendGoal_Event__response,  // get(index) function pointer
    fetch_function__DriveArc_SendGoal_Event__response,  // fetch(index, &value) function pointer
    assign_function__DriveArc_SendGoal_Event__response,  // assign(index, value) function pointer
    resize_function__DriveArc_SendGoal_Event__response,  // resize(index) function pointer
    false  // is_rosidl_buffer
  }
};

static const ::rosidl_typesupport_introspection_cpp::MessageMembers DriveArc_SendGoal_Event_message_members = {
  "irobot_create_msgs::action",  // message namespace
  "DriveArc_SendGoal_Event",  // message name
  3,  // number of fields
  sizeof(irobot_create_msgs::action::DriveArc_SendGoal_Event),
  false,  // has_any_key_member_
  DriveArc_SendGoal_Event_message_member_array,  // message members
  DriveArc_SendGoal_Event_init_function,  // function to initialize message memory (memory has to be allocated)
  DriveArc_SendGoal_Event_fini_function  // function to terminate message instance (will not free memory)
};

static const rosidl_message_type_support_t DriveArc_SendGoal_Event_message_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &DriveArc_SendGoal_Event_message_members,
  get_message_typesupport_handle_function,
  &irobot_create_msgs__action__DriveArc_SendGoal_Event__get_type_hash,
  &irobot_create_msgs__action__DriveArc_SendGoal_Event__get_type_description,
  &irobot_create_msgs__action__DriveArc_SendGoal_Event__get_type_description_sources,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace action

}  // namespace irobot_create_msgs


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<irobot_create_msgs::action::DriveArc_SendGoal_Event>()
{
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_SendGoal_Event_message_type_support_handle;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, irobot_create_msgs, action, DriveArc_SendGoal_Event)() {
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_SendGoal_Event_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
#include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_interface/macros.h"
// already included above
// #include "rosidl_typesupport_introspection_cpp/visibility_control.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__functions.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__struct.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
#include "rosidl_typesupport_introspection_cpp/service_introspection.hpp"
#include "rosidl_typesupport_introspection_cpp/service_type_support_decl.hpp"

namespace irobot_create_msgs
{

namespace action
{

namespace rosidl_typesupport_introspection_cpp
{

// this is intentionally not const to allow initialization later to prevent an initialization race
static ::rosidl_typesupport_introspection_cpp::ServiceMembers DriveArc_SendGoal_service_members = {
  "irobot_create_msgs::action",  // service namespace
  "DriveArc_SendGoal",  // service name
  // the following fields are initialized below on first access
  // see get_service_type_support_handle<irobot_create_msgs::action::DriveArc_SendGoal>()
  nullptr,  // request message
  nullptr,  // response message
  nullptr,  // event message
};

static const rosidl_service_type_support_t DriveArc_SendGoal_service_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &DriveArc_SendGoal_service_members,
  get_service_typesupport_handle_function,
  ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<irobot_create_msgs::action::DriveArc_SendGoal_Request>(),
  ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<irobot_create_msgs::action::DriveArc_SendGoal_Response>(),
  ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<irobot_create_msgs::action::DriveArc_SendGoal_Event>(),
  &::rosidl_typesupport_cpp::service_create_event_message<irobot_create_msgs::action::DriveArc_SendGoal>,
  &::rosidl_typesupport_cpp::service_destroy_event_message<irobot_create_msgs::action::DriveArc_SendGoal>,
  &irobot_create_msgs__action__DriveArc_SendGoal__get_type_hash,
  &irobot_create_msgs__action__DriveArc_SendGoal__get_type_description,
  &irobot_create_msgs__action__DriveArc_SendGoal__get_type_description_sources,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace action

}  // namespace irobot_create_msgs


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_service_type_support_t *
get_service_type_support_handle<irobot_create_msgs::action::DriveArc_SendGoal>()
{
  // get a handle to the value to be returned
  auto service_type_support =
    &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_SendGoal_service_type_support_handle;
  // get a non-const and properly typed version of the data void *
  auto service_members = const_cast<::rosidl_typesupport_introspection_cpp::ServiceMembers *>(
    static_cast<const ::rosidl_typesupport_introspection_cpp::ServiceMembers *>(
      service_type_support->data));
  // make sure all of the service_members are initialized
  // if they are not, initialize them
  if (
    service_members->request_members_ == nullptr ||
    service_members->response_members_ == nullptr ||
    service_members->event_members_ == nullptr)
  {
    // initialize the request_members_ with the static function from the external library
    service_members->request_members_ = static_cast<
      const ::rosidl_typesupport_introspection_cpp::MessageMembers *
      >(
      ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<
        ::irobot_create_msgs::action::DriveArc_SendGoal_Request
      >()->data
      );
    // initialize the response_members_ with the static function from the external library
    service_members->response_members_ = static_cast<
      const ::rosidl_typesupport_introspection_cpp::MessageMembers *
      >(
      ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<
        ::irobot_create_msgs::action::DriveArc_SendGoal_Response
      >()->data
      );
    // initialize the event_members_ with the static function from the external library
    service_members->event_members_ = static_cast<
      const ::rosidl_typesupport_introspection_cpp::MessageMembers *
      >(
      ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<
        ::irobot_create_msgs::action::DriveArc_SendGoal_Event
      >()->data
      );
  }
  // finally return the properly initialized service_type_support handle
  return service_type_support;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, irobot_create_msgs, action, DriveArc_SendGoal)() {
  return ::rosidl_typesupport_introspection_cpp::get_service_type_support_handle<irobot_create_msgs::action::DriveArc_SendGoal>();
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "array"
// already included above
// #include "cstddef"
// already included above
// #include "string"
// already included above
// #include "vector"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_interface/macros.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__functions.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__struct.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/field_types.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_introspection.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/visibility_control.h"

namespace irobot_create_msgs
{

namespace action
{

namespace rosidl_typesupport_introspection_cpp
{

void DriveArc_GetResult_Request_init_function(
  void * message_memory, rosidl_runtime_cpp::MessageInitialization _init)
{
  new (message_memory) irobot_create_msgs::action::DriveArc_GetResult_Request(_init);
}

void DriveArc_GetResult_Request_fini_function(void * message_memory)
{
  auto typed_message = static_cast<irobot_create_msgs::action::DriveArc_GetResult_Request *>(message_memory);
  typed_message->~DriveArc_GetResult_Request();
}

static const ::rosidl_typesupport_introspection_cpp::MessageMember DriveArc_GetResult_Request_message_member_array[1] = {
  {
    "goal_id",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<unique_identifier_msgs::msg::UUID>(),  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_GetResult_Request, goal_id),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  }
};

static const ::rosidl_typesupport_introspection_cpp::MessageMembers DriveArc_GetResult_Request_message_members = {
  "irobot_create_msgs::action",  // message namespace
  "DriveArc_GetResult_Request",  // message name
  1,  // number of fields
  sizeof(irobot_create_msgs::action::DriveArc_GetResult_Request),
  false,  // has_any_key_member_
  DriveArc_GetResult_Request_message_member_array,  // message members
  DriveArc_GetResult_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  DriveArc_GetResult_Request_fini_function  // function to terminate message instance (will not free memory)
};

static const rosidl_message_type_support_t DriveArc_GetResult_Request_message_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &DriveArc_GetResult_Request_message_members,
  get_message_typesupport_handle_function,
  &irobot_create_msgs__action__DriveArc_GetResult_Request__get_type_hash,
  &irobot_create_msgs__action__DriveArc_GetResult_Request__get_type_description,
  &irobot_create_msgs__action__DriveArc_GetResult_Request__get_type_description_sources,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace action

}  // namespace irobot_create_msgs


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<irobot_create_msgs::action::DriveArc_GetResult_Request>()
{
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_GetResult_Request_message_type_support_handle;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, irobot_create_msgs, action, DriveArc_GetResult_Request)() {
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_GetResult_Request_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "array"
// already included above
// #include "cstddef"
// already included above
// #include "string"
// already included above
// #include "vector"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_interface/macros.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__functions.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__struct.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/field_types.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_introspection.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/visibility_control.h"

namespace irobot_create_msgs
{

namespace action
{

namespace rosidl_typesupport_introspection_cpp
{

void DriveArc_GetResult_Response_init_function(
  void * message_memory, rosidl_runtime_cpp::MessageInitialization _init)
{
  new (message_memory) irobot_create_msgs::action::DriveArc_GetResult_Response(_init);
}

void DriveArc_GetResult_Response_fini_function(void * message_memory)
{
  auto typed_message = static_cast<irobot_create_msgs::action::DriveArc_GetResult_Response *>(message_memory);
  typed_message->~DriveArc_GetResult_Response();
}

static const ::rosidl_typesupport_introspection_cpp::MessageMember DriveArc_GetResult_Response_message_member_array[2] = {
  {
    "status",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_INT8,  // type
    0,  // upper bound of string
    nullptr,  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_GetResult_Response, status),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  },
  {
    "result",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<irobot_create_msgs::action::DriveArc_Result>(),  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_GetResult_Response, result),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  }
};

static const ::rosidl_typesupport_introspection_cpp::MessageMembers DriveArc_GetResult_Response_message_members = {
  "irobot_create_msgs::action",  // message namespace
  "DriveArc_GetResult_Response",  // message name
  2,  // number of fields
  sizeof(irobot_create_msgs::action::DriveArc_GetResult_Response),
  false,  // has_any_key_member_
  DriveArc_GetResult_Response_message_member_array,  // message members
  DriveArc_GetResult_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  DriveArc_GetResult_Response_fini_function  // function to terminate message instance (will not free memory)
};

static const rosidl_message_type_support_t DriveArc_GetResult_Response_message_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &DriveArc_GetResult_Response_message_members,
  get_message_typesupport_handle_function,
  &irobot_create_msgs__action__DriveArc_GetResult_Response__get_type_hash,
  &irobot_create_msgs__action__DriveArc_GetResult_Response__get_type_description,
  &irobot_create_msgs__action__DriveArc_GetResult_Response__get_type_description_sources,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace action

}  // namespace irobot_create_msgs


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<irobot_create_msgs::action::DriveArc_GetResult_Response>()
{
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_GetResult_Response_message_type_support_handle;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, irobot_create_msgs, action, DriveArc_GetResult_Response)() {
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_GetResult_Response_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "array"
// already included above
// #include "cstddef"
// already included above
// #include "string"
// already included above
// #include "vector"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_interface/macros.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__functions.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__struct.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/field_types.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_introspection.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/visibility_control.h"

namespace irobot_create_msgs
{

namespace action
{

namespace rosidl_typesupport_introspection_cpp
{

void DriveArc_GetResult_Event_init_function(
  void * message_memory, rosidl_runtime_cpp::MessageInitialization _init)
{
  new (message_memory) irobot_create_msgs::action::DriveArc_GetResult_Event(_init);
}

void DriveArc_GetResult_Event_fini_function(void * message_memory)
{
  auto typed_message = static_cast<irobot_create_msgs::action::DriveArc_GetResult_Event *>(message_memory);
  typed_message->~DriveArc_GetResult_Event();
}

size_t size_function__DriveArc_GetResult_Event__request(const void * untyped_member)
{
  const auto * member = reinterpret_cast<const std::vector<irobot_create_msgs::action::DriveArc_GetResult_Request> *>(untyped_member);
  return member->size();
}

const void * get_const_function__DriveArc_GetResult_Event__request(const void * untyped_member, size_t index)
{
  const auto & member =
    *reinterpret_cast<const std::vector<irobot_create_msgs::action::DriveArc_GetResult_Request> *>(untyped_member);
  return &member[index];
}

void * get_function__DriveArc_GetResult_Event__request(void * untyped_member, size_t index)
{
  auto & member =
    *reinterpret_cast<std::vector<irobot_create_msgs::action::DriveArc_GetResult_Request> *>(untyped_member);
  return &member[index];
}

void fetch_function__DriveArc_GetResult_Event__request(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const auto & item = *reinterpret_cast<const irobot_create_msgs::action::DriveArc_GetResult_Request *>(
    get_const_function__DriveArc_GetResult_Event__request(untyped_member, index));
  auto & value = *reinterpret_cast<irobot_create_msgs::action::DriveArc_GetResult_Request *>(untyped_value);
  value = item;
}

void assign_function__DriveArc_GetResult_Event__request(
  void * untyped_member, size_t index, const void * untyped_value)
{
  auto & item = *reinterpret_cast<irobot_create_msgs::action::DriveArc_GetResult_Request *>(
    get_function__DriveArc_GetResult_Event__request(untyped_member, index));
  const auto & value = *reinterpret_cast<const irobot_create_msgs::action::DriveArc_GetResult_Request *>(untyped_value);
  item = value;
}

void resize_function__DriveArc_GetResult_Event__request(void * untyped_member, size_t size)
{
  auto * member =
    reinterpret_cast<std::vector<irobot_create_msgs::action::DriveArc_GetResult_Request> *>(untyped_member);
  member->resize(size);
}

size_t size_function__DriveArc_GetResult_Event__response(const void * untyped_member)
{
  const auto * member = reinterpret_cast<const std::vector<irobot_create_msgs::action::DriveArc_GetResult_Response> *>(untyped_member);
  return member->size();
}

const void * get_const_function__DriveArc_GetResult_Event__response(const void * untyped_member, size_t index)
{
  const auto & member =
    *reinterpret_cast<const std::vector<irobot_create_msgs::action::DriveArc_GetResult_Response> *>(untyped_member);
  return &member[index];
}

void * get_function__DriveArc_GetResult_Event__response(void * untyped_member, size_t index)
{
  auto & member =
    *reinterpret_cast<std::vector<irobot_create_msgs::action::DriveArc_GetResult_Response> *>(untyped_member);
  return &member[index];
}

void fetch_function__DriveArc_GetResult_Event__response(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const auto & item = *reinterpret_cast<const irobot_create_msgs::action::DriveArc_GetResult_Response *>(
    get_const_function__DriveArc_GetResult_Event__response(untyped_member, index));
  auto & value = *reinterpret_cast<irobot_create_msgs::action::DriveArc_GetResult_Response *>(untyped_value);
  value = item;
}

void assign_function__DriveArc_GetResult_Event__response(
  void * untyped_member, size_t index, const void * untyped_value)
{
  auto & item = *reinterpret_cast<irobot_create_msgs::action::DriveArc_GetResult_Response *>(
    get_function__DriveArc_GetResult_Event__response(untyped_member, index));
  const auto & value = *reinterpret_cast<const irobot_create_msgs::action::DriveArc_GetResult_Response *>(untyped_value);
  item = value;
}

void resize_function__DriveArc_GetResult_Event__response(void * untyped_member, size_t size)
{
  auto * member =
    reinterpret_cast<std::vector<irobot_create_msgs::action::DriveArc_GetResult_Response> *>(untyped_member);
  member->resize(size);
}

static const ::rosidl_typesupport_introspection_cpp::MessageMember DriveArc_GetResult_Event_message_member_array[3] = {
  {
    "info",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<service_msgs::msg::ServiceEventInfo>(),  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_GetResult_Event, info),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  },
  {
    "request",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<irobot_create_msgs::action::DriveArc_GetResult_Request>(),  // members of sub message
    false,  // is key
    true,  // is array
    1,  // array size
    true,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_GetResult_Event, request),  // bytes offset in struct
    nullptr,  // default value
    size_function__DriveArc_GetResult_Event__request,  // size() function pointer
    get_const_function__DriveArc_GetResult_Event__request,  // get_const(index) function pointer
    get_function__DriveArc_GetResult_Event__request,  // get(index) function pointer
    fetch_function__DriveArc_GetResult_Event__request,  // fetch(index, &value) function pointer
    assign_function__DriveArc_GetResult_Event__request,  // assign(index, value) function pointer
    resize_function__DriveArc_GetResult_Event__request,  // resize(index) function pointer
    false  // is_rosidl_buffer
  },
  {
    "response",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<irobot_create_msgs::action::DriveArc_GetResult_Response>(),  // members of sub message
    false,  // is key
    true,  // is array
    1,  // array size
    true,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_GetResult_Event, response),  // bytes offset in struct
    nullptr,  // default value
    size_function__DriveArc_GetResult_Event__response,  // size() function pointer
    get_const_function__DriveArc_GetResult_Event__response,  // get_const(index) function pointer
    get_function__DriveArc_GetResult_Event__response,  // get(index) function pointer
    fetch_function__DriveArc_GetResult_Event__response,  // fetch(index, &value) function pointer
    assign_function__DriveArc_GetResult_Event__response,  // assign(index, value) function pointer
    resize_function__DriveArc_GetResult_Event__response,  // resize(index) function pointer
    false  // is_rosidl_buffer
  }
};

static const ::rosidl_typesupport_introspection_cpp::MessageMembers DriveArc_GetResult_Event_message_members = {
  "irobot_create_msgs::action",  // message namespace
  "DriveArc_GetResult_Event",  // message name
  3,  // number of fields
  sizeof(irobot_create_msgs::action::DriveArc_GetResult_Event),
  false,  // has_any_key_member_
  DriveArc_GetResult_Event_message_member_array,  // message members
  DriveArc_GetResult_Event_init_function,  // function to initialize message memory (memory has to be allocated)
  DriveArc_GetResult_Event_fini_function  // function to terminate message instance (will not free memory)
};

static const rosidl_message_type_support_t DriveArc_GetResult_Event_message_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &DriveArc_GetResult_Event_message_members,
  get_message_typesupport_handle_function,
  &irobot_create_msgs__action__DriveArc_GetResult_Event__get_type_hash,
  &irobot_create_msgs__action__DriveArc_GetResult_Event__get_type_description,
  &irobot_create_msgs__action__DriveArc_GetResult_Event__get_type_description_sources,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace action

}  // namespace irobot_create_msgs


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<irobot_create_msgs::action::DriveArc_GetResult_Event>()
{
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_GetResult_Event_message_type_support_handle;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, irobot_create_msgs, action, DriveArc_GetResult_Event)() {
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_GetResult_Event_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_interface/macros.h"
// already included above
// #include "rosidl_typesupport_introspection_cpp/visibility_control.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__functions.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__struct.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/service_introspection.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/service_type_support_decl.hpp"

namespace irobot_create_msgs
{

namespace action
{

namespace rosidl_typesupport_introspection_cpp
{

// this is intentionally not const to allow initialization later to prevent an initialization race
static ::rosidl_typesupport_introspection_cpp::ServiceMembers DriveArc_GetResult_service_members = {
  "irobot_create_msgs::action",  // service namespace
  "DriveArc_GetResult",  // service name
  // the following fields are initialized below on first access
  // see get_service_type_support_handle<irobot_create_msgs::action::DriveArc_GetResult>()
  nullptr,  // request message
  nullptr,  // response message
  nullptr,  // event message
};

static const rosidl_service_type_support_t DriveArc_GetResult_service_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &DriveArc_GetResult_service_members,
  get_service_typesupport_handle_function,
  ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<irobot_create_msgs::action::DriveArc_GetResult_Request>(),
  ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<irobot_create_msgs::action::DriveArc_GetResult_Response>(),
  ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<irobot_create_msgs::action::DriveArc_GetResult_Event>(),
  &::rosidl_typesupport_cpp::service_create_event_message<irobot_create_msgs::action::DriveArc_GetResult>,
  &::rosidl_typesupport_cpp::service_destroy_event_message<irobot_create_msgs::action::DriveArc_GetResult>,
  &irobot_create_msgs__action__DriveArc_GetResult__get_type_hash,
  &irobot_create_msgs__action__DriveArc_GetResult__get_type_description,
  &irobot_create_msgs__action__DriveArc_GetResult__get_type_description_sources,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace action

}  // namespace irobot_create_msgs


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_service_type_support_t *
get_service_type_support_handle<irobot_create_msgs::action::DriveArc_GetResult>()
{
  // get a handle to the value to be returned
  auto service_type_support =
    &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_GetResult_service_type_support_handle;
  // get a non-const and properly typed version of the data void *
  auto service_members = const_cast<::rosidl_typesupport_introspection_cpp::ServiceMembers *>(
    static_cast<const ::rosidl_typesupport_introspection_cpp::ServiceMembers *>(
      service_type_support->data));
  // make sure all of the service_members are initialized
  // if they are not, initialize them
  if (
    service_members->request_members_ == nullptr ||
    service_members->response_members_ == nullptr ||
    service_members->event_members_ == nullptr)
  {
    // initialize the request_members_ with the static function from the external library
    service_members->request_members_ = static_cast<
      const ::rosidl_typesupport_introspection_cpp::MessageMembers *
      >(
      ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<
        ::irobot_create_msgs::action::DriveArc_GetResult_Request
      >()->data
      );
    // initialize the response_members_ with the static function from the external library
    service_members->response_members_ = static_cast<
      const ::rosidl_typesupport_introspection_cpp::MessageMembers *
      >(
      ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<
        ::irobot_create_msgs::action::DriveArc_GetResult_Response
      >()->data
      );
    // initialize the event_members_ with the static function from the external library
    service_members->event_members_ = static_cast<
      const ::rosidl_typesupport_introspection_cpp::MessageMembers *
      >(
      ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<
        ::irobot_create_msgs::action::DriveArc_GetResult_Event
      >()->data
      );
  }
  // finally return the properly initialized service_type_support handle
  return service_type_support;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, irobot_create_msgs, action, DriveArc_GetResult)() {
  return ::rosidl_typesupport_introspection_cpp::get_service_type_support_handle<irobot_create_msgs::action::DriveArc_GetResult>();
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "array"
// already included above
// #include "cstddef"
// already included above
// #include "string"
// already included above
// #include "vector"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_interface/macros.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__functions.h"
// already included above
// #include "irobot_create_msgs/action/detail/drive_arc__struct.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/field_types.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_introspection.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/message_type_support_decl.hpp"
// already included above
// #include "rosidl_typesupport_introspection_cpp/visibility_control.h"

namespace irobot_create_msgs
{

namespace action
{

namespace rosidl_typesupport_introspection_cpp
{

void DriveArc_FeedbackMessage_init_function(
  void * message_memory, rosidl_runtime_cpp::MessageInitialization _init)
{
  new (message_memory) irobot_create_msgs::action::DriveArc_FeedbackMessage(_init);
}

void DriveArc_FeedbackMessage_fini_function(void * message_memory)
{
  auto typed_message = static_cast<irobot_create_msgs::action::DriveArc_FeedbackMessage *>(message_memory);
  typed_message->~DriveArc_FeedbackMessage();
}

static const ::rosidl_typesupport_introspection_cpp::MessageMember DriveArc_FeedbackMessage_message_member_array[2] = {
  {
    "goal_id",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<unique_identifier_msgs::msg::UUID>(),  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_FeedbackMessage, goal_id),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  },
  {
    "feedback",  // name
    ::rosidl_typesupport_introspection_cpp::ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    ::rosidl_typesupport_introspection_cpp::get_message_type_support_handle<irobot_create_msgs::action::DriveArc_Feedback>(),  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs::action::DriveArc_FeedbackMessage, feedback),  // bytes offset in struct
    nullptr,  // default value
    nullptr,  // size() function pointer
    nullptr,  // get_const(index) function pointer
    nullptr,  // get(index) function pointer
    nullptr,  // fetch(index, &value) function pointer
    nullptr,  // assign(index, value) function pointer
    nullptr,  // resize(index) function pointer
    false  // is_rosidl_buffer
  }
};

static const ::rosidl_typesupport_introspection_cpp::MessageMembers DriveArc_FeedbackMessage_message_members = {
  "irobot_create_msgs::action",  // message namespace
  "DriveArc_FeedbackMessage",  // message name
  2,  // number of fields
  sizeof(irobot_create_msgs::action::DriveArc_FeedbackMessage),
  false,  // has_any_key_member_
  DriveArc_FeedbackMessage_message_member_array,  // message members
  DriveArc_FeedbackMessage_init_function,  // function to initialize message memory (memory has to be allocated)
  DriveArc_FeedbackMessage_fini_function  // function to terminate message instance (will not free memory)
};

static const rosidl_message_type_support_t DriveArc_FeedbackMessage_message_type_support_handle = {
  ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  &DriveArc_FeedbackMessage_message_members,
  get_message_typesupport_handle_function,
  &irobot_create_msgs__action__DriveArc_FeedbackMessage__get_type_hash,
  &irobot_create_msgs__action__DriveArc_FeedbackMessage__get_type_description,
  &irobot_create_msgs__action__DriveArc_FeedbackMessage__get_type_description_sources,
};

}  // namespace rosidl_typesupport_introspection_cpp

}  // namespace action

}  // namespace irobot_create_msgs


namespace rosidl_typesupport_introspection_cpp
{

template<>
ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<irobot_create_msgs::action::DriveArc_FeedbackMessage>()
{
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_FeedbackMessage_message_type_support_handle;
}

}  // namespace rosidl_typesupport_introspection_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_INTROSPECTION_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, irobot_create_msgs, action, DriveArc_FeedbackMessage)() {
  return &::irobot_create_msgs::action::rosidl_typesupport_introspection_cpp::DriveArc_FeedbackMessage_message_type_support_handle;
}

#ifdef __cplusplus
}
#endif
