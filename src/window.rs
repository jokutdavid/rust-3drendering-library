pub struct Window {
    window: minifb::Window,
    framebuffer: Framebuffer
}

pub struct Framebuffer {
    pub data: Vec<u32>,
    width: usize,
    height: usize,
}

impl Window {
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        let options = minifb::WindowOptions {
            resize: true,
            ..Default::default()
        };

        let window = minifb::Window::new(
            name,
            width,
            height,
            options,
        ).expect("failed to create window");

        Window {
            window,
            framebuffer: Framebuffer::new(width, height),
        }

    }

    pub fn should_close(&self) -> bool {
        !self.window.is_open()
    }

    pub fn display(&mut self) {
        self.window.update_with_buffer(
            &self.framebuffer.data,
            self.framebuffer.width,
            self.framebuffer.height,
        ).expect("failed to update window");

        let (width, height) = self.window.get_size();
        if width != self.framebuffer.width || height != self.framebuffer.height {
            self.framebuffer = Framebuffer::new(width, height);
        }
    }

    pub fn framebuffer(&mut self) -> &mut Framebuffer {
        &mut self.framebuffer
    }
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            data: vec![0; width * height],
            width,
            height
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u32) {
        self.data[x + y * self.width] = value;
    }

    pub fn clear(&mut self, color: u32) {
        for i in 0..self.data.len() {
            self.data[i] = color;
        }
    }

}
