// generated from rosidl_generator_c/resource/idl__description.c.em
// with input from irobot_create_msgs:srv/EStop.idl
// generated code does not contain a copyright notice

#include "irobot_create_msgs/srv/detail/e_stop__functions.h"

ROSIDL_GENERATOR_C_PUBLIC_irobot_create_msgs
const rosidl_type_hash_t *
irobot_create_msgs__srv__EStop__get_type_hash(
  const rosidl_service_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_type_hash_t hash = {1, {
      0x7f, 0x76, 0x19, 0x71, 0xc2, 0xf3, 0x67, 0x1e,
      0xdc, 0x58, 0xbd, 0xba, 0x7c, 0x25, 0x74, 0x3e,
      0x0e, 0x9b, 0xc2, 0xed, 0xde, 0x8e, 0x33, 0xe8,
      0x93, 0x79, 0x54, 0x24, 0xaf, 0xee, 0x65, 0xfd,
    }};
  return &hash;
}

ROSIDL_GENERATOR_C_PUBLIC_irobot_create_msgs
const rosidl_type_hash_t *
irobot_create_msgs__srv__EStop_Request__get_type_hash(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_type_hash_t hash = {1, {
      0x98, 0x85, 0x97, 0xfd, 0xa7, 0xc4, 0xc5, 0x05,
      0xf1, 0x32, 0x59, 0x36, 0xf3, 0x79, 0xfe, 0xa8,
      0x60, 0xe4, 0x6a, 0x15, 0xa6, 0x5c, 0x91, 0xc0,
      0xd2, 0x1f, 0xe2, 0xd4, 0xc3, 0x27, 0x1f, 0xe5,
    }};
  return &hash;
}

ROSIDL_GENERATOR_C_PUBLIC_irobot_create_msgs
const rosidl_type_hash_t *
irobot_create_msgs__srv__EStop_Response__get_type_hash(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_type_hash_t hash = {1, {
      0xb0, 0xed, 0x1f, 0x5c, 0x62, 0x47, 0x87, 0x2e,
      0x3e, 0x20, 0xb4, 0xa3, 0x3a, 0x9d, 0xfd, 0xf1,
      0xcc, 0xd8, 0xb9, 0x58, 0xfa, 0xc8, 0x5f, 0x24,
      0x40, 0x63, 0xfa, 0x47, 0x9d, 0x52, 0xc2, 0x8f,
    }};
  return &hash;
}

ROSIDL_GENERATOR_C_PUBLIC_irobot_create_msgs
const rosidl_type_hash_t *
irobot_create_msgs__srv__EStop_Event__get_type_hash(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_type_hash_t hash = {1, {
      0xa3, 0xb7, 0x8d, 0xa5, 0xc7, 0xc4, 0xbc, 0x55,
      0xd1, 0x3d, 0x15, 0x21, 0xa3, 0xa0, 0x53, 0x39,
      0x0a, 0xf2, 0xda, 0xa7, 0xa1, 0xab, 0x81, 0x2c,
      0xbd, 0xf0, 0xde, 0x26, 0x18, 0xdb, 0x4c, 0xcd,
    }};
  return &hash;
}

#include <assert.h>
#include <string.h>

// Include directives for referenced types
#include "builtin_interfaces/msg/detail/time__functions.h"
#include "service_msgs/msg/detail/service_event_info__functions.h"

// Hashes for external referenced types
#ifndef NDEBUG
static const rosidl_type_hash_t builtin_interfaces__msg__Time__EXPECTED_HASH = {1, {
    0xb1, 0x06, 0x23, 0x5e, 0x25, 0xa4, 0xc5, 0xed,
    0x35, 0x09, 0x8a, 0xa0, 0xa6, 0x1a, 0x3e, 0xe9,
    0xc9, 0xb1, 0x8d, 0x19, 0x7f, 0x39, 0x8b, 0x0e,
    0x42, 0x06, 0xce, 0xa9, 0xac, 0xf9, 0xc1, 0x97,
  }};
static const rosidl_type_hash_t service_msgs__msg__ServiceEventInfo__EXPECTED_HASH = {1, {
    0x41, 0xbc, 0xbb, 0xe0, 0x7a, 0x75, 0xc9, 0xb5,
    0x2b, 0xc9, 0x6b, 0xfd, 0x5c, 0x24, 0xd7, 0xf0,
    0xfc, 0x0a, 0x08, 0xc0, 0xcb, 0x79, 0x21, 0xb3,
    0x37, 0x3c, 0x57, 0x32, 0x34, 0x5a, 0x6f, 0x45,
  }};
#endif

static char irobot_create_msgs__srv__EStop__TYPE_NAME[] = "irobot_create_msgs/srv/EStop";
static char builtin_interfaces__msg__Time__TYPE_NAME[] = "builtin_interfaces/msg/Time";
static char irobot_create_msgs__srv__EStop_Event__TYPE_NAME[] = "irobot_create_msgs/srv/EStop_Event";
static char irobot_create_msgs__srv__EStop_Request__TYPE_NAME[] = "irobot_create_msgs/srv/EStop_Request";
static char irobot_create_msgs__srv__EStop_Response__TYPE_NAME[] = "irobot_create_msgs/srv/EStop_Response";
static char service_msgs__msg__ServiceEventInfo__TYPE_NAME[] = "service_msgs/msg/ServiceEventInfo";

// Define type names, field names, and default values
static char irobot_create_msgs__srv__EStop__FIELD_NAME__request_message[] = "request_message";
static char irobot_create_msgs__srv__EStop__FIELD_NAME__response_message[] = "response_message";
static char irobot_create_msgs__srv__EStop__FIELD_NAME__event_message[] = "event_message";

static rosidl_runtime_c__type_description__Field irobot_create_msgs__srv__EStop__FIELDS[] = {
  {
    {irobot_create_msgs__srv__EStop__FIELD_NAME__request_message, 15, 15},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_NESTED_TYPE,
      0,
      0,
      {irobot_create_msgs__srv__EStop_Request__TYPE_NAME, 36, 36},
    },
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__srv__EStop__FIELD_NAME__response_message, 16, 16},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_NESTED_TYPE,
      0,
      0,
      {irobot_create_msgs__srv__EStop_Response__TYPE_NAME, 37, 37},
    },
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__srv__EStop__FIELD_NAME__event_message, 13, 13},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_NESTED_TYPE,
      0,
      0,
      {irobot_create_msgs__srv__EStop_Event__TYPE_NAME, 34, 34},
    },
    {NULL, 0, 0},
  },
};

static rosidl_runtime_c__type_description__IndividualTypeDescription irobot_create_msgs__srv__EStop__REFERENCED_TYPE_DESCRIPTIONS[] = {
  {
    {builtin_interfaces__msg__Time__TYPE_NAME, 27, 27},
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__srv__EStop_Event__TYPE_NAME, 34, 34},
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__srv__EStop_Request__TYPE_NAME, 36, 36},
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__srv__EStop_Response__TYPE_NAME, 37, 37},
    {NULL, 0, 0},
  },
  {
    {service_msgs__msg__ServiceEventInfo__TYPE_NAME, 33, 33},
    {NULL, 0, 0},
  },
};

const rosidl_runtime_c__type_description__TypeDescription *
irobot_create_msgs__srv__EStop__get_type_description(
  const rosidl_service_type_support_t * type_support)
{
  (void)type_support;
  static bool constructed = false;
  static const rosidl_runtime_c__type_description__TypeDescription description = {
    {
      {irobot_create_msgs__srv__EStop__TYPE_NAME, 28, 28},
      {irobot_create_msgs__srv__EStop__FIELDS, 3, 3},
    },
    {irobot_create_msgs__srv__EStop__REFERENCED_TYPE_DESCRIPTIONS, 5, 5},
  };
  if (!constructed) {
    assert(0 == memcmp(&builtin_interfaces__msg__Time__EXPECTED_HASH, builtin_interfaces__msg__Time__get_type_hash(NULL), sizeof(rosidl_type_hash_t)));
    description.referenced_type_descriptions.data[0].fields = builtin_interfaces__msg__Time__get_type_description(NULL)->type_description.fields;
    description.referenced_type_descriptions.data[1].fields = irobot_create_msgs__srv__EStop_Event__get_type_description(NULL)->type_description.fields;
    description.referenced_type_descriptions.data[2].fields = irobot_create_msgs__srv__EStop_Request__get_type_description(NULL)->type_description.fields;
    description.referenced_type_descriptions.data[3].fields = irobot_create_msgs__srv__EStop_Response__get_type_description(NULL)->type_description.fields;
    assert(0 == memcmp(&service_msgs__msg__ServiceEventInfo__EXPECTED_HASH, service_msgs__msg__ServiceEventInfo__get_type_hash(NULL), sizeof(rosidl_type_hash_t)));
    description.referenced_type_descriptions.data[4].fields = service_msgs__msg__ServiceEventInfo__get_type_description(NULL)->type_description.fields;
    constructed = true;
  }
  return &description;
}
// Define type names, field names, and default values
static char irobot_create_msgs__srv__EStop_Request__FIELD_NAME__e_stop_on[] = "e_stop_on";

static rosidl_runtime_c__type_description__Field irobot_create_msgs__srv__EStop_Request__FIELDS[] = {
  {
    {irobot_create_msgs__srv__EStop_Request__FIELD_NAME__e_stop_on, 9, 9},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_BOOLEAN,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
};

const rosidl_runtime_c__type_description__TypeDescription *
irobot_create_msgs__srv__EStop_Request__get_type_description(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static bool constructed = false;
  static const rosidl_runtime_c__type_description__TypeDescription description = {
    {
      {irobot_create_msgs__srv__EStop_Request__TYPE_NAME, 36, 36},
      {irobot_create_msgs__srv__EStop_Request__FIELDS, 1, 1},
    },
    {NULL, 0, 0},
  };
  if (!constructed) {
    constructed = true;
  }
  return &description;
}
// Define type names, field names, and default values
static char irobot_create_msgs__srv__EStop_Response__FIELD_NAME__success[] = "success";
static char irobot_create_msgs__srv__EStop_Response__FIELD_NAME__message[] = "message";

static rosidl_runtime_c__type_description__Field irobot_create_msgs__srv__EStop_Response__FIELDS[] = {
  {
    {irobot_create_msgs__srv__EStop_Response__FIELD_NAME__success, 7, 7},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_BOOLEAN,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__srv__EStop_Response__FIELD_NAME__message, 7, 7},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_STRING,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
};

const rosidl_runtime_c__type_description__TypeDescription *
irobot_create_msgs__srv__EStop_Response__get_type_description(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static bool constructed = false;
  static const rosidl_runtime_c__type_description__TypeDescription description = {
    {
      {irobot_create_msgs__srv__EStop_Response__TYPE_NAME, 37, 37},
      {irobot_create_msgs__srv__EStop_Response__FIELDS, 2, 2},
    },
    {NULL, 0, 0},
  };
  if (!constructed) {
    constructed = true;
  }
  return &description;
}
// Define type names, field names, and default values
static char irobot_create_msgs__srv__EStop_Event__FIELD_NAME__info[] = "info";
static char irobot_create_msgs__srv__EStop_Event__FIELD_NAME__request[] = "request";
static char irobot_create_msgs__srv__EStop_Event__FIELD_NAME__response[] = "response";

static rosidl_runtime_c__type_description__Field irobot_create_msgs__srv__EStop_Event__FIELDS[] = {
  {
    {irobot_create_msgs__srv__EStop_Event__FIELD_NAME__info, 4, 4},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_NESTED_TYPE,
      0,
      0,
      {service_msgs__msg__ServiceEventInfo__TYPE_NAME, 33, 33},
    },
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__srv__EStop_Event__FIELD_NAME__request, 7, 7},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_NESTED_TYPE_BOUNDED_SEQUENCE,
      1,
      0,
      {irobot_create_msgs__srv__EStop_Request__TYPE_NAME, 36, 36},
    },
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__srv__EStop_Event__FIELD_NAME__response, 8, 8},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_NESTED_TYPE_BOUNDED_SEQUENCE,
      1,
      0,
      {irobot_create_msgs__srv__EStop_Response__TYPE_NAME, 37, 37},
    },
    {NULL, 0, 0},
  },
};

static rosidl_runtime_c__type_description__IndividualTypeDescription irobot_create_msgs__srv__EStop_Event__REFERENCED_TYPE_DESCRIPTIONS[] = {
  {
    {builtin_interfaces__msg__Time__TYPE_NAME, 27, 27},
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__srv__EStop_Request__TYPE_NAME, 36, 36},
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__srv__EStop_Response__TYPE_NAME, 37, 37},
    {NULL, 0, 0},
  },
  {
    {service_msgs__msg__ServiceEventInfo__TYPE_NAME, 33, 33},
    {NULL, 0, 0},
  },
};

const rosidl_runtime_c__type_description__TypeDescription *
irobot_create_msgs__srv__EStop_Event__get_type_description(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static bool constructed = false;
  static const rosidl_runtime_c__type_description__TypeDescription description = {
    {
      {irobot_create_msgs__srv__EStop_Event__TYPE_NAME, 34, 34},
      {irobot_create_msgs__srv__EStop_Event__FIELDS, 3, 3},
    },
    {irobot_create_msgs__srv__EStop_Event__REFERENCED_TYPE_DESCRIPTIONS, 4, 4},
  };
  if (!constructed) {
    assert(0 == memcmp(&builtin_interfaces__msg__Time__EXPECTED_HASH, builtin_interfaces__msg__Time__get_type_hash(NULL), sizeof(rosidl_type_hash_t)));
    description.referenced_type_descriptions.data[0].fields = builtin_interfaces__msg__Time__get_type_description(NULL)->type_description.fields;
    description.referenced_type_descriptions.data[1].fields = irobot_create_msgs__srv__EStop_Request__get_type_description(NULL)->type_description.fields;
    description.referenced_type_descriptions.data[2].fields = irobot_create_msgs__srv__EStop_Response__get_type_description(NULL)->type_description.fields;
    assert(0 == memcmp(&service_msgs__msg__ServiceEventInfo__EXPECTED_HASH, service_msgs__msg__ServiceEventInfo__get_type_hash(NULL), sizeof(rosidl_type_hash_t)));
    description.referenced_type_descriptions.data[3].fields = service_msgs__msg__ServiceEventInfo__get_type_description(NULL)->type_description.fields;
    constructed = true;
  }
  return &description;
}

static char toplevel_type_raw_source[] =
  "bool e_stop_on # true enables e-stop disabling drive motor power, false disables e-stop re-enabling motor power\n"
  "---\n"
  "bool success   # indicate successful application of command\n"
  "string message # informational for result";

static char srv_encoding[] = "srv";
static char implicit_encoding[] = "implicit";

// Define all individual source functions

const rosidl_runtime_c__type_description__TypeSource *
irobot_create_msgs__srv__EStop__get_individual_type_description_source(
  const rosidl_service_type_support_t * type_support)
{
  (void)type_support;
  static const rosidl_runtime_c__type_description__TypeSource source = {
    {irobot_create_msgs__srv__EStop__TYPE_NAME, 28, 28},
    {srv_encoding, 3, 3},
    {toplevel_type_raw_source, 218, 218},
  };
  return &source;
}

const rosidl_runtime_c__type_description__TypeSource *
irobot_create_msgs__srv__EStop_Request__get_individual_type_description_source(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static const rosidl_runtime_c__type_description__TypeSource source = {
    {irobot_create_msgs__srv__EStop_Request__TYPE_NAME, 36, 36},
    {implicit_encoding, 8, 8},
    {NULL, 0, 0},
  };
  return &source;
}

const rosidl_runtime_c__type_description__TypeSource *
irobot_create_msgs__srv__EStop_Response__get_individual_type_description_source(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static const rosidl_runtime_c__type_description__TypeSource source = {
    {irobot_create_msgs__srv__EStop_Response__TYPE_NAME, 37, 37},
    {implicit_encoding, 8, 8},
    {NULL, 0, 0},
  };
  return &source;
}

const rosidl_runtime_c__type_description__TypeSource *
irobot_create_msgs__srv__EStop_Event__get_individual_type_description_source(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static const rosidl_runtime_c__type_description__TypeSource source = {
    {irobot_create_msgs__srv__EStop_Event__TYPE_NAME, 34, 34},
    {implicit_encoding, 8, 8},
    {NULL, 0, 0},
  };
  return &source;
}

const rosidl_runtime_c__type_description__TypeSource__Sequence *
irobot_create_msgs__srv__EStop__get_type_description_sources(
  const rosidl_service_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_runtime_c__type_description__TypeSource sources[6];
  static const rosidl_runtime_c__type_description__TypeSource__Sequence source_sequence = {sources, 6, 6};
  static bool constructed = false;
  if (!constructed) {
    sources[0] = *irobot_create_msgs__srv__EStop__get_individual_type_description_source(NULL),
    sources[1] = *builtin_interfaces__msg__Time__get_individual_type_description_source(NULL);
    sources[2] = *irobot_create_msgs__srv__EStop_Event__get_individual_type_description_source(NULL);
    sources[3] = *irobot_create_msgs__srv__EStop_Request__get_individual_type_description_source(NULL);
    sources[4] = *irobot_create_msgs__srv__EStop_Response__get_individual_type_description_source(NULL);
    sources[5] = *service_msgs__msg__ServiceEventInfo__get_individual_type_description_source(NULL);
    constructed = true;
  }
  return &source_sequence;
}

const rosidl_runtime_c__type_description__TypeSource__Sequence *
irobot_create_msgs__srv__EStop_Request__get_type_description_sources(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_runtime_c__type_description__TypeSource sources[1];
  static const rosidl_runtime_c__type_description__TypeSource__Sequence source_sequence = {sources, 1, 1};
  static bool constructed = false;
  if (!constructed) {
    sources[0] = *irobot_create_msgs__srv__EStop_Request__get_individual_type_description_source(NULL),
    constructed = true;
  }
  return &source_sequence;
}

const rosidl_runtime_c__type_description__TypeSource__Sequence *
irobot_create_msgs__srv__EStop_Response__get_type_description_sources(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_runtime_c__type_description__TypeSource sources[1];
  static const rosidl_runtime_c__type_description__TypeSource__Sequence source_sequence = {sources, 1, 1};
  static bool constructed = false;
  if (!constructed) {
    sources[0] = *irobot_create_msgs__srv__EStop_Response__get_individual_type_description_source(NULL),
    constructed = true;
  }
  return &source_sequence;
}

const rosidl_runtime_c__type_description__TypeSource__Sequence *
irobot_create_msgs__srv__EStop_Event__get_type_description_sources(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_runtime_c__type_description__TypeSource sources[5];
  static const rosidl_runtime_c__type_description__TypeSource__Sequence source_sequence = {sources, 5, 5};
  static bool constructed = false;
  if (!constructed) {
    sources[0] = *irobot_create_msgs__srv__EStop_Event__get_individual_type_description_source(NULL),
    sources[1] = *builtin_interfaces__msg__Time__get_individual_type_description_source(NULL);
    sources[2] = *irobot_create_msgs__srv__EStop_Request__get_individual_type_description_source(NULL);
    sources[3] = *irobot_create_msgs__srv__EStop_Response__get_individual_type_description_source(NULL);
    sources[4] = *service_msgs__msg__ServiceEventInfo__get_individual_type_description_source(NULL);
    constructed = true;
  }
  return &source_sequence;
}
