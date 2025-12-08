use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use shared::{dbg_and_time, get_day_input, glam::Vec3A, lazy_input, ordered_float, Lazy};
lazy_input!(8);

fn main() {
    dbg_and_time!(part1());
    dbg_and_time!(part2());
}

const SAMPLE_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

fn part1() -> usize {
    let input = &INPUT;
    // let input = SAMPLE_INPUT;

    let points = input
        .lines()
        .map(|line| {
            let mut nums = line.split(",").map(|num| match num.parse::<f32>() {
                Ok(ok) => ok,
                Err(_) => panic!("{num}"),
            });
            let mut get_num = || nums.next().unwrap();
            let vec = Vec3A::new(get_num(), get_num(), get_num());

            vec
        })
        .collect::<Vec<_>>();
    let dists = points
        .iter()
        .enumerate()
        .flat_map(|(i, &i_point)| {
            points.iter().enumerate().filter_map(move |(j, &j_point)| {
                let dist = (i != j).then_some((i_point - j_point).length())?;
                Some((ordered_float::OrderedFloat(dist), (i, j)))
            })
        })
        .collect::<BTreeMap<_, _>>();
    let top_n = 1000;
    let connections =
        dists
            .into_iter()
            .take(top_n)
            .fold(HashMap::new(), |mut map, (_dist, (start, end))| {
                map.entry(start).or_insert(Vec::new()).push(end);
                map.entry(end).or_default().push(start);
                map
            });
    let mut queue = connections.keys().copied().collect::<HashSet<_>>();

    let mut subgraph_sizes = vec![];

    while let Some(current) = queue.iter().copied().next() {
        let mut size = 0;
        dfs(&mut queue, current, &connections, &mut size);
        subgraph_sizes.push(size);
    }
    subgraph_sizes.sort();
    let top_sizes = 3;
    subgraph_sizes
        .iter()
        .rev()
        .take(top_sizes)
        .product::<usize>()
}

fn dfs(
    queue: &mut HashSet<usize>,
    current: usize,
    connections: &HashMap<usize, Vec<usize>>,
    graph_size: &mut usize,
) {
    if !queue.remove(&current) {
        return;
    }
    *graph_size += 1;
    for connection in connections.get(&current).unwrap() {
        dfs(queue, *connection, connections, graph_size);
    }
}

fn part2() -> usize {
    let input = &INPUT;
    // let input = SAMPLE_INPUT;

    let points = input
        .lines()
        .map(|line| {
            let mut nums = line.split(",").map(|num| match num.parse::<f32>() {
                Ok(ok) => ok,
                Err(_) => panic!("{num}"),
            });
            let mut get_num = || nums.next().unwrap();
            let vec = Vec3A::new(get_num(), get_num(), get_num());

            vec
        })
        .collect::<Vec<_>>();

    let dists = points
        .iter()
        .enumerate()
        .flat_map(|(i, &i_point)| {
            points.iter().enumerate().filter_map(move |(j, &j_point)| {
                let dist = (i != j).then_some((i_point - j_point).length())?;
                Some((ordered_float::OrderedFloat(dist), (i, j)))
            })
        })
        .collect::<BTreeMap<_, _>>();

    let mut connection_iter = dists.into_iter().map(|(_, connection)| connection);

    let mut graphs = HashMap::new();
    let mut graph_id = 0;
    let mut belongs_to = HashMap::new();
    loop {
        let Some((node0, node1)) = connection_iter.next() else {
            panic!()
        };
        let graph_id0 = belongs_to.get(&node0).copied();
        let graph_id1 = belongs_to.get(&node1).copied();

        match (graph_id0, graph_id1, node0, node1) {
            (None, None, _, _) => {
                graphs.insert(graph_id, vec![node0, node1]);
                belongs_to.insert(node0, graph_id);
                belongs_to.insert(node1, graph_id);
                graph_id += 1;
            }
            (None, Some(graph_id), to_push, _) | (Some(graph_id), None, _, to_push) => {
                graphs.get_mut(&graph_id).unwrap().push(to_push);
                belongs_to.insert(to_push, graph_id);
            }
            (Some(graph0), Some(graph1), _, _) => {
                if graph0 != graph1 {
                    let to_merge = graphs.remove(&graph1).unwrap();
                    for id in &to_merge {
                        *belongs_to.get_mut(id).unwrap() = graph0;
                    }
                    graphs.get_mut(&graph0).unwrap().extend(to_merge);
                }
            }
        }
        if belongs_to.len() == points.len() && graphs.len() == 1 {
            break points[node0].x as usize * points[node1].x as usize;
        }
    }
}
