use bevy::{
    math::Vec3,
    prelude::{OrthographicCameraBundle, OrthographicProjection},
    render::camera::{DepthCalculation, ScalingMode, Camera2d}
};



pub fn new_camera_2d() -> OrthographicCameraBundle<Camera2d> {
  let far = 1000.0;
  let mut camera = OrthographicCameraBundle::new_2d();
  camera.orthographic_projection = OrthographicProjection {
    far,
    depth_calculation: DepthCalculation::ZDifference,
    scaling_mode: ScalingMode::FixedHorizontal,
    ..Default::default()
  };
  camera.transform.scale = Vec3::new(15., 15., 1.);
  return camera;
}
