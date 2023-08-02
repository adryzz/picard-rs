
// TODO: investigate possible switch to u32
// The original code uses i32s
pub struct UniformAlloc {
    start: i32,
    end: i32,
    bound: i32,
    tend: i32
}

impl UniformAlloc {
    pub fn new(start: i32, end: i32) -> Self {
        Self {
            start,
            end,
            bound: end,
            tend: end
        }
    }

    pub fn clear_local(&mut self) {
        self.end = self.tend;
    }

    pub fn reinit(&mut self, start: i32, end: i32) {
        // idk how much this is needed in rust
        self.start = start;
        self.end = end;
        self.bound = end;
        self.tend = end;
    }

    pub fn alloc_global(&mut self, size: i32) -> i32 {
        if (self.start + size) > self.bound {
            return -1;
        }

        let ret = self.start;

        self.start += size;

        return ret;
    }

    pub fn alloc_local(&mut self, size: i32) -> i32 {
        let pos = self.end - size;

        if pos < self.start {
            return -1;
        }

        self.bound = if pos < self.bound {pos} else {self.bound};

        self.end = pos;

        return pos;
    }
}

pub struct UniformAllocBundle {
    f_vec_alloc: UniformAlloc,
    i_vec_alloc: UniformAlloc,
    bool_alloc: UniformAlloc
}

impl UniformAllocBundle {
    pub fn new() -> Self {
        Self {
            f_vec_alloc: UniformAlloc::new(0x20, 0x80),
            i_vec_alloc: UniformAlloc::new(0x80, 0x84),
            bool_alloc: UniformAlloc::new(0x88, 0x98)
        }
    }

    pub fn clear(&mut self) {
        self.f_vec_alloc.clear_local();
        self.i_vec_alloc.clear_local();
        self.bool_alloc.clear_local();
    }

    pub fn init_for_gsh(&mut self, first_free: i32) {
        self.f_vec_alloc.reinit(first_free, 0x80);
        self.i_vec_alloc.reinit(0x80, 0x84);
        self.bool_alloc.reinit(0x88, 0x97);
    }
}