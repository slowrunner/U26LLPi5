// generated from rosidl_generator_c/resource/idl__description.c.em
// with input from irobot_create_msgs:msg/Mouse.idl
// generated code does not contain a copyright notice

#include "irobot_create_msgs/msg/detail/mouse__functions.h"

ROSIDL_GENERATOR_C_PUBLIC_irobot_create_msgs
const rosidl_type_hash_t *
irobot_create_msgs__msg__Mouse__get_type_hash(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_type_hash_t hash = {1, {
      0xd5, 0xed, 0x7f, 0x5a, 0xdb, 0x7d, 0xa4, 0x87,
      0x88, 0x2e, 0xbe, 0x1d, 0x66, 0x08, 0x72, 0x8b,
      0xc5, 0x8e, 0xd4, 0x25, 0x35, 0x93, 0x88, 0x58,
      0xb2, 0x9d, 0x6b, 0x90, 0xb4, 0x12, 0xd0, 0x39,
    }};
  return &hash;
}

#include <assert.h>
#include <string.h>

// Include directives for referenced types
#include "builtin_interfaces/msg/detail/time__functions.h"
#include "std_msgs/msg/detail/header__functions.h"

// Hashes for external referenced types
#ifndef NDEBUG
static const rosidl_type_hash_t builtin_interfaces__msg__Time__EXPECTED_HASH = {1, {
    0xb1, 0x06, 0x23, 0x5e, 0x25, 0xa4, 0xc5, 0xed,
    0x35, 0x09, 0x8a, 0xa0, 0xa6, 0x1a, 0x3e, 0xe9,
    0xc9, 0xb1, 0x8d, 0x19, 0x7f, 0x39, 0x8b, 0x0e,
    0x42, 0x06, 0xce, 0xa9, 0xac, 0xf9, 0xc1, 0x97,
  }};
static const rosidl_type_hash_t std_msgs__msg__Header__EXPECTED_HASH = {1, {
    0xf4, 0x9f, 0xb3, 0xae, 0x2c, 0xf0, 0x70, 0xf7,
    0x93, 0x64, 0x5f, 0xf7, 0x49, 0x68, 0x3a, 0xc6,
    0xb0, 0x62, 0x03, 0xe4, 0x1c, 0x89, 0x1e, 0x17,
    0x70, 0x1b, 0x1c, 0xb5, 0x97, 0xce, 0x6a, 0x01,
  }};
#endif

static char irobot_create_msgs__msg__Mouse__TYPE_NAME[] = "irobot_create_msgs/msg/Mouse";
static char builtin_interfaces__msg__Time__TYPE_NAME[] = "builtin_interfaces/msg/Time";
static char std_msgs__msg__Header__TYPE_NAME[] = "std_msgs/msg/Header";

// Define type names, field names, and default values
static char irobot_create_msgs__msg__Mouse__FIELD_NAME__header[] = "header";
static char irobot_create_msgs__msg__Mouse__FIELD_NAME__integrated_x[] = "integrated_x";
static char irobot_create_msgs__msg__Mouse__FIELD_NAME__integrated_y[] = "integrated_y";
static char irobot_create_msgs__msg__Mouse__FIELD_NAME__frame_id[] = "frame_id";
static char irobot_create_msgs__msg__Mouse__FIELD_NAME__last_squal[] = "last_squal";

static rosidl_runtime_c__type_description__Field irobot_create_msgs__msg__Mouse__FIELDS[] = {
  {
    {irobot_create_msgs__msg__Mouse__FIELD_NAME__header, 6, 6},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_NESTED_TYPE,
      0,
      0,
      {std_msgs__msg__Header__TYPE_NAME, 19, 19},
    },
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__msg__Mouse__FIELD_NAME__integrated_x, 12, 12},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_FLOAT,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__msg__Mouse__FIELD_NAME__integrated_y, 12, 12},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_FLOAT,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__msg__Mouse__FIELD_NAME__frame_id, 8, 8},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_UINT32,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__msg__Mouse__FIELD_NAME__last_squal, 10, 10},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_UINT8,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
};

static rosidl_runtime_c__type_description__IndividualTypeDescription irobot_create_msgs__msg__Mouse__REFERENCED_TYPE_DESCRIPTIONS[] = {
  {
    {builtin_interfaces__msg__Time__TYPE_NAME, 27, 27},
    {NULL, 0, 0},
  },
  {
    {std_msgs__msg__Header__TYPE_NAME, 19, 19},
    {NULL, 0, 0},
  },
};

const rosidl_runtime_c__type_description__TypeDescription *
irobot_create_msgs__msg__Mouse__get_type_description(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static bool constructed = false;
  static const rosidl_runtime_c__type_description__TypeDescription description = {
    {
      {irobot_create_msgs__msg__Mouse__TYPE_NAME, 28, 28},
      {irobot_create_msgs__msg__Mouse__FIELDS, 5, 5},
    },
    {irobot_create_msgs__msg__Mouse__REFERENCED_TYPE_DESCRIPTIONS, 2, 2},
  };
  if (!constructed) {
    assert(0 == memcmp(&builtin_interfaces__msg__Time__EXPECTED_HASH, builtin_interfaces__msg__Time__get_type_hash(NULL), sizeof(rosidl_type_hash_t)));
    description.referenced_type_descriptions.data[0].fields = builtin_interfaces__msg__Time__get_type_description(NULL)->type_description.fields;
    assert(0 == memcmp(&std_msgs__msg__Header__EXPECTED_HASH, std_msgs__msg__Header__get_type_hash(NULL), sizeof(rosidl_type_hash_t)));
    description.referenced_type_descriptions.data[1].fields = std_msgs__msg__Header__get_type_description(NULL)->type_description.fields;
    constructed = true;
  }
  return &description;
}

static char toplevel_type_raw_source[] =
  "# This message contains a mouse raw measurement.\n"
  "\n"
  "std_msgs/Header header    # Header stamp should be acquisition time of mouse measure.\n"
  "float32 integrated_x      # Motions in the X dimension in meters.\n"
  "float32 integrated_y      # Motions in the Y dimension in meters.\n"
  "uint32 frame_id           # Value incremented every time mouse stops tracking\n"
  "uint8 last_squal          # Surface quality level. It's proportional to the number of valid features visible by the sensor.";

static char msg_encoding[] = "msg";

// Define all individual source functions

const rosidl_runtime_c__type_description__TypeSource *
irobot_create_msgs__msg__Mouse__get_individual_type_description_source(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static const rosidl_runtime_c__type_description__TypeSource source = {
    {irobot_create_msgs__msg__Mouse__TYPE_NAME, 28, 28},
    {msg_encoding, 3, 3},
    {toplevel_type_raw_source, 470, 470},
  };
  return &source;
}

const rosidl_runtime_c__type_description__TypeSource__Sequence *
irobot_create_msgs__msg__Mouse__get_type_description_sources(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_runtime_c__type_description__TypeSource sources[3];
  static const rosidl_runtime_c__type_description__TypeSource__Sequence source_sequence = {sources, 3, 3};
  static bool constructed = false;
  if (!constructed) {
    sources[0] = *irobot_create_msgs__msg__Mouse__get_individual_type_description_source(NULL),
    sources[1] = *builtin_interfaces__msg__Time__get_individual_type_description_source(NULL);
    sources[2] = *std_msgs__msg__Header__get_individual_type_description_source(NULL);
    constructed = true;
  }
  return &source_sequence;
}
