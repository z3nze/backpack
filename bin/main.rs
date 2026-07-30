use backpack::graphs::graph::Edge;
use backpack::graphs::kruskal::mst;
use backpack::io::console::Scanner;

fn main() {
    let mut sc = Scanner::default();
    let mut graph: Vec<Edge> = vec![];
    let mut wg = 0;
    for u in 0..40 {
        let s: String = sc.read();
        let se = s.split(",").collect::<Vec<_>>();
        for v in u + 1..40 {
            let e = se.get(v).unwrap();
            if e == &"-" {
                continue;
            }
            graph.push(Edge::new_weighted(u, v, e.parse().unwrap()));
            wg += e.parse::<usize>().unwrap();
        }
    }

    let mst_res = mst(&mut graph);
    let ans = mst_res.iter().map(|x| x.weight().unwrap()).sum::<usize>();
    println!("{}", wg - ans);
}
