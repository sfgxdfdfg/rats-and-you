use std::cmp::Ordering;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rand::Rng;
use console::{pad_str, style, Alignment};
use std::fs::File;
use std::io::Write;

pub struct Rat {
    hp: i32,
    damage: i32
}

pub struct Progress {
    tile: u8
}

pub struct Player {
    hp: i32,
    damage: i32
}

fn main() {
    let _ = enable_raw_mode();
    let title = "rats&you";
    let title = pad_str(title, 125, Alignment::Center, None);
    println!("{}", style(title).yellow());
    let main_menu = "|Main menu|";
    let main_menu = pad_str(main_menu, 125, Alignment::Center, None);
    println!("{}", style(main_menu).yellow());
	let controls = "p - START GAME || q - EXIT";
	let controls = pad_str(controls, 120, Alignment::Center, None);
    println!("{}", style(controls).cyan());

    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                state: KeyEventState::NONE,
                kind: KeyEventKind::Press,
            }) => {
                println!("(The program will now exit...)");
                let dur = Duration::new(5, 0);
                sleep(dur);
                exit(0);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::NONE,
                state: KeyEventState::NONE,
                kind: KeyEventKind::Press,
            }) => game_start(),
            _ => ()
        }
    }
}

fn game_start() {
    let difficulty: i32; // Minel lejjebb annal nehezebb, minel feljebb annal konnyebb!
    let set_diff: i32;
    println!("Before you start the game you NEED to set the difficulty!\n(1-10)\n6 - EASY\n5 - NORMAL\n4 - HARD");

    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                state: KeyEventState::NONE,
                kind: KeyEventKind::Press,
            }) => {
                println!("(The program will now exit...)");
                let dur = Duration::new(5, 0);
                sleep(dur);
                exit(0);
            }
			Event::Key(KeyEvent {
				code: KeyCode::Char('4'),
				modifiers: KeyModifiers::NONE,
				state: KeyEventState::NONE,
				kind: KeyEventKind::Press,
			}) => {
				set_diff = 4;
				break;
			}
            Event::Key(KeyEvent {
                code: KeyCode::Char('5'),
                modifiers: KeyModifiers::NONE,
                state: KeyEventState::NONE,
                kind: KeyEventKind::Press,
            }) => {
                set_diff = 5;
                break;
            }
			Event::Key(KeyEvent {
			    code: KeyCode::Char('6'),
			    modifiers: KeyModifiers::NONE,
			    state: KeyEventState::NONE,
			    kind: KeyEventKind::Press,
			}) => {
			    set_diff = 6;
			    break;
			}
            _ => ()
        }
    }

    println!("Setting difficulty: {}", set_diff);
    difficulty = set_diff;

    println!("You enter the dungeon!");
    println!("Press w to move forward!");
    
    let rat = Rat {
        hp: 5,
        damage: 2
    };
    let progress = Progress {
        tile: 0
    };
    let player = Player {
        hp: 20,
        damage: 5
    };
    going_forward(progress.tile, difficulty, rat.hp, rat.damage, player.hp, player.damage, false);
}

// megkell nézni
fn progress_forward() {
    progress += 1;
    println!("progress: {}", progress);
    println!("you advanced 1 tile!\n");
    let rat_spawn_chance = rand::rng().random_range(1..=10);
                
    // Debug start
    println!("tsc: {}", tsc);
    // Debug end

    // Debug start
    println!("rsc: {}", rat_spawn_chance);
    // Debug end
    match rat_spawn_chance.cmp(&diff) {
    Ordering::Less => going_forward(progress, diff, rhp, rdamage, php, pd, true),
    Ordering::Equal => going_forward(progress, diff, rhp, rdamage, php, pd, true),
    Ordering::Greater => {
        disable_raw_mode().unwrap();
        println!("You encountered a rat!");
        fight_with_rat(rhp, rdamage, diff, php, pd, progress);
        break;
    }
}

fn going_forward(mut progress: u8, diff: i32, rhp: i32, rdamage: i32, php: i32, pd: i32, pu: bool) {
    let mut tsc = rand::rng().random_range(4..=6);
    if pu == true {
        tsc = 0
    }

    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                state: KeyEventState::NONE,
                kind: KeyEventKind::Press,
            }) => {
                println!("(The program will now exit...)");
                let dur = Duration::new(5, 0);
                sleep(dur);
                exit(0);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::NONE,
                state: KeyEventState::NONE,
                kind: KeyEventKind::Press,
            }) => {
                progress += 1;
                println!("progress: {}", progress);
                println!("you advanced 1 tile!\n");
                let rat_spawn_chance = rand::rng().random_range(1..=10);
                
                // Debug start
                println!("tsc: {}", tsc);
                // Debug end

                // Debug start
                println!("rsc: {}", rat_spawn_chance);
                // Debug end
                match rat_spawn_chance.cmp(&diff) {
                    Ordering::Less => going_forward(progress, diff, rhp, rdamage, php, pd, true),
                    Ordering::Equal => going_forward(progress, diff, rhp, rdamage, php, pd, true),
                    Ordering::Greater => {
                        disable_raw_mode().unwrap();
                        println!("You encountered a rat!");
                        fight_with_rat(rhp, rdamage, diff, php, pd, progress);
                        break;
                    }
            }}
            // megkell nézni
            Event::Key(KeyEvent {
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::NONE,
                state: KeyEventState::NONE,
                kind: KeyEventKind::Press,
            } => {
                
            }
            _ => ()
        }
    }
}

fn fight_with_rat(mut rhp: i32, rdamage: i32, diff: i32, mut php: i32, pd: i32, p: u8) {
    let attack_chance = rand::rng().random_range(1..=10);
    // Debug start
    println!("ac: {}", attack_chance);
    // Debug end
    println!("Press f to fight!");
    
    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::NONE,
                state: KeyEventState::NONE,
                kind: KeyEventKind::Press,
            }) => {

                match attack_chance.cmp(&diff) {
                    Ordering::Less => {
                        rhp -= pd;
                        // Debug start
                        println!("rhp: {}", rhp);
                        println!("pd: {}", pd);
                        // Debug end
                        if rhp == 0 {
                            println!("You defeated the rat!\n");
                            rhp = 5;
                            going_forward(p, diff, rhp, rdamage, php, pd, true);
                        } else {
                            println!("The rat is not dead!");
                            fight_with_rat(rhp, rdamage, diff, php, pd, p);
                        }
                    }
                    Ordering::Equal => {
                        println!("The rat strikes you!\n");
                        php -= 2;
                        if php == 0 {
                            println!("You are dead!");
                            println!("Your progress: {}", style(p).blue());
							let data_inbytes: &[u8] = &[p];
							let mut data = File::create("data.txt").expect("Could not create file!");
							data.write(data_inbytes).expect("Could not write data!");
							// Debug start
							println!("Created file!");
							// Debug end
                            println!("(The program will now exit...)");
                            let dur = Duration::new(5, 0);
                            sleep(dur);
                            exit(0);
                        }
                        // Debug start
                        println!("php: {}", php);
                        // Debug end
                        fight_with_rat(rhp, rdamage, diff, php, pd, p);
                    }
                    Ordering::Greater => {
                        println!("The rat strikes you!\n");
                        php -= 2;
                        if php == 0 {
                            println!("You are dead!");
                            println!("Your progress: {}", style(p).blue());
							let data_inbytes: &[u8] = &[p];
							let mut data = File::create("data.txt").expect("Could not create file!");
							data.write(data_inbytes).expect("Could not write data!");
							// Debug start
							println!("File created!");
							// Debug end
                            println!("(The program will now exit...)");
                            let dur = Duration::new(5, 0);
                            sleep(dur);
                            exit(0);
                        }
                        // Debug start
                        println!("php: {}", php);
                        // Debug end
                        fight_with_rat(rhp, rdamage, diff, php, pd, p);
                    }
                }
            }
            _ => ()
        }
    }
}
