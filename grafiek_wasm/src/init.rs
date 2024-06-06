use wgpu::Features;

pub struct WgpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub instance: wgpu::Instance,
}

pub async fn new_ctx() -> Result<WgpuContext, String> {
    let instance = wgpu::Instance::default();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await.ok_or("failed to get adapter!")?;

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: Features::default() | Features::PUSH_CONSTANTS,
                ..Default::default()
            },
            None,
        )
        .await
        .map_err(|_|"Failed to get device and queue")?;

    device.on_uncaptured_error(Box::new(|e| match e {
        wgpu::Error::OutOfMemory { .. } => {
            panic!("Out Of GPU Memory! bailing");
        }
        wgpu::Error::Validation {
            description,
            source,
        } => {
            panic!("{description} : {source}");
        }
    }));

    Ok(WgpuContext {
        device,
        queue,
        instance,
    })
}
