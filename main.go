package main

import (
	"fmt"
	"math/rand"
	"os"
	"path/filepath"
	"regexp"
	"time"

	"github.com/ncruces/zenity"
)

func selectDirectory() (string, error) {
	dir, err := zenity.SelectFile(zenity.Directory())
	if err != nil {
		return "", err
	}
	return dir, nil
}

func main() {
	dir, err := selectDirectory()
	if err != nil {
		fmt.Println("Failed to select directory:", err)
		return
	}

	// Define audio file extensions
	audioExtensions := map[string]bool{
		".mp3": true, ".wav": true, ".m4a": true, ".aac": true,
		// Add other audio file extensions as needed
	}

	// Compile regex to match files starting with 3 digits
	re := regexp.MustCompile(`^\d{3}`)

	// Track used numbers to ensure uniqueness
	usedNumbers := make(map[int]bool)

	// Initialize random seed
	rand.Seed(time.Now().UnixNano())

	// Process files in the selected directory
	err = filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			fmt.Println("Error accessing path:", path, err)
			return err
		}

		if info.IsDir() {
			return nil // Skip directories
		}

		ext := filepath.Ext(info.Name())
		if _, exists := audioExtensions[ext]; !exists {
			return nil // Skip non-audio files
		}

		if !re.MatchString(info.Name()) {
			return nil // Skip files not starting with 3 digits
		}

		// Generate a unique random number
		var newNumber int
		for {
			newNumber = rand.Intn(1000) // 0 to 999
			if !usedNumbers[newNumber] {
				usedNumbers[newNumber] = true
				break
			}
		}

		// Construct new file name
		newName := fmt.Sprintf("%03d%s", newNumber, info.Name()[3:])
		newPath := filepath.Join(filepath.Dir(path), newName)

		// Rename the file
		if err := os.Rename(path, newPath); err != nil {
			fmt.Println("Failed to rename file:", err)
			return err
		}

		fmt.Println("Renamed:", info.Name(), "to", newName)
		return nil
	})

	if err != nil {
		fmt.Println("Error walking the path:", dir, err)
		return
	}

	fmt.Println("Processing complete.")
}
