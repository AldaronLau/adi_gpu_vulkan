// renderer/ffi/vulkan.rs -- Aldaron's Device Interface / GPU / Vulkan
// Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

use asi_vulkan;

pub struct Vulkan(pub asi_vulkan::Vk);

impl Vulkan {
	pub fn new() -> Option<Self> {
		let connection = asi_vulkan::Vk::new();

		if let Some(c) = connection {
			Some(Vulkan(c))
		} else {
			None // Couldn't find Vulkan
		}
	}
}
