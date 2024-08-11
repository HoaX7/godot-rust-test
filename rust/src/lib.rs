/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;

struct HotReload;

#[gdextension]
unsafe impl ExtensionLibrary for HotReload {
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

#[derive(GodotClass)]
#[class(base=Node)]
struct Reloadable {
    favorite_planet: Planet,
    //
    // HOT-RELOAD: uncomment this to add a new exported field (also update init() below).
    // #[export]
    // some_string: GString,
}

#[godot_api]
impl INode for Reloadable {
    fn init(_base: Base<Self::Base>) -> Self {
        godot_print!("hello test 2222 ");
        // HOT-RELOAD: change values to initialize with different defaults.
        Self {
            favorite_planet: Planet::Earth,
            //some_string: "Hello, world!".into(),
        }
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
enum Planet {
    Earth,
    Mars,
    Venus,
    //
    // HOT-RELOAD: uncomment this to extend enum.
    Jupiter,
}
