use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    rc::Rc,
};

use common::read_input;

#[derive(Debug, Clone, Ord, Eq, PartialOrd)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let mut split = value.split_whitespace().filter(|s| !s.is_empty());
        let fs = split.next().unwrap();
        let mut partial_fs = fs.split('-');
        partial_fs.next().unwrap();
        let (x, y) = (partial_fs.next().unwrap(), partial_fs.next().unwrap());
        let x = x.strip_prefix('x').unwrap().parse().unwrap();
        let y = y.strip_prefix('y').unwrap().parse().unwrap();
        let size = split
            .next()
            .unwrap()
            .strip_suffix('T')
            .unwrap()
            .parse()
            .unwrap();
        let used = split
            .next()
            .unwrap()
            .strip_suffix('T')
            .unwrap()
            .parse()
            .unwrap();
        Self { x, y, size, used }
    }
}

impl Node {
    fn avail(&self) -> usize {
        self.size - self.used
    }

    fn is_empty(&self) -> bool {
        self.used == 0
    }

    fn will_fit(&self, other: &Node) -> bool {
        self.used <= other.avail()
    }

    fn is_viable_pair(&self, other: &Node) -> bool {
        !self.is_empty() && self != other && self.will_fit(other)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ClusterState {
    data_x: usize,
    data_y: usize,
    zero_x: usize,
    zero_y: usize,
}

impl From<&Cluster> for ClusterState {
    fn from(value: &Cluster) -> Self {
        let (zero_x, zero_y) = value.get_zero_pos();
        Self {
            data_x: value.data_x,
            data_y: value.data_y,
            zero_x,
            zero_y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Cluster {
    nodes: Rc<Vec<Node>>,
    rows: usize,
    cols: usize,
    data_x: usize,
    data_y: usize,
}

impl From<&str> for Cluster {
    fn from(value: &str) -> Self {
        let nodes = value
            .trim()
            .lines()
            .skip(2)
            .map(|l| l.into())
            .collect::<Vec<Node>>();
        let (rows, cols) = nodes.iter().fold((1, 1), |(rows, cols), node| {
            let cols = if node.x > cols { node.x + 1 } else { cols };
            let rows = if node.y > rows { node.y + 1 } else { rows };
            (rows, cols)
        });
        Self {
            nodes: nodes.into(),
            rows,
            cols,
            data_x: cols - 1,
            data_y: 0,
        }
    }
}

fn heuristic(data_x: usize, data_y: usize) -> usize {
    data_x + data_y
}

impl Cluster {
    fn viable_pairs(&self) -> Vec<(Node, Node)> {
        self.nodes
            .iter()
            .flat_map(|node| {
                vec![node.clone(); self.nodes.len()]
                    .into_iter()
                    .zip(self.nodes.iter().cloned())
                    .collect::<Vec<(Node, Node)>>()
            })
            .filter(|(node, other)| node.is_viable_pair(other))
            .collect()
    }

    fn viable_neighbors(&self) -> Vec<(Node, Node)> {
        self.nodes
            .iter()
            .filter(|node| node.used == 0)
            .flat_map(|node| {
                let others = self
                    .neighbors(node)
                    .into_iter()
                    .cloned()
                    .filter(|neighbor| neighbor.is_viable_pair(node))
                    .collect::<Vec<_>>();
                vec![node.clone(); others.len()]
                    .into_iter()
                    .zip(others)
                    .collect::<Vec<(Node, Node)>>()
            })
            .collect()
    }

    fn get_node(&self, x: usize, y: usize) -> &Node {
        self.nodes
            .iter()
            .find(|node| node.x == x && node.y == y)
            .unwrap()
    }

    fn get_zero_pos(&self) -> (usize, usize) {
        self.nodes
            .iter()
            .filter(|node| node.used == 0)
            .map(|node| (node.x, node.y))
            .nth(0)
            .unwrap()
    }

    fn neighbors(&self, node: &Node) -> Vec<&Node> {
        let mut neighbors = vec![];
        if node.x > 0 {
            neighbors.push(self.get_node(node.x - 1, node.y));
        }
        if node.y > 0 {
            neighbors.push(self.get_node(node.x, node.y - 1));
        }
        if node.x < self.cols - 1 {
            neighbors.push(self.get_node(node.x + 1, node.y));
        }
        if node.y < self.rows - 1 {
            neighbors.push(self.get_node(node.x, node.y + 1));
        }
        neighbors
    }

    fn move_data(&self, from: &Node, to: &Node) -> Cluster {
        let new_from = Node {
            used: 0,
            ..from.clone()
        };
        let new_to = Node {
            used: to.used + from.used,
            ..to.clone()
        };
        let (data_x, data_y) = if from.x == self.data_x && from.y == self.data_y {
            (to.x, to.y)
        } else {
            (self.data_x, self.data_y)
        };
        let mut new_nodes = (*self.nodes).clone();
        new_nodes.iter_mut().for_each(|node| {
            if *node == *from {
                *node = new_from.clone();
            }
            if *node == *to {
                *node = new_to.clone();
            }
        });

        Cluster {
            nodes: new_nodes.into(),
            data_x,
            data_y,
            ..self.clone()
        }
    }

    fn move_data_to_goal(&self) -> usize {
        let mut queue = BinaryHeap::new();
        let mut visited: HashSet<ClusterState> = HashSet::new();
        let start_state: ClusterState = self.into();
        queue.push((
            Reverse(heuristic(self.data_x, self.data_y)),
            0,
            self.clone(),
        ));
        visited.insert(start_state);

        while let Some((_, depth, cluster)) = queue.pop() {
            if cluster.data_x == 0 && cluster.data_y == 0 {
                return depth;
            }
            cluster.viable_neighbors().iter().for_each(|(from, to)| {
                let new_cluster = cluster.move_data(to, from);
                let new_cluster_state = (&new_cluster).into();
                if visited.insert(new_cluster_state) {
                    let new_depth = depth + 1;
                    let priority = new_depth + heuristic(new_cluster.data_x, new_cluster.data_y);
                    queue.push((Reverse(priority), new_depth, new_cluster));
                }
            });
        }
        0
    }
}

fn main() {
    let input = read_input("day22.txt");
    let cluster = Cluster::from(input.as_str());
    println!("Part 1 = {}", cluster.viable_pairs().len());
    println!("Part 2 = {}", cluster.move_data_to_goal());
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "/dev/grid/node-x0-y0     91T   66T    25T   72%";
        let node = Node::from(input);
        assert_eq!(
            node,
            Node {
                x: 0,
                y: 0,
                size: 91,
                used: 66
            }
        );
        assert_eq!(node.avail(), 25);
    }

    #[test]
    fn part2() {
        let input = r#"root@ebhq-gridcenter# df -h
Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%"#;
        let cluster = Cluster::from(input);
        assert_eq!(cluster.move_data_to_goal(), 7);
    }

    #[test]
    fn test_available_moves() {
        let input = r#"root@ebhq-gridcenter# df -h
Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    0T     2T   77%
/dev/grid/node-x1-y1    8T    7T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%"#;
        let cluster = Cluster::from(input);
        let node_to = Node::from("/dev/grid/node-x2-y0   10T    6T     4T   60%");
        let node_from = Node::from("/dev/grid/node-x1-y0    9T    0T     2T   77%");
        assert!(cluster.viable_neighbors().contains(&(node_from, node_to)));
    }
}
