type NodeIndex = usize;
type EdgeIndex<'a> = &'a (usize, usize);

pub struct Graph {
    nodes: Vec<&'static str>,
    edges: Vec<(usize, usize)>,
}

impl<'a> dot::Labeller<'a, NodeIndex, EdgeIndex<'a>> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("graph").unwrap()
    }
    fn node_id(&'a self, n: &NodeIndex) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n)).unwrap()
    }
    fn node_label<'b>(&'b self, n: &NodeIndex) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(self.nodes[*n].into())
    }
    // fn edge_label<'b>(&'b self, _: &EdgeIndex) -> dot::LabelText<'b> {
    //     dot::LabelText::LabelStr("&sube;".into())
    // }
}

impl<'a> dot::GraphWalk<'a, NodeIndex, EdgeIndex<'a>> for Graph {
    fn nodes(&self) -> dot::Nodes<'a, NodeIndex> {
        (0..self.nodes.len()).collect()
    }
    fn edges(&'a self) -> dot::Edges<'a, EdgeIndex<'a>> {
        self.edges.iter().collect()
    }
    fn source(&self, e: &EdgeIndex) -> NodeIndex {
        e.0
    }
    fn target(&self, e: &EdgeIndex) -> NodeIndex {
        e.1
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_node(&mut self, id: &'static str) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(id);
        index
    }

    pub fn add_edge(&mut self, src: NodeIndex, dest: NodeIndex) {
        self.add_edge_dir(src, dest);
        self.add_edge_dir(dest, src);
    }

    pub fn add_edge_dir(&mut self, src: NodeIndex, dest: NodeIndex) {
        self.edges.push((src, dest));
    }

    pub fn dump(&self) {
        println!("v: {:?}", self.nodes);
        println!("e: {:?}", self.edges);
    }
}

#[cfg(test)]
mod test {}
