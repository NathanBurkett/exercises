const MATCH_3: &str = "Pling";
const MATCH_5: &str = "Plang";
const MATCH_7: &str = "Plong";

pub fn raindrops(n: usize) -> String {

    let mut output = String::new();

    for factor in factors(n) {
        let sound = match_factor_to_raindrop(factor);
        if !sound.is_empty() && !output.contains(sound) {
            output.push_str(sound)
        }
    }

    if output.is_empty() {
        output = n.to_string();
    }

    output
}

fn factors(num: usize) -> Vec<usize> {
    let mut factors: Vec<usize> = Vec::new();

    for i in 1..((num as f32).sqrt() as usize + 1) {
        if num % i == 0 {
            factors.push(i);
            factors.push(num/i);
        }
    }

    factors.sort();
    factors
}

fn match_factor_to_raindrop<'a>(factor: usize) -> &'a str {
    match factor {
        3 => MATCH_3,
        5 => MATCH_5,
        7 => MATCH_7,
        _ => "",
    }
}
