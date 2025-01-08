use super::Expr::{self, *};
use crate::optimize_stk::StackOptimizer;
use crate::piet_color::*;
use crate::piet_interpreter::*;
use image::Rgb;
use image::RgbImage;
use ndarray::Array;
use ndarray::Ix2;
use num::ToPrimitive;
use num::*;
use std::cmp;
use std::collections::HashMap;

impl super::PietStackExecutor {
    fn index_and_command_to_color_and_next_index(cmd: CMD, output: &mut Vec<String>) {
        let (mut previous_c, mut previous_r): (usize, usize) = REV_MAP[&output[output.len() - 1]];
        match cmd {
            CMD::Nop => {
                output.push(String::from("⚪"));
                output.push(String::from("🔴"));
                return;
            }
            CMD::Push(i) => {
                let x: BigInt = i - Into::<BigInt>::into(1);
                for _ in 0..(x.to_usize().unwrap()) {
                    output.push(String::from(COLORS[previous_r][previous_c]));
                }
                previous_r = (previous_r + 1) % 3
            }
            CMD::Pop => {
                previous_r = (previous_r + 2) % 3;
            }
            CMD::Add => {
                previous_c = (previous_c + 1) % 6;
            }
            CMD::Sub => {
                previous_c = (previous_c + 1) % 6;
                previous_r = (previous_r + 1) % 3;
            }
            CMD::Mul => {
                previous_c = (previous_c + 1) % 6;
                previous_r = (previous_r + 2) % 3;
            }
            CMD::Div => {
                previous_c = (previous_c + 2) % 6;
            }
            CMD::Mod => {
                previous_c = (previous_c + 2) % 6;
                previous_r = (previous_r + 1) % 3;
            }
            CMD::Not => {
                previous_c = (previous_c + 2) % 6;
                previous_r = (previous_r + 2) % 3;
            }
            CMD::Greater => {
                previous_c = (previous_c + 3) % 6;
            }
            CMD::Pointer => {
                previous_c = (previous_c + 3) % 6;
                previous_r = (previous_r + 1) % 3;
            }
            CMD::Switch => {
                previous_c = (previous_c + 3) % 6;
                previous_r = (previous_r + 2) % 3;
            }
            CMD::Dup => {
                previous_c = (previous_c + 4) % 6;
            }
            CMD::Roll => {
                previous_c = (previous_c + 4) % 6;
                previous_r = (previous_r + 1) % 3;
            }
            CMD::InN => {
                previous_c = (previous_c + 4) % 6;
                previous_r = (previous_r + 2) % 3;
            }
            CMD::InC => {
                previous_c = (previous_c + 5) % 6;
            }
            CMD::OutN => {
                previous_c = (previous_c + 5) % 6;
                previous_r = (previous_r + 1) % 3;
            }
            CMD::OutC => {
                previous_c = (previous_c + 5) % 6;
                previous_r = (previous_r + 2) % 3;
            }
        }
        output.push(String::from(COLORS[previous_r][previous_c]))
    }

    fn make_block(block: Vec<Expr>) -> (Vec<String>, (Option<String>, Option<String>, bool)) {
        let mut output = Vec::new();
        output.push(String::from("🔴"));
        for inp in block {
            match inp {
                Branch(t, e) => return (output, (Some(t), Some(e), false)),
                Goto(l) => return (output, (Some(l), None, false)),
                // Term =>
                //     return (output, (Some("term"), None)),
                Instr(cmd) => {
                    Self::index_and_command_to_color_and_next_index(cmd, &mut output);
                }
                Debug => {}
                Comment(_) => {}
                GotoStk => {
                    return (output, (None, None, true))
                    // output.push(String::from("⚫"));
                    // return (output, (Some(String::from("term")), None));
                    // NOP?
                    // todo!("goto_stk")
                }
            }
        }
        return (output, (None, None, false));
    }

    fn split_in_blocks(
        to_split: Vec<String>,
        j_width: usize,
        going_right: bool,
    ) -> (Vec<Vec<String>>, bool) {
        let mut block_blocks = vec![];
        let mut offset = 0;
        let mut running = true;
        let mut going_right = !going_right;
        while running {
            let last_offset = offset;
            let mut broke = false;
            for j in 0..j_width {
                if offset + j >= to_split.len() {
                    offset = to_split.len();
                    running = false;
                    broke = true;
                    break;
                }
                // TODO: What does this do?
                if offset + j_width < to_split.len() {
                    let mut typ_of_elems = vec![];
                    for i in to_split[offset + j..offset + j_width].into_iter() {
                        let mut do_break = true;
                        for ind in typ_of_elems.clone() {
                            if i == ind {
                                do_break = false;
                                break;
                            }
                        }
                        if do_break {
                            typ_of_elems.push(i);
                        }
                    }
                    if typ_of_elems.len() == 2 {
                        if j == 0 {
                            panic!("j was zero");
                        }
                        offset += j;
                        broke = true;
                        break;
                    }
                }
            }
            if !broke {
                offset += j_width
            }
            block_blocks.push(
                to_split
                    [last_offset..cmp::min(offset + (if running { 1 } else { 0 }), to_split.len())]
                    .to_vec(),
            );
            going_right = !going_right;
        }
        return (block_blocks, going_right);
    }

    fn goto_block_coord(optimizer: &mut StackOptimizer, id: usize) -> Vec<Expr> {
        let mut goto_exprs: Vec<Expr> = vec![];
        goto_exprs.extend(
            optimizer
                .optimize_number(id.into())
                .into_iter()
                .map(Instr)
                .collect::<Vec<Expr>>(),
        );
        goto_exprs
    }

    pub fn to_png(&self, optimizer: &mut StackOptimizer) -> image::RgbImage {
        let (parsed_blocks, block_index) = (self.blocks.clone(), self.block_index.clone());

        let pbl: usize = parsed_blocks.len() + 1;
        let b_width = (pbl as f32).sqrt().ceil() as usize; // (pbl).isqrt();
        let b_height = (pbl - 1) / b_width + 1;

        // println!("Blocks");

        let mut blocks: HashMap<String, (Vec<String>, (Option<String>, Option<String>, bool))> =
            HashMap::new();

        for (k, b) in parsed_blocks {
            // println!("Blocks: {}, {:?}", k, b);

            let mut nb = vec![];
            for x in b {
                // println!("      : -> {:?}", x);
                match x {
                    Instr(CMD::Push(n)) if n == 0.into() => {
                        nb.push(Instr(CMD::Push(1.into())));
                        nb.push(Instr(CMD::Not));
                    }
                    Instr(CMD::Push(n)) => {
                        if n < 0.into() {
                            nb.push(Instr(CMD::Push(1.into())));
                            nb.push(Instr(CMD::Not));
                            nb.extend(
                                optimizer
                                    .optimize_number(-n)
                                    .into_iter()
                                    .map(Instr)
                                    .collect::<Vec<Expr>>(),
                            );
                            nb.push(Instr(CMD::Sub));
                        } else {
                            nb.extend(
                                optimizer
                                    .optimize_number(n)
                                    .into_iter()
                                    .map(Instr)
                                    .collect::<Vec<Expr>>(),
                            );
                        }
                    }
                    _ => nb.push(x),
                }
            }

            blocks.insert(String::from(k), Self::make_block(nb));
        }

        // println!("Parsed blocks");

        let mut mid_blocks: HashMap<
            String,
            (Vec<String>, (Option<Vec<String>>, Option<Vec<String>>)),
        > = HashMap::new();

        // println!("Mid blocks");

        for (x, (x_vec, (x_t, x_e, x_g))) in blocks {
            if x_g {
                mid_blocks.insert(x, (x_vec.clone(), (Some(vec![]), None)));
            } else if x_t.is_some() && x_e.is_none() {
                if !block_index.contains_key(&x_t.clone().unwrap()) {
                    panic!("no block with name: {}", &x_t.unwrap());
                }

                // is goto
                let goto_statement = Self::make_block(Self::goto_block_coord(
                    optimizer,
                    block_index[&x_t.unwrap()],
                ))
                .0;

                mid_blocks.insert(x, (x_vec.clone(), (Some(goto_statement), None)));
                // TODO: make goto use color, instead of white!;
            } else if x_t.is_some() && x_e.is_some() {
                let mut goto_exprs_1: Vec<Expr> = vec![];
                goto_exprs_1.push(Instr(CMD::Pointer));
                goto_exprs_1.push(Instr(CMD::Nop));
                goto_exprs_1.extend(Self::goto_block_coord(
                    optimizer,
                    block_index[&x_t.unwrap()],
                ));

                // output.push("🔴")
                let goto_statement_1: Vec<_> = Self::make_block(goto_exprs_1).0;

                let mut goto_exprs_2: Vec<Expr> = vec![];
                goto_exprs_2.push(Instr(CMD::Pointer));
                goto_exprs_2.push(Instr(CMD::Nop));
                goto_exprs_2.extend(Self::goto_block_coord(
                    optimizer,
                    block_index[&x_e.unwrap()],
                ));

                let goto_statement_2: Vec<_> = Self::make_block(goto_exprs_2).0;

                mid_blocks.insert(
                    x,
                    (
                        x_vec.clone(),
                        (Some(goto_statement_1), Some(goto_statement_2)),
                    ),
                );
            } else {
                mid_blocks.insert(x, (x_vec, (None, None)));
            }
        }

        // println!("J width");

        // // # Split blocks
        let j_width = cmp::max(70, cmp::max(b_width, b_height) / 5);
        // let j_width = cmp::max(30, cmp::max(b_width, b_height) / 5);

        let mut final_blocks: HashMap<String, (Vec<Vec<String>>, Vec<String>)> = HashMap::new();

        // println!("Final blocks");

        for (x, (bx, (bx_t, bx_e))) in mid_blocks {
            let (mut splits, going_right) = Self::split_in_blocks(bx, j_width, true);
            let mut branch_blocks = vec![];
            if bx_t.is_some() && bx_e.is_none() {
                let (goto_splits, _) = Self::split_in_blocks(bx_t.unwrap(), j_width, true);
                splits.extend(goto_splits);
            } else if bx_t.is_some() && bx_e.is_some() {
                if going_right {
                    splits.push(vec![]);
                }
                let (goto_splits, _) = Self::split_in_blocks(bx_e.unwrap(), j_width, true);
                splits.extend(goto_splits);
                branch_blocks = bx_t.unwrap();
            }
            final_blocks.insert(x, (splits, branch_blocks));
        }

        // println!("Draw?");

        Self::to_png_draw(
            optimizer,
            b_width,
            b_height,
            final_blocks,
            block_index,
            j_width,
        )
    }

    // (Vec<&'a str>, (Option<&'a str>, Option<&'a str>))

    pub fn to_png_draw(
        optimizer: &mut StackOptimizer,
        b_width: usize,
        b_height: usize,
        final_blocks: HashMap<String, (Vec<Vec<String>>, Vec<String>)>,
        block_index: HashMap<String, usize>,
        j_width: usize,
    ) -> image::RgbImage {
        let pre = ["⚪", "⚪", "⚪"];
        let post = ["⚪"];

        let total_block_width = (j_width + 4 + 1) + 6;
        let height_per_block = final_blocks
            .clone()
            .into_iter()
            .map(|(_, (bx, bx_branch))| bx.len() * 5 / 2 + bx_branch.len())
            .max()
            .unwrap();

        let total_block_height = height_per_block + 7 + 25;

        let w = pre.len() + total_block_width * b_width + post.len();
        let h = 2 + 1 + total_block_height * b_height + 2;

        let mut arr: Array<String, Ix2> = Array::default((w, h));
        for y in 0..h {
            for x in 0..w {
                arr[(x, y)] = String::from("⚪");
            }
        }

        fn set_range(arr: &mut Array<String, Ix2>, h: usize, i: usize, j: usize, c: String) {
            for x in i..(i + j) {
                arr[(x, h)] = c.clone();
            }
        }

        set_range(
            &mut arr,
            1,
            0,
            pre.len() + total_block_width * b_width,
            String::from("⚫"),
        );

        for i in 0..b_height {
            set_range(
                &mut arr,
                1 + (1 + i) * total_block_height,
                pre.len(),
                total_block_width * b_width,
                String::from("⚫"),
            );
            set_range(
                &mut arr,
                3 + i * total_block_height,
                pre.len() - 1,
                1 + total_block_width * b_width,
                String::from("⚫"),
            );
            set_range(
                &mut arr,
                2 + (1 + i) * total_block_height,
                pre.len() - 2,
                1,
                String::from("⚫"),
            );
        }

        // Goto converter to a pair of numbers
        let converter_x_offset = 8;
        for (i, v) in Self::make_block(vec![
            Instr(CMD::Dup),
            Instr(CMD::Push(b_width.into())),
            Instr(CMD::Dup),
            Instr(CMD::Push(3.into())),
            Instr(CMD::Push(1.into())),
            Instr(CMD::Roll),
            Instr(CMD::Mod),
            Instr(CMD::Push(3.into())),
            Instr(CMD::Push(1.into())),
            Instr(CMD::Roll),
            Instr(CMD::Div),
        ])
        .0
        .into_iter()
        .enumerate()
        {
            arr[(converter_x_offset + i, 0)] = v.clone();
            arr[(converter_x_offset + i, 2)] = v;
        }
        // arr[(4, 2)] = String::from("❤");

        for (i, branch_instr) in
            Self::make_block(Self::goto_block_coord(optimizer, block_index["main"]))
                .0
                .into_iter()
                .enumerate()
        {
            arr[(i, 0)] = branch_instr;
        }

        let prepare_pointer = vec![
            Instr(CMD::Dup),
            Instr(CMD::Push(1.into())),
            Instr(CMD::Not),
            Instr(CMD::Greater),
            Instr(CMD::Not),
            Instr(CMD::Pointer),
            Instr(CMD::Push(1.into())),
            Instr(CMD::Sub),
        ];
        let prepare_pointer_index = 5 + 1; // prepare_pointer.index("pointer")+1

        let prepare_pointer_block = Self::make_block(prepare_pointer.clone()).0;
        let mut temp: Vec<_> = prepare_pointer[0..prepare_pointer_index].to_vec();
        temp.push(Instr(CMD::Pop));
        let prepare_pointer_pop = Self::make_block(temp).0;
        let prepare_pointer_pop =
            &prepare_pointer_pop[prepare_pointer_index + 1..prepare_pointer_pop.len()];

        for li in 0..b_height {
            for (j, branch_instr) in prepare_pointer_block.clone().into_iter().enumerate() {
                arr[(
                    pre.len() + total_block_width * b_width,
                    1 + total_block_height * li + total_block_height - prepare_pointer_index
                        + j
                        + 1,
                )] = branch_instr;
            }
            for (j, y) in prepare_pointer_pop.into_iter().enumerate() {
                arr[(
                    pre.len() + total_block_width * b_width - 1 - j,
                    1 + total_block_height * li + total_block_height + 1,
                )] = String::from(y.clone());
            }
        }

        fn block_line_left(arr: &mut Array<String, Ix2>, yi: usize, xi: usize, gap: usize) {
            // gap = 2
            arr[(xi - 3, yi)] = String::from("⚫");
            arr[(xi - 2, yi - 2)] = String::from("⚫");
            arr[(xi - 1, yi - 1)] = String::from("⚫");
            arr[(xi - 2, yi + gap + 2)] = String::from("⚫");
            arr[(xi - 4, yi + gap + 1)] = String::from("⚫");
            arr[(xi - 3, yi + gap)] = String::from("⚫");
        }

        fn block_line_right(arr: &mut Array<String, Ix2>, yi: usize, xi: usize, gap: usize) {
            // gap = 2
            arr[(xi + 1, yi)] = String::from("⚫");
            arr[(xi, yi + gap + 2)] = String::from("⚫");
        }

        // Draw all the code blocks
        for (x, (bx, bx_branch)) in final_blocks {
            let si = block_index[&x];

            let start_index = 7 + pre.len() + total_block_width * (si % b_width);
            let mut li = 7 + total_block_height * (si / b_width); // line index

            for (j, y) in prepare_pointer_block.clone().into_iter().enumerate() {
                arr[(start_index - prepare_pointer_index - 1 + j, li - 3)] =
                    String::from(y.clone());
            }
            for (j, y) in prepare_pointer_pop.into_iter().enumerate() {
                arr[(start_index - 1, li - 3 + 1 + j)] = String::from(y.clone());
            }

            if x == "term" {
                let (heart_x, heart_y) = (li + 1, start_index - 1);

                arr[(heart_y - 1, heart_x)] = String::from("🔴");
                arr[(heart_y, heart_x)] = String::from("🔴");
                arr[(heart_y + 1, heart_x)] = String::from("🔴");
                arr[(heart_y, heart_x + 1)] = String::from("🔴");

                arr[(heart_y - 1, heart_x - 1)] = String::from("⚫");
                arr[(heart_y + 1, heart_x - 1)] = String::from("⚫");
                arr[(heart_y - 2, heart_x)] = String::from("⚫");
                arr[(heart_y + 2, heart_x)] = String::from("⚫");
                arr[(heart_y - 1, heart_x + 1)] = String::from("⚫");
                arr[(heart_y + 1, heart_x + 1)] = String::from("⚫");
                arr[(heart_y, heart_x + 2)] = String::from("⚫");

                continue;
            }

            arr[(start_index - 1, li + 1)] = String::from("⚫");
            arr[(start_index - 3, li)] = String::from("⚫");
            arr[(start_index - 2, li - 1)] = String::from("⚫");

            let right_line_gap = 2;
            let left_line_gap = 1;

            let mut going_right = true;
            for block_block in bx {
                if going_right {
                    for (j, y) in block_block.into_iter().enumerate() {
                        arr[(start_index + j, li)] = String::from(y.clone());
                    }
                    block_line_right(&mut arr, li, start_index + j_width, right_line_gap);
                    li = li + right_line_gap + 1;
                } else {
                    // going left
                    for (j, y) in block_block.into_iter().enumerate() {
                        arr[(j_width + start_index - 1 - j, li)] = String::from(y.clone());
                    }
                    block_line_left(&mut arr, li, start_index, left_line_gap);
                    li = li + left_line_gap + 1;
                }
                going_right = !going_right;
            }

            if bx_branch.len() > 0 {
                for (j, y) in bx_branch[2..].into_iter().enumerate() {
                    arr[(start_index + 1, li - right_line_gap + j)] = String::from(y.clone());
                }
            }

            if !going_right {
                arr[(start_index + j_width, li + 1)] = String::from("⚪"); // 🔴
            } else {
                arr[(start_index - 2, li + 1)] = String::from("⚪"); // 🔴
            }
        }

        let mut new_h = 1;
        let mut remove_indexes = vec![];
        for y in 0..h - 1 {
            let mut success = true;
            for x in 0..w {
                if arr[(x, y)] != String::from("⚪") || arr[(x, y + 1)] != String::from("⚪") {
                    success = false;
                    break;
                }
            }
            if success {
                remove_indexes.push(y);
            } else {
                new_h += 1;
            }
        }

        let mut fin_arr: Array<String, Ix2> = Array::default((w, new_h));
        let mut h_ = 0;
        for y in 0..h {
            if remove_indexes.contains(&y) {
                h_ += 1;
                continue;
            }
            for x in 0..w {
                fin_arr[(x, y - h_)] = arr[(x, y)].clone();
            }
        }

        let mut img = RgbImage::new(w as u32, new_h as u32);

        for y in 0..new_h {
            for x in 0..w {
                let c: ValidColor = fin_arr[(x, y)].as_str().into();
                let r: Rgb<u8> = c.into();
                img[(x as u32, y as u32)] = r;
            }
        }

        return img;
    }
}
