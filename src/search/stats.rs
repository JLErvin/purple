pub struct Stats {
    pub nodes: usize,
    leaf_nodes: usize,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            nodes: 0,
            leaf_nodes: 0,
        }
    }

    pub fn reset(&mut self) {
        self.nodes = 0;
        self.leaf_nodes = 0;
    }

    pub fn count_node(&mut self) {
        self.nodes += 1;
    }
}
