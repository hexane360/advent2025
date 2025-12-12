use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

use petgraph::{Direction::Outgoing, algo::toposort, graph::{DiGraph, NodeIndex}};

use super::input_dir;


pub fn parse_dag<R: BufRead>(file: R) -> Result<(DiGraph<(), ()>, HashMap<Box<str>, NodeIndex<u32>>), String> {
    let mut graph = DiGraph::new();
    let mut node_map: HashMap<Box<str>, NodeIndex<u32>> = HashMap::new();

    let mut get_node = |s: &str| -> NodeIndex<u32> {
        node_map.get(s).copied().unwrap_or_else(|| {
            let idx = graph.add_node(());
            node_map.insert(s.into(), idx);
            idx
        })
    };

    let mut edges: Vec<(NodeIndex<u32>, NodeIndex<u32>)> = Vec::new();

    for line in file.lines() {
        let line = line.map_err(|e| format!("Error reading file: {e}"))?;
        if line.trim().len() == 0 { continue; }

        let (node, outgoing) = line.split_once(": ").ok_or_else(|| format!("Invalid line: {line}"))?;
        let outgoing: Vec<_> = outgoing.split_whitespace().collect();

        let node = get_node(node);
        for out_node in outgoing {
            edges.push((node, get_node(out_node)));
        }
    }

    edges.into_iter().for_each(|(i, o)| { graph.add_edge(i, o, ()); });
    Ok((graph, node_map))
}


pub fn run(test: bool) -> Result<(), String> {
    let mut input_path = input_dir().to_owned();
    input_path.push(if test { "day11_test.txt" } else { "day11.txt" });

    let file = File::open(input_path).expect("Failed to open input file");

    let (graph, node_map) = parse_dag(BufReader::new(file))?;

    let you_node = *node_map.get("you").ok_or_else(|| format!("Couldn't find 'you' node"))?;
    let out_node = *node_map.get("out").ok_or_else(|| format!("Couldn't find 'out' node"))?;

    println!("nodes: {}\nedges: {}", graph.node_count(), graph.edge_count());

    let mut weights = vec![0u64; graph.node_count()];
    weights[you_node.index()] = 1;

    let visit_order = toposort(&graph, None).map_err(|_| format!("Cyclic graph"))?;

    for node in visit_order.iter().copied() {
        //println!("Visiting {}...", node.index());
        for out in graph.neighbors_directed(node, Outgoing) {
            weights[out.index()] += weights[node.index()];
        }
    }
    println!("Part 1 # paths: {}", weights[out_node.index()]);

    let dac_node = *node_map.get("dac").ok_or_else(|| format!("Couldn't find 'dac' node"))?;
    let fft_node = *node_map.get("fft").ok_or_else(|| format!("Couldn't find 'fft' node"))?;
    let svr_node = *node_map.get("svr").ok_or_else(|| format!("Couldn't find 'svr' node"))?;

    // none, one, both
    let mut weights = vec![[0u64; 3]; graph.node_count()];
    weights[svr_node.index()][0] += 1;

    for node in visit_order.into_iter() {
        let mut in_weight = weights[node.index()];
        if node == fft_node || node == dac_node {
            //println!("weight at fft/dac node: {:?}", in_weight);
            // none => one, one => both
            in_weight = [0, in_weight[0], in_weight[1] + in_weight[2]];
        }

        for out in graph.neighbors_directed(node, Outgoing) {
            weights[out.index()].iter_mut().zip(in_weight).for_each(|(l, r)| *l += r);
        }
    }
    println!("paths: {:?}", weights[out_node.index()]);
    println!("Part 2 # paths: {}", weights[out_node.index()][2]);

    Ok(())
}