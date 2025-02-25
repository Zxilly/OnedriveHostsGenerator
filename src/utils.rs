use crate::DOMAIN_LIST;
use std::collections::HashMap;
use std::fmt;

pub trait StringLine {
    fn push_str_line(&mut self, string: &str);
}

impl StringLine for String {
    fn push_str_line(&mut self, string: &str) {
        self.push_str(string);
        self.push('\n');
    }
}

pub fn print_ips<T: fmt::Display>(
    ips: &HashMap<&str, Vec<T>>,
    content: &mut String,
    domain_len: usize,
    ip_len: usize,
    single: bool,
) {
    for domain in DOMAIN_LIST {
        let domain_ips = ips.get(domain);
        match domain_ips {
            Some(domain_ips) => {
                if domain_ips.is_empty() {
                    content.push_str_line(&format!("# {} not resolved", domain));
                    continue;
                }
                if single {
                    content.push_str_line(&format!(
                        "{:w1$} {:>w2$}",
                        domain_ips[0],
                        domain,
                        w1 = ip_len,
                        w2 = domain_len
                    ));
                    continue;
                }
                for ip in domain_ips {
                    content.push_str_line(&format!(
                        "{:w1$} {:>w2$}",
                        ip,
                        domain,
                        w1 = ip_len,
                        w2 = domain_len
                    ));
                }
            }
            None => {
                content.push_str_line(&format!("# {} not resolved", domain));
            }
        }
    }
}