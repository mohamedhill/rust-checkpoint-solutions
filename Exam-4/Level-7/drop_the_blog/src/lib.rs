use std::cell::{Cell, RefCell};
#[derive(Debug)]
pub struct Blog {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}
impl Blog {
    pub fn new() -> Self {
        Self {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }
    pub fn new_article(&self, body: String) -> (usize, Article<'_>) {
        let id = self.new_id();
        self.states.borrow_mut().push(false);
        (id, Article::new(id, body, self))
    }
    pub fn new_id(&self) -> usize {
        self.states.borrow().len()
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }
    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if states[id] {
            panic!("{} is already dropped", id);
        }
        states[id] = true;
        self.drops.set(self.drops.get() + 1);
    }
}
#[derive(Debug, Clone)]
pub struct Article<'a> {
    pub id: usize,
    pub body: String,
    pub parent: &'a Blog,
}
impl<'a> Article<'a> {
    pub fn new(id: usize, body: String, parent: &'a Blog) -> Self {
        Self { id, body, parent }
    }
    pub fn discard(self) {
        drop(self);
    }
}
impl<'a> Drop for Article<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.id);
    }
}








#[cfg(test)]
mod tests {
    use std::rc::Rc;

   use super::*;

    #[test]
    fn test_is_dropped_and_drops() {
        let blog = Blog::new();
        let (pid, article) = blog.new_article(String::from("gnome-shell"));
        let (pid0, article0) = blog.new_article(String::from("i3"));
        let (pid1, article1) = blog.new_article(String::from("shell"));
        let (pid2, article2) = blog.new_article(String::from("spotify"));

        article.discard();
        assert_eq!(blog.drops.get(), 1);
        article0.discard();
        assert_eq!(blog.drops.get(), 2);

        assert!(blog.is_dropped(pid), "{} should have been dropped", pid);
        assert!(blog.is_dropped(pid0), "{} should have been dropped", pid0);
        assert!(
            !blog.is_dropped(pid1),
            "{} should not have been dropped",
            pid1
        );
        assert!(
            !blog.is_dropped(pid2),
            "{} should not have been dropped",
            pid2
        );

        article1.discard();
        article2.discard();
        assert_eq!(blog.drops.get(), 4);
    }

    #[test]
    fn test_using_rc() {
        let blog = Blog::new();
        let (_, article) = blog.new_article(String::from("Xorg"));
        let article = Rc::new(article);
        let article_clone = Rc::clone(&article);

        assert_eq!(Rc::strong_count(&article), 2);
        drop(article_clone);
        assert_eq!(Rc::strong_count(&article), 1);
    }

    #[test]
    #[should_panic]
    fn test_drop_same_article() {
        let blog = Blog::new();
        let (_, article) = blog.new_article(String::from("gsd-rfkill"));
        let article_clone = article.clone();
        article.discard();
        article_clone.discard();
    }
}