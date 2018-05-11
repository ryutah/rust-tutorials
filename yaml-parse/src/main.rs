extern crate serde_json;
extern crate serde_yaml;

static YAML: &'static str = r#"
foo:
- bar: abc
  foobar: cde
- bar: bbb
  foobar: ccc
"#;

fn main() {
    let map: serde_json::Map<String, serde_json::Value> = serde_yaml::from_str(YAML).unwrap();
    println!("{:?}", map);

    let s = serde_yaml::to_string(&map).unwrap();
    println!("{}", s);
}
