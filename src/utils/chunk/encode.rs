use super::align::AlignedChunk;

pub(crate) fn encode_chunk(chunk_name: &str, chunk: &mut Vec<u8>) -> Vec<u8> {
  chunk.align();

  let mut content: Vec<u8> = vec![];
  content.extend(chunk_name.as_bytes());
  content.extend((chunk.len() as u32).to_be_bytes());
  content.extend(chunk.iter());

  content
}
