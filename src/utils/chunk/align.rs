pub(crate) trait AlignedChunk {
  fn align(&mut self);
}

impl AlignedChunk for Vec<u8> {
  fn align(&mut self) {
    const CHUNK_SIZE: usize = 4;
    let padding_size = (CHUNK_SIZE - (self.len() % CHUNK_SIZE)) % CHUNK_SIZE;
    self.extend(vec![0; padding_size]);
  }
}
