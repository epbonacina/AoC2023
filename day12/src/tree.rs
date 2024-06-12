pub type GroupLenghts = Vec<u8>;

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
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, conditions: &[SpringCondition], group_lengths: GroupLenghts) -> u32 {
        self._insert(
            conditions,
            group_lengths,
            &SpringCondition::Operational,
            Vec::new(),
            0,
        )
    }

    pub fn _insert(
        &mut self,
        conditions: &[SpringCondition],
        expected_group_lengths: GroupLenghts,
        previous_condition: &SpringCondition,
        current_group_lengths: GroupLenghts,
        current_count: u32,
    ) -> u32 {
        let mut new_count = current_count;
        if conditions.len() == 0 && current_group_lengths.eq(&expected_group_lengths) {
            new_count += 1;
        }
        if let Some(current_condition) = conditions.first() {
            match (previous_condition, current_condition) {
                (_, SpringCondition::Operational) => {
                    new_count += self._insert_left(
                        conditions,
                        expected_group_lengths,
                        current_condition,
                        current_group_lengths,
                        current_count,
                    );
                }
                (SpringCondition::Operational, SpringCondition::Damaged) => {
                    new_count += self._insert_right_pushing_new_group_length(
                        conditions,
                        expected_group_lengths,
                        current_condition,
                        current_group_lengths,
                        current_count,
                    );
                }
                (SpringCondition::Operational, SpringCondition::Unknown) => {
                    new_count += self._insert_left(
                        conditions,
                        expected_group_lengths.clone(),
                        &SpringCondition::Operational,
                        current_group_lengths.clone(),
                        current_count,
                    );
                    new_count += self._insert_right_pushing_new_group_length(
                        conditions,
                        expected_group_lengths,
                        &SpringCondition::Damaged,
                        current_group_lengths,
                        current_count,
                    );
                }
                (SpringCondition::Damaged, SpringCondition::Damaged) => {
                    new_count += self._insert_right_incrementing_last_group_length(
                        conditions,
                        expected_group_lengths,
                        current_condition,
                        current_group_lengths,
                        current_count,
                    );
                }
                (SpringCondition::Damaged, SpringCondition::Unknown) => {
                    new_count += self._insert_left(
                        conditions,
                        expected_group_lengths.clone(),
                        &SpringCondition::Operational,
                        current_group_lengths.clone(),
                        current_count,
                    );
                    new_count += self._insert_right_incrementing_last_group_length(
                        conditions,
                        expected_group_lengths,
                        &SpringCondition::Damaged,
                        current_group_lengths,
                        current_count,
                    );
                }

                _ => panic!("Invalid condition"),
            }
        }
        new_count
    }

    fn _insert_left(
        &mut self,
        conditions: &[SpringCondition],
        expected_group_lengths: GroupLenghts,
        current_condition: &SpringCondition,
        current_group_lengths: GroupLenghts,
        current_count: u32,
    ) -> u32 {
        let mut count = 0;
        if self.left.is_none() {
            self.left = Some(Box::new(Node::new()));
        }
        if let Some(left_node) = self.left.as_mut() {
            count = left_node._insert(
                &conditions[1..],
                expected_group_lengths,
                current_condition,
                current_group_lengths,
                current_count,
            );
        }
        count
    }

    fn _insert_right_pushing_new_group_length(
        &mut self,
        conditions: &[SpringCondition],
        expected_group_lengths: GroupLenghts,
        current_condition: &SpringCondition,
        mut current_group_lengths: GroupLenghts,
        current_count: u32,
    ) -> u32 {
        let mut count = 0;
        if self.right.is_none() {
            self.right = Some(Box::new(Node::new()));
        }
        if let Some(right_node) = self.right.as_mut() {
            current_group_lengths.push(1);
            count = right_node._insert(
                &conditions[1..],
                expected_group_lengths,
                current_condition,
                current_group_lengths,
                current_count,
            );
        }
        count
    }

    fn _insert_right_incrementing_last_group_length(
        &mut self,
        conditions: &[SpringCondition],
        expected_group_lengths: GroupLenghts,
        current_condition: &SpringCondition,
        mut current_group_lengths: GroupLenghts,
        current_count: u32,
    ) -> u32 {
        let mut count = 0;
        if self.right.is_none() {
            self.right = Some(Box::new(Node::new()));
        }
        if let Some(right_node) = self.right.as_mut() {
            if let Some(group_length) = current_group_lengths.last_mut() {
                *group_length += 1;
            } else {
                current_group_lengths.push(1);
            }
            count = right_node._insert(
                &conditions[1..],
                expected_group_lengths,
                current_condition,
                current_group_lengths,
                current_count,
            );
        }
        count
    }

    fn _is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}
