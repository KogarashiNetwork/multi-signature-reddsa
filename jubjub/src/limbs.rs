#[inline(always)]
pub(crate) const fn mul(a: [u64; 4], b: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    let s = a[0] as u128 * b[0] as u128;
    let (l0, c) = (s as u64, (s >> 64) as u64);
    let s = a[0] as u128 * b[1] as u128 + c as u128;
    let (l1, c) = (s as u64, (s >> 64) as u64);
    let s = a[0] as u128 * b[2] as u128 + c as u128;
    let (l2, c) = (s as u64, (s >> 64) as u64);
    let s = a[0] as u128 * b[3] as u128 + c as u128;
    let (l3, l4) = (s as u64, (s >> 64) as u64);

    let s = a[1] as u128 * b[0] as u128 + l1 as u128;
    let (l1, c) = (s as u64, (s >> 64) as u64);
    let s = l2 as u128 + a[1] as u128 * b[1] as u128 + c as u128;
    let (l2, c) = (s as u64, (s >> 64) as u64);
    let s = l3 as u128 + a[1] as u128 * b[2] as u128 + c as u128;
    let (l3, c) = (s as u64, (s >> 64) as u64);
    let s = l4 as u128 + a[1] as u128 * b[3] as u128 + c as u128;
    let (l4, l5) = (s as u64, (s >> 64) as u64);

    let s = a[2] as u128 * b[0] as u128 + l2 as u128;
    let (l2, c) = (s as u64, (s >> 64) as u64);
    let s = l3 as u128 + a[2] as u128 * b[1] as u128 + c as u128;
    let (l3, c) = (s as u64, (s >> 64) as u64);
    let s = l4 as u128 + a[2] as u128 * b[2] as u128 + c as u128;
    let (l4, c) = (s as u64, (s >> 64) as u64);
    let s = l5 as u128 + a[2] as u128 * b[3] as u128 + c as u128;
    let (l5, l6) = (s as u64, (s >> 64) as u64);

    let s = a[3] as u128 * b[0] as u128 + l3 as u128;
    let (l3, c) = (s as u64, (s >> 64) as u64);
    let s = l4 as u128 + a[3] as u128 * b[1] as u128 + c as u128;
    let (l4, c) = (s as u64, (s >> 64) as u64);
    let s = l5 as u128 + a[3] as u128 * b[2] as u128 + c as u128;
    let (l5, c) = (s as u64, (s >> 64) as u64);
    let s = l6 as u128 + a[3] as u128 * b[3] as u128 + c as u128;
    let (l6, l7) = (s as u64, (s >> 64) as u64);

    mont([l0, l1, l2, l3, l4, l5, l6, l7], p, inv)
}

#[inline(always)]
pub(crate) const fn mont(a: [u64; 8], p: [u64; 4], inv: u64) -> [u64; 4] {
    let rhs = a[0].wrapping_mul(inv);
    let s = rhs as u128 * p[0] as u128 + a[0] as u128;
    let d = (s >> 64) as u64;
    let s = a[1] as u128 + rhs as u128 * p[1] as u128 + d as u128;
    let (l1, d) = (s as u64, (s >> 64) as u64);
    let s = a[2] as u128 + rhs as u128 * p[2] as u128 + d as u128;
    let (l2, d) = (s as u64, (s >> 64) as u64);
    let s = a[3] as u128 + rhs as u128 * p[3] as u128 + d as u128;
    let (l3, d) = (s as u64, (s >> 64) as u64);
    let s = a[4] as u128 + d as u128;
    let (l4, e) = (s as u64, (s >> 64) as u64);

    let rhs = l1.wrapping_mul(inv);
    let s = rhs as u128 * p[0] as u128 + l1 as u128;
    let d = (s >> 64) as u64;
    let s = l2 as u128 + rhs as u128 * p[1] as u128 + d as u128;
    let (l2, d) = (s as u64, (s >> 64) as u64);
    let s = l3 as u128 + rhs as u128 * p[2] as u128 + d as u128;
    let (l3, d) = (s as u64, (s >> 64) as u64);
    let s = l4 as u128 + rhs as u128 * p[3] as u128 + d as u128;
    let (l4, d) = (s as u64, (s >> 64) as u64);
    let s = a[5] as u128 + e as u128 + d as u128;
    let (l5, e) = (s as u64, (s >> 64) as u64);

    let rhs = l2.wrapping_mul(inv);
    let s = rhs as u128 * p[0] as u128 + l2 as u128;
    let d = (s >> 64) as u64;
    let s = l3 as u128 + rhs as u128 * p[1] as u128 + d as u128;
    let (l3, d) = (s as u64, (s >> 64) as u64);
    let s = l4 as u128 + rhs as u128 * p[2] as u128 + d as u128;
    let (l4, d) = (s as u64, (s >> 64) as u64);
    let s = l5 as u128 + rhs as u128 * p[3] as u128 + d as u128;
    let (l5, d) = (s as u64, (s >> 64) as u64);
    let s = a[6] as u128 + e as u128 + d as u128;
    let (l6, e) = (s as u64, (s >> 64) as u64);

    let rhs = l3.wrapping_mul(inv);
    let s = rhs as u128 * p[0] as u128 + l3 as u128;
    let d = (s >> 64) as u64;
    let s = l4 as u128 + rhs as u128 * p[1] as u128 + d as u128;
    let (l4, d) = (s as u64, (s >> 64) as u64);
    let s = l5 as u128 + rhs as u128 * p[2] as u128 + d as u128;
    let (l5, d) = (s as u64, (s >> 64) as u64);
    let s = l6 as u128 + rhs as u128 * p[3] as u128 + d as u128;
    let (l6, d) = (s as u64, (s >> 64) as u64);
    let l7 = a[7].wrapping_add(e).wrapping_add(d);

    let s = (l4 as u128).wrapping_sub(p[0] as u128);
    let (l0, brw) = (s as u64, (s >> 64) as u64);
    let s = (l5 as u128).wrapping_sub(p[1] as u128 + (brw >> 63) as u128);
    let (l1, brw) = (s as u64, (s >> 64) as u64);
    let s = (l6 as u128).wrapping_sub(p[2] as u128 + (brw >> 63) as u128);
    let (l2, brw) = (s as u64, (s >> 64) as u64);
    let s = (l7 as u128).wrapping_sub(p[3] as u128 + (brw >> 63) as u128);
    let (l3, brw) = (s as u64, (s >> 64) as u64);

    let s = l0 as u128 + (p[0] & brw) as u128;
    let (l0, c) = (s as u64, (s >> 64) as u64);
    let s = l1 as u128 + (p[1] & brw) as u128 + c as u128;
    let (l1, c) = (s as u64, (s >> 64) as u64);
    let s = l2 as u128 + (p[2] & brw) as u128 + c as u128;
    let (l2, c) = (s as u64, (s >> 64) as u64);
    let l3 = l3.wrapping_add(p[3] & brw).wrapping_add(c);

    [l0, l1, l2, l3]
}