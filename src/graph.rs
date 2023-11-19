use cargo_snippet::snippet;

#[snippet("graph_distance_without_weight")]
fn graph_distance_without_weight_from(
    graph: Vec<Vec<usize>>,
    start_node: usize,
) -> Vec<Option<usize>> {
    let node_num = graph.len();
    let mut distances = vec![None; node_num];

    distances[start_node] = Some(0);

    let mut schedules = vec![start_node];

    while let Some(here) = schedules.pop() {
        let here_distance = distances[here].unwrap();
        let next_distance = Some(here_distance + 1);
        for &next_node in graph[here].iter() {
            if distances[next_node].is_none() {
                distances[next_node] = next_distance;
                schedules.push(next_node);
            }
        }
    }

    distances
}

#[snippet("graph_distance_with_weight")]
fn graph_distance_with_weight_from(
    graph_to_weight: Vec<Vec<(usize, isize)>>,
    start_node: usize,
) -> Option<Vec<Option<isize>>> {
    // 負の閉回路が存在するときはNoneを返す。

    // 予定をheapに書き出す。
    // heapから読みだしたときに、対象nodeのdistanceがnoneであれば、ようやくそのnodeの距離が確定する

    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let node_num = graph_to_weight.len();
    let mut distances = vec![None; node_num];

    let mut schedules = BinaryHeap::new();
    schedules.push((Reverse(0), start_node));

    while let Some(here) = schedules.pop() {
        let here_distance_candidate = here.0 .0;
        let here_node = here.1;

        if distances[here_node].is_none() {
            let here_distance = here_distance_candidate;
            distances[here_node] = Some(here_distance);
            for &(next_node, weight) in graph_to_weight[here_node].iter() {
                if distances[next_node].is_none() {
                    let next_distance = here_distance as isize + weight;
                    schedules.push((Reverse(next_distance), next_node))
                }
            }
        }
    }

    // 負の閉回路が存在するかを確認
    for (from, to_weight_list) in graph_to_weight.iter().enumerate() {
        let from_distamce = if let Some(x) = distances[from] {
            x
        } else {
            continue;
        };

        for &(to, weight) in to_weight_list {
            let to_distance = if let Some(x) = distances[to] {
                x
            } else {
                continue;
            };

            let d = to_distance - from_distamce;
            if d > weight {
                return None;
            }
        }
    }
    Some(distances)
}
