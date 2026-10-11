//! Test if a separate Vulkan device can import DMA-BUF and share with wgpu

use ash::vk;
use std::os::fd::AsRawFd;

fn main() {
    // Create a separate Vulkan instance
    let entry = unsafe { ash::Entry::load().expect("Failed to load Vulkan") };
    
    let app_info = vk::ApplicationInfo::default()
        .api_version(vk::API_VERSION_1_2);
    
    let instance_create_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info);
    
    let instance = unsafe { entry.create_instance(&instance_create_info, None) }
        .expect("Failed to create Vulkan instance");
    
    // Enumerate physical devices
    let physical_devices = unsafe { instance.enumerate_physical_devices() }
        .expect("Failed to enumerate physical devices");
    
    println!("Found {} physical devices", physical_devices.len());
    
    for (i, &physical_device) in physical_devices.iter().enumerate() {
        let props = unsafe { instance.get_physical_device_properties(physical_device) };
        let device_name = unsafe {
            std::ffi::CStr::from_ptr(props.device_name.as_ptr())
                .to_string_lossy()
        };
        
        // Check extensions
        let extensions = unsafe { instance.enumerate_device_extension_properties(physical_device) }
            .expect("Failed to enumerate extensions");
        
        let has_external_memory_fd = extensions.iter().any(|ext| {
            let name = unsafe { std::ffi::CStr::from_ptr(ext.extension_name.as_ptr()) };
            name.to_bytes() == b"VK_KHR_external_memory_fd"
        });
        
        let has_external_memory_dma_buf = extensions.iter().any(|ext| {
            let name = unsafe { std::ffi::CStr::from_ptr(ext.extension_name.as_ptr()) };
            name.to_bytes() == b"VK_EXT_external_memory_dma_buf"
        });
        
        let has_image_drm_format_modifier = extensions.iter().any(|ext| {
            let name = unsafe { std::ffi::CStr::from_ptr(ext.extension_name.as_ptr()) };
            name.to_bytes() == b"VK_EXT_image_drm_format_modifier"
        });
        
        println!("Device {}: {}", i, device_name);
        println!("  VK_KHR_external_memory_fd: {}", has_external_memory_fd);
        println!("  VK_EXT_external_memory_dma_buf: {}", has_external_memory_dma_buf);
        println!("  VK_EXT_image_drm_format_modifier: {}", has_image_drm_format_modifier);
        
        if has_external_memory_fd && has_external_memory_dma_buf {
            println!("  -> This device supports DMA-BUF import!");
        }
    }
    
    unsafe { instance.destroy_instance(None) };
}