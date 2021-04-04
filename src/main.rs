fn main() {
    let (_tx, rx) = std::sync::mpsc::channel::<()>();
    let instance = wgpu::Instance::new(wgpu::BackendBit::VULKAN);

    let adapter_options = wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: None,
    };

    let adapter = pollster::block_on(async { instance.request_adapter(&adapter_options).await.unwrap() });

    let (device, _queue) = pollster::block_on(async {
        adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap()
    });
    let device = std::sync::Arc::new(device);

    let dev = std::sync::Arc::clone(&device);
    let _handle = std::thread::spawn(move || {
        let _block_until_tx_drop = rx.recv();
        println!("{:?}", dev);
    });
}
