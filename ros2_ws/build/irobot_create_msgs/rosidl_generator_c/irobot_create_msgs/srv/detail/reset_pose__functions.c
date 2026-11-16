// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from irobot_create_msgs:srv/ResetPose.idl
// generated code does not contain a copyright notice
#include "irobot_create_msgs/srv/detail/reset_pose__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"

// Include directives for member types
// Member `pose`
#include "geometry_msgs/msg/detail/pose__functions.h"

bool
irobot_create_msgs__srv__ResetPose_Request__init(irobot_create_msgs__srv__ResetPose_Request * msg)
{
  if (!msg) {
    return false;
  }
  // pose
  if (!geometry_msgs__msg__Pose__init(&msg->pose)) {
    irobot_create_msgs__srv__ResetPose_Request__fini(msg);
    return false;
  }
  return true;
}

void
irobot_create_msgs__srv__ResetPose_Request__fini(irobot_create_msgs__srv__ResetPose_Request * msg)
{
  if (!msg) {
    return;
  }
  // pose
  geometry_msgs__msg__Pose__fini(&msg->pose);
}

bool
irobot_create_msgs__srv__ResetPose_Request__are_equal(const irobot_create_msgs__srv__ResetPose_Request * lhs, const irobot_create_msgs__srv__ResetPose_Request * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // pose
  if (!geometry_msgs__msg__Pose__are_equal(
      &(lhs->pose), &(rhs->pose)))
  {
    return false;
  }
  return true;
}

bool
irobot_create_msgs__srv__ResetPose_Request__copy(
  const irobot_create_msgs__srv__ResetPose_Request * input,
  irobot_create_msgs__srv__ResetPose_Request * output)
{
  if (!input || !output) {
    return false;
  }
  // pose
  if (!geometry_msgs__msg__Pose__copy(
      &(input->pose), &(output->pose)))
  {
    return false;
  }
  return true;
}

irobot_create_msgs__srv__ResetPose_Request *
irobot_create_msgs__srv__ResetPose_Request__create(void)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  irobot_create_msgs__srv__ResetPose_Request * msg = (irobot_create_msgs__srv__ResetPose_Request *)allocator.allocate(sizeof(irobot_create_msgs__srv__ResetPose_Request), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(irobot_create_msgs__srv__ResetPose_Request));
  bool success = irobot_create_msgs__srv__ResetPose_Request__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
irobot_create_msgs__srv__ResetPose_Request__destroy(irobot_create_msgs__srv__ResetPose_Request * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    irobot_create_msgs__srv__ResetPose_Request__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
irobot_create_msgs__srv__ResetPose_Request__Sequence__init(irobot_create_msgs__srv__ResetPose_Request__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  irobot_create_msgs__srv__ResetPose_Request * data = NULL;

  if (size) {
    if (size > SIZE_MAX / sizeof(irobot_create_msgs__srv__ResetPose_Request)) {
      return false;
    }
    data = (irobot_create_msgs__srv__ResetPose_Request *)allocator.zero_allocate(size, sizeof(irobot_create_msgs__srv__ResetPose_Request), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = irobot_create_msgs__srv__ResetPose_Request__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        irobot_create_msgs__srv__ResetPose_Request__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
irobot_create_msgs__srv__ResetPose_Request__Sequence__fini(irobot_create_msgs__srv__ResetPose_Request__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      irobot_create_msgs__srv__ResetPose_Request__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

irobot_create_msgs__srv__ResetPose_Request__Sequence *
irobot_create_msgs__srv__ResetPose_Request__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  irobot_create_msgs__srv__ResetPose_Request__Sequence * array = (irobot_create_msgs__srv__ResetPose_Request__Sequence *)allocator.allocate(sizeof(irobot_create_msgs__srv__ResetPose_Request__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = irobot_create_msgs__srv__ResetPose_Request__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
irobot_create_msgs__srv__ResetPose_Request__Sequence__destroy(irobot_create_msgs__srv__ResetPose_Request__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    irobot_create_msgs__srv__ResetPose_Request__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
irobot_create_msgs__srv__ResetPose_Request__Sequence__are_equal(const irobot_create_msgs__srv__ResetPose_Request__Sequence * lhs, const irobot_create_msgs__srv__ResetPose_Request__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!irobot_create_msgs__srv__ResetPose_Request__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
irobot_create_msgs__srv__ResetPose_Request__Sequence__copy(
  const irobot_create_msgs__srv__ResetPose_Request__Sequence * input,
  irobot_create_msgs__srv__ResetPose_Request__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    if (input->size > SIZE_MAX / sizeof(irobot_create_msgs__srv__ResetPose_Request)) {
      return false;
    }
    const size_t allocation_size =
      input->size * sizeof(irobot_create_msgs__srv__ResetPose_Request);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    irobot_create_msgs__srv__ResetPose_Request * data =
      (irobot_create_msgs__srv__ResetPose_Request *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!irobot_create_msgs__srv__ResetPose_Request__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          irobot_create_msgs__srv__ResetPose_Request__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!irobot_create_msgs__srv__ResetPose_Request__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


bool
irobot_create_msgs__srv__ResetPose_Response__init(irobot_create_msgs__srv__ResetPose_Response * msg)
{
  if (!msg) {
    return false;
  }
  // structure_needs_at_least_one_member
  return true;
}

void
irobot_create_msgs__srv__ResetPose_Response__fini(irobot_create_msgs__srv__ResetPose_Response * msg)
{
  if (!msg) {
    return;
  }
  // structure_needs_at_least_one_member
}

bool
irobot_create_msgs__srv__ResetPose_Response__are_equal(const irobot_create_msgs__srv__ResetPose_Response * lhs, const irobot_create_msgs__srv__ResetPose_Response * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // structure_needs_at_least_one_member
  if (lhs->structure_needs_at_least_one_member != rhs->structure_needs_at_least_one_member) {
    return false;
  }
  return true;
}

bool
irobot_create_msgs__srv__ResetPose_Response__copy(
  const irobot_create_msgs__srv__ResetPose_Response * input,
  irobot_create_msgs__srv__ResetPose_Response * output)
{
  if (!input || !output) {
    return false;
  }
  // structure_needs_at_least_one_member
  output->structure_needs_at_least_one_member = input->structure_needs_at_least_one_member;
  return true;
}

irobot_create_msgs__srv__ResetPose_Response *
irobot_create_msgs__srv__ResetPose_Response__create(void)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  irobot_create_msgs__srv__ResetPose_Response * msg = (irobot_create_msgs__srv__ResetPose_Response *)allocator.allocate(sizeof(irobot_create_msgs__srv__ResetPose_Response), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(irobot_create_msgs__srv__ResetPose_Response));
  bool success = irobot_create_msgs__srv__ResetPose_Response__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
irobot_create_msgs__srv__ResetPose_Response__destroy(irobot_create_msgs__srv__ResetPose_Response * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    irobot_create_msgs__srv__ResetPose_Response__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
irobot_create_msgs__srv__ResetPose_Response__Sequence__init(irobot_create_msgs__srv__ResetPose_Response__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  irobot_create_msgs__srv__ResetPose_Response * data = NULL;

  if (size) {
    if (size > SIZE_MAX / sizeof(irobot_create_msgs__srv__ResetPose_Response)) {
      return false;
    }
    data = (irobot_create_msgs__srv__ResetPose_Response *)allocator.zero_allocate(size, sizeof(irobot_create_msgs__srv__ResetPose_Response), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = irobot_create_msgs__srv__ResetPose_Response__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        irobot_create_msgs__srv__ResetPose_Response__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
irobot_create_msgs__srv__ResetPose_Response__Sequence__fini(irobot_create_msgs__srv__ResetPose_Response__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      irobot_create_msgs__srv__ResetPose_Response__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

irobot_create_msgs__srv__ResetPose_Response__Sequence *
irobot_create_msgs__srv__ResetPose_Response__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  irobot_create_msgs__srv__ResetPose_Response__Sequence * array = (irobot_create_msgs__srv__ResetPose_Response__Sequence *)allocator.allocate(sizeof(irobot_create_msgs__srv__ResetPose_Response__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = irobot_create_msgs__srv__ResetPose_Response__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
irobot_create_msgs__srv__ResetPose_Response__Sequence__destroy(irobot_create_msgs__srv__ResetPose_Response__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    irobot_create_msgs__srv__ResetPose_Response__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
irobot_create_msgs__srv__ResetPose_Response__Sequence__are_equal(const irobot_create_msgs__srv__ResetPose_Response__Sequence * lhs, const irobot_create_msgs__srv__ResetPose_Response__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!irobot_create_msgs__srv__ResetPose_Response__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
irobot_create_msgs__srv__ResetPose_Response__Sequence__copy(
  const irobot_create_msgs__srv__ResetPose_Response__Sequence * input,
  irobot_create_msgs__srv__ResetPose_Response__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    if (input->size > SIZE_MAX / sizeof(irobot_create_msgs__srv__ResetPose_Response)) {
      return false;
    }
    const size_t allocation_size =
      input->size * sizeof(irobot_create_msgs__srv__ResetPose_Response);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    irobot_create_msgs__srv__ResetPose_Response * data =
      (irobot_create_msgs__srv__ResetPose_Response *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!irobot_create_msgs__srv__ResetPose_Response__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          irobot_create_msgs__srv__ResetPose_Response__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!irobot_create_msgs__srv__ResetPose_Response__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


// Include directives for member types
// Member `info`
#include "service_msgs/msg/detail/service_event_info__functions.h"
// Member `request`
// Member `response`
// already included above
// #include "irobot_create_msgs/srv/detail/reset_pose__functions.h"

bool
irobot_create_msgs__srv__ResetPose_Event__init(irobot_create_msgs__srv__ResetPose_Event * msg)
{
  if (!msg) {
    return false;
  }
  // info
  if (!service_msgs__msg__ServiceEventInfo__init(&msg->info)) {
    irobot_create_msgs__srv__ResetPose_Event__fini(msg);
    return false;
  }
  // request
  if (!irobot_create_msgs__srv__ResetPose_Request__Sequence__init(&msg->request, 0)) {
    irobot_create_msgs__srv__ResetPose_Event__fini(msg);
    return false;
  }
  // response
  if (!irobot_create_msgs__srv__ResetPose_Response__Sequence__init(&msg->response, 0)) {
    irobot_create_msgs__srv__ResetPose_Event__fini(msg);
    return false;
  }
  return true;
}

void
irobot_create_msgs__srv__ResetPose_Event__fini(irobot_create_msgs__srv__ResetPose_Event * msg)
{
  if (!msg) {
    return;
  }
  // info
  service_msgs__msg__ServiceEventInfo__fini(&msg->info);
  // request
  irobot_create_msgs__srv__ResetPose_Request__Sequence__fini(&msg->request);
  // response
  irobot_create_msgs__srv__ResetPose_Response__Sequence__fini(&msg->response);
}

bool
irobot_create_msgs__srv__ResetPose_Event__are_equal(const irobot_create_msgs__srv__ResetPose_Event * lhs, const irobot_create_msgs__srv__ResetPose_Event * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // info
  if (!service_msgs__msg__ServiceEventInfo__are_equal(
      &(lhs->info), &(rhs->info)))
  {
    return false;
  }
  // request
  if (!irobot_create_msgs__srv__ResetPose_Request__Sequence__are_equal(
      &(lhs->request), &(rhs->request)))
  {
    return false;
  }
  // response
  if (!irobot_create_msgs__srv__ResetPose_Response__Sequence__are_equal(
      &(lhs->response), &(rhs->response)))
  {
    return false;
  }
  return true;
}

bool
irobot_create_msgs__srv__ResetPose_Event__copy(
  const irobot_create_msgs__srv__ResetPose_Event * input,
  irobot_create_msgs__srv__ResetPose_Event * output)
{
  if (!input || !output) {
    return false;
  }
  // info
  if (!service_msgs__msg__ServiceEventInfo__copy(
      &(input->info), &(output->info)))
  {
    return false;
  }
  // request
  if (!irobot_create_msgs__srv__ResetPose_Request__Sequence__copy(
      &(input->request), &(output->request)))
  {
    return false;
  }
  // response
  if (!irobot_create_msgs__srv__ResetPose_Response__Sequence__copy(
      &(input->response), &(output->response)))
  {
    return false;
  }
  return true;
}

irobot_create_msgs__srv__ResetPose_Event *
irobot_create_msgs__srv__ResetPose_Event__create(void)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  irobot_create_msgs__srv__ResetPose_Event * msg = (irobot_create_msgs__srv__ResetPose_Event *)allocator.allocate(sizeof(irobot_create_msgs__srv__ResetPose_Event), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(irobot_create_msgs__srv__ResetPose_Event));
  bool success = irobot_create_msgs__srv__ResetPose_Event__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
irobot_create_msgs__srv__ResetPose_Event__destroy(irobot_create_msgs__srv__ResetPose_Event * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    irobot_create_msgs__srv__ResetPose_Event__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
irobot_create_msgs__srv__ResetPose_Event__Sequence__init(irobot_create_msgs__srv__ResetPose_Event__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  irobot_create_msgs__srv__ResetPose_Event * data = NULL;

  if (size) {
    if (size > SIZE_MAX / sizeof(irobot_create_msgs__srv__ResetPose_Event)) {
      return false;
    }
    data = (irobot_create_msgs__srv__ResetPose_Event *)allocator.zero_allocate(size, sizeof(irobot_create_msgs__srv__ResetPose_Event), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = irobot_create_msgs__srv__ResetPose_Event__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        irobot_create_msgs__srv__ResetPose_Event__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
irobot_create_msgs__srv__ResetPose_Event__Sequence__fini(irobot_create_msgs__srv__ResetPose_Event__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      irobot_create_msgs__srv__ResetPose_Event__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

irobot_create_msgs__srv__ResetPose_Event__Sequence *
irobot_create_msgs__srv__ResetPose_Event__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  irobot_create_msgs__srv__ResetPose_Event__Sequence * array = (irobot_create_msgs__srv__ResetPose_Event__Sequence *)allocator.allocate(sizeof(irobot_create_msgs__srv__ResetPose_Event__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = irobot_create_msgs__srv__ResetPose_Event__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
irobot_create_msgs__srv__ResetPose_Event__Sequence__destroy(irobot_create_msgs__srv__ResetPose_Event__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    irobot_create_msgs__srv__ResetPose_Event__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
irobot_create_msgs__srv__ResetPose_Event__Sequence__are_equal(const irobot_create_msgs__srv__ResetPose_Event__Sequence * lhs, const irobot_create_msgs__srv__ResetPose_Event__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!irobot_create_msgs__srv__ResetPose_Event__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
irobot_create_msgs__srv__ResetPose_Event__Sequence__copy(
  const irobot_create_msgs__srv__ResetPose_Event__Sequence * input,
  irobot_create_msgs__srv__ResetPose_Event__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    if (input->size > SIZE_MAX / sizeof(irobot_create_msgs__srv__ResetPose_Event)) {
      return false;
    }
    const size_t allocation_size =
      input->size * sizeof(irobot_create_msgs__srv__ResetPose_Event);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    irobot_create_msgs__srv__ResetPose_Event * data =
      (irobot_create_msgs__srv__ResetPose_Event *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!irobot_create_msgs__srv__ResetPose_Event__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          irobot_create_msgs__srv__ResetPose_Event__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!irobot_create_msgs__srv__ResetPose_Event__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
