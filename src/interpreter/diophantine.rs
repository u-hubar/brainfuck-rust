use super::sign::SignedInt;

pub struct Diophantine;

impl Diophantine {
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
