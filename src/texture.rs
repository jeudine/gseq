use image::load_from_memory;
use std::num::NonZeroU32;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextureError {
	#[error("Failed to read image")]
	Reading(#[from] std::io::Error),
	#[error("Failed to decode image")]
	Decoding(#[from] image::ImageError),
	#[error("Image dimension is invalid")]
	InvalidImageDimension,
}

#[derive(Debug)]
pub struct TextureInner {
	pub texture: wgpu::Texture,
	pub view: wgpu::TextureView,
	pub sampler: wgpu::Sampler,
}

pub enum Texture {
	Depth(TextureInner),
	Framebuffer(TextureInner),
	Image(TextureInner),
}

impl Texture {
	pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

	pub fn new_depth(
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		label: &str,
	) -> Self {
		let size = wgpu::Extent3d {
			width: config.width,
			height: config.height,
			depth_or_array_layers: 1,
		};
		let desc = wgpu::TextureDescriptor {
			label: Some(label),
			size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: Self::DEPTH_FORMAT,
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
		};
		let texture = device.create_texture(&desc);

		let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
		let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Linear,
			min_filter: wgpu::FilterMode::Linear,
			mipmap_filter: wgpu::FilterMode::Nearest,
			compare: Some(wgpu::CompareFunction::LessEqual),
			lod_min_clamp: 0.0,
			lod_max_clamp: 100.0,
			..Default::default()
		});

		Self::Depth(TextureInner {
			texture,
			view,
			sampler,
		})
	}

	pub fn new_image(
		image: &[u8],
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		label: &str,
	) -> Result<Self, TextureError> {
		let img = load_from_memory(image)?.into_rgba8();

		let size = wgpu::Extent3d {
			width: img.width(),
			height: img.height(),
			depth_or_array_layers: 1,
		};
		let desc = wgpu::TextureDescriptor {
			label: Some(label),
			size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::Rgba8UnormSrgb,
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
		};
		let texture = device.create_texture(&desc);

		let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
		let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Linear,
			min_filter: wgpu::FilterMode::Linear,
			mipmap_filter: wgpu::FilterMode::Nearest,
			..Default::default()
		});

		queue.write_texture(
			// Tells wgpu where to copy the pixel data
			wgpu::ImageCopyTexture {
				texture: &texture,
				mip_level: 0,
				origin: wgpu::Origin3d::ZERO,
				aspect: wgpu::TextureAspect::All,
			},
			// The actual pixel data
			&img,
			// The layout of the texture
			wgpu::ImageDataLayout {
				offset: 0,
				bytes_per_row: Some(
					NonZeroU32::new(4 * img.width()).ok_or(TextureError::InvalidImageDimension)?,
				),
				rows_per_image: Some(
					NonZeroU32::new(img.height()).ok_or(TextureError::InvalidImageDimension)?,
				),
			},
			size,
		);

		Ok(Self::Image(TextureInner {
			texture,
			view,
			sampler,
		}))
	}

	pub fn new_framebuffer(
		device: &wgpu::Device,
		(width, height): (u32, u32),
		label: &str,
	) -> Self {
		let size = wgpu::Extent3d {
			width,
			height,
			depth_or_array_layers: 1,
		};
		let desc = wgpu::TextureDescriptor {
			label: Some(label),
			size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::Bgra8UnormSrgb,
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
		};
		let texture = device.create_texture(&desc);

		let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
		let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Linear,
			min_filter: wgpu::FilterMode::Linear,
			mipmap_filter: wgpu::FilterMode::Nearest,
			..Default::default()
		});

		Self::Framebuffer(TextureInner {
			texture,
			view,
			sampler,
		})
	}

	fn inner(&self) -> &TextureInner {
		match self {
			Texture::Depth(t) => t,
			Texture::Image(t) => t,
			Texture::Framebuffer(t) => t,
		}
	}

	pub fn view(&self) -> &wgpu::TextureView {
		return &self.inner().view;
	}

	pub fn create_texture_bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
		wgpu::BindGroupLayoutEntry {
			binding,
			visibility: wgpu::ShaderStages::FRAGMENT,
			ty: wgpu::BindingType::Texture {
				multisampled: false,
				view_dimension: wgpu::TextureViewDimension::D2,
				sample_type: wgpu::TextureSampleType::Float { filterable: true },
			},
			count: None,
		}
	}

	pub fn create_sampler_bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
		wgpu::BindGroupLayoutEntry {
			binding,
			visibility: wgpu::ShaderStages::FRAGMENT,
			ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
			count: None,
		}
	}

	pub fn create_bind_group(
		&self,
		device: &wgpu::Device,
		bind_group_layout: &wgpu::BindGroupLayout,
	) -> wgpu::BindGroup {
		let texture = self.inner();
		device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::TextureView(&texture.view),
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: wgpu::BindingResource::Sampler(&texture.sampler),
				},
			],
			label: Some("texture_bind_group"),
		})
	}
}

pub fn create_texture_image_bind_group_layout(
	nb_textures: usize,
	device: &wgpu::Device,
) -> wgpu::BindGroupLayout {
	let mut v: Vec<wgpu::BindGroupLayoutEntry> = vec![];
	for i in 0..nb_textures {
		v.push(Texture::create_texture_bind_group_layout_entry(
			2 * i as u32,
		));
		v.push(Texture::create_sampler_bind_group_layout_entry(
			(2 * i + 1) as u32,
		));
	}

	device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
		entries: &v[..],
		label: Some("texture_image_bind_group_layout"),
	})
}

pub fn create_texture_image_bind_group(
	texture_images: &Vec<Texture>,
	device: &wgpu::Device,
	bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::BindGroup {
	let mut v: Vec<wgpu::BindGroupEntry> = vec![];
	for i in 0..texture_images.len() {
		let texture = texture_images[i].inner();
		v.push(wgpu::BindGroupEntry {
			binding: (2 * i) as u32,
			resource: wgpu::BindingResource::TextureView(&texture.view),
		});
		v.push(wgpu::BindGroupEntry {
			binding: (2 * i + 1) as u32,
			resource: wgpu::BindingResource::Sampler(&texture.sampler),
		});
	}
	device.create_bind_group(&wgpu::BindGroupDescriptor {
		layout: bind_group_layout,
		entries: &v[..],
		label: Some("texture_image_bind_group"),
	})
}
