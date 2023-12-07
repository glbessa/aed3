pub struct Edge<V: Eq + PartialEq + Display + Clone> {
    src: V,
    dst: V,
    weight: u64
}

impl<V: Eq + PartialEq + Display + Clone> Edge<V> {
    pub fn new(src: V, dst: V, weight: u64) -> Self {
        Edge {
            src,
            dst,
            weight
        }
    }

    pub fn get_src(&self) -> &V {
        &self.src
    }

    pub fn get_dst(&self) -> &V {
        &self.dst
    }

    pub fn get_weight(&self) -> u64 {
        self.weight
    }
}