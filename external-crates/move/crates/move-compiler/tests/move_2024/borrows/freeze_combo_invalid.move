module 0x8675309::M {
    public struct S has copy, drop { f: u64, g: u64 }
    fun id<T>(r: &T): &T {
        r
    }
    fun id_mut<T>(r: &mut T): &mut T {
        r
    }

    fun t0(cond: bool, s: &mut S, other: &mut S) {
        let mut f;
        if (cond) f = &mut s.f else f = &mut other.f;
        freeze(s);
        *f;
    }

    fun t1(cond: bool, s: &mut S) {
        let mut f;
        if (cond) f = &mut s.f else f = &mut s.g;
        freeze(s);
        *f;
    }

    fun t2(cond: bool, s: &mut S, other: &mut S) {
        let mut x;
        if (cond) x = s else x = other;
        freeze(s);
        *x;
    }

}
