use std::{time::Duration, vec};

#[tokio::main]
async fn main() {
    const X: usize = 25;
    const Y: usize = 25;
    let mut map = [false; X * Y];

    spawn_glider(&mut map);

    run(&mut map, 10000).await;
    let _ = std::io::stdin();
    fn iterate(map: &mut [bool; X * Y]) {
        let mut new_map = [false; X * Y];
        for i in 0..X {
            for j in 0..Y {
                new_map[get_1d_index(i, j)] = should_live(i, j, &map);
            }
        }
        *map = new_map;
    }
    fn live_neighbors(i: usize, j: usize, map: &[bool; X * Y]) -> i32 {
        let mut alive = 0;
        let must_check_left = i > 0;
        let must_check_right = i < (X - 1);
        let must_check_top = j > 0;
        let must_check_bottom = j < (Y - 1);
        if must_check_left && map[get_1d_index(i - 1, j)] == true {
            alive += 1;
        }
        if must_check_right && map[get_1d_index(i + 1, j)] == true {
            alive += 1;
        }
        if must_check_top && map[get_1d_index(i, j - 1)] == true {
            alive += 1;
        }
        if must_check_bottom && map[get_1d_index(i, j + 1)] == true {
            alive += 1;
        }
        if must_check_top && must_check_left && map[get_1d_index(i - 1, j - 1)] == true {
            alive += 1;
        }
        if must_check_top && must_check_right && map[get_1d_index(i + 1, j - 1)] == true {
            alive += 1;
        }
        if must_check_bottom && must_check_left && map[get_1d_index(i - 1, j + 1)] == true {
            alive += 1;
        }

        if must_check_bottom && must_check_right && map[get_1d_index(i + 1, j + 1)] == true {
            alive += 1;
        }
        alive
    }
    fn should_live(x: usize, y: usize, map: &[bool; X * Y]) -> bool {
        let alive = map[get_1d_index(x, y)];
        let alive_neighbors = live_neighbors(x, y, &map);
        if (alive && alive_neighbors == 2) || alive_neighbors == 3 {
            return true;
        }
        if !alive && alive_neighbors == 3 {
            return true;
        }
        false
    }
    fn render(map: [bool; X * Y]) {
        let mut render_string: String = String::new();
        for i in 0..X {
            for j in 0..Y {
                if map[get_1d_index(i, j)] == true {
                    render_string.push('⬜')
                } else {
                    render_string.push('⬛')
                }
            }
            render_string.push('\n');
        }
        println!("{render_string}");
    }
    async fn run(map: &mut [bool; X * Y], iterations: i32) {
        use std::time::Instant;
        let now = Instant::now();
        // let mut interval = tokio::time::interval(Duration::from_millis(50));
        for _i in 0..iterations {
            // render(*map);
            iterate(map);
            // interval.tick().await;
            print!("{esc}c", esc = 27 as char);
        }
        println!("Elapsed: {:.2?}", now.elapsed());
    }

    fn spawn_glider(map: &mut [bool; X * Y]) {
        map[get_1d_index(1, 2)] = true;
        map[get_1d_index(2, 3)] = true;
        map[get_1d_index(3, 1)] = true;
        map[get_1d_index(3, 2)] = true;
        map[get_1d_index(3, 3)] = true;
    }
    fn get_1d_index(row: usize, col: usize) -> usize {
        return row * Y + col;
    }
}
