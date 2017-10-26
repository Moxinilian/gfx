use ash::vk;
use hal::{buffer, format, image, pass, pso, state};
use hal::command::{ClearColor, ClearDepthStencil, ClearValue, Offset};
use hal::device::Extent;
use hal::{IndexType, Primitive};
use std::ops::Range;


pub fn map_format(surface: format::SurfaceType, chan: format::ChannelType) -> Option<vk::Format> {
    use hal::format::SurfaceType::*;
    use hal::format::ChannelType::*;
    Some(match surface {
        R4_G4 => match chan {
            Unorm => vk::Format::R4g4UnormPack8,
            _ => return None,
        },
        R4_G4_B4_A4 => match chan {
            Unorm => vk::Format::R4g4b4a4UnormPack16,
            _ => return None,
        },
        R5_G5_B5_A1 => match chan {
            Unorm => vk::Format::R5g5b5a1UnormPack16,
             _ => return None,
        },
        R5_G6_B5 => match chan {
            Unorm => vk::Format::R5g6b5UnormPack16,
             _ => return None,
        },
        R8 => match chan {
            Int   => vk::Format::R8Sint,
            Uint  => vk::Format::R8Uint,
            Inorm => vk::Format::R8Snorm,
            Unorm => vk::Format::R8Unorm,
            Srgb  => vk::Format::R8Srgb,
            _ => return None,
        },
        R8_G8 => match chan {
            Int   => vk::Format::R8g8Sint,
            Uint  => vk::Format::R8g8Uint,
            Inorm => vk::Format::R8g8Snorm,
            Unorm => vk::Format::R8g8Unorm,
            Srgb  => vk::Format::R8g8Srgb,
            _ => return None,
        },
        R8_G8_B8_A8 => match chan {
            Int   => vk::Format::R8g8b8a8Sint,
            Uint  => vk::Format::R8g8b8a8Uint,
            Inorm => vk::Format::R8g8b8a8Snorm,
            Unorm => vk::Format::R8g8b8a8Unorm,
            Srgb  => vk::Format::R8g8b8a8Srgb,
            _ => return None,
        },
        R10_G10_B10_A2 => match chan {
            Int   => vk::Format::A2r10g10b10SintPack32,
            Uint  => vk::Format::A2r10g10b10UintPack32,
            Inorm => vk::Format::A2r10g10b10SnormPack32,
            Unorm => vk::Format::A2r10g10b10UnormPack32,
            _ => return None,
        },
        R11_G11_B10 => match chan {
            Float => vk::Format::B10g11r11UfloatPack32,
            _ => return None,
        },
        R16 => match chan {
            Int   => vk::Format::R16Sint,
            Uint  => vk::Format::R16Uint,
            Inorm => vk::Format::R16Snorm,
            Unorm => vk::Format::R16Unorm,
            Float => vk::Format::R16Sfloat,
            _ => return None,
        },
        R16_G16 => match chan {
            Int   => vk::Format::R16g16Sint,
            Uint  => vk::Format::R16g16Uint,
            Inorm => vk::Format::R16g16Snorm,
            Unorm => vk::Format::R16g16Unorm,
            Float => vk::Format::R16g16Sfloat,
            _ => return None,
        },
        R16_G16_B16 => match chan {
            Int   => vk::Format::R16g16b16Sint,
            Uint  => vk::Format::R16g16b16Uint,
            Inorm => vk::Format::R16g16b16Snorm,
            Unorm => vk::Format::R16g16b16Unorm,
            Float => vk::Format::R16g16b16Sfloat,
            _ => return None,
        },
        R16_G16_B16_A16 => match chan {
            Int   => vk::Format::R16g16b16a16Sint,
            Uint  => vk::Format::R16g16b16a16Uint,
            Inorm => vk::Format::R16g16b16a16Snorm,
            Unorm => vk::Format::R16g16b16a16Unorm,
            Float => vk::Format::R16g16b16a16Sfloat,
            _ => return None,
        },
        R32 => match chan {
            Int   => vk::Format::R32Sint,
            Uint  => vk::Format::R32Uint,
            Float => vk::Format::R32Sfloat,
            _ => return None,
        },
        R32_G32 => match chan {
            Int   => vk::Format::R32g32Sint,
            Uint  => vk::Format::R32g32Uint,
            Float => vk::Format::R32g32Sfloat,
            _ => return None,
        },
        R32_G32_B32 => match chan {
            Int   => vk::Format::R32g32b32Sint,
            Uint  => vk::Format::R32g32b32Uint,
            Float => vk::Format::R32g32b32Sfloat,
            _ => return None,
        },
        R32_G32_B32_A32 => match chan {
            Int   => vk::Format::R32g32b32a32Sint,
            Uint  => vk::Format::R32g32b32a32Uint,
            Float => vk::Format::R32g32b32a32Sfloat,
            _ => return None,
        },
        B8_G8_R8_A8 => match chan {
            Int   => vk::Format::B8g8r8a8Sint,
            Uint  => vk::Format::B8g8r8a8Uint,
            Inorm => vk::Format::B8g8r8a8Snorm,
            Unorm => vk::Format::B8g8r8a8Unorm,
            Srgb  => vk::Format::B8g8r8a8Srgb,
            _ => return None,
        },
        D16 => match chan {
            Unorm  => vk::Format::D16Unorm,
            _ => return None,
        },
        D24 => match chan {
            Unorm => vk::Format::X8D24UnormPack32,
            _ => return None,
        },
        D24_S8 => match chan {
            Unorm => vk::Format::D24UnormS8Uint,
            _ => return None,
        },
        D32 => match chan {
            Float => vk::Format::D32Sfloat,
            _ => return None,
        },
        D32_S8 => match chan {
            Float => vk::Format::D32SfloatS8Uint,
            _ => return None,
        },
    })
}

pub fn map_vk_format(
    fmt: vk::Format
) -> Option<(format::SurfaceType, format::ChannelType)> {
    use ash::vk::Format::*;
    use hal::format::SurfaceType::*;
    use hal::format::ChannelType::*;
    Some(match fmt {
        R4g4UnormPack8 => (R4_G4, Unorm),
        R4g4b4a4UnormPack16 => (R4_G4_B4_A4, Unorm),
        // B4g4r4a4UnormPack16 => (B4_G4_R4_A4, Unorm),
        R5g6b5UnormPack16 => (R5_G6_B5, Unorm),
        // B5g6r5UnormPack16 => (B5_G6_R5, Unorm),
        R5g5b5a1UnormPack16 => (R5_G5_B5_A1, Unorm),
        // B5g5r5a1UnormPack16 => (B5_G5_R5_A1, Unorm),
        // A1r5g5b5UnormPack16 => (A1_R5_G5_B5, Unorm),
        R8Unorm => (R8, Unorm),
        R8Snorm => (R8, Inorm),
        // R8Uscaled => (R8, Unorm),
        // R8Sscaled => (R8, Unorm),
        R8Uint => (R8, Uint),
        R8Sint => (R8, Int),
        R8Srgb => (R8, Srgb),
        R8g8Unorm => (R8_G8, Unorm),
        R8g8Snorm => (R8_G8, Inorm),
        // R8g8Uscaled => (R8_G8, Unorm),
        // R8g8Sscaled => (R8_G8, Unorm),
        R8g8Uint => (R8_G8, Uint),
        R8g8Sint => (R8_G8, Int),
        R8g8Srgb => (R8_G8, Srgb),
        // R8g8b8Unorm => (R8_G8_B8, Unorm),
        // R8g8b8Snorm => (R8_G8_B8, Inorm),
        // R8g8b8Uscaled => (R8_G8_B8, Unorm),
        // R8g8b8Sscaled => (R8_G8_B8, Unorm),
        // R8g8b8Uint => (R8_G8_B8, Uint),
        // R8g8b8Sint => (R8_G8_B8, Int),
        // R8g8b8Srgb => (R8_G8_B8, Srgb),
        // B8g8r8Unorm => (B8_G8_R8, Unorm),
        // B8g8r8Snorm => (B8_G8_R8, Inorm),
        // B8g8r8Uscaled => (B8_G8_R8, Unorm),
        // B8g8r8Sscaled => (B8_G8_R8, Unorm),
        // B8g8r8Uint => (B8_G8_R8, Uint),
        // B8g8r8Sint => (B8_G8_R8, Int),
        // B8g8r8Srgb => (B8_G8_R8, Srgb),
        R8g8b8a8Unorm => (R8_G8_B8_A8, Unorm),
        R8g8b8a8Snorm => (R8_G8_B8_A8, Inorm),
        // R8g8b8a8Uscaled => (R8_G8_B8_A8, Unorm),
        // R8g8b8a8Sscaled => (R8_G8_B8_A8, Unorm),
        R8g8b8a8Uint => (R8_G8_B8_A8, Uint),
        R8g8b8a8Sint => (R8_G8_B8_A8, Int),
        R8g8b8a8Srgb => (R8_G8_B8_A8, Srgb),
        B8g8r8a8Unorm => (B8_G8_R8_A8, Unorm),
        B8g8r8a8Snorm => (B8_G8_R8_A8, Inorm),
        // B8g8r8a8Uscaled => (B8_G8_R8_A8, Unorm),
        // B8g8r8a8Sscaled => (B8_G8_R8_A8, Unorm),
        B8g8r8a8Uint => (B8_G8_R8_A8, Uint),
        B8g8r8a8Sint => (B8_G8_R8_A8, Int),
        B8g8r8a8Srgb => (B8_G8_R8_A8, Srgb),
        // A8b8g8r8UnormPack32 => (A8_B8_G8_R8, Unorm),
        // A8b8g8r8SnormPack32 => (A8_B8_G8_R8, Inorm),
        // A8b8g8r8UscaledPack32 => (A8_B8_G8_R8, Unorm),
        // A8b8g8r8SscaledPack32 => (A8_B8_G8_R8, Unorm),
        // A8b8g8r8UintPack32 => (A8_B8_G8_R8, Uint),
        // A8b8g8r8SintPack32 => (A8_B8_G8_R8, Int),
        // A8b8g8r8SrgbPack32 => (A8_B8_G8_R8, Srgb),
        // A2r10g10b10UnormPack32 => (A2_R10_G10_B10, Unorm),
        // A2r10g10b10SnormPack32 => (A2_R10_G10_B10, Inorm),
        // A2r10g10b10UscaledPack32 => (A2_R10_G10_B10, Unorm),
        // A2r10g10b10SscaledPack32 => (A2_R10_G10_B10, Unorm),
        // A2r10g10b10UintPack32 => (A2_R10_G10_B10, Uint),
        // A2r10g10b10SintPack32 => (A2_R10_G10_B10, Int),
        // A2b10g10r10UnormPack32 => (A2_B10_G10_R10, Unorm),
        // A2b10g10r10SnormPack32 => (A2_B10_G10_R10, Inorm),
        // A2b10g10r10UscaledPack32 => (A2_B10_G10_R10, Unorm),
        // A2b10g10r10SscaledPack32 => (A2_B10_G10_R10, Unorm),
        // A2b10g10r10UintPack32 => (A2_B10_G10_R10, Uint),
        // A2b10g10r10SintPack32 => (A2_B10_G10_R10, Int),
        R16Unorm => (R16, Unorm),
        R16Snorm => (R16, Inorm),
        // R16Uscaled => (R16, Unorm),
        // R16Sscaled => (R16, Unorm),
        R16Uint => (R16, Uint),
        R16Sint => (R16, Int),
        R16Sfloat => (R16, Float),
        R16g16Unorm => (R16_G16, Unorm),
        R16g16Snorm => (R16_G16, Inorm),
        // R16g16Uscaled => (R16_G16, Unorm),
        // R16g16Sscaled => (R16_G16, Unorm),
        R16g16Uint => (R16_G16, Uint),
        R16g16Sint => (R16_G16, Int),
        R16g16Sfloat => (R16_G16, Float),
        R16g16b16Unorm => (R16_G16_B16, Unorm),
        R16g16b16Snorm => (R16_G16_B16, Inorm),
        // R16g16b16Uscaled => (R16_G16_B16, Unorm),
        // R16g16b16Sscaled => (R16_G16_B16, Unorm),
        R16g16b16Uint => (R16_G16_B16, Uint),
        R16g16b16Sint => (R16_G16_B16, Int),
        R16g16b16Sfloat => (R16_G16_B16, Float),
        R16g16b16a16Unorm => (R16_G16_B16_A16, Unorm),
        R16g16b16a16Snorm => (R16_G16_B16_A16, Inorm),
        // R16g16b16a16Uscaled => (R16_G16_B16_A16, Unorm),
        // R16g16b16a16Sscaled => (R16_G16_B16_A16, Unorm),
        R16g16b16a16Uint => (R16_G16_B16_A16, Uint),
        R16g16b16a16Sint => (R16_G16_B16_A16, Int),
        R16g16b16a16Sfloat => (R16_G16_B16_A16, Float),
        R32Uint => (R32, Uint),
        R32Sint => (R32, Int),
        R32Sfloat => (R32, Float),
        R32g32Uint => (R32_G32, Uint),
        R32g32Sint => (R32_G32, Int),
        R32g32Sfloat => (R32_G32, Float),
        R32g32b32Uint => (R32_G32_B32, Uint),
        R32g32b32Sint => (R32_G32_B32, Int),
        R32g32b32Sfloat => (R32_G32_B32, Float),
        R32g32b32a32Uint => (R32_G32_B32_A32, Uint),
        R32g32b32a32Sint => (R32_G32_B32_A32, Int),
        R32g32b32a32Sfloat => (R32_G32_B32_A32, Float),
        // R64Uint => (R64, Uint),
        // R64Sint => (R64, Int),
        // R64Sfloat => (R64, Float),
        // R64g64Uint => (R64_G64, Uint),
        // R64g64Sint => (R64_G64, Int),
        // R64g64Sfloat => (R64_G64, Float),
        // R64g64b64Uint => (R64_G64_B64, Uint),
        // R64g64b64Sint => (R64_G64_B64, Int),
        // R64g64b64Sfloat => (R64_G64_B64, Float),
        // R64g64b64a64Uint => (R64_G64_B64_A64, Uint),
        // R64g64b64a64Sint => (R64_G64_B64_A64, Int),
        // R64g64b64a64Sfloat => (R64_G64_B64_A64, Float),
        // B10g11r11UfloatPack32 => (B10_G11_R11, Unorm),
        // E5b9g9r9UfloatPack32 => (R4_G4_B4_A4, Unorm),
        D16Unorm => (D16, Unorm),
        X8D24UnormPack32 => (D24, Unorm),
        D32Sfloat => (D32, Float),
        // S8Uint => (S8, Uint),
        // D16UnormS8Uint => (D16_S8, Unorm),
        D24UnormS8Uint => (D24_S8, Unorm),
        D32SfloatS8Uint => (D32_S8, Float),
        /*
        Bc1RgbUnormBlock => (R4_G4_B4_A4, Unorm),
        Bc1RgbSrgbBlock => (R4_G4_B4_A4, Unorm),
        Bc1RgbaUnormBlock => (R4_G4_B4_A4, Unorm),
        Bc1RgbaSrgbBlock => (R4_G4_B4_A4, Unorm),
        Bc2UnormBlock => (R4_G4_B4_A4, Unorm),
        Bc2SrgbBlock => (R4_G4_B4_A4, Unorm),
        Bc3UnormBlock => (R4_G4_B4_A4, Unorm),
        Bc3SrgbBlock => (R4_G4_B4_A4, Unorm),
        Bc4UnormBlock => (R4_G4_B4_A4, Unorm),
        Bc4SnormBlock => (R4_G4_B4_A4, Unorm),
        Bc5UnormBlock => (R4_G4_B4_A4, Unorm),
        Bc5SnormBlock => (R4_G4_B4_A4, Unorm),
        Bc6hUfloatBlock => (R4_G4_B4_A4, Unorm),
        Bc6hSfloatBlock => (R4_G4_B4_A4, Unorm),
        Bc7UnormBlock => (R4_G4_B4_A4, Unorm),
        Bc7SrgbBlock => (R4_G4_B4_A4, Unorm),
        Etc2R8g8b8UnormBlock => (R4_G4_B4_A4, Unorm),
        Etc2R8g8b8SrgbBlock => (R4_G4_B4_A4, Unorm),
        Etc2R8g8b8a1UnormBlock => (R4_G4_B4_A4, Unorm),
        Etc2R8g8b8a1SrgbBlock => (R4_G4_B4_A4, Unorm),
        Etc2R8g8b8a8UnormBlock => (R4_G4_B4_A4, Unorm),
        Etc2R8g8b8a8SrgbBlock => (R4_G4_B4_A4, Unorm),
        EacR11UnormBlock => (R4_G4_B4_A4, Unorm),
        EacR11SnormBlock => (R4_G4_B4_A4, Unorm),
        EacR11g11UnormBlock => (R4_G4_B4_A4, Unorm),
        EacR11g11SnormBlock => (R4_G4_B4_A4, Unorm),
        Astc4x4UnormBlock => (R4_G4_B4_A4, Unorm),
        Astc4x4SrgbBlock => (R4_G4_B4_A4, Unorm),
        Astc5x4UnormBlock => (R4_G4_B4_A4, Unorm),
        Astc5x4SrgbBlock => (R4_G4_B4_A4, Unorm),
        Astc5x5UnormBlock => (R4_G4_B4_A4, Unorm),
        Astc5x5SrgbBlock => (R4_G4_B4_A4, Unorm),
        Astc6x5UnormBlock => (R4_G4_B4_A4, Unorm),
        Astc6x5SrgbBlock => (R4_G4_B4_A4, Unorm),
        Astc6x6UnormBlock => (R4_G4_B4_A4, Unorm),
        Astc6x6SrgbBlock => (R4_G4_B4_A4, Unorm),
        Astc8x5UnormBlock => (R4_G4_B4_A4, Unorm),
        Astc8x5SrgbBlock => (R4_G4_B4_A4, Unorm),
        Astc8x6UnormBlock => (R4_G4_B4_A4, Unorm),
        Astc8x6SrgbBlock => (R4_G4_B4_A4, Unorm),
        Astc8x8UnormBlock => (R4_G4_B4_A4, Unorm),
        Astc8x8SrgbBlock => (R4_G4_B4_A4, Unorm),
        Astc10x5UnormBlock => (R4_G4_B4_A4, Unorm),
        Astc10x5SrgbBlock => (R4_G4_B4_A4, Unorm),
        Astc10x6UnormBlock => (R4_G4_B4_A4, Unorm),
        Astc10x6SrgbBlock => (R4_G4_B4_A4, Unorm),
        Astc10x8UnormBlock => (R4_G4_B4_A4, Unorm),
        Astc10x8SrgbBlock => (R4_G4_B4_A4, Unorm),
        Astc10x10UnormBlock => (R4_G4_B4_A4, Unorm),
        Astc10x10SrgbBlock => (R4_G4_B4_A4, Unorm),
        Astc12x10UnormBlock => (R4_G4_B4_A4, Unorm),
        Astc12x10SrgbBlock => (R4_G4_B4_A4, Unorm),
        Astc12x12UnormBlock => (R4_G4_B4_A4, Unorm),
        Astc12x12SrgbBlock => (R4_G4_B4_A4, Unorm),
        */
        _ => return None,
    })
}

pub fn map_component(component: format::Component) -> vk::ComponentSwizzle {
    use hal::format::Component::*;
    match component {
        Zero => vk::ComponentSwizzle::Zero,
        One  => vk::ComponentSwizzle::One,
        R    => vk::ComponentSwizzle::R,
        G    => vk::ComponentSwizzle::G,
        B    => vk::ComponentSwizzle::B,
        A    => vk::ComponentSwizzle::A,
    }
}

pub fn map_swizzle(swizzle: format::Swizzle) -> vk::ComponentMapping {
    vk::ComponentMapping {
        r: map_component(swizzle.0),
        g: map_component(swizzle.1),
        b: map_component(swizzle.2),
        a: map_component(swizzle.3),
    }
}

pub fn map_index_type(index_type: IndexType) -> vk::IndexType {
    match index_type {
        IndexType::U16 => vk::IndexType::Uint16,
        IndexType::U32 => vk::IndexType::Uint32,
    }
}

pub fn map_image_layout(layout: image::ImageLayout) -> vk::ImageLayout {
    use hal::image::ImageLayout as Il;
    match layout {
        Il::General => vk::ImageLayout::General,
        Il::ColorAttachmentOptimal => vk::ImageLayout::ColorAttachmentOptimal,
        Il::DepthStencilAttachmentOptimal => vk::ImageLayout::DepthStencilAttachmentOptimal,
        Il::DepthStencilReadOnlyOptimal => vk::ImageLayout::DepthStencilReadOnlyOptimal,
        Il::ShaderReadOnlyOptimal => vk::ImageLayout::ShaderReadOnlyOptimal,
        Il::TransferSrcOptimal => vk::ImageLayout::TransferSrcOptimal,
        Il::TransferDstOptimal => vk::ImageLayout::TransferDstOptimal,
        Il::Undefined => vk::ImageLayout::Undefined,
        Il::Preinitialized => vk::ImageLayout::Preinitialized,
        Il::Present => vk::ImageLayout::PresentSrcKhr,
    }
}

pub fn map_image_aspects(aspects: image::AspectFlags) -> vk::ImageAspectFlags {
    let mut flags = vk::ImageAspectFlags::empty();
    if aspects.contains(image::ASPECT_COLOR) {
        flags |= vk::IMAGE_ASPECT_COLOR_BIT;
    }
    if aspects.contains(image::ASPECT_DEPTH) {
        flags |= vk::IMAGE_ASPECT_DEPTH_BIT;
    }
    if aspects.contains(image::ASPECT_STENCIL) {
        flags |= vk::IMAGE_ASPECT_STENCIL_BIT;
    }
    flags
}

pub fn map_clear_color(value: ClearColor) -> vk::ClearColorValue {
    match value {
        ClearColor::Float(v) => vk::ClearColorValue::new_float32(v),
        ClearColor::Int(v)   => vk::ClearColorValue::new_int32(v),
        ClearColor::Uint(v)  => vk::ClearColorValue::new_uint32(v),
    }
}

pub fn map_clear_ds(value: ClearDepthStencil) -> vk::ClearDepthStencilValue {
    vk::ClearDepthStencilValue {
        depth: value.depth,
        stencil: value.stencil,
    }
}

pub fn map_clear_value(value: &ClearValue) -> vk::ClearValue {
    match *value {
        ClearValue::Color(cv) => {
            let cv = map_clear_color(cv);
            vk::ClearValue::new_color(cv)
        },
        ClearValue::DepthStencil(dsv) => {
            let dsv = map_clear_ds(dsv);
            vk::ClearValue::new_depth_stencil(dsv)
        },
    }
}

pub fn map_offset(offset: Offset) -> vk::Offset3D {
    vk::Offset3D {
        x: offset.x,
        y: offset.y,
        z: offset.z,
    }
}

pub fn map_extent(offset: Extent) -> vk::Extent3D {
    vk::Extent3D {
        width: offset.width,
        height: offset.height,
        depth: offset.depth,
    }
}

pub fn map_subresource_layers(
    aspect_mask: vk::ImageAspectFlags,
    level: image::Level,
    layers: &Range<image::Layer>,
) -> vk::ImageSubresourceLayers {
    vk::ImageSubresourceLayers {
        aspect_mask,
        mip_level: level as _,
        base_array_layer: layers.start as _,
        layer_count: (layers.end - layers.start) as _,
    }
}

pub fn map_subresource_with_layers(
    aspect_mask: vk::ImageAspectFlags,
    (mip_level, base_layer): image::Subresource,
    layers: image::Layer,
) -> vk::ImageSubresourceLayers {
    map_subresource_layers(aspect_mask, mip_level, &(base_layer..base_layer+layers))
}

pub fn map_subresource_range(
    range: &image::SubresourceRange,
) -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange {
        aspect_mask: map_image_aspects(range.aspects),
        base_mip_level: range.levels.start as _,
        level_count: (range.levels.end - range.levels.start) as _,
        base_array_layer: range.layers.start as _,
        layer_count: (range.layers.end - range.layers.start) as _,
    }
}

pub fn map_attachment_load_op(op: pass::AttachmentLoadOp) -> vk::AttachmentLoadOp {
    use hal::pass::AttachmentLoadOp as Alo;
    match op {
        Alo::Load => vk::AttachmentLoadOp::Load,
        Alo::Clear => vk::AttachmentLoadOp::Clear,
        Alo::DontCare => vk::AttachmentLoadOp::DontCare,
    }
}

pub fn map_attachment_store_op(op: pass::AttachmentStoreOp) -> vk::AttachmentStoreOp {
    use hal::pass::AttachmentStoreOp as Aso;
    match op {
        Aso::Store => vk::AttachmentStoreOp::Store,
        Aso::DontCare => vk::AttachmentStoreOp::DontCare,
    }
}

pub fn map_buffer_access(access: buffer::Access) -> vk::AccessFlags {
    let mut flags = vk::AccessFlags::empty();

    if access.contains(buffer::TRANSFER_READ) {
        flags |= vk::ACCESS_TRANSFER_READ_BIT;
    }
    if access.contains(buffer::TRANSFER_WRITE) {
        flags |= vk::ACCESS_TRANSFER_WRITE_BIT;
    }
    if access.contains(buffer::INDEX_BUFFER_READ) {
        flags |= vk::ACCESS_INDEX_READ_BIT;
    }
    if access.contains(buffer::VERTEX_BUFFER_READ) {
        flags |= vk::ACCESS_VERTEX_ATTRIBUTE_READ_BIT;
    }
    if access.contains(buffer::CONSTANT_BUFFER_READ) {
        flags |= vk::ACCESS_UNIFORM_READ_BIT;
    }
    if access.contains(buffer::INDIRECT_COMMAND_READ) {
        flags |= vk::ACCESS_INDIRECT_COMMAND_READ_BIT;
    }
    if access.contains(buffer::SHADER_READ) {
        flags |= vk::ACCESS_SHADER_READ_BIT;
    }
    if access.contains(buffer::SHADER_WRITE) {
        flags |= vk::ACCESS_SHADER_WRITE_BIT;
    }
    if access.contains(buffer::HOST_READ) {
        flags |= vk::ACCESS_HOST_READ_BIT;
    }
    if access.contains(buffer::HOST_WRITE) {
        flags |= vk::ACCESS_HOST_WRITE_BIT;
    }
    if access.contains(buffer::MEMORY_READ) {
        flags |= vk::ACCESS_MEMORY_READ_BIT;
    }
    if access.contains(buffer::MEMORY_WRITE) {
        flags |= vk::ACCESS_MEMORY_WRITE_BIT;
    }

    flags
}

pub fn map_image_access(access: image::Access) -> vk::AccessFlags {
    let mut flags = vk::AccessFlags::empty();

    if access.contains(image::COLOR_ATTACHMENT_READ) {
        flags |= vk::ACCESS_COLOR_ATTACHMENT_READ_BIT;
    }
    if access.contains(image::COLOR_ATTACHMENT_WRITE) {
        flags |= vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT;
    }
    if access.contains(image::TRANSFER_READ) {
        flags |= vk::ACCESS_TRANSFER_READ_BIT;
    }
    if access.contains(image::TRANSFER_WRITE) {
        flags |= vk::ACCESS_TRANSFER_WRITE_BIT;
    }
    if access.contains(image::SHADER_READ) {
        flags |= vk::ACCESS_SHADER_READ_BIT;
    }
    if access.contains(image::SHADER_WRITE) {
        flags |= vk::ACCESS_SHADER_WRITE_BIT;
    }
    if access.contains(image::DEPTH_STENCIL_ATTACHMENT_READ) {
        flags |= vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT;
    }
    if access.contains(image::DEPTH_STENCIL_ATTACHMENT_WRITE) {
        flags |= vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT;
    }
    if access.contains(image::HOST_READ) {
        flags |= vk::ACCESS_HOST_READ_BIT;
    }
    if access.contains(image::HOST_WRITE) {
        flags |= vk::ACCESS_HOST_WRITE_BIT;
    }
    if access.contains(image::MEMORY_READ) {
        flags |= vk::ACCESS_MEMORY_READ_BIT;
    }
    if access.contains(image::MEMORY_WRITE) {
        flags |= vk::ACCESS_MEMORY_WRITE_BIT;
    }
    if access.contains(image::INPUT_ATTACHMENT_READ) {
        flags |= vk::ACCESS_INPUT_ATTACHMENT_READ_BIT;
    }

    flags
}

pub fn map_pipeline_stage(stage: pso::PipelineStage) -> vk::PipelineStageFlags {
    let mut flags = vk::PipelineStageFlags::empty();

    if stage.contains(pso::TOP_OF_PIPE) {
        flags |= vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT;
    }
    if stage.contains(pso::DRAW_INDIRECT) {
        flags |= vk::PIPELINE_STAGE_DRAW_INDIRECT_BIT;
    }
    if stage.contains(pso::VERTEX_INPUT) {
        flags |= vk::PIPELINE_STAGE_VERTEX_INPUT_BIT;
    }
    if stage.contains(pso::VERTEX_SHADER) {
        flags |= vk::PIPELINE_STAGE_VERTEX_SHADER_BIT;
    }
    if stage.contains(pso::HULL_SHADER) {
        flags |= vk::PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT;
    }
    if stage.contains(pso::DOMAIN_SHADER) {
        flags |= vk::PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT;
    }
    if stage.contains(pso::GEOMETRY_SHADER) {
        flags |= vk::PIPELINE_STAGE_GEOMETRY_SHADER_BIT;
    }
    if stage.contains(pso::FRAGMENT_SHADER) {
        flags |= vk::PIPELINE_STAGE_FRAGMENT_SHADER_BIT;
    }
    if stage.contains(pso::EARLY_FRAGMENT_TESTS) {
        flags |= vk::PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT;
    }
    if stage.contains(pso::LATE_FRAGMENT_TESTS) {
        flags |= vk::PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT;
    }
    if stage.contains(pso::COLOR_ATTACHMENT_OUTPUT) {
        flags |= vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;
    }
    if stage.contains(pso::COMPUTE_SHADER) {
        flags |= vk::PIPELINE_STAGE_COMPUTE_SHADER_BIT;
    }
    if stage.contains(pso::TRANSFER) {
        flags |= vk::PIPELINE_STAGE_TRANSFER_BIT;
    }
    if stage.contains(pso::BOTTOM_OF_PIPE) {
        flags |= vk::PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT;
    }
    if stage.contains(pso::HOST) {
        flags |= vk::PIPELINE_STAGE_HOST_BIT;
    }

    flags
}

pub fn map_buffer_usage(usage: buffer::Usage) -> vk::BufferUsageFlags {
    let mut flags = vk::BufferUsageFlags::empty();

    if usage.contains(buffer::TRANSFER_SRC) {
        flags |= vk::BUFFER_USAGE_TRANSFER_SRC_BIT;
    }
    if usage.contains(buffer::TRANSFER_DST) {
        flags |= vk::BUFFER_USAGE_TRANSFER_DST_BIT;
    }
    if usage.contains(buffer::UNIFORM) {
        flags |= vk::BUFFER_USAGE_UNIFORM_BUFFER_BIT;
    }
    if usage.contains(buffer::STORAGE) {
        flags |= vk::BUFFER_USAGE_STORAGE_BUFFER_BIT;
    }
    if usage.contains(buffer::UNIFORM_TEXEL) {
        flags |= vk::BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT;
    }
    if usage.contains(buffer::STORAGE_TEXEL) {
        flags |= vk::BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT;
    }
    if usage.contains(buffer::INDEX) {
        flags |= vk::BUFFER_USAGE_INDEX_BUFFER_BIT;
    }
    if usage.contains(buffer::INDIRECT) {
        flags |= vk::BUFFER_USAGE_INDIRECT_BUFFER_BIT;
    }
    if usage.contains(buffer::VERTEX) {
        flags |= vk::BUFFER_USAGE_VERTEX_BUFFER_BIT;
    }

    flags
}

pub fn map_image_usage(usage: image::Usage) -> vk::ImageUsageFlags {
    let mut flags = vk::ImageUsageFlags::empty();

    if usage.contains(image::TRANSFER_SRC) {
        flags |= vk::IMAGE_USAGE_TRANSFER_SRC_BIT;
    }
    if usage.contains(image::TRANSFER_DST) {
        flags |= vk::IMAGE_USAGE_TRANSFER_DST_BIT;
    }
    if usage.contains(image::COLOR_ATTACHMENT) {
        flags |= vk::IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
    }
    if usage.contains(image::DEPTH_STENCIL_ATTACHMENT) {
        flags |= vk::IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT;
    }
    if usage.contains(image::STORAGE) {
        flags |= vk::IMAGE_USAGE_STORAGE_BIT;
    }
    if usage.contains(image::SAMPLED) {
        flags |= vk::IMAGE_USAGE_SAMPLED_BIT;
    }

    flags
}

pub fn map_descriptor_type(ty: pso::DescriptorType) -> vk::DescriptorType {
    use hal::pso::DescriptorType as Dt;
    match ty {
        Dt::Sampler            => vk::DescriptorType::Sampler,
        Dt::SampledImage       => vk::DescriptorType::SampledImage,
        Dt::StorageImage       => vk::DescriptorType::StorageImage,
        Dt::UniformTexelBuffer => vk::DescriptorType::UniformTexelBuffer,
        Dt::StorageTexelBuffer => vk::DescriptorType::StorageTexelBuffer,
        Dt::UniformBuffer      => vk::DescriptorType::UniformBuffer,
        Dt::StorageBuffer      => vk::DescriptorType::StorageBuffer,
        Dt::InputAttachment    => vk::DescriptorType::InputAttachment,
    }
}

pub fn map_stage_flags(stages: pso::ShaderStageFlags) -> vk::ShaderStageFlags {
    let mut flags = vk::ShaderStageFlags::empty();

    if stages.contains(pso::STAGE_VERTEX) {
        flags |= vk::SHADER_STAGE_VERTEX_BIT;
    }

    if stages.contains(pso::STAGE_HULL) {
        flags |= vk::SHADER_STAGE_TESSELLATION_CONTROL_BIT;
    }

    if stages.contains(pso::STAGE_DOMAIN) {
        flags |= vk::SHADER_STAGE_TESSELLATION_EVALUATION_BIT;
    }

    if stages.contains(pso::STAGE_GEOMETRY) {
        flags |= vk::SHADER_STAGE_GEOMETRY_BIT;
    }

    if stages.contains(pso::STAGE_FRAGMENT) {
        flags |= vk::SHADER_STAGE_FRAGMENT_BIT;
    }

    if stages.contains(pso::STAGE_COMPUTE) {
        flags |= vk::SHADER_STAGE_COMPUTE_BIT;
    }

    flags
}


pub fn map_filter(filter: image::FilterMethod) -> (vk::Filter, vk::Filter, vk::SamplerMipmapMode, f32) {
    use hal::image::FilterMethod as Fm;
    match filter {
        Fm::Scale          => (vk::Filter::Nearest, vk::Filter::Nearest, vk::SamplerMipmapMode::Nearest, 1.0),
        Fm::Mipmap         => (vk::Filter::Nearest, vk::Filter::Nearest, vk::SamplerMipmapMode::Linear,  1.0),
        Fm::Bilinear       => (vk::Filter::Linear,  vk::Filter::Linear,  vk::SamplerMipmapMode::Nearest, 1.0),
        Fm::Trilinear      => (vk::Filter::Linear,  vk::Filter::Linear,  vk::SamplerMipmapMode::Linear,  1.0),
        Fm::Anisotropic(a) => (vk::Filter::Linear,  vk::Filter::Linear,  vk::SamplerMipmapMode::Linear,  a as f32),
    }
}

pub fn map_wrap(wrap: image::WrapMode) -> vk::SamplerAddressMode {
    use hal::image::WrapMode as Wm;
    match wrap {
        Wm::Tile   => vk::SamplerAddressMode::Repeat,
        Wm::Mirror => vk::SamplerAddressMode::MirroredRepeat,
        Wm::Clamp  => vk::SamplerAddressMode::ClampToEdge,
        Wm::Border => vk::SamplerAddressMode::ClampToBorder,
    }
}

pub fn map_border_color(col: image::PackedColor) -> Option<vk::BorderColor> {
    match col.0 {
        0x00000000 => Some(vk::BorderColor::FloatTransparentBlack),
        0xFF000000 => Some(vk::BorderColor::FloatOpaqueBlack),
        0xFFFFFFFF => Some(vk::BorderColor::FloatOpaqueWhite),
        _ => None
    }
}

pub fn map_topology(prim: Primitive) -> vk::PrimitiveTopology {
    match prim {
        Primitive::PointList     => vk::PrimitiveTopology::PointList,
        Primitive::LineList      => vk::PrimitiveTopology::LineList,
        Primitive::LineListAdjacency => vk::PrimitiveTopology::LineListWithAdjacency,
        Primitive::LineStrip     => vk::PrimitiveTopology::LineStrip,
        Primitive::LineStripAdjacency => vk::PrimitiveTopology::LineStripWithAdjacency,
        Primitive::TriangleList  => vk::PrimitiveTopology::TriangleList,
        Primitive::TriangleListAdjacency => vk::PrimitiveTopology::TriangleListWithAdjacency,
        Primitive::TriangleStrip => vk::PrimitiveTopology::TriangleStrip,
        Primitive::TriangleStripAdjacency => vk::PrimitiveTopology::TriangleStripWithAdjacency,
        Primitive::PatchList(_)  => vk::PrimitiveTopology::PatchList,
    }
}

pub fn map_polygon_mode(rm: state::RasterMethod) -> (vk::PolygonMode, f32) {
    match rm {
        state::RasterMethod::Point   => (vk::PolygonMode::Point, 1.0),
        state::RasterMethod::Line(w) => (vk::PolygonMode::Line, w as f32),
        state::RasterMethod::Fill    => (vk::PolygonMode::Fill, 1.0),
    }
}

pub fn map_cull_mode(cf: state::CullFace) -> vk::CullModeFlags {
    match cf {
        state::CullFace::Nothing => vk::CULL_MODE_NONE,
        state::CullFace::Front   => vk::CULL_MODE_FRONT_BIT,
        state::CullFace::Back    => vk::CULL_MODE_BACK_BIT,
    }
}

pub fn map_front_face(ff: state::FrontFace) -> vk::FrontFace {
    match ff {
        state::FrontFace::Clockwise        => vk::FrontFace::Clockwise,
        state::FrontFace::CounterClockwise => vk::FrontFace::CounterClockwise,
    }
}

pub fn map_comparison(fun: state::Comparison) -> vk::CompareOp {
    use hal::state::Comparison::*;
    match fun {
        Never        => vk::CompareOp::Never,
        Less         => vk::CompareOp::Less,
        LessEqual    => vk::CompareOp::LessOrEqual,
        Equal        => vk::CompareOp::Equal,
        GreaterEqual => vk::CompareOp::GreaterOrEqual,
        Greater      => vk::CompareOp::Greater,
        NotEqual     => vk::CompareOp::NotEqual,
        Always       => vk::CompareOp::Always,
    }
}

pub fn map_stencil_op(op: state::StencilOp) -> vk::StencilOp {
    use hal::state::StencilOp::*;
    match op {
        Keep           => vk::StencilOp::Keep,
        Zero           => vk::StencilOp::Zero,
        Replace        => vk::StencilOp::Replace,
        IncrementClamp => vk::StencilOp::IncrementAndClamp,
        IncrementWrap  => vk::StencilOp::IncrementAndWrap,
        DecrementClamp => vk::StencilOp::DecrementAndClamp,
        DecrementWrap  => vk::StencilOp::DecrementAndWrap,
        Invert         => vk::StencilOp::Invert,
    }
}

pub fn map_stencil_side(side: &state::StencilSide) -> vk::StencilOpState {
    vk::StencilOpState {
        fail_op: map_stencil_op(side.op_fail),
        pass_op: map_stencil_op(side.op_pass),
        depth_fail_op: map_stencil_op(side.op_depth_fail),
        compare_op: map_comparison(side.fun),
        compare_mask: side.mask_read as u32,
        write_mask: side.mask_write as u32,
        reference: 0,
    }
}

pub fn map_blend_factor(factor: state::Factor, scalar: bool) -> vk::BlendFactor {
    use hal::state::BlendValue::*;
    use hal::state::Factor::*;
    match factor {
        Zero => vk::BlendFactor::Zero,
        One => vk::BlendFactor::One,
        SourceAlphaSaturated => vk::BlendFactor::SrcAlphaSaturate,
        ZeroPlus(SourceColor) if !scalar => vk::BlendFactor::SrcColor,
        ZeroPlus(SourceAlpha) => vk::BlendFactor::SrcAlpha,
        ZeroPlus(DestColor) if !scalar => vk::BlendFactor::DstColor,
        ZeroPlus(DestAlpha) => vk::BlendFactor::DstAlpha,
        ZeroPlus(ConstColor) if !scalar => vk::BlendFactor::ConstantColor,
        ZeroPlus(ConstAlpha) => vk::BlendFactor::ConstantAlpha,
        OneMinus(SourceColor) if !scalar => vk::BlendFactor::OneMinusSrcColor,
        OneMinus(SourceAlpha) => vk::BlendFactor::OneMinusSrcAlpha,
        OneMinus(DestColor) if !scalar => vk::BlendFactor::OneMinusDstColor,
        OneMinus(DestAlpha) => vk::BlendFactor::OneMinusDstAlpha,
        OneMinus(ConstColor) if !scalar => vk::BlendFactor::OneMinusConstantColor,
        OneMinus(ConstAlpha) => vk::BlendFactor::OneMinusConstantAlpha,
        _ => {
            error!("Invalid blend factor requested for {}: {:?}",
                if scalar {"alpha"} else {"color"}, factor);
            vk::BlendFactor::Zero
        }
    }
}

pub fn map_blend_op(equation: state::Equation) -> vk::BlendOp {
    use hal::state::Equation::*;
    match equation {
        Add => vk::BlendOp::Add,
        Sub => vk::BlendOp::Subtract,
        RevSub => vk::BlendOp::ReverseSubtract,
        Min => vk::BlendOp::Min,
        Max => vk::BlendOp::Max,
    }
}