#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoolLerp<N> {
    value: N,
}

impl<N: From<u8>> BoolLerp<N> {
    pub fn truthy() -> Self {
        BoolLerp { value: N::from(1) }
    }

    pub fn falsey() -> Self {
        BoolLerp { value: N::from(0) }
    }
}

impl<N> BoolLerp<N>
where
    N: PartialOrd + From<u8>,
{
    pub fn new(n: N) -> Self {
        debug_assert!(
            n >= N::from(0) && n <= N::from(1),
            "intermediate value must be between 0 and 1"
        );

        BoolLerp { value: n }
    }
}

impl<N: Copy> BoolLerp<N> {
    pub fn value(&self) -> N {
        self.value
    }
}

impl<N: PartialOrd + From<u8>> Into<BoolLerp<N>> for bool {
    fn into(self) -> BoolLerp<N> {
        BoolLerp::new(N::from(self as u8))
    }
}
