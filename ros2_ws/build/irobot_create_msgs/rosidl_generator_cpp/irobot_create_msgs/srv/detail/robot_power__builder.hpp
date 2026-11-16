// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from irobot_create_msgs:srv/RobotPower.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "irobot_create_msgs/srv/robot_power.hpp"


#ifndef IROBOT_CREATE_MSGS__SRV__DETAIL__ROBOT_POWER__BUILDER_HPP_
#define IROBOT_CREATE_MSGS__SRV__DETAIL__ROBOT_POWER__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "irobot_create_msgs/srv/detail/robot_power__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace irobot_create_msgs
{

namespace srv
{


}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::irobot_create_msgs::srv::RobotPower_Request>()
{
  return ::irobot_create_msgs::srv::RobotPower_Request(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace irobot_create_msgs


namespace irobot_create_msgs
{

namespace srv
{

namespace builder
{

class Init_RobotPower_Response_message
{
public:
  explicit Init_RobotPower_Response_message(::irobot_create_msgs::srv::RobotPower_Response & msg)
  : msg_(msg)
  {}
  ::irobot_create_msgs::srv::RobotPower_Response message(::irobot_create_msgs::srv::RobotPower_Response::_message_type arg)
  {
    msg_.message = std::move(arg);
    return std::move(msg_);
  }

private:
  ::irobot_create_msgs::srv::RobotPower_Response msg_;
};

class Init_RobotPower_Response_success
{
public:
  Init_RobotPower_Response_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_RobotPower_Response_message success(::irobot_create_msgs::srv::RobotPower_Response::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_RobotPower_Response_message(msg_);
  }

private:
  ::irobot_create_msgs::srv::RobotPower_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::irobot_create_msgs::srv::RobotPower_Response>()
{
  return irobot_create_msgs::srv::builder::Init_RobotPower_Response_success();
}

}  // namespace irobot_create_msgs


namespace irobot_create_msgs
{

namespace srv
{

namespace builder
{

class Init_RobotPower_Event_response
{
public:
  explicit Init_RobotPower_Event_response(::irobot_create_msgs::srv::RobotPower_Event & msg)
  : msg_(msg)
  {}
  ::irobot_create_msgs::srv::RobotPower_Event response(::irobot_create_msgs::srv::RobotPower_Event::_response_type arg)
  {
    msg_.response = std::move(arg);
    return std::move(msg_);
  }

private:
  ::irobot_create_msgs::srv::RobotPower_Event msg_;
};

class Init_RobotPower_Event_request
{
public:
  explicit Init_RobotPower_Event_request(::irobot_create_msgs::srv::RobotPower_Event & msg)
  : msg_(msg)
  {}
  Init_RobotPower_Event_response request(::irobot_create_msgs::srv::RobotPower_Event::_request_type arg)
  {
    msg_.request = std::move(arg);
    return Init_RobotPower_Event_response(msg_);
  }

private:
  ::irobot_create_msgs::srv::RobotPower_Event msg_;
};

class Init_RobotPower_Event_info
{
public:
  Init_RobotPower_Event_info()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_RobotPower_Event_request info(::irobot_create_msgs::srv::RobotPower_Event::_info_type arg)
  {
    msg_.info = std::move(arg);
    return Init_RobotPower_Event_request(msg_);
  }

private:
  ::irobot_create_msgs::srv::RobotPower_Event msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::irobot_create_msgs::srv::RobotPower_Event>()
{
  return irobot_create_msgs::srv::builder::Init_RobotPower_Event_info();
}

}  // namespace irobot_create_msgs

#endif  // IROBOT_CREATE_MSGS__SRV__DETAIL__ROBOT_POWER__BUILDER_HPP_
