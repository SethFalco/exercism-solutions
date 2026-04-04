package blackjack

// Enumeration of actions a player can take when dealth a hand in Blackjack.
const (
	Stand            = "S"
	Hit              = "H"
	Split            = "P"
	AutomaticallyWin = "W"
)

// CardValueMap maps card names to their numeric value.
var CardValueMap = map[string]int{
	"ace":   11,
	"two":   2,
	"three": 3,
	"four":  4,
	"five":  5,
	"six":   6,
	"seven": 7,
	"eight": 8,
	"nine":  9,
	"ten":   10,
	"jack":  10,
	"queen": 10,
	"king":  10,
}

// ParseCard returns the integer value of a card following blackjack ruleset.
func ParseCard(card string) int {
	return CardValueMap[card]
}

// FirstTurn returns the decision for the first turn, given two cards of the
// player and one card of the dealer.
func FirstTurn(card1, card2, dealerCard string) string {
	card1Value := ParseCard(card1)
	card2Value := ParseCard(card2)

	if card1Value == 11 && card2Value == 11 {
		return Split
	}

	handValue := card1Value + card2Value
	dealerCardValue := ParseCard(dealerCard)

	if handValue == 21 {
		if dealerCardValue < 10 {
			return AutomaticallyWin
		}

		return Stand
	}

	if handValue >= 17 || (handValue >= 12 && dealerCardValue < 7) {
		return Stand
	}

	return Hit
}
