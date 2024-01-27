use std::{ffi::c_void, sync::Arc};

use ash::*;
use inline_spirv::include_spirv;

use crate::{PrimitiveBatch, Scene};

use super::{vulkan_atlas::VulkanAtlas, vulkan_pipeline::Pipeline};

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
    pipeline: Pipeline,
    framebuffers: Vec<vk::Framebuffer>,
    desc_pool: vk::DescriptorPool,
    desc_set: vk::DescriptorSet,
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

            let pipeline = Pipeline::build_pipeline(
                &device,
                include_spirv!("src/platform/windows/shaders/quad_vertex.glsl", vert),
                include_spirv!("src/platform/windows/shaders/quad_fragment.glsl", frag),
            );

            let framebuffers = image_views
                .iter()
                .map(|iv| {
                    device
                        .create_framebuffer(
                            &vk::FramebufferCreateInfo::default()
                                .render_pass(pipeline.renderpass)
                                .width(1424)
                                .height(714)
                                .layers(1)
                                .attachments(&[*iv]),
                            None,
                        )
                        .unwrap()
                })
                .collect::<Vec<_>>();

            let desc_pool = device
                .create_descriptor_pool(
                    &vk::DescriptorPoolCreateInfo::default()
                        .max_sets(1)
                        .pool_sizes(&[vk::DescriptorPoolSize::default()
                            .descriptor_count(1)
                            .ty(vk::DescriptorType::STORAGE_BUFFER)]),
                    None,
                )
                .unwrap();
            let desc_set = device
                .allocate_descriptor_sets(
                    &vk::DescriptorSetAllocateInfo::default()
                        .descriptor_pool(desc_pool)
                        .set_layouts(&[pipeline.desc_set_layout]),
                )
                .unwrap()[0];

            let buffer = device
                .create_buffer(
                    &vk::BufferCreateInfo::default()
                        .size(32 * 1024 * 1024)
                        .usage(vk::BufferUsageFlags::STORAGE_BUFFER)
                        .sharing_mode(vk::SharingMode::EXCLUSIVE)
                        .queue_family_indices(&[0]),
                    None,
                )
                .unwrap();
            let device_mem = device
                .allocate_memory(
                    &vk::MemoryAllocateInfo::default()
                        .allocation_size(32 * 1024 * 1024)
                        .memory_type_index(2),
                    None,
                )
                .unwrap();
            device.bind_buffer_memory(buffer, device_mem, 0).unwrap();
            let mapped = device
                .map_memory(device_mem, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())
                .unwrap();

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
                pipeline,
                framebuffers,
                desc_pool,
                desc_set,
                buffer,
                mapped,
            }
        }
    }

    pub fn sprite_atlas(&self) -> &Arc<VulkanAtlas> {
        &self.sprite_atlas
    }

    pub fn draw(&mut self, scene: &Scene) {
        unsafe {
            for batch in scene.batches() {
                #[allow(clippy::single_match)]
                match batch {
                    PrimitiveBatch::Quads(quads) => {
                        std::ptr::copy_nonoverlapping(
                            quads.as_ptr() as *const u8,
                            self.mapped as *mut u8,
                            std::mem::size_of_val(quads),
                        );

                        self.device.update_descriptor_sets(
                            &[vk::WriteDescriptorSet::default()
                                .dst_set(self.desc_set)
                                .descriptor_count(1)
                                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                                .buffer_info(&[vk::DescriptorBufferInfo::default()
                                    .buffer(self.buffer)
                                    .range(vk::WHOLE_SIZE)])],
                            &[],
                        );

                        let fence = self
                            .device
                            .create_fence(&vk::FenceCreateInfo::default(), None)
                            .unwrap();

                        let (index, _) = self
                            .swapchain_loader
                            .acquire_next_image(
                                self.swapchain,
                                u64::MAX,
                                vk::Semaphore::null(),
                                fence,
                            )
                            .unwrap();

                        self.device
                            .wait_for_fences(&[fence], true, u64::MAX)
                            .unwrap();

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
                            .begin_command_buffer(
                                cmd_buffer,
                                &vk::CommandBufferBeginInfo::default(),
                            )
                            .unwrap();

                        self.device.cmd_begin_render_pass(
                            cmd_buffer,
                            &vk::RenderPassBeginInfo::default()
                                .render_pass(self.pipeline.renderpass)
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
                        self.device.cmd_bind_pipeline(
                            cmd_buffer,
                            vk::PipelineBindPoint::GRAPHICS,
                            self.pipeline.pipeline,
                        );
                        self.device.cmd_bind_descriptor_sets(
                            cmd_buffer,
                            vk::PipelineBindPoint::GRAPHICS,
                            self.pipeline.pipeline_layout,
                            0,
                            &[self.desc_set],
                            &[],
                        );
                        self.device.cmd_push_constants(
                            cmd_buffer,
                            self.pipeline.pipeline_layout,
                            vk::ShaderStageFlags::VERTEX,
                            0,
                            bytemuck::cast_slice(&[1424, 714]),
                        );
                        self.device
                            .cmd_draw(cmd_buffer, 6, quads.len() as u32, 0, 0);
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
                    _ => {}
                }
            }
        }
    }
}
