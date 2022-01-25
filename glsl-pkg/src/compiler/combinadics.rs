
/*
  0t 0f
  1t 1f
  2t 1f

  0t 1t 2t
  0t 1t 2f
  0t 1f 2t
  0t 1f 2f

  0f 1t 2t
  0f 1t 2f
  0f 1f 2t
  0f 1f 2f

*/
struct Buffer {
  w:usize,
  h:usize,
  buffer:Vec<(usize,bool)>
}

impl Buffer {
  fn new(w:usize,h:usize) -> Self {
    let count = w * h;
    let buffer = vec![(0,true);count];
    let mut ret = Buffer { w, h, buffer };
    ret.init();
    ret
  }

  fn get_mut(&mut self,x:usize,y:usize) -> &mut (usize,bool) {
    let idx = y * self.w + x;
    self.buffer.get_mut(idx).unwrap()
  }

  fn init(&mut self) {
    for x in 0..self.w {
      for y in 0..self.h  {
         let elem = self.get_mut(x, y);
         elem.0 = x;
      }
    }
  }
}

pub fn start_combination<F>(n:usize,mut f:F) where F:FnMut(&[(usize,bool)]) {
    let count = 2_i32.pow(n as u32) as usize;
    let mut buffer = Buffer::new(n, count);
    let mut loop_number:usize = 2;
    let mut same_count:usize = count / 2;
    for x in 0..n {
      let mut b = true;
      for ln in 0..loop_number {
        for sc in 0..same_count {
           let y_idx = ln * same_count + sc;
           let elem = buffer.get_mut(x, y_idx);
           elem.1 = b;
        }
        b = !b;
      }
      loop_number *= 2;
      same_count = count / loop_number;
    }

    for y in 0..count {
        let ref_buffer = &buffer.buffer[(y * buffer.w) .. ((y + 1) * buffer.w)];
        f(ref_buffer);
    }
}




#[test]
fn test_number() {
    start_combination(2,|idxs| {
      println!("{:?}",idxs);
    });
}
