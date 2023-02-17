pub struct Board<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Board<T> {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    #[must_use]
    #[inline]
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[x + y * self.width]
    }

    #[must_use]
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.data[x + y * self.width]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.data[x + y * self.width] = val;
    }

    #[must_use]
    #[inline]
    pub fn get_width(&self) -> usize {
        self.width
    }

    #[must_use]
    #[inline]
    pub fn get_height(&self) -> usize {
        self.height
    }
}
