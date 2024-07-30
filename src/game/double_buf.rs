#[derive(Clone, Copy, Debug)]
enum Pointer {
    ONE,
    TWO,
}

pub struct DoubleBuf<T> {
    elem1: T,
    elem2: T,
    pointer: Pointer,
}

impl<T> DoubleBuf<T> {
    pub fn new(elem1: T, elem2: T) -> Self {
        Self {
            elem1: elem1,
            elem2: elem2,
            pointer: Pointer::ONE,
        }
    }

    pub fn switch(&mut self) {
        self.pointer = match self.pointer {
            Pointer::ONE => Pointer::TWO,
            Pointer::TWO => Pointer::ONE,
        };
    }

    pub fn get_cur_mut(&mut self) -> &mut T {
        match self.pointer {
            Pointer::ONE => &mut self.elem1,
            Pointer::TWO => &mut self.elem2,
        }
    }

    pub fn get_buf_mut(&mut self) -> &mut T {
        match self.pointer {
            Pointer::ONE => &mut self.elem2,
            Pointer::TWO => &mut self.elem1,
        }
    }

    pub fn get_cur(&self) -> &T {
        match self.pointer {
            Pointer::ONE => &self.elem1,
            Pointer::TWO => &self.elem2,
        }
    }

    pub fn get_buf(&self) -> &T {
        match self.pointer {
            Pointer::ONE => &self.elem2,
            Pointer::TWO => &self.elem1,
        }
    }
}

mod test {
    
    #[test]
    fn test_double_buf() {
        use super::DoubleBuf;

        let v1 = vec![7, 7, 7, 7, 7];

        let mut db = DoubleBuf::new(v1.clone(), v1);

        macro_rules! test_closure {
            ( $ans1: expr, $ans2: expr, $ans3: expr, $ans4: expr, $i: expr ) => {
                assert_eq!(db.elem1[$i], $ans1);
                assert_eq!(db.elem2[$i], $ans2);
                assert_eq!(db.get_cur()[$i], $ans3);
                assert_eq!(db.get_buf()[$i], $ans4);
                assert_eq!(db.get_cur_mut()[$i], $ans3);
                assert_eq!(db.get_buf_mut()[$i], $ans4);
            }
        }

        test_closure!(7, 7, 7, 7, 0);

        db.get_cur_mut()[0] = 77;

        test_closure!(77, 7, 77, 7, 0);

        db.switch();

        test_closure!(77, 7, 7, 77, 0);
    }
}
