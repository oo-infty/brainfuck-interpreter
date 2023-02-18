use bf_exec::Interpreter;
use common::execution::memory::config::{*, Config as MemoryConfig};
use common::execution::stream::config::{*, Config as StreamConfig};
use criterion::{criterion_group, criterion_main, Criterion};

const BRAINFUCK_CODE: &str =
    "++++[>+++++<-]>[<+++++>-]+<+[>[>+>+<<-]++>>[<<+>>-]>>>[-]++>[-]+>>>+[[-]++++++>>>]<<<[[<++++++++<++>>-]+<.<[>----<-]<]<<[>>>>>[>>>[-]+++++++++<[>-<-]+++++++++>[-[<->-]+[<<<]]<[>+<-]>]<<-]<<-]";

fn interpret() {
    let memory_config = MemoryConfig {
        len: 32768,
        addr: Addr::Unsigned,
        cell: Cell::I8,
        overflow: Overflow::Wrap,
        eof: Eof::Ignore,
    };
    let stream_config = StreamConfig {
        input: Input::Null,
        output: Output::Null,
    };
    let mut interpreter = Interpreter::new(memory_config, stream_config);
    interpreter.run(BRAINFUCK_CODE).unwrap();
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("squares", |b| b.iter(interpret));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
