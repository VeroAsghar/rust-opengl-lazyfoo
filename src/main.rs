use std::{str, slice};
use std::ffi::CString;
use rust_opengl_lazyfoo::*; 

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;
const SCREEN_FPS: i32 = 60;

unsafe fn init_gl() -> Result<(), Box<GLenum>> {

    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();

    glMatrixMode(GL_MODELVIEW);
    glLoadIdentity();

    glClearColor(0f32, 0f32, 0f32, 1f32);

    let error = glGetError();
    if error != GL_NO_ERROR {
        return Err(Box::new(error));
    }

    Ok(())
}

fn update() {
}

unsafe extern "C" fn render() {
    glClear(GL_COLOR_BUFFER_BIT);

    glBegin(GL_QUADS);
    glVertex2f(-0.5f32, -0.5f32);
    glVertex2f(0.5f32, -0.5f32);
    glVertex2f(0.5f32, 0.5f32);
    glVertex2f(-0.5f32, 0.5f32);
    glEnd();

    glutSwapBuffers();
}

unsafe extern "C" fn run_main_loop(val: i32) {

    update();
    render();

    glutTimerFunc(1000 / SCREEN_FPS as u32, Some(run_main_loop), val);

}

fn main() {
    unsafe { 
        let mut argc: i32 = 0;
        glutInit(&mut argc, std::ptr::null_mut());
        glutInitContextVersion(2, 1);
        glutInitDisplayMode(GLUT_DOUBLE);
        glutInitWindowSize(SCREEN_WIDTH, SCREEN_HEIGHT);
        let window_title = CString::new("OpenGL").expect("CString::new failed");
        glutCreateWindow(window_title.as_ptr());
        if let Err(e) = init_gl() {
            let error_string = str::from_utf8(slice::from_raw_parts(gluErrorString(*e), 1)).unwrap();
            eprintln!("Error initializing OpenGL! {}", error_string);
            panic!()
        }

        glutDisplayFunc(Some(render));

        glutTimerFunc(1000 / SCREEN_FPS as u32, Some(run_main_loop), 0);

        glutMainLoop();


    }

}

