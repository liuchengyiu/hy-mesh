use react_mqtt::init;
pub trait FrameProcessor {
    fn on_new_frame(&self, frame:&[u8]);
    fn get_frame_type(&self) -> u8;
}

pub struct FrameProcessorMaster {
    pub frame_processors: Vec<Box<dyn FrameProcessor>>
}

impl FrameProcessorMaster {
    pub fn register_frame_processor(&mut self, frame_processor: Box<dyn FrameProcessor>) {
        self.frame_processors.push(frame_processor);
    }
    pub fn dispatch_new_frame(&self, frame: &[u8]) {
        for processor in &self.frame_processors {
            if processor.get_frame_type() != frame[3] {
                continue;
            }
            processor.on_new_frame(frame);
        }
    }
    pub fn del_frame_processor(&mut self, frame_type: u8) {
        let mut dels: Vec<usize> = vec![];
        for i in 0 .. self.frame_processors.len() {
            if self.frame_processors[i].get_frame_type() == frame_type {
                dels.push(i);
            }
        }
        for i in dels {
            self.frame_processors.remove(i);
        }
    }
}

pub struct Processor90 {
    pub frame_type: u8
}
#[derive(Debug)]
pub struct NodeStatus {
    pub mac: Vec<u8>,
    pub net_routes: Vec<NetRoute>,
    pub nbrs: Vec<NetNbr>
}
#[derive(Debug)]
pub struct NetRoute {
    pub parent_mac: Vec<u8>,
    pub pan_id: u16,
    pub layer: u8,
    pub rank: u32
}
#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct NetNbr {
    pub nbr_mac: Vec<u8>,
    pub rssi: u8,
    pub lqi: u8
}

impl FrameProcessor for Processor90 {
    fn get_frame_type(&self) -> u8{
        self.frame_type
    }
    fn on_new_frame(&self, frame: &[u8]) {
        let mut mac: Vec<u8> = vec![];
        let mut index: usize = 4;
        let mut node_status = NodeStatus{
            mac: Vec::new(),
            net_routes: Vec::new(),
            nbrs: Vec::new(),
        };

        for i in (index + 1) .. (index + 17) {
            match frame.get(i) {
                Some(x) => mac.push(*x),
                None => return                
            }
        }
        index = index + 17 + 32;
        
        match frame.get(index + 1) {
            Some(_) => {},
            None => return
        }
        node_status.mac = mac;
        if frame[index] == 0 && frame[index + 1] == 0 {
            return;
        }
        let route_num: u8 = frame[index];
        index = index + 1;
        for _i in 0 .. route_num {
            let mut parent_mac: Vec<u8> = vec![];
            let mut pan_id: u16 = 0;
            let mut layer: u8 = 0;
            let mut rank: u32 = 0;

            match frame.get(index + (16 + 2 + 1 + 4) ) {
                Some(_) => {},
                None => return
            }
            for j in index .. (index + 16) {
                parent_mac.push(frame[j]);
            }
            index = index + 16;
            pan_id = frame[index] as u16 + frame[index + 1] as u16 * 256;
            index = index + 2;
            layer = frame[index];
            index = index + 1;
            rank = frame[index] as u32 + frame[index + 1] as u32 * 256 + frame[index + 2] as u32 * 256u32.pow(2) + frame[index + 3] as u32 * 256u32.pow(3);
            index = index + 4;
            node_status.net_routes.push(NetRoute{
                parent_mac: parent_mac,
                pan_id: pan_id,
                layer: layer,
                rank: rank
            })
        }
        match frame.get(index) {
            Some(_) => {},
            None => return
        }
        let nbr_num: u8 = frame[index];
        index = index + 1;
        for _i in 0 .. nbr_num {
            let mut nbr_mac: Vec<u8> = vec![];
            let mut rssi: u8 = 0;
            let mut lqi: u8 = 0;

            if frame.len() < (index + 16 + 1 + 1) {
                return;
            }
            for j in index .. (index + 16) {
                nbr_mac.push(frame[j]);
            }
            index = index + 16;
            rssi = frame[index];
            index = index + 1;
            lqi = frame[index];
            index = index + 1;
            node_status.nbrs.push(NetNbr{
                nbr_mac: nbr_mac,
                rssi: rssi,
                lqi: lqi
            })
        }
        println!("{:?}", node_status);
        init::deal_node_status(node_status);
    }
}

pub struct Processor92 {
    pub frame_type: u8
}

impl FrameProcessor for Processor92 {
    fn get_frame_type(&self) -> u8{
        self.frame_type
    }
    fn on_new_frame(&self, frame: &[u8]) {
        let mut index: usize = 4;
        let mut leave_node: Vec<Vec<u8>> = Vec::new();
        match frame.get(index) {
            Some(_) => {},
            None => return
        }
        let num = frame[index];
        index = index + 1;
        for _i in 0 .. num {
            let mut mac: Vec<u8> = vec![];
            if frame.len() < index + 16 {
                return;
            }
            for j in index .. (index + 16) {
                mac.push(frame[j]);
            }
            index = index + 16;
            leave_node.push(mac);
        }
        init::deal_node_leave(leave_node.clone());
    }
}

pub struct Processor97 {
    pub frame_type: u8
}

impl FrameProcessor for Processor97 {
    fn get_frame_type(&self) -> u8{
        self.frame_type
    }
    fn on_new_frame(&self, frame: &[u8]) {
        let mut mac: Vec<u8> = vec![];
        let mut index: usize = 4;
        let mut node_status = NodeStatus{
            mac: Vec::new(),
            net_routes: Vec::new(),
            nbrs: Vec::new(),
        };
        for i in index .. (index + 16) {
            match frame.get(i) {
                Some(x) => mac.push(*x),
                None => return                
            }
        }
        index = index + 16;
        
        match frame.get(index + 1) {
            Some(_) => {},
            None => return
        }
        node_status.mac = mac;
        init::recv_node_reponse(&node_status.mac.clone());
        if frame[index] == 0 && frame[index + 1] == 0 {
            init::deal_node_status(node_status);
            return;
        }
        let route_num: u8 = frame[index];
        index = index + 1;
        for _i in 0 .. route_num {
            let mut parent_mac: Vec<u8> = vec![];
            let mut pan_id: u16 = 0;
            let mut layer: u8 = 0;
            let mut rank: u32 = 0;

            match frame.get(index + (16 + 2 + 1 + 4) ) {
                Some(_) => {},
                None => return
            }
            for j in index .. (index + 16) {
                parent_mac.push(frame[j]);
            }
            index = index + 16;
            pan_id = frame[index] as u16 + frame[index + 1] as u16 * 256;
            index = index + 2;
            layer = frame[index];
            index = index + 1;
            rank = frame[index] as u32 + frame[index + 1] as u32 * 256 + frame[index + 2] as u32 * 256u32.pow(2) + frame[index + 3] as u32 * 256u32.pow(3);
            index = index + 4;
            node_status.net_routes.push(NetRoute{
                parent_mac: parent_mac,
                pan_id: pan_id,
                layer: layer,
                rank: rank
            })
        }
        match frame.get(index) {
            Some(_) => {},
            None => return
        }
        let nbr_num: u8 = frame[index];
        index = index + 1;
        for _i in 0 .. nbr_num {
            let mut nbr_mac: Vec<u8> = vec![];
            let mut rssi: u8 = 0;
            let mut lqi: u8 = 0;

            if frame.len() < (index + 16 + 1 + 1) {
                return;
            }
            for j in index .. (index + 16) {
                nbr_mac.push(frame[j]);
            }
            index = index + 16;
            rssi = frame[index];
            index = index + 1;
            lqi = frame[index];
            index = index + 1;
            node_status.nbrs.push(NetNbr{
                nbr_mac: nbr_mac,
                rssi: rssi,
                lqi: lqi
            })
        }
        println!("{:?}", node_status);
        init::deal_node_status(node_status);
    }
}