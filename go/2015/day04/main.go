package main

import (
	"crypto/md5"
	"fmt"
)

type Range struct {
	min, max uint32
}

type HashInput struct {
	id         uint32
	input      string
	rng        Range
	zeroString string
}

type HashOutput struct {
	id   uint32
	salt uint32
	hash string
}

const numWorkers = 1
const perTaskRange = 1000000
const puzzleInput = "ckczppom"
const zeroString = "000000"

func main() {
	out := make(chan HashOutput)
	rangeMax := uint32(0)
	inputID := uint32(0)

	makeInput := func() HashInput {
		start := rangeMax
		rangeMax += perTaskRange
		inputID++
		return HashInput{
			inputID,
			puzzleInput,
			Range{start, rangeMax},
			zeroString,
		}
	}

	// crank out workers
	for index := 0; index < numWorkers; index++ {
		go searchWorker(makeInput(), out)
	}

	// wait till we get an output
	for {
		hashOutput := <-out
		if len(hashOutput.hash) == 0 {
			// if the hash is an empty string then produce another worker
			fmt.Printf("[%v] No output\n", hashOutput.id)
			go searchWorker(makeInput(), out)
		} else {
			fmt.Printf("[%v] %v, %v, %v", hashOutput.id, puzzleInput, hashOutput.salt, hashOutput.hash)
			break
		}
	}
}

func searchWorker(hashInput HashInput, out chan HashOutput) {
	id, salt, hash := search(&hashInput)
	out <- HashOutput{id, salt, hash}
}

func search(hashInput *HashInput) (uint32, uint32, string) {
	fmt.Printf("Running range [%v] {%v:%v}\n", hashInput.id, hashInput.rng.min, hashInput.rng.max)
	zeroStringLen := len(hashInput.zeroString)
	for salt := hashInput.rng.min; salt < hashInput.rng.max; salt++ {
		hash := md5.Sum([]byte(fmt.Sprintf("%v%v", hashInput.input, salt)))
		hashStr := fmt.Sprintf("%x", hash)
		if hashStr[:zeroStringLen] == hashInput.zeroString {
			return hashInput.id, salt, hashStr
		}
	}
	return hashInput.id, 0, ""
}
