// "adi_gpu_vulkan" - Aldaron's Device Interface / GPU / Vulkan
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)
//
//! Vulkan implementation for adi_gpu.

// TODO: Make surface a buffer and blit onto screen with window manager.

use adi_gpu_base::WindowConnection;

use asi_vulkan;
use asi_vulkan::Vulkan;

pub fn create_surface(c: &mut Vulkan, connection: WindowConnection) {
	match connection {
		WindowConnection::Xcb(connection,window) => {
			asi_vulkan::new_surface_xcb(c, connection, window)
		}
		WindowConnection::Wayland => panic!("Wayland Rendering Not Supported Yet"),
		WindowConnection::DirectFB => panic!("DirectFB Rendering Not Supported Yet"),
		WindowConnection::Windows(connection, window) => {
			asi_vulkan::new_surface_windows(c, connection, window)
		}
		WindowConnection::Android => panic!("Android Rendering Not Supported Yet"),
		WindowConnection::IOS => panic!("IOS Rendering Not Supported Yet"),
		WindowConnection::AldaronsOS => panic!("AldaronsOS Rendering Not Supported Yet"),
		WindowConnection::Arduino => panic!("Arduino Rendering Not Supported Yet"),
		WindowConnection::Switch => panic!("Switch Rendering Not Supported Yet"),
		WindowConnection::Web => panic!("Web Assembly Rendering Not Supported Yet"),
		WindowConnection::NoOS => panic!("No OS Rendering Not Supported Yet"),
	}
}
