// "adi_gpu_vulkan" - Aldaron's Device Interface / GPU / Vulkan
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)
//
//! Vulkan implementation for adi_gpu.

use std::{ mem };
use libc::memcpy;

// TODO: absorb into ffi, only once internal todo is resolved.

use asi_vulkan;
use asi_vulkan::types::*;
use asi_vulkan::Gpu;

pub fn copy_memory<T>(connection: &Gpu, vk_memory: VkDeviceMemory,
	data: &T) where T: Clone
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

pub fn copy_memory_pitched<T>(connection: &Gpu, vk_memory: VkDeviceMemory,
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
			memcpy(mapped.offset(i * pitch / mem::size_of::<T>()
					as isize) as *mut _,
				data.as_ptr().offset(i * width) as *const _,
				width as usize * mem::size_of::<T>());
		}
	}

	unsafe {
		asi_vulkan::unmap_memory(connection, vk_memory);
	}
}
