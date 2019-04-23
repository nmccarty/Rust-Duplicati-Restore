use base64::*;

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

pub fn base64_url_to_plain(url: &str) -> String {
    base64::encode(&base64::decode_config(url, base64::URL_SAFE).unwrap())
}
