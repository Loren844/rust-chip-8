pub struct Stack {
    stack: [u16; 16]
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: [0;16] }
    }

    pub fn push(&mut self, val:u16) {
        for i in 0..16 {
            if self.stack[i] == 0 {
                self.stack[i] = val;
            }
        }
    }

    pub fn pop(&mut self) -> u16 {
        if let Some(index) = self.stack.iter().rposition(|&x| x != 0) {
            let val = self.stack[index];
            self.stack[index] = 0;
            return val;
        } else {
            return 0;
        }
    }
}