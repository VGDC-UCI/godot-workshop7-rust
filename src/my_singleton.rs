use gdnative::*;
use gdnative::init::{Property, PropertyHint, PropertyUsage};

pub struct MySingleton {
    text: String,
}

impl NativeClass for MySingleton {
    type Base = Node;
    type UserData = user_data::MutexData<MySingleton>;

    fn class_name() -> &'static str {
        "MySingleton"
    }

    fn init(_owner: Self::Base) -> Self {
        Self::_init()
    }

    fn register_properties(builder: &init::ClassBuilder<Self>) {
        builder.add_property(Property {
            name: "text",
            default: String::from("Hello!"),
            hint: PropertyHint::None,
            getter: |this: &MySingleton| this.text.clone(),
            setter: |this: &mut MySingleton, v| this.text = v,
            usage: PropertyUsage::DEFAULT,
        });
    }
}

#[methods]
impl MySingleton {
    fn _init() -> Self {
        MySingleton {
            text: String::from("Hello!"),
        }
    }

    #[export]
    pub fn print_text(&self, _owner: Node) {
        godot_print!("{}", self.text);
    }
}

impl MySingleton {
    pub fn print_text_with(&self, given_text: &str) {
        let text = self.text.clone() + given_text;
        godot_print!("{}", text);
    }
}
