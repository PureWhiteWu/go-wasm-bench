package main

import (
	"fmt"
	"testing"
	"time"
)

func BenchmarkMatrixMultiplication(b *testing.B) {
	rows, cols := 100, 100
	matrixA := create2DMatrix(rows, cols)
	matrixB := create2DMatrix(rows, cols)

	rowsA := len(matrixA)
	colsB := len(matrixB[0])

	result := create2DMatrix(rowsA, colsB)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = multiplyMatrix(matrixA, matrixB, result)
	}
}

func create2DMatrix(rows, cols int) [][]int {
	matrix := make([][]int, rows)
	for i := 0; i < rows; i++ {
		matrix[i] = make([]int, cols)
		for j := 0; j < cols; j++ {
			matrix[i][j] = i * j
		}
	}
	return matrix
}

func multiplyMatrix(matrixA, matrixB [][]int, result [][]int) [][]int {
	rowsA := len(matrixA)
	colsA := len(matrixA[0])
	colsB := len(matrixB[0])

	for i := 0; i < rowsA; i++ {
		for j := 0; j < colsB; j++ {
			sum := 0
			for k := 0; k < colsA; k++ {
				sum += matrixA[i][k] * matrixB[k][j]
			}
			result[i][j] = sum
		}
	}
	return result
}

func main() {
	result := testing.Benchmark(BenchmarkMatrixMultiplication)
	d := time.Duration(result.NsPerOp())
	fmt.Printf("%s\n", result)
	fmt.Printf("%s/op\n", d)
}
