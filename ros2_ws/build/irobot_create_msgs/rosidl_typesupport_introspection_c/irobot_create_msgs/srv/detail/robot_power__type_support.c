// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from irobot_create_msgs:srv/RobotPower.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "irobot_create_msgs/srv/detail/robot_power__rosidl_typesupport_introspection_c.h"
#include "irobot_create_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "irobot_create_msgs/srv/detail/robot_power__functions.h"
#include "irobot_create_msgs/srv/detail/robot_power__struct.h"


#ifdef __cplusplus
extern "C"
{
#endif

void irobot_create_msgs__srv__RobotPower_Request__rosidl_typesupport_introspection_c__RobotPower_Request_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  irobot_create_msgs__srv__RobotPower_Request__init(message_memory);
}

void irobot_create_msgs__srv__RobotPower_Request__rosidl_typesupport_introspection_c__RobotPower_Request_fini_function(void * message_memory)
{
  irobot_create_msgs__srv__RobotPower_Request__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember irobot_create_msgs__srv__RobotPower_Request__rosidl_typesupport_introspection_c__RobotPower_Request_message_member_array[1] = {
  {
    "structure_needs_at_least_one_member",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_UINT8,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs__srv__RobotPower_Request, structure_needs_at_least_one_member),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL,  // resize(index) function pointer
    false  // is_rosidl_buffer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers irobot_create_msgs__srv__RobotPower_Request__rosidl_typesupport_introspection_c__RobotPower_Request_message_members = {
  "irobot_create_msgs__srv",  // message namespace
  "RobotPower_Request",  // message name
  1,  // number of fields
  sizeof(irobot_create_msgs__srv__RobotPower_Request),
  false,  // has_any_key_member_
  irobot_create_msgs__srv__RobotPower_Request__rosidl_typesupport_introspection_c__RobotPower_Request_message_member_array,  // message members
  irobot_create_msgs__srv__RobotPower_Request__rosidl_typesupport_introspection_c__RobotPower_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  irobot_create_msgs__srv__RobotPower_Request__rosidl_typesupport_introspection_c__RobotPower_Request_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t irobot_create_msgs__srv__RobotPower_Request__rosidl_typesupport_introspection_c__RobotPower_Request_message_type_support_handle = {
  0,
  &irobot_create_msgs__srv__RobotPower_Request__rosidl_typesupport_introspection_c__RobotPower_Request_message_members,
  get_message_typesupport_handle_function,
  &irobot_create_msgs__srv__RobotPower_Request__get_type_hash,
  &irobot_create_msgs__srv__RobotPower_Request__get_type_description,
  &irobot_create_msgs__srv__RobotPower_Request__get_type_description_sources,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_irobot_create_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, irobot_create_msgs, srv, RobotPower_Request)() {
  if (!irobot_create_msgs__srv__RobotPower_Request__rosidl_typesupport_introspection_c__RobotPower_Request_message_type_support_handle.typesupport_identifier) {
    irobot_create_msgs__srv__RobotPower_Request__rosidl_typesupport_introspection_c__RobotPower_Request_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &irobot_create_msgs__srv__RobotPower_Request__rosidl_typesupport_introspection_c__RobotPower_Request_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "irobot_create_msgs/srv/detail/robot_power__rosidl_typesupport_introspection_c.h"
// already included above
// #include "irobot_create_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "irobot_create_msgs/srv/detail/robot_power__functions.h"
// already included above
// #include "irobot_create_msgs/srv/detail/robot_power__struct.h"


// Include directives for member types
// Member `message`
#include "rosidl_runtime_c/string_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void irobot_create_msgs__srv__RobotPower_Response__rosidl_typesupport_introspection_c__RobotPower_Response_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  irobot_create_msgs__srv__RobotPower_Response__init(message_memory);
}

void irobot_create_msgs__srv__RobotPower_Response__rosidl_typesupport_introspection_c__RobotPower_Response_fini_function(void * message_memory)
{
  irobot_create_msgs__srv__RobotPower_Response__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember irobot_create_msgs__srv__RobotPower_Response__rosidl_typesupport_introspection_c__RobotPower_Response_message_member_array[2] = {
  {
    "success",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_BOOLEAN,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs__srv__RobotPower_Response, success),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL,  // resize(index) function pointer
    false  // is_rosidl_buffer
  },
  {
    "message",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs__srv__RobotPower_Response, message),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL,  // resize(index) function pointer
    false  // is_rosidl_buffer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers irobot_create_msgs__srv__RobotPower_Response__rosidl_typesupport_introspection_c__RobotPower_Response_message_members = {
  "irobot_create_msgs__srv",  // message namespace
  "RobotPower_Response",  // message name
  2,  // number of fields
  sizeof(irobot_create_msgs__srv__RobotPower_Response),
  false,  // has_any_key_member_
  irobot_create_msgs__srv__RobotPower_Response__rosidl_typesupport_introspection_c__RobotPower_Response_message_member_array,  // message members
  irobot_create_msgs__srv__RobotPower_Response__rosidl_typesupport_introspection_c__RobotPower_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  irobot_create_msgs__srv__RobotPower_Response__rosidl_typesupport_introspection_c__RobotPower_Response_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t irobot_create_msgs__srv__RobotPower_Response__rosidl_typesupport_introspection_c__RobotPower_Response_message_type_support_handle = {
  0,
  &irobot_create_msgs__srv__RobotPower_Response__rosidl_typesupport_introspection_c__RobotPower_Response_message_members,
  get_message_typesupport_handle_function,
  &irobot_create_msgs__srv__RobotPower_Response__get_type_hash,
  &irobot_create_msgs__srv__RobotPower_Response__get_type_description,
  &irobot_create_msgs__srv__RobotPower_Response__get_type_description_sources,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_irobot_create_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, irobot_create_msgs, srv, RobotPower_Response)() {
  if (!irobot_create_msgs__srv__RobotPower_Response__rosidl_typesupport_introspection_c__RobotPower_Response_message_type_support_handle.typesupport_identifier) {
    irobot_create_msgs__srv__RobotPower_Response__rosidl_typesupport_introspection_c__RobotPower_Response_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &irobot_create_msgs__srv__RobotPower_Response__rosidl_typesupport_introspection_c__RobotPower_Response_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "irobot_create_msgs/srv/detail/robot_power__rosidl_typesupport_introspection_c.h"
// already included above
// #include "irobot_create_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "irobot_create_msgs/srv/detail/robot_power__functions.h"
// already included above
// #include "irobot_create_msgs/srv/detail/robot_power__struct.h"


// Include directives for member types
// Member `info`
#include "service_msgs/msg/service_event_info.h"
// Member `info`
#include "service_msgs/msg/detail/service_event_info__rosidl_typesupport_introspection_c.h"
// Member `request`
// Member `response`
#include "irobot_create_msgs/srv/robot_power.h"
// Member `request`
// Member `response`
// already included above
// #include "irobot_create_msgs/srv/detail/robot_power__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  irobot_create_msgs__srv__RobotPower_Event__init(message_memory);
}

void irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_fini_function(void * message_memory)
{
  irobot_create_msgs__srv__RobotPower_Event__fini(message_memory);
}

size_t irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__size_function__RobotPower_Event__request(
  const void * untyped_member)
{
  const irobot_create_msgs__srv__RobotPower_Request__Sequence * member =
    (const irobot_create_msgs__srv__RobotPower_Request__Sequence *)(untyped_member);
  return member->size;
}

const void * irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__get_const_function__RobotPower_Event__request(
  const void * untyped_member, size_t index)
{
  const irobot_create_msgs__srv__RobotPower_Request__Sequence * member =
    (const irobot_create_msgs__srv__RobotPower_Request__Sequence *)(untyped_member);
  return &member->data[index];
}

void * irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__get_function__RobotPower_Event__request(
  void * untyped_member, size_t index)
{
  irobot_create_msgs__srv__RobotPower_Request__Sequence * member =
    (irobot_create_msgs__srv__RobotPower_Request__Sequence *)(untyped_member);
  return &member->data[index];
}

void irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__fetch_function__RobotPower_Event__request(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const irobot_create_msgs__srv__RobotPower_Request * item =
    ((const irobot_create_msgs__srv__RobotPower_Request *)
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__get_const_function__RobotPower_Event__request(untyped_member, index));
  irobot_create_msgs__srv__RobotPower_Request * value =
    (irobot_create_msgs__srv__RobotPower_Request *)(untyped_value);
  *value = *item;
}

void irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__assign_function__RobotPower_Event__request(
  void * untyped_member, size_t index, const void * untyped_value)
{
  irobot_create_msgs__srv__RobotPower_Request * item =
    ((irobot_create_msgs__srv__RobotPower_Request *)
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__get_function__RobotPower_Event__request(untyped_member, index));
  const irobot_create_msgs__srv__RobotPower_Request * value =
    (const irobot_create_msgs__srv__RobotPower_Request *)(untyped_value);
  *item = *value;
}

bool irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__resize_function__RobotPower_Event__request(
  void * untyped_member, size_t size)
{
  irobot_create_msgs__srv__RobotPower_Request__Sequence * member =
    (irobot_create_msgs__srv__RobotPower_Request__Sequence *)(untyped_member);
  irobot_create_msgs__srv__RobotPower_Request__Sequence__fini(member);
  return irobot_create_msgs__srv__RobotPower_Request__Sequence__init(member, size);
}

size_t irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__size_function__RobotPower_Event__response(
  const void * untyped_member)
{
  const irobot_create_msgs__srv__RobotPower_Response__Sequence * member =
    (const irobot_create_msgs__srv__RobotPower_Response__Sequence *)(untyped_member);
  return member->size;
}

const void * irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__get_const_function__RobotPower_Event__response(
  const void * untyped_member, size_t index)
{
  const irobot_create_msgs__srv__RobotPower_Response__Sequence * member =
    (const irobot_create_msgs__srv__RobotPower_Response__Sequence *)(untyped_member);
  return &member->data[index];
}

void * irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__get_function__RobotPower_Event__response(
  void * untyped_member, size_t index)
{
  irobot_create_msgs__srv__RobotPower_Response__Sequence * member =
    (irobot_create_msgs__srv__RobotPower_Response__Sequence *)(untyped_member);
  return &member->data[index];
}

void irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__fetch_function__RobotPower_Event__response(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const irobot_create_msgs__srv__RobotPower_Response * item =
    ((const irobot_create_msgs__srv__RobotPower_Response *)
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__get_const_function__RobotPower_Event__response(untyped_member, index));
  irobot_create_msgs__srv__RobotPower_Response * value =
    (irobot_create_msgs__srv__RobotPower_Response *)(untyped_value);
  *value = *item;
}

void irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__assign_function__RobotPower_Event__response(
  void * untyped_member, size_t index, const void * untyped_value)
{
  irobot_create_msgs__srv__RobotPower_Response * item =
    ((irobot_create_msgs__srv__RobotPower_Response *)
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__get_function__RobotPower_Event__response(untyped_member, index));
  const irobot_create_msgs__srv__RobotPower_Response * value =
    (const irobot_create_msgs__srv__RobotPower_Response *)(untyped_value);
  *item = *value;
}

bool irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__resize_function__RobotPower_Event__response(
  void * untyped_member, size_t size)
{
  irobot_create_msgs__srv__RobotPower_Response__Sequence * member =
    (irobot_create_msgs__srv__RobotPower_Response__Sequence *)(untyped_member);
  irobot_create_msgs__srv__RobotPower_Response__Sequence__fini(member);
  return irobot_create_msgs__srv__RobotPower_Response__Sequence__init(member, size);
}

static rosidl_typesupport_introspection_c__MessageMember irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_message_member_array[3] = {
  {
    "info",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(irobot_create_msgs__srv__RobotPower_Event, info),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL,  // resize(index) function pointer
    false  // is_rosidl_buffer
  },
  {
    "request",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is key
    true,  // is array
    1,  // array size
    true,  // is upper bound
    offsetof(irobot_create_msgs__srv__RobotPower_Event, request),  // bytes offset in struct
    NULL,  // default value
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__size_function__RobotPower_Event__request,  // size() function pointer
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__get_const_function__RobotPower_Event__request,  // get_const(index) function pointer
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__get_function__RobotPower_Event__request,  // get(index) function pointer
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__fetch_function__RobotPower_Event__request,  // fetch(index, &value) function pointer
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__assign_function__RobotPower_Event__request,  // assign(index, value) function pointer
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__resize_function__RobotPower_Event__request,  // resize(index) function pointer
    false  // is_rosidl_buffer
  },
  {
    "response",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is key
    true,  // is array
    1,  // array size
    true,  // is upper bound
    offsetof(irobot_create_msgs__srv__RobotPower_Event, response),  // bytes offset in struct
    NULL,  // default value
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__size_function__RobotPower_Event__response,  // size() function pointer
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__get_const_function__RobotPower_Event__response,  // get_const(index) function pointer
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__get_function__RobotPower_Event__response,  // get(index) function pointer
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__fetch_function__RobotPower_Event__response,  // fetch(index, &value) function pointer
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__assign_function__RobotPower_Event__response,  // assign(index, value) function pointer
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__resize_function__RobotPower_Event__response,  // resize(index) function pointer
    false  // is_rosidl_buffer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_message_members = {
  "irobot_create_msgs__srv",  // message namespace
  "RobotPower_Event",  // message name
  3,  // number of fields
  sizeof(irobot_create_msgs__srv__RobotPower_Event),
  false,  // has_any_key_member_
  irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_message_member_array,  // message members
  irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_init_function,  // function to initialize message memory (memory has to be allocated)
  irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_message_type_support_handle = {
  0,
  &irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_message_members,
  get_message_typesupport_handle_function,
  &irobot_create_msgs__srv__RobotPower_Event__get_type_hash,
  &irobot_create_msgs__srv__RobotPower_Event__get_type_description,
  &irobot_create_msgs__srv__RobotPower_Event__get_type_description_sources,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_irobot_create_msgs
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, irobot_create_msgs, srv, RobotPower_Event)() {
  irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, service_msgs, msg, ServiceEventInfo)();
  irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, irobot_create_msgs, srv, RobotPower_Request)();
  irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_message_member_array[2].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, irobot_create_msgs, srv, RobotPower_Response)();
  if (!irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_message_type_support_handle.typesupport_identifier) {
    irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "irobot_create_msgs/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "irobot_create_msgs/srv/detail/robot_power__rosidl_typesupport_introspection_c.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/service_introspection.h"

// this is intentionally not const to allow initialization later to prevent an initialization race
static rosidl_typesupport_introspection_c__ServiceMembers irobot_create_msgs__srv__detail__robot_power__rosidl_typesupport_introspection_c__RobotPower_service_members = {
  "irobot_create_msgs__srv",  // service namespace
  "RobotPower",  // service name
  // the following fields are initialized below on first access
  NULL,  // request message
  // irobot_create_msgs__srv__detail__robot_power__rosidl_typesupport_introspection_c__RobotPower_Request_message_type_support_handle,
  NULL,  // response message
  // irobot_create_msgs__srv__detail__robot_power__rosidl_typesupport_introspection_c__RobotPower_Response_message_type_support_handle
  NULL  // event_message
  // irobot_create_msgs__srv__detail__robot_power__rosidl_typesupport_introspection_c__RobotPower_Response_message_type_support_handle
};


static rosidl_service_type_support_t irobot_create_msgs__srv__detail__robot_power__rosidl_typesupport_introspection_c__RobotPower_service_type_support_handle = {
  0,
  &irobot_create_msgs__srv__detail__robot_power__rosidl_typesupport_introspection_c__RobotPower_service_members,
  get_service_typesupport_handle_function,
  &irobot_create_msgs__srv__RobotPower_Request__rosidl_typesupport_introspection_c__RobotPower_Request_message_type_support_handle,
  &irobot_create_msgs__srv__RobotPower_Response__rosidl_typesupport_introspection_c__RobotPower_Response_message_type_support_handle,
  &irobot_create_msgs__srv__RobotPower_Event__rosidl_typesupport_introspection_c__RobotPower_Event_message_type_support_handle,
  ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_CREATE_EVENT_MESSAGE_SYMBOL_NAME(
    rosidl_typesupport_c,
    irobot_create_msgs,
    srv,
    RobotPower
  ),
  ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_DESTROY_EVENT_MESSAGE_SYMBOL_NAME(
    rosidl_typesupport_c,
    irobot_create_msgs,
    srv,
    RobotPower
  ),
  &irobot_create_msgs__srv__RobotPower__get_type_hash,
  &irobot_create_msgs__srv__RobotPower__get_type_description,
  &irobot_create_msgs__srv__RobotPower__get_type_description_sources,
};

// Forward declaration of message type support functions for service members
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, irobot_create_msgs, srv, RobotPower_Request)(void);

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, irobot_create_msgs, srv, RobotPower_Response)(void);

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, irobot_create_msgs, srv, RobotPower_Event)(void);

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_irobot_create_msgs
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, irobot_create_msgs, srv, RobotPower)(void) {
  if (!irobot_create_msgs__srv__detail__robot_power__rosidl_typesupport_introspection_c__RobotPower_service_type_support_handle.typesupport_identifier) {
    irobot_create_msgs__srv__detail__robot_power__rosidl_typesupport_introspection_c__RobotPower_service_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  rosidl_typesupport_introspection_c__ServiceMembers * service_members =
    (rosidl_typesupport_introspection_c__ServiceMembers *)irobot_create_msgs__srv__detail__robot_power__rosidl_typesupport_introspection_c__RobotPower_service_type_support_handle.data;

  if (!service_members->request_members_) {
    service_members->request_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, irobot_create_msgs, srv, RobotPower_Request)()->data;
  }
  if (!service_members->response_members_) {
    service_members->response_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, irobot_create_msgs, srv, RobotPower_Response)()->data;
  }
  if (!service_members->event_members_) {
    service_members->event_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, irobot_create_msgs, srv, RobotPower_Event)()->data;
  }

  return &irobot_create_msgs__srv__detail__robot_power__rosidl_typesupport_introspection_c__RobotPower_service_type_support_handle;
}
