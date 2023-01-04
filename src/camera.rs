use std::f32::consts::PI;

use cgmath::Point3;
use winit::event::{DeviceEvent, ElementState, MouseScrollDelta};

use crate::state::OPENGL_TO_WGPU_MATRIX;

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);

        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}
pub struct CameraController {
    last_mouse_position: [f64; 2],
    delta_mouse: [f64; 2],
    radius: f32,
    yaw: f32,
    pitch: f32,
    right_click: bool,
    was_right_click: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            last_mouse_position: [0., 0.],
            delta_mouse: [0., 0.],
            radius: 4.,
            yaw: 0.,
            pitch: 0.,
            right_click: false,
            was_right_click: false,
        }
    }

    pub fn process_events(&mut self, event: &DeviceEvent) -> bool {
        match event {
            DeviceEvent::MouseMotion { delta, .. } => {
                if self.right_click {
                    self.delta_mouse = [-delta.0, delta.1];
                    self.was_right_click = self.right_click;
                }
                true
            }
            DeviceEvent::Button { state, button, .. } => {
                if *button == 3 {
                    if let ElementState::Pressed = state {
                        self.right_click = true;
                    } else {
                        self.right_click = false;
                        self.delta_mouse = [0., 0.];
                        self.last_mouse_position = [0., 0.];
                    }
                }
                true
            }
            DeviceEvent::MouseWheel { delta } => {
                if let MouseScrollDelta::LineDelta(_, y) = delta {
                    self.radius += y * 0.01;
                }
                true
            }
            _ => false,
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        self.yaw += self.delta_mouse[0] as f32 * 0.05;
        self.pitch += self.delta_mouse[1] as f32 * 0.05;
        self.pitch = self.pitch.clamp(-PI/2., PI/2.);

        camera.eye = Point3::new(self.radius * self.yaw.sin() * self.pitch.cos(),
            self.radius * self.pitch.sin(),
            self.radius * self.yaw.cos() * self.pitch.cos());
        self.delta_mouse = [0., 0.];
    }
}
