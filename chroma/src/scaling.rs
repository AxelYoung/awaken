pub struct ScalingMatrix {
   pub transform: ultraviolet::Mat4,
   clip_rect: (u32, u32, u32, u32)
}

impl ScalingMatrix {
   pub fn new(texture_size: (f32, f32), screen_size: (f32, f32)) -> Self {
      let (texture_width, texture_height) = texture_size;
      let (screen_width, screen_height) = screen_size;

      let width_ratio = screen_width / texture_width;
      let height_ratio = screen_height / texture_height;

      let scale = width_ratio.clamp(1.0, height_ratio).floor();

      let scaled_width = scale * texture_width;
      let scaled_height = scale * texture_height;

      // Matrixes, how tf do they work, nobody knows
      let sw = scaled_width / screen_width;
      let sh = scaled_height / screen_height;

      let tx = (screen_width / 2.0).fract() / screen_width;
      let ty = (screen_height / 2.0).fract() / screen_height;

      let transform: [f32; 16] = [
         sw, 0.0, 0.0, 0.0,
         0.0, sh, 0.0, 0.0,
         0.0, 0.0, 1.0, 0.0,
         tx, ty,   0.0, 1.0
      ];

      let clip_rect = {
         let scaled_width = scaled_width.min(screen_width);
         let scaled_height = scaled_height.min(screen_height);

         let x = ((screen_width - scaled_width) / 2.0) as u32;
         let y = ((screen_height - scaled_height) / 2.0) as u32;

         (x, y, scaled_width as u32, scaled_height as u32)
      };

      Self {
         transform: ultraviolet::Mat4::from(transform),
         clip_rect
      }
   }

   pub fn as_bytes(&self) -> &[u8] {
      self.transform.as_byte_slice()
   }

   pub fn clip_rect(&self) -> (u32, u32, u32, u32) {
      self.clip_rect
   }
}