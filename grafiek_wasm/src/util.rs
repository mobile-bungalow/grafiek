use futures_intrusive::channel::shared::oneshot_channel;
use log::error;

pub async fn read_texture_contents_to_vec(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
) -> Result<Vec<u8>, String> {
    let width = texture.width();
    let height = texture.height();
    let block_size = texture
        .format()
        .block_copy_size(Some(wgpu::TextureAspect::All))
        .expect("It seems like you are trying to render to a Depth Stencil. Stop that.");
    let mut out = vec![0; (width * height * block_size) as usize];

    let row_byte_ct = block_size * width;
    let padded_row_byte_ct = (row_byte_ct + 255) & !255;

    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Texture Read Buffer"),
        size: (height * padded_row_byte_ct) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Texture Read Encoder"),
    });

    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::ImageCopyBuffer {
            buffer: &buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(padded_row_byte_ct),
                rows_per_image: None,
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );

    queue.submit(Some(encoder.finish()));

    {
        let buffer_slice = buffer.slice(..);

        let (tx, rx) = oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });

        match rx.receive().await {
            Some(res) => match res {
                Err(e) => return Err(format!("{e:?}")),
                _ => {}
            },
            None => return Err("sender channel died".to_owned()),
        };

        device.poll(wgpu::Maintain::Wait);

        let gpu_slice = buffer_slice.get_mapped_range();
        let gpu_chunks = gpu_slice.chunks(padded_row_byte_ct as usize);
        let slice_chunks = &mut out.chunks_mut(row_byte_ct as usize);
        let iter = slice_chunks.zip(gpu_chunks);

        for (output_chunk, gpu_chunk) in iter {
            output_chunk.copy_from_slice(&gpu_chunk[..row_byte_ct as usize]);
        }
    };

    buffer.unmap();
    Ok(out)
}
