use super::mesh_h::*;

static mut MESH_PROCESSORS: FrameProcessorMaster =  FrameProcessorMaster{frame_processors:Vec::new()};

pub fn init_mesh_processors() {
    let processor_90 = Processor90{frame_type: 144};
    let processor_92 = Processor92{frame_type: 146};
    let processor_97 = Processor97{frame_type: 151};
    let processor_AA = ProcessorAA{frame_type: 170};
    unsafe {
        MESH_PROCESSORS.register_frame_processor(Box::new(processor_90));
        MESH_PROCESSORS.register_frame_processor(Box::new(processor_92));
        MESH_PROCESSORS.register_frame_processor(Box::new(processor_97));
        MESH_PROCESSORS.register_frame_processor(Box::new(processor_AA));
    }
}

pub fn process_new_frame(frame: &[u8]) {
    unsafe {
        MESH_PROCESSORS.dispatch_new_frame(frame);
    }
}