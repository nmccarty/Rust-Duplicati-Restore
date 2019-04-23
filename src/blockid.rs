pub struct BlockToFile {
    pub(self) id: i32,
    pub(self) block_id: String,
    pub(self) file_num: i64,
}

impl BlockToFile {
    pub fn new(id: i32, block_id: String, file_num: i64) -> BlockToFile {
        BlockToFile {
            id,
            block_id,
            file_num,
        }
    }
}
