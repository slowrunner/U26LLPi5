// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from irobot_create_msgs:action/WallFollow.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "irobot_create_msgs/action/wall_follow.h"


#ifndef IROBOT_CREATE_MSGS__ACTION__DETAIL__WALL_FOLLOW__STRUCT_H_
#define IROBOT_CREATE_MSGS__ACTION__DETAIL__WALL_FOLLOW__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Constant 'FOLLOW_RIGHT'.
enum
{
  irobot_create_msgs__action__WallFollow_Goal__FOLLOW_RIGHT = -1
};

/// Constant 'FOLLOW_LEFT'.
enum
{
  irobot_create_msgs__action__WallFollow_Goal__FOLLOW_LEFT = 1
};

// Include directives for member types
// Member 'max_runtime'
#include "builtin_interfaces/msg/detail/duration__struct.h"

/// Struct defined in action/WallFollow in the package irobot_create_msgs.
typedef struct irobot_create_msgs__action__WallFollow_Goal
{
  int8_t follow_side;
  builtin_interfaces__msg__Duration max_runtime;
} irobot_create_msgs__action__WallFollow_Goal;

// Struct for a sequence of irobot_create_msgs__action__WallFollow_Goal.
typedef struct irobot_create_msgs__action__WallFollow_Goal__Sequence
{
  irobot_create_msgs__action__WallFollow_Goal * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__action__WallFollow_Goal__Sequence;

// Constants defined in the message

// Include directives for member types
// Member 'runtime'
// already included above
// #include "builtin_interfaces/msg/detail/duration__struct.h"

/// Struct defined in action/WallFollow in the package irobot_create_msgs.
typedef struct irobot_create_msgs__action__WallFollow_Result
{
  builtin_interfaces__msg__Duration runtime;
} irobot_create_msgs__action__WallFollow_Result;

// Struct for a sequence of irobot_create_msgs__action__WallFollow_Result.
typedef struct irobot_create_msgs__action__WallFollow_Result__Sequence
{
  irobot_create_msgs__action__WallFollow_Result * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__action__WallFollow_Result__Sequence;

// Constants defined in the message

/// Struct defined in action/WallFollow in the package irobot_create_msgs.
typedef struct irobot_create_msgs__action__WallFollow_Feedback
{
  bool engaged;
} irobot_create_msgs__action__WallFollow_Feedback;

// Struct for a sequence of irobot_create_msgs__action__WallFollow_Feedback.
typedef struct irobot_create_msgs__action__WallFollow_Feedback__Sequence
{
  irobot_create_msgs__action__WallFollow_Feedback * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__action__WallFollow_Feedback__Sequence;

// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'goal'
#include "irobot_create_msgs/action/detail/wall_follow__struct.h"

/// Struct defined in action/WallFollow in the package irobot_create_msgs.
typedef struct irobot_create_msgs__action__WallFollow_SendGoal_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
  irobot_create_msgs__action__WallFollow_Goal goal;
} irobot_create_msgs__action__WallFollow_SendGoal_Request;

// Struct for a sequence of irobot_create_msgs__action__WallFollow_SendGoal_Request.
typedef struct irobot_create_msgs__action__WallFollow_SendGoal_Request__Sequence
{
  irobot_create_msgs__action__WallFollow_SendGoal_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__action__WallFollow_SendGoal_Request__Sequence;

// Constants defined in the message

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.h"

/// Struct defined in action/WallFollow in the package irobot_create_msgs.
typedef struct irobot_create_msgs__action__WallFollow_SendGoal_Response
{
  bool accepted;
  builtin_interfaces__msg__Time stamp;
} irobot_create_msgs__action__WallFollow_SendGoal_Response;

// Struct for a sequence of irobot_create_msgs__action__WallFollow_SendGoal_Response.
typedef struct irobot_create_msgs__action__WallFollow_SendGoal_Response__Sequence
{
  irobot_create_msgs__action__WallFollow_SendGoal_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__action__WallFollow_SendGoal_Response__Sequence;

// Constants defined in the message

// Include directives for member types
// Member 'info'
#include "service_msgs/msg/detail/service_event_info__struct.h"

// constants for array fields with an upper bound
// request
enum
{
  irobot_create_msgs__action__WallFollow_SendGoal_Event__request__MAX_SIZE = 1
};
// response
enum
{
  irobot_create_msgs__action__WallFollow_SendGoal_Event__response__MAX_SIZE = 1
};

/// Struct defined in action/WallFollow in the package irobot_create_msgs.
typedef struct irobot_create_msgs__action__WallFollow_SendGoal_Event
{
  service_msgs__msg__ServiceEventInfo info;
  irobot_create_msgs__action__WallFollow_SendGoal_Request__Sequence request;
  irobot_create_msgs__action__WallFollow_SendGoal_Response__Sequence response;
} irobot_create_msgs__action__WallFollow_SendGoal_Event;

// Struct for a sequence of irobot_create_msgs__action__WallFollow_SendGoal_Event.
typedef struct irobot_create_msgs__action__WallFollow_SendGoal_Event__Sequence
{
  irobot_create_msgs__action__WallFollow_SendGoal_Event * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__action__WallFollow_SendGoal_Event__Sequence;

// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"

/// Struct defined in action/WallFollow in the package irobot_create_msgs.
typedef struct irobot_create_msgs__action__WallFollow_GetResult_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
} irobot_create_msgs__action__WallFollow_GetResult_Request;

// Struct for a sequence of irobot_create_msgs__action__WallFollow_GetResult_Request.
typedef struct irobot_create_msgs__action__WallFollow_GetResult_Request__Sequence
{
  irobot_create_msgs__action__WallFollow_GetResult_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__action__WallFollow_GetResult_Request__Sequence;

// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "irobot_create_msgs/action/detail/wall_follow__struct.h"

/// Struct defined in action/WallFollow in the package irobot_create_msgs.
typedef struct irobot_create_msgs__action__WallFollow_GetResult_Response
{
  int8_t status;
  irobot_create_msgs__action__WallFollow_Result result;
} irobot_create_msgs__action__WallFollow_GetResult_Response;

// Struct for a sequence of irobot_create_msgs__action__WallFollow_GetResult_Response.
typedef struct irobot_create_msgs__action__WallFollow_GetResult_Response__Sequence
{
  irobot_create_msgs__action__WallFollow_GetResult_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__action__WallFollow_GetResult_Response__Sequence;

// Constants defined in the message

// Include directives for member types
// Member 'info'
// already included above
// #include "service_msgs/msg/detail/service_event_info__struct.h"

// constants for array fields with an upper bound
// request
enum
{
  irobot_create_msgs__action__WallFollow_GetResult_Event__request__MAX_SIZE = 1
};
// response
enum
{
  irobot_create_msgs__action__WallFollow_GetResult_Event__response__MAX_SIZE = 1
};

/// Struct defined in action/WallFollow in the package irobot_create_msgs.
typedef struct irobot_create_msgs__action__WallFollow_GetResult_Event
{
  service_msgs__msg__ServiceEventInfo info;
  irobot_create_msgs__action__WallFollow_GetResult_Request__Sequence request;
  irobot_create_msgs__action__WallFollow_GetResult_Response__Sequence response;
} irobot_create_msgs__action__WallFollow_GetResult_Event;

// Struct for a sequence of irobot_create_msgs__action__WallFollow_GetResult_Event.
typedef struct irobot_create_msgs__action__WallFollow_GetResult_Event__Sequence
{
  irobot_create_msgs__action__WallFollow_GetResult_Event * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__action__WallFollow_GetResult_Event__Sequence;

// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'feedback'
// already included above
// #include "irobot_create_msgs/action/detail/wall_follow__struct.h"

/// Struct defined in action/WallFollow in the package irobot_create_msgs.
typedef struct irobot_create_msgs__action__WallFollow_FeedbackMessage
{
  unique_identifier_msgs__msg__UUID goal_id;
  irobot_create_msgs__action__WallFollow_Feedback feedback;
} irobot_create_msgs__action__WallFollow_FeedbackMessage;

// Struct for a sequence of irobot_create_msgs__action__WallFollow_FeedbackMessage.
typedef struct irobot_create_msgs__action__WallFollow_FeedbackMessage__Sequence
{
  irobot_create_msgs__action__WallFollow_FeedbackMessage * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} irobot_create_msgs__action__WallFollow_FeedbackMessage__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // IROBOT_CREATE_MSGS__ACTION__DETAIL__WALL_FOLLOW__STRUCT_H_
