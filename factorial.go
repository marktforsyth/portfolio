package main

import (
	"fmt"
	"strconv"
)

func factorial(num int) int {
	if num > 20 {
		fmt.Println("Error! Number is too high --cannot compute factorial.")
		return 0
	}

	sum := 1
	for x := 1; x <= num; x ++ {
		sum *= x
	}
	return sum
}

func main() {
	var passwd string
	var num int

	
	fmt.Println("Please enter password to use service: ")
	fmt.Scanln(&passwd)
	
	if passwd == "123password" {
		fmt.Println("Success! Enter an integer below to find its factorial. Don't go above 20.")
		fmt.Scanln(&num)
		
		if factorial(num) != 0 {
			fmt.Println("Output: " + strconv.Itoa(factorial(num)))
		}
	} else {
		fmt.Println("Password incorrect. Access denied.")
	}
}