use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::mouse::MouseMotion;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{Material2d, MaterialMesh2dBundle},
};

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let window = windows.get_primary_mut().unwrap();

    let size = Extent3d {
        width: window.width() as u32,
        height: window.height() as u32,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    image.resize(size);

    let image_handle = images.add(image);
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0)),
            },
            transform: Transform::from_xyz(
                (SCREEN_WIDTH / 2) as f32 - 8.0,
                (SCREEN_HEIGHT / 2) as f32 - 8.0,
                999.9,
            ),
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        UiCameraConfig { show_ui: false },
        MainCamera,
    ));

    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        size.width as f32,
        size.height as f32,
    ))));

    let material_handle = post_processing_materials.add(PostProcessingMaterial {
        screen_shape_factor: 0.2,
        rows: 128.0,
        brightness: 3.0,
        edges_transition_size: 0.05,
        channels_mask_min: 0.1,
        source_image: image_handle,
    });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                ..default()
            },
            ..default()
        },
        post_processing_pass_layer,
    ));

    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0)),
            },
            camera: Camera {
                priority: 1,
                ..default()
            },
            ..Camera2dBundle::default()
        },
        post_processing_pass_layer,
    ));
}

pub fn pan_camera(
    mut ev_motion: EventReader<MouseMotion>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    let pan_button = MouseButton::Left;

    let mut pan = Vec2::ZERO;

    if input_mouse.pressed(pan_button) {
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }

    for mut transform in query.iter_mut() {
        transform.translation += Vec3::new(-pan.x, pan.y, 0.0);
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "b17e3ec0-b8e2-4b66-a62e-1ed9f4374350"]
pub struct PostProcessingMaterial {
    #[texture(0)]
    #[sampler(1)]
    source_image: Handle<Image>,

    /// the larger the value, the more rounded the screen (must be between 0 and 1)
    #[uniform(2)]
    screen_shape_factor: f32,

    /// controls amount of screen rows
    #[uniform(3)]
    rows: f32,

    /// screen brightness (I recommend setting it to 3 or 4 if you do not want create a horror game)
    #[uniform(4)]
    brightness: f32,

    /// screen edge shadow effect size
    #[uniform(5)]
    edges_transition_size: f32,

    /// Each pixel contains 3 sub-pixels (red, green and blue).
    /// This option allows you to display the color of all channels in any subpixels.
    /// I really recommend play with it (only use values between 0 and 1)
    #[uniform(6)]
    channels_mask_min: f32,
}

impl Material2d for PostProcessingMaterial {
    fn fragment_shader() -> ShaderRef {
        "post_processing.wgsl".into()
    }
}
