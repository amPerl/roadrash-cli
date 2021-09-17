use binrw::BinRead;

#[derive(BinRead, Debug)]
#[br(magic = b"CRSR")]
pub struct RsrcHeader {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub entries_offset: u32,
    pub data_offset: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
}

#[derive(BinRead, Debug)]
pub struct RsrcEntries {
    #[br(map = |x: [u8; 4]| String::from_utf8_lossy(&x).to_string().chars().rev().collect())]
    pub kind: String,
    pub data_offset: u32,
    pub entry_count: u32,
    pub unk3: u32,
    #[br(count = entry_count)]
    pub sub_entries: Vec<RsrcEntry>,
}

#[derive(BinRead, Debug)]
pub struct RsrcEntry {
    pub kind: RsrcEntryKind,
    pub index: u32,
    pub data_offset: u32,
    pub data_size: u32,
    pub runtime_fields: (u32, u32, u32, u32),
}

#[derive(BinRead, Debug)]
pub enum RsrcEntryKind {
    #[br(magic = b"MINA")]
    Anim,
    #[br(magic = b"SNAC")]
    Cans,
    #[br(magic = b" LEC")]
    Cel,
    #[br(magic = b" DON")]
    Nod,
    #[br(magic = b" SGS")]
    Sgs,
    Unknown(
        #[br(map = |x: [u8; 4]| String::from_utf8_lossy(&x).to_string().chars().rev().collect())]
        String,
    ),
}

impl std::fmt::Display for RsrcEntryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RsrcEntryKind::Anim => write!(f, "ANIM"),
            RsrcEntryKind::Cans => write!(f, "CANS"),
            RsrcEntryKind::Cel => write!(f, "CEL"),
            RsrcEntryKind::Nod => write!(f, "NOD"),
            RsrcEntryKind::Sgs => write!(f, "SGS"),
            RsrcEntryKind::Unknown(kind) => write!(f, "{}", kind),
        }
    }
}
