/// Return the minimum of two values
#[inline]
pub(crate) fn min<T: PartialOrd>(v0: T, v1: T) -> T {
    if v0 < v1 {
        v0
    } else {
        v1
    }
}

/// Return the maximum of two values
#[inline]
pub(crate) fn max<T: PartialOrd>(v0: T, v1: T) -> T {
    if v0 > v1 {
        v0
    } else {
        v1
    }
}

#[derive(Default, Debug, Clone)]
pub(crate) struct Series3D(pub Vec<(f64, f64, f64)>);

impl Series3D {
    /// Get the x range of the series
    pub(crate) fn x_range(&self) -> std::ops::Range<f64> {
        if self.0.is_empty() {
            return f64::MIN..f64::MAX;
        }
        let min = self.0.iter().fold(self.0[0].0, |m, v| min(m, v.0));
        let max = self.0.iter().fold(self.0[0].0, |m, v| max(m, v.0));
        if min == max {
            return min..max + 1.0;
        }
        min..max
    }

    /// Get the y range of the series
    pub(crate) fn y_range(&self) -> std::ops::Range<f64> {
        if self.0.is_empty() {
            return f64::MIN..f64::MAX;
        }
        let min = self.0.iter().fold(self.0[0].1, |m, v| min(m, v.1));
        let max = self.0.iter().fold(self.0[0].1, |m, v| max(m, v.1));
        if min == max {
            return min..max + 1.0;
        }
        min..max
    }

    /// Get the z range of the series
    pub(crate) fn z_range(&self) -> std::ops::Range<f64> {
        if self.0.is_empty() {
            return f64::MIN..f64::MAX;
        }
        let min = self.0.iter().fold(self.0[0].2, |m, v| min(m, v.2));
        let max = self.0.iter().fold(self.0[0].2, |m, v| max(m, v.2));
        if min == max {
            return min..max + 1.0;
        }
        min..max
    }

    #[inline(always)]
    pub(crate) fn iter(&self) -> std::slice::Iter<(f64, f64, f64)> {
        self.0.iter()
    }

    #[inline(always)]
    pub(crate) fn inner(self) -> Vec<(f64, f64, f64)> {
        self.0
    }

    #[inline(always)]
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline(always)]
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}
