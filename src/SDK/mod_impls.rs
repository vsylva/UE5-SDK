use super::{
    FMinimalViewInfo::FMinimalViewInfo,
    Structs::{FVector::FVector, FVector2D::FVector2D},
};

#[inline]
pub unsafe fn find_property_offset(ustruct: *mut super::UStruct::UStruct, name: &str,) -> Option<i32,> {
    if ustruct.is_null()
    {
        return None;
    }

    let mut current: *mut super::FField::FField = (*ustruct).ChildProperties;

    while !current.is_null()
    {
        let field = &*current;

        println!("{} {}", (*(current as *mut super::FProperty::FProperty)).Offset, field.Name.to_string());

        if field.Name.to_string() == name
        {
            let prop = current as *mut super::FProperty::FProperty;
            return Some((*prop).Offset,);
        }

        current = field.Next;
    }

    None
}

#[inline]
pub unsafe fn find_property_offset_in_hierarchy(ustruct: *mut super::UStruct::UStruct, name: &str,) -> Option<i32,> {
    let mut current_struct = ustruct;

    while !current_struct.is_null()
    {
        if let Some(offset,) = find_property_offset(current_struct, name,)
        {
            return Some(offset,);
        }

        current_struct = (*current_struct).SuperStruct as *mut super::UStruct::UStruct;
    }

    None
}

#[inline]
pub unsafe fn distance_sq(pos1: FVector, pos2: FVector,) -> f64 {
    use ::core::arch::x86_64::*;

    let a_xy = _mm_set_pd(pos1.Y, pos1.X,);
    let b_xy = _mm_set_pd(pos2.Y, pos2.X,);
    let diff_xy = _mm_sub_pd(b_xy, a_xy,);
    let sq_xy = _mm_mul_pd(diff_xy, diff_xy,);

    let hadd = _mm_hadd_pd(sq_xy, sq_xy,);
    let sum_xy = _mm_cvtsd_f64(hadd,);

    let dz = pos2.Z - pos1.Z;
    sum_xy + dz * dz
}

#[inline]
pub unsafe fn distance_cm(pos1: FVector, pos2: FVector,) -> f64 {
    distance_sq(pos1, pos2,).sqrt()
}

#[inline]
pub unsafe fn distance_m(pos1: FVector, pos2: FVector,) -> f64 {
    distance_cm(pos1, pos2,) * 0.01
}

#[inline]
pub unsafe fn world_to_screen(pov: FMinimalViewInfo, world_location: FVector,) -> Option<FVector2D,> {
    use ::core::arch::x86_64::*;

    let screen_size: FVector2D = super::Globals::SCREEN_SIZE;

    let deg_to_rad = 0.017453;
    let (sp, cp,) = (pov.rotation.Pitch * deg_to_rad).sin_cos();
    let (sy, cy,) = (pov.rotation.Yaw * deg_to_rad).sin_cos();
    let (sr, cr,) = (pov.rotation.Roll * deg_to_rad).sin_cos();

    let half_fov_tan = (pov.fov * 0.008726).tan();
    let h_w = screen_size.X * 0.5;
    let h_h = screen_size.Y * 0.5;
    let sz_factor = h_w / half_fov_tan as f64;

    let v_delta_xy = _mm_set_pd(world_location.Y - pov.location.Y, world_location.X - pov.location.X,);
    let v_delta_z = _mm_set_sd(world_location.Z - pov.location.Z,);

    const MASK: i32 = 0x31;

    let z_cam_v = _mm_add_sd(
        _mm_dp_pd(v_delta_xy, _mm_set_pd(cp * sy, cp * cy,), MASK,),
        _mm_mul_sd(v_delta_z, _mm_set_sd(sp,),),
    );
    let z_cam = _mm_cvtsd_f64(z_cam_v,);

    if z_cam < 0.01
    {
        return None;
    }

    let x_cam_v = _mm_add_sd(
        _mm_dp_pd(v_delta_xy, _mm_set_pd(sr * sp * sy + cr * cy, sr * sp * cy - cr * sy,), MASK,),
        _mm_mul_sd(v_delta_z, _mm_set_sd(-sr * cp,),),
    );
    let y_cam_v = _mm_add_sd(
        _mm_dp_pd(v_delta_xy, _mm_set_pd(cr * cy * sr - cr * sp * sy, -(cr * sp * cy + sr * sy),), MASK,),
        _mm_mul_sd(v_delta_z, _mm_set_sd(cr * cp,),),
    );

    let v_cam_xy = _mm_unpacklo_pd(x_cam_v, y_cam_v,);
    let v_z_cam = _mm_set1_pd(z_cam,);
    let v_sz = _mm_set1_pd(sz_factor,);

    let v_offset = _mm_mul_pd(_mm_div_pd(v_cam_xy, v_z_cam,), v_sz,);

    let v_screen_half = _mm_set_pd(h_h, h_w,);
    let v_op_mask = _mm_set_pd(-1.0, 1.0,);
    let v_result = _mm_add_pd(v_screen_half, _mm_mul_pd(v_offset, v_op_mask,),);

    let v_min = _mm_setzero_pd();
    let v_max = _mm_set_pd(screen_size.Y, screen_size.X,);

    let in_bounds = _mm_and_pd(_mm_cmpge_pd(v_result, v_min,), _mm_cmple_pd(v_result, v_max,),);

    if _mm_movemask_pd(in_bounds,) != 3
    {
        return None;
    }

    let mut out_screen_pos: FVector2D = FVector2D::zero();
    _mm_storel_pd(&mut out_screen_pos.X, v_result,);
    _mm_storeh_pd(&mut out_screen_pos.Y, v_result,);

    Some(out_screen_pos,)
}

#[inline]
pub unsafe fn world_to_screen_clip(
    pov: FMinimalViewInfo,
    start: FVector,
    end: FVector,
) -> Option<(FVector2D, FVector2D,),> {
    use ::core::arch::x86_64::*;

    let screen_size: FVector2D = super::Globals::SCREEN_SIZE;

    let rad = 0.017453;
    let (sp, cp,) = (pov.rotation.Pitch * rad).sin_cos();
    let (sy, cy,) = (pov.rotation.Yaw * rad).sin_cos();
    let (sr, cr,) = (pov.rotation.Roll * rad).sin_cos();

    let half_fov_tan = (pov.fov * 0.008726).tan();
    let h_w = screen_size.X * 0.5;
    let h_h = screen_size.Y * 0.5;
    let sz_factor = h_w / half_fov_tan as f64;

    const NEAR: f64 = 0.01;
    const MASK: i32 = 0x31;

    let calc_z = |loc: &FVector| -> f64 {
        let v_delta_xy = _mm_set_pd(loc.Y - pov.location.Y, loc.X - pov.location.X,);
        let v_delta_z = _mm_set_sd(loc.Z - pov.location.Z,);
        _mm_cvtsd_f64(_mm_add_sd(
            _mm_dp_pd(v_delta_xy, _mm_set_pd(cp * sy, cp * cy,), MASK,),
            _mm_mul_sd(v_delta_z, _mm_set_sd(sp,),),
        ),)
    };

    let start_z = calc_z(&start,);
    let end_z = calc_z(&end,);

    if start_z < NEAR && end_z < NEAR
    {
        return None;
    }

    let lerp = |a: FVector, b: FVector, t: f64| FVector {
        X: a.X + (b.X - a.X) * t,
        Y: a.Y + (b.Y - a.Y) * t,
        Z: a.Z + (b.Z - a.Z) * t,
    };

    let clipped_start = if start_z < NEAR { lerp(start, end, (NEAR - start_z) / (end_z - start_z),) } else { start };

    let clipped_end = if end_z < NEAR { lerp(end, start, (NEAR - end_z) / (start_z - end_z),) } else { end };

    let project = |loc: FVector| -> FVector2D {
        let mut out = FVector2D::zero();
        let v_delta_xy = _mm_set_pd(loc.Y - pov.location.Y, loc.X - pov.location.X,);
        let v_delta_z = _mm_set_sd(loc.Z - pov.location.Z,);

        let z_cam = _mm_cvtsd_f64(_mm_add_sd(
            _mm_dp_pd(v_delta_xy, _mm_set_pd(cp * sy, cp * cy,), MASK,),
            _mm_mul_sd(v_delta_z, _mm_set_sd(sp,),),
        ),);

        let x_cam_v = _mm_add_sd(
            _mm_dp_pd(v_delta_xy, _mm_set_pd(sr * sp * sy + cr * cy, sr * sp * cy - cr * sy,), MASK,),
            _mm_mul_sd(v_delta_z, _mm_set_sd(-sr * cp,),),
        );
        let y_cam_v = _mm_add_sd(
            _mm_dp_pd(v_delta_xy, _mm_set_pd(cr * cy * sr - cr * sp * sy, -(cr * sp * cy + sr * sy),), MASK,),
            _mm_mul_sd(v_delta_z, _mm_set_sd(cr * cp,),),
        );

        let v_cam_xy = _mm_unpacklo_pd(x_cam_v, y_cam_v,);
        let v_offset = _mm_mul_pd(_mm_div_pd(v_cam_xy, _mm_set1_pd(z_cam,),), _mm_set1_pd(sz_factor,),);
        let v_result = _mm_add_pd(_mm_set_pd(h_h, h_w,), _mm_mul_pd(v_offset, _mm_set_pd(-1.0, 1.0,),),);

        _mm_storel_pd(&mut out.X, v_result,);
        _mm_storeh_pd(&mut out.Y, v_result,);
        out
    };

    Some((project(clipped_start,), project(clipped_end,),),)
}
