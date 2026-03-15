use ::core::ffi::c_void;

use super::{
    ACharacter::ACharacter,
    Structs::{FName::FName, FQuat::FQuat, FRotator::FRotator, FVector::FVector, TWeakObjectPtr::TWeakObjectPtr},
    UPrimitiveComponent::UPrimitiveComponent,
    USceneComponent::USceneComponent,
};

#[derive(Debug,)]
#[repr(C)]
pub struct FSavedMove_Character {
    pub vtable: *mut c_void,

    pub character_owner: *mut ACharacter,

    pub pad:           u8,
    pub movement_mode: u8,

    pub start_packed_movement_mode: u8,
    pub end_packed_movement_mode:   u8,

    pub time_stamp:                f32,
    pub delta_time:                f32,
    pub custom_time_dilation:      f32,
    pub jump_key_hold_time:        f32,
    pub jump_force_time_remaining: f32,
    pub jump_max_count:            i32,
    pub jump_current_count:        i32,

    pub start_location: FVector,
    pub start_relative_location: FVector,
    pub start_velocity: FVector,
    pub start_floor: [u8; 0x0118],
    pub start_rotation: FRotator,
    pub start_control_rotation: FRotator,
    pub start_base_rotation: FQuat,
    pub start_capsule_radius: f32,
    pub start_capsule_half_height: f32,
    pub start_base: TWeakObjectPtr<UPrimitiveComponent,>,
    pub start_bone_name: FName,
    pub start_actor_overlap_counter: u32,
    pub start_component_overlap_counter: u32,
    pub start_attach_parent: TWeakObjectPtr<USceneComponent,>,
    pub start_attach_socket_name: FName,
    pub start_attach_relative_location: FVector,
    pub start_attach_relative_rotation: FRotator,

    pub saved_location: FVector,
    pub saved_rotation: FRotator,
    pub saved_velocity: FVector,
    pub saved_relative_location: FVector,
    pub saved_relative_acceleration: FVector,
    pub saved_control_rotation: FRotator,
    pub end_base: TWeakObjectPtr<UPrimitiveComponent,>,
    pub end_bone_name: FName,
    pub end_actor_overlap_counter: u32,
    pub end_component_overlap_counter: u32,
    pub end_attach_parent: TWeakObjectPtr<USceneComponent,>,
    pub end_attach_socket_name: FName,
    pub end_attach_relative_location: FVector,
    pub end_attach_relative_rotation: FRotator,

    pub acceleration: FVector,
    pub max_speed:    f32,
    pub accel_normal: FVector,
    pub accel_mag:    f32,

    pub root_motion_montage:                 TWeakObjectPtr<[u8; 0x0148],>,
    pub root_motion_track_position:          f32,
    pub root_motion_previous_track_position: f32,
    pub root_motion_play_rate_with_scale:    f32,
    pub root_motion_movement:                [u8; 0x0070],
    pub saved_root_motion:                   [u8; 0x0048],

    pub accel_dot_threshold:         f32,
    pub accel_mag_threshold:         f32,
    pub accel_dot_threshold_combine: f32,
    pub max_speed_threshold_combine: f32,

    _padding_to_1104: [u8; 16],
}
