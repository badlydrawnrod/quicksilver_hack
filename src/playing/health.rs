pub struct Health(bool);

impl Health {
    pub fn new() -> Self {
        Health(true)
    }

    pub fn kill(&mut self) {
        self.0 = false;
    }

    pub fn is_alive(&self) -> bool {
        self.0
    }

    pub fn is_dead(&self) -> bool {
        !self.0
    }
}

pub fn reap<T: AsRef<Health>>(a: &mut Vec<T>) {
    let mut i = 0;
    while i < a.len() {
        if a[i].as_ref().is_dead() {
            a.remove(i);
        } else {
            i += 1;
        }
    }
}
