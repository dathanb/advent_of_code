/**
Generate all permutations of the input using iterative implementation of Heap's algorithm.

Translated from pseudocode at https://en.wikipedia.org/wiki/Heap%27s_algorithm
*/
pub fn permute(input: Vec<i32>) -> Vec<Vec<i32>> {
    let mut accumulator: Vec<Vec<i32>> = Vec::new();

    let mut mutable_vec = input.clone();

    // c represents the counters for each stack frame from the recursive Heap's algo
    let mut c: Vec<usize> = Vec::with_capacity(input.len());

    for _ in 0..input.len() {
        c.push(0);
    }

    accumulator.push(input.clone());

    let mut i:usize = 0;
    while i < input.len() {
        if c[i] < i {
            if i % 2 == 0 {
                let tmp = mutable_vec[0];
                mutable_vec[0] = mutable_vec[i];
                mutable_vec[i] = tmp;
            } else {
                let tmp = mutable_vec[c[i]];
                mutable_vec[c[i]] = mutable_vec[i];
                mutable_vec[i] = tmp;
            }
            accumulator.push(mutable_vec.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    accumulator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permute_one() {
        let v = vec![1];
        let permutations = permute(v);
        assert_eq!(permutations.len(), 1);
        assert_eq!(permutations[0].len(), 1);
        assert_eq!(permutations[0][0], 1);
    }

    #[test]
    fn permute_two() {
        let v = vec![1, 2];
        let permutations = permute(v);
        assert_eq!(permutations.len(), 2);
        assert_eq!(permutations[0].len(), 2);
        assert_eq!(permutations[1].len(), 2);
        assert_eq!(permutations[0][0], 1);
        assert_eq!(permutations[0][1], 2);
        assert_eq!(permutations[1][0], 2);
        assert_eq!(permutations[1][1], 1);
    }

    #[test]
    fn permute_three() {
        let v = vec![1, 2, 3];
        let permutations = permute(v);
        assert_eq!(permutations.len(), 6);
        assert_eq!(permutations[0].len(), 3);
        assert_eq!(permutations[0][0], 1);
        assert_eq!(permutations[0][1], 2);
        assert_eq!(permutations[0][2], 3);
        assert_eq!(permutations[1].len(), 3);
        assert_eq!(permutations[1][0], 2);
        assert_eq!(permutations[1][1], 1);
        assert_eq!(permutations[1][2], 3);
        assert_eq!(permutations[2].len(), 3);
        assert_eq!(permutations[2][0], 3);
        assert_eq!(permutations[2][1], 1);
        assert_eq!(permutations[2][2], 2);
        assert_eq!(permutations[3].len(), 3);
        assert_eq!(permutations[3][0], 1);
        assert_eq!(permutations[3][1], 3);
        assert_eq!(permutations[3][2], 2);
        assert_eq!(permutations[4].len(), 3);
        assert_eq!(permutations[4][0], 2);
        assert_eq!(permutations[4][1], 3);
        assert_eq!(permutations[4][2], 1);
        assert_eq!(permutations[5].len(), 3);
        assert_eq!(permutations[5][0], 3);
        assert_eq!(permutations[5][1], 2);
        assert_eq!(permutations[5][2], 1);
    }
}
