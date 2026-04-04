package blackjack

// ParseCard returns the integer value of a card following blackjack ruleset.
func ParseCard(card string) int {
	switch card {
	case "ace":
		return 11
	case "two":
		return 2
	case "three":
		return 3
	case "four":
		return 4
	case "five":
		return 5
	case "six":
		return 6
	case "seven":
		return 7
	case "eight":
		return 8
	case "nine":
		return 9
	case "ten":
		fallthrough
	case "jack":
		fallthrough
	case "queen":
		fallthrough
	case "king":
		return 10
	}

	return 0
}

// FirstTurn returns the decision for the first turn, given two cards of the
// player and one card of the dealer.
func FirstTurn(card1, card2, dealerCard string) string {
	card1Value := ParseCard(card1)
	card2Value := ParseCard(card2)

	if card1Value == 11 && card2Value == 11 {
		return "P"
	}

	handValue := card1Value + card2Value
	dealerCardValue := ParseCard(dealerCard)

	if handValue == 21 {
		if dealerCardValue < 10 {
			return "W"
		}

		return "S"
	}

	if handValue >= 17 || (handValue >= 12 && dealerCardValue < 7) {
		return "S"
	}

	return "H"
}
