pub struct AdminUser {
    pub username: String,
    pub password: String,
}
pub struct User {
    pub username: String,
    pub password: String,
}
trait CanEdit {
    fn edit(&self) {
        println!("admin is editing");
    }
}

trait CanCreate {
    fn create(&self) {
        println!("admin is creating");
    }
}

trait CanDelete {
    fn delete(&self) {
        println!("admin is deleting");
    }
}

impl CanEdit for AdminUser {}
impl CanCreate for AdminUser {}
impl CanDelete for AdminUser {}

impl CanEdit for User {
    fn edit(&self) {
        println!("A standard user {} is editing", self.username);
    }
}

pub fn create<T: CanCreate>(user: &T) {
    user.create();
}

pub fn edit<T: CanEdit>(user: &T) {
    user.edit();
}

pub fn delete<T: CanDelete>(user: &T) {
    user.delete();
}

pub fn cache<T: CanCreate + CanDelete>(user: &T) {
    user.delete();
}
