// renderer/ffi/mod.rs -- Aldaron's Device Interface / GPU / Vulkan
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use std::{ mem };
use libc::memcpy;

pub mod vulkan;
// TODO: absorb into ffi, only once internal todo is resolved.
pub mod create_surface;

use asi_vulkan;
use asi_vulkan::types::*;
use asi_vulkan::Vk;

/*pub struct VulkanRenderer {
	native: vulkan::Vulkan,
}

impl ::RenderOps for VulkanRenderer {
	#[allow(unused)]
	fn new(app_name: &str, window: ::awi::WindowConnection) -> Self {
		let native = vulkan::Vulkan::new(app_name).unwrap();

		VulkanRenderer { native }
	}

	fn update(&self) -> () {
	}
}*/

pub fn copy_memory<T>(connection: &mut Vk, vk_memory: VkDeviceMemory, data: &T)
	where T: Clone
{
	let mapped : *mut T = unsafe {
		asi_vulkan::map_memory(connection, vk_memory, !0)
	};

	if mapped.is_null() {
		panic!("Couldn't Map Buffer Memory?  Unknown cause.");
	}

	unsafe {
		*mapped = data.clone();
		asi_vulkan::unmap_memory(connection, vk_memory);
	}
}

pub fn copy_memory_pitched<T>(connection: &mut Vk, vk_memory: VkDeviceMemory,
	data: &[T], width: isize, height: isize, pitch: isize) where T: Clone
{
	let mapped : *mut T = unsafe {
		asi_vulkan::map_memory(connection, vk_memory, !0)
	};

	if mapped.is_null() {
		panic!("Couldn't Map Buffer Memory?  Unknown cause.");
	}

	for i in 0..height {
		unsafe {
			memcpy(cast_mut!(mapped.offset(
					i * pitch / mem::size_of::<T>() as isize
				)),
				cast!(data.as_ptr().offset(i * width)),
				width as usize * mem::size_of::<T>());
		}
	}

	unsafe {
		asi_vulkan::unmap_memory(connection, vk_memory);
	}
}

pub fn cmd_draw(connection: &mut Vk, vertex_count: u32) {
	unsafe {
		asi_vulkan::cmd_draw(connection, vertex_count, 1, 0, 0);
	}
}
