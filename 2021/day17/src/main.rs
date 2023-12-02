use std::cmp::{max, min};

struct Window {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

fn window_contains(x_pos: i32, y_pos: i32, window: &Window) -> bool {
    x_pos >= window.x_min && x_pos <= window.x_max && y_pos >= window.y_min && y_pos <= window.y_max
}

fn will_hit(velocity: (i32, i32), window: &Window) -> bool {
    let mut x_pos = 0;
    let mut y_pos = 0;

    let mut x_velocity = velocity.0;
    let mut y_velocity = velocity.1;

    // This condition assumes that x is positive
    while y_pos >= window.y_min {
        if window_contains(x_pos, y_pos, &window) {
            return true;
        }

        x_pos += x_velocity;
        y_pos += y_velocity;

        if x_velocity > 0 {
            x_velocity -= 1;
        }
        if x_velocity < 0 {
            x_velocity += 1;
        }

        y_velocity -= 1;
    }

    false
}

fn calculate_peak(velocity: i32) -> i32 {
    let mut out = 0;

    let min = min(0, velocity - 1);
    let max = max(0, velocity + 1);

    for i in min..max {
        out += i;
    }

    out
}

fn main() {
    let x_min = 195;
    let x_max = 238;
    let y_min = -93;
    let y_max = -67;

    /* Test data */
    // let x_min = 20;
    // let x_max = 30;
    // let y_min = -10;
    // let y_max = -5;

    let window = Window {
        x_min,
        x_max,
        y_min,
        y_max,
    };

    // Find the minimal x velocity that reaches the window
    let mut min_x_velocity = 0;

    while calculate_peak(min_x_velocity) < window.x_min {
        min_x_velocity += 1;
    }

    let mut max_y_velocity = 0;

    // Find peak y position that can be reached with a shot that hits the target
    for y_velocity in 0..2000 {
        if will_hit((min_x_velocity, y_velocity), &window) {
            max_y_velocity = y_velocity;
        }
    }

    println!("min x velocity: {}", min_x_velocity);
    println!("max y velocity: {}", max_y_velocity);
    println!("peak: {}", calculate_peak(max_y_velocity));

    // Count every velocity that hits
    let mut count = 0;

    for x in min_x_velocity..window.x_max + 1 {
        for y in window.y_min..max_y_velocity + 1 {
            if will_hit((x, y), &window) {
                count += 1;
            }
        }
    }

    println!("Velocities that hit: {}", count);
}
