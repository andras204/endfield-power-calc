use crate::*;

const R: &str = "\x1b[0m";
const Y: &str = "\x1b[93m";
const O: &str = "\x1b[33m";
const C: &str = "\x1b[96m";
const G: &str = "\x1b[92m";
const B: &str = "\x1b[94m";
const D: &str = "\x1b[90m";
const U: &str = "\x1b[4m";

pub fn print_build(full_tbs: f64, div_stack: &Option<Vec<i32>>) {
    let mut build = Vec::new();
    if div_stack.is_some() {
        let div_stack = div_stack.as_ref().unwrap();
        build.extend_from_slice(&[
            format!("├─full thermal banks: {Y}{full_tbs:.0}{R}"),
            format!("└┬divider stack: {Y}{}{R}", format_slice(&div_stack)),
            format!(" ├─length: {B}{}{R}", div_stack.len()),
            format!(
                " ├─3s: {G}{}{R}",
                div_stack.iter().filter(|&&x| x == 3).count()
            ),
            format!(
                " └─2s: {C}{}{R}",
                div_stack.iter().filter(|&&x| x == 2).count()
            ),
        ]);
    } else {
        build.push(format!("└─full thermal banks: {Y}{full_tbs:.0}{R}"));
    }
    print_with_box(&build, &format!("{U}BUILD{R}"));
}

pub fn print_stats(
    power_draw: f64,
    battery_power: f64,
    full_tbs: f64,
    div_stack: &Option<Vec<i32>>,
) {
    let mut stats = Vec::new();
    if div_stack.is_some() {
        let div_stack = div_stack.as_ref().unwrap();
        let mut div_stack_feed_rate = BELT_THROUGHPUT;
        let mut div_stack_feed_interval = BELT_INTERVAL as f64;
        for &d in div_stack.iter() {
            div_stack_feed_rate /= d as f64;
            div_stack_feed_interval *= d as f64;
        }
        let off_time = div_stack_feed_interval - TB_PROCESS_TIME;
        let lowest_reserve = PAC_POWER_RESERVE - (power_draw * off_time);
        let charge_time = (PAC_POWER_RESERVE - lowest_reserve) / (battery_power - power_draw);
        let efficiency = ((charge_time * battery_power)
            + ((TB_PROCESS_TIME - charge_time) * power_draw))
            / (TB_PROCESS_TIME * battery_power);
        let power_headroom = f64::min(lowest_reserve / off_time, battery_power - power_draw);
        stats.extend_from_slice(&[
            format!(
                "├┬PAC power reserve: {}",
                format_time(PAC_POWER_RESERVE / power_draw)
            ),
            format!("│├─power deficit: {C}{power_draw:.0}{R}"),
        ]);
        if lowest_reserve < 3500f64 {
            stats.push(format!("│├┬lowest reserve: {C}{lowest_reserve:.0}{R}"));
            stats.push(format!(
                "││└─{O}⚠︎ the game doesn't display reserve power below 3%{R}"
            ));
        } else {
            stats.push(format!("│├─lowest reserve: {C}{lowest_reserve:.0}{R}"));
        }
        stats.extend_from_slice(&[
            format!("│├─charge time: {B}~{}", format_time(charge_time)),
            format!("│├─efficiency: {G}{:.2}%{R}", 100f64 * efficiency),
            format!("│└─power headroom: {C}{:.2}{R}", power_headroom),
            format!(
                "├┬div stack feed interval: {}",
                format_time(div_stack_feed_interval)
            ),
            format!("│├─off time: {}", format_time(off_time)),
            format!("│└─feed rate: {Y}{div_stack_feed_rate:.3}{D}/m{R}"),
            format!(
                "└─total battery consumption: {Y}{:.3}{D}/m{R}",
                full_tbs * TB_FEED_RATE + div_stack_feed_rate
            ),
        ]);
    } else {
        stats.extend_from_slice(&[
            format!(
                "├─efficiency: {G}{:.2}%{R}",
                100f64 * (power_draw / battery_power)
            ),
            format!(
                "└─total battery consumption: {Y}{:.1}{D}/m{R}",
                full_tbs * TB_FEED_RATE
            ),
        ]);
    }
    print_with_box(&stats, &format!("{U}STATS{R}"));
}

fn print_with_box(lines: &[String], title: &str) {
    let mut header = "╭".to_string();
    header.push_str(&"─".repeat(32));
    header.push_str(title);
    header.push_str("─┄┈");
    println!("{}", header);
    for s in lines {
        println!("{}", s);
    }
}

fn format_slice(v: &[i32]) -> String {
    let mut buf = format!("{D}[");
    for x in 0..(v.len() - 1) {
        if v[x] == 2 {
            buf.push_str(&format!("{C}2{D}, "));
        } else {
            buf.push_str(&format!("{G}3{D}, "));
        }
    }
    if *v.last().unwrap() == 2 {
        buf.push_str(&format!("{C}2"));
    } else {
        buf.push_str(&format!("{G}3"));
    }
    buf.push_str(&format!("{D}]{R}"));
    buf
}

fn format_time(mut seconds: f64) -> String {
    if seconds < 60f64 {
        return format!("{B}{seconds:.0}s{R}");
    }

    let mut minutes = (seconds / 60f64).floor();
    seconds = seconds % 60f64;

    if minutes < 60f64 {
        return format!("{B}{minutes:.0}m {seconds:.0}s{R}");
    }

    let hours = (minutes / 60f64).floor();
    minutes %= 60f64;
    return format!("{B}{hours:.0}h {minutes:.0}m {seconds:.0}s{R}");
}
