use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn sort_domain(domain_list: Vec<&str>) -> Vec<String> {
    // Sort domain list by domain root name, then by domain name
    let domain_list: HashSet<&str> = domain_list.into_iter().collect();
    let mut domain_list: Vec<String> = domain_list.into_iter().map(String::from).collect();

    domain_list.sort_by(|a, b| {
        let a_parts: Vec<&str> = a.split('.').rev().collect();
        let a = a_parts.join(".");

        let b_parts: Vec<&str> = b.split('.').rev().collect();
        let b = b_parts.join(".");

        a.cmp(&b)
    });

    domain_list
}

fn main() {
    println!("cargo:rerun-if-changed=domains.txt");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("domains.rs");

    let domain_list = include_str!("./domains.txt");
    let domain_list: Vec<&str> = domain_list
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    let domain_list = sort_domain(domain_list);

    let mut buf = format!("static DOMAIN_LIST: [&str; {}] = [\n", domain_list.len());
    for domain in domain_list.into_iter() {
        let line = format!("    \"{}\",\n", domain);
        buf.push_str(&line);
    }
    let buf = format!("{}];", buf);

    let mut f = File::create(dest_path.clone()).unwrap();
    f.write_all(buf.as_bytes()).unwrap();
}
