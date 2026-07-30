use crate::data_structures::dsu::DSU;
use crate::graphs::graph::Edge;

pub fn mst(graph: &mut Vec<Edge>) -> Vec<Edge> {
    graph.sort_by(|x, y| x.weight().unwrap().cmp(&y.weight().unwrap()));
    let n = graph.len();
    let mut dsu = DSU::new(n);

    let mut res: Vec<Edge> = vec![];
    for edge in graph {
        if dsu.get(edge.from()) != dsu.get(edge.to()) {
            dsu.merge(edge.from(), edge.to());
            res.push(edge.clone());
        }
    }

    return res;
}
