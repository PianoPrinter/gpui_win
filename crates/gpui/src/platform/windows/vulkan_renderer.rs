mod pipeline;

use std::{ffi::c_void, sync::Arc};

use ash::*;
use inline_spirv::include_spirv;

use crate::{PrimitiveBatch, Scene};

use self::pipeline::Pipeline;

use super::vulkan_atlas::VulkanAtlas;

#[allow(unused)]
pub(crate) struct VulkanRenderer {
    sprite_atlas: Arc<VulkanAtlas>,
    entry: Entry,
    instance: Instance,
    surface: vk::SurfaceKHR,
    pdevice: vk::PhysicalDevice,
    device: Device,
    swapchain_loader: extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
    images: Vec<vk::Image>,
    queue: vk::Queue,
    cmd_pool: vk::CommandPool,
    renderpass: vk::RenderPass,
    quads_pipeline: Pipeline,
    shadows_pipeline: Pipeline,
    underlines_pipeline: Pipeline,
    framebuffers: Vec<vk::Framebuffer>,
    desc_pool: vk::DescriptorPool,
    buffer: vk::Buffer,
    mapped: *mut c_void,
}

impl VulkanRenderer {
    pub fn new(hinstance: isize, hwnd: isize) -> Self {
        unsafe {
            let entry = Entry::load().unwrap();
            let instance = entry
                .create_instance(
                    &vk::InstanceCreateInfo::default().enabled_extension_names(&[
                        extensions::khr::Surface::NAME.as_ptr(),
                        extensions::khr::Win32Surface::NAME.as_ptr(),
                    ]),
                    None,
                )
                .unwrap();
            let win32_surface_loader = extensions::khr::Win32Surface::new(&entry, &instance);
            let surface = win32_surface_loader
                .create_win32_surface(
                    &vk::Win32SurfaceCreateInfoKHR::default()
                        .hinstance(hinstance)
                        .hwnd(hwnd),
                    None,
                )
                .unwrap();
            let pdevice = instance.enumerate_physical_devices().unwrap()[0];
            let device = instance
                .create_device(
                    pdevice,
                    &vk::DeviceCreateInfo::default()
                        .queue_create_infos(&[vk::DeviceQueueCreateInfo::default()
                            .queue_family_index(0)
                            .queue_priorities(&[0.0])])
                        .enabled_features(
                            &vk::PhysicalDeviceFeatures::default().shader_clip_distance(true),
                        )
                        .enabled_extension_names(&[extensions::khr::Swapchain::NAME.as_ptr()]),
                    None,
                )
                .unwrap();
            let swapchain_loader = extensions::khr::Swapchain::new(&instance, &device);
            let swapchain = swapchain_loader
                .create_swapchain(
                    &vk::SwapchainCreateInfoKHR::default()
                        .surface(surface)
                        .min_image_count(2)
                        .image_format(vk::Format::B8G8R8A8_UNORM)
                        .image_color_space(vk::ColorSpaceKHR::SRGB_NONLINEAR)
                        .image_array_layers(1)
                        .image_extent(vk::Extent2D {
                            width: 1424,
                            height: 714,
                        })
                        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
                        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                        .queue_family_indices(&[0])
                        .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
                        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
                        .present_mode(vk::PresentModeKHR::FIFO_RELAXED)
                        .clipped(true),
                    None,
                )
                .unwrap();
            let images = swapchain_loader.get_swapchain_images(swapchain).unwrap();
            let image_views = images
                .iter()
                .map(|image| {
                    device
                        .create_image_view(
                            &vk::ImageViewCreateInfo::default()
                                .image(*image)
                                .view_type(vk::ImageViewType::TYPE_2D)
                                .format(vk::Format::B8G8R8A8_UNORM)
                                .subresource_range(
                                    vk::ImageSubresourceRange::default()
                                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                                        .level_count(1)
                                        .layer_count(1),
                                ),
                            None,
                        )
                        .unwrap()
                })
                .collect::<Vec<_>>();
            let queue = device.get_device_queue(0, 0);
            let cmd_pool = device
                .create_command_pool(
                    &vk::CommandPoolCreateInfo::default().queue_family_index(0),
                    None,
                )
                .unwrap();

            let renderpass = Self::create_renderpass(&device);

            let desc_pool = device
                .create_descriptor_pool(
                    &vk::DescriptorPoolCreateInfo::default()
                        .max_sets(3)
                        .pool_sizes(&[vk::DescriptorPoolSize::default()
                            .descriptor_count(1)
                            .ty(vk::DescriptorType::STORAGE_BUFFER_DYNAMIC)]),
                    None,
                )
                .unwrap();

            let quads_pipeline = Pipeline::build_pipeline(
                &device,
                include_spirv!("src/platform/windows/shaders/quad_vertex.glsl", vert),
                include_spirv!("src/platform/windows/shaders/quad_fragment.glsl", frag),
                renderpass,
                desc_pool,
            );

            let shadows_pipeline = Pipeline::build_pipeline(
                &device,
                include_spirv!("src/platform/windows/shaders/shadow_vertex.glsl", vert),
                include_spirv!("src/platform/windows/shaders/shadow_fragment.glsl", frag),
                renderpass,
                desc_pool,
            );

            let underlines_pipeline = Pipeline::build_pipeline(
                &device,
                include_spirv!("src/platform/windows/shaders/underline_vertex.glsl", vert),
                include_spirv!("src/platform/windows/shaders/underline_fragment.glsl", frag),
                renderpass,
                desc_pool,
            );

            let framebuffers = image_views
                .iter()
                .map(|iv| {
                    device
                        .create_framebuffer(
                            &vk::FramebufferCreateInfo::default()
                                .render_pass(renderpass)
                                .width(1424)
                                .height(714)
                                .layers(1)
                                .attachments(&[*iv]),
                            None,
                        )
                        .unwrap()
                })
                .collect::<Vec<_>>();

            let (buffer, mapped) = Self::create_buffer(&device);

            Self {
                sprite_atlas: Arc::new(VulkanAtlas::new()),
                entry,
                instance,
                surface,
                pdevice,
                device,
                swapchain_loader,
                swapchain,
                images,
                queue,
                cmd_pool,
                renderpass,
                quads_pipeline,
                shadows_pipeline,
                underlines_pipeline,
                framebuffers,
                desc_pool,
                buffer,
                mapped,
            }
        }
    }

    pub fn sprite_atlas(&self) -> &Arc<VulkanAtlas> {
        &self.sprite_atlas
    }

    pub fn draw(&mut self, scene: &Scene) {
        let mut offset = 0;

        unsafe {
            let fence = self.create_fence();

            let (index, _) = self
                .swapchain_loader
                .acquire_next_image(self.swapchain, u64::MAX, vk::Semaphore::null(), fence)
                .unwrap();

            self.device
                .wait_for_fences(&[fence], true, u64::MAX)
                .unwrap();

            let size_batches = std::mem::size_of_val(&scene.batches()) as u64;

            self.device.update_descriptor_sets(
                &[
                    vk::WriteDescriptorSet::default()
                        .dst_set(self.quads_pipeline.desc_set)
                        .descriptor_count(1)
                        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER_DYNAMIC)
                        .buffer_info(&[vk::DescriptorBufferInfo::default()
                            .buffer(self.buffer)
                            .range(size_batches)]),
                    vk::WriteDescriptorSet::default()
                        .dst_set(self.shadows_pipeline.desc_set)
                        .descriptor_count(1)
                        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER_DYNAMIC)
                        .buffer_info(&[vk::DescriptorBufferInfo::default()
                            .buffer(self.buffer)
                            .range(size_batches)]),
                ],
                &[],
            );

            let cmd_buffer = self
                .device
                .allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo::default()
                        .command_pool(self.cmd_pool)
                        .level(vk::CommandBufferLevel::PRIMARY)
                        .command_buffer_count(1),
                )
                .unwrap()[0];

            self.device
                .begin_command_buffer(cmd_buffer, &vk::CommandBufferBeginInfo::default())
                .unwrap();

            self.device.cmd_begin_render_pass(
                cmd_buffer,
                &vk::RenderPassBeginInfo::default()
                    .render_pass(self.renderpass)
                    .clear_values(&[vk::ClearValue {
                        color: vk::ClearColorValue {
                            float32: [0.0, 0.0, 0.0, 1.0],
                        },
                    }])
                    .render_area(vk::Rect2D {
                        offset: vk::Offset2D { x: 0, y: 0 },
                        extent: vk::Extent2D {
                            width: 1424,
                            height: 714,
                        },
                    })
                    .framebuffer(self.framebuffers[index as usize]),
                vk::SubpassContents::INLINE,
            );

            for batch in scene.batches() {
                match batch {
                    PrimitiveBatch::Quads(quads) => {
                        // temporal fix on alignment, revisit later.
                        offset = ((offset + 255) / 256) * 256;
                        let quad_bytes_len = std::mem::size_of_val(quads);
                        std::ptr::copy_nonoverlapping(
                            quads.as_ptr() as *const u8,
                            (self.mapped as *mut u8).add(offset),
                            quad_bytes_len,
                        );

                        self.quads_pipeline.bind(
                            &self.device,
                            cmd_buffer,
                            offset as u32,
                            bytemuck::cast_slice(&[1424, 714]),
                        );

                        self.device
                            .cmd_draw(cmd_buffer, 6, quads.len() as u32, 0, 0);

                        offset += quad_bytes_len;
                    }
                    PrimitiveBatch::Shadows(shadows) => {
                        // temporal fix on alignment, revisit later.
                        offset = ((offset + 255) / 256) * 256;
                        let shadow_bytes_len = std::mem::size_of_val(shadows);
                        std::ptr::copy_nonoverlapping(
                            shadows.as_ptr() as *const u8,
                            (self.mapped as *mut u8).add(offset),
                            shadow_bytes_len,
                        );

                        self.shadows_pipeline.bind(
                            &self.device,
                            cmd_buffer,
                            offset as u32,
                            bytemuck::cast_slice(&[1424, 714]),
                        );

                        self.device
                            .cmd_draw(cmd_buffer, 6, shadows.len() as u32, 0, 0);

                        offset += shadow_bytes_len;
                    }
                    PrimitiveBatch::Underlines(underlines) => {
                        dbg!(underlines.len());
                    }
                    _ => {}
                }
            }

            self.device.cmd_end_render_pass(cmd_buffer);
            self.device.end_command_buffer(cmd_buffer).unwrap();

            let fence = self
                .device
                .create_fence(&vk::FenceCreateInfo::default(), None)
                .unwrap();

            self.device
                .queue_submit(
                    self.queue,
                    &[vk::SubmitInfo::default().command_buffers(&[cmd_buffer])],
                    fence,
                )
                .unwrap();

            self.device
                .wait_for_fences(&[fence], true, u64::MAX)
                .unwrap();

            self.swapchain_loader
                .queue_present(
                    self.queue,
                    &vk::PresentInfoKHR::default()
                        .swapchains(&[self.swapchain])
                        .image_indices(&[index]),
                )
                .unwrap();
        }
    }

    fn create_renderpass(device: &Device) -> vk::RenderPass {
        let attachment_description = vk::AttachmentDescription::default()
            .format(vk::Format::B8G8R8A8_UNORM)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

        let attachment_reference = vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

        let subpass_description = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(std::slice::from_ref(&attachment_reference));

        let create_info = vk::RenderPassCreateInfo::default()
            .attachments(std::slice::from_ref(&attachment_description))
            .subpasses(std::slice::from_ref(&subpass_description));

        unsafe { device.create_render_pass(&create_info, None) }.unwrap()
    }

    fn create_buffer(device: &Device) -> (vk::Buffer, *mut c_void) {
        let buffer = {
            let create_info = vk::BufferCreateInfo::default()
                .size(32 * 1024 * 1024)
                .usage(vk::BufferUsageFlags::STORAGE_BUFFER)
                .sharing_mode(vk::SharingMode::EXCLUSIVE)
                .queue_family_indices(&[0]);

            unsafe { device.create_buffer(&create_info, None) }.unwrap()
        };

        let device_mem = {
            let allocate_info = vk::MemoryAllocateInfo::default()
                .allocation_size(32 * 1024 * 1024)
                .memory_type_index(2);

            unsafe { device.allocate_memory(&allocate_info, None) }.unwrap()
        };

        unsafe { device.bind_buffer_memory(buffer, device_mem, 0) }.unwrap();

        let mapped = unsafe {
            device.map_memory(device_mem, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())
        }
        .unwrap();

        (buffer, mapped)
    }

    fn create_fence(&self) -> vk::Fence {
        let create_info = vk::FenceCreateInfo::default();

        unsafe { self.device.create_fence(&create_info, None) }.unwrap()
    }
}
