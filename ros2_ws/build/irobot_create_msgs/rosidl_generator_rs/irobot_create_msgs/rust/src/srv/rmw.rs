#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__srv__EStop_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__srv__EStop_Request__init(msg: *mut EStop_Request) -> bool;
    fn irobot_create_msgs__srv__EStop_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EStop_Request>, size: usize) -> bool;
    fn irobot_create_msgs__srv__EStop_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EStop_Request>);
    fn irobot_create_msgs__srv__EStop_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EStop_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<EStop_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__srv__EStop_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EStop_Request {
    /// true enables e-stop disabling drive motor power, false disables e-stop re-enabling motor power
    pub e_stop_on: bool,

}



impl Default for EStop_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__srv__EStop_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__srv__EStop_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EStop_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__EStop_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__EStop_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__EStop_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EStop_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EStop_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/srv/EStop_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__srv__EStop_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__srv__EStop_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__srv__EStop_Response__init(msg: *mut EStop_Response) -> bool;
    fn irobot_create_msgs__srv__EStop_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EStop_Response>, size: usize) -> bool;
    fn irobot_create_msgs__srv__EStop_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EStop_Response>);
    fn irobot_create_msgs__srv__EStop_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EStop_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<EStop_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__srv__EStop_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EStop_Response {
    /// indicate successful application of command
    pub success: bool,

    /// informational for result
    pub message: rosidl_runtime_rs::String,

}



impl Default for EStop_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__srv__EStop_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__srv__EStop_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EStop_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__EStop_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__EStop_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__EStop_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EStop_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EStop_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/srv/EStop_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__srv__EStop_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__srv__ResetPose_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__srv__ResetPose_Request__init(msg: *mut ResetPose_Request) -> bool;
    fn irobot_create_msgs__srv__ResetPose_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ResetPose_Request>, size: usize) -> bool;
    fn irobot_create_msgs__srv__ResetPose_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ResetPose_Request>);
    fn irobot_create_msgs__srv__ResetPose_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ResetPose_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ResetPose_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__srv__ResetPose_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResetPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::Pose,

}



impl Default for ResetPose_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__srv__ResetPose_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__srv__ResetPose_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ResetPose_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__ResetPose_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__ResetPose_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__ResetPose_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ResetPose_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ResetPose_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/srv/ResetPose_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__srv__ResetPose_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__srv__ResetPose_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__srv__ResetPose_Response__init(msg: *mut ResetPose_Response) -> bool;
    fn irobot_create_msgs__srv__ResetPose_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ResetPose_Response>, size: usize) -> bool;
    fn irobot_create_msgs__srv__ResetPose_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ResetPose_Response>);
    fn irobot_create_msgs__srv__ResetPose_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ResetPose_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ResetPose_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__srv__ResetPose_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ResetPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ResetPose_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__srv__ResetPose_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__srv__ResetPose_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ResetPose_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__ResetPose_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__ResetPose_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__ResetPose_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ResetPose_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ResetPose_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/srv/ResetPose_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__srv__ResetPose_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__srv__RobotPower_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__srv__RobotPower_Request__init(msg: *mut RobotPower_Request) -> bool;
    fn irobot_create_msgs__srv__RobotPower_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotPower_Request>, size: usize) -> bool;
    fn irobot_create_msgs__srv__RobotPower_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotPower_Request>);
    fn irobot_create_msgs__srv__RobotPower_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotPower_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotPower_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__srv__RobotPower_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotPower_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for RobotPower_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__srv__RobotPower_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__srv__RobotPower_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotPower_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__RobotPower_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__RobotPower_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__RobotPower_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotPower_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotPower_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/srv/RobotPower_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__srv__RobotPower_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__srv__RobotPower_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__srv__RobotPower_Response__init(msg: *mut RobotPower_Response) -> bool;
    fn irobot_create_msgs__srv__RobotPower_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RobotPower_Response>, size: usize) -> bool;
    fn irobot_create_msgs__srv__RobotPower_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RobotPower_Response>);
    fn irobot_create_msgs__srv__RobotPower_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RobotPower_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RobotPower_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__srv__RobotPower_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RobotPower_Response {
    /// indicate successful application of command, robot may power down before response is transmitted
    pub success: bool,

    /// informational for result, robot may power down before response is transmitted
    pub message: rosidl_runtime_rs::String,

}



impl Default for RobotPower_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__srv__RobotPower_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__srv__RobotPower_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RobotPower_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__RobotPower_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__RobotPower_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__srv__RobotPower_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RobotPower_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RobotPower_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/srv/RobotPower_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__srv__RobotPower_Response() }
  }
}






#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__srv__EStop() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__srv__EStop
#[allow(missing_docs, non_camel_case_types)]
pub struct EStop;

impl rosidl_runtime_rs::Service for EStop {
    type Request = EStop_Request;
    type Response = EStop_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__srv__EStop() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__srv__ResetPose() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__srv__ResetPose
#[allow(missing_docs, non_camel_case_types)]
pub struct ResetPose;

impl rosidl_runtime_rs::Service for ResetPose {
    type Request = ResetPose_Request;
    type Response = ResetPose_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__srv__ResetPose() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__srv__RobotPower() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__srv__RobotPower
#[allow(missing_docs, non_camel_case_types)]
pub struct RobotPower;

impl rosidl_runtime_rs::Service for RobotPower {
    type Request = RobotPower_Request;
    type Response = RobotPower_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__srv__RobotPower() }
    }
}


