use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    }
    println!("{}", resolve(ab));
}

fn resolve(ab: Vec<(usize, usize)>) -> i32 {
    let ab = to_index(ab);
    let graph = make_graph(ab);
    let dist = dfs(&graph, 0);
    let max_dist_index = dist.iter().enumerate().max_by_key(|&(_, x)| x).unwrap().0;
    dfs(&graph, max_dist_index)
        .iter()
        .max()
        .map_or(0, |&x| x + 1)
}

fn to_index(ab: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    ab.iter().map(|(a, b)| (a - 1, b - 1)).collect()
}

fn make_graph(ab: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let max_point = ab.iter().map(|(a, b)| a.max(b)).max().unwrap();
    let mut graph = vec![vec![]; max_point + 1];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph
}

fn dfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<i32> {
    let mut stack = vec![start];
    let mut dist = vec![-1; graph.len()];
    dist[start] = 0;

    while let Some(node) = stack.pop() {
        for &next in &graph[node] {
            if dist[next] == -1 {
                dist[next] = dist[node] + 1;
                stack.push(next);
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        assert_eq!(resolve(vec![(1, 2), (2, 3)]), 3);
        assert_eq!(resolve(vec![(1, 2), (2, 3), (3, 4), (3, 5)]), 4)
    }

    #[test]
    fn test_to_index() {
        assert_eq!(
            to_index(vec![(1, 2), (2, 3), (3, 4)]),
            vec![(0, 1), (1, 2), (2, 3)]
        )
    }

    #[test]
    fn test_make_graph() {
        let ab = vec![(0, 1), (1, 2), (2, 3)];
        let graph = make_graph(ab);
        assert_eq!(graph[0], vec![1]);
        assert_eq!(graph[1], vec![0, 2]);
        assert_eq!(graph[2], vec![1, 3]);
        assert_eq!(graph[3], vec![2]);
    }

    #[test]
    fn test_dfs() {
        assert_eq!(
            dfs(&vec![vec![1], vec![0, 2], vec![1, 3], vec![2],], 0),
            vec![0, 1, 2, 3]
        )
    }
}
