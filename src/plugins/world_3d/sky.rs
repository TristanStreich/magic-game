use bevy::{
    asset::LoadState,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupError, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
            OwnedBindingResource, PreparedBindGroup, RenderPipelineDescriptor, SamplerBindingType,
            ShaderRef, ShaderStages, SpecializedMeshPipelineError, TextureSampleType,
            TextureViewDescriptor, TextureViewDimension,
        },
        renderer::RenderDevice,
        texture::FallbackImage,
    },
};

use crate::plugins::world_3d::config::*;

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(MaterialPlugin::<CubemapMaterial>::default())
        .insert_resource(ClearColor(Color::rgb(0.5294, 0.8087, 0.9216)))
        .add_startup_system(spawn_sun)
        .add_startup_system(init_cube_map)
        .add_system(asset_loaded);
    }
}

fn spawn_sun(
    mut commands: Commands
) {
    commands
    .spawn(DirectionalLightBundle{
        directional_light: DirectionalLight {
            illuminance: SUN_INTENSITY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            SUN_ROTATION.0,
            SUN_ROTATION.1,
            SUN_ROTATION.2)),
        ..default()
    })
    .insert(Name::new("Sun"));
    
    commands.insert_resource(AmbientLight {
        brightness: SUN_AMBIENT_LIGHT,
        ..default()
        });
}


#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}

fn init_cube_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let skybox_handle = asset_server.load("textures/sky_boxes/Ryfjallet_cubemap.png");
    
    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle,
    });
}


fn asset_loaded(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cubemap_materials: ResMut<Assets<CubemapMaterial>>,
    mut cubemap: ResMut<Cubemap>,
) {
    if !cubemap.is_loaded
        && asset_server.get_load_state(cubemap.image_handle.clone_weak()) == LoadState::Loaded
    {
        let mut image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(
                image.texture_descriptor.size.height / image.texture_descriptor.size.width,
            );
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        commands.spawn(MaterialMeshBundle::<CubemapMaterial> {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 10000.0 })),
            material: cubemap_materials.add(CubemapMaterial {
                base_color_texture: Some(cubemap.image_handle.clone_weak()),
            }),
            ..default()
        });

        cubemap.is_loaded = true;
    }
}

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "9509a0f8-3c05-48ee-a13e-a93226c7f488"]
struct CubemapMaterial {
    base_color_texture: Option<Handle<Image>>,
}

impl Material for CubemapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cubemap_unlit.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

impl AsBindGroup for CubemapMaterial {
    type Data = ();

    fn as_bind_group(
        &self,
        layout: &BindGroupLayout,
        render_device: &RenderDevice,
        images: &RenderAssets<Image>,
        _fallback_image: &FallbackImage,
    ) -> Result<PreparedBindGroup<Self>, AsBindGroupError> {
        let base_color_texture = self
            .base_color_texture
            .as_ref()
            .ok_or(AsBindGroupError::RetryNextUpdate)?;
        let image = images
            .get(base_color_texture)
            .ok_or(AsBindGroupError::RetryNextUpdate)?;
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&image.texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&image.sampler),
                },
            ],
            label: Some("cubemap_texture_material_bind_group"),
            layout,
        });

        Ok(PreparedBindGroup {
            bind_group,
            bindings: vec![
                OwnedBindingResource::TextureView(image.texture_view.clone()),
                OwnedBindingResource::Sampler(image.sampler.clone()),
            ],
            data: (),
        })
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                // Cubemap Base Color Texture
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::Cube,
                    },
                    count: None,
                },
                // Cubemap Base Color Texture Sampler
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: None,
        })
    }
}