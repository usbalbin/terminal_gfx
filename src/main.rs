
extern crate terminal_gfx;

use terminal_gfx::TerminalGraphics;
use terminal_gfx::types::*;

fn main() {
    let mut gfx = TerminalGraphics::example_gfx(80, 24);

    let mut vertices = [                                     //                #
        SimpleVertex{ pos: Vector3 {x: -1.0, y: -1.0, z: 0.0}, color: 1.0 },//            #####
        SimpleVertex{ pos: Vector3 {x:  1.0, y:  1.0, z: 0.0}, color: 0.5 },//        #########
        SimpleVertex{ pos: Vector3 {x:  1.0, y: -1.0, z: 0.0}, color: 0.1 },//    #############
                                                                            //#################
    ];

    let mut vertex_scratch = [SimpleVertex::default(); 3];

    let indices = [
        (0, 1, 2)
    ];

    let start = std::time::SystemTime::now();
    
    loop {
        let now = std::time::SystemTime::now();
        let duration = now.duration_since(start)
            .unwrap();

        let now_s = duration.as_secs() as f32 +
            duration.subsec_nanos() as f32 / 1_000_000_000.0;
        vertices[1].pos.x = (now_s * 1.0).sin();

        gfx.draw(&vertices, &mut vertex_scratch, &indices, TerminalGraphics::pixel_shader, TerminalGraphics::vertex_shader);
        gfx.flush();
        gfx.clear(true);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
