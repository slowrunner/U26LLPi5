// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from irobot_create_msgs:srv/ResetPose.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "irobot_create_msgs/srv/reset_pose.hpp"


#ifndef IROBOT_CREATE_MSGS__SRV__DETAIL__RESET_POSE__BUILDER_HPP_
#define IROBOT_CREATE_MSGS__SRV__DETAIL__RESET_POSE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "irobot_create_msgs/srv/detail/reset_pose__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace irobot_create_msgs
{

namespace srv
{

namespace builder
{

class Init_ResetPose_Request_pose
{
public:
  Init_ResetPose_Request_pose()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::irobot_create_msgs::srv::ResetPose_Request pose(::irobot_create_msgs::srv::ResetPose_Request::_pose_type arg)
  {
    msg_.pose = std::move(arg);
    return std::move(msg_);
  }

private:
  ::irobot_create_msgs::srv::ResetPose_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::irobot_create_msgs::srv::ResetPose_Request>()
{
  return irobot_create_msgs::srv::builder::Init_ResetPose_Request_pose();
}

}  // namespace irobot_create_msgs


namespace irobot_create_msgs
{

namespace srv
{


}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::irobot_create_msgs::srv::ResetPose_Response>()
{
  return ::irobot_create_msgs::srv::ResetPose_Response(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace irobot_create_msgs


namespace irobot_create_msgs
{

namespace srv
{

namespace builder
{

class Init_ResetPose_Event_response
{
public:
  explicit Init_ResetPose_Event_response(::irobot_create_msgs::srv::ResetPose_Event & msg)
  : msg_(msg)
  {}
  ::irobot_create_msgs::srv::ResetPose_Event response(::irobot_create_msgs::srv::ResetPose_Event::_response_type arg)
  {
    msg_.response = std::move(arg);
    return std::move(msg_);
  }

private:
  ::irobot_create_msgs::srv::ResetPose_Event msg_;
};

class Init_ResetPose_Event_request
{
public:
  explicit Init_ResetPose_Event_request(::irobot_create_msgs::srv::ResetPose_Event & msg)
  : msg_(msg)
  {}
  Init_ResetPose_Event_response request(::irobot_create_msgs::srv::ResetPose_Event::_request_type arg)
  {
    msg_.request = std::move(arg);
    return Init_ResetPose_Event_response(msg_);
  }

private:
  ::irobot_create_msgs::srv::ResetPose_Event msg_;
};

class Init_ResetPose_Event_info
{
public:
  Init_ResetPose_Event_info()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_ResetPose_Event_request info(::irobot_create_msgs::srv::ResetPose_Event::_info_type arg)
  {
    msg_.info = std::move(arg);
    return Init_ResetPose_Event_request(msg_);
  }

private:
  ::irobot_create_msgs::srv::ResetPose_Event msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::irobot_create_msgs::srv::ResetPose_Event>()
{
  return irobot_create_msgs::srv::builder::Init_ResetPose_Event_info();
}

}  // namespace irobot_create_msgs

#endif  // IROBOT_CREATE_MSGS__SRV__DETAIL__RESET_POSE__BUILDER_HPP_
