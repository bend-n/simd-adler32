fn main() {
  if let Some(imp) = simd_adler32::imp::avx512::get_imp() {
    afl::fuzz!(|data: &[u8]| {
      imp(1, 0, data);
    });
  }
}