use binrw::BinRead;

#[derive(BinRead, Debug)]
pub struct Fam {
    pub unk1: u32,
    pub compressed_length: u32,
    pub unk2: u32,
    pub unk3: u32,
    #[br(count = compressed_length)]
    pub compressed_data: Vec<u8>,
}
