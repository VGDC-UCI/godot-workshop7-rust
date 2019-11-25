use gdnative::*;
use gdnative::init::{Property, PropertyHint, PropertyUsage};

pub struct MyNode2D {
    speed: f64,
    singleton_path: NodePath,
}

impl NativeClass for MyNode2D {
    type Base = Node2D;
    type UserData = user_data::MutexData<MyNode2D>;

    fn class_name() -> &'static str {
        "MyNode2D"
    }

    fn init(_owner: Self::Base) -> Self {
        Self::_init()
    }

    fn register_properties(builder: &init::ClassBuilder<Self>) {
        builder.add_property(Property {
            name: "speed",
            default: 1.0 as f64,
            hint: PropertyHint::None,
            getter: |this: &MyNode2D| this.speed,
            setter: |this: &mut MyNode2D, v| this.speed = v,
            usage: PropertyUsage::DEFAULT,
        });

        builder.add_property(Property {
            name: "singleton_path",
            default: NodePath::from_str("/root/MySingleton"),
            hint: PropertyHint::None,
            getter: |this: &MyNode2D| this.singleton_path.new_ref(),
            setter: |this: &mut MyNode2D, v| this.singleton_path = v,
            usage: PropertyUsage::DEFAULT,
        });
    }
}

#[methods]
impl MyNode2D {
    fn _init() -> Self {
        MyNode2D {
            speed: 0 as f64,
            singleton_path: NodePath::from_str("/root/MySingleton"),
        }
    }

    #[export]
    unsafe fn _ready(&self, owner: Node2D) {
        use crate::my_singleton::MySingleton;

        // Find singleton node
        let singleton_node = owner.get_node(self.singleton_path.new_ref()).unwrap();

        // Get Instance from node
        // Nodes are not reference counted, so this code is unsafe.
        let singleton_ref = Instance::<MySingleton>::try_from_unsafe_base(singleton_node).unwrap();

        godot_print!("From MyNode2D:");

        // code inside closure will run if we're able to borrow the reference.
        singleton_ref.map_aliased(|singleton, owner| {
            singleton.print_text(owner);
        }).unwrap();

        // Get only the MySingleton part of the node.
        let singleton_script_ref = singleton_ref.into_script();

        // Again, code will run if we're able to borrow the reference.
        singleton_script_ref.map(|singleton| {
            singleton.print_text_with(" Rust only!");
        }).unwrap();
    }

    #[export]
    unsafe fn _process(&self, mut owner: Node2D, delta: f64) {
        let input = Input::godot_singleton();

        let direction = Vector2::new(
            input.get_action_strength(GodotString::from_str("move_right")) as f32 - input.get_action_strength(GodotString::from_str("move_left")) as f32,
            input.get_action_strength(GodotString::from_str("move_down")) as f32 - input.get_action_strength(GodotString::from_str("move_up")) as f32
        );

        owner.translate(direction * self.speed as f32 * delta as f32);
    }
}