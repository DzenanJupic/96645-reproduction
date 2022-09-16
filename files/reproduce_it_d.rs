/// https://www.reddit.com/r/rust/comments/xercaw/odd_compile_error_which_resolves_with_new_line/
mod d {
    use std::collections::HashMap;

    pub struct D {
        blocks: HashMap<Hash, Block>,
    }

    #[derive(PartialEq, Eq, Hash)]
    pub struct Hash;
    pub struct Block;

    impl D {
        pub async fn get_mut_block(&mut self, block_hash: &Hash) -> Option<&mut Block> {
            // this comment

            self.blocks.get_mut(block_hash)
        }
    }
}
