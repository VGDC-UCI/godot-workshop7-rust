use gdnative::*;
use init::InitHandle;

mod my_singleton;
mod my_node2d;

fn init(handle: InitHandle) {
    handle.add_class::<my_singleton::MySingleton>();
    handle.add_class::<my_node2d::MyNode2D>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();