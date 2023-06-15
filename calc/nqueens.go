package main

import (
	"fmt"
	"testing"
	"time"
)

func BenchmarkNQueens(b *testing.B) {
	n := 12

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = nQueens(n)
	}
}

func nQueens(n int) int {
	solutions := 0
	board := make([]int, n)
	placeQueen(board, 0, &solutions)
	return solutions
}

func placeQueen(board []int, currentRow int, solutions *int) {
	n := len(board)
	if currentRow == n {
		*solutions++
	} else {
		for col := 0; col < n; col++ {
			if canPlaceQueen(board, currentRow, col) {
				board[currentRow] = col
				placeQueen(board, currentRow+1, solutions)
			}
		}
	}
}

func canPlaceQueen(board []int, row, col int) bool {
	for i := 0; i < row; i++ {
		if board[i] == col || abs(row-i) == abs(col-board[i]) {
			return false
		}
	}
	return true
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func main() {
	result := testing.Benchmark(BenchmarkNQueens)
	d := time.Duration(result.NsPerOp())
	fmt.Printf("%s\n", result)
	fmt.Printf("%s/op\n", d)
}
