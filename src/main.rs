
extern crate lifegame;

use std::{thread, time};
use rand::prelude::*;

use lifegame::LifeGame;
use lifegame::Life;


fn main() {
    let mut l = LifeGame::new();

    //init_lifes(&mut l);
    init_lifes__random(&mut l);
        
    AnsiEscape::clear_screen();
    AnsiEscape::hide_cursor();

    loop {
        AnsiEscape::move_to_top();

        render(&l);

        thread::sleep(time::Duration::from_secs(1));
        // let mut s = String::new();
        // io::stdin().read_line(&mut s)
        //     .expect("failed to read line");

        l.update();
    }
}


fn init_lifes(l: &mut LifeGame) {
    
    l.lifes[3][3] = Life::birth();
    l.lifes[4][3] = Life::birth();
    l.lifes[3][4] = Life::birth();
    
    l.lifes[6][5] = Life::birth();
    l.lifes[6][6] = Life::birth();
    l.lifes[5][6] = Life::birth();
}


fn init_lifes__random(l: &mut LifeGame) {
  
    let mut rng = rand::thread_rng();

    let size = l.x_size * l.y_size / 4;

    for i in 0..size {
        let x = rng.gen_range(0..l.x_size);
        let y = rng.gen_range(0..l.y_size);
        l.lifes[x][y] = Life::birth();
    }
}



fn render(l: &LifeGame) {

    for m in l.lifes.iter() {
        for n in m.iter() {
            let c = match n {
                Life::Death => ' ',
                Life::Living(n) => *n
            };
            print!("{}", c);
        }
        println!("");
    }

    // for h in 0..l.h {
    //     for w in 0..l.w {
    //         let c = if l.lifes[w][h] == Life::Death {
    //             ' '
    //         } else {
	// 			'█'
	// 			//'*'
    //         };
    //         print!("{}", c);
    //     }

    //     println!("");
    // }
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




