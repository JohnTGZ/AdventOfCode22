use aoc_common::FileContents;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Assignments {
        let test_input_filepath = "../input/day4/test_input.txt";
        let split_delim = "\n";

        let file_contents = FileContents::build(test_input_filepath, split_delim, -1, -1)
            .unwrap_or_else(|err| {
                panic!("Unable to parse file: {err}");
            });

        Assignments::build(&file_contents)
    }

    #[test]
    fn test_build() {
        let assignments = setup();

        assert_eq!(2, assignments.left[0][0]);
        assert_eq!(3, assignments.left[1][1]);
        assert_eq!(7, assignments.right[2][0]);
        assert_eq!(7, assignments.right[3][1]);
    }

    #[test]
    fn test_fully_contains() {
        let assignments = setup();

        assert_eq!(
            false,
            Assignments::fully_contains(&assignments.left[0], &assignments.right[0])
        );
        assert_eq!(
            false,
            Assignments::fully_contains(&assignments.left[1], &assignments.right[1])
        );
        assert_eq!(
            false,
            Assignments::fully_contains(&assignments.left[2], &assignments.right[2])
        );
        assert_eq!(
            true,
            Assignments::fully_contains(&assignments.left[3], &assignments.right[3])
        );

        assert_eq!(
            true,
            Assignments::fully_contains(&assignments.left[4], &assignments.right[4])
        );
        assert_eq!(
            false,
            Assignments::fully_contains(&assignments.left[5], &assignments.right[5])
        );
    }

    #[test]
    fn test_count_fully_contains() {
        let assignments = setup();

        assert_eq!(2, assignments.count_fully_contains());
    }

    #[test]
    fn test_partially_contains() {
        let assignments = setup();

        assert_eq!(
            false,
            Assignments::partially_contains(&assignments.left[0], &assignments.right[0])
        );
        assert_eq!(
            false,
            Assignments::partially_contains(&assignments.left[1], &assignments.right[1])
        );
        assert_eq!(
            true,
            Assignments::partially_contains(&assignments.left[2], &assignments.right[2])
        );
        assert_eq!(
            true,
            Assignments::partially_contains(&assignments.left[3], &assignments.right[3])
        );
        assert_eq!(
            true,
            Assignments::partially_contains(&assignments.left[4], &assignments.right[4])
        );
        assert_eq!(
            true,
            Assignments::partially_contains(&assignments.left[5], &assignments.right[5])
        );
    }

    #[test]
    fn test_count_partially_contains() {
        let assignments = setup();

        assert_eq!(4, assignments.count_partially_contains());
    }
}

#[derive(Debug)]
pub struct Assignments {
    pub left: Vec<[i32; 2]>,
    pub right: Vec<[i32; 2]>,
}

impl Assignments {
    pub fn build(file_contents: &FileContents) -> Assignments {
        let (mut left, mut right): (Vec<[i32; 2]>, Vec<[i32; 2]>) = (Vec::new(), Vec::new());

        for content in &file_contents.split_contents {
            let split_assignment_str = FileContents::split_line(content, ",");
            let left_min_max = FileContents::split_into_i32(split_assignment_str[0], "-");
            let right_min_max = FileContents::split_into_i32(split_assignment_str[1], "-");

            left.push([left_min_max[0], left_min_max[1]]);
            right.push([right_min_max[0], right_min_max[1]]);
        }

        Assignments { left, right }
    }

    pub fn fully_contains(left: &[i32; 2], right: &[i32; 2]) -> bool {
        // Which is shorter, left or right?
        let l_len = left[1] - left[0];
        let r_len = right[1] - right[0];

        match l_len <= r_len {
            true => {
                if right[1] >= left[1] && right[0] <= left[0] {
                    return true;
                }
            }
            false => {
                if left[1] >= right[1] && left[0] <= right[0] {
                    return true;
                }
            }
        }

        false
    }

    pub fn count_fully_contains(&self) -> i32 {
        let mut count: i32 = 0;

        for i in 0..self.left.len() {
            match Self::fully_contains(&self.left[i], &self.right[i]) {
                true => count += 1,
                false => (),
            }
        }

        count
    }

    pub fn partially_contains(a: &[i32; 2], b: &[i32; 2]) -> bool {
        //Check for the cases where there are no overlap
        if a[1] < b[0] {
            return false;
        } else if a[0] > b[1] {
            return false;
        }

        true
    }

    pub fn count_partially_contains(&self) -> i32 {
        let mut count: i32 = 0;

        for i in 0..self.left.len() {
            match Self::partially_contains(&self.left[i], &self.right[i]) {
                true => count += 1,
                false => (),
            }
        }

        count
    }
}
