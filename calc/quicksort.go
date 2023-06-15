package main

import (
	"fmt"
	"math/rand"
	"testing"
	"time"
)

func BenchmarkQuickSort(b *testing.B) {
	arraySize := 10000
	randomArray := createRandomArray(arraySize)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		arr := make([]int, len(randomArray))
		copy(arr, randomArray)
		quickSort(arr, 0, len(arr)-1)
	}
}

func createRandomArray(size int) []int {
	rand.Seed(time.Now().UnixNano())
	arr := make([]int, size)
	for i := 0; i < size; i++ {
		arr[i] = rand.Intn(size * 10)
	}
	return arr
}

func quickSort(arr []int, low, high int) {
	if low < high {
		p := partition(arr, low, high)
		quickSort(arr, low, p-1)
		quickSort(arr, p+1, high)
	}
}

func partition(arr []int, low, high int) int {
	pivot := arr[high]
	i := low
	for j := low; j <= high-1; j++ {
		if arr[j] < pivot {
			arr[i], arr[j] = arr[j], arr[i]
			i++
		}
	}
	arr[i], arr[high] = arr[high], arr[i]
	return i
}

func main() {
	result := testing.Benchmark(BenchmarkQuickSort)
	d := time.Duration(result.NsPerOp())
	fmt.Printf("%s\n", result)
	fmt.Printf("%s/op\n", d)
}
