// This chapter is dedicated to some collections: vectors, strings and hash maps

use std::collections::{HashMap, HashSet};

// VECTORS
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `second_largest(vec: &[i32]) -> Option<i32>` that returns the second largest
// element in the array. If the array has fewer than 2 elements, return `None`.

pub fn second_largest(vec: &[i32]) -> Option<i32> {
    vec.iter()
        .copied()
        .fold((None, None), |(first, second), num| match (first, second) {
            (None, _) => (Some(num), None),
            (Some(f), _) if num > f => (Some(num), Some(f)),
            (Some(f), None) if num != f => (Some(f), Some(num)),
            (Some(f), Some(s)) if num > s && num != f => (Some(f), Some(num)),
            state => state,
        })
        .1
}

// ----- 2 --------------------------------------
// Write a function `longest_increasing_subsequence(vec: &[i32]) -> Vec<i32>`` that finds the
// longest strictly increasing subsequence (not necessarily contiguous) in the array.
//
// For the simplicity, assume that there is only one longest increasing subsequence.

pub fn longest_increasing_subsequence(init_sequence: &[i32]) -> Vec<i32> {
    if init_sequence.is_empty() {
        return Vec::new();
    }

    let mut tails = Vec::new();
    let mut indices = Vec::new();
    let mut parent = vec![None; init_sequence.len()];

    for (i, &num) in init_sequence.iter().enumerate() {
        let pos = tails.binary_search(&num).unwrap_or_else(|e| e);

        if pos == tails.len() {
            tails.push(num);
            indices.push(i);
        } else {
            tails[pos] = num;
            indices[pos] = i;
        }

        parent[i] = if pos > 0 { Some(indices[pos - 1]) } else { None }
    }

    let mut result: Vec<i32> = Vec::new();
    let mut current = Some(*indices.last().unwrap());

    while let Some(idx) = current {
        result.push(init_sequence[idx]);
        current = parent[idx];
    }

    result.reverse();
    result
}

// STRINGS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `reverse_words(sentence: &str) -> String` that reverses the order of words in a
// sentence but does not reverse the characters inside each word.

pub fn reverse_words(sentence: &str) -> String {
    sentence.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

// ----- 4 --------------------------------------
// Write a function `normalize_and_capitalize(sentence: &str) -> String` that:
// - Trims extra spaces at the beginning and end.
// - Converts multiple spaces between words into a single space.
// - Makes the first letter of every word uppercase, and every other letter lowercase, for example
//   "пРеВеД МеДвЕд -> Превед Медвед"

pub fn normalize_and_capitalize(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => {
                    format!("{}{}", f.to_uppercase(), chars.as_str().to_lowercase())
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// HASH SET
// ================================================================================================

// ----- 5 --------------------------------------
// Write a function `unique_chars(s: &str) -> bool` that returns true if a string has all unique
// characters (ignoring case), and false otherwise.

pub fn unique_chars(s: &str) -> bool {
    let mut used_char = HashSet::new();

    for c in s.to_lowercase().chars() {
        if !used_char.insert(c) {
            return false;
        }
    }

    true
}

// HASH MAP
// ================================================================================================

// ----- 6 --------------------------------------
// Write a function `top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32>` that returns the `k` most
// frequent numbers in the vector. If `k` is greater than the total number of unique elements in the
// vector, return all of them.

pub fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut count_map = HashMap::new();

    nums.iter()
        .for_each(|&c| {
            let count = count_map.entry(c).or_insert(0);
            *count += 1
        });

    let mut count_vec: Vec<_> = count_map.into_iter().collect();
    count_vec.sort_by(|x, y| y.1.cmp(&x.1));

    count_vec.into_iter().take(k).map(|(x, _)| x).collect()
}
