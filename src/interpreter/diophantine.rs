use super::{gcd::GCD, sign::SignedInt};

pub struct Diophantine;

impl Diophantine {
    // ax + by = c
    // step_size * x - 256 * y = -start_cell_value
    // We only need to find x, as it stands for number of iterations inside the loop
    pub fn solve_extended_gcd(a: isize, b: isize, c: isize) -> u8 {
        let (gcd, xg, yg) = GCD::extended_euclidean(a, b);

        if c % gcd != 0 {
            panic!("Solution for Diophantine equation isn't exist!");
        } else {
            let x0 = xg * c / gcd;
            let y0 = yg * c / gcd;

            // x = x0 + k * (b / g)
            // y = y0 - k * (a / g)
            // Find such k, that y > 0
            let k = (y0 as f32 * gcd as f32 / a as f32).floor() as isize;

            // As x represents number of iterations in chosen loop,
            // we don't care if it's negative or overflowing u8,
            // because it will also overflow other cells:
            //
            // [number_of_iterations * depth_multiplicand]
            //
            // Let's take number_of_iterations = 322
            // and depth_multiplicand = 5:
            //
            // if number_of_iterations > 255:
            //     322 * 5 = 1610 > 74
            // if number_of_iterations < 255:
            //     66 * 5 = 330 > 74
            //    /  \
            // (322 - 256)
            (x0 + k * b / gcd).wrapping_abs() as u8
        }
    }

    pub fn solve_naive(start_cell_value: u8, zero_depth_multiplicand: SignedInt<u8>) -> u8 {
        let mut counter: u8 = 0;
        let mut actual_cell_value = start_cell_value;

        match zero_depth_multiplicand {
            SignedInt::Pos(zdm) => {
                while actual_cell_value != 0 {
                    actual_cell_value = actual_cell_value.wrapping_add(zdm);
                    counter = counter.wrapping_add(1);
                }
            },
            SignedInt::Neg(zdm) => {
                while actual_cell_value != 0 {
                    actual_cell_value = actual_cell_value.wrapping_sub(zdm);
                    counter = counter.wrapping_add(1);
                }
            },
        }

        counter
    }
}
