use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use super::{Annotation, AnnotationType, Line, SyntaxHighlighter};
use crate::prelude::*;
#[derive(Default)]
pub struct RustSyntaxHighlighter {
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}
fn is_valid_number(word: &str) -> bool {
    if word.is_empty() {
        return false;
    }
    let mut chars = word.chars();
    // Check the first character
    if let Some(first_char) = chars.next() {
        if !first_char.is_ascii_digit() {
            return false; // Numbers must start with a digit
        }
    }
    let mut seen_dot = false;
    let mut seen_e = false;
    let mut prev_was_digit = true;
    // Iterate over the remaining characters
    for char in chars {
        match char {
            '0'..='9' => {
                prev_was_digit = true;
            }
            '_' => {
                if !prev_was_digit {
                    return false; // Underscores must be between digits
                }
                prev_was_digit = false;
            }
            '.' => {
                if seen_dot || seen_e || !prev_was_digit {
                    return false; // Disallow multiple dots, dots after 'e' or dots not after a digit
                }
                seen_dot = true;
                prev_was_digit = false;
            }
            'e' | 'E' => {
                if seen_e || !prev_was_digit {
                    return false; // Disallow multiple 'e's or 'e' not after a digit
                }
                seen_e = true;
                prev_was_digit = false;
            }
            _ => {
                return false; // Invalid character
            }
        }
    }
    prev_was_digit // Must end with a digit
}
impl SyntaxHighlighter for RustSyntaxHighlighter {
    fn highlight(&mut self, idx: LineIdx, line: &Line) {
        let mut result = Vec::new();
        for (start_idx, word) in line.split_word_bound_indices() {
            if is_valid_number(word) {
                result.push(Annotation {
                    annotation_type: AnnotationType::Number,
                    start: start_idx,
                    end: start_idx.saturating_add(word.len()),
                });
            }
        }
        self.highlights.insert(idx, result);
    }
    fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>> {
        self.highlights.get(&idx)
    }
}