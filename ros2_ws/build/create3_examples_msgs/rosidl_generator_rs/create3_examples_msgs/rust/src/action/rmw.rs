
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "create3_examples_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_Goal() -> *const std::ffi::c_void;
}

#[link(name = "create3_examples_msgs__rosidl_generator_c")]
extern "C" {
    fn create3_examples_msgs__action__Coverage_Goal__init(msg: *mut Coverage_Goal) -> bool;
    fn create3_examples_msgs__action__Coverage_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Coverage_Goal>, size: usize) -> bool;
    fn create3_examples_msgs__action__Coverage_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Coverage_Goal>);
    fn create3_examples_msgs__action__Coverage_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Coverage_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Coverage_Goal>) -> bool;
}

// Corresponds to create3_examples_msgs__action__Coverage_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Coverage_Goal {
    /// The robot will explore the environment for `explore_duration` time.
    /// After that, it will try to dock as soon as it sees the docking station.
    /// The `max_runtime` duration defines when the behavior should end regardless of whether
    /// the dock has been found or not.
    pub explore_duration: builtin_interfaces::msg::rmw::Duration,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_runtime: builtin_interfaces::msg::rmw::Duration,

}



impl Default for Coverage_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !create3_examples_msgs__action__Coverage_Goal__init(&mut msg as *mut _) {
        panic!("Call to create3_examples_msgs__action__Coverage_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Coverage_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Coverage_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Coverage_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "create3_examples_msgs/action/Coverage_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_Goal() }
  }
}


#[link(name = "create3_examples_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_Result() -> *const std::ffi::c_void;
}

#[link(name = "create3_examples_msgs__rosidl_generator_c")]
extern "C" {
    fn create3_examples_msgs__action__Coverage_Result__init(msg: *mut Coverage_Result) -> bool;
    fn create3_examples_msgs__action__Coverage_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Coverage_Result>, size: usize) -> bool;
    fn create3_examples_msgs__action__Coverage_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Coverage_Result>);
    fn create3_examples_msgs__action__Coverage_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Coverage_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Coverage_Result>) -> bool;
}

// Corresponds to create3_examples_msgs__action__Coverage_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Coverage_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_docked: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub duration: builtin_interfaces::msg::rmw::Duration,

}



impl Default for Coverage_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !create3_examples_msgs__action__Coverage_Result__init(&mut msg as *mut _) {
        panic!("Call to create3_examples_msgs__action__Coverage_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Coverage_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Coverage_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Coverage_Result where Self: Sized {
  const TYPE_NAME: &'static str = "create3_examples_msgs/action/Coverage_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_Result() }
  }
}


#[link(name = "create3_examples_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "create3_examples_msgs__rosidl_generator_c")]
extern "C" {
    fn create3_examples_msgs__action__Coverage_Feedback__init(msg: *mut Coverage_Feedback) -> bool;
    fn create3_examples_msgs__action__Coverage_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Coverage_Feedback>, size: usize) -> bool;
    fn create3_examples_msgs__action__Coverage_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Coverage_Feedback>);
    fn create3_examples_msgs__action__Coverage_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Coverage_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Coverage_Feedback>) -> bool;
}

// Corresponds to create3_examples_msgs__action__Coverage_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Coverage_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_behavior: i32,

}

impl Coverage_Feedback {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DOCK: i32 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DRIVE_STRAIGHT: i32 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ROTATE: i32 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SPIRAL: i32 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNDOCK: i32 = 4;

}


impl Default for Coverage_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !create3_examples_msgs__action__Coverage_Feedback__init(&mut msg as *mut _) {
        panic!("Call to create3_examples_msgs__action__Coverage_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Coverage_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Coverage_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Coverage_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "create3_examples_msgs/action/Coverage_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_Feedback() }
  }
}


#[link(name = "create3_examples_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "create3_examples_msgs__rosidl_generator_c")]
extern "C" {
    fn create3_examples_msgs__action__Coverage_FeedbackMessage__init(msg: *mut Coverage_FeedbackMessage) -> bool;
    fn create3_examples_msgs__action__Coverage_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Coverage_FeedbackMessage>, size: usize) -> bool;
    fn create3_examples_msgs__action__Coverage_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Coverage_FeedbackMessage>);
    fn create3_examples_msgs__action__Coverage_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Coverage_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Coverage_FeedbackMessage>) -> bool;
}

// Corresponds to create3_examples_msgs__action__Coverage_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Coverage_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Coverage_Feedback,

}



impl Default for Coverage_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !create3_examples_msgs__action__Coverage_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to create3_examples_msgs__action__Coverage_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Coverage_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Coverage_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Coverage_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "create3_examples_msgs/action/Coverage_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_FeedbackMessage() }
  }
}




#[link(name = "create3_examples_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "create3_examples_msgs__rosidl_generator_c")]
extern "C" {
    fn create3_examples_msgs__action__Coverage_SendGoal_Request__init(msg: *mut Coverage_SendGoal_Request) -> bool;
    fn create3_examples_msgs__action__Coverage_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Coverage_SendGoal_Request>, size: usize) -> bool;
    fn create3_examples_msgs__action__Coverage_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Coverage_SendGoal_Request>);
    fn create3_examples_msgs__action__Coverage_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Coverage_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Coverage_SendGoal_Request>) -> bool;
}

// Corresponds to create3_examples_msgs__action__Coverage_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Coverage_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Coverage_Goal,

}



impl Default for Coverage_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !create3_examples_msgs__action__Coverage_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to create3_examples_msgs__action__Coverage_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Coverage_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Coverage_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Coverage_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "create3_examples_msgs/action/Coverage_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_SendGoal_Request() }
  }
}


#[link(name = "create3_examples_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "create3_examples_msgs__rosidl_generator_c")]
extern "C" {
    fn create3_examples_msgs__action__Coverage_SendGoal_Response__init(msg: *mut Coverage_SendGoal_Response) -> bool;
    fn create3_examples_msgs__action__Coverage_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Coverage_SendGoal_Response>, size: usize) -> bool;
    fn create3_examples_msgs__action__Coverage_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Coverage_SendGoal_Response>);
    fn create3_examples_msgs__action__Coverage_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Coverage_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Coverage_SendGoal_Response>) -> bool;
}

// Corresponds to create3_examples_msgs__action__Coverage_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Coverage_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Coverage_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !create3_examples_msgs__action__Coverage_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to create3_examples_msgs__action__Coverage_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Coverage_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Coverage_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Coverage_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "create3_examples_msgs/action/Coverage_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_SendGoal_Response() }
  }
}


#[link(name = "create3_examples_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "create3_examples_msgs__rosidl_generator_c")]
extern "C" {
    fn create3_examples_msgs__action__Coverage_GetResult_Request__init(msg: *mut Coverage_GetResult_Request) -> bool;
    fn create3_examples_msgs__action__Coverage_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Coverage_GetResult_Request>, size: usize) -> bool;
    fn create3_examples_msgs__action__Coverage_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Coverage_GetResult_Request>);
    fn create3_examples_msgs__action__Coverage_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Coverage_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Coverage_GetResult_Request>) -> bool;
}

// Corresponds to create3_examples_msgs__action__Coverage_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Coverage_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Coverage_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !create3_examples_msgs__action__Coverage_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to create3_examples_msgs__action__Coverage_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Coverage_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Coverage_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Coverage_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "create3_examples_msgs/action/Coverage_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_GetResult_Request() }
  }
}


#[link(name = "create3_examples_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "create3_examples_msgs__rosidl_generator_c")]
extern "C" {
    fn create3_examples_msgs__action__Coverage_GetResult_Response__init(msg: *mut Coverage_GetResult_Response) -> bool;
    fn create3_examples_msgs__action__Coverage_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Coverage_GetResult_Response>, size: usize) -> bool;
    fn create3_examples_msgs__action__Coverage_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Coverage_GetResult_Response>);
    fn create3_examples_msgs__action__Coverage_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Coverage_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Coverage_GetResult_Response>) -> bool;
}

// Corresponds to create3_examples_msgs__action__Coverage_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Coverage_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Coverage_Result,

}



impl Default for Coverage_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !create3_examples_msgs__action__Coverage_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to create3_examples_msgs__action__Coverage_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Coverage_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { create3_examples_msgs__action__Coverage_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Coverage_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Coverage_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "create3_examples_msgs/action/Coverage_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__create3_examples_msgs__action__Coverage_GetResult_Response() }
  }
}






#[link(name = "create3_examples_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__create3_examples_msgs__action__Coverage_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to create3_examples_msgs__action__Coverage_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Coverage_SendGoal;

impl rosidl_runtime_rs::Service for Coverage_SendGoal {
    type Request = Coverage_SendGoal_Request;
    type Response = Coverage_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__create3_examples_msgs__action__Coverage_SendGoal() }
    }
}




#[link(name = "create3_examples_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__create3_examples_msgs__action__Coverage_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to create3_examples_msgs__action__Coverage_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Coverage_GetResult;

impl rosidl_runtime_rs::Service for Coverage_GetResult {
    type Request = Coverage_GetResult_Request;
    type Response = Coverage_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__create3_examples_msgs__action__Coverage_GetResult() }
    }
}


