package main

import (
	"fmt"
	"testing"
	"time"
)

func BenchmarkFibonacci(b *testing.B) {
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = fibRecursion(40)
	}
}

func fibRecursion(n int) int {
	if n <= 1 {
		return n
	}
	return fibRecursion(n-1) + fibRecursion(n-2)
}

func main() {
	result := testing.Benchmark(BenchmarkFibonacci)
	d := time.Duration(result.NsPerOp())
	fmt.Printf("%s\n", result)
	fmt.Printf("%s/op\n", d)
}
