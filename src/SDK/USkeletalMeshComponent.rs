use super::Structs::{FTransform::FTransform, FVector::FVector, TArray::TArray};

#[struct_macro::inherit(super::USkinnedMeshComponent::USkinnedMeshComponent, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct USkeletalMeshComponent {
    #[offset(0x0AB8)]
    pub CachedComponentSpaceTransforms: TArray<FTransform,>,
}

impl USkeletalMeshComponent {
    #[inline]
    pub unsafe fn get_bone_location(&self, bone_idx: i32,) -> Option<FVector,> {
        let bone_array = &self.CachedComponentSpaceTransforms;

        if bone_array.Data.is_null()
        {
            return None;
        }

        let bone_local_ptr = bone_array.Data.add(bone_idx as usize,);
        if bone_local_ptr.is_null()
        {
            return None;
        }

        let bone_local = bone_local_ptr.as_ref_unchecked();

        let c2w = &self.ComponentToWorld;

        let v = FVector {
            X: bone_local.Translation.X * c2w.Scale3D.X,
            Y: bone_local.Translation.Y * c2w.Scale3D.Y,
            Z: bone_local.Translation.Z * c2w.Scale3D.Z,
        };

        let q = &c2w.Rotation;

        let t_x = 2.0 * (q.Y * v.Z - q.Z * v.Y);
        let t_y = 2.0 * (q.Z * v.X - q.X * v.Z);
        let t_z = 2.0 * (q.X * v.Y - q.Y * v.X);

        let rotated_x = v.X + q.W * t_x + (q.Y * t_z - q.Z * t_y);
        let rotated_y = v.Y + q.W * t_y + (q.Z * t_x - q.X * t_z);
        let rotated_z = v.Z + q.W * t_z + (q.X * t_y - q.Y * t_x);

        Some(FVector {
            X: rotated_x + c2w.Translation.X,
            Y: rotated_y + c2w.Translation.Y,
            Z: rotated_z + c2w.Translation.Z,
        },)
    }
}
