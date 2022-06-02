use std::cmp::min;

pub struct GCD;

impl GCD {
    pub fn extended_euclidean(a: isize, b: isize) -> (isize, isize, isize) {
        if a == 0 {
            return (b, 0, 1)
        }
        
        let remainder = b % a;
    
        let (gcd, x1, y1) = GCD::extended_euclidean(remainder, a);
    
        let xg = y1 - (b / a) * x1;
        let yg = x1;
    
        (gcd, xg, yg)
    }

    pub fn extended_steins(mut a: isize, mut b: isize) -> (isize, isize, isize) {
        // GCD(2^i * a, 2^j * b) = 2^k * GCD(a, b), where k = min(i, j)
        let i = a.trailing_zeros();
        let j = b.trailing_zeros();
        let k = min(i, j);

        a = a.abs() >> k;
        b = b.abs() >> k;
        let (mut sx, mut sy, mut tx, mut ty) = (1, 0, 0, 1);

        while a != b {
            if a % 2 == 0 {
                a /= 2;

                if ((sx % 2) == 0) && ((sy % 2) == 0) {
                    sx /= 2;
                    tx /= 2;
                } else {
                    sx = (sx + b) / 2;
                    tx = (tx - a) / 2;
                }
            }
            else {
                if a > b {
                    a = a - b;
                    sx = sx - sy;
                    tx = tx - ty;
                } else {
                    b = b - a;
                    sy = sy - sx;
                    ty = ty - tx;
                }
            }
        }

        (a << k, sx, tx)
    }
}
