use rand::Rng;
use std::fs::File;

use image::codecs::gif::GifEncoder;

const DENSITY: f64 = 0.5; // can not be bigger than 0.5
const CAR_NUM: usize = 500; // must be even number
const FRAME_NUM: usize = 3000;

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
    let mut model = model();

    let mut frames = Vec::new();
    for index in 0..FRAME_NUM {
        let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

        // convert Tstatus to rgba
        for i in 0..imgx {
            for j in 0..imgy {
                let pixel = imgbuf.get_pixel_mut(j, i);
                *pixel = match model.traffic[i as usize][j as usize] {
                    Tstatus::Blue => image::Rgba([78, 104, 150, 255]),
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
        update(&mut model);
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
    // delete blue cars at odd numbered colmns and red cars at odd numbered rows
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
    }
}

// simple updater
fn update(model: &mut Model) {
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
