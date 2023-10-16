use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn sort_domain(domain_list: Vec<&str>) -> Vec<String> {
    // Sort domain list by domain root name, then by domain name
    let dedup_domain_list: Vec<String> = domain_list
        .into_iter()
        .map(|s| s.to_string())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();

    let mut domains_by_primary: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    for domain in dedup_domain_list.into_iter() {
        let domain_parts: Vec<&str> = domain.split('.').collect();
        let primary = domain_parts.rchunks(2).next().unwrap().join(".");
        domains_by_primary.entry(primary).or_default().push(
            domain_parts[0..domain_parts.len() - 2]
                .iter()
                .copied()
                .map(|s| s.to_string())
                .rev()
                .collect(),
        );
    }

    let mut primary_list: Vec<String> = domains_by_primary.keys().cloned().collect();
    primary_list.sort();

    let mut ret = Vec::new();

    for domain in primary_list {
        domains_by_primary.get_mut(&domain).unwrap().sort();
        for sub_parts in domains_by_primary.get(&domain).unwrap().iter() {
            let mut parts = sub_parts.clone();
            parts.reverse();
            parts.push(domain.clone());
            ret.push(parts.join("."));
        }
    }
    ret
}

fn main() {
    println!("cargo:rerun-if-changed=domains.txt");
    println!("cargo:rerun-if-changed=build.rs");

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

    let mut f = File::create(dest_path).unwrap();
    f.write_all(buf.as_bytes()).unwrap();
}
