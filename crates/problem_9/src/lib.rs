pub fn part_1(input: &str) -> Option<u64> {
    let nums = parse_digits(input);
    let mut blocks = construct_blocks(&nums);
    defrag_string(&mut blocks);
    let result = checksum(&blocks);
    Some(result as u64)
}

fn parse_digits(input: &str) -> Vec<u32> {
    input
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect()
}

fn defrag_string(blocks: &mut [Option<u32>]) {
    let mut left = 0;
    let mut right = blocks.len() - 1;
    while left < right {
        if blocks[left].is_none() && blocks[right].is_some() {
            blocks.swap(left, right);
        } else if blocks[left].is_none() {
            right -= 1;
        } else {
            left += 1;
        }
    }
}

fn defrag_files(blocks: &mut [Option<u32>]) {
    let mut right = blocks.len() - 1;
    while right > 0 {
        if let Some(file_id) = blocks[right] {
            let mut left = right;
            while left > 0 && blocks[left] == blocks[right] {
                left -= 1;
            }
            left += 1;
            let file_size = right - left + 1;
            println!(
                "file_id: {}, left: {}, right: {}, size: {}",
                file_id, left, right, file_size
            );
            // find first empty block of the same size
            for i in 0..left {
                if blocks[i..i + file_size].iter().all(|&b| b.is_none()) {
                    blocks[i..i + file_size]
                        .iter_mut()
                        .for_each(|b| *b = Some(file_id));
                    blocks[left..=right].iter_mut().for_each(|b| *b = None);
                    break;
                }
            }
            right = left - 1;
        } else {
            right -= 1;
        }
    }
}

fn construct_blocks(map: &[u32]) -> Vec<Option<u32>> {
    let mut blocks = Vec::new();
    let mut file_id = 0;
    let mut is_space = false;
    for &num in map {
        for _ in 0..num {
            if is_space {
                blocks.push(None);
            } else {
                blocks.push(Some(file_id));
            }
        }
        if !is_space {
            file_id += 1;
        }
        is_space = !is_space;
    }
    blocks
}

fn checksum(blocks: &[Option<u32>]) -> u64 {
    let mut sum = 0;
    for (i, &block) in blocks.iter().enumerate() {
        if let Some(block) = block {
            sum += i as u64 * block as u64;
        }
    }
    sum
}

pub fn part_2(input: &str) -> Option<u64> {
    let nums = parse_digits(input);
    let mut blocks = construct_blocks(&nums);
    defrag_files(&mut blocks);
    let result = checksum(&blocks);
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block_as_string(block: Option<u32>) -> String {
        match block {
            Some(id) => id.to_string(),
            None => ".".to_string(),
        }
    }

    fn blocks_as_string(blocks: &[Option<u32>]) -> String {
        blocks.iter().map(|b| block_as_string(*b)).collect()
    }

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_construct() {
        let nums = parse_digits(INPUT);
        let result = construct_blocks(&nums);
        let expected = "00...111...2...333.44.5555.6666.777.888899";
        assert_eq!(blocks_as_string(&result), expected.to_string());
    }

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_2() {
        // let result = part_2(INPUT);
        let nums = parse_digits(INPUT);
        let mut blocks = construct_blocks(&nums);
        println!("{:?}", blocks_as_string(&blocks));
        defrag_files(&mut blocks);
        println!("{:?}", blocks_as_string(&blocks));
        let result = checksum(&blocks);
        assert_eq!(Some(result), Some(2858));
    }
}
