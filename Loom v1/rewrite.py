import sys

def rewrite_prompt_block():
    with open('src-tauri/src/terminal_engine/filters/rules/prompt_block.rs', 'r', encoding='utf-8') as f:
        content = f.read()

    # Replacements
    content = content.replace(
        'pub(crate) fn extract_recent_bullet_block(\\n  lines: &[String],\\n  input_lines: Option<&[String]>,\\n  max_bullets: usize,\\n) -> PromptBlockResult {',
        'pub(crate) fn extract_recent_bullet_block(\\n  lines: &[String],\\n  input_lines: Option<&[String]>,\\n  max_bullets: usize,\\n  prompt_marker: &str,\\n  bullet_marker: &str,\\n) -> PromptBlockResult {'
    )
    content = content.replace(
        'extract_bullet_block(lines, input_lines, max_bullets)',
        'extract_bullet_block(lines, input_lines, max_bullets, prompt_marker, bullet_marker)'
    )

    content = content.replace(
        'pub(crate) fn extract_last_bullet_block(\\n  lines: &[String],\\n  input_lines: Option<&[String]>,\\n) -> PromptBlockResult {',
        'pub(crate) fn extract_last_bullet_block(\\n  lines: &[String],\\n  input_lines: Option<&[String]>,\\n  prompt_marker: &str,\\n  bullet_marker: &str,\\n) -> PromptBlockResult {'
    )
    content = content.replace(
        'extract_bullet_block(lines, input_lines, 1)',
        'extract_bullet_block(lines, input_lines, 1, prompt_marker, bullet_marker)'
    )

    content = content.replace(
        'fn find_prompt_matches_any(lines: &[String]) -> Vec<PromptMatch> {',
        'fn find_prompt_matches_any(lines: &[String], prompt_marker: &str) -> Vec<PromptMatch> {'
    )
    content = content.replace(
        'is_non_empty_prompt(line)',
        'is_non_empty_prompt(line, prompt_marker)'
    )

    content = content.replace(
        'fn find_prompt_block_by_input(\\n  lines: &[String],\\n  input_lines: &[String],\\n) -> Option<(PromptMatch, PromptMatch)> {',
        'fn find_prompt_block_by_input(\\n  lines: &[String],\\n  input_lines: &[String],\\n  prompt_marker: &str,\\n  bullet_marker: &str,\\n) -> Option<(PromptMatch, PromptMatch)> {'
    )
    content = content.replace(
        'find_prompt_matches_any(lines)',
        'find_prompt_matches_any(lines, prompt_marker)'
    )
    content = content.replace(
        'is_bullet_line(line)',
        'is_bullet_line(line, bullet_marker)'
    )
    content = content.replace(
        'matches_input_segment(segment, input_lines)',
        'matches_input_segment(segment, input_lines, prompt_marker)'
    )

    content = content.replace(
        'fn match_input_from_prompt(\\n  lines: &[String],\\n  start: usize,\\n  input_lines: &[String],\\n) -> Option<usize> {',
        'fn match_input_from_prompt(\\n  lines: &[String],\\n  start: usize,\\n  input_lines: &[String],\\n  prompt_marker: &str,\\n  bullet_marker: &str,\\n) -> Option<usize> {'
    )

    content = content.replace(
        'fn find_prompt_match_by_input_tail(\\n  lines: &[String],\\n  input_lines: &[String],\\n) -> Option<usize> {',
        'fn find_prompt_match_by_input_tail(\\n  lines: &[String],\\n  input_lines: &[String],\\n  prompt_marker: &str,\\n  bullet_marker: &str,\\n) -> Option<usize> {'
    )
    content = content.replace(
        'is_non_empty_prompt(lines[idx].as_str())',
        'is_non_empty_prompt(lines[idx].as_str(), prompt_marker)'
    )
    content = content.replace(
        'match_input_from_prompt(lines, idx, input_lines)',
        'match_input_from_prompt(lines, idx, input_lines, prompt_marker, bullet_marker)'
    )

    content = content.replace(
        'fn extract_bullet_block_before_prompt(\\n  lines: &[String],\\n  prompt_index: usize,\\n) -> Option<Vec<String>> {',
        'fn extract_bullet_block_before_prompt(\\n  lines: &[String],\\n  prompt_index: usize,\\n  bullet_marker: &str,\\n) -> Option<Vec<String>> {'
    )
    content = content.replace(
        'strip_bullet_prefix(line)',
        'strip_bullet_prefix(line, bullet_marker)'
    )
    
    content = content.replace(
        'fn matches_input_segment(segment: &[String], input_lines: &[String]) -> bool {',
        'fn matches_input_segment(segment: &[String], input_lines: &[String], prompt_marker: &str) -> bool {'
    )
    content = content.replace(
        \\"starts_with('›')\\",
        \\"starts_with(prompt_marker)\\"
    )
    content = content.replace(
        \\"trim_start_matches('›')\\",
        \\"trim_start_matches(prompt_marker)\\"
    )
    
    content = content.replace(
        'fn is_non_empty_prompt(line: &str) -> bool {',
        'fn is_non_empty_prompt(line: &str, prompt_marker: &str) -> bool {'
    )

    content = content.replace(
        'fn is_bullet_line(line: &str) -> bool {\\n  let trimmed = line.trim_start();\\n  trimmed.starts_with(\\'•\\') || trimmed.starts_with(\\'?\\')\\n}',
        'fn is_bullet_line(line: &str, bullet_marker: &str) -> bool {\\n  let trimmed = line.trim_start();\\n  trimmed.starts_with(bullet_marker)\\n}'
    )
    
    content = content.replace(
        'fn strip_bullet_prefix(line: &str) -> String {',
        'fn strip_bullet_prefix(line: &str, bullet_marker: &str) -> String {'
    )
    content = content.replace(
        \\"if ch != '•' && ch != '?' {\\",
        \\"if !line[index..].starts_with(bullet_marker) {\\"
    )
    content = content.replace(
        \\"let after_bullet = &line[index + ch.len_utf8()..];\\",
        \\"let after_bullet = &line[index + bullet_marker.len()..];\\"
    )
    content = content.replace(
        \\"line.len().saturating_sub(ch.len_utf8())\\",
        \\"line.len().saturating_sub(bullet_marker.len())\\"
    )

    content = content.replace(
        'fn extract_bullet_block(\\n  lines: &[String],\\n  input_lines: Option<&[String]>,\\n  max_bullets: usize,\\n) -> PromptBlockResult {',
        'fn extract_bullet_block(\\n  lines: &[String],\\n  input_lines: Option<&[String]>,\\n  max_bullets: usize,\\n  prompt_marker: &str,\\n  bullet_marker: &str,\\n) -> PromptBlockResult {'
    )
    content = content.replace(
        'find_prompt_block_by_input(lines, input_lines)',
        'find_prompt_block_by_input(lines, input_lines, prompt_marker, bullet_marker)'
    )
    content = content.replace(
        'find_prompt_match_by_input_tail(lines, input_lines)',
        'find_prompt_match_by_input_tail(lines, input_lines, prompt_marker, bullet_marker)'
    )
    content = content.replace(
        'extract_bullet_block_before_prompt(lines, prompt_index)',
        'extract_bullet_block_before_prompt(lines, prompt_index, bullet_marker)'
    )

    with open('src-tauri/src/terminal_engine/filters/rules/prompt_block.rs', 'w', encoding='utf-8') as f:
        f.write(content)

rewrite_prompt_block()
