// "adi_gpu_vulkan" - Aldaron's Device Interface / GPU / Vulkan
//
// Copyright Jeron A. Lau 2018.
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)
//
//! Vulkan implementation for adi_gpu.

// #![no_std]

extern crate asi_vulkan;
extern crate adi_gpu_base;
extern crate libc;

/// Transform represents a transformation matrix.
pub(crate) mod renderer;

pub use base::Shape;
pub use base::Gradient;
pub use base::Model;
pub use base::TexCoords;
pub use base::Texture;

use adi_gpu_base as base;
use adi_gpu_base::*;

/// To render anything with adi_gpu, you have to make a `Display`
pub struct Display {
	window: adi_gpu_base::Window,
	renderer: renderer::Renderer,
}

pub fn new<G: AsRef<Graphic>>(title: &str, icon: G)
	-> Result<Box<Display>, &'static str>
{
	let window = adi_gpu_base::Window::new(title, icon.as_ref(),
		None);
	let renderer = renderer::Renderer::new(window.get_connection(),
		(0.0, 0.0, 0.0))?;

	Ok(Box::new(Display { window, renderer }))
}

impl base::Display for Display {
	fn color(&mut self, color: (f32, f32, f32)) {
		self.renderer.bg_color(color);
	}

	fn update(&mut self) -> Option<adi_gpu_base::Input> {
		if let Some(input) = self.window.update() {
			return Some(input);
		}

		// Update Window:
		self.renderer.update();
		// Return None, there was no input, updated screen.
		None
	}

	fn camera(&mut self, xyz: Vec3, rotate_xyz: Vec3) {
		self.renderer.set_camera(xyz, rotate_xyz);
		self.renderer.camera();
	}

	fn model(&mut self, vertices: &[f32], fans: Vec<(u32, u32)>) -> Model {
		Model(self.renderer.model(vertices, fans))
	}

	fn fog(&mut self, fog: Option<(f32, f32)>) -> () {
		if let Some(fog) = fog {
			self.renderer.fog(fog);
		} else {
			self.renderer.fog((::std::f32::MAX, 0.0));
		}
	}

	fn texture(&mut self, graphic: &Graphic) -> Texture {
		let (w, h, pixels) = graphic.as_ref().as_slice();

		Texture(self.renderer.texture(w, h, pixels))
	}

	fn gradient(&mut self, colors: &[f32]) -> Gradient {
		Gradient(self.renderer.colors(colors))
	}

	fn texcoords(&mut self, texcoords: &[f32]) -> TexCoords {
		TexCoords(self.renderer.texcoords(texcoords))
	}

	fn set_texture(&mut self, texture: &mut Texture, pixels: &[u32]) {
		self.renderer.set_texture(texture.0, pixels);
	}

	#[inline(always)]
	fn shape_solid(&mut self, model: &Model, transform: Transform,
		color: [f32; 4], blending: bool, fog: bool,
		camera: bool) -> Shape
	{
		base::new_shape(self.renderer.solid(model.0, transform, color,
			blending, fog, camera))
	}

	#[inline(always)]
	fn shape_gradient(&mut self, model: &Model, transform: Transform,
		colors: Gradient, blending: bool, fog: bool,
		camera: bool) -> Shape
	{
		base::new_shape(self.renderer.gradient(model.0, transform,
			colors.0, blending, fog, camera))
	}

	#[inline(always)]
	fn shape_texture(&mut self, model: &Model, transform: Transform,
		texture: &Texture, tc: TexCoords, blending: bool,
		fog: bool, camera: bool) -> Shape
	{
		base::new_shape(self.renderer.textured(model.0, transform,
			texture.0, tc.0, blending, fog, camera))
	}

	#[inline(always)]
	fn shape_faded(&mut self, model: &Model, transform: Transform,
		texture: &Texture, tc: TexCoords, alpha: f32,
		fog: bool, camera: bool) -> Shape
	{
		base::new_shape(self.renderer.faded(model.0, transform,
			texture.0, tc.0, alpha, fog, camera))
	}

	#[inline(always)]
	fn shape_tinted(&mut self, model: &Model, transform: Transform,
		texture: &Texture, tc: TexCoords, tint: [f32; 4], blending: bool,
		fog: bool, camera: bool) -> Shape
	{
		base::new_shape(self.renderer.tinted(model.0, transform,
			texture.0, tc.0, tint, blending, fog, camera))
	}

	#[inline(always)]
	fn shape_complex(&mut self, model: &Model, transform: Transform,
		texture: &Texture, tc: TexCoords, tints: Gradient,
		blending: bool, fog: bool, camera: bool) -> Shape
	{
		base::new_shape(self.renderer.complex(model.0, transform,
			texture.0, tc.0, tints.0, blending, fog, camera))
	}

	fn transform(&mut self, shape: &Shape, transform: Transform) {
		self.renderer.transform(&base::get_shape(shape), transform);
	}

	fn resize(&mut self, wh: (u32, u32)) -> () {
		self.renderer.resize(wh);
	}

	fn wh(&self) -> (u32, u32) {
		self.window.wh()
	}
}
