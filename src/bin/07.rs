use std::ops::Add;

#[derive(Debug)]
struct Obj {
    name: String,
    is_dir: bool,
    size: Option<usize>,
}

#[derive(Debug)]
struct Arena {
    nodes: Vec<Node>,
}

#[derive(Debug, Clone, Copy)]
struct NodeId {
    index: usize,
}

impl NodeId {
    fn new(index: usize) -> Self {
        NodeId { index }
    }
}

impl Arena {
    fn new() -> Self {
        Arena { nodes: vec![] }
    }

    fn get(&self, id: NodeId) -> &Obj {
        &self.nodes.get(id.index).unwrap().data
    }

    fn add(&mut self, obj: Obj, parent_id: Option<NodeId>) -> NodeId {
        let node_id = NodeId::new(self.nodes.len());
        let new_node = Node {
            id: node_id,
            data: obj,
            children: vec![],
            parent: parent_id,
        };
        self.nodes.push(new_node);
        if let Some(parnet) = parent_id {
            self.nodes
                .get_mut(parnet.index)
                .unwrap()
                .children
                .push(node_id);
        }
        node_id
    }

    fn get_children(&self, id: NodeId) -> Vec<&NodeId> {
        self.nodes
            .get(id.index)
            .unwrap()
            .children
            .iter()
            .map(|x| &self.nodes.get(x.index).unwrap().id)
            .collect()
    }

    fn get_parent(&self, id: NodeId) -> Option<NodeId> {
        self.nodes.get(id.index).unwrap().parent
    }

    fn format_node(&self, id: NodeId, depth: usize) -> String {
        let children = self
            .get_children(id)
            .iter()
            .map(|x| self.format_node(**x, depth + 1))
            .collect::<Vec<String>>()
            .join("\n")
            .split('\n')
            .map(|x| format!("{}{}{}", "\n", "\t".repeat(depth), x))
            .collect::<String>();
        let children_str = format!("{}", children);
        format!(
            "Node(id:{}, data:{:#?}, children:{})",
            id.index,
            self.get(id),
            children_str
        )
    }

    fn print(&self, id: NodeId) {
        print!("{}", self.format_node(id, 0))
    }

    fn get_size(&self, id: NodeId) -> usize {
        let node = self.get(id);
        if node.is_dir {
            self.get_children(id)
                .iter()
                .map(|x| self.get_size(**x))
                .sum()
        } else {
            node.size.unwrap()
        }
    }
    fn get_ids(&self) -> Vec<NodeId> {
        (0..self.nodes.len()).map(|x| NodeId::new(x)).collect()
    }
}

#[derive(Debug)]
struct Node {
    id: NodeId,
    data: Obj,
    children: Vec<NodeId>,
    parent: Option<NodeId>,
}

fn parse_instructions(input: &str) -> Arena {
    let mut arena = Arena::new();
    let mut cursor = NodeId::new(0);
    let mut is_ls = false;
    arena.add(
        Obj {
            name: "/".into(),
            is_dir: true,
            size: None,
        },
        None,
    );
    for line in input.lines().filter(|x| !x.is_empty()) {
        let mut line_segments = line.split_whitespace();
        if line_segments.next().unwrap() == "$" {
            match line_segments.next().unwrap() {
                "cd" => {
                    let current_dir = line_segments.next().unwrap();
                    if current_dir == "/" {
                        continue;
                    } else if current_dir == ".." {
                        cursor = arena.get_parent(cursor).unwrap()
                    } else {
                        cursor = **arena
                            .get_children(cursor)
                            .iter()
                            .find(|x| arena.get(***x).name == current_dir)
                            .unwrap();
                    }
                }
                "ls" => is_ls = true,
                _ => todo!(),
            }
        } else if is_ls {
            let (part_1, part_2) = line.split_once(' ').unwrap();
            if part_1 == "dir" {
                arena.add(
                    Obj {
                        name: part_2.into(),
                        is_dir: true,
                        size: None,
                    },
                    Some(cursor),
                );
            } else {
                arena.add(
                    Obj {
                        name: part_2.into(),
                        is_dir: false,
                        size: Some(part_1.parse::<usize>().unwrap()),
                    },
                    Some(cursor),
                );
            }
        }
    }
    arena
}

pub fn part_one(input: &str) -> Option<usize> {
    let tree = parse_instructions(input);
    let mut size = 0;
    for id in tree.get_ids() {
        let node = tree.get(id);
        if node.is_dir {
            let node_size = tree.get_size(id);
            if node_size < 100000 {
                size += node_size;
            }
        }
    }
    Some(size)
}

pub fn part_two(input: &str) -> Option<usize> {
    let tree = parse_instructions(input);
    let total_space: usize = 70000000;
    let update_space: usize = 30000000;
    let used_space = tree.get_size(NodeId::new(0));
    Some(
        tree.get_ids()
            .iter()
            .filter(|x| tree.get(**x).is_dir)
            .map(|x| tree.get_size(*x))
            .filter(|size| (total_space - (used_space - size)) >= update_space)
            .min()
            .unwrap(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
