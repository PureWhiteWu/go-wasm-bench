package main

import (
	"fmt"
	"testing"
	"time"
)

func BenchmarkPrimeFactorization(b *testing.B) {
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = primeFactors(2310)
	}
}

func primeFactors(n int) []int {
	factors := []int{}
	for n%2 == 0 {
		factors = append(factors, 2)
		n = n / 2
	}
	for i := 3; i*i <= n; i = i + 2 {
		for n%i == 0 {
			factors = append(factors, i)
			n = n / i
		}
	}
	if n > 2 {
		factors = append(factors, n)
	}
	return factors
}

func main() {
	result := testing.Benchmark(BenchmarkPrimeFactorization)
	d := time.Duration(result.NsPerOp())
	fmt.Printf("%s\n", result)
	fmt.Printf("%s/op\n", d)
}
