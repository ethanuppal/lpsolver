use std::hash::{Hash, Hasher};

const MANTISSA_BITS: usize = 52;

#[derive(PartialEq, Eq, Hash)]
struct Float64IEEE754 {
    mantissa: u64,
    exponent: i16,
    sign: i8,
}

impl From<f64> for Float64IEEE754 {
    // https://stackoverflow.com/a/39639200
    fn from(value: f64) -> Self {
        let bits = value.to_bits();
        let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
        let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
        let mantissa = if exponent == 0 {
            (bits & 0xfffffffffffff) << 1
        } else {
            (bits & 0xfffffffffffff) | 0x10000000000000
        };
        exponent -= 1023 + 52;
        Self {
            mantissa,
            exponent,
            sign,
        }
    }
}

impl PartialOrd for Float64IEEE754 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Float64IEEE754 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.sign != other.sign {
            // since sign == 1 means negative
            return self.sign.cmp(&-other.sign);
        }

        let order = self
            .exponent
            .cmp(&other.exponent)
            .then_with(|| self.mantissa.cmp(&other.mantissa));

        if self.sign == 0 {
            order
        } else {
            order.reverse()
        }
    }
}

/// A real number with a specified maximum binary precision.
#[derive(Debug, Clone, Copy)]
pub struct Real<const PRECISION: usize>(pub f64);

impl<const PRECISION: usize> Real<PRECISION> {
    fn canonicalize(&self) -> Float64IEEE754 {
        let Self(value) = self;
        let mut encoding = Float64IEEE754::from(*value);
        encoding.mantissa &= (1u64 << (MANTISSA_BITS - PRECISION)) - 1;
        encoding
    }
}

impl<const PRECISION: usize> PartialEq for Real<PRECISION> {
    fn eq(&self, other: &Self) -> bool {
        self.canonicalize() == other.canonicalize()
    }
}

impl<const PRECISION: usize> Eq for Real<PRECISION> {}

impl<const PRECISION: usize> Hash for Real<PRECISION> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonicalize().hash(state)
    }
}

impl<const PRECISION: usize> PartialOrd for Real<PRECISION> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const PRECISION: usize> Ord for Real<PRECISION> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.canonicalize().cmp(&other.canonicalize())
    }
}

// impl<const PRECISION: usize> From<f64> for Real<PRECISION> {
//     fn from(value: f64) -> Self {
//         Self(value)
//     }
// }

impl<const PRECISION: usize> From<Real<PRECISION>> for f64 {
    fn from(value: Real<PRECISION>) -> Self {
        let Real(value) = value;
        value
    }
}
