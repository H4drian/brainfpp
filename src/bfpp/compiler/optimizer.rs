/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */ 

pub fn optimize_bf(bf_code: &str) -> String {
    let mut output_string: String = String::new();
    let chaincode: Vec<&str> = chain_chars(bf_code);

    for chain in chaincode {
        match chain.chars().nth(0).unwrap() {
            '+' => {
                if chain.len() > 9 {
                    let mut temp_string: String = String::new();
                    let chain_is_prime: bool = is_prime(chain.len());
                    let (factor1, factor2): (usize, usize);

                    if chain_is_prime { 
                        (factor1, factor2) = smallest_factor_pair(chain.len() - 1)
                    } else {
                        (factor1, factor2) = smallest_factor_pair(chain.len())
                    }

                    temp_string.push('>');

                    for _ in 0..factor1 { temp_string.push('+') }
                    temp_string.push_str("[<");
                    for _ in 0..factor2 { temp_string.push('+') }
                    temp_string.push_str(">-]<");
                    if chain_is_prime { temp_string.push('+')}

                    output_string.push_str(&temp_string);
                } else {
                    output_string.push_str(chain);
                }
            }
            '-' => {
                if chain.len() > 9 {
                    let mut temp_string: String = String::new();
                    let chain_is_prime: bool = is_prime(chain.len());
                    let (factor1, factor2): (usize, usize);

                    if chain_is_prime { 
                        (factor1, factor2) = smallest_factor_pair(chain.len() - 1)
                    } else {
                        (factor1, factor2) = smallest_factor_pair(chain.len())
                    }

                    temp_string.push('>');

                    for _ in 0..factor1 { temp_string.push('+') }
                    temp_string.push_str("[<");
                    for _ in 0..factor2 { temp_string.push('-') }
                    temp_string.push_str(">-]<");
                    if chain_is_prime { temp_string.push('+') }

                    output_string.push_str(&temp_string);
                } else {
                    output_string.push_str(chain);
                }
            }
                _  => {
                output_string.push_str(chain);
                }
        }
    }

    output_string
}

fn chain_chars(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut current_char = None;
    let mut start_index = 0;

    for (i, c) in s.chars().enumerate() {
        if let Some(prev_char) = current_char {
            if c != prev_char {
                result.push(&s[start_index..i]);
                start_index = i;
            }
        }
        current_char = Some(c);
    }

    if let Some(_prev_char) = current_char {
        result.push(&s[start_index..]);
    }

    result
}

fn smallest_factor_pair(num: usize) -> (usize, usize) {
    let mut pair = (1, num);

    for i in 2..=((num as f64).sqrt() as usize) {
        if num % i == 0 {
            if i < pair.1 / i {
                pair = (i, num / i);
            }
        }
    }

    pair
}

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
