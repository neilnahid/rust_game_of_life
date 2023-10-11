use std::time::Duration;

#[tokio::main]
async fn main() {
    const X: usize = 25;
    const Y: usize = 25;
    let mut map: [[bool; X]; Y] = [[false; X]; Y];

    map = spawn_glider(map);

    let future_map: [[bool; X]; Y] = [[false; X]; Y];
    run(map, future_map, 10000).await;
    let _ = std::io::stdin();
    fn iterate(map: [[bool; X]; Y], mut future_map: [[bool; X]; Y]) -> [[bool; X]; Y] {
        let mut i_idx = 0;
        let mut j_idx = 0;
        for i in map {
            for _j in i {
                future_map[i_idx][j_idx] = should_live(i_idx, j_idx, map);
                j_idx += 1;
            }
            i_idx += 1;
            j_idx = 0;
        }
        future_map
    }
    fn live_neighbors(i: usize, j: usize, map: [[bool; X]; Y]) -> i32 {
        let mut alive = 0;
        let must_check_left = i > 0;
        let must_check_right = i < (X - 1);
        let must_check_top = j > 0;
        let must_check_bottom = j < (Y - 1);
        if must_check_left && map[i - 1][j] == true {
            alive += 1;
        }
        if must_check_right && map[i + 1][j] == true {
            alive += 1;
        }
        if must_check_top && map[i][j - 1] == true {
            alive += 1;
        }
        if must_check_bottom && map[i][j + 1] == true {
            alive += 1;
        }
        if must_check_top && must_check_left && map[i - 1][j - 1] == true {
            alive += 1;
        }
        if must_check_top && must_check_right && map[i + 1][j - 1] == true {
            alive += 1;
        }
        if must_check_bottom && must_check_left && map[i - 1][j + 1] == true {
            alive += 1;
        }

        if must_check_bottom && must_check_right && map[i + 1][j + 1] == true {
            alive += 1;
        }
        alive
    }
    fn should_live(x: usize, y: usize, map: [[bool; X]; Y]) -> bool {
        let alive = map[x][y];
        let alive_neighbors = live_neighbors(x, y, map);
        if (alive && alive_neighbors == 2) || alive_neighbors == 3 {
            return true;
        }
        if !alive && alive_neighbors == 3 {
            return true;
        }
        false
    }
    fn render(map: [[bool; X]; Y]) {
        let mut i_idx = 0;
        let mut j_idx = 0;
        let mut render_string: String = String::new();
        for i in map {
            for _j in i {
                if map[i_idx][j_idx] == true {
                    render_string.push('⬜')
                } else {
                    render_string.push('⬛')
                }
                j_idx += 1;
            }
            render_string.push('\n');
            i_idx += 1;
            j_idx = 0;
        }
        println!("{render_string}");
    }
    async fn run(mut map: [[bool; X]; Y], future_map: [[bool; X]; Y], iterations: i32) {
        use std::time::Instant;
        let now = Instant::now();
        // let mut interval = tokio::time::interval(Duration::from_millis(50));
        for _i in 0..iterations {
            // render(map);
            map = iterate(map, future_map);
            // interval.tick().await;
            print!("{esc}c", esc = 27 as char);
        }
        println!("Elapsed:{0:2?} ", now.elapsed())
    }

    fn spawn_glider(mut map: [[bool; X]; Y]) -> [[bool; X]; Y] {
        map[1][2] = true;
        map[2][3] = true;
        map[3][1] = true;
        map[3][2] = true;
        map[3][3] = true;
        map
    }
}
