#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__AudioNote() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__AudioNote__init(msg: *mut AudioNote) -> bool;
    fn irobot_create_msgs__msg__AudioNote__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioNote>, size: usize) -> bool;
    fn irobot_create_msgs__msg__AudioNote__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioNote>);
    fn irobot_create_msgs__msg__AudioNote__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioNote>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioNote>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__AudioNote
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Play a note (frequency) for a specific duration

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioNote {
    /// Frequency of note in Hz.
    pub frequency: u16,

    /// Time note should play for.
    pub max_runtime: builtin_interfaces::msg::rmw::Duration,

}



impl Default for AudioNote {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__AudioNote__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__AudioNote__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioNote {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__AudioNote__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__AudioNote__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__AudioNote__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioNote {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioNote where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/AudioNote";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__AudioNote() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__AudioNoteVector() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__AudioNoteVector__init(msg: *mut AudioNoteVector) -> bool;
    fn irobot_create_msgs__msg__AudioNoteVector__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteVector>, size: usize) -> bool;
    fn irobot_create_msgs__msg__AudioNoteVector__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteVector>);
    fn irobot_create_msgs__msg__AudioNoteVector__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioNoteVector>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioNoteVector>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__AudioNoteVector
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Play a sequence of audio notes.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioNoteVector {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub notes: rosidl_runtime_rs::Sequence<super::super::msg::rmw::AudioNote>,

    /// If the notes should be appended or overwrite the currently playing notes.
    pub append: bool,

}



impl Default for AudioNoteVector {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__AudioNoteVector__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__AudioNoteVector__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioNoteVector {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__AudioNoteVector__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__AudioNoteVector__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__AudioNoteVector__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioNoteVector {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioNoteVector where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/AudioNoteVector";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__AudioNoteVector() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__Button() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__Button__init(msg: *mut Button) -> bool;
    fn irobot_create_msgs__msg__Button__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Button>, size: usize) -> bool;
    fn irobot_create_msgs__msg__Button__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Button>);
    fn irobot_create_msgs__msg__Button__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Button>, out_seq: *mut rosidl_runtime_rs::Sequence<Button>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__Button
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message gives the status for a button

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Button {
    /// Header stamp is time and frame at which information was collected
    pub header: std_msgs::msg::rmw::Header,

    /// Whether or not button is depressed
    pub is_pressed: bool,

    /// The last time a button press was initiated
    pub last_start_pressed_time: builtin_interfaces::msg::rmw::Time,

    /// The duration that the button was depressed
    pub last_pressed_duration: builtin_interfaces::msg::rmw::Duration,

}



impl Default for Button {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__Button__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__Button__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Button {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__Button__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__Button__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__Button__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Button {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Button where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/Button";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__Button() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__DockStatus() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__DockStatus__init(msg: *mut DockStatus) -> bool;
    fn irobot_create_msgs__msg__DockStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DockStatus>, size: usize) -> bool;
    fn irobot_create_msgs__msg__DockStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DockStatus>);
    fn irobot_create_msgs__msg__DockStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DockStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<DockStatus>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__DockStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains information about whether the robot is docked or sees the dock in its sensors.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DockStatus {
    /// Header stamp is when dock info was queried.
    pub header: std_msgs::msg::rmw::Header,

    /// Whether robot sees dock in its sensors used for docking
    pub dock_visible: bool,

    /// Whether the robot is docked
    pub is_docked: bool,

}



impl Default for DockStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__DockStatus__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__DockStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DockStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__DockStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__DockStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__DockStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DockStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DockStatus where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/DockStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__DockStatus() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__HazardDetection() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__HazardDetection__init(msg: *mut HazardDetection) -> bool;
    fn irobot_create_msgs__msg__HazardDetection__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HazardDetection>, size: usize) -> bool;
    fn irobot_create_msgs__msg__HazardDetection__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HazardDetection>);
    fn irobot_create_msgs__msg__HazardDetection__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HazardDetection>, out_seq: *mut rosidl_runtime_rs::Sequence<HazardDetection>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__HazardDetection
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message describes a detected hazard.
/// The frame ID allows to retrieve the location of the sensor that made the detection.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HazardDetection {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: u8,

}

impl HazardDetection {
    /// The robot has reached its backup limit. It will not drive further backward for safety reasons.
    /// You can disable this limit through the Create3 "safety_override" parameter by setting it
    /// to "backup_only" or "full".
    /// The Create3 webserver can be used to set a default value for the parameter.
    pub const BACKUP_LIMIT: u8 = 0;

    /// The robot has bumped against an obstacle
    pub const BUMP: u8 = 1;

    /// The robot detected a cliff
    pub const CLIFF: u8 = 2;

    /// The wheels of the robot are stalled against an obstacle
    pub const STALL: u8 = 3;

    /// The wheels of the robot are fully dropped
    pub const WHEEL_DROP: u8 = 4;

    /// The robot detects an obstacle in close proximity
    pub const OBJECT_PROXIMITY: u8 = 5;

}


impl Default for HazardDetection {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__HazardDetection__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__HazardDetection__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HazardDetection {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__HazardDetection__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__HazardDetection__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__HazardDetection__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HazardDetection {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HazardDetection where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/HazardDetection";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__HazardDetection() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__HazardDetectionVector() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__HazardDetectionVector__init(msg: *mut HazardDetectionVector) -> bool;
    fn irobot_create_msgs__msg__HazardDetectionVector__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HazardDetectionVector>, size: usize) -> bool;
    fn irobot_create_msgs__msg__HazardDetectionVector__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HazardDetectionVector>);
    fn irobot_create_msgs__msg__HazardDetectionVector__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HazardDetectionVector>, out_seq: *mut rosidl_runtime_rs::Sequence<HazardDetectionVector>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__HazardDetectionVector
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains a vector of detected hazards.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HazardDetectionVector {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub detections: rosidl_runtime_rs::Sequence<super::super::msg::rmw::HazardDetection>,

}



impl Default for HazardDetectionVector {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__HazardDetectionVector__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__HazardDetectionVector__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HazardDetectionVector {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__HazardDetectionVector__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__HazardDetectionVector__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__HazardDetectionVector__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HazardDetectionVector {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HazardDetectionVector where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/HazardDetectionVector";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__HazardDetectionVector() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__InterfaceButtons() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__InterfaceButtons__init(msg: *mut InterfaceButtons) -> bool;
    fn irobot_create_msgs__msg__InterfaceButtons__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<InterfaceButtons>, size: usize) -> bool;
    fn irobot_create_msgs__msg__InterfaceButtons__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<InterfaceButtons>);
    fn irobot_create_msgs__msg__InterfaceButtons__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<InterfaceButtons>, out_seq: *mut rosidl_runtime_rs::Sequence<InterfaceButtons>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__InterfaceButtons
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message is the status of the 3 interface buttons on the Create faceplate

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InterfaceButtons {
    /// Header stamp is time at which information was collected
    pub header: std_msgs::msg::rmw::Header,

    /// Left button on faceplate marked with 1 dot
    pub button_1: super::super::msg::rmw::Button,

    /// Center Power button on faceplate
    pub button_power: super::super::msg::rmw::Button,

    /// Right button on faceplate marked with 2 dots
    pub button_2: super::super::msg::rmw::Button,

}



impl Default for InterfaceButtons {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__InterfaceButtons__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__InterfaceButtons__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for InterfaceButtons {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__InterfaceButtons__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__InterfaceButtons__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__InterfaceButtons__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for InterfaceButtons {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for InterfaceButtons where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/InterfaceButtons";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__InterfaceButtons() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__IrIntensity() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__IrIntensity__init(msg: *mut IrIntensity) -> bool;
    fn irobot_create_msgs__msg__IrIntensity__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IrIntensity>, size: usize) -> bool;
    fn irobot_create_msgs__msg__IrIntensity__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IrIntensity>);
    fn irobot_create_msgs__msg__IrIntensity__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IrIntensity>, out_seq: *mut rosidl_runtime_rs::Sequence<IrIntensity>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__IrIntensity
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message provides the ir intensity readings

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IrIntensity {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i16,

}



impl Default for IrIntensity {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__IrIntensity__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__IrIntensity__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IrIntensity {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__IrIntensity__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__IrIntensity__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__IrIntensity__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IrIntensity {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IrIntensity where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/IrIntensity";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__IrIntensity() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__IrIntensityVector() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__IrIntensityVector__init(msg: *mut IrIntensityVector) -> bool;
    fn irobot_create_msgs__msg__IrIntensityVector__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IrIntensityVector>, size: usize) -> bool;
    fn irobot_create_msgs__msg__IrIntensityVector__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IrIntensityVector>);
    fn irobot_create_msgs__msg__IrIntensityVector__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IrIntensityVector>, out_seq: *mut rosidl_runtime_rs::Sequence<IrIntensityVector>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__IrIntensityVector
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message provides the ir intensity readings

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IrIntensityVector {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub readings: rosidl_runtime_rs::Sequence<super::super::msg::rmw::IrIntensity>,

}



impl Default for IrIntensityVector {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__IrIntensityVector__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__IrIntensityVector__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IrIntensityVector {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__IrIntensityVector__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__IrIntensityVector__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__IrIntensityVector__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IrIntensityVector {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IrIntensityVector where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/IrIntensityVector";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__IrIntensityVector() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__IrOpcode() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__IrOpcode__init(msg: *mut IrOpcode) -> bool;
    fn irobot_create_msgs__msg__IrOpcode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<IrOpcode>, size: usize) -> bool;
    fn irobot_create_msgs__msg__IrOpcode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<IrOpcode>);
    fn irobot_create_msgs__msg__IrOpcode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<IrOpcode>, out_seq: *mut rosidl_runtime_rs::Sequence<IrOpcode>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__IrOpcode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message describes a detected ir opcode.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IrOpcode {
    /// time when we saw the opcode
    pub header: std_msgs::msg::rmw::Header,

    /// opcode
    pub opcode: u8,

    /// the sensor which saw the opcode
    pub sensor: u8,

}

impl IrOpcode {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_FORCE_FIELD: u8 = 161;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_VIRTUAL_WALL: u8 = 162;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_BUOY_GREEN: u8 = 164;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_BUOY_RED: u8 = 168;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_BUOY_BOTH: u8 = 172;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_EVAC_GREEN_FIELD: u8 = 244;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_EVAC_RED_FIELD: u8 = 248;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_EVAC_BOTH_FIELD: u8 = 252;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SENSOR_OMNI: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SENSOR_DIRECTIONAL_FRONT: u8 = 1;

}


impl Default for IrOpcode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__IrOpcode__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__IrOpcode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for IrOpcode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__IrOpcode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__IrOpcode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__IrOpcode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for IrOpcode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for IrOpcode where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/IrOpcode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__IrOpcode() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__KidnapStatus() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__KidnapStatus__init(msg: *mut KidnapStatus) -> bool;
    fn irobot_create_msgs__msg__KidnapStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<KidnapStatus>, size: usize) -> bool;
    fn irobot_create_msgs__msg__KidnapStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<KidnapStatus>);
    fn irobot_create_msgs__msg__KidnapStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<KidnapStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<KidnapStatus>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__KidnapStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message provides the robot kidnap status

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct KidnapStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_kidnapped: bool,

}



impl Default for KidnapStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__KidnapStatus__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__KidnapStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for KidnapStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__KidnapStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__KidnapStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__KidnapStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for KidnapStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for KidnapStatus where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/KidnapStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__KidnapStatus() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__LedColor() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__LedColor__init(msg: *mut LedColor) -> bool;
    fn irobot_create_msgs__msg__LedColor__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LedColor>, size: usize) -> bool;
    fn irobot_create_msgs__msg__LedColor__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LedColor>);
    fn irobot_create_msgs__msg__LedColor__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LedColor>, out_seq: *mut rosidl_runtime_rs::Sequence<LedColor>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__LedColor
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message is to set RGB values (0-255) for an LED

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LedColor {

    // This member is not documented.
    #[allow(missing_docs)]
    pub red: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub green: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub blue: u8,

}



impl Default for LedColor {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__LedColor__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__LedColor__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LedColor {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__LedColor__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__LedColor__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__LedColor__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LedColor {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LedColor where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/LedColor";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__LedColor() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__LightringLeds() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__LightringLeds__init(msg: *mut LightringLeds) -> bool;
    fn irobot_create_msgs__msg__LightringLeds__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LightringLeds>, size: usize) -> bool;
    fn irobot_create_msgs__msg__LightringLeds__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LightringLeds>);
    fn irobot_create_msgs__msg__LightringLeds__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LightringLeds>, out_seq: *mut rosidl_runtime_rs::Sequence<LightringLeds>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__LightringLeds
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message sets the RGB values of the lightright LEDs

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LightringLeds {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// If you look at the lightring as an analog clock with 12 at the front
    /// of the robot
    /// led index 0 is 4 o'clock,
    /// led index 1 is 2 o'clock,
    /// led index 2 is 12 o'clock,
    /// led index 3 is 10 o'clock,
    /// led index 4 is 8 o'clock,
    /// led index 5 is 6 o'clock
    pub leds: [super::super::msg::rmw::LedColor; 6],

    /// Whether to override system's default light patterns with given command
    /// Set to false to return control of lights to system
    pub override_system: bool,

}



impl Default for LightringLeds {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__LightringLeds__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__LightringLeds__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LightringLeds {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__LightringLeds__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__LightringLeds__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__LightringLeds__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LightringLeds {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LightringLeds where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/LightringLeds";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__LightringLeds() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__Mouse() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__Mouse__init(msg: *mut Mouse) -> bool;
    fn irobot_create_msgs__msg__Mouse__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Mouse>, size: usize) -> bool;
    fn irobot_create_msgs__msg__Mouse__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Mouse>);
    fn irobot_create_msgs__msg__Mouse__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Mouse>, out_seq: *mut rosidl_runtime_rs::Sequence<Mouse>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__Mouse
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains a mouse raw measurement.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Mouse {
    /// Header stamp should be acquisition time of mouse measure.
    pub header: std_msgs::msg::rmw::Header,

    /// Motions in the X dimension in meters.
    pub integrated_x: f32,

    /// Motions in the Y dimension in meters.
    pub integrated_y: f32,

    /// Value incremented every time mouse stops tracking
    pub frame_id: u32,

    /// Surface quality level. It's proportional to the number of valid features visible by the sensor.
    pub last_squal: u8,

}



impl Default for Mouse {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__Mouse__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__Mouse__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Mouse {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__Mouse__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__Mouse__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__Mouse__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Mouse {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Mouse where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/Mouse";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__Mouse() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__SlipStatus() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__SlipStatus__init(msg: *mut SlipStatus) -> bool;
    fn irobot_create_msgs__msg__SlipStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SlipStatus>, size: usize) -> bool;
    fn irobot_create_msgs__msg__SlipStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SlipStatus>);
    fn irobot_create_msgs__msg__SlipStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SlipStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<SlipStatus>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__SlipStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message provides the robot slip status

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SlipStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_slipping: bool,

}



impl Default for SlipStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__SlipStatus__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__SlipStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SlipStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__SlipStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__SlipStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__SlipStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SlipStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SlipStatus where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/SlipStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__SlipStatus() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__StopStatus() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__StopStatus__init(msg: *mut StopStatus) -> bool;
    fn irobot_create_msgs__msg__StopStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StopStatus>, size: usize) -> bool;
    fn irobot_create_msgs__msg__StopStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StopStatus>);
    fn irobot_create_msgs__msg__StopStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StopStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<StopStatus>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__StopStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message provides the robot stopped status

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StopStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_stopped: bool,

}



impl Default for StopStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__StopStatus__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__StopStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StopStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__StopStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__StopStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__StopStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StopStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StopStatus where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/StopStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__StopStatus() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__WheelStatus() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__WheelStatus__init(msg: *mut WheelStatus) -> bool;
    fn irobot_create_msgs__msg__WheelStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WheelStatus>, size: usize) -> bool;
    fn irobot_create_msgs__msg__WheelStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WheelStatus>);
    fn irobot_create_msgs__msg__WheelStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WheelStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<WheelStatus>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__WheelStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains status on the wheels

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WheelStatus {
    /// Header stamp should be acquisition time of measure.
    pub header: std_msgs::msg::rmw::Header,

    /// Current measurement for left wheel in milliamps
    pub current_ma_left: i16,

    /// Current measurement for right wheel in milliamps
    pub current_ma_right: i16,

    /// PWM % duty cycle measurement (where int16::max is +100%) for left wheel
    pub pwm_left: i16,

    /// PWM % duty cycle measurement (where int16::max is +100%) for right wheel
    pub pwm_right: i16,

    /// Whether wheels are enabled or disabled (disabled when E-Stopped)
    pub wheels_enabled: bool,

}



impl Default for WheelStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__WheelStatus__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__WheelStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WheelStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__WheelStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__WheelStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__WheelStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WheelStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WheelStatus where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/WheelStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__WheelStatus() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__WheelTicks() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__WheelTicks__init(msg: *mut WheelTicks) -> bool;
    fn irobot_create_msgs__msg__WheelTicks__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WheelTicks>, size: usize) -> bool;
    fn irobot_create_msgs__msg__WheelTicks__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WheelTicks>);
    fn irobot_create_msgs__msg__WheelTicks__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WheelTicks>, out_seq: *mut rosidl_runtime_rs::Sequence<WheelTicks>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__WheelTicks
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains wheel ticks raw measurements.
/// It can be used both for publishing integrated as well as absolute encoder readings.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WheelTicks {
    /// Header stamp should be acquisition time of measure.
    pub header: std_msgs::msg::rmw::Header,

    /// Count of encoder ticks for left wheel.
    pub ticks_left: i32,

    /// Count of encoder ticks for right wheel.
    pub ticks_right: i32,

}



impl Default for WheelTicks {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__WheelTicks__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__WheelTicks__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WheelTicks {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__WheelTicks__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__WheelTicks__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__WheelTicks__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WheelTicks {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WheelTicks where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/WheelTicks";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__WheelTicks() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__WheelVels() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__msg__WheelVels__init(msg: *mut WheelVels) -> bool;
    fn irobot_create_msgs__msg__WheelVels__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WheelVels>, size: usize) -> bool;
    fn irobot_create_msgs__msg__WheelVels__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WheelVels>);
    fn irobot_create_msgs__msg__WheelVels__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WheelVels>, out_seq: *mut rosidl_runtime_rs::Sequence<WheelVels>) -> bool;
}

// Corresponds to irobot_create_msgs__msg__WheelVels
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains wheel velocity measurements.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WheelVels {
    /// Header stamp should be acquisition time of measure.
    pub header: std_msgs::msg::rmw::Header,

    /// Velocity measure for left wheel in rad/sec.
    pub velocity_left: f32,

    /// Velocity measure for right wheel in rad/sec.
    pub velocity_right: f32,

}



impl Default for WheelVels {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__msg__WheelVels__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__msg__WheelVels__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WheelVels {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__WheelVels__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__WheelVels__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__msg__WheelVels__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WheelVels {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WheelVels where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/msg/WheelVels";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__msg__WheelVels() }
  }
}


