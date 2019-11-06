pub trait Kill {
    fn kill(&mut self);
    fn is_alive(&self) -> bool;
    fn is_dead(&self) -> bool;
}

macro_rules! killable {
    ($name : ident) => {
        impl crate::playing::killable::Kill for $name {
            fn kill(&mut self) {
                self.alive = false;
            }

            fn is_alive(&self) -> bool {
                self.alive
            }

            fn is_dead(&self) -> bool {
                !self.alive
            }
        }
    };
}

pub trait Reap {
    fn reap(&mut self);
}

impl<T: Kill> Reap for Vec<T> {
    fn reap(&mut self) {
        let mut i = 0;
        while i < self.len() {
            if self[i].is_dead() {
                self.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
