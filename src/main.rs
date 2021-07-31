
extern crate lifegame;

use std::{thread, time};
use rand::prelude::*;

use lifegame::LifeGame;
use lifegame::Cell;


fn main() {
    let mut l = LifeGame::new();

    //init_lifes(&mut l);
    init_lifes_random(&mut l);
        
    AnsiEscape::clear_screen();
    //AnsiEscape::hide_cursor();

    loop {
        AnsiEscape::move_to_top();

        render(&l);

        thread::sleep(time::Duration::from_secs(1));

        if l.update() == false {
            break;
        }
    }
}


fn init_lifes(l: &mut LifeGame) {
    let c = l.generate_character();

    l.cells[3][3] = Cell::Life(c);
    l.cells[4][3] = Cell::Life(c);
    l.cells[3][4] = Cell::Life(c);
    
    l.cells[6][5] = Cell::Life(c);
    l.cells[6][6] = Cell::Life(c);
    l.cells[5][6] = Cell::Life(c);
}


fn init_lifes_random(l: &mut LifeGame) {
    let mut rng = rand::thread_rng();
    let c = l.generate_character();

    for x in 0..l.x_size {
        for y in 0..l.y_size {
            if rng.gen_range(0..10) < 3 {
                l.cells[x][y] = Cell::Life(c);
            } else {
                l.cells[x][y] = Cell::Death;
            }
        }
    }
}



fn render(l: &LifeGame) {

    for m in l.cells.iter() {
        for n in m.iter() {
            let c = match n {
                Cell::Death => ' ',
                Cell::Life(n) => *n
            };
            print!("{}", c);
        }
        println!("");
    }
}


struct AnsiEscape {
}


impl AnsiEscape {
    fn clear_screen() {
        print!("\x1b[2J");
        print!("\x1b[0;0H");
    }

    fn move_to_top() {
        print!("\x1b[0;0H");
    }

    fn show_cursor() {
        print!("\x1b[?25h");
    }

    fn hide_cursor() {
        print!("\x1b[?25l");
    }
}




