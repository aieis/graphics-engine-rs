pub struct Arr<T: Clone, const N: usize> {
    data: [T; N],
    count: usize,
}

impl<T: Clone, const N: usize> Arr<T,N> {
    const CAPACITY: usize = N;

    pub fn new() -> Self {
        Self {
            data: unsafe { std::mem::MaybeUninit::<[T; N]>::uninit().assume_init() },
            count: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn push(&mut self, v: T) {
        self.data[self.count] = v;
        self.count += 1;
    }

    pub fn pop(&mut self, i: usize) {
        while i < self.count - 1 {
            self.data[i] = self.data[i+1];
        }
        self.count -= 1;
    }

    pub fn pop_swap(&mut self, i: usize) {
        // self.data
    }
}
