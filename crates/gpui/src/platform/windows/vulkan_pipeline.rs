use ash::*;
use cstr::cstr;

pub struct Pipeline {
    pub renderpass: vk::RenderPass,
    pub desc_set_layout: vk::DescriptorSetLayout,
    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
}

impl Pipeline {
    pub fn build_pipeline(device: &Device, vert_code: &[u32], frag_code: &[u32]) -> Self {
        unsafe {
            let renderpass = Self::create_renderpass(device);
            let desc_set_layout = Self::create_descriptor_set_layout(device);
            let pipeline_layout = Self::create_pipeline_layout(device, desc_set_layout);
            let vert_shader_module = Self::create_shader_module(device, vert_code);
            let frag_shader_module = Self::create_shader_module(device, frag_code);
            let vertex_input = vk::PipelineVertexInputStateCreateInfo::default();
            let tessellation = vk::PipelineTessellationStateCreateInfo::default();
            let depth_stencil = vk::PipelineDepthStencilStateCreateInfo::default();
            let dynamic = vk::PipelineDynamicStateCreateInfo::default();

            let vert_shader_stage = vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vert_shader_module)
                .name(cstr!("main"));

            let frag_shader_stage = vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(frag_shader_module)
                .name(cstr!("main"));

            let stages = [vert_shader_stage, frag_shader_stage];

            let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
                .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

            let scissor = [vk::Rect2D {
                offset: vk::Offset2D::default(),
                extent: vk::Extent2D {
                    width: 1424,
                    height: 714,
                },
            }];

            let viewport = vk::PipelineViewportStateCreateInfo::default()
                .viewports(&[vk::Viewport {
                    x: 0.0,
                    y: 0.0,
                    width: 1424.0,
                    height: 714.0,
                    min_depth: 0.0,
                    max_depth: 1.0,
                }])
                .scissors(&scissor);

            let rasterization = vk::PipelineRasterizationStateCreateInfo::default()
                .polygon_mode(vk::PolygonMode::FILL)
                .line_width(1.0);

            let multisample = vk::PipelineMultisampleStateCreateInfo::default()
                .rasterization_samples(vk::SampleCountFlags::TYPE_1);

            let color_blend_attachment = [vk::PipelineColorBlendAttachmentState::default()
                .blend_enable(true)
                .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
                .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
                .color_blend_op(vk::BlendOp::ADD)
                .src_alpha_blend_factor(vk::BlendFactor::ONE)
                .dst_alpha_blend_factor(vk::BlendFactor::ONE)
                .alpha_blend_op(vk::BlendOp::ADD)
                .color_write_mask(vk::ColorComponentFlags::RGBA)];

            let color_blend = vk::PipelineColorBlendStateCreateInfo::default()
                .attachments(&color_blend_attachment);

            let create_info = vk::GraphicsPipelineCreateInfo::default()
                .stages(&stages)
                .vertex_input_state(&vertex_input)
                .input_assembly_state(&input_assembly)
                .tessellation_state(&tessellation)
                .viewport_state(&viewport)
                .rasterization_state(&rasterization)
                .multisample_state(&multisample)
                .depth_stencil_state(&depth_stencil)
                .color_blend_state(&color_blend)
                .dynamic_state(&dynamic)
                .layout(pipeline_layout)
                .render_pass(renderpass);

            let pipeline = device
                .create_graphics_pipelines(vk::PipelineCache::null(), &[create_info], None)
                .unwrap()[0];

            Self {
                renderpass,
                desc_set_layout,
                pipeline_layout,
                pipeline,
            }
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

    fn create_descriptor_set_layout(device: &Device) -> vk::DescriptorSetLayout {
        let descriptor_set_layout_binding = vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT);

        let create_info = vk::DescriptorSetLayoutCreateInfo::default()
            .bindings(std::slice::from_ref(&descriptor_set_layout_binding));

        unsafe { device.create_descriptor_set_layout(&create_info, None) }.unwrap()
    }

    fn create_pipeline_layout(
        device: &Device,
        desc_set_layout: vk::DescriptorSetLayout,
    ) -> vk::PipelineLayout {
        let push_constant_range = vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::VERTEX)
            .offset(0)
            .size(8);

        let create_info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(std::slice::from_ref(&desc_set_layout))
            .push_constant_ranges(std::slice::from_ref(&push_constant_range));

        unsafe { device.create_pipeline_layout(&create_info, None) }.unwrap()
    }

    fn create_shader_module(device: &Device, code: &[u32]) -> vk::ShaderModule {
        let create_info = vk::ShaderModuleCreateInfo::default().code(code);

        unsafe { device.create_shader_module(&create_info, None) }.unwrap()
    }
}
