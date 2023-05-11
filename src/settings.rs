use crate::color::Color;
use crate::scenes::SceneType;
use crate::time::Time;
use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct CameraSettings {
    pub cam_pos: Vec3,
    pub look_at: Vec3,
    pub vup: Vec3,
    pub vfov: f32,
    pub aspect_ratio: f32,
    pub aperture: f32,
    pub focus_dist: f32,
    pub time: Time,
}

#[derive(Debug, Clone, Copy)]
pub struct ImageSettings {
    pub image_width: usize,
    pub image_height: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct SceneSettings {
    pub background: Color,
    pub scene_type: SceneType,
    pub time: Time,
}

#[derive(Debug, Clone, Copy)]
pub struct RendererSettings {
    pub samples_per_pixel: usize,
    pub max_depth: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub camera: CameraSettings,
    pub image: ImageSettings,
    pub scene: SceneSettings,
    pub renderer: RendererSettings,
}

impl Settings {
    pub fn from_scene_type(scene_type: SceneType) -> Self {
        let mut settings = Self::default();
        settings.scene.scene_type = scene_type;

        match scene_type {
            SceneType::BookCover => {
                settings.camera.aperture = 0.1;
            }
            SceneType::SimpleLight => {
                settings.camera.cam_pos = Point3::new(0.0, 3.0, 26.0);
                settings.camera.look_at = Point3::new(0.0, 2.0, 0.0);
                settings.scene.background = Color::new(0.0, 0.0, 0.0);
            }
            SceneType::CornellBox | SceneType::CornellSmoke => {
                settings.camera.aspect_ratio = 1.0;
                settings.camera.cam_pos = Point3::new(278.0, 278.0, -800.0);
                settings.camera.look_at = Point3::new(278.0, 278.0, 0.0);
                settings.camera.vfov = 40.0;
                settings.scene.background = Color::new(0.0, 0.0, 0.0);
            }
            SceneType::FinalScene => {
                settings.camera.aspect_ratio = 1.0;
                settings.camera.cam_pos = Point3::new(478.0, 278.0, -600.0);
                settings.camera.look_at = Point3::new(278.0, 278.0, 0.0);
                settings.camera.vfov = 40.0;
                settings.scene.background = Color::new(0.0, 0.0, 0.0);
            }
            _ => {}
        }

        settings.image.image_width =
            (settings.image.image_height as f32 * settings.camera.aspect_ratio) as usize;

        settings
    }
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            cam_pos: Vec3::new(13.0, 2.0, 3.0),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            vfov: 20.0,
            aspect_ratio: 16.0 / 9.0,
            aperture: 0.0,
            focus_dist: 10.0,
            time: Time::new(0.0, 1.0),
        }
    }
}

impl Default for ImageSettings {
    fn default() -> Self {
        Self {
            image_width: 1280,
            image_height: 720,
        }
    }
}

impl Default for SceneSettings {
    fn default() -> Self {
        Self {
            background: Color::new(0.7, 0.8, 1.0),
            scene_type: SceneType::SimpleLight,
            time: Time::new(0.0, 1.0),
        }
    }
}

impl Default for RendererSettings {
    fn default() -> Self {
        Self {
            samples_per_pixel: 500,
            max_depth: 50,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            camera: CameraSettings::default(),
            image: ImageSettings::default(),
            scene: SceneSettings::default(),
            renderer: RendererSettings::default(),
        }
    }
}
