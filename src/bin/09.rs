use std::{collections::HashSet, io};

type FileID = u16;
type Memory = Vec<Option<FileID>>;

#[derive(Debug, Clone, Copy)]
struct FreeBlocks {
    index: usize,
    size: usize,
}

pub fn main() {
    let mut mem: Memory = Vec::new();
    let mut id: FileID = 0;
    let mut is_file = true;
    let mut free_blocks: Vec<FreeBlocks> = Vec::new();
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        let s = line.trim();
        if s.is_empty() {
            continue;
        }

        for c in line.chars() {
            let n = c.to_digit(10).unwrap() as usize;
            if !is_file {
                free_blocks.push(FreeBlocks {
                    index: mem.len(),
                    size: n,
                })
            }
            for _ in 0..n {
                mem.push(if is_file { Some(id) } else { None });
            }
            if is_file {
                id += 1;
            }
            is_file = !is_file;
        }
    }

    let mut mem1 = mem.clone();
    let len = move_blocks(&mut mem1);
    move_files(&mut mem, &mut free_blocks);

    println!("Part 1: {}", compute_checksum(&mem1, len));
    println!("Part 2: {}", compute_checksum(&mem, mem.len()));
}

fn compute_checksum(mem: &Memory, used_len: usize) -> u64 {
    mem.iter()
        .enumerate()
        .take(used_len + 1)
        .map(|(pos, contents)| {
            if let Some(id) = contents {
                pos as u64 * *id as u64
            } else {
                0
            }
        })
        .sum()
}

fn move_blocks(mem: &mut Memory) -> usize {
    let mut prev_free_pos = 0;
    let mut prev_used_pos = mem.len();
    while let (Some(free_pos), Some(used_pos)) = (
        find_first_free_block(mem, prev_free_pos),
        find_last_used_block(mem, prev_used_pos),
    ) {
        if used_pos <= free_pos {
            break;
        }
        mem.swap(free_pos, used_pos);
        prev_free_pos = free_pos;
        prev_used_pos = used_pos;
    }

    prev_used_pos
}

fn find_last_used_block(mem: &Memory, end_pos: usize) -> Option<usize> {
    for i in (0..end_pos).rev() {
        if mem[i].is_some() {
            return Some(i);
        }
    }

    None
}

fn find_first_free_block(mem: &Memory, start_pos: usize) -> Option<usize> {
    for i in start_pos..mem.len() {
        if mem[i].is_none() {
            return Some(i);
        }
    }

    None
}

fn move_files(mem: &mut Memory, free_blocks: &mut Vec<FreeBlocks>) {
    let mut moved_ids: HashSet<FileID> = HashSet::new();
    let mut prev_used_pos = mem.len();
    while let Some(file_end_pos) = find_last_used_block(mem, prev_used_pos) {
        let mut file_start_pos = file_end_pos;
        while file_start_pos > 0 && mem[file_start_pos - 1] == mem[file_end_pos] {
            file_start_pos -= 1;
        }
        prev_used_pos = file_start_pos;

        // find_last_used_block should only return nonempty blocks, if that fails then panicking is good
        let file_id = mem[file_end_pos].unwrap();
        let size_needed = (file_end_pos - file_start_pos) + 1;
        if moved_ids.contains(&file_id) {
            continue;
        }

        if let Some(first_available_blocks) = free_blocks
            .iter_mut()
            .find(|f| f.size >= size_needed && f.index < file_start_pos)
        {
            moved_ids.insert(file_id);
            for i in 0..size_needed {
                mem.swap(first_available_blocks.index + i, file_start_pos + i);
            }
            first_available_blocks.size -= size_needed;
            first_available_blocks.index += size_needed;
        }
    }
}

#[allow(dead_code)]
fn dump(mem: &Memory) {
    for v in mem {
        if let Some(val) = v {
            print!("{}", val);
        } else {
            print!(".");
        }
    }
    println!();
}
