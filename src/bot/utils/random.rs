pub fn get_random_bool(username: String) -> bool {
    let blacklist = std::env::var("BLACKLIST")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    if blacklist.contains(&username) {
        return get_probabilistic_bool(0.3333); // 🥸
    }
    let mut buffer = [0u8; 1];
    if getrandom::fill(&mut buffer).is_err() {
        buffer[0] = rand::random();
    }
    buffer[0] % 2 == 0
}

pub fn get_probabilistic_bool(true_probability: f64) -> bool {
    let mut buffer = [0u8; 8];
    if getrandom::fill(&mut buffer).is_err() {
        buffer = rand::random();
    }

    let random_u64 = u64::from_le_bytes(buffer);
    let max_u64 = u64::MAX;

    let random_float: f64 = (random_u64 as f64) / (max_u64 as f64);
    random_float < true_probability
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    const N_RUNS: usize = 10_000_000;

    #[test]
    fn test_get_random_bool() {
        env::set_var("BLACKLIST", "");

        let mut true_count = 0;
        for _ in 0..N_RUNS {
            if get_random_bool("test_user".to_string()) {
                true_count += 1;
            }
        }

        let probability = (true_count as f64) / (N_RUNS as f64);
        println!("Observed probability: {}", probability);

        assert!(
            (0.49..=0.51).contains(&probability),
            "Probability {} is out of expected range",
            probability
        );
    }

    #[test]
    fn test_get_probabilistic_bool() {
        let expected_probabilities = [0.1, 0.25, 0.5, 0.75, 0.9];

        for &p in &expected_probabilities {
            let mut true_count = 0;
            for _ in 0..N_RUNS {
                if get_probabilistic_bool(p) {
                    true_count += 1;
                }
            }

            let probability = (true_count as f64) / (N_RUNS as f64);
            println!("Expected: {}, Observed: {}", p, probability);

            let lower_bound = p - 0.01;
            let upper_bound = p + 0.01;

            assert!(
                (lower_bound..=upper_bound).contains(&probability),
                "For p = {}, observed probability {} is out of range [{}, {}]",
                p,
                probability,
                lower_bound,
                upper_bound
            );
        }
    }
}
