package dp

func fib(n int) int {
	state := make(map[int]int)
	return recursive(n, state)
}

func recursive(n int,state map[int]int)int{
	if n == 0{
		return 0
	}
	if n ==1 || n == 2 {
		return 1
	}
	if v,ok := state[n]; ok {
		return v
	}
	state[n] =  recursive(n-1, state) + recursive(n-2, state)
	return state[n]
}
// fib(n) = fib(n-1) + fib(n-2)
func iterator(n int)int{
	if n <= 1{
		return n
	}
	dpTable := make([]int,0, n+1)
	// base case
	dpTable[0] = 0
	dpTable[1] = 1
	// state transform
	for i := 2; i <=n ;i++{
		dpTable[i] = dpTable[i-1] + dpTable[i-2];
	}
	return dpTable[n]
}

