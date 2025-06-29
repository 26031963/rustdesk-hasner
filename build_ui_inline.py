import re
from pathlib import Path

def strip(s):
    return re.sub(r'\s+\n', '\n', re.sub(r'\n\s+', '\n', s))

def inline_file(filepath):
    path = Path(filepath)
    return path.read_text(encoding='utf-8') if path.exists() else ''

def process_html(path):
    text = inline_file(path)
    text = re.sub(r'@import url\((.*?)\);', lambda m: inline_file(path.parent / m.group(1)), text)
    text = re.sub(r'include\s+"(.*?)";', lambda m: inline_file(path.parent / m.group(1)), text)
    return text

def rust_raw_string(s):
    return 'r#"' + s.replace('"', '\\"') + '"#'

entries = {
    '_COMMON_CSS': strip(inline_file('src/ui/common.css')),
    '_COMMON_TIS': strip(inline_file('src/ui/common.tis')),
    '_INDEX': process_html(Path('src/ui/index.html')),
    '_REMOTE': process_html(Path('src/ui/remote.html')),
    '_CHATBOX': inline_file('src/ui/chatbox.html'),
    '_INSTALL': process_html(Path('src/ui/install.html')),
    '_CONNECTION_MANAGER': process_html(Path('src/ui/cm.html')),
}

with open('src/ui/ui_inline.rs', 'wt', encoding='utf-8') as fh:
    for name, content in entries.items():
        rust_array = f"&[{','.join(str(b) for b in content.encode('utf-8'))}]"
        fh.write(f"const {name}: &[u8] = {rust_array};\n")

    fh.write('''
fn get(data: &[u8]) -> String {
    String::from_utf8_lossy(data).to_string()
}
fn replace(data: &[u8]) -> String {
    let css = get(&_COMMON_CSS[..]);
    let res = get(data).replace("@import url(common.css);", &css);
    let tis = get(&_COMMON_TIS[..]);
    res.replace("include \\"common.tis\\";", &tis)
}
#[inline]
pub fn get_index() -> String {
    replace(&_INDEX[..])
}
#[inline]
pub fn get_remote() -> String {
    replace(&_REMOTE[..])
}
#[inline]
pub fn get_install() -> String {
    replace(&_INSTALL[..])
}
#[inline]
pub fn get_chatbox() -> String {
    replace(&_CHATBOX[..])
}
#[inline]
pub fn get_cm() -> String {
    replace(&_CONNECTION_MANAGER[..])
}
''')
