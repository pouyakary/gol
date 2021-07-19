
//
// Pouya's very first rust code :)
//

//
// ─── IMPORTS ────────────────────────────────────────────────────────────────────
//

    use std::{thread, time};

//
// ─── CONSTANTS ──────────────────────────────────────────────────────────────────
//

    const WIDTH: usize =
        40;
    const HEIGHT: usize =
        24;
    const ALIVE: bool =
        true;
    const DEAD: bool =
        false;
    const SCREEN_SIZE: usize =
        WIDTH * HEIGHT;

//
// ─── SCREEN ACCESSORS ───────────────────────────────────────────────────────────
//

    fn get_cell( x: usize, y: usize, game: & [bool] ) -> bool {
        return game[ y * WIDTH + x ]
    }

    fn set_cell( x: usize, y: usize, game: &mut [bool], value: bool ) {
        game[ y * WIDTH + x ] =
            value;
    }

//
// ─── NEIGHBORS ──────────────────────────────────────────────────────────────────
//

    fn count_neighbors( x: usize, y: usize, game: & [bool] ) -> i8 {
        let mut neighbors: i8 =
            0;

        // top left neighbor
        if x > 0 && y > 0 && get_cell( x - 1, y - 1, & game ) {
            neighbors += 1;
        }

        // top neighbor
        if y > 0 && get_cell( x, y - 1, & game ) {
            neighbors += 1;
        }

        // top right neighbor
        if x < WIDTH - 1 && y > 0 && get_cell( x + 1, y - 1, & game ) {
            neighbors += 1;
        }

        // right neighbor
        if x < WIDTH - 1 && y > 0 && get_cell( x + 1, y, & game ) {
            neighbors += 1;
        }

        // bottom right neighbor
        if x < WIDTH - 1 && y < HEIGHT - 1 && get_cell( x + 1, y + 1, & game ) {
            neighbors += 1;
        }

        // bottom neighbor
        if y < HEIGHT - 1 && get_cell( x, y + 1, & game ) {
            neighbors += 1;
        }

        // bottom left neighbor
        if x > 0 && y < HEIGHT - 1 && get_cell( x - 1, y + 1, & game ) {
            neighbors += 1;
        }

        // left neighbor
        if x > 0 && get_cell( x - 1, y, & game ) {
            neighbors += 1;
        }

        return neighbors;
    }

//
// ─── RUN GAME FRAME ─────────────────────────────────────────────────────────────
//

    fn run_game( previous_game: & [bool] ) -> [bool; SCREEN_SIZE] {
        let mut new_game: [ bool; SCREEN_SIZE ] =
            [ false; SCREEN_SIZE ];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let neighbors: i8 =
                    count_neighbors( x, y, & previous_game );
                let mut cell =
                    get_cell( x, y, & previous_game );

                // Any live cell with fewer than two live neighbors dies, as if by depopulation.
                if neighbors < 2 {
                    cell = DEAD;
                }

                // Any live cell with two or three live neighbors lives on to the next generation.
                // Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
                else if neighbors == 2 || neighbors == 3 {
                    cell = ( neighbors == 3 ) || cell
                }

                // Any live cell with more than three live neighbors dies, as if by overpopulation.
                else if neighbors > 3 {
                    cell = DEAD;
                }


                set_cell( x, y, & mut new_game, cell );
            }
        }

        return new_game;
    }

//
// ─── SLEEP ──────────────────────────────────────────────────────────────────────
//

    fn sleep( ) {
        let sleep_time = time::Duration::from_millis( 150 );
        thread::sleep( sleep_time );
    }

//
// ─── PRINT SCREEN ───────────────────────────────────────────────────────────────
//

    fn print_game( game: & [bool] ) {
        print!("\x1B[2J\x1B[1;1H");
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if get_cell( x, y, & game ) {
                    print!( "██" );
                } else {
                    print!( "  " );
                }
            }
            println!( "" );
        }
    }

//
// ─── ELEMENTS ───────────────────────────────────────────────────────────────────
//

    fn setup_glider( game: &mut [bool] ) {
        set_cell( 2, 1, game, ALIVE );
        set_cell( 3, 2, game, ALIVE );
        set_cell( 1, 3, game, ALIVE );
        set_cell( 2, 3, game, ALIVE );
        set_cell( 3, 3, game, ALIVE );
    }

    fn setup_toad( game: &mut [bool] ) {
        let dx: usize = 10;
        let dy: usize = 2;
        set_cell( dx + 1, dy + 0, game, ALIVE );
        set_cell( dx + 2, dy + 0, game, ALIVE );
        set_cell( dx + 3, dy + 0, game, ALIVE );

        set_cell( dx + 0, dy + 1, game, ALIVE );
        set_cell( dx + 1, dy + 1, game, ALIVE );
        set_cell( dx + 2, dy + 1, game, ALIVE );
    }

    fn setup_blinker( game: &mut [bool] ) {
        let dx: usize = 20;
        let dy: usize = 2;
        set_cell( dx + 1, dy, game, ALIVE );
        set_cell( dx + 2, dy, game, ALIVE );
        set_cell( dx + 3, dy, game, ALIVE );
    }

    fn setup_beacon( game: &mut [bool] ) {
        let dx: usize = 30;
        let dy: usize = 2;

        set_cell( dx + 0, dy + 0, game, ALIVE );
        set_cell( dx + 1, dy + 0, game, ALIVE );
        set_cell( dx + 0, dy + 1, game, ALIVE );
        set_cell( dx + 1, dy + 1, game, ALIVE );

        set_cell( dx + 2, dy + 2, game, ALIVE );
        set_cell( dx + 3, dy + 2, game, ALIVE );
        set_cell( dx + 2, dy + 3, game, ALIVE );
        set_cell( dx + 3, dy + 3, game, ALIVE );

    }
//
// ─── MAIN ───────────────────────────────────────────────────────────────────────
//

    fn main( ) {
        // where everything happens!
        let mut game: [ bool; SCREEN_SIZE ] =
            [ DEAD; SCREEN_SIZE ];

        setup_glider( &mut game );
        setup_toad( &mut game );
        setup_blinker( &mut game );
        setup_beacon( &mut game );

        loop {
            print_game( & game );
            game = run_game( & game );
            sleep();
        }
    }

// ────────────────────────────────────────────────────────────────────────────────
