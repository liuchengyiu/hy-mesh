use super::mesh_h::*;

static mut MESH_MASTER: FrameProcessorMaster =  FrameProcessorMaster{frame_processors:Vec::new()};

pub fn init_mesh_processors() {
    let processor_90 = Processor90{frame_type: 144};
    let processor_92 = Processor92{frame_type: 146};
    let processor_97 = Processor97{frame_type: 151};
    unsafe {
        MESH_MASTER.register_frame_processor(Box::new(processor_90));
        MESH_MASTER.register_frame_processor(Box::new(processor_92));
        MESH_MASTER.register_frame_processor(Box::new(processor_97));
    }
}

pub fn process_new_frame(frame: &[u8]) {
    unsafe {
        MESH_MASTER.dispatch_new_frame(frame);
    }
}