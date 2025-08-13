// JS13K Space Invaders - Rust/WASM implementation
// Size optimization: no_std to avoid standard library overhead (~100KB)
#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use alloc::vec;
use core::panic::PanicInfo;
use core::clone::Clone;

// Custom panic handler - required for no_std
// Size optimization: Simple loop instead of unwinding (~5KB saved)
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

// Size optimization: wee_alloc is ~1KB vs default allocator ~10KB
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Game constants
const WIDTH: usize = 400;   // Canvas width in pixels
const HEIGHT: usize = 300;  // Canvas height in pixels
const SCALE: usize = 10;    // Unused - kept for future scaling

// Game state - kept minimal for size
struct Game {
    player_x: i32,                      // Player horizontal position
    enemies: Vec<(i32, i32, bool)>,     // (x, y, alive) for each enemy
    enemy_dx: i32,                      // Enemy movement direction
    bullets: Vec<(i32, i32)>,           // Player bullets (x, y)
    enemy_bullets: Vec<(i32, i32)>,     // Enemy bullets (x, y)
    score: u32,                         // Current score
    game_over: bool,                    // Game state flag
    tick: u32,                          // Frame counter for timing
    screen: Vec<u8>,                    // RGBA pixel buffer (WIDTH*HEIGHT*4)
}

impl Game {
    fn new() -> Game {
        let mut enemies = Vec::new();
        for y in 0..4 {
            for x in 0..8 {
                enemies.push((x * 40 + 60, y * 30 + 30, true));
            }
        }
        
        Game {
            player_x: 200,
            enemies,
            enemy_dx: 1,
            bullets: Vec::new(),
            enemy_bullets: Vec::new(),
            score: 0,
            game_over: false,
            tick: 0,
            screen: vec![0; WIDTH * HEIGHT * 4],
        }
    }

    fn update(&mut self, keys: u8) {
        if self.game_over {
            return;
        }

        self.tick += 1;

        // Player movement
        if keys & 1 != 0 && self.player_x > 20 {
            self.player_x -= 3;
        }
        if keys & 2 != 0 && self.player_x < WIDTH as i32 - 20 {
            self.player_x += 3;
        }
        if keys & 4 != 0 && self.bullets.len() < 2 {
            self.bullets.push((self.player_x, HEIGHT as i32 - 30));
        }

        // Update bullets
        for i in (0..self.bullets.len()).rev() {
            self.bullets[i].1 -= 5;
            if self.bullets[i].1 < 0 {
                self.bullets.remove(i);
                continue;
            }

            // Check enemy collisions
            for enemy in &mut self.enemies {
                if enemy.2 && 
                   (self.bullets[i].0 - enemy.0).abs() < 15 &&
                   (self.bullets[i].1 - enemy.1).abs() < 10 {
                    enemy.2 = false;
                    self.bullets.remove(i);
                    self.score += 10;
                    break;
                }
            }
        }

        // Move enemies
        if self.tick % 20 == 0 {
            let mut hit_edge = false;
            for enemy in &self.enemies {
                if enemy.2 {
                    if enemy.0 + self.enemy_dx * 10 <= 10 || enemy.0 + self.enemy_dx * 10 >= WIDTH as i32 - 10 {
                        hit_edge = true;
                    }
                }
            }
            
            if hit_edge {
                self.enemy_dx = -self.enemy_dx;
                for enemy in &mut self.enemies {
                    enemy.1 += 15;
                    if enemy.2 && enemy.1 > HEIGHT as i32 - 50 {
                        self.game_over = true;
                    }
                }
            } else {
                for enemy in &mut self.enemies {
                    enemy.0 += self.enemy_dx * 10;
                }
            }
        }

        // Enemy bullets
        if self.tick % 60 == 0 {
            for enemy in &self.enemies {
                if enemy.2 && self.tick % 120 == 0 {
                    self.enemy_bullets.push((enemy.0, enemy.1 + 10));
                    break;
                }
            }
        }

        // Update enemy bullets
        for i in (0..self.enemy_bullets.len()).rev() {
            self.enemy_bullets[i].1 += 3;
            if self.enemy_bullets[i].1 > HEIGHT as i32 {
                self.enemy_bullets.remove(i);
                continue;
            }

            if (self.enemy_bullets[i].0 - self.player_x).abs() < 15 &&
               self.enemy_bullets[i].1 > HEIGHT as i32 - 30 {
                self.game_over = true;
            }
        }

        // Check win
        if self.enemies.iter().all(|e| !e.2) {
            self.game_over = true;
        }
    }

    fn render(&mut self) {
        // Clear screen
        for i in 0..WIDTH * HEIGHT {
            let idx = i * 4;
            self.screen[idx] = 0;
            self.screen[idx + 1] = 0;
            self.screen[idx + 2] = 0;
            self.screen[idx + 3] = 255;
        }

        // Draw enemies
        let enemies_to_draw: Vec<_> = self.enemies.iter().filter(|e| e.2).cloned().collect();
        for enemy in enemies_to_draw {
            self.draw_rect(enemy.0 - 15, enemy.1 - 8, 30, 16, 255, 0, 0);
        }

        // Draw player
        self.draw_rect(self.player_x - 15, HEIGHT as i32 - 25, 30, 20, 0, 255, 0);

        // Draw bullets
        let bullets_to_draw = self.bullets.clone();
        for bullet in bullets_to_draw {
            self.draw_rect(bullet.0 - 2, bullet.1 - 4, 4, 8, 255, 255, 0);
        }

        // Draw enemy bullets
        let enemy_bullets_to_draw = self.enemy_bullets.clone();
        for bullet in enemy_bullets_to_draw {
            self.draw_rect(bullet.0 - 2, bullet.1 - 4, 4, 8, 255, 0, 255);
        }

        // Draw score
        self.draw_text(10, 10, self.score);

        if self.game_over {
            self.draw_game_over();
        }
    }

    fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: u8, g: u8, b: u8) {
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx;
                let py = y + dy;
                if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                    let idx = ((py as usize * WIDTH + px as usize) * 4) as usize;
                    self.screen[idx] = r;
                    self.screen[idx + 1] = g;
                    self.screen[idx + 2] = b;
                    self.screen[idx + 3] = 255;
                }
            }
        }
    }

    fn draw_text(&mut self, x: i32, y: i32, score: u32) {
        // Simple score display with blocks
        let digits = [
            (score / 1000) % 10,
            (score / 100) % 10,
            (score / 10) % 10,
            score % 10,
        ];
        
        for (i, d) in digits.iter().enumerate() {
            let dx = x + i as i32 * 8;
            for j in 0..*d {
                self.draw_rect(dx, y + j as i32 * 3, 6, 2, 0, 255, 0);
            }
        }
    }

    fn draw_game_over(&mut self) {
        // Draw GAME OVER with blocks
        let cx = WIDTH as i32 / 2;
        let cy = HEIGHT as i32 / 2;
        
        // G
        self.draw_rect(cx - 50, cy - 10, 20, 2, 255, 255, 255);
        self.draw_rect(cx - 50, cy - 10, 2, 20, 255, 255, 255);
        self.draw_rect(cx - 50, cy + 8, 20, 2, 255, 255, 255);
        self.draw_rect(cx - 32, cy, 2, 10, 255, 255, 255);
        self.draw_rect(cx - 40, cy, 10, 2, 255, 255, 255);
        
        // O
        self.draw_rect(cx - 20, cy - 10, 15, 2, 255, 255, 255);
        self.draw_rect(cx - 20, cy - 10, 2, 20, 255, 255, 255);
        self.draw_rect(cx - 20, cy + 8, 15, 2, 255, 255, 255);
        self.draw_rect(cx - 7, cy - 10, 2, 20, 255, 255, 255);
    }
}

// Global game instance - required for WASM/JS interop
static mut GAME: Option<Game> = None;

// WASM exports - these functions are called from JavaScript
// #[no_mangle] prevents Rust from changing function names

#[no_mangle]
pub extern "C" fn init() {
    unsafe {
        GAME = Some(Game::new());
    }
}

#[no_mangle]
pub extern "C" fn tick(keys: u8) {
    // Keys are bitflags: bit 0 = left, bit 1 = right, bit 2 = shoot
    unsafe {
        if let Some(game) = &mut GAME {
            game.update(keys);
            game.render();
        }
    }
}

#[no_mangle]
pub extern "C" fn render() -> *const u8 {
    // Returns pointer to pixel buffer for direct JS access
    // This avoids serialization overhead
    unsafe {
        if let Some(game) = &GAME {
            game.screen.as_ptr()
        } else {
            core::ptr::null()
        }
    }
}