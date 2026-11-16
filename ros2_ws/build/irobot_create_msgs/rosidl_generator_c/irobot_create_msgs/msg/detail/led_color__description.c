// generated from rosidl_generator_c/resource/idl__description.c.em
// with input from irobot_create_msgs:msg/LedColor.idl
// generated code does not contain a copyright notice

#include "irobot_create_msgs/msg/detail/led_color__functions.h"

ROSIDL_GENERATOR_C_PUBLIC_irobot_create_msgs
const rosidl_type_hash_t *
irobot_create_msgs__msg__LedColor__get_type_hash(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_type_hash_t hash = {1, {
      0xe7, 0xfc, 0x6d, 0x32, 0x59, 0xf3, 0x48, 0x6f,
      0xe2, 0x65, 0xc7, 0x64, 0x7b, 0x5a, 0x61, 0x50,
      0x13, 0x30, 0x5d, 0x5f, 0xd2, 0xda, 0x83, 0x12,
      0xcd, 0xb4, 0x33, 0x74, 0x6f, 0xaf, 0x61, 0xba,
    }};
  return &hash;
}

#include <assert.h>
#include <string.h>

// Include directives for referenced types

// Hashes for external referenced types
#ifndef NDEBUG
#endif

static char irobot_create_msgs__msg__LedColor__TYPE_NAME[] = "irobot_create_msgs/msg/LedColor";

// Define type names, field names, and default values
static char irobot_create_msgs__msg__LedColor__FIELD_NAME__red[] = "red";
static char irobot_create_msgs__msg__LedColor__FIELD_NAME__green[] = "green";
static char irobot_create_msgs__msg__LedColor__FIELD_NAME__blue[] = "blue";

static rosidl_runtime_c__type_description__Field irobot_create_msgs__msg__LedColor__FIELDS[] = {
  {
    {irobot_create_msgs__msg__LedColor__FIELD_NAME__red, 3, 3},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_UINT8,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__msg__LedColor__FIELD_NAME__green, 5, 5},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_UINT8,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
  {
    {irobot_create_msgs__msg__LedColor__FIELD_NAME__blue, 4, 4},
    {
      rosidl_runtime_c__type_description__FieldType__FIELD_TYPE_UINT8,
      0,
      0,
      {NULL, 0, 0},
    },
    {NULL, 0, 0},
  },
};

const rosidl_runtime_c__type_description__TypeDescription *
irobot_create_msgs__msg__LedColor__get_type_description(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static bool constructed = false;
  static const rosidl_runtime_c__type_description__TypeDescription description = {
    {
      {irobot_create_msgs__msg__LedColor__TYPE_NAME, 31, 31},
      {irobot_create_msgs__msg__LedColor__FIELDS, 3, 3},
    },
    {NULL, 0, 0},
  };
  if (!constructed) {
    constructed = true;
  }
  return &description;
}

static char toplevel_type_raw_source[] =
  "# This message is to set RGB values (0-255) for an LED\n"
  "uint8 red\n"
  "uint8 green\n"
  "uint8 blue";

static char msg_encoding[] = "msg";

// Define all individual source functions

const rosidl_runtime_c__type_description__TypeSource *
irobot_create_msgs__msg__LedColor__get_individual_type_description_source(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static const rosidl_runtime_c__type_description__TypeSource source = {
    {irobot_create_msgs__msg__LedColor__TYPE_NAME, 31, 31},
    {msg_encoding, 3, 3},
    {toplevel_type_raw_source, 88, 88},
  };
  return &source;
}

const rosidl_runtime_c__type_description__TypeSource__Sequence *
irobot_create_msgs__msg__LedColor__get_type_description_sources(
  const rosidl_message_type_support_t * type_support)
{
  (void)type_support;
  static rosidl_runtime_c__type_description__TypeSource sources[1];
  static const rosidl_runtime_c__type_description__TypeSource__Sequence source_sequence = {sources, 1, 1};
  static bool constructed = false;
  if (!constructed) {
    sources[0] = *irobot_create_msgs__msg__LedColor__get_individual_type_description_source(NULL),
    constructed = true;
  }
  return &source_sequence;
}
