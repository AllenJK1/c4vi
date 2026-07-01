use std::io::{self, BufRead, Read, Write};
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::OnceLock;

static VFS: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn get_vfs() -> &'static Mutex<HashMap<String, String>> {
    VFS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn read_message() -> io::Result<Option<String>> {
    let mut stdin = io::stdin().lock();
    let mut content_length = 0;
    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = stdin.read_line(&mut line)?;
        if bytes_read == 0 {
            return Ok(None);
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        if trimmed.to_lowercase().starts_with("content-length:") {
            if let Some(val) = trimmed.split(':').nth(1) {
                if let Ok(len) = val.trim().parse::<usize>() {
                    content_length = len;
                }
            }
        }
    }
    if content_length == 0 {
        return Ok(None);
    }
    let mut buf = vec![0; content_length];
    stdin.read_exact(&mut buf)?;
    let body = String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(Some(body))
}

fn send_response(id: Option<serde_json::Value>, result: serde_json::Value) {
    let mut resp = serde_json::Map::new();
    resp.insert("jsonrpc".to_string(), serde_json::Value::String("2.0".to_string()));
    if let Some(id_val) = id {
        resp.insert("id".to_string(), id_val);
    }
    resp.insert("result".to_string(), result);
    let payload = serde_json::to_string(&resp).unwrap();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = write!(stdout, "Content-Length: {}\r\n\r\n{}", payload.len(), payload);
    let _ = stdout.flush();
}

fn send_notification(method: &str, params: serde_json::Value) {
    let mut resp = serde_json::Map::new();
    resp.insert("jsonrpc".to_string(), serde_json::Value::String("2.0".to_string()));
    resp.insert("method".to_string(), serde_json::Value::String(method.to_string()));
    resp.insert("params".to_string(), params);
    let payload = serde_json::to_string(&resp).unwrap();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = write!(stdout, "Content-Length: {}\r\n\r\n{}", payload.len(), payload);
    let _ = stdout.flush();
}

fn get_imported_symbols(current_file_uri: &str, mod_path: &str) -> Vec<String> {
    let mut symbols = Vec::new();
    let current_path = if current_file_uri.starts_with("file://") {
        &current_file_uri[7..]
    } else {
        current_file_uri
    };
    if let Some(current_dir) = std::path::Path::new(current_path).parent() {
        let parts: Vec<&str> = mod_path.split("::").collect();
        let mut rel_path_1 = current_dir.to_path_buf();
        for p in &parts {
            rel_path_1.push(p);
        }
        rel_path_1.set_extension("c4");
        
        let mut rel_path_2 = current_dir.to_path_buf();
        if let Some(last) = parts.last() {
            rel_path_2.push(last);
        }
        rel_path_2.set_extension("c4");
        
        let target_path = if rel_path_1.is_file() {
            Some(rel_path_1)
        } else if rel_path_2.is_file() {
            Some(rel_path_2)
        } else {
            None
        };
        
        if let Some(path) = target_path {
            if let Ok(content) = std::fs::read_to_string(path) {
                let re_fn = regex::Regex::new(r"\bfn\s+([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
                let re_type = regex::Regex::new(r"\b(?:struct|enum|sum|union|type)\s+([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
                for cap in re_fn.captures_iter(&content) {
                    symbols.push(cap[1].to_string());
                }
                for cap in re_type.captures_iter(&content) {
                    symbols.push(cap[1].to_string());
                }
            }
        }
    }
    symbols
}

fn get_current_func(text: &str, cursor_line: usize) -> Option<String> {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut in_func = false;
    let mut func_name = String::new();
    let mut block_depth = 0;
    
    let re_fn = regex::Regex::new(r"^\s*(?:pub\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();
    
    for (idx, line) in lines.iter().enumerate() {
        let clean_line = if let Some(pos) = line.find("//") {
            &line[..pos]
        } else {
            line
        };
        let clean_line_stripped = clean_line.trim();
        
        if !in_func {
            if let Some(cap) = re_fn.captures(clean_line) {
                in_func = true;
                func_name = cap[1].to_string();
                block_depth = 1;
            }
        } else {
            if clean_line_stripped.ends_with('{') && clean_line_stripped != "{" {
                block_depth += 1;
            } else if clean_line_stripped == "}" {
                block_depth -= 1;
                if block_depth == 0 {
                    in_func = false;
                }
            }
        }
        
        if idx == cursor_line {
            if in_func {
                return Some(func_name);
            }
            break;
        }
    }
    None
}

fn clean_line_preserving_length(line: &str) -> String {
    let mut chars: Vec<char> = line.chars().collect();
    let mut in_string = false;
    let mut string_char = '"';
    let mut i = 0;
    while i < chars.len() {
        if !in_string {
            if chars[i] == '/' && i + 1 < chars.len() && chars[i+1] == '/' {
                for j in i..chars.len() {
                    chars[j] = ' ';
                }
                break;
            } else if chars[i] == '"' || chars[i] == '\'' {
                in_string = true;
                string_char = chars[i];
                chars[i] = ' ';
            }
        } else {
            if chars[i] == '\\' && i + 1 < chars.len() {
                chars[i] = ' ';
                chars[i+1] = ' ';
                i += 2;
                continue;
            } else if chars[i] == string_char {
                in_string = false;
                chars[i] = ' ';
            } else {
                chars[i] = ' ';
            }
        }
        i += 1;
    }
    chars.into_iter().collect()
}

fn resolve_var_type(text: &str, func_name: &str, var_name: &str) -> Option<String> {
    if var_name == "self" {
        let lines: Vec<&str> = text.split('\n').collect();
        let mut current_impl = String::new();
        let mut in_impl = false;
        let mut impl_depth = 0;
        let mut in_func = false;
        let mut block_depth = 0;
        
        let re_impl = regex::Regex::new(r"^\s*impl\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
        let re_fn = regex::Regex::new(r"^\s*(?:pub\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();
        
        for line in lines {
            let clean_line = if let Some(pos) = line.find("//") {
                &line[..pos]
            } else {
                line
            };
            let clean_line_stripped = clean_line.trim();
            
            if !in_impl {
                if let Some(cap) = re_impl.captures(clean_line) {
                    in_impl = true;
                    current_impl = cap[1].to_string();
                    impl_depth = 1;
                }
            } else {
                if !in_func {
                    if let Some(cap) = re_fn.captures(clean_line) {
                        if &cap[1] == func_name {
                            return Some(current_impl + "*");
                        }
                        in_func = true;
                        block_depth = 1;
                    }
                    if clean_line_stripped.ends_with('{') && clean_line_stripped != "{" {
                        impl_depth += 1;
                    } else if clean_line_stripped == "}" {
                        impl_depth -= 1;
                        if impl_depth == 0 {
                            in_impl = false;
                            current_impl.clear();
                        }
                    }
                } else {
                    if clean_line_stripped.ends_with('{') && clean_line_stripped != "{" {
                        block_depth += 1;
                    } else if clean_line_stripped == "}" {
                        block_depth -= 1;
                        if block_depth == 0 {
                            in_func = false;
                        }
                    }
                }
            }
        }
    }
    
    let lines: Vec<&str> = text.split('\n').collect();
    let mut in_func = false;
    let mut block_depth = 0;
    
    let re_fn_sig = regex::Regex::new(&format!(r"^\s*(?:pub\s+)?fn\s+{}\s*\((.*?)\)", regex::escape(func_name))).unwrap();
    let re_decl = regex::Regex::new(r"^\s*([a-zA-Z0-9_<>*\[\]:]+)[[:space:]]*(\*+)?[[:space:]]*([a-zA-Z_][a-zA-Z0-9_]*)(?:\s*=\s*(.+))?$").unwrap();
    let re_word = regex::Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap();
    
    for line in lines {
        let clean_line = clean_line_preserving_length(&line);
        let clean_line_stripped = clean_line.trim();
        
        if !in_func {
            if let Some(cap) = re_fn_sig.captures(&clean_line) {
                in_func = true;
                block_depth = 1;
                let arg_list = &cap[1];
                for arg in arg_list.split(',') {
                    let words: Vec<&str> = re_word.find_iter(arg).map(|m| m.as_str()).collect();
                    if let Some(&last) = words.last() {
                        if last == var_name {
                            let type_part = arg.replace(var_name, "");
                            return Some(type_part.trim().to_string());
                        }
                    }
                }
            }
        } else {
            if clean_line_stripped.ends_with('{') && clean_line_stripped != "{" {
                block_depth += 1;
            } else if clean_line_stripped == "}" {
                block_depth -= 1;
                if block_depth == 0 {
                    break;
                }
            }
            
            if let Some(cap) = re_decl.captures(clean_line_stripped) {
                if &cap[3] == var_name {
                    let mut t = cap[1].to_string();
                    if let Some(stars) = cap.get(2) {
                        t.push_str(stars.as_str());
                    }
                    return Some(t);
                }
            }
        }
    }
    
    let re_global = regex::Regex::new(r"(?m)^(?:pub\s+)?(?:const\s+)?([a-zA-Z0-9_<>*\[\]:]+)\s*(\*+)?\s*([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    for cap in re_global.captures_iter(text) {
        if &cap[3] == var_name {
            let mut t = cap[1].to_string();
            if let Some(stars) = cap.get(2) {
                t.push_str(stars.as_str());
            }
            return Some(t);
        }
    }
    
    None
}

fn parse_impls_in_text(text: &str, base_type: &str) -> Vec<String> {
    let mut methods = Vec::new();
    let base_name = base_type.split("::").last().unwrap_or(base_type);
    let lines: Vec<&str> = text.split('\n').collect();
    let mut in_impl = false;
    let mut block_depth = 0;
    
    let re_impl = regex::Regex::new(r"^\s*impl\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
    let re_fn = regex::Regex::new(r"^\s*(?:pub\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    
    for line in lines {
        let clean_line = if let Some(pos) = line.find("//") {
            &line[..pos]
        } else {
            line
        };
        let clean_line_stripped = clean_line.trim();
        
        if !in_impl {
            if let Some(cap) = re_impl.captures(clean_line) {
                if &cap[1] == base_type || &cap[1] == base_name {
                    in_impl = true;
                    block_depth = 1;
                }
            }
        } else {
            if clean_line_stripped.ends_with('{') && clean_line_stripped != "{" {
                block_depth += 1;
            } else if clean_line_stripped == "}" {
                block_depth -= 1;
                if block_depth == 0 {
                    in_impl = false;
                    continue;
                }
            }
            if let Some(cap) = re_fn.captures(clean_line) {
                methods.push(cap[1].to_string());
            }
        }
    }
    methods
}

fn get_impl_methods(current_file_uri: &str, base_type: &str) -> Vec<String> {
    let mut methods = Vec::new();
    let text = {
        let vfs = get_vfs().lock().unwrap();
        vfs.get(current_file_uri).cloned().unwrap_or_default()
    };
    methods.extend(parse_impls_in_text(&text, base_type));
    
    let re_import = regex::Regex::new(r"\buse\s+([a-zA-Z0-9_:]+)\s+as\s+([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    let current_path = if current_file_uri.starts_with("file://") {
        &current_file_uri[7..]
    } else {
        current_file_uri
    };
    if let Some(current_dir) = std::path::Path::new(current_path).parent() {
        for cap in re_import.captures_iter(&text) {
            let mod_path = &cap[1];
            let parts: Vec<&str> = mod_path.split("::").collect();
            let mut rel_path_1 = current_dir.to_path_buf();
            for p in &parts {
                rel_path_1.push(p);
            }
            rel_path_1.set_extension("c4");
            
            let mut rel_path_2 = current_dir.to_path_buf();
            if let Some(last) = parts.last() {
                rel_path_2.push(last);
            }
            rel_path_2.set_extension("c4");
            
            let target_path = if rel_path_1.is_file() {
                Some(rel_path_1)
            } else if rel_path_2.is_file() {
                Some(rel_path_2)
            } else {
                None
            };
            if let Some(path) = target_path {
                if let Ok(content) = std::fs::read_to_string(path) {
                    methods.extend(parse_impls_in_text(&content, base_type));
                }
            }
        }
    }
    methods.sort();
    methods.dedup();
    methods
}

fn parse_struct_fields(text: &str, struct_name: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let struct_base = struct_name.split("::").last().unwrap_or(struct_name);
    let re_struct = regex::Regex::new(&format!(r"(?m)^\s*(?:pub\s+)?struct\s+{}\b[^{{]*\{{", regex::escape(struct_base))).unwrap();
    if let Some(m) = re_struct.find(text) {
        let start_pos = m.end();
        let mut depth = 1;
        let mut i = start_pos;
        let chars: Vec<char> = text.chars().collect();
        let mut body = String::new();
        while i < chars.len() {
            if chars[i] == '{' {
                depth += 1;
            } else if chars[i] == '}' {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            body.push(chars[i]);
            i += 1;
        }
        
        let re_field = regex::Regex::new(r"(?m)^\s*(?:pub\s+)?([a-zA-Z0-9_<>*\[\]:]+)[[:space:]]*(\*+)?[[:space:]]+([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
        for cap in re_field.captures_iter(&body) {
            let field_name = &cap[3];
            fields.push(field_name.to_string());
        }
    }
    fields
}

fn get_struct_fields(current_file_uri: &str, struct_name: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let text = {
        let vfs = get_vfs().lock().unwrap();
        vfs.get(current_file_uri).cloned().unwrap_or_default()
    };
    fields.extend(parse_struct_fields(&text, struct_name));
    
    let re_import = regex::Regex::new(r"\buse\s+([a-zA-Z0-9_:]+)\s+as\s+([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    let current_path = if current_file_uri.starts_with("file://") {
        &current_file_uri[7..]
    } else {
        current_file_uri
    };
    if let Some(current_dir) = std::path::Path::new(current_path).parent() {
        for cap in re_import.captures_iter(&text) {
            let mod_path = &cap[1];
            let parts: Vec<&str> = mod_path.split("::").collect();
            let mut rel_path_1 = current_dir.to_path_buf();
            for p in &parts {
                rel_path_1.push(p);
            }
            rel_path_1.set_extension("c4");
            
            let mut rel_path_2 = current_dir.to_path_buf();
            if let Some(last) = parts.last() {
                rel_path_2.push(last);
            }
            rel_path_2.set_extension("c4");
            
            let target_path = if rel_path_1.is_file() {
                Some(rel_path_1)
            } else if rel_path_2.is_file() {
                Some(rel_path_2)
            } else {
                None
            };
            if let Some(path) = target_path {
                if let Ok(content) = std::fs::read_to_string(path) {
                    fields.extend(parse_struct_fields(&content, struct_name));
                }
            }
        }
    }
    fields.sort();
    fields.dedup();
    fields
}

fn get_completions(uri: &str, line: usize, char_idx: usize) -> serde_json::Value {
    let text = {
        let vfs = get_vfs().lock().unwrap();
        vfs.get(uri).cloned().unwrap_or_default()
    };
    let lines: Vec<&str> = text.split('\n').collect();
    if line >= lines.len() {
        return serde_json::json!({ "isIncomplete": false, "items": [] });
    }
    
    let cur_line = lines[line];
    let char_limit = char_idx.min(cur_line.len());
    let prefix_line = &cur_line[..char_limit];
    
    let re_member = regex::Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)(?:\.|->)([a-zA-Z_0-9]*)$").unwrap();
    if let Some(cap) = re_member.captures(prefix_line) {
        let var_name = &cap[1];
        let member_prefix = &cap[2];
        let mut completions = Vec::new();
        let func_name = get_current_func(&text, line).unwrap_or_default();
        if let Some(var_type) = resolve_var_type(&text, &func_name, var_name) {
                let re_clean = regex::Regex::new(r"<.*?>|[*\[\]\s]").unwrap();
                let base_type = re_clean.replace_all(&var_type, "");
                let fields = get_struct_fields(uri, &base_type);
                for f in fields {
                    if f.starts_with(member_prefix) {
                        completions.push(serde_json::json!({ "label": f, "kind": 5 }));
                    }
                }
                let methods = get_impl_methods(uri, &base_type);
                for m in methods {
                    if m.starts_with(member_prefix) {
                        completions.push(serde_json::json!({ "label": m, "kind": 2 }));
                    }
                }
            }
        return serde_json::json!({ "isIncomplete": false, "items": completions });
    }
    
    let mut completions = Vec::new();
    
    let keywords = [
        "use", "foreign", "fn", "struct", "enum", "sum", "union", "impl", "type", "const", "pub", "as",
        "if", "elif", "else", "while", "loop", "match", "return", "break", "continue", "and", "or", "not",
        "sizeof", "alignof", "true", "false", "null", "self"
    ];
    let types = [
        "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "f32", "f64", "char", "str", "bool", "void"
    ];
    let intrinsics = [
        "@read", "@write", "@advance", "@offset", "@diff"
    ];
    
    for kw in &keywords {
        completions.push(serde_json::json!({ "label": kw, "kind": 14 }));
    }
    for t in &types {
        completions.push(serde_json::json!({ "label": t, "kind": 25, "detail": "type" }));
    }
    for intr in &intrinsics {
        completions.push(serde_json::json!({ "label": intr, "kind": 3 }));
    }
    
    let re_fn = regex::Regex::new(r"\bfn\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
    let mut funcs = Vec::new();
    for cap in re_fn.captures_iter(&text) {
        funcs.push(cap[1].to_string());
    }
    funcs.sort();
    funcs.dedup();
    for fn_name in &funcs {
        completions.push(serde_json::json!({ "label": fn_name, "kind": 3 }));
    }
    
    let re_type = regex::Regex::new(r"\b(?:struct|enum|sum|union|type)\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
    let mut type_names = Vec::new();
    for cap in re_type.captures_iter(&text) {
        type_names.push(cap[1].to_string());
    }
    type_names.sort();
    type_names.dedup();
    for t_name in &type_names {
        completions.push(serde_json::json!({ "label": t_name, "kind": 25, "detail": "type" }));
    }
    
    let re_import = regex::Regex::new(r"\buse\s+([a-zA-Z0-9_:]+)\s+as\s+([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    let mut imports = Vec::new();
    for cap in re_import.captures_iter(&text) {
        let mod_path = &cap[1];
        let alias = &cap[2];
        imports.push((mod_path.to_string(), alias.to_string()));
        let imported = get_imported_symbols(uri, mod_path);
        for sym in imported {
            completions.push(serde_json::json!({ "label": format!("{}::{}", alias, sym), "kind": 3 }));
        }
    }
    
    let re_word = regex::Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap();
    let mut all_words = Vec::new();
    for cap in re_word.captures_iter(&text) {
        all_words.push(cap[0].to_string());
    }
    all_words.sort();
    all_words.dedup();
    
    for word in all_words {
        if !keywords.contains(&word.as_str()) && !types.contains(&word.as_str()) {
            if !funcs.contains(&word) && !type_names.contains(&word) && !imports.iter().any(|(_, alias)| alias == &word) {
                completions.push(serde_json::json!({ "label": word, "kind": 6 }));
            }
        }
    }
    
    serde_json::json!({ "isIncomplete": false, "items": completions })
}

fn get_hover(uri: &str, line: usize, char_idx: usize) -> serde_json::Value {
    let text = {
        let vfs = get_vfs().lock().unwrap();
        vfs.get(uri).cloned().unwrap_or_default()
    };
    let lines: Vec<&str> = text.split('\n').collect();
    if line >= lines.len() {
        return serde_json::Value::Null;
    }
    
    let cur_line = lines[line];
    let re_word = regex::Regex::new(r"@?[a-zA-Z_0-9]+").unwrap();
    let mut word = String::new();
    for m in re_word.find_iter(cur_line) {
        if m.start() <= char_idx && char_idx <= m.end() {
            word = m.as_str().to_string();
            break;
        }
    }
    
    if word.is_empty() {
        return serde_json::Value::Null;
    }
    
    let hover_docs = HashMap::from([
        ("@read", "### C4 Intrinsic: `@read`\nReads the value stored at the pointer address.\n\n**Syntax:**\n`ptr @read()` -> `T`"),
        ("@write", "### C4 Intrinsic: `@write`\nWrites a value to the pointer address.\n\n**Syntax:**\n`ptr @write(value)` -> `void`"),
        ("@advance", "### C4 Intrinsic: `@advance`\nAdvances the pointer by the given element offset (scaled by type size).\n\n**Syntax:**\n`ptr @advance(offset)` -> `T*`"),
        ("@offset", "### C4 Intrinsic: `@offset`\nCalculates byte offset relative to the pointer.\n\n**Syntax:**\n`ptr @offset(bytes)` -> `T*`"),
        ("@diff", "### C4 Intrinsic: `@diff`\nCalculates pointer difference.\n\n**Syntax:**\n`ptr1 @diff(ptr2)` -> `i64`"),
        ("sizeof", "### C4 Operator: `sizeof`\nReturns the size of the given type in bytes.\n\n**Syntax:**\n`sizeof(Type)` -> `u64`"),
        ("alignof", "### C4 Operator: `alignof`\nReturns the alignment of the given type in bytes.\n\n**Syntax:**\n`alignof(Type)` -> `u64`"),
    ]);
    
    if let Some(&doc) = hover_docs.get(word.as_str()) {
        return serde_json::json!({
            "contents": {
                "kind": "markdown",
                "value": doc
            }
        });
    }
    
    let re_sig = regex::Regex::new(&format!(r"(?m)^\s*(?:pub\s+)?(fn\s+{}\s*\(.*?\)[^{{]*)", regex::escape(&word))).unwrap();
    if let Some(cap) = re_sig.captures(&text) {
        let sig = cap[1].trim().to_string();
        return serde_json::json!({
            "contents": {
                "kind": "markdown",
                "value": format!("```c4\n{}\n```", sig)
            }
        });
    }
    
    let func_name = get_current_func(&text, line).unwrap_or_default();
    if let Some(var_type) = resolve_var_type(&text, &func_name, &word) {
            return serde_json::json!({
                "contents": {
                    "kind": "markdown",
                    "value": format!("```c4\n(variable) {}: {}\n```", word, var_type)
                }
            });
        }
    
    let re_type_def = regex::Regex::new(&format!(r"(?m)^\s*(?:pub\s+)?((?:struct|enum|sum|union|type)\s+{}\b[^{{]*)", regex::escape(&word))).unwrap();
    if let Some(cap) = re_type_def.captures(&text) {
        let type_def = cap[1].trim().to_string();
        return serde_json::json!({
            "contents": {
                "kind": "markdown",
                "value": format!("```c4\n{}\n```", type_def)
            }
        });
    }
    
    serde_json::Value::Null
}

fn check_line_words(
    expr_str: &str,
    line_idx: usize,
    globals_declared: &[String],
    local_scope: &[String],
    diagnostics: &mut Vec<serde_json::Value>,
) {
    let clean = regex::Regex::new(r"->[a-zA-Z_][a-zA-Z0-9_]*").unwrap().replace_all(expr_str, "");
    let clean2 = regex::Regex::new(r"\.[a-zA-Z_][a-zA-Z0-9_]*").unwrap().replace_all(&clean, "");
    
    let re_word = regex::Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)*\b").unwrap();
    let keywords = [
        "use", "foreign", "fn", "struct", "enum", "sum", "union", "impl", "type", "const", "pub", "as",
        "if", "elif", "else", "while", "loop", "match", "return", "break", "continue", "and", "or", "not",
        "sizeof", "alignof", "true", "false", "null", "self"
    ];
    let types = [
        "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "f32", "f64", "char", "str", "bool", "void"
    ];
    
    for m in re_word.find_iter(&clean2) {
        let word = m.as_str();
        if keywords.contains(&word) || types.contains(&word) {
            continue;
        }
        if word.contains("::") {
            let parts: Vec<&str> = word.split("::").collect();
            let base = parts[0];
            if !globals_declared.contains(&base.to_string()) && !globals_declared.contains(&word.to_string()) {
                diagnostics.push(serde_json::json!({
                    "range": {
                        "start": { "line": line_idx, "character": m.start() },
                        "end": { "line": line_idx, "character": m.end() }
                    },
                    "severity": 1,
                    "message": format!("Undeclared module or symbol '{}'", word)
                }));
            }
            continue;
        }
        
        if !globals_declared.contains(&word.to_string()) && !local_scope.contains(&word.to_string()) {
            diagnostics.push(serde_json::json!({
                "range": {
                    "start": { "line": line_idx, "character": m.start() },
                    "end": { "line": line_idx, "character": m.end() }
                },
                "severity": 1,
                "message": format!("Undeclared variable '{}'", word)
            }));
        }
    }
}

fn check_diagnostics(uri: &str) -> Vec<serde_json::Value> {
    let text = {
        let vfs = get_vfs().lock().unwrap();
        vfs.get(uri).cloned().unwrap_or_default()
    };
    let mut diagnostics = Vec::new();
    let mut globals_declared = vec![
        "printf".to_string(), "malloc".to_string(), "free".to_string()
    ];
    
    let re_fn = regex::Regex::new(r"\bfn\s+([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    for cap in re_fn.captures_iter(&text) {
        globals_declared.push(cap[1].to_string());
    }
    
    let re_type = regex::Regex::new(r"\b(?:struct|enum|sum|union|type)\s+([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    for cap in re_type.captures_iter(&text) {
        globals_declared.push(cap[1].to_string());
    }
    
    let re_impl = regex::Regex::new(r"\bimpl\s+([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    for cap in re_impl.captures_iter(&text) {
        globals_declared.push(cap[1].to_string());
    }
    
    let re_import = regex::Regex::new(r"\buse\s+([a-zA-Z0-9_:]+)\s+as\s+([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    for cap in re_import.captures_iter(&text) {
        let mod_path = &cap[1];
        let alias = &cap[2];
        globals_declared.push(alias.to_string());
        let imported = get_imported_symbols(uri, mod_path);
        for sym in imported {
            globals_declared.push(format!("{}::{}", alias, sym));
        }
    }
    
    let re_global = regex::Regex::new(r"(?m)^(?:pub\s+)?(?:const\s+)?([a-zA-Z0-9_<>*\[\]:]+)\s*(\*+)?\s*([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    let keywords = [
        "use", "foreign", "fn", "struct", "enum", "sum", "union", "impl", "type", "const", "pub", "as",
        "if", "elif", "else", "while", "loop", "match", "return", "break", "continue", "and", "or", "not",
        "sizeof", "alignof", "true", "false", "null", "self"
    ];
    let types = [
        "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "f32", "f64", "char", "str", "bool", "void"
    ];
    for cap in re_global.captures_iter(&text) {
        let t_name = &cap[1];
        let var_name = &cap[3];
        if !keywords.contains(&t_name) {
            globals_declared.push(var_name.to_string());
        }
    }
    
    let lines: Vec<&str> = text.split('\n').collect();
    let mut in_function = false;
    let mut in_impl = false;
    let mut local_scope: Vec<String> = Vec::new();
    let mut block_depth = 0;
    let mut impl_depth = 0;
    
    let re_impl_decl = regex::Regex::new(r"^\s*impl\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
    let re_fn_decl = regex::Regex::new(r"^\s*(?:pub\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\((.*?)\)[^{]*\{\s*$").unwrap();
    let re_word = regex::Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap();
    let re_decl = regex::Regex::new(r"^\s*([a-zA-Z0-9_<>*\[\]:]+)[[:space:]]*(\*+)?[[:space:]]*([a-zA-Z_][a-zA-Z0-9_]*)(?:\s*=\s*(.+))?$").unwrap();
    
    let re_intrinsic = regex::Regex::new(r"@[a-zA-Z_][a-zA-Z0-9_]*").unwrap();
    for (idx, line) in lines.iter().enumerate() {
        let mut clean_line = clean_line_preserving_length(line);
        clean_line = re_intrinsic.replace_all(&clean_line, |caps: &regex::Captures| {
            " ".repeat(caps[0].len())
        }).to_string();
        let clean_line_stripped = clean_line.trim();
        if clean_line_stripped.is_empty() {
            continue;
        }
        
        if !in_impl {
            if re_impl_decl.captures(&clean_line).is_some() {
                in_impl = true;
                impl_depth = 1;
                continue;
            }
        } else {
            if !in_function {
                if clean_line_stripped.ends_with('{') && clean_line_stripped != "{" {
                    impl_depth += 1;
                } else if clean_line_stripped == "}" {
                    impl_depth -= 1;
                    if impl_depth == 0 {
                        in_impl = false;
                        continue;
                    }
                }
            }
        }
        
        if !in_function {
            if let Some(cap) = re_fn_decl.captures(&clean_line) {
                in_function = true;
                block_depth = 1;
                local_scope.clear();
                if in_impl {
                    local_scope.push("self".to_string());
                }
                let arg_list = &cap[2];
                for arg in arg_list.split(',') {
                    let words: Vec<&str> = re_word.find_iter(arg).map(|m| m.as_str()).collect();
                    if let Some(&last) = words.last() {
                        local_scope.push(last.to_string());
                    }
                }
                continue;
            }
        } else {
            if clean_line_stripped.ends_with('{') && clean_line_stripped != "{" {
                block_depth += 1;
            } else if clean_line_stripped == "}" {
                block_depth -= 1;
                if block_depth == 0 {
                    in_function = false;
                    continue;
                }
            }
            
            if clean_line_stripped.ends_with('{') && clean_line_stripped != "{" {
                let is_control = ["if", "elif", "while", "loop", "match", "fn", "pub", "struct", "enum", "sum", "union", "impl"]
                    .iter()
                    .any(|kw| clean_line_stripped.starts_with(kw));
                if !is_control {
                    let re_paren = regex::Regex::new(r"\((.*?)\)").unwrap();
                    if let Some(m_paren) = re_paren.captures(clean_line_stripped) {
                        for word in re_word.find_iter(&m_paren[1]) {
                            local_scope.push(word.as_str().to_string());
                        }
                    }
                    let re_brace = regex::Regex::new(r"\{(.*?)\}").unwrap();
                    if let Some(m_brace) = re_brace.captures(clean_line_stripped) {
                        for part in m_brace[1].split(',') {
                            let words: Vec<&str> = re_word.find_iter(part).map(|m| m.as_str()).collect();
                            if let Some(&last) = words.last() {
                                local_scope.push(last.to_string());
                            }
                        }
                    }
                }
            }
            
            if let Some(cap) = re_decl.captures(clean_line_stripped) {
                let t_name = cap[1].to_string() + cap.get(2).map(|m| m.as_str()).unwrap_or("");
                let var_name = &cap[3];
                let is_type = types.contains(&t_name.as_str())
                    || globals_declared.contains(&t_name)
                    || t_name.chars().next().map_or(false, |c| c.is_uppercase())
                    || t_name.contains('*')
                    || t_name.contains('[')
                    || t_name.contains('<')
                    || t_name.contains("::");
                if is_type {
                    local_scope.push(var_name.to_string());
                    let check_expr = cap.get(4).map(|m| m.as_str()).unwrap_or("");
                    check_line_words(check_expr, idx, &globals_declared, &local_scope, &mut diagnostics);
                    continue;
                }
            }
            check_line_words(&clean_line, idx, &globals_declared, &local_scope, &mut diagnostics);
        }
    }
    diagnostics
}

fn handle_request(payload: &str) {
    let Ok(json) = serde_json::from_str::<serde_json::Value>(payload) else {
        return;
    };
    let Some(method) = json.get("method").and_then(|v| v.as_str()) else {
        return;
    };
    let id_node = json.get("id").cloned();
    let params = json.get("params");
    
    match method {
        "initialize" => {
            let result = serde_json::json!({
                "capabilities": {
                    "textDocumentSync": 1,
                    "completionProvider": {
                        "resolveProvider": false,
                        "triggerCharacters": ["@", ".", ">"]
                    },
                    "hoverProvider": true
                }
            });
            send_response(id_node, result);
        }
        "initialized" => {}
        "textDocument/didOpen" => {
            if let Some(doc) = params.and_then(|p| p.get("textDocument")) {
                if let (Some(uri), Some(text)) = (doc.get("uri").and_then(|u| u.as_str()), doc.get("text").and_then(|t| t.as_str())) {
                    let mut vfs = get_vfs().lock().unwrap();
                    vfs.insert(uri.to_string(), text.to_string());
                    drop(vfs);
                    publish_diagnostics_lsp(uri);
                }
            }
        }
        "textDocument/didChange" => {
            if let Some(doc) = params.and_then(|p| p.get("textDocument")) {
                if let Some(uri) = doc.get("uri").and_then(|u| u.as_str()) {
                    if let Some(changes) = params.and_then(|p| p.get("contentChanges")).and_then(|c| c.as_array()) {
                        if let Some(change) = changes.first() {
                            if let Some(text) = change.get("text").and_then(|t| t.as_str()) {
                                let mut vfs = get_vfs().lock().unwrap();
                                vfs.insert(uri.to_string(), text.to_string());
                                drop(vfs);
                                publish_diagnostics_lsp(uri);
                            }
                        }
                    }
                }
            }
        }
        "textDocument/completion" => {
            if let Some(doc) = params.and_then(|p| p.get("textDocument")) {
                if let Some(uri) = doc.get("uri").and_then(|u| u.as_str()) {
                    if let Some(pos) = params.and_then(|p| p.get("position")) {
                        let line = pos.get("line").and_then(|l| l.as_u64()).unwrap_or(0) as usize;
                        let character = pos.get("character").and_then(|c| c.as_u64()).unwrap_or(0) as usize;
                        let result = get_completions(uri, line, character);
                        send_response(id_node, result);
                    }
                }
            }
        }
        "textDocument/hover" => {
            if let Some(doc) = params.and_then(|p| p.get("textDocument")) {
                if let Some(uri) = doc.get("uri").and_then(|u| u.as_str()) {
                    if let Some(pos) = params.and_then(|p| p.get("position")) {
                        let line = pos.get("line").and_then(|l| l.as_u64()).unwrap_or(0) as usize;
                        let character = pos.get("character").and_then(|c| c.as_u64()).unwrap_or(0) as usize;
                        let result = get_hover(uri, line, character);
                        send_response(id_node, result);
                    }
                }
            }
        }
        "shutdown" => {
            send_response(id_node, serde_json::Value::Null);
        }
        "exit" => {
            std::process::exit(0);
        }
        _ => {}
    }
}

fn publish_diagnostics_lsp(uri: &str) {
    let diags = check_diagnostics(uri);
    let params = serde_json::json!({
        "uri": uri,
        "diagnostics": diags
    });
    send_notification("textDocument/publishDiagnostics", params);
}

fn main() {
    loop {
        match read_message() {
            Ok(Some(msg)) => {
                handle_request(&msg);
            }
            Ok(None) => break,
            Err(_) => break,
        }
    }
}
