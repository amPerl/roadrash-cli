use binrw::BinRead;

#[derive(BinRead, Debug)]
pub struct Dat {
    #[br(parse_with = binrw::until_eof)]
    pub textures: Vec<DatTexture>,
}

#[derive(BinRead, Debug)]
pub struct DatTexture {
    pub width: u8,
    pub height: u8,
    #[br(count = width as usize * height as usize)]
    pub palette_indices: Vec<u8>,
}
