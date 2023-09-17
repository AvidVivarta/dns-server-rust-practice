#[derive(Debug)]
pub struct DnsBytePacketBuffer {
    buf: [u8; 512],
    pos: usize,
}

impl DnsBytePacketBuffer {

    pub fn new() -> DnsBytePacketBuffer {
        DnsBytePacketBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }

    pub fn pos(&self) ->usize {
        self.pos
    }
}

