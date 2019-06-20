#[macro_use]
extern crate bencher;
use bencher::Bencher;

extern crate bitintr;
use crate::bitintr::Rbit;

fn u8_runner<F: Fn(u8) -> u8>(bench: &mut Bencher, f: F) {
    bench.iter(|| {
        for v in 0..=u8::max_value() {
            bencher::black_box(f(bencher::black_box(v)));
        }
    })
}

fn rbit_u8_3_impl(x: u8) -> u8 {
    ((((x as u64) * 0x0202020202u64) & 0x010884422010u64) % 1023u64) as u8
}

fn rbit_u8_4_impl(x: u8) -> u8 {
    (((((x as u64) * 0x80200802u64) & 0x0884422110u64) * 0x0101010101u64)
        >> 32) as u8
}

fn rbit_u8_7_impl(x: u8) -> u8 {
    ((((((x as u32) * 0x0802u32) & 0x22110u32)
        | (((x as u32) * 0x8020u32) & 0x88440u32))
        * 0x10101u32)
        >> 16) as u8
}

fn rbit_u8_bitintr(bench: &mut Bencher) {
    u8_runner(bench, |x| x.rbit())
}

fn rbit_u8_3(bench: &mut Bencher) {
    u8_runner(bench, |x| rbit_u8_3_impl(x))
}

fn rbit_u8_4(bench: &mut Bencher) {
    u8_runner(bench, |x| rbit_u8_4_impl(x))
}

fn rbit_u8_7(bench: &mut Bencher) {
    u8_runner(bench, |x| rbit_u8_7_impl(x))
}

benchmark_group!(rbit, rbit_u8_bitintr, rbit_u8_3, rbit_u8_4, rbit_u8_7);
benchmark_main!(rbit);
