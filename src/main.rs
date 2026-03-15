use std::collections::HashMap;

mod output;

const PAC_POWER_RESERVE: f64 = 100_000.0;
const PAC_POWER_OUTPUT: f64 = 200.0;
const TB_PROCESS_TIME: f64 = 40.0;
const TB_FEED_RATE: f64 = 60f64 / TB_PROCESS_TIME;
const BELT_THROUGHPUT: f64 = 30.0;
const BELT_INTERVAL: i32 = 2;

const BATTERIES: [(&str, f64); 5] = [
    ("LC Valley battery (220)", 220f64),
    ("SC Valley battery (420)", 420f64),
    ("HC Valley battery (1100)", 1100f64),
    ("LC Wuling battery (1600)", 1600f64),
    ("SC Wuling battery (3200)", 3200f64),
];

fn main() {
    let (mut power_draw, battery_power) = input();

    if power_draw < PAC_POWER_OUTPUT {
        println!("The PAC generates enough power...");
        std::process::exit(0);
    }

    power_draw -= PAC_POWER_OUTPUT;

    let mut full_tbs = (power_draw / battery_power).floor();

    power_draw -= full_tbs * battery_power;

    // don't let the total charge delta over a cyle go negative
    let charge_rate = battery_power - power_draw;
    let total_charge = charge_rate * TB_PROCESS_TIME;
    let max_discharge_time = (total_charge / power_draw) + TB_PROCESS_TIME;

    // don't let the PAC discharge completely
    let drain_time = PAC_POWER_RESERVE / power_draw;
    let ideal_battery_feed_interval = drain_time + TB_PROCESS_TIME;

    let div_stack = calc_divider_stack(f64::min(max_discharge_time, ideal_battery_feed_interval));

    if div_stack.is_none() {
        full_tbs += 1f64;
    }

    output::print_build(full_tbs, &div_stack);
    output::print_stats(power_draw, battery_power, full_tbs, &div_stack);

    // final readline to keep console open on windows
    #[cfg(target_os = "windows")]
    let _ = {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap()
    };
}

fn calc_divider_stack(ideal_battery_feed_interval: f64) -> Option<Vec<i32>> {
    let stack_max = ideal_battery_feed_interval.log2().ceil() as usize;
    let stack_min = ideal_battery_feed_interval.log(3f64).floor() as usize;

    let mut solutions = HashMap::new();

    for l in (stack_min..=stack_max).rev() {
        let mut segments = vec![2; l];
        for x in 0..segments.len() {
            let mut t = BELT_INTERVAL;
            for s in segments.iter() {
                t *= s;
            }
            if t as f64 >= ideal_battery_feed_interval {
                break;
            }
            solutions.insert(
                // fun fact: float -> int transformation preserves ordering
                (ideal_battery_feed_interval - (t as f64)).to_bits(),
                segments.clone(),
            );
            segments[x] = 3;
        }
    }

    let best = *solutions.keys().min().unwrap();
    let bs = solutions.remove(&best).unwrap();
    let mut bs_time = BELT_INTERVAL;
    for x in bs.iter() {
        bs_time *= x;
    }
    if (bs_time as f64) < TB_PROCESS_TIME {
        None
    } else {
        Some(bs)
    }
}

fn input() -> (f64, f64) {
    let power_draw = match inquire::Text::new("Input power draw: ")
        .with_validator(|input: &str| match input.parse::<f64>() {
            Ok(n) => Ok(if n > 0f64 {
                inquire::validator::Validation::Valid
            } else {
                inquire::validator::Validation::Invalid("power draw must be more than zero".into())
            }),
            Err(_) => Ok(inquire::validator::Validation::Invalid(
                "cannot parse number".into(),
            )),
        })
        .prompt()
    {
        Ok(text) => text.parse::<f64>().unwrap(),
        Err(_) => std::process::exit(1),
    };

    let b: Vec<&str> = BATTERIES.iter().map(|(name, _)| *name).collect();
    let battery = match inquire::Select::new("Select battery type", b).prompt() {
        Ok(c) => c,
        Err(_) => std::process::exit(1),
    };
    let battery_power = get_power(battery);

    (power_draw, battery_power)
}

fn get_power(battery: &str) -> f64 {
    BATTERIES.iter().find(|(b, _)| *b == battery).unwrap().1
}
