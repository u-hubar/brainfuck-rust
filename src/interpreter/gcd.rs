pub struct GCD;

impl GCD {
    pub fn find_extended(a: isize, b: isize) -> (isize, isize, isize) {
        if a == 0 {
            return (b, 0, 1)
        }
        
        let remainder = b % a;
    
        let (gcd, x1, y1) = GCD::find_extended(remainder, a);
    
        let xg = y1 - (b / a) * x1;
        let yg = x1;
    
        (gcd, xg, yg)
    }
}
