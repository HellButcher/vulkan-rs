/*
Copyright (c) 2016, Christoph Hommelsheim
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

* Redistributions of source code must retain the above copyright notice, this
  list of conditions and the following disclaimer.

* Redistributions in binary form must reproduce the above copyright notice,
  this list of conditions and the following disclaimer in the documentation
  and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

extern crate vulkan_rs;
extern crate winit;
#[macro_use]
extern crate log;
extern crate env_logger;
mod utils;
use utils::Application;

use vulkan_rs::prelude::*;

fn main() {
    env_logger::init().unwrap();
    let events_loop = winit::EventsLoop::new();

    let window = winit::WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&events_loop)
        .unwrap();

    let app = Application::new("initialization", &window).unwrap();

    app.begin().unwrap();
    vkCmdDraw(app.get_command_buffer(), 3, 1, 0, 0);
    app.end().unwrap();

    events_loop.run_forever(|event| {
        info!("{:?}", event);

        match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => {
                events_loop.interrupt()
            }
            _ => (),
        };

        app.begin().unwrap();
        vkCmdDraw(app.get_command_buffer(), 3, 1, 0, 0);
        app.end().unwrap();

    });

    app.wait_idle();

    drop(app);

}
