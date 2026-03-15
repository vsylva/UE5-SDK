use std::{collections::HashMap, env, fs, io, path::Path};

#[derive(Debug, Clone,)]
struct MemberInfo {
    offset: u32,
}

type SdkIndex = HashMap<String, HashMap<String, MemberInfo,>,>;

fn parse_first_hex(s: &str,) -> Option<u32,> {
    let b = s.as_bytes();
    let len = b.len();
    let mut i = 0;
    while i + 1 < len
    {
        if b[i] == b'0' && (b[i + 1] == b'x' || b[i + 1] == b'X')
        {
            let start = i + 2;
            let mut end = start;
            while end < len && b[end].is_ascii_hexdigit()
            {
                end += 1;
            }
            if end > start
            {
                if let Ok(v,) = u32::from_str_radix(&s[start..end], 16,)
                {
                    return Some(v,);
                }
            }
            i = end;
        }
        else
        {
            i += 1;
        }
    }
    None
}

fn extract_member_cpp(line: &str,) -> Option<(String, MemberInfo,),> {
    let comment_pos = line.find("//",)?;
    let before = &line[..comment_pos];
    let offset = parse_first_hex(&line[comment_pos..],)?;

    let mut s = before.trim_end_matches(|c: char| c == ';' || c == ' ' || c == '\t',);

    if let Some(pos,) = s.rfind(':',)
    {
        if s[pos + 1..].trim().chars().all(|c| c.is_ascii_digit(),)
        {
            s = s[..pos].trim_end();
        }
    }

    if s.ends_with(']',)
    {
        if let Some(pos,) = s.rfind('[',)
        {
            s = s[..pos].trim_end();
        }
    }

    let name: String =
        s.chars().rev().take_while(|c| c.is_alphanumeric() || *c == '_',).collect::<String>().chars().rev().collect();

    if name.is_empty() || name.chars().next()?.is_ascii_digit()
    {
        return None;
    }
    Some((name, MemberInfo { offset, },),)
}

fn is_class_header_cpp(line: &str,) -> Option<String,> {
    if line.starts_with(' ',) || line.starts_with('\t',)
    {
        return None;
    }
    let t = line.trim();
    if t.contains("//",)
    {
        return None;
    }

    let rest = if t.starts_with("class ",)
    {
        &t[6..]
    }
    else if t.starts_with("struct ",)
    {
        &t[7..]
    }
    else
    {
        return None;
    };

    let rest = skip_modifiers(rest.trim_start(),);

    let end = rest.find(|c: char| c == ' ' || c == ':' || c == '{' || c == ';',).unwrap_or(rest.len(),);
    let name = rest[..end].trim();

    if name.is_empty() || name.starts_with('(',) || !name.starts_with(|c: char| c.is_alphabetic() || c == '_',)
    {
        return None;
    }
    Some(name.to_string(),)
}

fn skip_modifiers(s: &str,) -> &str {
    let mut cur = s;
    loop
    {
        let ident_end = cur.find(|c: char| !c.is_alphanumeric() && c != '_',).unwrap_or(cur.len(),);
        if ident_end == 0
        {
            break;
        }
        let after_ident = &cur[ident_end..];
        let after_ident_trimmed = after_ident.trim_start();
        if after_ident_trimmed.starts_with('(',)
        {
            let mut depth = 0i32;
            let mut end = 0usize;
            let bytes = after_ident_trimmed.as_bytes();
            for (i, &b,) in bytes.iter().enumerate()
            {
                match b
                {
                    b'(' => depth += 1,
                    b')' =>
                    {
                        depth -= 1;
                        if depth == 0
                        {
                            end = i + 1;
                            break;
                        }
                    }
                    _ =>
                    {}
                }
            }
            if end > 0
            {
                cur = after_ident_trimmed[end..].trim_start();
                continue;
            }
        }
        break;
    }
    cur
}

fn build_dumper7_index(sdk_dir: &Path,) -> io::Result<SdkIndex,> {
    let mut sdk: SdkIndex = HashMap::new();
    let mut file_count = 0usize;
    scan_cpp_dir(sdk_dir, &mut sdk, &mut file_count,)?;
    eprintln!("[info] Dumper-7: 扫描了 {} 个文件，共找到 {} 个类/结构体", file_count, sdk.len());
    Ok(sdk,)
}

fn scan_cpp_dir(dir: &Path, sdk: &mut SdkIndex, file_count: &mut usize,) -> io::Result<(),> {
    for entry in fs::read_dir(dir,)?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir()
        {
            scan_cpp_dir(&path, sdk, file_count,)?;
        }
        else if let Some(ext,) = path.extension()
        {
            let ext = ext.to_string_lossy().to_lowercase();
            if ext == "hpp" || ext == "h" || ext == "cpp"
            {
                parse_cpp_file(&path, sdk,);
                *file_count += 1;
            }
        }
    }
    Ok((),)
}

fn parse_cpp_file(path: &Path, sdk: &mut SdkIndex,) {
    let content = match fs::read_to_string(path,)
    {
        Ok(c,) => c,
        Err(e,) =>
        {
            eprintln!("[warn] 无法读取 {}: {}", path.display(), e);
            return;
        }
    };

    let mut current_class: Option<String,> = None;
    let mut brace_depth: i32 = 0;

    for line in content.lines()
    {
        let open = line.chars().filter(|&c| c == '{',).count() as i32;
        let close = line.chars().filter(|&c| c == '}',).count() as i32;

        if current_class.is_none()
        {
            if let Some(name,) = is_class_header_cpp(line,)
            {
                current_class = Some(name.clone(),);
                sdk.entry(name,).or_default();
                brace_depth = 0;
            }
        }

        brace_depth += open - close;

        if current_class.is_some() && brace_depth <= 0 && (open > 0 || close > 0)
        {
            current_class = None;
            brace_depth = 0;
            continue;
        }

        if let Some(ref class_name,) = current_class
        {
            if brace_depth >= 1 && line.contains("//",)
            {
                if let Some((member_name, info,),) = extract_member_cpp(line,)
                {
                    sdk.entry(class_name.clone(),).or_default().entry(member_name,).or_insert(info,);
                }
            }
        }
    }
}

#[derive(Debug,)]
struct RustOffsetEntry {
    file: std::path::PathBuf,

    offset_line_idx: usize,

    class_name: String,

    member_name: String,

    current_offset: u32,
}

fn parse_rust_offset_attr(line: &str,) -> Option<u32,> {
    let trimmed = line.trim();
    if !trimmed.starts_with("#[offset(",)
    {
        return None;
    }
    let inner_start = "#[offset(".len();
    let inner = &trimmed[inner_start..];
    let end = inner.find(')',)?;
    let hex_str = inner[..end].trim();
    let hex = if hex_str.starts_with("0x",) || hex_str.starts_with("0X",) { &hex_str[2..] } else { hex_str };
    u32::from_str_radix(hex, 16,).ok()
}

fn parse_bits_attr_name(line: &str,) -> Option<String,> {
    let trimmed = line.trim();
    if !trimmed.starts_with("#[bits(",)
    {
        return None;
    }
    let inner_start = "#[bits(".len();
    let inner = &trimmed[inner_start..];
    let end = inner.find(')',)?;
    let args = &inner[..end];

    let comma = args.find(',',)?;
    let name = args[comma + 1..].trim();
    if name.is_empty()
    {
        return None;
    }
    Some(name.to_string(),)
}

fn parse_field_name(line: &str,) -> Option<String,> {
    let trimmed = line.trim();

    if trimmed.starts_with('#',) || trimmed.starts_with("//",) || trimmed.is_empty()
    {
        return None;
    }

    let s = trimmed
        .trim_start_matches("pub(crate)",)
        .trim_start_matches("pub(super)",)
        .trim_start_matches("pub",)
        .trim_start();

    let colon = s.find(':',)?;
    let name = s[..colon].trim();
    if name.is_empty() || !name.starts_with(|c: char| c.is_alphabetic() || c == '_',)
    {
        return None;
    }
    Some(name.to_string(),)
}

fn parse_struct_name(line: &str,) -> Option<String,> {
    let trimmed = line.trim();

    let s = trimmed
        .trim_start_matches("pub(crate)",)
        .trim_start_matches("pub(super)",)
        .trim_start_matches("pub",)
        .trim_start();
    if !s.starts_with("struct ",)
    {
        return None;
    }
    let rest = s["struct ".len()..].trim_start();
    let end = rest
        .find(|c: char| c == '<' || c == '{' || c == ';' || c == '(' || c == ' ' || c == '\t',)
        .unwrap_or(rest.len(),);
    let name = rest[..end].trim();
    if name.is_empty() || !name.starts_with(|c: char| c.is_alphabetic() || c == '_',)
    {
        return None;
    }
    Some(name.to_string(),)
}

fn scan_rust_dir(dir: &Path, entries: &mut Vec<RustOffsetEntry,>,) -> io::Result<(),> {
    for entry in fs::read_dir(dir,)?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir()
        {
            scan_rust_dir(&path, entries,)?;
        }
        else if let Some(ext,) = path.extension()
        {
            if ext == "rs"
            {
                parse_rust_file(&path, entries,);
            }
        }
    }
    Ok((),)
}

fn parse_rust_file(path: &Path, entries: &mut Vec<RustOffsetEntry,>,) {
    let content = match fs::read_to_string(path,)
    {
        Ok(c,) => c,
        Err(e,) =>
        {
            eprintln!("[warn] 无法读取 {}: {}", path.display(), e);
            return;
        }
    };

    let lines: Vec<&str,> = content.lines().collect();
    let mut current_struct: Option<String,> = None;
    let mut brace_depth: i32 = 0;

    let mut i = 0;
    while i < lines.len()
    {
        let line = lines[i];

        if brace_depth == 0
        {
            if let Some(name,) = parse_struct_name(line,)
            {
                current_struct = Some(name,);
            }
        }

        let open = line.chars().filter(|&c| c == '{',).count() as i32;
        let close = line.chars().filter(|&c| c == '}',).count() as i32;
        brace_depth += open - close;
        if brace_depth < 0
        {
            brace_depth = 0;
        }

        if brace_depth >= 1
        {
            if let Some(offset_val,) = parse_rust_offset_attr(line,)
            {
                let offset_line_idx = i;

                let mut member_name: Option<String,> = None;
                let mut bits_name: Option<String,> = None;

                let mut j = i + 1;
                while j < lines.len()
                {
                    let next = lines[j].trim();
                    if next.is_empty()
                    {
                        j += 1;
                        continue;
                    }
                    if next.starts_with('#',)
                    {
                        if let Some(bname,) = parse_bits_attr_name(lines[j],)
                        {
                            bits_name = Some(bname,);
                        }
                        j += 1;
                        continue;
                    }

                    member_name = parse_field_name(lines[j],);
                    break;
                }

                let final_name = bits_name.or(member_name,);

                if let (Some(class_name,), Some(member_name,),) = (current_struct.clone(), final_name,)
                {
                    entries.push(RustOffsetEntry {
                        file: path.to_path_buf(),
                        offset_line_idx,
                        class_name,
                        member_name,
                        current_offset: offset_val,
                    },);
                }
            }
        }

        i += 1;
    }
}

fn fix_rust_file(path: &Path, fixes: &[(usize, u32, u32,)],) -> io::Result<usize,> {
    if fixes.is_empty()
    {
        return Ok(0,);
    }

    let content = fs::read_to_string(path,)?;
    let mut lines: Vec<String,> = content.lines().map(|l| l.to_string(),).collect();

    let trailing_newline = content.ends_with('\n',);

    let mut applied = 0;
    for &(line_idx, old_off, new_off,) in fixes
    {
        let line = &lines[line_idx];
        let old_str = format!("0x{:04X}", old_off);
        let new_str = format!("0x{:04X}", new_off);

        let old_str_lower = format!("0x{:04x}", old_off);

        if line.contains(&old_str,)
        {
            lines[line_idx] = line.replacen(&old_str, &new_str, 1,);
            applied += 1;
        }
        else if line.contains(&old_str_lower,)
        {
            lines[line_idx] = line.replacen(&old_str_lower, &new_str, 1,);
            applied += 1;
        }
        else
        {
            if let Some(replaced,) = replace_offset_in_attr_line(line, new_off,)
            {
                lines[line_idx] = replaced;
                applied += 1;
            }
            else
            {
                eprintln!("[warn] 行 {} 替换失败，原文: {}", line_idx + 1, line.trim());
            }
        }
    }

    let mut new_content = lines.join("\n",);
    if trailing_newline
    {
        new_content.push('\n',);
    }
    fs::write(path, new_content,)?;
    Ok(applied,)
}

fn replace_offset_in_attr_line(line: &str, new_offset: u32,) -> Option<String,> {
    let prefix = "#[offset(";
    let start = line.find(prefix,)?;
    let after = &line[start + prefix.len()..];
    let end = after.find(')',)?;
    let hex_part = after[..end].trim();
    let hex_start_in_line = start + prefix.len() + after.find(hex_part,)?;
    let hex_end_in_line = hex_start_in_line + hex_part.len();

    let lower = hex_part.to_lowercase();
    if !lower.starts_with("0x",)
    {
        return None;
    }
    u32::from_str_radix(&lower[2..], 16,).ok()?;

    let new_hex = if hex_part[2..].chars().any(|c| c.is_uppercase(),)
    {
        format!("0x{:04X}", new_offset)
    }
    else
    {
        format!("0x{:04x}", new_offset)
    };

    let mut result = line.to_string();
    result.replace_range(hex_start_in_line..hex_end_in_line, &new_hex,);
    Some(result,)
}

fn main() -> io::Result<(),> {
    let args: Vec<String,> = env::args().collect();

    if args.len() < 3
    {
        eprintln!("用法: {} <dumper7_sdk_dir> <rust_sdk_dir> [--dry-run]", args[0]);
        eprintln!("  --dry-run  只报告差异，不修改文件");
        std::process::exit(1,);
    }

    let dumper7_dir = Path::new(&args[1],);
    let rust_dir = Path::new(&args[2],);
    let dry_run = args.iter().any(|a| a == "--dry-run",);

    if !dumper7_dir.is_dir()
    {
        eprintln!("[error] '{}' 不是目录", dumper7_dir.display());
        std::process::exit(1,);
    }
    if !rust_dir.is_dir()
    {
        eprintln!("[error] '{}' 不是目录", rust_dir.display());
        std::process::exit(1,);
    }

    if dry_run
    {
        eprintln!("[info] 模式: dry-run（仅报告，不修改）");
    }
    else
    {
        eprintln!("[info] 模式: 自动修正（将直接修改 Rust 源文件）");
    }

    eprintln!("[info] 正在扫描 Dumper-7 SDK: {}", dumper7_dir.display());
    let dumper7 = build_dumper7_index(dumper7_dir,)?;

    eprintln!("[info] 正在扫描 Rust SDK: {}", rust_dir.display());
    let mut rust_entries: Vec<RustOffsetEntry,> = Vec::new();
    scan_rust_dir(rust_dir, &mut rust_entries,)?;
    eprintln!("[info] Rust SDK: 找到 {} 条偏移记录", rust_entries.len());

    let mut file_fixes: HashMap<std::path::PathBuf, Vec<(usize, u32, u32,),>,> = HashMap::new();

    println!("\n========== 偏移对比结果 ==========\n");
    println!("  {:<4} {:<70} {}", "状态", "类::成员", "说明");
    println!("  {}", "-".repeat(100));

    let (mut ok, mut fixed, mut not_found, mut class_not_found,) = (0usize, 0usize, 0usize, 0usize,);

    for entry in &rust_entries
    {
        let key = format!("{}::{}", entry.class_name, entry.member_name);
        let rust_hex = format!("0x{:04X}", entry.current_offset);

        match dumper7.get(&entry.class_name,)
        {
            None =>
            {
                println!("  🔍  {:<70} Rust={} 类在 Dumper-7 中不存在", key, rust_hex);
                class_not_found += 1;
            }
            Some(members,) => match members.get(&entry.member_name,)
            {
                None =>
                {
                    println!("  ⚠️   {:<70} Rust={} 成员在 Dumper-7 中不存在", key, rust_hex);
                    not_found += 1;
                }
                Some(info,) =>
                {
                    if info.offset == entry.current_offset
                    {
                        println!("  ✅  {:<70} = {}", key, rust_hex);
                        ok += 1;
                    }
                    else
                    {
                        let d7_hex = format!("0x{:04X}", info.offset);
                        let action = if dry_run { "（dry-run）" } else { "→ 已修正" };
                        println!("  ❌  {:<70} Rust={} Dumper7={} {}", key, rust_hex, d7_hex, action);
                        if !dry_run
                        {
                            file_fixes.entry(entry.file.clone(),).or_default().push((
                                entry.offset_line_idx,
                                entry.current_offset,
                                info.offset,
                            ),);
                        }
                        fixed += 1;
                    }
                }
            },
        }
    }

    if !dry_run && !file_fixes.is_empty()
    {
        println!("\n========== 修改文件 ==========\n");
        let mut total_fixed_lines = 0usize;
        for (file, fixes,) in &file_fixes
        {
            match fix_rust_file(file, fixes,)
            {
                Ok(n,) =>
                {
                    println!("  📝  {} → 修正 {} 处", file.display(), n);
                    total_fixed_lines += n;
                }
                Err(e,) =>
                {
                    eprintln!("[error] 修改 {} 失败: {}", file.display(), e);
                }
            }
        }
        println!("\n  共修改 {} 个文件，合计修正 {} 处偏移", file_fixes.len(), total_fixed_lines);
    }

    println!("\n========== 汇总 ==========");
    println!(
        "  总计 {} | ✅ {} | ❌ {}（{}） | ⚠️  {} | 🔍 {}",
        rust_entries.len(),
        ok,
        fixed,
        if dry_run { "dry-run" } else { "已修正" },
        not_found,
        class_not_found
    );

    Ok((),)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rust_offset_attr() {
        assert_eq!(parse_rust_offset_attr("    #[offset(0x21C8)]"), Some(0x21C8));
        assert_eq!(parse_rust_offset_attr("    #[offset(0x28E3)]"), Some(0x28E3));
        assert_eq!(parse_rust_offset_attr("    #[offset(0xabcd)]"), Some(0xabcd));
        assert_eq!(parse_rust_offset_attr("    pub field: Type,"), None);
        assert_eq!(parse_rust_offset_attr("    #[bits(3, name)]"), None);
    }

    #[test]
    fn test_parse_bits_attr_name() {
        assert_eq!(parse_bits_attr_name("    #[bits(3, bIsFemale)]"), Some("bIsFemale".to_string()));
        assert_eq!(
            parse_bits_attr_name("    #[bits(2, bFlyerDinoAllowBackwardsFlight)]"),
            Some("bFlyerDinoAllowBackwardsFlight".to_string())
        );
        assert_eq!(parse_bits_attr_name("    #[offset(0x1234)]"), None);
    }

    #[test]
    fn test_parse_field_name() {
        assert_eq!(parse_field_name("    pub Rider: TWeakObjectPtr<AShooterCharacter>,"), Some("Rider".to_string()));
        assert_eq!(parse_field_name("    _0: bool,"), Some("_0".to_string()));
        assert_eq!(parse_field_name("    pub bIsRiding: bool,"), Some("bIsRiding".to_string()));

        assert_eq!(parse_field_name("    #[offset(0x1234)]"), None);
    }

    #[test]
    fn test_parse_struct_name() {
        assert_eq!(parse_struct_name("pub struct APrimalDinoCharacter {"), Some("APrimalDinoCharacter".to_string()));
        assert_eq!(parse_struct_name("pub struct FHitResult<T> {"), Some("FHitResult".to_string()));
        assert_eq!(parse_struct_name("    pub field: Type,"), None);
    }

    #[test]
    fn test_replace_offset_in_attr_line() {
        let line = "    #[offset(0x21C8)]";
        let result = replace_offset_in_attr_line(line, 0x21D0,).unwrap();
        assert_eq!(result, "    #[offset(0x21D0)]");

        let line2 = "    #[offset(0x21c8)]";
        let result2 = replace_offset_in_attr_line(line2, 0x21D0,).unwrap();
        assert_eq!(result2, "    #[offset(0x21d0)]");
    }

    #[test]
    fn test_full_parse_rust_snippet() {
        let snippet = r#"
pub struct APrimalDinoCharacter {
    #[offset(0x28E3)]
    #[bits(3, bIsFemale)]
    _0: bool,

    #[offset(0x21C8)]
    pub Rider: TWeakObjectPtr<AShooterCharacter>,

    #[offset(0x28F0)]
    #[bits(2, bFlyerDinoAllowBackwardsFlight)]
    pub bFlyerDinoAllowBackwardsFlight: bool,
}
"#;

        let tmp = std::env::temp_dir().join("test_rust_snippet.rs",);
        fs::write(&tmp, snippet,).unwrap();
        let mut entries = Vec::new();
        parse_rust_file(&tmp, &mut entries,);
        fs::remove_file(&tmp,).unwrap();

        assert_eq!(entries.len(), 3);

        assert_eq!(entries[0].member_name, "bIsFemale");
        assert_eq!(entries[0].current_offset, 0x28E3);

        assert_eq!(entries[1].member_name, "Rider");
        assert_eq!(entries[1].current_offset, 0x21C8);

        assert_eq!(entries[2].member_name, "bFlyerDinoAllowBackwardsFlight");
        assert_eq!(entries[2].current_offset, 0x28F0);
    }
}
