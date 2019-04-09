type NodeIndex = usize;
type EdgeIndex<'a> = &'a (usize, usize);

#[derive(Debug)]
pub struct NodeData {
    name: &'static str,
    weight: u32,
}

pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<(usize, usize)>,
}

impl<'a> dot::Labeller<'a, NodeIndex, EdgeIndex<'a>> for Graph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("exampl").unwrap()
    }
    fn node_id(&'a self, n: &NodeIndex) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n)).unwrap()
    }
    fn node_label<'b>(&'b self, n: &NodeIndex) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(self.nodes[*n].name.into())
    }
    fn kind(&self) -> dot::Kind {
        dot::Kind::Graph
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
        // keep only half of the edges b/c undirected graph
        let mut iter = self.edges.iter();
        let mut edge_list = vec![];

        while let Some(x) = iter.nth(1) {
            edge_list.push(x);
        }

        edge_list.into()
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

    pub fn add_node(&mut self, id: &'static str, weight: u32) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            name: id,
            weight: weight,
        });
        
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
