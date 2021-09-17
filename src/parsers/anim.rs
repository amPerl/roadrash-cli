use binrw::BinRead;

#[derive(BinRead, Debug)]
pub struct RsrcAnimEntry {
    pub kind: [u8; 4],
    pub next_offset: u32,
    #[br(args { kind })]
    pub data: RsrcAnimEntryData,
}

#[derive(BinRead, Debug)]
#[br(import { kind: [u8; 4] })]
pub enum RsrcAnimEntryData {
    #[br(pre_assert(&kind == b"NARR"))]
    Rran {
        three_more_offsets: [u32; 3],
    },
    #[br(pre_assert(&kind == b"DFRR"))]
    Rrfd {
        #[br(count = 85)]
        unk: Vec<u32>,
    },
    #[br(pre_assert(&kind == b"MINA"))]
    Anim {
        #[br(count = 2)]
        unk1: Vec<u32>,
        rrpd_count: u32,
        #[br(count = 8)]
        unk2: Vec<u32>,
    },
    #[br(pre_assert(&kind == b"RCRR"))]
    Rrcr,
    #[br(pre_assert(&kind == b"TULP"))]
    Plut {
        #[br(count = 17)]
        unk: Vec<u32>,
    },
    #[br(pre_assert(&kind == b"DPRR"))]
    Rrpd {
        unk: u32,
        #[br(if(unk == 12345678))]
        data: Option<RsrcAnimRrpdData>,
    },

    Unknown,
}

#[derive(BinRead, Debug)]
pub struct RsrcAnimRrpdData {
    #[br(count = 12)]
    pub pre_name_unk: Vec<u8>,
    #[br(map = |x: [u8; 8]| String::from_utf8_lossy(&x).to_string().trim_end_matches('\0').to_string())]
    pub name: String,
    pub post_name_unk1: [u32; 2],
    pub mip1_width: u32,
    pub mip1_height: u32,
    pub mip_count: u32,
    pub post_name_unk2: u32,
    #[br(count = mip_count)]
    pub mip_headers: Vec<(u32, u32, u32)>,
}
