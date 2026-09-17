pub struct Arr<T: Copy, const N: usize> {
    data: [T; N],
    count: usize,
}


/// This struct is used to keep a small amount of values on the stack
/// Of important note, T should not need any drop logic;

impl<T: Copy, const N: usize> Arr<T,N> {
    const CAPACITY: usize = N;

    pub fn new() -> Self {
        const {
            assert! (
                !std::mem::needs_drop::<T>(),
                "Type T needs to be trivially 'removable'. I.e. unsafely forgotten."
            );
        }

        Self {
            data: unsafe { std::mem::MaybeUninit::<[T; N]>::uninit().assume_init() },
            count: 0,
        }
    }


    pub fn of(v: T, count: usize) -> Self {
        assert!(count <= Self::CAPACITY, "Number of items must not be greater than capacity of the array.");

        let mut arr = Self::new();
        let mut i = 0;
        while i < count {
            arr.push(v);
            i += 1;
        }

        arr
    }


    pub fn len(&self) -> usize {
        self.count
    }


    pub fn push(&mut self, v: T) {
        assert!(self.count < Self::CAPACITY, "Container is fully saturated.");
        self.data[self.count] = v;
        self.count += 1;
    }


    pub fn pop(&mut self, i: usize) {
        assert!(i < self.count, "Pop index must be less than the count.");

        let mut i = i;
        while i < self.count - 1 {
            self.data[i] = self.data[i+1];
            i += 1;
        }
        self.count -= 1;
    }


    pub fn pop_swap(&mut self, i: usize) {
        assert!(i < self.count, "Pop index must be less than the count");
        self.data[i] = self.data[self.count - 1];
        self.count -= 1;
    }
}
