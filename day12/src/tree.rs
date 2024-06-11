pub type GroupLenghts = Vec<u8>;
pub type SpringConditions = Vec<SpringCondition>;

#[derive(Clone, Debug)]
pub enum SpringCondition {
    Operational,
    Damaged,
    Unknown,
}

impl SpringCondition {
    pub fn from(ch: char) -> SpringCondition {
        match ch {
            '.' => SpringCondition::Operational,
            '#' => SpringCondition::Damaged,
            '?' => SpringCondition::Unknown,
            _ => panic!("Invalid spring condition"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub spring_conditions: SpringConditions,
    pub group_lengths: GroupLenghts,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            spring_conditions: vec![SpringCondition::Operational],
            group_lengths: GroupLenghts::new(),
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, condition: SpringCondition) {
        match condition {
            SpringCondition::Operational => self._add_condition_to_leaves(&condition),
            SpringCondition::Damaged => self._add_condition_to_leaves(&condition),
            SpringCondition::Unknown => self._add_level(),
        }
    }

    fn _add_condition_to_leaves(&mut self, new_condition: &SpringCondition) {
        if self._is_leaf() {
            self._update_group_lengths(new_condition);
            self.spring_conditions.push(new_condition.clone());
        } else {
            self.left
                .as_mut()
                .unwrap()
                ._add_condition_to_leaves(new_condition);
            self.right
                .as_mut()
                .unwrap()
                ._add_condition_to_leaves(new_condition);
        }
    }

    fn _update_group_lengths(&mut self, new_condition: &SpringCondition) {
        let condition = self
            .spring_conditions
            .last()
            .unwrap_or(&SpringCondition::Operational);
        match (condition, new_condition) {
            (_, SpringCondition::Operational) => {}
            (SpringCondition::Operational, SpringCondition::Damaged) => self.group_lengths.push(1),
            (SpringCondition::Damaged, SpringCondition::Damaged) => {
                match self.group_lengths.last_mut() {
                    Some(group_length) => *group_length += 1,
                    None => self.group_lengths.push(1),
                }
            }
            _ => panic!("Invalid spring conditions"),
        }
    }

    fn _add_level(&mut self) {
        if self._is_leaf() {
            let left_node = Node::_make_son_of(&self, SpringCondition::Operational);
            let mut right_node = Node::_make_son_of(&self, SpringCondition::Damaged);

            match self.spring_conditions.last().unwrap() {
                SpringCondition::Operational => {
                    right_node.group_lengths.push(1);
                }
                SpringCondition::Damaged => match right_node.group_lengths.last_mut() {
                    Some(group_length) => *group_length += 1,
                    None => right_node.group_lengths.push(1),
                },
                _ => panic!("Node with invalid spring condition"),
            }

            self.left = Some(Box::new(left_node));
            self.right = Some(Box::new(right_node));
        } else {
            self.left.as_mut().unwrap()._add_level();
            self.right.as_mut().unwrap()._add_level();
        }
    }

    fn _make_son_of(node: &Node, spring_condition: SpringCondition) -> Node {
        Node {
            spring_conditions: vec![spring_condition],
            group_lengths: node.group_lengths.clone(),
            left: None,
            right: None,
        }
    }

    pub fn get_leaves(&self) -> Vec<Node> {
        self._get_leaves(Vec::new())
    }

    fn _get_leaves<'a>(&'a self, leaves: Vec<Node>) -> Vec<Node> {
        let mut new_leaves = leaves.clone();
        if self._is_leaf() {
            new_leaves.push(self.clone());
        } else {
            if let Some(ref left) = self.left {
                new_leaves.extend(left._get_leaves(leaves.clone()));
            }
            if let Some(ref right) = self.right {
                new_leaves.extend(right._get_leaves(leaves.clone()));
            }
        }
        new_leaves
    }

    fn _is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}
