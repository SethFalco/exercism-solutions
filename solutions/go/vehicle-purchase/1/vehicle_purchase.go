package purchase

// RequiresLicense is an arary of the types of vehicles that require a license to operate.
var RequiresLicense = []string{
	"car",
	"truck",
}

// Contains checks if a given array of strings contains a particular string.
func Contains(array []string, body string) bool {
	for _, element := range array {
		if element == body {
			return true
		}
	}

	return false
}

// NeedsLicense determines whether a license is needed to drive a type of vehicle. Only "car" and "truck" require a license.
func NeedsLicense(kind string) bool {
	return Contains(RequiresLicense, kind)
}

// ChooseVehicle recommends a vehicle for selection. It always recommends the vehicle that comes first in lexicographical order.
func ChooseVehicle(option1, option2 string) string {
	var option string

	if option1 <= option2 {
		option = option1
	} else {
		option = option2
	}

	return option + " is clearly the better choice."
}

// CalculateResellPrice calculates how much a vehicle can resell for at a certain age.
func CalculateResellPrice(originalPrice, age float64) float64 {
	if age < 3 {
		return originalPrice * 0.8
	}

	if age < 10 {
		return originalPrice * 0.7
	}

	return originalPrice * 0.5
}
