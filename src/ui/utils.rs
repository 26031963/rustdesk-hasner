use sciter::Value;

pub fn value_crash_workaround(args: Value) -> Vec<Value> {
    match args {
        Value::Array(v) => v,
        v => vec![v],
    }
}

pub fn get_icon() -> Option<Vec<u8>> {
    use std::fs::File;
    use std::io::Read;
    if let Ok(mut f) = File::open("icon.png") {
        let mut buf = vec![];
        if f.read_to_end(&mut buf).is_ok() {
            return Some(buf);
        }
    }
    None
}
