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

pub trait Killable {
    fn kill(&mut self);
}

pub trait Alive {
    fn is_alive(&self) -> bool;

    fn is_dead(&self) -> bool;
}

pub trait Reapable {
    fn reap(&mut self);
}

impl<T: AsMut<Health>> Killable for T {
    fn kill(&mut self) {
        self.as_mut().kill();
    }
}

impl<T: AsRef<Health>> Alive for T {
    fn is_alive(&self) -> bool {
        self.as_ref().is_alive()
    }

    fn is_dead(&self) -> bool {
        self.as_ref().is_dead()
    }
}

impl<T: AsRef<Health>> Reapable for Vec<T> {
    fn reap(&mut self) {
        reap(self);
    }
}

fn reap<T: AsRef<Health>>(a: &mut Vec<T>) {
    let mut i = 0;
    while i < a.len() {
        if a[i].is_dead() {
            a.remove(i);
        } else {
            i += 1;
        }
    }
}
