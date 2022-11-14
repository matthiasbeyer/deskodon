pub struct State {
    username: String,
    instance: String,
}


impl State {
    pub fn empty() -> Self {
        Self {
            username: String::new(),
            instance: String::new(),
        }
    }

    pub fn set_username(&mut self, name: Option<String>) {
        self.username = name;
    }

    pub fn set_instance(&mut self, instance: Option<url::Url>) {
        self.instance = instance;
    }
}
