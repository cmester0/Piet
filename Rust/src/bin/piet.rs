use image::open;
use image::Rgb;

use std::collections::HashMap;
use std::collections::HashSet;

use clap::Parser;

use itertools::Itertools;

use ndarray::ArrayView;
use ndarray::Ix2;

use std::io::Read;

use piet::piet_color::*;
use piet::piet_interpreter::*;

// use num::bigint::BigInt;

#[derive(Debug, Copy, Clone)]
struct ABI {
    r_u: (usize, usize),
    r_d: (usize, usize),

    d_r: (usize, usize),
    d_l: (usize, usize),

    l_d: (usize, usize),
    l_u: (usize, usize),

    u_l: (usize, usize),
    u_r: (usize, usize),

    bs: usize,

    c: ValidColor,
}

struct PietImageData {
    width: usize,
    height: usize,
    blobs: [HashSet<(usize, usize)>; 20],
    pix_to_blob: HashMap<(usize, usize), usize>,
    all_blobs_indexed: Vec<ABI>,
}

#[derive(Copy, Clone)]
struct PietCursor {
    cx: usize,
    cy: usize,
    dp: usize,
    cc: usize,
    last_color: ValidColor,
    last_bs: usize,
}

#[derive(Clone)]
struct PietExecution<'a> {
    map: &'a PietImageData,
    cursor: PietCursor,
    stack: Vec<isize>, /* TODO: needs bigint math? */
}

impl<'a> PietExecution<'a> {
    fn check_valid_pixel(&self, (nx, ny): (usize, usize)) -> bool {
        !(self.map.blobs[ValidColor::from("⚫") as usize].contains(&(nx, ny)))
            && (ny < self.map.height)
            && (nx < self.map.width)
    }

    fn next_step_from_dp(&self, (cx, cy): (usize, usize)) -> ((usize, usize), bool) {
        let (nx, ny) = match self.cursor.dp {
            0 if cx < self.map.width => (cx + 1, cy),
            1 if cy < self.map.height => (cx, cy + 1),
            2 if cx > 0 => (cx - 1, cy),
            3 if cy > 0 => (cx, cy - 1),
            _ => return ((usize::MAX, usize::MAX), false),
        };
        let hit_wall = self.check_valid_pixel((nx, ny));

        ((nx, ny), hit_wall)
    }

    fn step(&mut self, input: &mut std::iter::Peekable<std::io::Bytes<std::io::Stdin>>) -> bool {
        let curr_pos = (self.cursor.cx, self.cursor.cy);
        let (next_pos, valid_pix) = self.next_step_from_dp(curr_pos);
        self.continue_step(curr_pos, next_pos, valid_pix, input)
    }

    fn continue_step(
        &mut self,
        mut curr_pos: (usize, usize),
        mut next_pos: (usize, usize),
        mut valid_pix: bool,
        input: &mut std::iter::Peekable<std::io::Bytes<std::io::Stdin>>,
    ) -> bool {
        if !valid_pix {
            let mut block = 0;
            while !valid_pix {
                if block % 2 == 0 {
                    self.cursor.cc = (self.cursor.cc + 1) % 2
                } else {
                    self.cursor.dp = (self.cursor.dp + 1) % 4
                }

                block += 1;
                if block >= 8 {
                    return true;
                }

                if !(self.map.blobs[ValidColor::from("⚪") as usize]
                    .contains(&(self.cursor.cx, self.cursor.cy)))
                {
                    // r_u, r_d, d_r, d_l, l_d, l_u, u_l, u_r, bs, c =
                    let abi = self.map.all_blobs_indexed
                        [self.map.pix_to_blob[&(self.cursor.cx, self.cursor.cy)]];

                    curr_pos = match (self.cursor.dp, self.cursor.cc) {
                        (0, 0) => abi.r_u,
                        (0, 1) => abi.r_d,
                        (1, 0) => abi.d_r,
                        (1, 1) => abi.d_l,
                        (2, 0) => abi.l_d,
                        (2, 1) => abi.l_u,
                        (3, 0) => abi.u_l,
                        (3, 1) => abi.u_r,
                        _ => panic!(),
                    };
                }
                (_, valid_pix) = self.next_step_from_dp(curr_pos);
            }
            (self.cursor.cx, self.cursor.cy) = curr_pos;
            return false;
        } else if self.map.blobs[ValidColor::from("⚪") as usize].contains(&next_pos) {
            self.cursor.last_color = ValidColor::from("⚪");
            let mut last_pos = next_pos;
            while self.map.blobs[ValidColor::from("⚪") as usize].contains(&next_pos) {
                last_pos = next_pos;
                (next_pos, _) = self.next_step_from_dp(next_pos);
            }
            (self.cursor.cx, self.cursor.cy) = last_pos;
            return false;
        } else {
            //     r_u, r_d, d_r, d_l, l_d, l_u, u_l, u_r, bs, c = all_blobs_indexed[pix_to_blob[next_pos[1]][next_pos[0]]]
            let abi = self.map.all_blobs_indexed[self.map.pix_to_blob[&next_pos]];

            if self.cursor.last_color != ValidColor::from("⚪") {
                let (lc_c, lc_r) = REV_MAP[self.cursor.last_color.into()];
                let (cc_c, cc_r) = REV_MAP[abi.c.into()];

                if let Some(cmd) = match ((cc_c + 6 - lc_c) % 6, (cc_r + 3 - lc_r) % 3) {
                    (0, 1) => Some(CMD::Push(self.cursor.last_bs as isize)),
                    (0, 2) => Some(CMD::Pop),
                    (1, 0) => Some(CMD::Add),
                    (1, 1) => Some(CMD::Sub),
                    (1, 2) => Some(CMD::Mul),
                    (2, 0) => Some(CMD::Div),
                    (2, 1) => Some(CMD::Mod),
                    (2, 2) => Some(CMD::Not),
                    (3, 0) => Some(CMD::Greater),
                    (3, 1) => {
                        if !(self.stack.len() >= 1) {
                            panic!()
                        }
                        let a = self.stack.pop().unwrap();
                        self.cursor.dp = (self.cursor.dp + (a.rem_euclid(4) as usize)) % 4;
                        None // Pointer
                    }
                    (3, 2) => {
                        if !(self.stack.len() >= 1) {
                            panic!()
                        }
                        let a = self.stack.pop().unwrap();
                        self.cursor.cc = (self.cursor.cc + (a.abs() as usize)) % 2;
                        None // Switch
                    }
                    (4, 0) => Some(CMD::Dup),
                    (4, 1) => Some(CMD::Roll),
                    (4, 2) => Some(CMD::InN),
                    (5, 0) => Some(CMD::InC),
                    (5, 1) => Some(CMD::OutN),
                    (5, 2) => Some(CMD::OutC),
                    _ => None,
                } {
                    cmd.interpret(&mut self.stack, input);
                }
            }

            self.cursor.last_color = abi.c;
            self.cursor.last_bs = abi.bs;

            (self.cursor.cx, self.cursor.cy) = match (self.cursor.dp, self.cursor.cc) {
                (0, 0) => abi.r_u,
                (0, 1) => abi.r_d,
                (1, 0) => abi.d_r,
                (1, 1) => abi.d_l,
                (2, 0) => abi.l_d,
                (2, 1) => abi.l_u,
                (3, 0) => abi.u_l,
                (3, 1) => abi.u_r,
                _ => panic!(),
            };
            return false;
        }
    }
}

fn interpret(filepath: String) {
    let img = open(filepath).unwrap().into_rgb8();
    let (w, h) = img.dimensions();

    let img_enum: Vec<ValidColor> = (0..h)
        .cartesian_product(0..w)
        .map(|(y, x)| {
            let r: ValidColor = match img[(x, y)] {
                Rgb([255, 192, 192]) => "❤",
                Rgb([255, 255, 192]) => "🧡",
                Rgb([192, 255, 192]) => "💛",
                Rgb([192, 255, 255]) => "💚",
                Rgb([192, 192, 255]) => "💙",
                Rgb([255, 192, 255]) => "💜",
                Rgb([255, 0, 0]) => "🔴",
                Rgb([255, 255, 0]) => "🟠",
                Rgb([0, 255, 0]) => "🟡",
                Rgb([0, 255, 255]) => "🟢",
                Rgb([0, 0, 255]) => "🔵",
                Rgb([255, 0, 255]) => "🟣",
                Rgb([192, 0, 0]) => "🟥",
                Rgb([192, 192, 0]) => "🟧",
                Rgb([0, 192, 0]) => "🟨",
                Rgb([0, 192, 192]) => "🟩",
                Rgb([0, 0, 192]) => "🟦",
                Rgb([192, 0, 192]) => "🟪",
                Rgb([0, 0, 0]) => "⚫",
                _ => "⚪", // Whitespace if not exact color
            }
            .into();
            r
        })
        .collect();

    let pxls = ArrayView::from_shape(Ix2(h as usize, w as usize), &img_enum).unwrap();

    let mut blobs: [HashSet<(usize, usize)>; 20] = core::array::from_fn(|_| HashSet::new());

    for ((y, x), data) in pxls.indexed_iter() {
        blobs[(*data) as usize].insert((x, y));
        if *data == "⚫".into() || *data == "⚪".into() {
            continue;
        }
    }

    // Watershed (image processing)
    let mut all_blobs: Vec<(ValidColor, HashSet<(usize, usize)>)> = Vec::new();
    for l in ALL_COLORS {
        if l == "⚫".into() || l == "⚪".into() {
            continue;
        }
        let mut checked_coord: HashSet<(usize, usize)> = HashSet::new();
        let mut seperate_blobs: Vec<HashSet<(usize, usize)>> = Vec::new();
        for (x, y) in blobs[l as usize].iter().cloned() {
            let mut sep_blob: HashSet<(usize, usize)> = HashSet::new();
            let mut queue: Vec<(usize, usize)> = vec![(x, y)];

            while queue.len() > 0 {
                let (xi, yi) = queue.pop().unwrap();
                if checked_coord.contains(&(xi, yi)) {
                    continue;
                }
                checked_coord.insert((xi, yi));
                sep_blob.insert((xi, yi));

                let mut possible = Vec::new();
                if xi + 1 < w as usize {
                    possible.push((xi + 1, yi))
                }
                if yi + 1 < h as usize {
                    possible.push((xi, yi + 1))
                }
                if yi > 0 {
                    possible.push((xi, yi - 1))
                }
                if xi > 0 {
                    possible.push((xi - 1, yi))
                }

                for (xn, yn) in possible {
                    if blobs[l as usize].contains(&(xn, yn)) {
                        queue.push((xn, yn));
                    }
                }
            }

            if sep_blob.len() > 0 {
                seperate_blobs.push(sep_blob);
            }
        }

        for x in seperate_blobs {
            all_blobs.push((l, x))
        }
    }

    // let mut pix_to_blob = Vec::new();
    let mut all_blobs_indexed: Vec<ABI> = Vec::new();
    let mut pix_to_blob: HashMap<(usize, usize), usize> = HashMap::new();
    for (i, (c, blob)) in all_blobs.iter().cloned().enumerate() {
        let l: Vec<_> = vec![
            (false, false, true),
            (false, false, false),
            (true, false, false),
            (true, true, false),
            (false, true, false),
            (false, true, true),
            (true, true, true),
            (true, false, true),
        ]
        .into_iter()
        .map(|(swap, rev1, rev2)| {
            *blob
                .iter()
                .max_by(|(a, b), (x, y)| {
                    let cmp1 = a.cmp(x);
                    let cmp2 = b.cmp(y);

                    let cmp1 = if rev1 { cmp1.reverse() } else { cmp1 };
                    let cmp2 = if rev2 { cmp2.reverse() } else { cmp2 };

                    if swap {
                        cmp1.then(cmp2)
                    } else {
                        cmp2.then(cmp1)
                    }
                })
                .unwrap()
        })
        .collect();
        let [r_u, r_d, d_r, d_l, l_d, l_u, u_l, u_r] = l[..] else {
            panic!()
        };

        let abi = ABI {
            r_u,
            r_d,
            d_r,
            d_l,
            l_d,
            l_u,
            u_l,
            u_r,
            bs: blob.len(),
            c,
        };

        all_blobs_indexed.push(abi);

        for (y, x) in blob {
            pix_to_blob.insert((y, x), i);
        }
    }

    let mut runner = PietExecution {
        cursor: PietCursor {
            cx: 0,
            cy: 0,
            dp: 0,
            cc: 0,
            last_color: ValidColor::from("⚪"),
            last_bs: 0,
        },
        map: &PietImageData {
            width: w as usize,
            height: h as usize,
            blobs,
            pix_to_blob,
            all_blobs_indexed,
        },
        stack: Vec::new(),
        //
    };

    // let mut total_steps = 0;
    let mut input = std::io::stdin().bytes().peekable();
    // runner
    if !runner.continue_step((0, 0), (0, 0), runner.check_valid_pixel((0, 0)), &mut input) {
        while !runner.step(&mut input) {
            // total_steps += 1;
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
}

fn main() {
    let args = Args::parse();
    interpret(args.filepath);
}
