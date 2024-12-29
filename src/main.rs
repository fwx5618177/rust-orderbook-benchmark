fn main() {
    println!("Rust Orderbook Benchmark - main()");
    println!("Run `cargo bench` to execute the benchmarks.");
    println!("Manual test:");
    rust_orderbook_benchmark::benchmark::simple_test_rb_tree();
    rust_orderbook_benchmark::benchmark::simple_test_btree();
    println!("Now run `cargo bench` to see performance results.");
}
