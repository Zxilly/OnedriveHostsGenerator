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

pub fn trim_mean(numbers: &mut Vec<u64>, trimming_percentage: f64) -> u64 {
    numbers.sort();

    let trim_count =
        ((numbers.len() as f64) * (trimming_percentage / 100.0) / 2.0).round() as usize;

    numbers.drain(0..trim_count);
    numbers.drain((numbers.len() - trim_count)..);

    let sum: u64 = numbers.iter().sum();

    sum / numbers.len() as u64
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_mean() {
        let mut numbers = vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50];
        let trimming_percentage = 20.0;

        let result = trim_mean(&mut numbers, trimming_percentage);

        assert_eq!(result, 27);
    }

    #[test]
    fn test_trim_mean_empty_input() {
        let mut numbers: Vec<u64> = vec![];
        let trimming_percentage = 20.0;

        let result = trim_mean(&mut numbers, trimming_percentage);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_trim_mean_single_value() {
        let mut numbers = vec![42];
        let trimming_percentage = 10.0;

        let result = trim_mean(&mut numbers, trimming_percentage);

        assert_eq!(result, 42);
    }
}
