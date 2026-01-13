pub struct Scene {
    id: uint32,
    title: String,
    preview: String,
    text: String,
    tags: array<String>,
    position: uint32,
}

pub fn insert_block(b: block, position: uint32) {
    following_blocks = get_blocks_after(&position);
    for b in following_blocks {
        //increase position of b in database
    }
    //insert b into database
}

fn get_blocks_after(&position: uint32) -> array<Block> {
    //retrieve all blocks from database after position
}
