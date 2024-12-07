use common::read_input;

#[derive(Debug, Clone)]
struct Node {
    _fs: String,
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
        Self {
            _fs: fs.to_string(),
            x,
            y,
            size,
            used,
        }
    }
}

impl Node {
    fn avail(&self) -> usize {
        self.size - self.used
    }

    fn use_percent(&self) -> usize {
        self.used * 100 / self.size
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

struct Cluster {
    nodes: Vec<Node>,
}

impl From<&str> for Cluster {
    fn from(value: &str) -> Self {
        Self {
            nodes: value.trim().lines().skip(2).map(|l| l.into()).collect(),
        }
    }
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
}

fn main() {
    let input = read_input("day22.txt");
    let cluster = Cluster::from(input.as_str());
    println!("Part 1 = {}", cluster.viable_pairs().len());
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
                _fs: "/dev/grid/node-x0-y0".to_string(),
                x: 0,
                y: 0,
                size: 91,
                used: 66
            }
        );
        assert_eq!(node.avail(), 25);
        assert_eq!(node.use_percent(), 72);
    }
}
