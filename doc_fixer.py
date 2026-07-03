import os
import re

def fix_missing_docs(directory):
    for root, dirs, files in os.walk(directory):
        for file in files:
            if not file.endswith('.rs'):
                continue
            filepath = os.path.join(root, file)
            with open(filepath, 'r', encoding='utf-8') as f:
                lines = f.readlines()
            
            new_lines = []
            for i, line in enumerate(lines):
                # Check if this line defines a public item
                match = re.search(r'^\s*pub\s+(struct|enum|fn|trait|const|static|mod|use)\s+', line)
                if match:
                    # Check if the previous line(s) have a doc comment or attribute
                    has_doc = False
                    j = i - 1
                    while j >= 0:
                        prev_line = lines[j].strip()
                        if prev_line.startswith('///') or prev_line.startswith('/**'):
                            has_doc = True
                            break
                        if prev_line.startswith('#[') or prev_line == '':
                            j -= 1
                        else:
                            break
                    if not has_doc:
                        # Add doc comment
                        indent = line[:len(line) - len(line.lstrip())]
                        item_type = match.group(1)
                        # Extract name
                        name_match = re.search(r'^\s*pub\s+(?:struct|enum|fn|trait|const|static|mod)\s+([A-Za-z0-9_]+)', line)
                        if item_type == 'use':
                            new_lines.append(indent + '/// Exported module/item.\n')
                        else:
                            name = name_match.group(1) if name_match else item_type
                            new_lines.append(indent + f'/// Documentation for {name}.\n')
                
                # Check for public fields in structs
                # (Simple heuristic: pub field: type)
                field_match = re.search(r'^\s*pub\s+([A-Za-z0-9_]+)\s*:', line)
                if field_match and not line.strip().startswith('//') and not line.strip().startswith('pub fn'):
                    has_doc = False
                    j = i - 1
                    while j >= 0:
                        prev_line = lines[j].strip()
                        if prev_line.startswith('///') or prev_line.startswith('/**'):
                            has_doc = True
                            break
                        if prev_line.startswith('#[') or prev_line == '':
                            j -= 1
                        else:
                            break
                    if not has_doc:
                        indent = line[:len(line) - len(line.lstrip())]
                        name = field_match.group(1)
                        new_lines.append(indent + f'/// Documentation for field `{name}`.\n')

                # Check for enum variants
                # This is harder to catch reliably with regex, we will try
                enum_variant_match = re.search(r'^\s*([A-Z][a-zA-Z0-9_]*)\s*(?:\{|\(|,|$)', line)
                if enum_variant_match and 'enum' not in line and not line.strip().startswith('//') and not line.strip().startswith('pub '):
                    # We only want to do this if we are inside an enum block
                    # Let's skip enum variants for now to avoid false positives, 
                    # or we can do a pass just looking for missing docs.
                    pass
                    
                new_lines.append(line)
                
            with open(filepath, 'w', encoding='utf-8') as f:
                f.writelines(new_lines)

if __name__ == "__main__":
    fix_missing_docs('crates')
    # Also add top-level docs to lib.rs / main.rs if they lack #![no_std] or //!
    for root, dirs, files in os.walk('crates'):
        for file in files:
            if file in ['lib.rs', 'main.rs', 'mod.rs']:
                filepath = os.path.join(root, file)
                with open(filepath, 'r', encoding='utf-8') as f:
                    content = f.read()
                if '//! ' not in content:
                    with open(filepath, 'w', encoding='utf-8') as f:
                        f.write(f'//! Auto-generated crate documentation for {os.path.basename(root)}.\n\n' + content)

