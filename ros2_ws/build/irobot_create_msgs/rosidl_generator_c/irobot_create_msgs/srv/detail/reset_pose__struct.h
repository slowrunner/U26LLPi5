// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from irobot_create_msgs:srv/ResetPose.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "irobot_create_msgs/srv/reset_pose.h"


#ifndef IROBOT_CREATE_MSGS__SRV__DETAIL__RESET_POSE__STRUCT_H_
#define IROBOT_CREATE_MSGS__SRV__DETAIL__RESET_POSE__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

// Include directives for member types
// Member 'pose'
#include "geometry_msgs/msg/detail/pose__struct.h"

/// Struct defined in srv/ResetPose in the package irobot_create_msgs.
typedef struct irobot_create_msgs__srv__ResetPose_Request
{
  geometry_msgs__msg__Pose pose;
} irobot_create_msgs__srv__ResetPose_Request;

// Struct for a sequence of irobot_create_msgs__srv__ResetPose_Request.
typedef struct irobot_create_msgs__srv__ResetPose_Request__Sequence
{
  irobot_create_msgs__srv__ResetPose_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__srv__ResetPose_Request__Sequence;

// Constants defined in the message

/// Struct defined in srv/ResetPose in the package irobot_create_msgs.
typedef struct irobot_create_msgs__srv__ResetPose_Response
{
  uint8_t structure_needs_at_least_one_member;
} irobot_create_msgs__srv__ResetPose_Response;

// Struct for a sequence of irobot_create_msgs__srv__ResetPose_Response.
typedef struct irobot_create_msgs__srv__ResetPose_Response__Sequence
{
  irobot_create_msgs__srv__ResetPose_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__srv__ResetPose_Response__Sequence;

// Constants defined in the message

// Include directives for member types
// Member 'info'
#include "service_msgs/msg/detail/service_event_info__struct.h"

// constants for array fields with an upper bound
// request
enum
{
  irobot_create_msgs__srv__ResetPose_Event__request__MAX_SIZE = 1
};
// response
enum
{
  irobot_create_msgs__srv__ResetPose_Event__response__MAX_SIZE = 1
};

/// Struct defined in srv/ResetPose in the package irobot_create_msgs.
typedef struct irobot_create_msgs__srv__ResetPose_Event
{
  service_msgs__msg__ServiceEventInfo info;
  irobot_create_msgs__srv__ResetPose_Request__Sequence request;
  irobot_create_msgs__srv__ResetPose_Response__Sequence response;
} irobot_create_msgs__srv__ResetPose_Event;

// Struct for a sequence of irobot_create_msgs__srv__ResetPose_Event.
typedef struct irobot_create_msgs__srv__ResetPose_Event__Sequence
{
  irobot_create_msgs__srv__ResetPose_Event * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__srv__ResetPose_Event__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // IROBOT_CREATE_MSGS__SRV__DETAIL__RESET_POSE__STRUCT_H_
