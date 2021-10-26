use packing::{Packed, PackedSize};

// https://www.mikrocontroller.net/attachment/41812/0x23_READFMT_070123.pdf: Table 704
#[derive(Clone, Copy, Eq, PartialEq, Debug, Packed)]
#[packed(big_endian, lsb0)]
pub struct CurrentCapacityDescriptor {
    #[pkd(7, 0, 0, 3)]
    pub number_of_blocks: u32,

    #[pkd(7, 2, 4, 4)]
    pub reserved: u8,

    #[pkd(1, 0, 4, 4)]
    pub descriptor_type: u8,

    #[pkd(7, 0, 5, 8)]
    pub block_length: u32,
}

// https://www.mikrocontroller.net/attachment/41812/0x23_READFMT_070123.pdf: Table 701
#[derive(Clone, Copy, Eq, PartialEq, Debug, Packed)]
#[packed(big_endian, lsb0)]
pub struct FormatingCapacities {
    #[pkd(7, 0, 0, 2)]
    pub reserved: u32, //remove

    #[pkd(7, 0, 3, 3)]
    pub list_length: u8,

    #[pkd(7, 0, 4, 13)]
    pub descriptor: CurrentCapacityDescriptor,
}
impl FormatingCapacities {
    pub fn new(block_count: u32, block_length: usize) -> Self {
        Self {
            reserved: 0,
            list_length: CurrentCapacityDescriptor::BYTES as u8,
            descriptor: CurrentCapacityDescriptor {
                number_of_blocks: block_count,
                reserved: 0,
                descriptor_type: 0b10,
                block_length: block_length as u32,
            },
         }
     }
}