use rand::Rng;
use std::fs::File;

use image::codecs::gif::{GifDecoder, GifEncoder};
use image::{AnimationDecoder, ImageBuffer, ImageDecoder};

const DENSITY: f64 = 1.0;
const CAR_SIZE: usize = 1;
const CAR_NUM: usize = 500; //must be even number
const FRAME_NUM : usize = 3000;

#[derive(Clone, Copy, PartialEq)]
enum Car {
    Blue,
    Red,
    None,
    Update,
}

fn main() -> std::io::Result<()> {
    let imgx = (CAR_NUM * CAR_SIZE) as u32;
    let imgy = (CAR_NUM * CAR_SIZE) as u32;

    let mut model = model();

    let mut frames = Vec::new();

    for index in 0..FRAME_NUM {
        let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
        for i in 0..imgx {
            for j in 0..imgy {
                let pixel = imgbuf.get_pixel_mut(j, i);
                *pixel = match model.cars[i as usize][j as usize] {
                    Car::Blue => image::Rgba([78, 104, 150, 255]),
                    Car::Red => image::Rgba([184, 120, 139, 255]),
                    _ => image::Rgba([253, 248, 241, 0]),
                }
            }
        }

        let frame = image::Frame::from_parts(imgbuf,0,0,image::Delay::from_numer_denom_ms(20, 1));
        frames.push(frame);
        update(&mut model);
        println!("{}/{}",index+1,FRAME_NUM);
    }

    let mut file_out = File::create("target/out.gif").expect("koko");
    let mut encoder = GifEncoder::new(file_out);
    println!("encoding");
    encoder.encode_frames(frames.into_iter());

    Ok(())
}

struct Model {
    cars: [[Car; CAR_NUM]; CAR_NUM],
    // blue_standby: [u8; CAR_NUM],
    // red_standby: [u8; CAR_NUM],
    blue_turn: bool,
}

fn model() -> Model {
    let mut cars = [[Car::None; CAR_NUM]; CAR_NUM];
    let mut rng = rand::thread_rng();
    for i in 0..CAR_NUM {
        for j in 0..CAR_NUM {
            let r: f64 = rng.gen();
            if r < (DENSITY / 2.0) {
                cars[i][j] = Car::Blue;
            } else if r < DENSITY {
                cars[i][j] = Car::Red;
            }
        }
    }
    for i in 0..CAR_NUM {
        for j in 0..CAR_NUM {
            if (i % 2) == 0 && cars[i][j] == Car::Blue {
                cars[i][j] = Car::None;
            }
            if (j % 2) == 0 && cars[i][j] == Car::Red {
                cars[i][j] = Car::None;
            }
        }
    }
    Model {
        cars,
        // blue_standby: [0; CAR_NUM],
        // red_standby: [0; CAR_NUM],
        blue_turn: true,
    }
}

fn update(model: &mut Model) {
    if model.blue_turn {
        model.blue_turn = false;
        
        for i in 0..CAR_NUM {
            for j in 0..CAR_NUM - 1 {
                match model.cars[i][j] {
                    Car::Blue => {
                        if model.cars[i][j + 1] == Car::None {
                            model.cars[i][j] = Car::None;
                            model.cars[i][j + 1] = Car::Update;
                        }
                    }
                    _ => (),
                }
            }
        }
        for i in 0..CAR_NUM  {
            match model.cars[i][CAR_NUM-1] {
                Car::Blue => {
                    if model.cars[i][0] == Car::None {
                        model.cars[i][CAR_NUM-1] = Car::None;
                        model.cars[i][0] = Car::Update;
                    }
                }
                _ => (),
            }
        }
       
        for i in 0..CAR_NUM {
            for j in 0..CAR_NUM {
                match model.cars[i][j] {
                    Car::Update => {
                        model.cars[i][j] = Car::Blue;
                    }
                    _ => (),
                }
            }
        }
    } else {
        model.blue_turn = true;
        
        for i in 0..CAR_NUM - 1 {
            for j in 0..CAR_NUM {
                match model.cars[i][j] {
                    Car::Red => {
                        if model.cars[i + 1][j] == Car::None {
                            model.cars[i][j] = Car::None;
                            model.cars[i + 1][j] = Car::Update;
                        }
                    }
                    _ => (),
                }
            }
        }
        for j in 0..CAR_NUM  {
            match model.cars[CAR_NUM-1][j] {
                Car::Red => {
                    if model.cars[0][j] == Car::None {
                        model.cars[CAR_NUM-1][j] = Car::None;
                        model.cars[0][j] = Car::Update;
                    }
                }
                _ => (),
            }
        }
        
        for i in 0..CAR_NUM {
            for j in 0..CAR_NUM {
                match model.cars[i][j] {
                    Car::Update => {
                        model.cars[i][j] = Car::Red;
                    }
                    _ => (),
                }
            }
        }
    }
}
