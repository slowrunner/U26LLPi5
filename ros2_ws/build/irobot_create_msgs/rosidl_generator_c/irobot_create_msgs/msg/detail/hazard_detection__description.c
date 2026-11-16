// generated from rosidl_generator_c/resource/idl__description.c.em
// with input from irobot_create_msgs:msg/HazardDetection.idl
// generated code does not contain a copyright notice

#include "irobot_create_msgs/msg/detail/hazard_detection__functions.h"

ROSIDL_GENERATOR_C_PUBLIC_irobot_create_msgs
const rosidl_type_hash_t *
irobot_create_msgs__msg__HazardDetection__get_type_hash(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_type_hash_t hash = {1, {
      0xca, 0x2f, 0xe3, 0x6a, 0x5b, 0xcf, 0x9c, 0x30,
      0x9e, 0x62, 0x06, 0x07, 0x16, 0xd6, 0xea, 0x35,
      0x4e, 0xe4, 0x77, 0x68, 0x6d, 0xac, 0x2e, 0xc0,
      0x89, 0xf7, 0xc9, 0x1f, 0x2f, 0x30, 0xd8, 0xe6,
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

static char irobot_create_msgs__msg__HazardDetection__TYPE_NAME[] = "irobot_create_msgs/msg/HazardDetection";
static char builtin_interfaces__msg__Time__TYPE_NAME[] = "builtin_interfaces/msg/Time";
static char std_msgs__msg__Header__TYPE_NAME[] = "std_msgs/msg/Header";

// Define type names, field names, and default values
static char irobot_create_msgs__msg__HazardDetection__FIELD_NAME__header[] = "header";
static char irobot_create_msgs__msg__HazardDetection__FIELD_NAME__type[] = "type";

static rosidl_runtime_c__type_description__Field irobot_create_msgs__msg__HazardDetection__FIELDS[] = {
  {
    {irobot_create_msgs__msg__HazardDetection__FIELD_NAME__header, 6, 6},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_NESTED_TYPE,
      0,
      0,
      {std_msgs__msg__Header__TYPE_NAME, 19, 19},
    },
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__msg__HazardDetection__FIELD_NAME__type, 4, 4},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_UINT8,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
};

static rosidl_runtime_c__type_description__IndividualTypeDescription irobot_create_msgs__msg__HazardDetection__REFERENCED_TYPE_DESCRIPTIONS[] = {
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
irobot_create_msgs__msg__HazardDetection__get_type_description(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static bool constructed = false;
  static const rosidl_runtime_c__type_description__TypeDescription description = {
    {
      {irobot_create_msgs__msg__HazardDetection__TYPE_NAME, 38, 38},
      {irobot_create_msgs__msg__HazardDetection__FIELDS, 2, 2},
    },
    {irobot_create_msgs__msg__HazardDetection__REFERENCED_TYPE_DESCRIPTIONS, 2, 2},
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
  "# This message describes a detected hazard.\n"
  "# The frame ID allows to retrieve the location of the sensor that made the detection.\n"
  "\n"
  "# The robot has reached its backup limit. It will not drive further backward for safety reasons.\n"
  "# You can disable this limit through the Create3 \"safety_override\" parameter by setting it \n"
  "# to \"backup_only\" or \"full\".\n"
  "# The Create3 webserver can be used to set a default value for the parameter.\n"
  "uint8 BACKUP_LIMIT=0\n"
  "# The robot has bumped against an obstacle\n"
  "uint8 BUMP=1\n"
  "# The robot detected a cliff\n"
  "uint8 CLIFF=2\n"
  "# The wheels of the robot are stalled against an obstacle\n"
  "uint8 STALL=3\n"
  "# The wheels of the robot are fully dropped\n"
  "uint8 WHEEL_DROP=4\n"
  "# The robot detects an obstacle in close proximity\n"
  "uint8 OBJECT_PROXIMITY=5\n"
  "\n"
  "std_msgs/Header header\n"
  "uint8 type";

static char msg_encoding[] = "msg";

// Define all individual source functions

const rosidl_runtime_c__type_description__TypeSource *
irobot_create_msgs__msg__HazardDetection__get_individual_type_description_source(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static const rosidl_runtime_c__type_description__TypeSource source = {
    {irobot_create_msgs__msg__HazardDetection__TYPE_NAME, 38, 38},
    {msg_encoding, 3, 3},
    {toplevel_type_raw_source, 794, 794},
  };
  return &source;
}

const rosidl_runtime_c__type_description__TypeSource__Sequence *
irobot_create_msgs__msg__HazardDetection__get_type_description_sources(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_runtime_c__type_description__TypeSource sources[3];
  static const rosidl_runtime_c__type_description__TypeSource__Sequence source_sequence = {sources, 3, 3};
  static bool constructed = false;
  if (!constructed) {
    sources[0] = *irobot_create_msgs__msg__HazardDetection__get_individual_type_description_source(NULL),
    sources[1] = *builtin_interfaces__msg__Time__get_individual_type_description_source(NULL);
    sources[2] = *std_msgs__msg__Header__get_individual_type_description_source(NULL);
    constructed = true;
  }
  return &source_sequence;
}
