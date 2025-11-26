#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

pub const TRIANGLE: [Vertex; 3] = [
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

fn to_linear(mut color: [f32; 3]) -> [f32; 3] {
    (0..color.len()).for_each(|c| {
        color[c] = ((color[c] + 0.055) / 1.055).powf(2.4);
    });

    color
}

pub fn vertices() -> [Vertex; 4] {
    [
        Vertex {
            position: [-1.0, 1.0, 0.0],
            color: to_linear([0.28235, 0.18039, 0.0745098]),
        },
        Vertex {
            position: [1.0, 1.0, 0.0],
            color: to_linear([0.3568, 0.290196, 0.211764]),
        },
        Vertex {
            position: [-1.0, -1.0, 0.0],
            color: to_linear([0.1333, 0.1843, 0.2980]),
        },
        Vertex {
            position: [1.0, -1.0, 0.0],
            color: to_linear([0.6196, 0.54509, 0.43529]),
        },
    ]
}

pub const INDICES: &[u16] = &[2, 1, 0, 3, 1, 2];

pub const CUBE: &[Vertex] = &[];
pub const SPHERE: &[Vertex] = &[];

/*
# Graphics Programming Discussion - RGB, sRGB, and Color Spaces

## RGB vs sRGB Fundamentals

**RGB** is a color model - a way to represent colors using Red, Green, Blue components. It doesn't specify what those numbers mean in terms of actual light.

**Color spaces** define what RGB numbers represent. Examples:
- **sRGB** (monitors/web)
- **Adobe RGB** (professional photography)
- **Linear RGB** (mathematically linear light values)

**sRGB** is a specific RGB color space designed to match human brightness perception. Our eyes don't perceive light linearly - we're more sensitive to changes in dark colors than bright colors.

## The wgpu Tutorial Problem

Your vertex colors: `(0.5, 0.0, 0.5)` (linear RGB)
→ GPU converts for sRGB display
→ Results in hex `#BC00BC` = `(188, 0, 188)` = `(0.737, 0, 0.737)` when measured

The mismatch occurs because of gamma correction during linear → sRGB conversion.

## Gamma Correction Deep Dive

### Historical Origin
CRT monitors had non-linear voltage-to-light response: `light_output = voltage^2.2`
To get desired brightness, engineers pre-corrected signals: `corrected_voltage = desired_brightness^(1/2.2)`

### What Gamma Correction Does
Redistributes brightness levels using power functions:
```
gamma_corrected = linear^(1/gamma)
linear = gamma_corrected^gamma
```
Where gamma ≈ 2.2 for displays.

### Visual Effect
**Linear gradient** 0.0, 0.25, 0.5, 0.75, 1.0 appears unevenly spaced to human eyes
**Gamma corrected** 0.0, 0.537, 0.735, 0.881, 1.0 appears evenly spaced

### sRGB Formula
```
// sRGB to Linear
linear = ((sRGB + 0.055) / 1.055)^2.4

// Linear to sRGB
sRGB = 1.055 * linear^(1/2.4) - 0.055
```

## Matrix Transformations vs Gamma Correction

**Gamma correction** (sRGB ↔ Linear): Non-linear per-channel transformation using power functions

**Color space conversion** (different RGB primaries): Matrix multiplication
```
[R_new]   [m11 m12 m13] [R_old]
[G_new] = [m21 m22 m23] [G_old]
[B_new]   [m31 m32 m33] [B_old]
```

Your tutorial case uses only gamma correction, not matrix operations.

## Human Visual System Limitations

**Luminance sensitivity**:
- Green: ~59% of perceived brightness
- Red: ~30%
- Blue: ~11%

**Why graphics ignores this**: Historical pragmatism, "good enough" results, established workflows would break if changed.

**Where it IS considered**:
- Video compression (Y/Cb/Cr channels)
- Advanced color spaces (LAB, Oklab)
- Professional image processing

## Oklab Color Space

Modern perceptually uniform color space (2020) with better hue preservation and mathematical elegance.

**For your current situation**: Won't help with gamma correction issue. You'd still need Linear RGB for GPU and gamma correction for display.

**Where Oklab excels**: Color manipulation, gradients, palette generation - but adds complexity for real-time graphics.
*/
