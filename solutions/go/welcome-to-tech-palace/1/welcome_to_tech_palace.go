package techpalace

import "strings"

// WelcomeMessage returns a welcome message for the customer.
func WelcomeMessage(customer string) string {
	nameUpper := strings.ToUpper(customer)
	return "Welcome to the Tech Palace, " + nameUpper
}

// AddBorder adds a border to a welcome message.
func AddBorder(welcomeMsg string, numStarsPerLine int) string {
	border := strings.Repeat("*", numStarsPerLine)
	return border + "\n" + welcomeMsg + "\n" + border
}

// CleanupMessage cleans up an old marketing message.
func CleanupMessage(oldMsg string) string {
	borderRemoved := strings.ReplaceAll(oldMsg, "*", "")
	return strings.TrimSpace(borderRemoved)
}
