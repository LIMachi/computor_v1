pub fn gcd(mut v1: i32, mut v2: i32) -> i32 {
    if v1 == 0 || v2 == 0 {
        0 //technically an error
    } else {
        if v1 < 0 {
            v1 = -v1;
        }
        if v2 < 0 {
            v2 = -v2;
        }
        if v1 > v2 {
            let t = v1;
            v1 = v2;
            v2 = t;
        }
        while v1 != 0 {
            let t = v1;
            v1 = v2 % v1;
            v2 = t;
        }
        v2
    }
}