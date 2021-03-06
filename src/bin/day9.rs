fn main() {
    let input: &str = include_str!("../inputs/day9");
    println!("part 1 {:?}", part1(input, 25));
    println!("part 2 {:?}", part2(input, part1(input, 25) as isize));
}

fn part1(input_str: &str, window_size: usize) -> usize {
    let input: Vec<usize> = input_str
        .lines()
        .map(|p| p.parse::<usize>().expect(&format!("Parse error {:?}", p)))
        .collect();

    for i in window_size..input.len() {
        if !pair_sum(&input[i - window_size..i], input[i]) {
            return input[i];
        }
    }
    panic!("No errors!");
}

fn pair_sum(v: &[usize], target: usize) -> bool {
    for i in 0..v.len() {
        let iv = v[i];
        for j in i..v.len() {
            if iv + v[j] == target {
                return true;
            }
        }
    }
    false
}

fn part2(input_str: &str, target: isize) -> isize {
    let input: Vec<isize> = input_str
        .lines()
        .map(|p| p.parse::<isize>().expect(&format!("Parse error {:?}", p)))
        .collect();
    for i in 0..input.len() {
        let mut sum: isize = 0;
        let v: Vec<&isize> = input[i..input.len()]
            .iter()
            .take_while(|n| {
                sum += **n;
                sum <= target
            })
            .collect();

        // Sum it up and check if exactly target,
        if v.iter().fold(0, |x, y| x + *y) == target {
            return **v.iter().min().unwrap() + **v.iter().max().unwrap();
        }
    }
    panic!("No seq found");
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTCASE: &str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_day9() {
        assert_eq!(part1(TESTCASE, 5), 127);
        assert_eq!(part2(TESTCASE, 127), 62);
    }
}
