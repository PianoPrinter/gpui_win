use ash::*;
use cstr::cstr;

pub struct Pipeline {
    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
}

impl Pipeline {
    pub fn build_pipeline(
        device: &Device,
        vert_code: &[u32],
        frag_code: &[u32],
        renderpass: vk::RenderPass,
        desc_set_layout: vk::DescriptorSetLayout,
        width: i32,
        height: i32,
    ) -> Self {
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
                width: width as u32,
                height: height as u32,
            },
        }];

        let vp = [vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: width as f32,
            height: height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];
        let viewport = vk::PipelineViewportStateCreateInfo::default()
            .viewports(&vp)
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

        let color_blend =
            vk::PipelineColorBlendStateCreateInfo::default().attachments(&color_blend_attachment);

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

        let pipeline = unsafe {
            device.create_graphics_pipelines(vk::PipelineCache::null(), &[create_info], None)
        }
        .unwrap()[0];

        Self {
            pipeline_layout,
            pipeline,
        }
    }

    pub fn bind(
        &self,
        device: &Device,
        cmd_buffer: vk::CommandBuffer,
        offset: u32,
        push_constants: &[u8],
        desc_set: vk::DescriptorSet,
    ) {
        unsafe {
            device.cmd_bind_pipeline(cmd_buffer, vk::PipelineBindPoint::GRAPHICS, self.pipeline);
            device.cmd_bind_descriptor_sets(
                cmd_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout,
                0,
                &[desc_set],
                &[offset],
            );
            device.cmd_push_constants(
                cmd_buffer,
                self.pipeline_layout,
                vk::ShaderStageFlags::VERTEX,
                0,
                push_constants,
            );
        }
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
