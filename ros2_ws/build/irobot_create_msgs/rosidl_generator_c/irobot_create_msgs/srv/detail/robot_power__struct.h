// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from irobot_create_msgs:srv/RobotPower.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "irobot_create_msgs/srv/robot_power.h"


#ifndef IROBOT_CREATE_MSGS__SRV__DETAIL__ROBOT_POWER__STRUCT_H_
#define IROBOT_CREATE_MSGS__SRV__DETAIL__ROBOT_POWER__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in srv/RobotPower in the package irobot_create_msgs.
typedef struct irobot_create_msgs__srv__RobotPower_Request
{
  uint8_t structure_needs_at_least_one_member;
} irobot_create_msgs__srv__RobotPower_Request;

// Struct for a sequence of irobot_create_msgs__srv__RobotPower_Request.
typedef struct irobot_create_msgs__srv__RobotPower_Request__Sequence
{
  irobot_create_msgs__srv__RobotPower_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__srv__RobotPower_Request__Sequence;

// Constants defined in the message

// Include directives for member types
// Member 'message'
#include "rosidl_runtime_c/string.h"

/// Struct defined in srv/RobotPower in the package irobot_create_msgs.
typedef struct irobot_create_msgs__srv__RobotPower_Response
{
  /// indicate successful application of command, robot may power down before response is transmitted
  bool success;
  /// informational for result, robot may power down before response is transmitted
  rosidl_runtime_c__String message;
} irobot_create_msgs__srv__RobotPower_Response;

// Struct for a sequence of irobot_create_msgs__srv__RobotPower_Response.
typedef struct irobot_create_msgs__srv__RobotPower_Response__Sequence
{
  irobot_create_msgs__srv__RobotPower_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__srv__RobotPower_Response__Sequence;

// Constants defined in the message

// Include directives for member types
// Member 'info'
#include "service_msgs/msg/detail/service_event_info__struct.h"

// constants for array fields with an upper bound
// request
enum
{
  irobot_create_msgs__srv__RobotPower_Event__request__MAX_SIZE = 1
};
// response
enum
{
  irobot_create_msgs__srv__RobotPower_Event__response__MAX_SIZE = 1
};

/// Struct defined in srv/RobotPower in the package irobot_create_msgs.
typedef struct irobot_create_msgs__srv__RobotPower_Event
{
  service_msgs__msg__ServiceEventInfo info;
  irobot_create_msgs__srv__RobotPower_Request__Sequence request;
  irobot_create_msgs__srv__RobotPower_Response__Sequence response;
} irobot_create_msgs__srv__RobotPower_Event;

// Struct for a sequence of irobot_create_msgs__srv__RobotPower_Event.
typedef struct irobot_create_msgs__srv__RobotPower_Event__Sequence
{
  irobot_create_msgs__srv__RobotPower_Event * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__srv__RobotPower_Event__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // IROBOT_CREATE_MSGS__SRV__DETAIL__ROBOT_POWER__STRUCT_H_
