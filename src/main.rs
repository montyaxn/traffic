use rand::Rng;
use std::fs::File;

use image::codecs::gif::GifEncoder;

const DENSITY: f64 = 0.5; // can not be bigger than 0.5
const CAR_NUM: usize = 500; // must be even number
const SAND_MAX: usize = 500; // must be even number too
const TIDE_FREQUENCY: i32 = 300;
const FRAME_NUM: usize = 1000;

// traffic status
#[derive(Clone, Copy, PartialEq)]
enum Tstatus {
    Blue,
    Red,
    None,
    Update,
}

fn main() -> std::io::Result<()> {
    let imgx = CAR_NUM as u32;
    let imgy = CAR_NUM as u32;

    // init model
    let mut rng = rand::thread_rng();
    let mut counter: i32 = 0;
    let mut model = model();

    let mut frames = Vec::new();
    for index in 0..FRAME_NUM {
        let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

        // convert Tstatus to rgba
        for i in 0..imgx {
            for j in 0..imgy {
                let pixel = imgbuf.get_pixel_mut(j, i);
                *pixel = match model.traffic[i as usize][j as usize] {
                    Tstatus::Blue => {
                        if 0 < j
                            && j < imgy - 1
                            && model.traffic[i as usize][(j+1) as usize] == Tstatus::Red
                            && model.traffic[i as usize][(j-1) as usize] == Tstatus::Red
                            && rng.gen_bool(0.3)
                        {
                            image::Rgba([255, 200, 230, 255])
                        } else {
                            image::Rgba([78, 104, 150, 255])
                        }
                    }
                    Tstatus::Red => image::Rgba([184, 120, 139, 255]),
                    _ => image::Rgba([255, 255, 255, 0]),
                }
            }
        }

        // make frame from buffer
        let frame =
            image::Frame::from_parts(imgbuf, 0, 0, image::Delay::from_numer_denom_ms(20, 1));
        frames.push(frame);

        // update model
        simple_update(&mut model);

        // rotate occasionally
        if counter % 5 == 0 {
            rotate(&mut model);
        };
        let p = ((((counter + TIDE_FREQUENCY) % (TIDE_FREQUENCY * 2)) - TIDE_FREQUENCY).abs() ^ 2)
            as f64
            / (TIDE_FREQUENCY ^ 2) as f64;
        if rng.gen_bool(1. / 10. + p * 1. / 5.) {
            randomize(&mut model, (SAND_MAX as f64 * p) as usize);
            print!("shuffled");
        };
        counter += 1;

        println!("{}/{}", index + 1, FRAME_NUM);
    }

    let file_out = File::create("target/out.gif").expect("couldn't make target file");
    let mut encoder = GifEncoder::new(file_out);

    println!("encoding, can open gif even while encoding");
    encoder.encode_frames(frames.into_iter()).unwrap();

    Ok(())
}

struct Model {
    traffic: [[Tstatus; CAR_NUM]; CAR_NUM],
    // blue_standby: [u8; CAR_NUM],
    // red_standby: [u8; CAR_NUM],
    blue_turn: bool,
    odd_was_odd: bool,
}

// initializer for model
fn model() -> Model {
    // fill with None
    let mut traffic = [[Tstatus::None; CAR_NUM]; CAR_NUM];

    // generate car according to the density
    let mut rng = rand::thread_rng();
    // generate with double the density
    for i in 0..CAR_NUM {
        for j in 0..CAR_NUM {
            let r: f64 = rng.gen();
            if r < (DENSITY) {
                traffic[i][j] = Tstatus::Blue;
            } else if r < (DENSITY * 2.) {
                traffic[i][j] = Tstatus::Red;
            }
        }
    }
    // delete blue cars at even numbered colmns and red cars at even numbered rows
    for i in 0..CAR_NUM {
        for j in 0..CAR_NUM {
            if (i % 2) == 0 && traffic[i][j] == Tstatus::Blue {
                traffic[i][j] = Tstatus::None;
            }
            if (j % 2) == 0 && traffic[i][j] == Tstatus::Red {
                traffic[i][j] = Tstatus::None;
            }
        }
    }
    Model {
        traffic,
        // blue_standby: [0; CAR_NUM],
        // red_standby: [0; CAR_NUM],
        blue_turn: true,
        odd_was_odd: true,
    }
}

// simple updater
fn simple_update(model: &mut Model) {
    if model.blue_turn {
        model.blue_turn = false;

        for i in 0..CAR_NUM {
            for j in 0..CAR_NUM - 1 {
                match model.traffic[i][j] {
                    Tstatus::Blue => {
                        if model.traffic[i][j + 1] == Tstatus::None {
                            model.traffic[i][j] = Tstatus::None;
                            model.traffic[i][j + 1] = Tstatus::Update;
                        }
                    }
                    _ => (),
                }
            }
        }
        for i in 0..CAR_NUM {
            match model.traffic[i][CAR_NUM - 1] {
                Tstatus::Blue => {
                    if model.traffic[i][0] == Tstatus::None {
                        model.traffic[i][CAR_NUM - 1] = Tstatus::None;
                        model.traffic[i][0] = Tstatus::Update;
                    }
                }
                _ => (),
            }
        }

        for i in 0..CAR_NUM {
            for j in 0..CAR_NUM {
                match model.traffic[i][j] {
                    Tstatus::Update => {
                        model.traffic[i][j] = Tstatus::Blue;
                    }
                    _ => (),
                }
            }
        }
    } else {
        model.blue_turn = true;

        for i in 0..CAR_NUM - 1 {
            for j in 0..CAR_NUM {
                match model.traffic[i][j] {
                    Tstatus::Red => {
                        if model.traffic[i + 1][j] == Tstatus::None {
                            model.traffic[i][j] = Tstatus::None;
                            model.traffic[i + 1][j] = Tstatus::Update;
                        }
                    }
                    _ => (),
                }
            }
        }
        for j in 0..CAR_NUM {
            match model.traffic[CAR_NUM - 1][j] {
                Tstatus::Red => {
                    if model.traffic[0][j] == Tstatus::None {
                        model.traffic[CAR_NUM - 1][j] = Tstatus::None;
                        model.traffic[0][j] = Tstatus::Update;
                    }
                }
                _ => (),
            }
        }

        for i in 0..CAR_NUM {
            for j in 0..CAR_NUM {
                match model.traffic[i][j] {
                    Tstatus::Update => {
                        model.traffic[i][j] = Tstatus::Red;
                    }
                    _ => (),
                }
            }
        }
    }
}

// rotate !
fn rotate(model: &mut Model) {
    let clone = model.traffic.clone();
    for i in 1..CAR_NUM {
        for j in 0..CAR_NUM - 1 {
            model.traffic[i][j] = clone[i - 1][j + 1];
        }
    }
    for i in 1..CAR_NUM {
        model.traffic[i][CAR_NUM - 1] = clone[i - 1][0];
    }
    for j in 0..CAR_NUM - 1 {
        model.traffic[0][j] = clone[CAR_NUM - 1][j + 1];
    }
    model.traffic[0][CAR_NUM - 1] = clone[CAR_NUM - 1][0];

    // // some randomeness
    // let mut rng = rand::thread_rng();
    // let speed = rng.gen_range(1..=2) * 2 + 1;
    // let target_id = rng.gen_range(0..CAR_NUM*2-1);
    // let thickness : i32 = rng.gen_range(3..6);
    // for i in -thickness..=thickness {
    //     let id = ((target_id as i32  + i)%(CAR_NUM*2-1)as i32)as usize;
    //     if id+1<=speed || 2*CAR_NUM-1<=id+speed{}
    //     else if id <= CAR_NUM-1{
    //         for n in 0..id+1-speed{
    //             model.traffic[id-n][n] = clone[id-n-speed][n+speed];
    //         }
    //         for n in 0..speed{
    //             model.traffic[speed-1-n][1+id-speed+n] = clone[id-n][n];
    //         }
    //     }else{
    //         for n in 0..2*CAR_NUM-1-id-speed{
    //             model.traffic[CAR_NUM-n-1][id-CAR_NUM+1+n] = clone[CAR_NUM-n-1-speed][id-CAR_NUM+1+n+speed];
    //         }
    //         for n in 0..speed{
    //             model.traffic[CAR_NUM-(2*CAR_NUM-id-1-speed)-n-1][id-CAR_NUM+1+(2*CAR_NUM-id-1-speed)+n] = clone[CAR_NUM-n-1][id-CAR_NUM+1+n];
    //         }
    //     }
    // }

    model.odd_was_odd = !model.odd_was_odd;
}

fn randomize(model: &mut Model, sand: usize) {
    let mut rng = rand::thread_rng();
    for s in 0..sand {
        if rng.gen_bool(0.04) {
            for i in 0..s {
                let r: f64 = rng.gen();
                if r < (DENSITY) {
                    model.traffic[CAR_NUM - 1 - i][s - 1 - i] = Tstatus::Blue;
                } else if r < (DENSITY * 2.) {
                    model.traffic[CAR_NUM - 1 - i][s - 1 - i] = Tstatus::Red;
                }
            }

            // delete blue cars at even numbered colmns and red cars at even numbered rows
            for i in 0..s {
                if model.odd_was_odd {
                    if (CAR_NUM - 1 - i) % 2 == 0 && model.traffic[CAR_NUM - 1 - i][s - 1 - i] == Tstatus::Blue{
                        model.traffic[CAR_NUM - 1 - i][s - 1 - i] = Tstatus::None;
                    }else if (s - 1 - i) % 2 == 0 && model.traffic[CAR_NUM - 1 - i][s - 1 - i] == Tstatus::Red{
                        model.traffic[CAR_NUM - 1 - i][s - 1 - i] = Tstatus::None;
                    }
                    
                } else{
                    if (CAR_NUM - i) % 2 == 0 && model.traffic[CAR_NUM - 1 - i][s - 1 - i] == Tstatus::Blue{
                        model.traffic[CAR_NUM - 1 - i][s - 1 - i] = Tstatus::None;
                    }else if (s - i) % 2 == 0 && model.traffic[CAR_NUM - 1 - i][s - 1 - i] == Tstatus::Red{
                        model.traffic[CAR_NUM - 1 - i][s - 1 - i] = Tstatus::None;
                    }
                }
            }
        }
    }
}
