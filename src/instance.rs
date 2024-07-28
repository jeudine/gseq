use cgmath::{One, Zero};

#[derive(Copy, Clone, Debug)]
pub struct Instance {
    pub scale: f32,
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Basis3<f32>,
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[allow(dead_code)]
pub struct InstanceRaw {
    color: [f32; 4],
    model: [[f32; 4]; 4],
}

impl Instance {
    pub fn new() -> Self {
        let position = cgmath::Vector3::zero();
        let rotation = cgmath::Basis3::one();
        let color = [1.0, 1.0, 1.0, 1.0];

        Instance {
            position,
            rotation,
            scale: 1.0,
            color,
        }
    }

    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            color: self.color,
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(cgmath::Matrix3::from(self.rotation))
                * cgmath::Matrix4::from_scale(self.scale))
            .into(),
        }
    }
}

impl InstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We'll have to reassemble the mat4 in the shader.
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials, we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5, not conflict with them later
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}
/*
    pub fn to_raw_translate(&self, m: &Material, t: cgmath::Vector3<f32>) -> InstanceRaw {
        InstanceRaw {
            ambient: m.ambient.into(),
            diffuse: m.diffuse.into(),
            spec: m.spec.into(),
            shin: m.shin,
            model: (cgmath::Matrix4::from_translation(self.position + t)
                * cgmath::Matrix4::from(cgmath::Matrix3::from(self.rotation))
                * cgmath::Matrix4::from_scale(self.scale))
            .into(),
            normal: cgmath::Matrix3::from(self.rotation).into(),
        }
    }

    pub fn to_raw_translate_rotate(
        &self,
        m: &Material,
        t: cgmath::Vector3<f32>,
        rotation: &cgmath::Basis3<f32>,
    ) -> InstanceRaw {
        let rotation = cgmath::Matrix3::from(self.rotation * rotation);
        InstanceRaw {
            ambient: m.ambient.into(),
            diffuse: m.diffuse.into(),
            spec: m.spec.into(),
            shin: m.shin,
            model: (cgmath::Matrix4::from_translation(self.position + t)
                * cgmath::Matrix4::from(rotation)
                * cgmath::Matrix4::from_scale(self.scale))
            .into(),
            normal: cgmath::Matrix3::from(rotation).into(),
        }
    }

    pub fn to_raw_rotate(&self, m: &Material, rotation: &cgmath::Basis3<f32>) -> InstanceRaw {
        let rotation = cgmath::Matrix3::from(self.rotation * rotation);
        InstanceRaw {
            ambient: m.ambient.into(),
            diffuse: m.diffuse.into(),
            spec: m.spec.into(),
            shin: m.shin,
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(rotation)
                * cgmath::Matrix4::from_scale(self.scale))
            .into(),
            normal: cgmath::Matrix3::from(rotation).into(),
        }
    }
    pub fn to_raw_scale_rotate(
        &self,
        m: &Material,
        scale: f32,
        rotation: &cgmath::Basis3<f32>,
    ) -> InstanceRaw {
        let rotation = cgmath::Matrix3::from(self.rotation * rotation);
        InstanceRaw {
            ambient: m.ambient.into(),
            diffuse: m.diffuse.into(),
            spec: m.spec.into(),
            shin: m.shin,
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(rotation)
                * cgmath::Matrix4::from_scale(self.scale * scale))
            .into(),
            normal: cgmath::Matrix3::from(rotation).into(),
        }
    }

    pub fn to_raw_scale(&self, m: &Material, scale: f32) -> InstanceRaw {
        InstanceRaw {
            ambient: m.ambient.into(),
            diffuse: m.diffuse.into(),
            spec: m.spec.into(),
            shin: m.shin,
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(cgmath::Matrix3::from(self.rotation))
                * cgmath::Matrix4::from_scale(self.scale * scale))
            .into(),
            normal: cgmath::Matrix3::from(self.rotation).into(),
        }
    }

    pub fn rotate(&mut self, rotation: &cgmath::Basis3<f32>) {
        self.rotation = self.rotation * rotation;
    }
    pub fn raw_zero() -> InstanceRaw {
        InstanceRaw {
            ambient: [0.0, 0.0, 0.0],
            diffuse: [0.0, 0.0, 0.0],
            spec: [0.0, 0.0, 0.0],
            shin: 0.0,
            model: cgmath::Matrix4::zero().into(),
            normal: cgmath::Matrix3::zero().into(),
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 9]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 10]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 14]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 18]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 22]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 26]>() as wgpu::BufferAddress,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 29]>() as wgpu::BufferAddress,
                    shader_location: 11,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 32]>() as wgpu::BufferAddress,
                    shader_location: 12,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
*/
